use std::fmt;

use crate::account::{Account, EMPTY_CODE_HASH_BYTES};
use crate::call::{CallKind, CallMessage, CallResult};
use crate::constants::{
    gas_cost, precompiles, CallType, BLOCK_HASH_HISTORY, CALL_STACK_LIMIT, MAX_STACK_SIZE,
};
use crate::db::{Database, DatabaseError};
use crate::env::{CfgEnv, Env};
use crate::executor::ExecutionEngine;
use crate::handler::{Frame, Handler};
use crate::host::{AccountLoad, CodeLoad, Host, SStoreResult, SelfDestructResult, StateLoad};
use crate::journaled_state::{JournalCheckpoint, JournalEntry, JournaledState};
use crate::precompiles::{blake2f, ecrecover, identity, modexp, ripemd_160, sha2_256};
use crate::result::EVMError;
use crate::{gas, symbols, ExitStatusCode};
use dora_primitives::spec::SpecId;
use dora_primitives::{Address, Bytecode, Bytes, Bytes32, B256, U256};
use revm_primitives::{keccak256, PrecompileErrors};
use sha3::{Digest, Keccak256};

/// Converts a `U256` value to a `u64`, saturating to `MAX` if the value is too large.
#[macro_export]
macro_rules! as_u64_saturated {
    ($v:expr) => {
        match $v.as_limbs() {
            x => {
                if (x[1] == 0) & (x[2] == 0) & (x[3] == 0) {
                    x[0]
                } else {
                    u64::MAX
                }
            }
        }
    };
}

/// Function type for the main entrypoint of the generated code.
pub type MainFunc = extern "C" fn(&mut RuntimeContext, initial_gas: u64) -> u8;

/// The main context for smart contract execution environment.
pub struct VMContext<'a, DB: Database> {
    /// Environment contains all the information about config, block and transaction.
    pub env: Box<Env>,
    /// Database to load data from.
    pub db: DB,
    /// Handler is a component of the of VM that contains the execution logic.
    pub handler: Handler<'a, DB>,
    /// State with journaling support.
    pub journaled_state: JournaledState,
}

impl<'a, DB: Database> VMContext<'a, DB> {
    /// Creates a new context with the given database.
    #[inline]
    pub fn new(db: DB, env: Env, spec_id: SpecId, handler: Handler<'a, DB>) -> Self {
        Self {
            db,
            env: Box::new(env),
            handler,
            journaled_state: JournaledState::new(spec_id, Default::default()),
        }
    }

    /// Returns the configured EVM spec ID.
    #[inline]
    pub const fn spec_id(&self) -> SpecId {
        self.journaled_state.spec_id
    }

    /// Load access list for berlin hard fork.
    ///
    /// Loading of accounts/storages is needed to make them warm.
    #[inline]
    pub fn load_access_list(&mut self) -> Result<(), DB::Error> {
        for access_list in &self.env.tx.access_list {
            self.journaled_state.initial_account_load(
                access_list.0,
                access_list.1.iter().map(|i| Bytes32::from_be_bytes(i.0)),
                &mut self.db,
            )?;
        }
        Ok(())
    }

    /// Load accounts
    #[inline]
    pub fn load_accounts(&mut self) -> Result<(), EVMError> {
        // load coinbase
        // EIP-3651: Warm COINBASE. Starts the `COINBASE` address warm
        if self.spec_id().is_enabled_in(SpecId::SHANGHAI) {
            let coinbase = self.env.block.coinbase;
            self.journaled_state
                .warm_preloaded_addresses
                .insert(coinbase);
        }

        // Load blockhash storage address
        // EIP-2935: Serve historical block hashes from state
        if self.spec_id().is_enabled_in(SpecId::PRAGUE) {
            self.journaled_state
                .warm_preloaded_addresses
                .insert(Address::from_slice(&hex_literal::hex!(
                    "25a219378dad9b3503c8268c9ca836a52427a4fb"
                )));
        }

        // Load access list
        self.load_access_list()
            .map_err(|_| EVMError::Database(DatabaseError))?;
        Ok(())
    }

    /// Set precompile addresses into the warm preloaded address list.
    #[inline]
    pub fn set_precompiles(&mut self) {
        // Set warm loaded addresses.
        self.journaled_state.warm_preloaded_addresses.extend(
            &[
                precompiles::ECRECOVER_ADDRESS,
                precompiles::IDENTITY_ADDRESS,
                precompiles::SHA2_256_ADDRESS,
                precompiles::RIPEMD_160_ADDRESS,
                precompiles::MODEXP_ADDRESS,
                precompiles::BLAKE2F_ADDRESS,
            ]
            .map(|addr_u64| Bytes32::from(addr_u64).to_address()),
        );
    }

    /// Deducts the caller balance to the transaction limit.
    pub fn deduct_caller(&mut self) -> Result<(), EVMError> {
        let caller = self.env.tx.caller;
        // load caller's account.
        let mut caller_account = self
            .journaled_state
            .load_account(caller, &mut self.db)
            .map_err(|_| EVMError::Database(DatabaseError))?;

        let is_call = !self.env.tx.get_address().is_zero();

        // Subtract gas costs from the caller's account.
        // We need to saturate the gas cost to prevent underflow in case that `disable_balance_check` is enabled.
        let mut gas_cost =
            U256::from(self.env.tx.gas_limit).saturating_mul(self.env.effective_gas_price());

        // EIP-4844
        if let Some(data_fee) = self.env.calc_data_fee() {
            gas_cost = gas_cost.saturating_add(data_fee);
        }

        // Set new caller account balance.
        caller_account.info.balance = caller_account.info.balance.saturating_sub(gas_cost);

        // bump the nonce for calls. Nonce for CREATE will be bumped in `handle_create`.
        if is_call {
            // Nonce is already checked
            caller_account.info.nonce = caller_account.info.nonce.saturating_add(1);
        }
        // touch account so we know it is changed.
        caller_account.mark_touch();

        // Ensure tx kind is call
        if is_call {
            // Push NonceChange entry
            self.journaled_state
                .journal
                .last_mut()
                .unwrap()
                .push(JournalEntry::NonceChange { address: caller });
        }
        Ok(())
    }

    /// Reimburse the caller with gas that were not used.
    pub fn reimburse_caller(
        &mut self,
        gas_remaining: u64,
        gas_refunded: i64,
    ) -> Result<(), EVMError> {
        let caller = self.env.tx.caller;
        let effective_gas_price = self.env.effective_gas_price();

        // Return balance of not used gas.
        let caller_account = self
            .journaled_state
            .load_account(caller, &mut self.db)
            .map_err(|_| EVMError::Database(DatabaseError))?;

        caller_account.data.info.balance =
            caller_account.data.info.balance.saturating_add(
                effective_gas_price * U256::from(gas_remaining + gas_refunded as u64),
            );

        Ok(())
    }

    /// Reward beneficiary with gas fee.
    pub fn reward_beneficiary(&mut self, gas_used: u64, gas_refunded: i64) -> Result<(), EVMError> {
        let beneficiary = self.env.block.coinbase;
        let effective_gas_price = self.env.effective_gas_price();

        // transfer fee to coinbase/beneficiary.
        // EIP-1559 discard basefee for coinbase transfer. Basefee amount of gas is discarded.
        let coinbase_gas_price = if self.spec_id().is_enabled_in(SpecId::LONDON) {
            effective_gas_price.saturating_sub(self.env.block.basefee)
        } else {
            effective_gas_price
        };

        let coinbase_account = self
            .journaled_state
            .load_account(beneficiary, &mut self.db)
            .map_err(|_| EVMError::Database(DatabaseError))?;

        coinbase_account.data.mark_touch();
        coinbase_account.data.info.balance = coinbase_account
            .data
            .info
            .balance
            .saturating_add(coinbase_gas_price * U256::from(gas_used - gas_refunded as u64));

        Ok(())
    }

    /// Handle output of the transaction
    pub fn last_frame_return(&mut self, result: &mut CallResult) {
        let remaining = result.gas_remaining;
        let refunded = result.gas_refunded;
        result.gas_limit = self.env.tx.gas_limit;
        result.gas_remaining = 0;
        result.gas_refunded = 0;
        if result.status.is_ok() {
            result.gas_remaining = remaining;
            result.gas_refunded = refunded;
        } else if result.status.is_revert() {
            result.gas_remaining = remaining;
        }
    }

