use std::{
    cmp::Ordering,
    ops::{Deref, DerefMut},
};

use dora_primitives::{Env, InvalidTransaction, SpecId, U256, spec_to_generic};

use crate::{
    ExitStatusCode,
    account::Account,
    call::{CallKind, CallMessage, CallResult},
    constants::env::DORA_TRACING,
    context::VMContext,
    db::{Database, DatabaseError},
    gas,
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
        let initial_gas_cost = self.preverify_transaction().inspect_err(|_| {
            self.clear();
        })?;

        let output = self.transact_preverified(initial_gas_cost);
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
    fn preverify_transaction(&mut self) -> Result<u64, VMError> {
        let spec_id = self.spec_id();
        spec_to_generic!(spec_id, self.env.validate_block_env::<SPEC>())?;
        spec_to_generic!(spec_id, self.env.validate_tx::<SPEC>())?;
        let initial_gas_cost = Self::validate_initial_tx_gas(&self.context.env, self.spec_id())?;
        self.validate_tx_against_state()?;
        Ok(initial_gas_cost)
    }

    /// Validates transaction against the state.
    #[inline]
    fn validate_tx_against_state(&mut self) -> Result<(), VMError> {
        let spec_id = self.spec_id();
        let tx_caller = self.context.env.tx.caller;
        let caller_account = self
            .context
            .journaled_state
            .load_code(tx_caller, &mut self.context.db)
            .map_err(|_| VMError::Database(DatabaseError))?;
        Self::validate_tx_against_account(caller_account.data, &self.context.env, spec_id)
            .map_err(VMError::Transaction)?;

        Ok(())
    }

    /// Validate initial transaction gas.
    fn validate_initial_tx_gas(env: &Env, spec_id: SpecId) -> Result<u64, VMError> {
        let is_create = env.tx.transact_to.is_create();
        let authorization_list_num = env
            .tx
            .authorization_list
            .as_ref()
            .map(|l| l.len() as u64)
            .unwrap_or_default();
        let initial_gas_cost = gas::validate_initial_tx_gas(
            spec_id,
            &env.tx.data,
            is_create,
            &env.tx.access_list,
            authorization_list_num,
        );
        // Additional check to see if limit is big enough to cover initial gas.
        if initial_gas_cost > env.tx.gas_limit {
            return Err(VMError::Transaction(
                InvalidTransaction::CallGasCostMoreThanGasLimit,
            ));
        }
        Ok(initial_gas_cost)
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
        if let Some(tx) = env.tx.nonce {
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
            .checked_mul(env.tx.gas_price)
            .and_then(|gas_cost| gas_cost.checked_add(env.tx.value))
            .ok_or(InvalidTransaction::OverflowPaymentInTransaction)?;

        if spec_id.is_enabled_in(SpecId::CANCUN) {
            // if the tx is not a blob tx, this will be None, so we add zero
            let data_fee = env.calc_max_data_fee().unwrap_or_default();
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
    fn transact_preverified(&mut self, initial_gas_cost: u64) -> Result<ResultAndState, VMError> {
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
            let gas_limit = ctx.env.tx.gas_limit - initial_gas_cost;
            let call_msg = CallMessage {
                kind: if ctx.env.tx.transact_to.is_create() {
                    CallKind::Create
                } else {
                    CallKind::Call
                },
                input: ctx.env.tx.data.clone(),
                value: ctx.env.tx.value,
                depth: 0,
                gas_limit,
                caller: ctx.env.tx.caller,
                recipient: ctx.env.tx.transact_to.into_to().unwrap_or_default(),
                salt: None,
                code_address: ctx.env.tx.transact_to.into_to().unwrap_or_default(),
                is_static: false,
                is_eof_init: false,
                validate_eof: true,
            };
            if std::env::var(DORA_TRACING).is_ok() {
                println!("info: tx call msg {:?}", call_msg);
            }
            let mut result = ctx.call(call_msg)?;
            ctx.last_frame_return(&mut result);
            if std::env::var(DORA_TRACING).is_ok() {
                println!("info: tx call ret {:?}", result);
            }
            result
        };

        // Post excution
        {
            // Record the pre execution refunded gas including EIP-7702, etc.
            result.record_refund(pre_exec_gas_refund as i64);
            // Set a refund value for final refund.
            result.set_final_refund(ctx.spec_id().is_enabled_in(SpecId::LONDON));
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
