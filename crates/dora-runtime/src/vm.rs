use std::{
    cmp::Ordering,
    ops::{Deref, DerefMut},
};

use dora_primitives::{
    B256, Bytes, Cfg, Env, InvalidHeader, InvalidTransaction, SpecId, TransactionType, U256,
    eip4844,
};

use crate::{
    ExitStatusCode,
    account::Account,
    call::{CallKind, CallMessage, CallResult},
    context::VMContext,
    db::{Database, DatabaseError},
    gas::{self, InitialGas},
    result::{
        ExecutionResult, HaltReason, OutOfGasError, Output, ResultAndState, SuccessReason, VMError,
    },
};

/// EVM/WASM instance containing internal VM context and run actions
pub struct VM<'a, DB: Database> {
    pub context: VMContext<'a, DB>,
}

impl<'a, DB: Database> VM<'a, DB> {
    /// Create a new VM.
    pub fn new(context: VMContext<DB>) -> VM<DB> {
        VM { context }
    }

    /// Returns internal database and external struct.
    #[inline]
    pub fn into_context(self) -> VMContext<'a, DB> {
        self.context
    }

    /// Transact transaction
    ///
    /// This function will validate the transaction.
    #[inline]
    pub fn transact(&mut self) -> Result<ResultAndState, VMError> {
        let gas = self.preverify_transaction().inspect_err(|_| {
            self.clear();
        })?;

        let output = self.transact_preverified(gas);
        self.clear();
        output
    }

    /// Commit the changes to the database.
    pub fn transact_commit(&mut self) -> Result<ExecutionResult, VMError> {
        let ResultAndState { result, state } = self.transact()?;
        self.context.db.commit(state);
        Ok(result)
    }

    /// Pre verify transaction inner.
    #[inline]
    fn preverify_transaction(&mut self) -> Result<InitialGas, VMError> {
        self.validate_env()?;
        let gas = Self::validate_initial_tx_gas(&self.context.env, self.spec_id())?;
        self.validate_tx_against_state()?;
        Ok(gas)
    }

    fn validate_env(&mut self) -> Result<(), VMError> {
        let spec_id = self.spec_id();
        if spec_id.is_enabled_in(SpecId::MERGE) && self.env.block.prevrandao.is_none() {
            return Err(VMError::Header(InvalidHeader::PrevrandaoNotSet));
        }
        if spec_id.is_enabled_in(SpecId::CANCUN)
            && self.env.block.blob_excess_gas_and_price.is_none()
        {
            return Err(VMError::Header(InvalidHeader::ExcessBlobGasNotSet));
        }
        let tx_type = self.env.tx.tx_type;
        let base_fee = if self.env.cfg.is_base_fee_check_disabled() {
            None
        } else {
            Some(self.env.block.basefee as u128)
        };

        match TransactionType::from(tx_type) {
            TransactionType::Legacy => {
                // Check chain_id only if it is present in the legacy transaction.
                // EIP-155: Simple replay attack protection
                if let Some(chain_id) = self.env.tx.chain_id {
                    if chain_id != self.env.cfg.chain_id() {
                        return Err(VMError::Transaction(InvalidTransaction::InvalidChainId));
                    }
                }
                // Gas price must be at least the basefee.
                if let Some(base_fee) = base_fee {
                    if self.env.tx.gas_price < base_fee {
                        return Err(VMError::Transaction(
                            InvalidTransaction::GasPriceLessThanBasefee,
                        ));
                    }
                }
            }
            TransactionType::Eip2930 => {
                // Enabled in BERLIN hardfork
                if !spec_id.is_enabled_in(SpecId::BERLIN) {
                    return Err(VMError::Transaction(
                        InvalidTransaction::Eip2930NotSupported,
                    ));
                }

                if Some(self.env.cfg.chain_id()) != self.env.tx.chain_id {
                    return Err(VMError::Transaction(InvalidTransaction::InvalidChainId));
                }

                // Gas price must be at least the basefee.
                if let Some(base_fee) = base_fee {
                    if self.env.tx.gas_price < base_fee {
                        return Err(VMError::Transaction(
                            InvalidTransaction::GasPriceLessThanBasefee,
                        ));
                    }
                }
            }
            TransactionType::Eip1559 => {
                if !spec_id.is_enabled_in(SpecId::LONDON) {
                    return Err(VMError::Transaction(
                        InvalidTransaction::Eip1559NotSupported,
                    ));
                }
                if Some(self.env.cfg.chain_id()) != self.env.tx.chain_id {
                    return Err(VMError::Transaction(InvalidTransaction::InvalidChainId));
                }

                Self::validate_priority_fee_tx(
                    self.env.max_fee_per_gas(),
                    self.env.max_priority_fee_per_gas().unwrap_or_default(),
                    base_fee,
                )?;
            }
            TransactionType::Eip4844 => {
                if !spec_id.is_enabled_in(SpecId::CANCUN) {
                    return Err(VMError::Transaction(
                        InvalidTransaction::Eip4844NotSupported,
                    ));
                }

                if Some(self.env.cfg.chain_id()) != self.env.tx.chain_id {
                    return Err(VMError::Transaction(InvalidTransaction::InvalidChainId));
                }

                Self::validate_priority_fee_tx(
                    self.env.max_fee_per_gas(),
                    self.env.max_priority_fee_per_gas().unwrap_or_default(),
                    base_fee,
                )?;

                Self::validate_eip4844_tx(
                    self.env.blob_versioned_hashes(),
                    self.env.max_fee_per_blob_gas(),
                    self.env.blob_gasprice().unwrap_or_default(),
                    self.env.cfg.blob_max_count(spec_id),
                )?;
            }
            TransactionType::Eip7702 => {
                // Check if EIP-7702 transaction is enabled.
                if !spec_id.is_enabled_in(SpecId::PRAGUE) {
                    return Err(VMError::Transaction(
                        InvalidTransaction::Eip7702NotSupported,
                    ));
                }

                if Some(self.env.cfg.chain_id()) != self.env.tx.chain_id {
                    return Err(VMError::Transaction(InvalidTransaction::InvalidChainId));
                }

                Self::validate_priority_fee_tx(
                    self.env.max_fee_per_gas(),
                    self.env.max_priority_fee_per_gas().unwrap_or_default(),
                    base_fee,
                )?;

                // The transaction is considered invalid if the length of authorization_list is zero.
                if self.env.tx.authorization_list.is_empty() {
                    return Err(VMError::Transaction(
                        InvalidTransaction::EmptyAuthorizationList,
                    ));
                }
            }
            TransactionType::Custom => {
                // Custom transaction type check is not done here.
            }
        };

        // Check if gas_limit is more than block_gas_limit
        if !self.env.cfg.is_block_gas_limit_disabled()
            && self.env.tx.gas_limit > self.env.block.gas_limit
        {
            return Err(VMError::Transaction(
                InvalidTransaction::CallerGasLimitMoreThanBlock,
            ));
        }

        // EIP-3860: Limit and meter initcode
        if spec_id.is_enabled_in(SpecId::SHANGHAI) && self.env.tx.kind.is_create() {
            let max_initcode_size = self.env.cfg.max_code_size().saturating_mul(2);
            if self.env.tx.data.len() > max_initcode_size {
                return Err(VMError::Transaction(
                    InvalidTransaction::CreateInitCodeSizeLimit,
                ));
            }
        }

        Ok(())
    }

    /// Validate transaction that has EIP-1559 priority fee
    fn validate_priority_fee_tx(
        max_fee: u128,
        max_priority_fee: u128,
        base_fee: Option<u128>,
    ) -> Result<(), VMError> {
        if max_priority_fee > max_fee {
            // Or gas_max_fee for eip1559
            return Err(VMError::Transaction(
                InvalidTransaction::PriorityFeeGreaterThanMaxFee,
            ));
        }

        // Check minimal cost against basefee
        if let Some(base_fee) = base_fee {
            let effective_gas_price =
                std::cmp::min(max_fee, base_fee.saturating_add(max_priority_fee));
            if effective_gas_price < base_fee {
                return Err(VMError::Transaction(
                    InvalidTransaction::GasPriceLessThanBasefee,
                ));
            }
        }

        Ok(())
    }

    /// Validate EIP-4844 transaction.
    fn validate_eip4844_tx(
        blobs: &[B256],
        max_blob_fee: u128,
        block_blob_gas_price: u128,
        max_blobs: u8,
    ) -> Result<(), VMError> {
        // Ensure that the user was willing to at least pay the current blob gasprice
        if block_blob_gas_price > max_blob_fee {
            return Err(VMError::Transaction(
                InvalidTransaction::BlobGasPriceGreaterThanMax,
            ));
        }

        // There must be at least one blob
        if blobs.is_empty() {
            return Err(VMError::Transaction(InvalidTransaction::EmptyBlobs));
        }

        // All versioned blob hashes must start with VERSIONED_HASH_VERSION_KZG
        for blob in blobs {
            if blob[0] != eip4844::VERSIONED_HASH_VERSION_KZG {
                return Err(VMError::Transaction(
                    InvalidTransaction::BlobVersionNotSupported,
                ));
            }
        }

        // Ensure the total blob gas spent is at most equal to the limit
        // assert blob_gas_used <= MAX_BLOB_GAS_PER_BLOCK
        if blobs.len() > max_blobs as usize {
            return Err(VMError::Transaction(InvalidTransaction::TooManyBlobs {
                have: blobs.len(),
                max: max_blobs as usize,
            }));
        }
        Ok(())
    }

    /// Validates transaction against the state.
    #[inline]
    fn validate_tx_against_state(&mut self) -> Result<(), VMError> {
        let spec_id = self.spec_id();
        let tx_caller = self.context.env.tx.caller;
        let caller_account = self
            .context
            .journaled_state
            .load_code(&mut self.context.db, tx_caller)
            .map_err(|_| VMError::Database(DatabaseError))?;
        Self::validate_tx_against_account(caller_account.data, &self.context.env, spec_id)
            .map_err(VMError::Transaction)?;

        Ok(())
    }

    /// Validate initial transaction gas.
    fn validate_initial_tx_gas(env: &Env, spec_id: SpecId) -> Result<InitialGas, VMError> {
        let gas = gas::calculate_initial_tx_gas(
            spec_id,
            &env.tx.data,
            env.tx.kind.is_create(),
            &env.tx.access_list,
            env.tx.authorization_list.len() as u64,
        );
        // Additional check to see if limit is big enough to cover initial gas.
        if gas.initial_gas > env.tx.gas_limit {
            return Err(VMError::Transaction(
                InvalidTransaction::CallGasCostMoreThanGasLimit {
                    gas_limit: env.tx.gas_limit,
                    initial_gas: gas.initial_gas,
                },
            ));
        }
        // EIP-7623: Increase calldata cost
        // floor gas should be less than gas limit.
        if spec_id.is_enabled_in(SpecId::PRAGUE) && gas.floor_gas > env.tx.gas_limit {
            return Err(VMError::Transaction(
                InvalidTransaction::GasFloorMoreThanGasLimit {
                    gas_floor: gas.floor_gas,
                    gas_limit: env.tx.gas_limit,
                },
            ));
        };

        Ok(gas)
    }

    /// Validate account against the transaction.
    fn validate_tx_against_account(
        account: &mut Account,
        env: &Env,
        spec_id: SpecId,
    ) -> Result<(), InvalidTransaction> {
        if !env.cfg.is_eip3607_disabled() {
            let bytecode = &account.info.code.as_ref().unwrap();
            // Allow EOAs whose code is a valid delegation designation,
            // i.e. 0xef0100 || address, to continue to originate transactions.
            if !bytecode.is_empty() && !bytecode.is_eip7702() {
                return Err(InvalidTransaction::RejectCallerWithCode);
            }
        }
        // Check that the transaction's nonce is correct
        if !env.cfg.is_nonce_check_disabled() {
            let tx = env.tx.nonce;
            let state = account.info.nonce;
            match tx.cmp(&state) {
                Ordering::Greater => {
                    return Err(InvalidTransaction::NonceTooHigh { tx, state });
                }
                Ordering::Less => {
                    return Err(InvalidTransaction::NonceTooLow { tx, state });
                }
                _ => {}
            }
        }

        // gas_limit * max_fee + value
        let mut balance_check = U256::from(env.tx.gas_limit)
            .checked_mul(U256::from(env.max_fee_per_gas()))
            .and_then(|gas_cost| gas_cost.checked_add(env.tx.value))
            .ok_or(InvalidTransaction::OverflowPaymentInTransaction)?;

        if spec_id.is_enabled_in(SpecId::CANCUN) {
            // if the tx is not a blob tx, this will be None, so we add zero
            let data_fee = env.calc_max_data_fee();
            balance_check = balance_check
                .checked_add(data_fee)
                .ok_or(InvalidTransaction::OverflowPaymentInTransaction)?;
        }

        if balance_check > account.info.balance {
            if env.cfg.is_balance_check_disabled() {
                // Add transaction cost to balance to ensure execution doesn't fail.
                account.info.balance = account.info.balance.saturating_add(balance_check);
            } else {
                return Err(InvalidTransaction::LackOfFundForMaxFee {
                    fee: Box::new(balance_check),
                    balance: Box::new(account.info.balance),
                });
            }
        }

        Ok(())
    }

    /// Transact pre-verified transaction.
    fn transact_preverified(&mut self, gas: InitialGas) -> Result<ResultAndState, VMError> {
        let ctx = &mut self.context;
        // Pre execution
        let pre_exec_gas_refund = {
            // Load access list and beneficiary if needed.
            ctx.load_accounts()?;
            // Set precompile addresses into the warm preloaded address list.
            ctx.set_precompiles();
            // Deduce caller balance with its limit.
            ctx.deduct_caller()?;
            // Apply EIP-7702 auth list
            ctx.apply_eip7702_auth_list()?
        };

        // Execution
        let mut result = {
            let gas_limit = ctx.env.tx.gas_limit - gas.initial_gas;
            let call_msg = CallMessage {
                kind: if ctx.env.tx.kind.is_create() {
                    CallKind::Create
                } else {
                    CallKind::Call
                },
                input: ctx.env.tx.data.clone(),
                init_code: Bytes::new(),
                value: ctx.env.tx.value,
                depth: 0,
                gas_limit,
                caller: ctx.env.tx.caller,
                recipient: ctx.env.tx.kind.into_to().unwrap_or_default(),
                salt: None,
                code_address: ctx.env.tx.kind.into_to().unwrap_or_default(),
                is_static: false,
                is_eof_init: false,
                validate_eof: true,
            };
            let mut result = ctx.call(call_msg)?;
            ctx.last_frame_return(&mut result);
            result
        };

        // Post excution
        {
            // Record the pre execution refunded gas including EIP-7702, etc.
            result.record_refund(pre_exec_gas_refund as i64);
            // Set a refund value for final refund.
            result.set_final_refund(ctx.spec_id().is_enabled_in(SpecId::LONDON));
            // EIP-7623: Increase calldata cost spend at least a gas_floor amount of gas.
            if result.spent_sub_refunded() < gas.floor_gas {
                result.set_spent(gas.floor_gas);
                // Clear refund
                result.set_refund(0);
            }
            // Reimburse the caller with gas that were not used.
            ctx.reimburse_caller(result.gas_remaining, result.gas_refunded)?;
            // Reward beneficiary
            ctx.reward_beneficiary(result.gas_used(), result.gas_refunded)?;
        }
        // Returns output of transaction.
        Ok(self.output(result))
    }

    /// Build output using the call result
    pub fn output(&mut self, result: CallResult) -> ResultAndState {
        let gas_limit = self.env.tx.gas_limit;
        // Used gas with refund calculated.
        let gas_refunded = result.gas_refunded as u64;
        let gas_used = result.gas_used() - gas_refunded;

        let return_values = result.output.to_vec();
        let exit_status = result.status;
        // Reset journal and return present state.
        let (state, logs) = self.journaled_state.finalize();

        let result = match exit_status {
            ExitStatusCode::Continue | ExitStatusCode::Return => ExecutionResult::Success {
                reason: SuccessReason::Return,
                gas_used,
                gas_refunded,
                output: Output::Call(return_values.into()),
                logs,
            },
            ExitStatusCode::Stop => ExecutionResult::Success {
                reason: SuccessReason::Stop,
                gas_used,
                gas_refunded,
                output: Output::Call(return_values.into()),
                logs,
            },
            ExitStatusCode::Selfdestruct => ExecutionResult::Success {
                reason: SuccessReason::Selfdestruct,
                gas_used,
                gas_refunded,
                output: Output::Call(return_values.into()),
                logs,
            },
            ExitStatusCode::Suspend => ExecutionResult::Halt {
                reason: HaltReason::InvalidSuspend,
                gas_limit,
                gas_used,
            },
            ExitStatusCode::Revert
            | ExitStatusCode::CreateInitCodeStartingEF00
            | ExitStatusCode::InvalidEOFInitCode => ExecutionResult::Revert {
                output: return_values.into(),
                gas_used,
            },
            ExitStatusCode::CallTooDeep => ExecutionResult::Halt {
                reason: HaltReason::CallTooDeep,
                gas_limit,
                gas_used,
            },
            ExitStatusCode::OutOfFunds => ExecutionResult::Halt {
                reason: HaltReason::OutOfFunds,
                gas_limit,
                gas_used,
            },
            ExitStatusCode::OutOfGas => ExecutionResult::Halt {
                reason: HaltReason::OutOfGas(OutOfGasError::Basic),
                gas_limit,
                gas_used,
            },
            ExitStatusCode::MemoryOOG => ExecutionResult::Halt {
                reason: HaltReason::OutOfGas(OutOfGasError::Memory),
                gas_limit,
                gas_used,
            },
            ExitStatusCode::MemoryLimitOOG => ExecutionResult::Halt {
                reason: HaltReason::OutOfGas(OutOfGasError::MemoryLimit),
                gas_limit,
                gas_used,
            },
            ExitStatusCode::PrecompileOOG => ExecutionResult::Halt {
                reason: HaltReason::OutOfGas(OutOfGasError::Precompile),
                gas_limit,
                gas_used,
            },
            ExitStatusCode::InvalidOperandOOG => ExecutionResult::Halt {
                reason: HaltReason::OutOfGas(OutOfGasError::InvalidOperand),
                gas_limit,
                gas_used,
            },
            ExitStatusCode::OpcodeNotFound => ExecutionResult::Halt {
                reason: HaltReason::OpcodeNotFound,
                gas_limit,
                gas_used,
            },
            ExitStatusCode::CallNotAllowedInsideStatic => ExecutionResult::Halt {
                reason: HaltReason::CallNotAllowedInsideStatic,
                gas_limit,
                gas_used,
            },
            ExitStatusCode::StateChangeDuringStaticcall => ExecutionResult::Halt {
                reason: HaltReason::StateChangeDuringStaticcall,
                gas_limit,
                gas_used,
            },
            ExitStatusCode::InvalidFEOpcode => ExecutionResult::Halt {
                reason: HaltReason::InvalidFEOpcode,
                gas_limit,
                gas_used,
            },
            ExitStatusCode::InvalidJump => ExecutionResult::Halt {
                reason: HaltReason::InvalidJump,
                gas_limit,
                gas_used,
            },
            ExitStatusCode::NotActivated => ExecutionResult::Halt {
                reason: HaltReason::NotActivated,
                gas_limit,
                gas_used,
            },
            ExitStatusCode::StackUnderflow => ExecutionResult::Halt {
                reason: HaltReason::StackUnderflow,
                gas_limit,
                gas_used,
            },
            ExitStatusCode::StackOverflow => ExecutionResult::Halt {
                reason: HaltReason::StackOverflow,
                gas_limit,
                gas_used,
            },
            ExitStatusCode::OutOfOffset => ExecutionResult::Halt {
                reason: HaltReason::OutOfOffset,
                gas_limit,
                gas_used,
            },
            ExitStatusCode::CreateCollision => ExecutionResult::Halt {
                reason: HaltReason::CreateCollision,
                gas_limit,
                gas_used,
            },
            ExitStatusCode::OverflowPayment => ExecutionResult::Halt {
                reason: HaltReason::OverflowPayment,
                gas_limit,
                gas_used,
            },
            ExitStatusCode::PrecompileError => ExecutionResult::Halt {
                reason: HaltReason::PrecompileError,
                gas_limit,
                gas_used,
            },
            ExitStatusCode::NonceOverflow => ExecutionResult::Halt {
                reason: HaltReason::NonceOverflow,
                gas_limit,
                gas_used,
            },
            ExitStatusCode::CreateContractSizeLimit => ExecutionResult::Halt {
                reason: HaltReason::CreateContractSizeLimit,
                gas_limit,
                gas_used,
            },
            ExitStatusCode::CreateContractStartingWithEF => ExecutionResult::Halt {
                reason: HaltReason::CreateContractStartingWithEF,
                gas_limit,
                gas_used,
            },
            ExitStatusCode::CreateInitCodeSizeLimit => ExecutionResult::Halt {
                reason: HaltReason::CreateInitCodeSizeLimit,
                gas_limit,
                gas_used,
            },
            ExitStatusCode::EOFOpcodeDisabledInLegacy
            | ExitStatusCode::ReturnContractInNotInitEOF => ExecutionResult::Halt {
                reason: HaltReason::OpcodeNotFound,
                gas_limit,
                gas_used,
            },
            ExitStatusCode::EOFFunctionStackOverflow => ExecutionResult::Halt {
                reason: HaltReason::EOFFunctionStackOverflow,
                gas_limit,
                gas_used,
            },
            ExitStatusCode::EofAuxDataOverflow => ExecutionResult::Halt {
                reason: HaltReason::EofAuxDataOverflow,
                gas_limit,
                gas_used,
            },
            ExitStatusCode::EofAuxDataTooSmall => ExecutionResult::Halt {
                reason: HaltReason::EofAuxDataTooSmall,
                gas_limit,
                gas_used,
            },
            ExitStatusCode::InvalidExtCallTarget => ExecutionResult::Halt {
                reason: HaltReason::InvalidExtCallTarget,
                gas_limit,
                gas_used,
            },
            ExitStatusCode::InvalidExtDelegatecallTarget => ExecutionResult::Halt {
                reason: HaltReason::InvalidExtDelegatecallTarget,
                gas_limit,
                gas_used,
            },
            ExitStatusCode::FatalExternalError => ExecutionResult::FatalExternalError,
        };

        ResultAndState { result, state }
    }

    #[inline]
    fn clear(&mut self) {
        self.context.journaled_state.clear();
    }
}

impl<'a, DB: Database> Deref for VM<'a, DB> {
    type Target = VMContext<'a, DB>;

    fn deref(&self) -> &Self::Target {
        &self.context
    }
}

impl<DB: Database> DerefMut for VM<'_, DB> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.context
    }
}