    /// Return environment.
    #[inline]
    pub fn env(&mut self) -> &mut Env {
        &mut self.env
    }

    /// Returns reference to [`CfgEnv`].
    #[inline]
    pub fn cfg(&self) -> &CfgEnv {
        &self.env.cfg
    }

    /// Fetch block hash from database.
    #[inline]
    pub fn block_hash(&mut self, number: u64) -> Result<B256, DB::Error> {
        self.db.block_hash(U256::from(number))
    }

    /// Mark account as touched as only touched accounts will be added to state.
    #[inline]
    pub fn touch(&mut self, address: &Address) {
        self.journaled_state.touch(address);
    }

    /// Loads an account into memory. Returns `true` if it is cold accessed.
    #[inline]
    pub fn load_account(&mut self, address: Address) -> Result<StateLoad<&mut Account>, DB::Error> {
        self.journaled_state.load_account(address, &mut self.db)
    }

    /// Load account from database to JournaledState.
    ///
    /// Return boolean pair where first is `is_cold` second bool `exists`.
    #[inline]
    pub fn load_account_delegated(&mut self, address: Address) -> Result<AccountLoad, DB::Error> {
        self.journaled_state
            .load_account_delegated(address, &mut self.db)
    }

    /// Return account balance and is_cold flag.
    #[inline]
    pub fn balance(&mut self, address: Address) -> Result<StateLoad<U256>, DB::Error> {
        self.journaled_state
            .load_account(address, &mut self.db)
            .map(|acc| acc.map(|a| a.info.balance))
    }

    /// Return account code bytes and if address is cold loaded.
    #[inline]
    pub fn code(&mut self, address: Address) -> Result<CodeLoad<Bytes>, DB::Error> {
        let acc = self.journaled_state.load_code(address, &mut self.db)?;
        Ok(CodeLoad::new_not_delegated(
            acc.info.code.as_ref().cloned().unwrap_or_else(Bytes::new),
            acc.is_cold,
        ))
    }

    /// Get code hash of address.
    #[inline]
    pub fn code_hash(&mut self, address: Address) -> Result<CodeLoad<Bytes32>, DB::Error> {
        let acc = self.journaled_state.load_code(address, &mut self.db)?;
        if acc.is_empty() {
            return Ok(CodeLoad::new_not_delegated(Bytes32::ZERO, acc.is_cold));
        }
        Ok(CodeLoad::new_not_delegated(
            acc.info.code_hash.into(),
            acc.is_cold,
        ))
    }

    /// Load storage slot, if storage is not present inside the account then it will be loaded from database.
    #[inline]
    pub fn sload(
        &mut self,
        address: Address,
        index: Bytes32,
    ) -> Result<StateLoad<Bytes32>, DB::Error> {
        // account is always warm. reference on that statement https://eips.ethereum.org/EIPS/eip-2929 see `Note 2:`
        self.journaled_state.sload(address, index, &mut self.db)
    }

    /// Storage change of storage slot, before storing `sload` will be called for that slot.
    #[inline]
    pub fn sstore(
        &mut self,
        address: Address,
        index: Bytes32,
        value: Bytes32,
    ) -> Result<StateLoad<SStoreResult>, DB::Error> {
        self.journaled_state
            .sstore(address, index, value, &mut self.db)
    }

    /// Returns the transient storage value.
    #[inline]
    pub fn tload(&mut self, address: Address, index: Bytes32) -> Bytes32 {
        self.journaled_state.tload(address, index)
    }

    /// Stores the transient storage value.
    #[inline]
    pub fn tstore(&mut self, address: Address, index: Bytes32, value: Bytes32) {
        self.journaled_state.tstore(address, index, value)
    }

    /// Selfdestructs the account.
    #[inline]
    pub fn selfdestruct(
        &mut self,
        address: Address,
        target: Address,
    ) -> Result<StateLoad<SelfDestructResult>, DB::Error> {
        self.journaled_state
            .selfdestruct(address, target, &mut self.db)
    }

    fn call_frame(&mut self, frame: Frame) -> Result<CallResult, EVMError> {
        let call_frame_func = self.handler.call_frame.clone();
        call_frame_func(frame, self)
    }

    fn is_precompile_address(address: Address) -> bool {
        match address {
            x if x == Bytes32::from(precompiles::ECRECOVER_ADDRESS).to_address() => true,
            x if x == Bytes32::from(precompiles::IDENTITY_ADDRESS).to_address() => true,
            x if x == Bytes32::from(precompiles::SHA2_256_ADDRESS).to_address() => true,
            x if x == Bytes32::from(precompiles::RIPEMD_160_ADDRESS).to_address() => true,
            x if x == Bytes32::from(precompiles::MODEXP_ADDRESS).to_address() => true,
            x if x == Bytes32::from(precompiles::BLAKE2F_ADDRESS).to_address() => true,
            _ => false,
        }
    }

    /// Call precompile contract
    #[inline]
    fn call_precompile(
        &mut self,
        address: Address,
        calldata: &Bytes,
        gas_limit: u64,
    ) -> Result<Option<CallResult>, EVMError> {
        let result = match address {
            x if x == Bytes32::from(precompiles::ECRECOVER_ADDRESS).to_address() => {
                ecrecover(calldata, gas_limit)
            }
            x if x == Bytes32::from(precompiles::IDENTITY_ADDRESS).to_address() => {
                identity(calldata, gas_limit)
            }
            x if x == Bytes32::from(precompiles::SHA2_256_ADDRESS).to_address() => {
                sha2_256(calldata, gas_limit)
            }
            x if x == Bytes32::from(precompiles::RIPEMD_160_ADDRESS).to_address() => {
                ripemd_160(calldata, gas_limit)
            }
            x if x == Bytes32::from(precompiles::MODEXP_ADDRESS).to_address() => {
                modexp(calldata, gas_limit)
            }
            x if x == Bytes32::from(precompiles::BLAKE2F_ADDRESS).to_address() => {
                blake2f(calldata, gas_limit)
            }
            _ => return Ok(None),
        };
        let mut call_result = CallResult::new_with_gas_limit(gas_limit);
        match result {
            Ok(output) => {
                call_result.output = output.bytes.0.to_vec();
                if !call_result.record_cost(output.gas_used) {
                    call_result.status = ExitStatusCode::PrecompileOOG;
                }
            }
            Err(PrecompileErrors::Error(e)) => {
                if e.is_oog() {
                    call_result.status = ExitStatusCode::PrecompileOOG;
                } else {
                    call_result.status = ExitStatusCode::PrecompileError;
                };
            }
            Err(PrecompileErrors::Fatal { msg }) => {
                return Err(EVMError::Precompile(msg));
            }
        }
        Ok(Some(call_result))
    }

