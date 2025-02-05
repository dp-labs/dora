use std::cmp::min;
use std::fmt;

use crate::account::{Account, EMPTY_CODE_HASH_BYTES};
use crate::call::{CallKind, CallMessage, CallResult};
use crate::constants::env::DORA_TRACING;
use crate::constants::gas_cost::MIN_CALLEE_GAS;
use crate::constants::{
    gas_cost, CallType, ExtCallType, BLOCK_HASH_HISTORY, CALL_STACK_LIMIT, MAX_STACK_SIZE,
};
use crate::db::{Database, DatabaseError};
use crate::env::{CfgEnv, Env};
use crate::executor::ExecutionEngine;
use crate::handler::{Frame, Handler};
use crate::host::{AccountLoad, CodeLoad, Host, SStoreResult, SelfdestructResult, StateLoad};
use crate::journaled_state::{JournalCheckpoint, JournalEntry, JournaledState};
use crate::result::EVMError;
use crate::{gas, symbols, ExitStatusCode};
use dora_primitives::spec::SpecId;
use dora_primitives::{
    keccak256, Address, Bytecode, Bytes, Bytes32, Precompile, PrecompileErrors, B256, U256,
};
use revm_precompile::{PrecompileSpecId, Precompiles};

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

/// Converts a [U256] value to a [usize], saturating to [MAX][usize] if the value is too large.
#[macro_export]
macro_rules! as_usize_saturated {
    ($v:expr) => {
        usize::try_from($crate::as_u64_saturated!($v)).unwrap_or(usize::MAX)
    };
}

#[repr(C)]
pub struct Stack([Bytes32; MAX_STACK_SIZE]);

impl Stack {
    #[inline]
    pub const fn new() -> Self {
        Self([Bytes32::ZERO; MAX_STACK_SIZE])
    }
}

impl Default for Stack {
    fn default() -> Self {
        Self::new()
    }
}

/// Function type for the EVM main entrypoint of the generated code.
pub type EVMMainFunc = extern "C" fn(
    *mut RuntimeContext,
    initial_gas: *mut u64,
    stack: *mut Stack,
    stack_size: *mut u64,
) -> u8;

/// Function type for the EVM main entrypoint of the generated code.
pub type WASMMainFunc = extern "C" fn(*mut wasmer_vm::VMContext);

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
    /// Precompiles that are available for evm.
    pub precompiles: &'a Precompiles,
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
            precompiles: Precompiles::new(PrecompileSpecId::from_spec_id(spec_id)),
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
        self.journaled_state
            .warm_preloaded_addresses
            .extend(self.precompiles.addresses());
    }

    /// Deducts the caller balance to the transaction limit.
    pub fn deduct_caller(&mut self) -> Result<(), EVMError> {
        let caller = self.env.tx.caller;
        // load caller's account.
        let mut caller_account = self
            .journaled_state
            .load_account(caller, &mut self.db)
            .map_err(|_| EVMError::Database(DatabaseError))?;

        let is_call = self.env.tx.transact_to.is_call();

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
            acc.info
                .code
                .as_ref()
                .cloned()
                .unwrap_or_else(Bytecode::empty)
                .bytes(),
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
    ) -> Result<StateLoad<SelfdestructResult>, DB::Error> {
        self.journaled_state
            .selfdestruct(address, target, &mut self.db)
    }

    fn call_frame(&mut self, frame: Frame) -> Result<CallResult, EVMError> {
        let call_frame_func = self.handler.call_frame.clone();
        call_frame_func(frame, self)
    }

    #[inline]
    fn is_precompile_address(&self, address: &Address) -> bool {
        self.precompiles.get(address).is_some()
    }

    /// Call precompile contract
    #[inline]
    fn call_precompile(
        &mut self,
        address: Address,
        calldata: &Bytes,
        gas_limit: u64,
    ) -> Result<Option<CallResult>, EVMError> {
        let result = match self.precompiles.get(&address) {
            Some(precompile) => {
                let Precompile::Standard(func) = precompile else {
                    return Ok(None);
                };
                func(calldata, gas_limit)
            }
            None => return Ok(None),
        };
        let mut call_result = CallResult::new_with_gas_limit(gas_limit);
        match result {
            Ok(output) => {
                call_result.output = output.bytes;
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
            CallKind::Call | CallKind::Callcode | CallKind::Delegatecall | CallKind::Staticcall => {
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
                if let Some(call_result) =
                    self.call_precompile(msg.code_address, &msg.input.clone(), msg.gas_limit)?
                {
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
                        msg.input.clone(),
                        bytecode,
                        Some(code_hash),
                    );
                    let call_result = self.call_frame(Frame {
                        contract,
                        gas_limit: msg.gas_limit,
                        is_static: msg.is_static,
                        is_eof_init: msg.is_eof_init,
                        validate_eof: msg.validate_eof,
                        depth: self.journaled_state.depth(),
                    })?;
                    self.call_return(&call_result.status, checkpoint);
                    Ok(call_result)
                }
            }
            CallKind::ExtCall | CallKind::ExtStaticcall | CallKind::ExtDelegatecall => {
                // Make account warm and loaded
                let _ = self
                    .journaled_state
                    .load_account_delegated(msg.code_address, &mut self.db);
                let checkpoint = self.journaled_state.checkpoint();
                if matches!(msg.kind, CallKind::ExtDelegatecall) {
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
                if let Some(call_result) =
                    self.call_precompile(msg.code_address, &msg.input.clone(), msg.gas_limit)?
                {
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
                        msg.input.clone(),
                        bytecode,
                        Some(code_hash),
                    );
                    let call_result = self.call_frame(Frame {
                        contract,
                        gas_limit: msg.gas_limit,
                        is_static: msg.is_static,
                        is_eof_init: msg.is_eof_init,
                        validate_eof: msg.validate_eof,
                        depth: self.journaled_state.depth(),
                    })?;
                    self.call_return(&call_result.status, checkpoint);
                    Ok(call_result)
                }
            }
            CallKind::EofCreate => {
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
                        init_code_hash = keccak256(&msg.input);
                        msg.caller.create2(s.0, init_code_hash)
                    }
                    _ => msg.caller.create(old_nonce),
                };
                // Created address is not allowed to be a precompile.
                if self.is_precompile_address(&created_address) {
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
                    code: Bytecode::new(msg.input.clone()),
                    hash: Some(init_code_hash),
                    target_address: created_address,
                    code_address: created_address,
                    caller: msg.caller,
                    call_value: msg.value,
                };
                let mut call_result = self.call_frame(Frame {
                    contract,
                    gas_limit: msg.gas_limit,
                    is_static: msg.is_static,
                    is_eof_init: msg.is_eof_init,
                    validate_eof: msg.validate_eof,
                    depth: self.journaled_state.depth(),
                })?;
                self.create_return(&mut call_result, created_address, checkpoint);
                Ok(call_result)
            }
            CallKind::ReturnContract => {
                self.journaled_state.checkpoint_commit();

                // Eof bytecode is going to be hashed.
                self.journaled_state.set_code(msg.recipient, msg.input);
                Ok(CallResult::new_with_gas_limit_and_status(
                    msg.gas_limit,
                    ExitStatusCode::Return,
                ))
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
                        init_code_hash = keccak256(&msg.input);
                        msg.caller.create2(s.0, init_code_hash)
                    }
                    _ => msg.caller.create(old_nonce),
                };
                // Created address is not allowed to be a precompile.
                if self.is_precompile_address(&created_address) {
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
                    code: Bytecode::new(msg.input.clone()),
                    hash: Some(init_code_hash),
                    target_address: created_address,
                    code_address: created_address,
                    caller: msg.caller,
                    call_value: msg.value,
                };
                let mut call_result = self.call_frame(Frame {
                    contract,
                    gas_limit: msg.gas_limit,
                    is_static: msg.is_static,
                    is_eof_init: msg.is_eof_init,
                    validate_eof: msg.validate_eof,
                    depth: self.journaled_state.depth(),
                })?;
                self.create_return(&mut call_result, created_address, checkpoint);
                Ok(call_result)
            }
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
        // If ok, check contract creation limit and calculate gas deduction on output len.
        //
        // EIP-3541: Reject new contract code starting with the 0xEF byte
        if spec_id.is_enabled_in(SpecId::LONDON) && result.output.first() == Some(&0xEF) {
            self.journaled_state.checkpoint_revert(journal_checkpoint);
            result.status = ExitStatusCode::CreateContractStartingWithEF;
            return;
        }

        // EIP-170: Contract code size limit
        // By default limit is 0x6000 (~25kb)
        if spec_id.is_enabled_in(SpecId::SPURIOUS_DRAGON)
            && result.output.len() > self.cfg().max_code_size()
        {
            self.journaled_state.checkpoint_revert(journal_checkpoint);
            result.status = ExitStatusCode::CreateContractSizeLimit;
            return;
        }
        let gas_for_code = result.output.len() as u64 * gas_cost::CODEDEPOSIT;
        if !result.record_cost(gas_for_code) {
            // Record code deposit gas cost and check if we are out of gas.
            // EIP-2 point 3: If contract creation does not have enough gas to pay for the
            // final gas fee for adding the contract code to the state, the contract
            // creation fails (i.e. goes out-of-gas) rather than leaving an empty contract.
            if spec_id.is_enabled_in(SpecId::HOMESTEAD) {
                self.journaled_state.checkpoint_revert(journal_checkpoint);
                result.status = ExitStatusCode::OutOfGas;
                return;
            } else {
                result.output = Bytes::new();
            }
        }
        // If we have enough gas we can commit changes.
        self.journaled_state.checkpoint_commit();

        // Set the code to the journaled state.
        self.journaled_state
            .set_code(address, result.output.clone());

        result.status = ExitStatusCode::Return;
    }
}

