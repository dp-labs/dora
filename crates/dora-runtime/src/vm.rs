use std::{
    cmp::Ordering,
    ops::{Deref, DerefMut},
};

use dora_primitives::{SpecId, U256};

use crate::{
    account::Account,
    call::{CallKind, CallMessage, CallResult},
    constants::env::DORA_TRACING,
    context::VMContext,
    db::{Database, DatabaseError},
    env::{CfgEnv, TxEnv},
    result::{
        EVMError, ExecutionResult, HaltReason, InvalidTransaction, OutOfGasError, Output,
        ResultAndState, SuccessReason,
    },
    transaction::TransactionType,
    ExitStatusCode,
};

/// EVM instance containing internal VM context and run actions
pub struct VM<'a, DB: Database> {
    pub context: VMContext<'a, DB>,
}

impl<'a, DB: Database> VM<'a, DB> {
    /// Create new EVM.
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
    pub fn transact(&mut self) -> Result<ResultAndState, EVMError> {
        let initial_gas_cost = self.preverify_transaction().inspect_err(|_| {
            self.clear();
        })?;

        let output = self.transact_preverified(initial_gas_cost);
        self.clear();
        output
    }

    /// Commit the changes to the database.
    pub fn transact_commit(&mut self) -> Result<ExecutionResult, EVMError> {
        let ResultAndState { result, state } = self.transact()?;
        self.context.db.commit(state);
        Ok(result)
    }

    /// Pre verify transaction inner.
    #[inline]
    fn preverify_transaction(&mut self) -> Result<u64, EVMError> {
        self.context.env.validate_transaction()?;
        let initial_gas_cost = self.context.env.validate_initial_tx_gas(self.spec_id())?;
        self.validate_tx_against_state()?;
        Ok(initial_gas_cost)
    }

    /// Validates transaction against the state.
    #[inline]
    fn validate_tx_against_state(&mut self) -> Result<(), EVMError> {
        let tx_caller = self.context.env.tx.caller;
        let caller_account = self
            .context
            .journaled_state
            .load_code(tx_caller, &mut self.context.db)
            .map_err(|_| EVMError::Database(DatabaseError))?;

        Self::validate_tx_against_account(
            caller_account.data,
            &self.context.env.tx,
            &self.context.env.cfg,
        )
        .map_err(EVMError::Transaction)?;

        Ok(())
    }

    /// Validate account against the transaction.
    fn validate_tx_against_account(
        account: &mut Account,
        tx: &TxEnv,
        cfg: &CfgEnv,
    ) -> Result<(), InvalidTransaction> {
        // Check that the transaction's nonce is correct
        if !cfg.is_nonce_check_disabled() {
            let tx = tx.nonce;
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
        let mut balance_check = U256::from(tx.gas_limit)
            .checked_mul(U256::from(tx.max_fee()))
            .and_then(|gas_cost| gas_cost.checked_add(tx.value))
            .ok_or(InvalidTransaction::OverflowPaymentInTransaction)?;

        if tx.tx_type == TransactionType::Eip4844 {
            // if the tx is not a blob tx, this will be None, so we add zero
            let data_fee = tx.calc_max_data_fee().unwrap_or_default();
            balance_check = balance_check
                .checked_add(data_fee)
                .ok_or(InvalidTransaction::OverflowPaymentInTransaction)?;
        }

        if balance_check > account.info.balance {
            if cfg.is_balance_check_disabled() {
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
    fn transact_preverified(&mut self, initial_gas_cost: u64) -> Result<ResultAndState, EVMError> {
        let ctx = &mut self.context;
        // Pre execution
        {
            // Load access list and beneficiary if needed.
            ctx.load_accounts()?;
            // Set precompile addresses into the warm preloaded address list.
            ctx.set_precompiles();
            // Deduce caller balance with its limit.
            ctx.deduct_caller()?;
        }

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
                recipient: ctx.env.tx.get_address(),
                salt: None,
                code_address: ctx.env.tx.get_address(),
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
            // Calculate final refund and add EIP-7702 refund to gas.
            let eip7702_gas_refund = 0;
            result.gas_refunded += eip7702_gas_refund;
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
        let gas_used = result.gas_used();
        let gas_refunded = result.gas_refunded as u64;

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
            ExitStatusCode::SelfDestruct => ExecutionResult::Success {
                reason: SuccessReason::SelfDestruct,
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