    /// Handle frame sub call.
    pub fn call(&mut self, msg: CallMessage) -> Result<CallResult, EVMError> {
        // Check depth
        if self.journaled_state.depth() > CALL_STACK_LIMIT {
            return Ok(CallResult::new_with_gas_limit_and_status(
                msg.gas_limit,
                ExitStatusCode::CallTooDeep,
            ));
        }
        match msg.kind {
            CallKind::Call | CallKind::CallCode | CallKind::Delegatecall | CallKind::Staticcall => {
                // Make account warm and loaded
                let _ = self
                    .journaled_state
                    .load_account_delegated(msg.code_address, &mut self.db);
                let checkpoint = self.journaled_state.checkpoint();
                if !matches!(msg.kind, CallKind::Delegatecall) {
                    // Create the checkpont for the sub call.
                    if msg.value.is_zero() {
                        self.load_account(msg.recipient)
                            .map_err(|_| EVMError::Database(DatabaseError))?;
                        self.journaled_state.touch(&msg.recipient);
                    } else {
                        // Transfer value from caller to called account. As value get transferred
                        // target gets touched.
                        if let Some(status) = self
                            .journaled_state
                            .transfer(&msg.caller, &msg.recipient, msg.value, &mut self.db)
                            .map_err(|_| EVMError::Database(DatabaseError))?
                        {
                            self.journaled_state.checkpoint_revert(checkpoint);
                            return Ok(CallResult::new_with_gas_limit_and_status(
                                msg.gas_limit,
                                status,
                            ));
                        }
                    }
                }
                if let Some(call_result) = self.call_precompile(
                    msg.code_address,
                    &msg.input.clone().into(),
                    msg.gas_limit,
                )? {
                    if call_result.status.is_ok() {
                        self.journaled_state.checkpoint_commit();
                    } else {
                        self.journaled_state.checkpoint_revert(checkpoint);
                    }
                    Ok(call_result)
                } else {
                    let account = self
                        .journaled_state
                        .load_code(msg.code_address, &mut self.db)
                        .map_err(|_| EVMError::Database(DatabaseError))?;
                    let code_hash = account.info.code_hash;
                    let bytecode = account.info.code.clone().unwrap_or_default();
                    if bytecode.is_empty() {
                        self.journaled_state.checkpoint_commit();
                        return Ok(CallResult::new_with_gas_limit_and_status(
                            msg.gas_limit,
                            ExitStatusCode::Stop,
                        ));
                    }
                    let contract = Contract::new_with_call_message(
                        &msg,
                        msg.input.clone().into(),
                        bytecode,
                        Some(code_hash),
                    );
                    let call_result = self.call_frame(Frame {
                        contract,
                        gas_limit: msg.gas_limit,
                    })?;
                    self.call_return(&call_result.status, checkpoint);
                    Ok(call_result)
                }
            }
            CallKind::Create | CallKind::Create2 => {
                // Fetch balance of caller.
                let caller_balance = self
                    .balance(msg.caller)
                    .map_err(|_| EVMError::Database(DatabaseError))?;
                // Check if caller has enough balance to send to the created contract.
                if caller_balance.data < msg.value {
                    return Ok(CallResult::new_with_gas_limit_and_status(
                        msg.gas_limit,
                        ExitStatusCode::OutOfFunds,
                    ));
                }
                // Increase nonce of caller and check if it overflows
                let old_nonce;
                if let Some(nonce) = self.journaled_state.inc_nonce(msg.caller) {
                    old_nonce = nonce - 1;
                } else {
                    return Ok(CallResult::new_with_gas_limit_and_status(
                        msg.gas_limit,
                        ExitStatusCode::NonceOverflow,
                    ));
                }
                // Created address
                let mut init_code_hash = B256::ZERO;
                let created_address = match msg.salt {
                    Some(s) => {
                        init_code_hash = keccak256(msg.caller);
                        msg.caller.create2(s.0, init_code_hash)
                    }
                    _ => msg.caller.create(old_nonce),
                };
                // Created address is not allowed to be a precompile.
                if Self::is_precompile_address(created_address) {
                    return Ok(CallResult::new_with_gas_limit_and_status(
                        msg.gas_limit,
                        ExitStatusCode::CreateCollision,
                    ));
                }
                // Warm load account.
                self.load_account(created_address)
                    .map_err(|_| EVMError::Database(DatabaseError))?;
                // Create account, transfer funds and make the journal checkpoint.
                let checkpoint = match self.journaled_state.create_account_checkpoint(
                    msg.caller,
                    created_address,
                    msg.value,
                    self.spec_id(),
                ) {
                    Ok(checkpoint) => checkpoint,
                    Err(status) => {
                        return Ok(CallResult::new_with_gas_limit_and_status(
                            msg.gas_limit,
                            status,
                        ));
                    }
                };

                let contract = Contract {
                    input: Bytes::new(),
                    code: msg.input.clone().into(),
                    hash: Some(init_code_hash),
                    target_address: created_address,
                    code_address: created_address,
                    caller: msg.caller,
                    call_value: msg.value,
                };
                let mut call_result = self.call_frame(Frame {
                    contract,
                    gas_limit: msg.gas_limit,
                })?;
                self.create_return(
                    &mut call_result,
                    created_address,
                    msg.input.into(),
                    checkpoint,
                );
                Ok(call_result)
            }
            CallKind::ExtCall
            | CallKind::ExtStaticcall
            | CallKind::ExtDelegatecall
            | CallKind::EofCreate => unimplemented!("{:?}", msg.kind),
        }
    }

    /// Handles call return.
    pub fn call_return(
        &mut self,
        status_code: &ExitStatusCode,
        journal_checkpoint: JournalCheckpoint,
    ) {
        // revert changes or not.
        if status_code.is_ok() {
            self.journaled_state.checkpoint_commit();
        } else {
            self.journaled_state.checkpoint_revert(journal_checkpoint);
        }
    }

    /// Handles create return.
    pub fn create_return(
        &mut self,
        result: &mut CallResult,
        address: Address,
        code: Bytes,
        journal_checkpoint: JournalCheckpoint,
    ) {
        result.create_address = Some(address);
        // if return is not ok revert and return.
        if !result.status.is_ok() {
            self.journaled_state.checkpoint_revert(journal_checkpoint);
            return;
        }
        let spec_id = self.spec_id();
        // Host error if present on execution
        // if ok, check contract creation limit and calculate gas deduction on output len.
        //
        // EIP-3541: Reject new contract code starting with the 0xEF byte
        if spec_id.is_enabled_in(SpecId::LONDON) && code.first() == Some(&0xEF) {
            self.journaled_state.checkpoint_revert(journal_checkpoint);
            result.status = ExitStatusCode::CreateContractStartingWithEF;
            return;
        }

        // EIP-170: Contract code size limit
        // By default limit is 0x6000 (~25kb)
        if spec_id.is_enabled_in(SpecId::SPURIOUS_DRAGON) && code.len() > self.cfg().max_code_size()
        {
            self.journaled_state.checkpoint_revert(journal_checkpoint);
            result.status = ExitStatusCode::CreateContractSizeLimit;
            return;
        }
        let gas_for_code = code.len() as u64 * gas_cost::CODEDEPOSIT;
        if !result.record_cost(gas_for_code) {
            // record code deposit gas cost and check if we are out of gas.
            // EIP-2 point 3: If contract creation does not have enough gas to pay for the
            // final gas fee for adding the contract code to the state, the contract
            //  creation fails (i.e. goes out-of-gas) rather than leaving an empty contract.
            if spec_id.is_enabled_in(SpecId::HOMESTEAD) {
                self.journaled_state.checkpoint_revert(journal_checkpoint);
                result.status = ExitStatusCode::OutOfGas;
                return;
            } else {
                result.output = Vec::new();
            }
        }
        // if we have enough gas we can commit changes.
        self.journaled_state.checkpoint_commit();

        // Set the code to the journaled state.
        self.journaled_state.set_code(address, code);

        result.status = ExitStatusCode::Return;
    }
}

impl<'a, DB: Database> Host for VMContext<'a, DB> {
    #[inline]
    fn env(&self) -> &Env {
        &self.env
    }

    #[inline]
    fn env_mut(&mut self) -> &mut Env {
        &mut self.env
    }

    #[inline]
    fn sload(&mut self, addr: Address, key: Bytes32) -> Option<StateLoad<Bytes32>> {
        self.sload(addr, key).ok()
    }

    #[inline]
    fn sstore(
        &mut self,
        addr: Address,
        key: Bytes32,
        value: Bytes32,
    ) -> Option<StateLoad<SStoreResult>> {
        self.sstore(addr, key, value).ok()
    }

    #[inline]
    fn tload(&mut self, addr: Address, key: Bytes32) -> Bytes32 {
        self.tload(addr, key)
    }

    #[inline]
    fn tstore(&mut self, addr: Address, key: Bytes32, value: Bytes32) {
        self.tstore(addr, key, value)
    }

    #[inline]
    fn load_account_delegated(&mut self, addr: Address) -> Option<AccountLoad> {
        self.load_account_delegated(addr).ok()
    }

    #[inline]
    fn balance(&mut self, addr: Address) -> Option<StateLoad<Bytes32>> {
        self.journaled_state
            .load_account(addr, &mut self.db)
            .map(|acc| acc.map(|a| a.info.balance.into()))
            .ok()
    }

    #[inline]
    fn code(&mut self, addr: Address) -> Option<CodeLoad<Bytes>> {
        self.code(addr).ok()
    }

    #[inline]
    fn code_hash(&mut self, addr: Address) -> Option<CodeLoad<Bytes32>> {
        self.code_hash(addr).ok()
    }

    #[inline]
    fn selfdestruct(
        &mut self,
        addr: Address,
        target: Address,
    ) -> Option<StateLoad<SelfDestructResult>> {
        self.journaled_state
            .selfdestruct(addr, target, &mut self.db)
            .ok()
    }