impl<DB: Database> Host for VMContext<'_, DB> {
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
    ) -> Option<StateLoad<SelfdestructResult>> {
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
/// [`InnerContext`] contains critical data used to manage the execution environment of smart contracts
/// or other EVM-related programs.
///
/// It tracks the execution memory, return data, remaining gas, logs, and exit status.
///
/// # Fields:
/// - `memory`: A vector representing the contract's memory during execution.
/// - `returndata`: Optional tuple representing the start and length of the return data.
/// - `program`: A vector containing the bytecode of the program being executed.
/// - `gas_remaining`: The amount of gas remaining for execution, if applicable.
/// - `gas_refund`: The amount of gas to be refunded after execution.
/// - `exit_status`: Optional status code indicating the exit condition of the execution (e.g., success, revert).
/// - `depth`: The depth of the call stack.
/// - `is_static`: A boolean flag indicating whether the context is static.
/// - `is_eof_init`: A boolean flag indicating whether the context is EOF init.
/// - `spec_id`: The EVM spec ID from [SpecId].
/// ```
#[derive(Debug, Default, Clone)]
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
    /// The instruction result buffer.
    result: RuntimeResult<u64>,
    /// Depth in the call stack.
    pub depth: usize,
    /// Whether the context is static.
    pub is_static: bool,
    /// Whether the context is EOF init.
    pub is_eof_init: bool,
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

unsafe impl Send for RuntimeContext<'_> {}
unsafe impl Sync for RuntimeContext<'_> {}

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
            target_address: env.tx.get_address(),
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
#[derive(Debug, Clone, Default)]
pub struct RuntimeResult<T> {
    /// The error, if any, encountered during execution.
    pub error: u8,
    /// The gas consumed during the execution of the function call.
    pub gas_used: u64,
    /// The result value of the function call. None indicates no value returned.
    /// Note: make sure the value is defined at the last of the structure.
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
    /// Creates a new [`RuntimeContext`] with the given environment, journal, and call frame.
    ///
    /// # Parameters
    ///
    /// - `env`: The environment in which the EVM execution is taking place.
    /// - `journal`: A mutable log of state changes made during execution.
    /// - `call_frame`: The frame associated with the current execution call.
    ///
    /// # Returns
    ///
    /// - A new [`RuntimeContext`] instance initialized with the provided values.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let context = RuntimeContext::new(env, journal, call_frame);
    /// ```
    pub fn new(
        contract: Contract,
        depth: usize,
        is_static: bool,
        is_eof_init: bool,
        host: &'a mut dyn Host,
        spec_id: SpecId,
    ) -> Self {
        Self {
            inner: InnerContext {
                spec_id,
                depth,
                memory: Vec::with_capacity(4 * 1024),
                is_static,
                is_eof_init,
                ..Default::default()
            },
            host,
            contract,
        }
    }

    /// Retrieves the return data produced during execution.
    ///
    /// If return data exists, this function will return a slice containing the data.
    /// Otherwise, it returns an empty slice.
    ///
    /// # Returns
    ///
    /// - `&[u8]`: A slice of bytes representing the return data from execution.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let returndata = context.return_values();
    /// ```
    #[inline]
    pub fn return_values(&self) -> &[u8] {
        &self.inner.returndata
    }

    /// Retrieves the return data produced during execution.
    #[inline]
    pub fn return_bytes(&self) -> Bytes {
        self.inner.returndata.to_vec().into()
    }

    /// Retrieves the return data size produced during execution.
    #[inline]
    pub fn return_data_size(&self) -> usize {
        self.inner.returndata.len()
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
impl RuntimeContext<'_> {
    extern "C" fn nop() {}

    extern "C" fn tracing(
        &mut self,
        op: u8,
        gas: u64,
        stack_ptr: *mut Bytes32,
        stack_size_ptr: *mut u64,
    ) {
        let stack_size = unsafe { *stack_size_ptr } as usize;
        let stack =
            unsafe { Vec::<Bytes32>::from_raw_parts(stack_ptr, stack_size, MAX_STACK_SIZE) };
        let nano_seconds = if let Ok(duration_since_epoch) =
            std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)
        {
            duration_since_epoch.as_nanos()
        } else {
            0
        };
        println!(
            "time: {}ns, op: {}, opHex: {:x}, gas: 0x{:x}, memSize: {}, stack: {:?}, stackSize: {}, depth: {}",
            nano_seconds,
            op,
            op,
            gas,
            self.memory().len(),
            stack
                .iter()
                .map(|v| hex::encode(v.to_be_bytes())
                    .trim_start_matches('0')
                    .to_string())
                .map(|v| if v.is_empty() {
                    "0x0".to_string()
                } else {
                    format!("0x{v}")
                })
                .collect::<Vec<_>>(),
            stack_size,
            self.inner.depth,
        );
        // DO NOT free the stack pointer.
        stack.leak();
    }

    extern "C" fn write_result(
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

    extern "C" fn returndata(&mut self) -> *mut u8 {
        self.inner.returndata.as_ptr() as _
    }

    extern "C" fn returndata_size(&mut self) -> u64 {
        self.inner.returndata.len() as u64
    }

    extern "C" fn returndata_copy(
        &mut self,
        memory_offset: u64,
        data_offset: &Bytes32,
        size: u64,
    ) -> *const RuntimeResult<()> {
        let data_offset = as_usize_saturated!(data_offset.to_u256());
        let memory_offset = memory_offset as usize;
        let size = size as usize;
        let (data_end, overflow) = data_offset.overflowing_add(size);
        // Check bounds
        if overflow || data_end > self.inner.returndata.len() {
            self.inner.result.error = ExitStatusCode::OutOfOffset.to_u8();
            return unsafe {
                &*(&self.inner.result as *const RuntimeResult<u64> as *const RuntimeResult<()>)
            };
        }
        // Copy calldata to memory
        if size != 0 {
            self.inner.memory[memory_offset..memory_offset + size]
                .copy_from_slice(&self.inner.returndata[data_offset..data_end]);
        }
        unsafe { &*(&self.inner.result as *const RuntimeResult<u64> as *const RuntimeResult<()>) }
    }

    extern "C" fn call(
        &mut self,
        local_gas_limit: &Bytes32,
        call_to_address: &Bytes32,
        value_to_transfer: &Bytes32,
        args_offset: u64,
        args_size: u64,
        ret_offset: u64,
        ret_size: u64,
        original_remaining_gas: u64,
        call_type: u8,
    ) -> *const RuntimeResult<u64> {
        let args_offset = args_offset as usize;
        let args_size = args_size as usize;
        let call_type =
            CallType::try_from(call_type).expect("Error while parsing CallType on call syscall");
        let to = Address::from(call_to_address);
        // Load account and calculate gas cost.
        let mut account_load = match self.host.load_account_delegated(to) {
            Some(account_load) => account_load,
            None => {
                self.inner.result.error = ExitStatusCode::FatalExternalError.to_u8();
                self.inner.result.value = 0;
                return &self.inner.result as _;
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
            self.inner.result.error = ExitStatusCode::OutOfGas.to_u8();
            self.inner.result.value = 1;
            return &self.inner.result as _;
        }
        let local_gas_limit = as_u64_saturated!(local_gas_limit.to_u256());
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
            self.inner.result.error = ExitStatusCode::OutOfGas.to_u8();
            self.inner.result.value = 1;
            return &self.inner.result as _;
        }
        // Add call stipend if there is value to be transferred.
        if matches!(call_type, CallType::Call | CallType::Callcode) && transfers_value {
            gas_limit = gas_limit.saturating_add(gas_cost::CALL_STIPEND);
        }
        let call_msg = CallMessage {
            kind: call_type.into(),
            input: if args_size != 0 {
                self.inner.memory[args_offset..args_offset + args_size]
                    .to_vec()
                    .into()
            } else {
                Bytes::new()
            },
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
            recipient: if matches!(call_type, CallType::Delegatecall | CallType::Callcode) {
                self.contract.target_address
            } else {
                to
            },
            code_address: to,
            is_static: self.inner.is_static || call_type == CallType::Staticcall,
            is_eof_init: false,
            validate_eof: true,
        };
        if std::env::var(DORA_TRACING).is_ok() {
            println!("info: sub call msg {:?}", call_msg);
        }
        let call_result = self
            .host
            .call(call_msg)
            .unwrap_or_else(|_| CallResult::new_with_gas_limit(gas_limit));
        if std::env::var(DORA_TRACING).is_ok() {
            println!("info: sub call ret {:?}", call_result);
        }
        self.inner.returndata = call_result.output.to_vec();
        let ret_offset = ret_offset as usize;
        let ret_size = ret_size as usize;
        let target_len = min(ret_size, self.inner.returndata.len());
        // Check the error message.
        if call_result.status.is_ok() {
            let gas_remaining = gas_remaining + call_result.gas_remaining;
            self.inner.gas_refunded += call_result.gas_refunded;
            // Copy call output to the memory.
            self.inner.memory[ret_offset..ret_offset + target_len]
                .copy_from_slice(&self.inner.returndata[..target_len]);
            self.inner.result.value = 1;
            self.inner.result.gas_used = original_remaining_gas - gas_remaining;
        } else if call_result.status.is_revert() {
            let gas_remaining = gas_remaining + call_result.gas_remaining;
            // Copy call output to the memory.
            self.inner.memory[ret_offset..ret_offset + target_len]
                .copy_from_slice(&self.inner.returndata[..target_len]);
            self.inner.result.value = 0;
            self.inner.result.gas_used = original_remaining_gas - gas_remaining;
        } else {
            self.inner.result.value = 0;
            self.inner.result.gas_used = original_remaining_gas - gas_remaining;
        }
        &self.inner.result as _
    }

    extern "C" fn extcall(
        &mut self,
        call_to_address: &Bytes32,
        value_to_transfer: &Bytes32,
        input_offset: u64,
        input_size: u64,
        original_remaining_gas: u64,
        call_type: u8,
    ) -> *const RuntimeResult<u64> {
        let input_offset = input_offset as usize;
        let input_size = input_size as usize;
        let call_type = ExtCallType::try_from(call_type)
            .expect("Error while parsing ExtCallType on call syscall");
        let to = Address::from(call_to_address);
        // Load account and calculate gas cost.
        let mut account_load = match self.host.load_account_delegated(to) {
            Some(account_load) => account_load,
            None => {
                self.inner.result.error = ExitStatusCode::FatalExternalError.to_u8();
                self.inner.result.value = 0;
                return &self.inner.result as _;
            }
        };
        if call_type != ExtCallType::Call {
            account_load.is_empty = false;
        }
        let transfers_value = !value_to_transfer.as_u256().is_zero();
        let gas_cost = gas::call_cost(self.inner.spec_id, transfers_value, account_load);
        // original_gas - gas_cost
        let (gas_remaining, overflow) = original_remaining_gas.overflowing_sub(gas_cost);
        if overflow {
            self.inner.result.error = ExitStatusCode::OutOfGas.to_u8();
            self.inner.result.value = 1;
            return &self.inner.result as _;
        }
        // Calculate the gas available to callee as caller’s
        // remaining gas reduced by max(ceil(gas/64), MIN_RETAINED_GAS) (MIN_RETAINED_GAS is 5000).
        let gas_reduce = (gas_remaining / 64).max(5000);
        let gas_limit = gas_remaining.saturating_sub(gas_reduce);

        self.inner.returndata.clear();

        // The MIN_CALLEE_GAS rule is a replacement for stipend:
        // it simplifies the reasoning about the gas costs and is
        // applied uniformly for all introduced EXT*CALL instructions.
        //
        // If Gas available to callee is less than MIN_CALLEE_GAS trigger light failure (Same as Revert).
        if gas_limit < MIN_CALLEE_GAS {
            // Push 1 to stack to indicate that call light failed.
            // It is safe to ignore stack overflow error as we already popped multiple values from stack.
            self.inner.result.error = ExitStatusCode::OutOfGas.to_u8();
            self.inner.result.value = 1;
            return &self.inner.result as _;
        }

        let (gas_remaining, _) = gas_remaining.overflowing_sub(gas_limit);

        let call_msg = CallMessage {
            kind: call_type.into(),
            input: if input_size != 0 {
                self.inner.memory[input_offset..input_offset + input_size]
                    .to_vec()
                    .into()
            } else {
                Bytes::new()
            },
            value: if call_type == ExtCallType::Delegatecall {
                self.contract.call_value
            } else {
                value_to_transfer.to_u256()
            },
            depth: self.inner.depth as u32,
            gas_limit,
            caller: if call_type == ExtCallType::Delegatecall {
                self.contract.caller
            } else {
                self.contract.target_address
            },
            salt: None,
            recipient: if call_type == ExtCallType::Delegatecall {
                self.contract.target_address
            } else {
                to
            },
            code_address: to,
            is_static: self.inner.is_static || call_type == ExtCallType::Staticcall,
            is_eof_init: true,
            validate_eof: true,
        };
        if std::env::var(DORA_TRACING).is_ok() {
            println!("info: sub call msg {:?}", call_msg);
        }
        let call_result = self
            .host
            .call(call_msg)
            .unwrap_or_else(|_| CallResult::new_with_gas_limit(gas_limit));
        if std::env::var(DORA_TRACING).is_ok() {
            println!("info: sub call ret {:?}", call_result);
        }
        self.inner.returndata = call_result.output.to_vec();
        // Check the error message.
        if call_result.status.is_ok() {
            let gas_remaining = gas_remaining + call_result.gas_remaining;
            self.inner.gas_refunded += call_result.gas_refunded;
            self.inner.result.value = 0;
            self.inner.result.gas_used = original_remaining_gas - gas_remaining;
        } else if call_result.status.is_revert() {
            let gas_remaining = gas_remaining + call_result.gas_remaining;
            self.inner.result.value = 1;
            self.inner.result.gas_used = original_remaining_gas - gas_remaining;
        } else {
            self.inner.result.value = 2;
            self.inner.result.gas_used = original_remaining_gas - gas_remaining;
        }
        &self.inner.result as _
    }

    fn memory_set_data(
        &mut self,
        memory_offset: usize,
        data_offset: usize,
        len: usize,
        data: &[u8],
    ) {
        if data_offset > data.len() {
            self.memory_slice_mut(memory_offset, len).fill(0);
            return;
        }
        let data_end = min(data_offset + len, data.len());
        let data_len = data_end - data_offset;
        debug_assert!(data_offset <= data.len() && data_end <= data.len());
        let data = unsafe { data.get_unchecked(data_offset..data_end) };
        self.memory_slice_mut(memory_offset, data_len)
            .copy_from_slice(data);
        self.memory_slice_mut(memory_offset + data_len, len - data_len)
            .fill(0);
    }

    #[inline]
    fn memory_slice_mut(&mut self, offset: usize, size: usize) -> &mut [u8] {
        let end = offset + size;
        &mut self.inner.memory[offset..end]
    }

    extern "C" fn store_in_selfbalance_ptr(
        &mut self,
        balance: &mut Bytes32,
    ) -> *const RuntimeResult<()> {
        match self.host.balance(self.contract.target_address) {
            Some(state) => {
                *balance = state.data;
                unsafe {
                    &*(&self.inner.result as *const RuntimeResult<u64> as *const RuntimeResult<()>)
                }
            }
            None => {
                self.inner.result.error = ExitStatusCode::FatalExternalError.to_u8();
                unsafe {
                    &*(&self.inner.result as *const RuntimeResult<u64> as *const RuntimeResult<()>)
                }
            }
        }
    }

    extern "C" fn ctx_is_static(&mut self) -> u8 {
        self.inner.is_static as u8
    }

    extern "C" fn exp(&mut self, base: &Bytes32, exponent_ptr: &mut Bytes32) {
        let exponent = exponent_ptr.to_u256();
        *exponent_ptr = base.to_u256().pow(exponent).into();
    }

    extern "C" fn keccak256_hasher(&mut self, offset: u64, size: u64, hash_ptr: &mut Bytes32) {
        if size == 0 {
            *hash_ptr = Bytes32::from_be_bytes(EMPTY_CODE_HASH_BYTES);
        } else {
            let data = &self.inner.memory[offset as usize..offset as usize + size as usize];
            let result = keccak256(data);
            *hash_ptr = Bytes32::from_be_bytes(result.into());
        }
    }

    extern "C" fn callvalue(&self, value: &mut Bytes32) {
        *value = self.contract.call_value.into();
    }

    extern "C" fn store_in_blobbasefee_ptr(&self, value: &mut Bytes32) {
        *value = self
            .host
            .env()
            .block
            .blob_excess_gas_and_price
            .clone()
            .unwrap_or_default()
            .blob_gasprice
            .into();
    }

    extern "C" fn caller(&self, value: &mut Bytes32) {
        value.copy_from(&self.contract.caller);
    }

    extern "C" fn store_in_gaslimit_ptr(&self, value: &mut Bytes32) {
        *value = self.host.env().block.gas_limit.into();
    }

    extern "C" fn store_in_gasprice_ptr(&self, value: &mut Bytes32) {
        *value = self.host.env().tx.gas_price.into();
    }

    extern "C" fn chainid(&self) -> u64 {
        self.host.env().cfg.chain_id
    }

    extern "C" fn calldata(&mut self) -> *mut u8 {
        self.contract.input.as_ptr() as _
    }

    extern "C" fn calldata_size(&self) -> u64 {
        self.contract.input.len() as u64
    }

    extern "C" fn calldata_copy(&mut self, memory_offset: u64, data_offset: &Bytes32, size: u64) {
        let size = size as usize;
        if size != 0 {
            let data_offset = as_usize_saturated!(data_offset.to_u256());
            let memory_offset = memory_offset as usize;
            self.memory_set_data(
                memory_offset,
                data_offset,
                size,
                &self.contract.input.clone(),
            );
        }
    }

    extern "C" fn data_section(&mut self) -> *mut u8 {
        self.contract.code.eof().expect("eof").data().as_ptr() as _
    }

    extern "C" fn data_section_size(&self) -> u64 {
        self.contract.code.eof().expect("eof").header.data_size as u64
    }

    extern "C" fn data_section_copy(
        &mut self,
        memory_offset: u64,
        data_offset: &Bytes32,
        size: u64,
    ) {
        let size = size as usize;
        if size != 0 {
            let data_offset = as_usize_saturated!(data_offset.to_u256());
            let memory_offset = memory_offset as usize;
            let eof_data = self.contract.code.eof().expect("eof").data().to_vec();
            self.memory_set_data(memory_offset, data_offset, size, &eof_data);
        }
    }

    extern "C" fn origin(&self, address: &mut Bytes32) {
        address.copy_from(&self.host.env().tx.caller);
    }

    extern "C" fn memory_ptr(&mut self) -> *mut u8 {
        self.inner.memory.as_mut_ptr() as _
    }

    extern "C" fn memory_size(&mut self) -> u64 {
        self.inner.memory.len() as _
    }

    extern "C" fn extend_memory(&mut self, new_size: u64) -> *const RuntimeResult<()> {
        // Note the overflow on the 32-bit machine for the max memory e.g., 4GB
        let new_size = new_size as usize;
        if new_size <= self.inner.memory.len() {
            return unsafe {
                &*(&self.inner.result as *const RuntimeResult<u64> as *const RuntimeResult<()>)
            };
        }
        // Check the memory usage bound
        match self
            .inner
            .memory
            .try_reserve(new_size - self.inner.memory.len())
        {
            Ok(()) => {
                self.inner.memory.resize(new_size, 0);
                unsafe {
                    &*(&self.inner.result as *const RuntimeResult<u64> as *const RuntimeResult<()>)
                }
            }
            Err(_) => {
                self.inner.result.error = ExitStatusCode::MemoryLimitOOG.to_u8();
                unsafe {
                    &*(&self.inner.result as *const RuntimeResult<u64> as *const RuntimeResult<()>)
                }
            }
        }
    }

    extern "C" fn code_copy(&mut self, memory_offset: u64, code_offset: &Bytes32, size: u64) {
        let size = size as usize;
        if size != 0 {
            let code_offset =
                as_usize_saturated!(code_offset.to_u256()).min(self.contract.code.len());
            let memory_offset = memory_offset as usize;
            self.memory_set_data(
                memory_offset,
                code_offset,
                size,
                &self.contract.code.bytecode().clone(),
            );
        }
    }

    extern "C" fn sload(
        &mut self,
        stg_key: &Bytes32,
        stg_value: &mut Bytes32,
    ) -> *const RuntimeResult<()> {
        let result = match self.host.sload(self.contract.target_address, *stg_key) {
            Some(result) => result,
            None => {
                self.inner.result.error = ExitStatusCode::FatalExternalError.to_u8();
                return unsafe {
                    &*(&self.inner.result as *const RuntimeResult<u64> as *const RuntimeResult<()>)
                };
            }
        };
        *stg_value = result.data;

        let gas_cost = gas::sload_cost(self.inner.spec_id, result.is_cold);

        self.inner.result.gas_used = gas_cost;
        unsafe { &*(&self.inner.result as *const RuntimeResult<u64> as *const RuntimeResult<()>) }
    }

    extern "C" fn sstore(
        &mut self,
        stg_key: &Bytes32,
        stg_value: &Bytes32,
        gas_remaining: u64,
    ) -> *const RuntimeResult<()> {
        if self.inner.is_static {
            self.inner.result.error = ExitStatusCode::StateChangeDuringStaticcall.to_u8();
            return unsafe {
                &*(&self.inner.result as *const RuntimeResult<u64> as *const RuntimeResult<()>)
            };
        }
        let mut result = match self
            .host
            .sstore(self.contract.target_address, *stg_key, *stg_value)
        {
            Some(result) => result,
            None => {
                self.inner.result.error = ExitStatusCode::FatalExternalError.to_u8();
                return unsafe {
                    &*(&self.inner.result as *const RuntimeResult<u64> as *const RuntimeResult<()>)
                };
            }
        };
        if let SStoreResult::Slot(slot) = &mut result.data {
            slot.new_value = *stg_value;
        }

        match gas::sstore_cost(
            self.inner.spec_id,
            &result.data,
            gas_remaining,
            result.is_cold,
        ) {
            Some(gas_cost) => {
                self.inner.result.gas_used = gas_cost;
            }
            None => {
                self.inner.result.error = ExitStatusCode::OutOfGas.to_u8();
            }
        }
        unsafe { &*(&self.inner.result as *const RuntimeResult<u64> as *const RuntimeResult<()>) }
    }

    extern "C" fn append_log(&mut self, offset: u64, size: u64) {
        self.create_log(offset, size, vec![]);
    }

    extern "C" fn append_log_with_one_topic(&mut self, offset: u64, size: u64, topic: &Bytes32) {
        self.create_log(offset, size, vec![topic.to_b256()]);
    }

    extern "C" fn append_log_with_two_topics(
        &mut self,
        offset: u64,
        size: u64,
        topic1: &Bytes32,
        topic2: &Bytes32,
    ) {
        self.create_log(offset, size, vec![topic1.to_b256(), topic2.to_b256()]);
    }

    extern "C" fn append_log_with_three_topics(
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

    extern "C" fn append_log_with_four_topics(
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

    extern "C" fn block_number(&self, number: &mut Bytes32) {
        *number = self.host.env().block.number.into();
    }

    extern "C" fn block_hash(&mut self, number: &mut Bytes32) -> *const RuntimeResult<()> {
        match self.host.block_hash(as_u64_saturated!(number.as_u256())) {
            Some(hash) => {
                *number = hash;
            }
            None => {
                self.inner.result.error = ExitStatusCode::FatalExternalError.to_u8();
            }
        };
        unsafe { &*(&self.inner.result as *const RuntimeResult<u64> as *const RuntimeResult<()>) }
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

    extern "C" fn extcodesize(&mut self, address: &Bytes32) -> *const RuntimeResult<u64> {
        let (code, load) = match self.host.code(address.to_address()) {
            Some(code_load) => code_load.into_components(),
            None => {
                self.inner.result.error = ExitStatusCode::FatalExternalError.to_u8();
                return &self.inner.result as _;
            }
        };
        let size = code.len();
        let gas_cost = gas::extcodesize_gas_cost(self.inner.spec_id, load);

        self.inner.result.value = size as u64;
        self.inner.result.gas_used = gas_cost;
        &self.inner.result as _
    }

    extern "C" fn address(&mut self) -> *mut u8 {
        self.contract.target_address.as_ptr() as *mut u8
    }

    extern "C" fn prevrandao(&self, prevrandao: &mut Bytes32) {
        let env = self.host.env();
        *prevrandao = if self.inner.spec_id.is_enabled_in(SpecId::MERGE) {
            let randao = env.block.prevrandao.unwrap_or_default();
            Bytes32::from_be_bytes(randao.into())
        } else {
            env.block.difficulty.into()
        };
    }

    extern "C" fn coinbase(&self) -> *mut u8 {
        self.host.env().block.coinbase.as_ptr() as *mut u8
    }

    extern "C" fn store_in_timestamp_ptr(&self, value: &mut Bytes32) {
        *value = Bytes32::from(self.host.env().block.timestamp);
    }

    extern "C" fn store_in_basefee_ptr(&self, basefee: &mut Bytes32) {
        *basefee = Bytes32::from(&self.host.env().block.basefee);
    }

    /// This function reads an address pointer and set the balance of the address to the same pointer
    extern "C" fn store_in_balance(&mut self, address: &mut Bytes32) -> *const RuntimeResult<()> {
        let addr = address.to_address();
        let result = match self.host.balance(addr) {
            Some(result) => result,
            None => {
                self.inner.result.error = ExitStatusCode::FatalExternalError.to_u8();
                return unsafe {
                    &*(&self.inner.result as *const RuntimeResult<u64> as *const RuntimeResult<()>)
                };
            }
        };
        *address = result.data;
        let gas_cost = gas::balance_gas_cost(self.inner.spec_id, result.is_cold);

        self.inner.result.gas_used = gas_cost;
        unsafe { &*(&self.inner.result as *const RuntimeResult<u64> as *const RuntimeResult<()>) }
    }

    extern "C" fn blob_hash(&mut self, index: &mut Bytes32) {
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

    extern "C" fn ext_code_copy(
        &mut self,
        address_value: &Bytes32,
        code_offset: &Bytes32,
        size: u64,
        memory_offset: u64,
    ) -> *const RuntimeResult<()> {
        let addr = address_value.to_address();
        let (code, load) = match self.host.code(addr) {
            Some(code_load) => code_load.into_components(),
            None => {
                self.inner.result.error = ExitStatusCode::FatalExternalError.to_u8();
                return unsafe {
                    &*(&self.inner.result as *const RuntimeResult<u64> as *const RuntimeResult<()>)
                };
            }
        };
        let code_offset = as_usize_saturated!(code_offset.to_u256());
        let memory_offset = memory_offset as usize;
        let size = size as usize;
        if size != 0 {
            let code_offset = code_offset.min(code.len());
            self.memory_set_data(memory_offset, code_offset, size, &code);
        }
        let gas_cost = gas::extcodecopy_gas_cost(self.inner.spec_id, load);

        self.inner.result.gas_used = gas_cost;
        unsafe { &*(&self.inner.result as *const RuntimeResult<u64> as *const RuntimeResult<()>) }
    }

    extern "C" fn ext_code_hash(&mut self, address: &mut Bytes32) -> *const RuntimeResult<()> {
        let addr = Address::from(address as &Bytes32);
        let (hash, load) = match self.host.code_hash(addr) {
            Some(code_load) => code_load.into_components(),
            None => {
                self.inner.result.error = ExitStatusCode::FatalExternalError.to_u8();
                return unsafe {
                    &*(&self.inner.result as *const RuntimeResult<u64> as *const RuntimeResult<()>)
                };
            }
        };
        *address = hash;
        let gas_cost = gas::extcodehash_gas_cost(self.inner.spec_id, load);

        self.inner.result.gas_used = gas_cost;
        unsafe { &*(&self.inner.result as *const RuntimeResult<u64> as *const RuntimeResult<()>) }
    }

    fn create_aux(
        &mut self,
        size: u64,
        offset: u64,
        value: &mut Bytes32,
        original_remaining_gas: u64,
        salt: Option<&Bytes32>,
    ) -> *const RuntimeResult<()> {
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
            self.inner.result.error = ExitStatusCode::OutOfGas.to_u8();
            return unsafe {
                &*(&self.inner.result as *const RuntimeResult<u64> as *const RuntimeResult<()>)
            };
        }

        let call_msg = CallMessage {
            kind: if salt.is_some() {
                CallKind::Create2
            } else {
                CallKind::Create
            },
            input: bytecode.into(),
            value: value.to_u256(),
            depth: self.inner.depth as u32,
            gas_limit,
            caller: self.contract.target_address,
            salt: salt.map(|salt| salt.to_b256()),
            recipient: Address::default(),
            code_address: Address::default(),
            is_static: self.inner.is_static,
            is_eof_init: false,
            validate_eof: true,
        };
        let call_result = match self.host.call(call_msg) {
            Ok(result) => result,
            Err(_) => {
                self.inner.result.error = ExitStatusCode::FatalExternalError.to_u8();
                return unsafe {
                    &*(&self.inner.result as *const RuntimeResult<u64> as *const RuntimeResult<()>)
                };
            }
        };
        self.inner.returndata = if call_result.status.is_revert() {
            call_result.output.to_vec()
        } else {
            Vec::new()
        };

        // Check the error message.
        if call_result.status.is_ok() {
            // Set created address to the value.
            value.copy_from(&call_result.create_address.unwrap_or_default());
            let gas_remaining = gas_remaining + call_result.gas_remaining;
            self.inner.gas_refunded += call_result.gas_refunded;
            self.inner.result.gas_used = original_remaining_gas - gas_remaining;
        } else if call_result.status.is_revert() {
            *value = Bytes32::ZERO;
            let gas_remaining = gas_remaining + call_result.gas_remaining;
            self.inner.result.gas_used = original_remaining_gas - gas_remaining;
        } else {
            *value = Bytes32::ZERO;
            self.inner.result.gas_used = original_remaining_gas - gas_remaining;
        }
        unsafe { &*(&self.inner.result as *const RuntimeResult<u64> as *const RuntimeResult<()>) }
    }

    extern "C" fn eofcreate(
        &mut self,
        initcontainer_index: u8,
        input_size: u64,
        input_offset: u64,
        value: &mut Bytes32,
        original_remaining_gas: u64,
        salt: &Bytes32,
    ) -> *const RuntimeResult<u64> {
        let container_index = initcontainer_index as usize;
        let data_size = input_size as usize;
        let data_offset = input_offset as usize;

        let container = self
            .contract
            .code
            .eof()
            .expect("eof")
            .body
            .container_section
            .get(container_index)
            .expect("valid container")
            .clone();
        let bytecode = Bytecode::new(container);
        if !bytecode.eof().expect("eof").body.is_data_filled {
            // Should be always false as it is verified by eof verification.
            panic!("Panic if data section is not full");
        }
        let memory_len = self.inner.memory.len();
        let available_size = memory_len - data_offset;
        let actual_size = data_size.min(available_size);
        let input = if data_size == 0 {
            vec![]
        } else {
            self.inner.memory[data_offset..data_offset + actual_size].to_vec()
        };

        // Deduct gas for container execution
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
            kind: CallKind::EofCreate,
            input: bytecode.bytes(),
            value: value.to_u256(),
            depth: self.inner.depth as u32,
            gas_limit,
            caller: self.contract.target_address,
            salt: Some(salt.to_b256()),
            recipient: Address::default(),
            code_address: Address::default(),
            is_static: self.inner.is_static,
            is_eof_init: true,
            validate_eof: true,
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
        // Populate returndata if execution reverted
        self.inner.returndata = if call_result.status.is_revert() {
            call_result.output.to_vec()
        } else {
            Vec::new()
        };

        // Check the error message.
        if call_result.status.is_ok() {
            let new_address = call_result.create_address.unwrap_or_default();
            // Call ReturnContract with the new address
            let call_msg = CallMessage {
                kind: CallKind::ReturnContract,
                input: input.into(),
                value: value.to_u256(),
                depth: self.inner.depth as u32,
                gas_limit: call_result.gas_remaining,
                caller: self.contract.target_address,
                salt: Some(salt.to_b256()),
                recipient: new_address,
                code_address: new_address,
                is_static: self.inner.is_static,
                is_eof_init: true,
                validate_eof: true,
            };
            if self.host.call(call_msg).is_err() {
                return Box::into_raw(Box::new(RuntimeResult::error(
                    ExitStatusCode::FatalExternalError.to_u8(),
                    0,
                )));
            }

            // Set created address to the value.
            value.copy_from(&new_address);
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

    extern "C" fn returncontract(
        &mut self,
        deploy_container_index: u8,
        aux_data_size: u64,
        aux_data_offset: u64,
        max_code_size: usize,
        remaining_gas: u64,
        execution_result: u8,
    ) -> *const RuntimeResult<u64> {
        // Check if returncontract is in EOF init
        if !self.inner.is_eof_init {
            return Box::into_raw(Box::new(RuntimeResult::error(
                ExitStatusCode::ReturnContractInNotInitEOF.to_u8(),
                1,
            )));
        }

        let container_index = deploy_container_index as usize;
        let data_size = aux_data_size as usize;
        let data_offset = aux_data_offset as usize;

        let container = self
            .contract
            .code
            .eof()
            .expect("eof")
            .body
            .container_section
            .get(container_index)
            .expect("valid container")
            .clone();
        let bytecode_header = Bytecode::new(container.clone())
            .eof()
            .expect("eof")
            .header
            .clone();
        let static_aux_size = bytecode_header.eof_size() - container.len();

        let memory_len = self.inner.memory.len();
        let available_size = memory_len - data_offset;
        let actual_size = data_size.min(available_size);
        let mut output = container.to_vec();
        if data_size != 0 {
            output.extend_from_slice(&self.inner.memory[data_offset..data_offset + actual_size]);
        }

        // `data_size - static_aux_size` give us current data `container` size.
        // And with `aux_slice` len we can calculate new data size.
        let new_data_size = bytecode_header.data_size as usize - static_aux_size + data_size;
        if new_data_size > max_code_size {
            // Aux data is too big
            return Box::into_raw(Box::new(RuntimeResult::error(
                ExitStatusCode::EofAuxDataOverflow.to_u8(),
                1,
            )));
        }
        if new_data_size < bytecode_header.data_size as usize {
            // Aux data is too small
            return Box::into_raw(Box::new(RuntimeResult::error(
                ExitStatusCode::EofAuxDataTooSmall.to_u8(),
                1,
            )));
        }
        let new_data_size = (new_data_size as u16).to_be_bytes();
        // Set new data size in eof bytes as we know exact index.
        output[bytecode_header.data_size_raw_i()..][..2].clone_from_slice(&new_data_size);
        let output: Bytes = output.into();

        let call_msg = CallMessage {
            kind: CallKind::ReturnContract,
            input: output,
            value: self.contract.call_value,
            depth: self.inner.depth as u32,
            gas_limit: remaining_gas,
            caller: self.contract.caller,
            salt: None,
            recipient: self.contract.target_address,
            code_address: Address::default(),
            is_static: self.inner.is_static,
            is_eof_init: true,
            validate_eof: true,
        };
        match self.host.call(call_msg) {
            Ok(_) => {
                self.inner.exit_status = Some(ExitStatusCode::from_u8(execution_result));
                &self.inner.result
            }
            Err(_) => Box::into_raw(Box::new(RuntimeResult::error(
                ExitStatusCode::FatalExternalError.to_u8(),
                0,
            ))),
        }
    }

    extern "C" fn create(
        &mut self,
        size: u64,
        offset: u64,
        value: &mut Bytes32,
        remaining_gas: u64,
    ) -> *const RuntimeResult<()> {
        self.create_aux(size, offset, value, remaining_gas, None)
    }

    extern "C" fn create2(
        &mut self,
        size: u64,
        offset: u64,
        value: &mut Bytes32,
        remaining_gas: u64,
        salt: &Bytes32,
    ) -> *const RuntimeResult<()> {
        self.create_aux(size, offset, value, remaining_gas, Some(salt))
    }

    extern "C" fn selfdestruct(&mut self, receiver_address: &Bytes32) -> *const RuntimeResult<u64> {
        let receiver_address = Address::from(receiver_address);
        let result = match self
            .host
            .selfdestruct(self.contract.target_address, receiver_address)
        {
            Some(result) => result,
            None => {
                self.inner.result.error = ExitStatusCode::FatalExternalError.to_u8();
                return &self.inner.result as _;
            }
        };
        // EIP-3529: Reduction in refunds
        if !self.inner.spec_id.is_enabled_in(SpecId::LONDON) && !result.previously_destroyed {
            self.inner.gas_refunded += gas_cost::SELFDESTRUCT;
        }
        let gas_cost = gas::selfdestruct_cost(self.inner.spec_id, &result);

        self.inner.result.gas_used = gas_cost;
        &self.inner.result as _
    }

    extern "C" fn tload(&mut self, stg_key: &Bytes32, stg_value: &mut Bytes32) {
        let result = self.host.tload(self.contract.target_address, *stg_key);
        *stg_value = result;
    }

    extern "C" fn tstore(&mut self, stg_key: &Bytes32, stg_value: &mut Bytes32) {
        self.host
            .tstore(self.contract.target_address, *stg_key, *stg_value);
    }
}

type SymbolSignature = (&'static str, *const fn() -> ());

impl RuntimeContext<'_> {
    /// Registers all the syscalls as symbols in the execution engine.
    pub fn register_evm_symbols(engine: &ExecutionEngine) {
        unsafe {
            // Global variables and syscalls with corresponding function signatures
            let symbols_and_signatures: &[SymbolSignature] = &[
                // Debug functions
                (symbols::NOP, RuntimeContext::nop as *const _),
                (symbols::TRACING, RuntimeContext::tracing as *const _),
                // Syscalls
                (
                    symbols::WRITE_RESULT,
                    RuntimeContext::write_result as *const _,
                ),
                (
                    symbols::CTX_IS_STATIC,
                    RuntimeContext::ctx_is_static as *const _,
                ),
                (symbols::EXP, RuntimeContext::exp as *const _),
                (
                    symbols::KECCAK256_HASHER,
                    RuntimeContext::keccak256_hasher as *const _,
                ),
                (
                    symbols::EXTEND_MEMORY,
                    RuntimeContext::extend_memory as *const _,
                ),
                (symbols::MEMORY_PTR, RuntimeContext::memory_ptr as *const _),
                (
                    symbols::MEMORY_SIZE,
                    RuntimeContext::memory_size as *const _,
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
                (symbols::CALLDATA, RuntimeContext::calldata as *const _),
                (
                    symbols::CALLDATA_SIZE,
                    RuntimeContext::calldata_size as *const _,
                ),
                (
                    symbols::CALLDATA_COPY,
                    RuntimeContext::calldata_copy as *const _,
                ),
                (
                    symbols::DATA_SECTION,
                    RuntimeContext::data_section as *const _,
                ),
                (
                    symbols::DATA_SECTION_SIZE,
                    RuntimeContext::data_section_size as *const _,
                ),
                (
                    symbols::DATA_SECTION_COPY,
                    RuntimeContext::data_section_copy as *const _,
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
                (
                    symbols::STORE_IN_GASLIMIT_PTR,
                    RuntimeContext::store_in_gaslimit_ptr as *const _,
                ),
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
                (symbols::EOFCREATE, RuntimeContext::eofcreate as *const _),
                (
                    symbols::RETURNCONTRACT,
                    RuntimeContext::returncontract as *const _,
                ),
                (symbols::CREATE, RuntimeContext::create as *const _),
                (symbols::CREATE2, RuntimeContext::create2 as *const _),
                (symbols::CALL, RuntimeContext::call as *const _),
                (symbols::EXTCALL, RuntimeContext::extcall as *const _),
                (symbols::RETURNDATA, RuntimeContext::returndata as *const _),
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

    /// Registers all WASM libcalls as symbols in the execution engine.
    pub fn register_wasm_symbols(engine: &ExecutionEngine) {
        unsafe {
            let symbols_and_signatures: &[SymbolSignature] = &[
                (
                    symbols::wasm::TABLE_INIT,
                    wasmer_vm::libcalls::wasmer_vm_table_init as *const _,
                ),
                (
                    symbols::wasm::TABLE_COPY,
                    wasmer_vm::libcalls::wasmer_vm_table_copy as *const _,
                ),
                (
                    symbols::wasm::TABLE_FILL,
                    wasmer_vm::libcalls::wasmer_vm_table_fill as *const _,
                ),
                (
                    symbols::wasm::TABLE_SIZE,
                    wasmer_vm::libcalls::wasmer_vm_table_size as *const _,
                ),
                (
                    symbols::wasm::TABLE_GET,
                    wasmer_vm::libcalls::wasmer_vm_table_get as *const _,
                ),
                (
                    symbols::wasm::TABLE_SET,
                    wasmer_vm::libcalls::wasmer_vm_table_set as *const _,
                ),
                (
                    symbols::wasm::TABLE_GROW,
                    wasmer_vm::libcalls::wasmer_vm_table_grow as *const _,
                ),
                (
                    symbols::wasm::IMPORTED_TABLE_SIZE,
                    wasmer_vm::libcalls::wasmer_vm_imported_table_size as *const _,
                ),
                (
                    symbols::wasm::IMPORTED_TABLE_GET,
                    wasmer_vm::libcalls::wasmer_vm_imported_table_get as *const _,
                ),
                (
                    symbols::wasm::IMPORTED_TABLE_SET,
                    wasmer_vm::libcalls::wasmer_vm_imported_table_set as *const _,
                ),
                (
                    symbols::wasm::IMPORTED_TABLE_GROW,
                    wasmer_vm::libcalls::wasmer_vm_imported_table_grow as *const _,
                ),
                (
                    symbols::wasm::MEMORY_INIT,
                    wasmer_vm::libcalls::wasmer_vm_memory32_init as *const _,
                ),
                (
                    symbols::wasm::MEMORY_SIZE,
                    wasmer_vm::libcalls::wasmer_vm_memory32_size as *const _,
                ),
                (
                    symbols::wasm::MEMORY_GROW,
                    wasmer_vm::libcalls::wasmer_vm_memory32_grow as *const _,
                ),
                (
                    symbols::wasm::MEMORY_COPY,
                    wasmer_vm::libcalls::wasmer_vm_memory32_copy as *const _,
                ),
                (
                    symbols::wasm::MEMORY_FILL,
                    wasmer_vm::libcalls::wasmer_vm_memory32_fill as *const _,
                ),
                (
                    symbols::wasm::MEMORY_NOTIFY,
                    wasmer_vm::libcalls::wasmer_vm_memory32_atomic_notify as *const _,
                ),
                (
                    symbols::wasm::MEMORY_WAIT32,
                    wasmer_vm::libcalls::wasmer_vm_memory32_atomic_wait32 as *const _,
                ),
                (
                    symbols::wasm::MEMORY_WAIT64,
                    wasmer_vm::libcalls::wasmer_vm_memory32_atomic_wait64 as *const _,
                ),
                (
                    symbols::wasm::IMPORTED_MEMORY_SIZE,
                    wasmer_vm::libcalls::wasmer_vm_imported_memory32_size as *const _,
                ),
                (
                    symbols::wasm::IMPORTED_MEMORY_GROW,
                    wasmer_vm::libcalls::wasmer_vm_imported_memory32_grow as *const _,
                ),
                (
                    symbols::wasm::IMPORTED_MEMORY_COPY,
                    wasmer_vm::libcalls::wasmer_vm_imported_memory32_copy as *const _,
                ),
                (
                    symbols::wasm::IMPORTED_MEMORY_FILL,
                    wasmer_vm::libcalls::wasmer_vm_imported_memory32_fill as *const _,
                ),
                (
                    symbols::wasm::IMPORTED_MEMORY_NOTIFY,
                    wasmer_vm::libcalls::wasmer_vm_imported_memory32_atomic_notify as *const _,
                ),
                (
                    symbols::wasm::IMPORTED_MEMORY_WAIT32,
                    wasmer_vm::libcalls::wasmer_vm_imported_memory32_atomic_wait32 as *const _,
                ),
                (
                    symbols::wasm::IMPORTED_MEMORY_WAIT64,
                    wasmer_vm::libcalls::wasmer_vm_imported_memory32_atomic_wait64 as *const _,
                ),
                (
                    symbols::wasm::FUNC_REF,
                    wasmer_vm::libcalls::wasmer_vm_func_ref as *const _,
                ),
                (
                    symbols::wasm::DATA_DROP,
                    wasmer_vm::libcalls::wasmer_vm_data_drop as *const _,
                ),
                (
                    symbols::wasm::ELEM_DROP,
                    wasmer_vm::libcalls::wasmer_vm_elem_drop as *const _,
                ),
                (
                    symbols::wasm::RAISE_TRAP,
                    wasmer_vm::libcalls::wasmer_vm_raise_trap as *const _,
                ),
            ];

            for (symbol, signature) in symbols_and_signatures {
                engine.register_symbol(symbol, *signature as *mut ());
            }
        }
    }
}