    fn block_hash(&mut self, number: u64) -> Option<Bytes32> {
        let block_number = as_u64_saturated!(self.env.block.number);
        let Some(diff) = block_number.checked_sub(number) else {
            return Some(Bytes32::ZERO);
        };
        if diff == 0 {
            return Some(Bytes32::ZERO);
        }
        if diff <= BLOCK_HASH_HISTORY {
            return self.block_hash(number).map(Bytes32::from).ok();
        }
        Some(Bytes32::ZERO)
    }

    #[inline]
    fn log(&mut self, log: Log) {
        self.journaled_state.log(log);
    }

    #[inline]
    fn call(&mut self, msg: CallMessage) -> Result<CallResult, EVMError> {
        self.call(msg)
    }
}

/// The internal execution context, which holds the memory, gas, and program state during contract execution.
///
/// This struct contains critical data used to manage the execution environment of smart contracts
/// or other EVM-related programs. It tracks the execution memory, return data, remaining gas, logs, and exit status.
///
/// # Fields:
/// - `memory`: A vector representing the contract's memory during execution.
/// - `returndata`: Optional tuple representing the start and length of the return data.
/// - `program`: A vector containing the bytecode of the program being executed.
/// - `gas_remaining`: The amount of gas remaining for execution, if applicable.
/// - `gas_refund`: The amount of gas to be refunded after execution.
/// - `exit_status`: Optional status code indicating the exit condition of the execution (e.g., success, revert).
/// - `logs`: A vector of logs generated during the execution.
///
/// # Example Usage:
/// ```no_check
/// let inner_context = InnerContext {
///     memory: vec![0; 1024],
///     returndata: None,
///     program: vec![0x60, 0x0A, 0x60, 0x00],  // Sample bytecode
///     gas_remaining: Some(100000),
///     gas_refunded: 0,
///     exit_status: None,
///     logs: Vec::new(),
/// };
/// ```
#[derive(Debug, Default)]
pub struct InnerContext {
    /// Represents the mutable, byte-addressable memory used during contract execution.
    /// This memory is accessible by smart contracts for reading and writing data.
    memory: Vec<u8>,
    /// The return data buffer for internal calls.
    /// It has multi usage:
    ///
    /// * It contains the output bytes of call sub call.
    /// * When this interpreter finishes execution it contains the output bytes of this contract.
    returndata: Vec<u8>,
    /// The remaining gas for the current execution.
    gas_remaining: Option<u64>,
    /// The total gas to be refunded at the end of execution.
    gas_refunded: i64,
    /// The exit status code of the VM execution.
    exit_status: Option<ExitStatusCode>,
    /// Depth in the call stack.
    pub depth: usize,
    /// Whether the context is static.
    pub is_static: bool,
    /// Whether the context is EOF init.
    pub is_eof: bool,
    /// VM spec id
    pub spec_id: SpecId,
}

/// The runtime context for smart contract execution, encapsulating the environment and execution state.
///
/// The `RuntimeContext` struct holds all the necessary information required during the execution of a contract.
/// It tracks the environment, execution journal, current call frame, inner execution context, and transient storage.
/// This is a core struct used in contract execution to manage the overall execution state.
pub struct RuntimeContext<'a> {
    pub inner: InnerContext,
    pub contract: Contract,
    pub host: &'a mut dyn Host,
}

/// VM contract information.
#[derive(Clone, Debug, Default)]
pub struct Contract {
    /// Contracts data
    pub input: Bytes,
    /// The smart contract's bytecode being executed.
    pub code: Bytecode,
    /// Bytecode hash.
    pub hash: Option<B256>,
    /// Target address of the account. Storage of this address is going to be modified.
    pub target_address: Address,
    /// Address of the account the bytecode was loaded from. This can be different from target_address
    /// in the case of DELEGATECALL or CALLCODE
    pub code_address: Address,
    /// Caller of the EVM.
    pub caller: Address,
    /// Value send to contract from transaction or from CALL opcodes.
    pub call_value: U256,
}

impl Contract {
    /// Creates a new contract from the given [`Env`], [`Bytecode`] and optional bytecode hash [`Option<B256>`].
    #[inline]
    pub fn new_with_env(env: &Env, bytecode: Bytecode, hash: Option<B256>) -> Self {
        Self {
            input: env.tx.data.clone(),
            code: bytecode,
            hash,
            target_address: env.tx.transact_to,
            caller: env.tx.caller,
            code_address: env.tx.caller,
            call_value: env.tx.value,
        }
    }

    /// Creates a new contract from the given sub call message [`CallMessage`].
    #[inline]
    pub fn new_with_call_message(
        call_msg: &CallMessage,
        input: Bytes,
        bytecode: Bytecode,
        hash: Option<B256>,
    ) -> Self {
        Self {
            input,
            code: bytecode,
            hash,
            target_address: call_msg.recipient,
            code_address: call_msg.code_address,
            caller: call_msg.caller,
            call_value: call_msg.value,
        }
    }
}

/// Represents log data generated by contract execution, including topics and data.
///
/// `LogData` is used to represent the log entries emitted by contracts during execution.
/// Each log entry can have multiple topics and a binary data field, which can be indexed
/// by listeners or other contracts.
///
/// # Fields:
/// - `topics`: A vector of `U256` values representing indexed topics.
/// - `data`: A binary vector representing the data associated with the log entry.
///
/// # Example Usage:
/// ```no_check
/// let log_data = LogData {
///     topics: vec![U256::from(0x123), Bytes32::from(0x456)],
///     data: vec![0xDE, 0xAD, 0xBE, 0xEF],
/// };
/// ```
#[derive(Clone, Default, Eq, PartialEq, Hash)]
pub struct LogData {
    pub topics: Vec<B256>,
    pub data: Vec<u8>,
}

impl fmt::Debug for LogData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("LogData")
            .field("topics", &self.topics)
            // Use the hex output format
            .field("data", &hex::encode(&self.data))
            .finish()
    }
}

impl LogData {
    /// Creates a new log, without length-checking. This allows creation of
    /// invalid logs. May be safely used when the length of the topic list is
    /// known to be 4 or less.
    #[inline]
    pub const fn new_unchecked(topics: Vec<B256>, data: Vec<u8>) -> Self {
        Self { topics, data }
    }

    /// Creates a new log.
    #[inline]
    pub fn new(topics: Vec<B256>, data: Vec<u8>) -> Option<Self> {
        let this = Self::new_unchecked(topics, data);
        this.is_valid().then_some(this)
    }

    /// Creates a new empty log.
    #[inline]
    pub const fn empty() -> Self {
        Self {
            topics: Vec::new(),
            data: Vec::new(),
        }
    }

    /// True if valid, false otherwise.
    #[inline]
    pub fn is_valid(&self) -> bool {
        self.topics.len() <= 4
    }
}

/// Represents a log entry created during contract execution.
///
/// A log entry consists of the emitting contract's address and the log data (including topics and data).
/// It is emitted during contract execution and can be processed by listeners or other contracts after
/// the transaction is completed.
///
/// # Fields:
/// - `address`: The address of the contract that emitted the log.
/// - `data`: The log data containing topics and binary data.
///
/// # Example Usage:
/// ```no_check
/// let log = Log {
///     address: Address::from_low_u64_be(0x123),
///     data: LogData::default(),
/// };
/// ```
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct Log {
    pub address: Address,
    pub data: LogData,
}

/// A generic struct to represent the result of a runtime function call.
#[repr(C)]
pub struct RuntimeResult<T> {
    /// The error, if any, encountered during execution.
    pub error: u8,
    /// The gas consumed during the execution of the function call.
    pub gas_used: u64,
    /// The result value of the function call. None indicates no value returned.
    pub value: T,
}

impl<T> RuntimeResult<T> {
    /// Creates a new successful result with a value.
    #[inline]
    pub fn success(value: T) -> Self {
        Self {
            error: 0,
            gas_used: 0,
            value,
        }
    }

    /// Creates a new successful result with a value and gas used.
    #[inline]
    pub fn success_with_gas(value: T, gas_used: u64) -> Self {
        Self {
            error: 0,
            gas_used,
            value,
        }
    }

    /// Creates a new error result with an error and gas used.
    #[inline]
    pub fn error(error: u8, value: T) -> Self {
        Self {
            error,
            gas_used: 0,
            value,
        }
    }
}

/// Accessors for managing and retrieving execution results in a runtime context.
impl<'a> RuntimeContext<'a> {
    /// Creates a new `RuntimeContext` with the given environment, journal, and call frame.
    ///
    /// # Parameters
    ///
    /// - `env`: The environment in which the EVM execution is taking place.
    /// - `journal`: A mutable log of state changes made during execution.
    /// - `call_frame`: The frame associated with the current execution call.
    ///
    /// # Returns
    ///
    /// - A new `RuntimeContext` instance initialized with the provided values.
    ///
    /// # Example
    ///
    /// ```no_check
    /// let context = RuntimeContext::new(env, journal, call_frame);
    /// ```
    pub fn new(contract: Contract, host: &'a mut dyn Host, spec_id: SpecId) -> Self {
        Self {
            inner: InnerContext {
                spec_id,
                memory: Vec::with_capacity(4 * 1024),
                ..Default::default()
            },
            host,
            contract,
        }
    }

    /// Retrieves the return data produced during execution.
    ///
    /// If return data exists, this function will return a slice containing the data. Otherwise, it returns an empty slice.
    ///
    /// # Returns
    ///
    /// - `&[u8]`: A slice of bytes representing the return data from execution.
    ///
    /// # Example
    ///
    /// ```no_check
    /// let returndata = context.return_values();
    /// ```
    pub fn return_values(&self) -> &[u8] {
        &self.inner.returndata
    }

    /// Retrieves the memory used during execution.
    #[inline]
    pub fn memory(&self) -> &[u8] {
        &self.inner.memory
    }

    /// Retrieves the execution status code.
    #[inline]
    pub fn status(&self) -> ExitStatusCode {
        self.inner
            .exit_status
            .clone()
            .unwrap_or(ExitStatusCode::Return)
    }

    /// The remaining gas at the end of execution.
    #[inline]
    pub fn gas_remaining(&self) -> u64 {
        self.inner.gas_remaining.unwrap_or_default()
    }

    /// The total gas to be refunded at the end of execution.
    #[inline]
    pub fn gas_refunded(&self) -> i64 {
        self.inner.gas_refunded
    }

    /// Set the last call return data.
    #[inline]
    pub fn set_returndata(&mut self, data: Vec<u8>) {
        self.inner.returndata = data;
    }
}

// System call functions
impl<'a> RuntimeContext<'a> {
    pub(crate) extern "C" fn nop() {}

    pub(crate) extern "C" fn tracing(
        &mut self,
        op: u8,
        gas: u64,
        stack_top_ptr: *mut Bytes32,
        stack_size: u64,
    ) {
        let stack = unsafe {
            let stack_bottom_ptr = stack_top_ptr.sub(stack_size as usize);
            Vec::<Bytes32>::from_raw_parts(stack_bottom_ptr, stack_size as usize, MAX_STACK_SIZE)
        };
        let nano_seconds = if let Ok(duration_since_epoch) =
            std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)
        {
            duration_since_epoch.as_nanos()
        } else {
            0
        };
        println!(
            "time: {}ns, op: {}, opHex: {:x}, gas: 0x{:x}, memSize: {}, stack: {:?}, stackSize: {}",
            nano_seconds,
            op,
            op,
            gas,
            self.memory().len(),
            stack.iter().map(|v| v.to_u256()).collect::<Vec<_>>(),
            stack_size,
        );
        // DO NOT free the stack pointer.
        stack.leak();
    }

    pub extern "C" fn write_result(
        &mut self,
        offset: u64,
        bytes_len: u64,
        remaining_gas: u64,
        execution_result: u8,
    ) {
        self.inner.returndata = if bytes_len != 0 {
            self.inner.memory[offset as usize..offset as usize + bytes_len as usize].to_vec()
        } else {
            vec![]
        };
        self.inner.gas_remaining = Some(remaining_gas);
        self.inner.exit_status = Some(ExitStatusCode::from_u8(execution_result));
    }

    pub extern "C" fn returndata_size(&mut self) -> u64 {
        self.inner.returndata.len() as u64
    }

    pub extern "C" fn returndata_copy(
        &mut self,
        dest_offset: u64,
        offset: u64,
        size: u64,
    ) -> *mut RuntimeResult<()> {
        Self::copy_exact(
            &mut self.inner.memory,
            &self.inner.returndata,
            dest_offset,
            offset,
            size,
        )
    }

    pub extern "C" fn call(
        &mut self,
        local_gas_limit: u64,
        call_to_address: &Bytes32,
        value_to_transfer: &Bytes32,
        args_offset: u64,
        args_size: u64,
        ret_offset: u64,
        ret_size: u64,
        original_remaining_gas: u64,
        call_type: u8,
    ) -> *mut RuntimeResult<u8> {
        let args_offset = args_offset as usize;
        let args_size = args_size as usize;
        let call_type =
            CallType::try_from(call_type).expect("Error while parsing CallType on call syscall");
        let to = Address::from(call_to_address);
        // Load account and calculate gas cost.
        let mut account_load = match self.host.load_account_delegated(to) {
            Some(account_load) => account_load,
            None => {
                return Box::into_raw(Box::new(RuntimeResult::error(
                    ExitStatusCode::FatalExternalError.to_u8(),
                    0,
                )));
            }
        };
        if call_type != CallType::Call {
            account_load.is_empty = false;
        }
        let transfers_value = !value_to_transfer.as_u256().is_zero();
        let gas_cost = gas::call_cost(self.inner.spec_id, transfers_value, account_load);
        // original_gas - gas_cost
        let (gas_remaining, overflow) = original_remaining_gas.overflowing_sub(gas_cost);
        if overflow {
            return Box::into_raw(Box::new(RuntimeResult::error(
                ExitStatusCode::OutOfGas.to_u8(),
                1,
            )));
        }
        // EIP-150: Gas cost changes for IO-heavy operations
        let mut gas_limit = if self.inner.spec_id.is_enabled_in(SpecId::TANGERINE) {
            // take l64 part of gas_limit
            (gas_remaining - gas_remaining / 64).min(local_gas_limit)
        } else {
            local_gas_limit
        };
        // original_gas - gas_cost - gas_limit
        let (gas_remaining, overflow) = gas_remaining.overflowing_sub(gas_limit);
        if overflow {
            return Box::into_raw(Box::new(RuntimeResult::error(
                ExitStatusCode::OutOfGas.to_u8(),
                1,
            )));
        }
        // Add call stipend if there is value to be transferred.
        if matches!(call_type, CallType::Call | CallType::CallCode) && transfers_value {
            gas_limit = gas_limit.saturating_add(gas_cost::CALL_STIPEND);
        }
        let call_msg = CallMessage {
            input: if args_size != 0 {
                self.inner.memory[args_offset..args_offset + args_size].to_vec()
            } else {
                vec![]
            },
            kind: call_type.into(),
            value: if call_type == CallType::Delegatecall {
                self.contract.call_value
            } else {
                value_to_transfer.to_u256()
            },
            depth: self.inner.depth as u32,
            gas_limit,
            caller: if call_type == CallType::Delegatecall {
                self.contract.caller
            } else {
                self.contract.target_address
            },
            salt: None,
            recipient: if matches!(call_type, CallType::Delegatecall | CallType::CallCode) {
                self.contract.target_address
            } else {
                to
            },
            code_address: to,
            is_static: self.inner.is_static || call_type == CallType::Staticcall,
            is_eof: false,
        };
        let call_result = self
            .host
            .call(call_msg)
            .unwrap_or_else(|_| CallResult::new_with_gas_limit(gas_limit));
        self.inner.returndata = call_result.output.clone();
        // Check the error message.
        if call_result.status.is_ok() {
            let gas_remaining = gas_remaining + call_result.gas_remaining;
            self.inner.gas_refunded += call_result.gas_refunded;
            // Copy call output to the memory
            Self::copy_exact(
                &mut self.inner.memory,
                &call_result.output,
                ret_offset,
                0,
                ret_size,
            );
            Box::into_raw(Box::new(RuntimeResult::success_with_gas(
                1,
                original_remaining_gas - gas_remaining,
            )))
        } else if call_result.status.is_revert() {
            let gas_remaining = gas_remaining + call_result.gas_remaining;
            Box::into_raw(Box::new(RuntimeResult::success_with_gas(
                0,
                original_remaining_gas - gas_remaining,
            )))
        } else {
            Box::into_raw(Box::new(RuntimeResult::success_with_gas(
                0,
                original_remaining_gas - gas_remaining,
            )))
        }
    }

    fn copy_exact(
        target: &mut [u8],
        source: &[u8],
        target_offset: u64,
        source_offset: u64,
        size: u64,
    ) -> *mut RuntimeResult<()> {
        let target_offset = target_offset as usize;
        let source_offset = source_offset as usize;
        let size = size as usize;

        let (source_end, overflow) = source_offset.overflowing_add(size);
        // Check bounds
        if overflow || source_end > source.len() {
            return Box::into_raw(Box::new(RuntimeResult::error(
                ExitStatusCode::OutOfOffset.to_u8(),
                (),
            )));
        }
        if size + source_offset > source.len() {
            return Box::into_raw(Box::new(RuntimeResult::error(
                ExitStatusCode::OutOfOffset.to_u8(),
                (),
            )));
        }

        // Calculate bytes to copy
        let available_target_space = target.len() - target_offset;
        let available_source_bytes = source.len() - source_offset;
        let bytes_to_copy = size.min(available_target_space).min(available_source_bytes);

        // Perform the copy
        target[target_offset..target_offset + bytes_to_copy]
            .copy_from_slice(&source[source_offset..source_offset + bytes_to_copy]);

        Box::into_raw(Box::new(RuntimeResult::success(())))
    }

    pub extern "C" fn store_in_selfbalance_ptr(
        &mut self,
        balance: &mut Bytes32,
    ) -> *mut RuntimeResult<()> {
        match self.host.balance(self.contract.target_address) {
            Some(state) => {
                *balance = state.data;
                Box::into_raw(Box::new(RuntimeResult::success(())))
            }
            None => Box::into_raw(Box::new(RuntimeResult::error(
                ExitStatusCode::FatalExternalError.to_u8(),
                (),
            ))),
        }
    }

    pub extern "C" fn keccak256_hasher(&mut self, offset: u64, size: u64, hash_ptr: &mut Bytes32) {
        if size == 0 {
            *hash_ptr = Bytes32::from_be_bytes(EMPTY_CODE_HASH_BYTES);
        } else {
            let data = &self.inner.memory[offset as usize..offset as usize + size as usize];
            let mut hasher = Keccak256::new();
            hasher.update(data);
            let result = hasher.finalize();
            *hash_ptr = Bytes32::from_be_bytes(result.into());
        }
    }

    pub extern "C" fn callvalue(&self, value: &mut Bytes32) {
        *value = self.contract.call_value.into();
    }

    pub extern "C" fn store_in_blobbasefee_ptr(&self, value: &mut Bytes32) {
        *value = self
            .host
            .env()
            .block
            .blob_gasprice
            .unwrap_or_default()
            .into();
    }

    pub extern "C" fn gaslimit(&self) -> u64 {
        self.host.env().tx.gas_limit
    }

    pub extern "C" fn caller(&self, value: &mut Bytes32) {
        value.copy_from(&self.contract.caller);
    }

    pub extern "C" fn store_in_gasprice_ptr(&self, value: &mut Bytes32) {
        *value = self.host.env().tx.gas_price.into();
    }

    pub extern "C" fn chainid(&self) -> u64 {
        self.host.env().cfg.chain_id
    }

    pub extern "C" fn calldata(&mut self) -> *mut u8 {
        self.host.env().tx.data.as_ptr() as _
    }

    pub extern "C" fn calldata_size(&self) -> u64 {
        self.host.env().tx.data.len() as u64
    }

    pub extern "C" fn origin(&self, address: &mut Bytes32) {
        address.copy_from(&self.host.env().tx.caller);
    }

    pub extern "C" fn extend_memory(&mut self, new_size: u64) -> *mut RuntimeResult<*mut u8> {
        // Note the overflow on the 32-bit machine for the max memory e.g., 4GB
        let new_size = new_size as usize;
        if new_size <= self.inner.memory.len() {
            return Box::into_raw(Box::new(RuntimeResult::success(
                self.inner.memory.as_mut_ptr() as _,
            )));
        }
        // Check the memory usage bound
        match self
            .inner
            .memory
            .try_reserve(new_size - self.inner.memory.len())
        {
            Ok(()) => {
                self.inner.memory.resize(new_size, 0);
                Box::into_raw(Box::new(RuntimeResult::success(
                    self.inner.memory.as_mut_ptr() as _,
                )))
            }
            Err(_) => Box::into_raw(Box::new(RuntimeResult::error(
                ExitStatusCode::MemoryLimitOOG.to_u8(),
                std::ptr::null_mut(),
            ))),
        }
    }

    pub extern "C" fn code_copy(&mut self, code_offset: u64, size: u64, dest_offset: u64) {
        let code = &self.contract.code;
        let code_size = code.len();
        let code_offset = code_offset as usize;
        let dest_offset = dest_offset as usize;
        let size = size as usize;
        let code_offset = code_offset.min(code_size);
        let code_end = core::cmp::min(code_offset + size, code_size);
        let code_len: usize = code_end - code_offset;
        if size != 0 {
            let code_slice = &code[code_offset..code_end];
            self.inner.memory[dest_offset..dest_offset + code_len].copy_from_slice(code_slice);
            // Zero-fill the remaining space
            if size > code_len {
                self.inner.memory[dest_offset + code_len..dest_offset + size].fill(0);
            }
        }
    }

    pub extern "C" fn sload(
        &mut self,
        stg_key: &Bytes32,
        stg_value: &mut Bytes32,
    ) -> *mut RuntimeResult<()> {
        let result = match self.host.sload(self.contract.target_address, *stg_key) {
            Some(result) => result,
            None => {
                return Box::into_raw(Box::new(RuntimeResult::error(
                    ExitStatusCode::FatalExternalError.to_u8(),
                    (),
                )))
            }
        };
        *stg_value = result.data;

        let gas_cost = gas::sload_cost(self.inner.spec_id, result.is_cold);
        Box::into_raw(Box::new(RuntimeResult::success_with_gas((), gas_cost)))
    }

    pub extern "C" fn sstore(
        &mut self,
        stg_key: &Bytes32,
        stg_value: &Bytes32,
        gas_remaining: u64,
    ) -> *mut RuntimeResult<()> {
        let result = match self
            .host
            .sstore(self.contract.target_address, *stg_key, *stg_value)
        {
            Some(result) => result,
            None => {
                return Box::into_raw(Box::new(RuntimeResult::error(
                    ExitStatusCode::FatalExternalError.to_u8(),
                    (),
                )))
            }
        };

        let original = result.original_value.to_u256();
        let current = result.present_value.to_u256();
        let new = stg_value.to_u256();

        match gas::sstore_cost(
            self.inner.spec_id,
            original,
            current,
            new,
            gas_remaining,
            result.is_cold,
        ) {
            Some(gas_cost) => {
                self.inner.gas_refunded +=
                    gas::sstore_refund(self.inner.spec_id, original, current, new);
                Box::into_raw(Box::new(RuntimeResult::success_with_gas((), gas_cost)))
            }
            None => Box::into_raw(Box::new(RuntimeResult::error(
                ExitStatusCode::OutOfGas.to_u8(),
                (),
            ))),
        }
    }

    pub extern "C" fn append_log(&mut self, offset: u64, size: u64) {
        self.create_log(offset, size, vec![]);
    }

    pub extern "C" fn append_log_with_one_topic(
        &mut self,
        offset: u64,
        size: u64,
        topic: &Bytes32,
    ) {
        self.create_log(offset, size, vec![topic.to_b256()]);
    }

    pub extern "C" fn append_log_with_two_topics(
        &mut self,
        offset: u64,
        size: u64,
        topic1: &Bytes32,
        topic2: &Bytes32,
    ) {
        self.create_log(offset, size, vec![topic1.to_b256(), topic2.to_b256()]);
    }

    pub extern "C" fn append_log_with_three_topics(
        &mut self,
        offset: u64,
        size: u64,
        topic1: &Bytes32,
        topic2: &Bytes32,
        topic3: &Bytes32,
    ) {
        self.create_log(
            offset,
            size,
            vec![topic1.to_b256(), topic2.to_b256(), topic3.to_b256()],
        );
    }

    pub extern "C" fn append_log_with_four_topics(
        &mut self,
        offset: u64,
        size: u64,
        topic1: &Bytes32,
        topic2: &Bytes32,
        topic3: &Bytes32,
        topic4: &Bytes32,
    ) {
        self.create_log(
            offset,
            size,
            vec![
                topic1.to_b256(),
                topic2.to_b256(),
                topic3.to_b256(),
                topic4.to_b256(),
            ],
        );
    }

    pub extern "C" fn block_number(&self, number: &mut Bytes32) {
        *number = self.host.env().block.number.into();
    }

    pub extern "C" fn block_hash(&mut self, number: &mut Bytes32) -> *mut RuntimeResult<()> {
        let hash = match self.host.block_hash(as_u64_saturated!(number.as_u256())) {
            Some(hash) => hash,
            None => {
                return Box::into_raw(Box::new(RuntimeResult::error(
                    ExitStatusCode::FatalExternalError.to_u8(),
                    (),
                )))
            }
        };
        *number = hash;
        Box::into_raw(Box::new(RuntimeResult::success(())))
    }

    fn create_log(&mut self, offset: u64, size: u64, topics: Vec<B256>) {
        let offset = offset as usize;
        let size = size as usize;
        let data: Vec<u8> = if size != 0 {
            self.inner.memory[offset..offset + size].into()
        } else {
            vec![]
        };
        self.host.log(Log {
            address: self.contract.target_address,
            data: LogData { data, topics },
        });
    }

    pub extern "C" fn extcodesize(&mut self, address: &Bytes32) -> *mut RuntimeResult<u64> {
        let (code, load) = match self.host.code(address.to_address()) {
            Some(code_load) => code_load.into_components(),
            None => {
                return Box::into_raw(Box::new(RuntimeResult::error(
                    ExitStatusCode::FatalExternalError.to_u8(),
                    0,
                )))
            }
        };
        let size = code.len();

        let gas_cost = gas::extcodesize_gas_cost(self.inner.spec_id, load);

        Box::into_raw(Box::new(RuntimeResult {
            gas_used: gas_cost,
            error: 0,
            value: size as u64,
        }))
    }

    #[allow(clippy::clone_on_copy)]
    pub extern "C" fn address(&mut self) -> *mut u8 {
        self.contract.target_address.as_ptr() as *mut u8
    }

    pub extern "C" fn prevrandao(&self, prevrandao: &mut Bytes32) {
        let env = self.host.env();
        *prevrandao = if self.inner.spec_id.is_enabled_in(SpecId::MERGE) {
            let randao = env.block.prevrandao.unwrap_or_default();
            Bytes32::from_be_bytes(randao.into())
        } else {
            env.block.difficulty.into()
        };
    }

    pub extern "C" fn coinbase(&self) -> *mut u8 {
        self.host.env().block.coinbase.as_ptr() as *mut u8
    }

    pub extern "C" fn store_in_timestamp_ptr(&self, value: &mut Bytes32) {
        *value = Bytes32::from(self.host.env().block.timestamp);
    }

    pub extern "C" fn store_in_basefee_ptr(&self, basefee: &mut Bytes32) {
        *basefee = Bytes32::from(&self.host.env().block.basefee);
    }

    /// This function reads an address pointer and set the balance of the address to the same pointer
    pub extern "C" fn store_in_balance(&mut self, address: &mut Bytes32) -> *mut RuntimeResult<()> {
        let addr = address.to_address();
        let result = match self.host.balance(addr) {
            Some(result) => result,
            None => {
                return Box::into_raw(Box::new(RuntimeResult::error(
                    ExitStatusCode::FatalExternalError.to_u8(),
                    (),
                )))
            }
        };
        *address = result.data;
        let gas_cost = gas::balance_gas_cost(self.inner.spec_id, result.is_cold);

        Box::into_raw(Box::new(RuntimeResult {
            gas_used: gas_cost,
            error: 0,
            value: (),
        }))
    }

    pub extern "C" fn blob_hash(&mut self, index: &mut Bytes32) {
        // Check if the high 12 bytes are zero, indicating a valid usize-compatible index
        if index.slice()[0..12] != [0u8; 12] {
            *index = Bytes32::default();
        }

        // Convert the low 20 bytes into a usize for the index
        let idx = usize::from_be_bytes(index.slice()[12..32].try_into().unwrap_or_default());
        *index = self
            .host
            .env()
            .tx
            .blob_hashes
            .get(idx)
            .cloned()
            .map(|x| Bytes32::from_be_bytes(x.into()))
            .unwrap_or_default();
    }

    pub extern "C" fn ext_code_copy(
        &mut self,
        address_value: &Bytes32,
        code_offset: u64,
        size: u64,
        dest_offset: u64,
    ) -> *mut RuntimeResult<()> {
        let addr = address_value.to_address();
        let (code, load) = match self.host.code(addr) {
            Some(code_load) => code_load.into_components(),
            None => {
                return Box::into_raw(Box::new(RuntimeResult::error(
                    ExitStatusCode::FatalExternalError.to_u8(),
                    (),
                )))
            }
        };
        let code_size = code.len();
        let code_offset = code_offset as usize;
        let dest_offset = dest_offset as usize;
        let size = size as usize;
        let code_offset = code_offset.min(code_size);
        let code_end = core::cmp::min(code_offset + size, code_size);
        let code_len = code_end - code_offset;
        if size != 0 {
            let code_slice = &code[code_offset..code_end];
            self.inner.memory[dest_offset..dest_offset + code_len].copy_from_slice(code_slice);
            // Zero-fill the remaining space
            if size > code_len {
                self.inner.memory[dest_offset + code_len..dest_offset + size].fill(0);
            }
        }
        let gas_cost = gas::extcodecopy_gas_cost(self.inner.spec_id, load);
        Box::into_raw(Box::new(RuntimeResult::success_with_gas((), gas_cost)))
    }

    pub extern "C" fn ext_code_hash(&mut self, address: &mut Bytes32) -> *mut RuntimeResult<()> {
        let addr = Address::from(address as &Bytes32);
        let (hash, load) = match self.host.code_hash(addr) {
            Some(code_load) => code_load.into_components(),
            None => {
                return Box::into_raw(Box::new(RuntimeResult::error(
                    ExitStatusCode::FatalExternalError.to_u8(),
                    (),
                )))
            }
        };
        *address = hash;
        let gas_cost = gas::extcodehash_gas_cost(self.inner.spec_id, load);

        Box::into_raw(Box::new(RuntimeResult {
            gas_used: gas_cost,
            error: 0,
            value: (),
        }))
    }

    fn create_aux(
        &mut self,
        size: u64,
        offset: u64,
        value: &mut Bytes32,
        original_remaining_gas: u64,
        salt: Option<&Bytes32>,
    ) -> *mut RuntimeResult<u8> {
        let offset = offset as usize;
        let size = size as usize;
        let memory_len = self.inner.memory.len();
        let available_size = memory_len - offset;
        let actual_size = size.min(available_size);
        let bytecode = if size == 0 {
            vec![]
        } else {
            self.inner.memory[offset..offset + actual_size].to_vec()
        };
        let mut gas_limit = original_remaining_gas;
        if self.inner.spec_id.is_enabled_in(SpecId::TANGERINE) {
            gas_limit -= gas_limit / 64;
        }
        // Original remainning gas - sub call gas limit
        let (gas_remaining, overflow) = original_remaining_gas.overflowing_sub(gas_limit);
        if overflow {
            return Box::into_raw(Box::new(RuntimeResult::error(
                ExitStatusCode::OutOfGas.to_u8(),
                1,
            )));
        }

        let call_msg = CallMessage {
            input: bytecode,
            kind: if salt.is_some() {
                CallKind::Create2
            } else {
                CallKind::Create
            },
            value: value.to_u256(),
            depth: self.inner.depth as u32,
            gas_limit,
            caller: self.contract.target_address,
            salt: salt.map(|salt| salt.to_b256()),
            recipient: Address::default(),
            code_address: Address::default(),
            is_static: self.inner.is_static,
            is_eof: false,
        };
        let call_result = match self.host.call(call_msg) {
            Ok(result) => result,
            Err(_) => {
                return Box::into_raw(Box::new(RuntimeResult::error(
                    ExitStatusCode::FatalExternalError.to_u8(),
                    0,
                )))
            }
        };
        self.inner.returndata = if call_result.status.is_revert() {
            call_result.output
        } else {
            Vec::new()
        };

        // Check the error message.
        if call_result.status.is_ok() {
            // Set created address to the value.
            value.copy_from(&call_result.create_address.unwrap_or_default());
            let gas_remaining = gas_remaining + call_result.gas_remaining;
            self.inner.gas_refunded += call_result.gas_refunded;
            Box::into_raw(Box::new(RuntimeResult::success_with_gas(
                0,
                original_remaining_gas - gas_remaining,
            )))
        } else if call_result.status.is_revert() {
            *value = Bytes32::ZERO;
            let gas_remaining = gas_remaining + call_result.gas_remaining;
            Box::into_raw(Box::new(RuntimeResult::success_with_gas(
                0,
                original_remaining_gas - gas_remaining,
            )))
        } else {
            *value = Bytes32::ZERO;
            Box::into_raw(Box::new(RuntimeResult::success_with_gas(
                0,
                original_remaining_gas - gas_remaining,
            )))
        }
    }

    pub extern "C" fn create(
        &mut self,
        size: u64,
        offset: u64,
        value: &mut Bytes32,
        remaining_gas: u64,
    ) -> *mut RuntimeResult<u8> {
        self.create_aux(size, offset, value, remaining_gas, None)
    }

    pub extern "C" fn create2(
        &mut self,
        size: u64,
        offset: u64,
        value: &mut Bytes32,
        remaining_gas: u64,
        salt: &Bytes32,
    ) -> *mut RuntimeResult<u8> {
        self.create_aux(size, offset, value, remaining_gas, Some(salt))
    }

    pub extern "C" fn selfdestruct(
        &mut self,
        receiver_address: &Bytes32,
    ) -> *mut RuntimeResult<u64> {
        let receiver_address = Address::from(receiver_address);
        let result = match self
            .host
            .selfdestruct(self.contract.target_address, receiver_address)
        {
            Some(result) => result,
            None => {
                return Box::into_raw(Box::new(RuntimeResult::error(
                    ExitStatusCode::FatalExternalError.to_u8(),
                    0,
                )))
            }
        };
        // EIP-3529: Reduction in refunds
        if !self.inner.spec_id.is_enabled_in(SpecId::LONDON) && !result.previously_destroyed {
            self.inner.gas_refunded += gas_cost::SELFDESTRUCT;
        }

        Box::into_raw(Box::new(RuntimeResult {
            error: 0,
            gas_used: gas::selfdestruct_cost(self.inner.spec_id, &result),
            value: 0,
        }))
    }

    pub extern "C" fn tload(&mut self, stg_key: &Bytes32, stg_value: &mut Bytes32) {
        let result = self.host.tload(self.contract.target_address, *stg_key);
        *stg_value = result;
    }

    pub extern "C" fn tstore(&mut self, stg_key: &Bytes32, stg_value: &mut Bytes32) {
        self.host
            .tstore(self.contract.target_address, *stg_key, *stg_value);
    }
}

type SymbolSignature = (&'static str, *const fn() -> ());

impl<'a> RuntimeContext<'a> {
    /// Registers all the syscalls as symbols in the execution engine.
    pub fn register_symbols(&self, engine: &ExecutionEngine) {
        unsafe {
            // Global variables and syscalls with corresponding function signatures
            let symbols_and_signatures: &[SymbolSignature] = &[
                // Global variables
                (
                    symbols::CTX_IS_STATIC,
                    &self.inner.is_static as *const bool as *const _,
                ),
                // Debug functions
                (symbols::NOP, RuntimeContext::nop as *const _),
                (symbols::TRACING, RuntimeContext::tracing as *const _),
                // Syscalls
                (
                    symbols::WRITE_RESULT,
                    RuntimeContext::write_result as *const _,
                ),
                (
                    symbols::KECCAK256_HASHER,
                    RuntimeContext::keccak256_hasher as *const _,
                ),
                (
                    symbols::EXTEND_MEMORY,
                    RuntimeContext::extend_memory as *const _,
                ),
                (symbols::SLOAD, RuntimeContext::sload as *const _),
                (symbols::SSTORE, RuntimeContext::sstore as *const _),
                (symbols::APPEND_LOG, RuntimeContext::append_log as *const _),
                (
                    symbols::APPEND_LOG_ONE_TOPIC,
                    RuntimeContext::append_log_with_one_topic as *const _,
                ),
                (
                    symbols::APPEND_LOG_TWO_TOPICS,
                    RuntimeContext::append_log_with_two_topics as *const _,
                ),
                (
                    symbols::APPEND_LOG_THREE_TOPICS,
                    RuntimeContext::append_log_with_three_topics as *const _,
                ),
                (
                    symbols::APPEND_LOG_FOUR_TOPICS,
                    RuntimeContext::append_log_with_four_topics as *const _,
                ),
                (symbols::CALL, RuntimeContext::call as *const _),
                (symbols::CALLDATA, RuntimeContext::calldata as *const _),
                (
                    symbols::CALLDATA_SIZE,
                    RuntimeContext::calldata_size as *const _,
                ),
                (symbols::CODE_COPY, RuntimeContext::code_copy as *const _),
                (symbols::ORIGIN, RuntimeContext::origin as *const _),
                (symbols::ADDRESS, RuntimeContext::address as *const _),
                (symbols::CALLVALUE, RuntimeContext::callvalue as *const _),
                (
                    symbols::STORE_IN_BLOBBASEFEE_PTR,
                    RuntimeContext::store_in_blobbasefee_ptr as *const _,
                ),
                (
                    symbols::EXT_CODE_SIZE,
                    RuntimeContext::extcodesize as *const _,
                ),
                (symbols::COINBASE, RuntimeContext::coinbase as *const _),
                (
                    symbols::STORE_IN_TIMESTAMP_PTR,
                    RuntimeContext::store_in_timestamp_ptr as *const _,
                ),
                (
                    symbols::STORE_IN_BASEFEE_PTR,
                    RuntimeContext::store_in_basefee_ptr as *const _,
                ),
                (symbols::CALLER, RuntimeContext::caller as *const _),
                (symbols::GASLIMIT, RuntimeContext::gaslimit as *const _),
                (
                    symbols::STORE_IN_GASPRICE_PTR,
                    RuntimeContext::store_in_gasprice_ptr as *const _,
                ),
                (
                    symbols::BLOCK_NUMBER,
                    RuntimeContext::block_number as *const _,
                ),
                (symbols::PREVRANDAO, RuntimeContext::prevrandao as *const _),
                (symbols::BLOB_HASH, RuntimeContext::blob_hash as *const _),
                (symbols::CHAINID, RuntimeContext::chainid as *const _),
                (
                    symbols::STORE_IN_BALANCE,
                    RuntimeContext::store_in_balance as *const _,
                ),
                (
                    symbols::STORE_IN_SELFBALANCE_PTR,
                    RuntimeContext::store_in_selfbalance_ptr as *const _,
                ),
                (
                    symbols::EXT_CODE_COPY,
                    RuntimeContext::ext_code_copy as *const _,
                ),
                (symbols::BLOCK_HASH, RuntimeContext::block_hash as *const _),
                (
                    symbols::EXT_CODE_HASH,
                    RuntimeContext::ext_code_hash as *const _,
                ),
                (symbols::CREATE, RuntimeContext::create as *const _),
                (symbols::CREATE2, RuntimeContext::create2 as *const _),
                (
                    symbols::RETURNDATA_SIZE,
                    RuntimeContext::returndata_size as *const _,
                ),
                (
                    symbols::RETURNDATA_COPY,
                    RuntimeContext::returndata_copy as *const _,
                ),
                (
                    symbols::SELFDESTRUCT,
                    RuntimeContext::selfdestruct as *const _,
                ),
                (symbols::TLOAD, RuntimeContext::tload as *const _),
                (symbols::TSTORE, RuntimeContext::tstore as *const _),
            ];

            for (symbol, signature) in symbols_and_signatures {
                engine.register_symbol(symbol, *signature as *mut ());
            }
        }
    }
}
