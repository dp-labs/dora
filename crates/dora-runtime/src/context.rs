use crate::account::AccountInfo;
use crate::constants::gas_cost::MAX_CODE_SIZE;
use crate::constants::{call_opcode, gas_cost, precompiles, CallType, CALL_STACK_LIMIT};
use crate::db::{Database, StorageSlot};
use crate::executor::ExecutionEngine;
use crate::gas;
use crate::host::Host;
use crate::precompiles::{blake2f, ecrecover, identity, modexp, ripemd_160, sha2_256};
use crate::result::{
    EVMError, ExecutionResult, HaltReason, InternalResult, OutOfGasError, Output, ResultAndState,
    SuccessReason,
};
use crate::transaction::Transaction;
use crate::{symbols, ExitStatusCode};
use bytes::Bytes;
use dora_primitives::spec::SpecId;
use dora_primitives::{Bytes32, EVMAddress as Address, B256, H160, U256};
use sha3::{Digest, Keccak256};
use std::sync::{Arc, RwLock};

/// Function type for the main entrypoint of the generated code.
pub type MainFunc<DB> = extern "C" fn(&mut RuntimeContext<DB>, initial_gas: u64) -> u8;

/// The internal execution context, which holds the memory, gas, and program state during contract execution.
///
/// This struct contains critical data used to manage the execution environment of smart contracts
/// or other EVM-related programs. It tracks the execution memory, return data, remaining gas, logs, and exit status.
///
/// # Fields:
/// - `memory`: A vector representing the contract's memory during execution.
/// - `return_data`: Optional tuple representing the start and length of the return data.
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
///     return_data: None,
///     program: vec![0x60, 0x0A, 0x60, 0x00],  // Sample bytecode
///     gas_remaining: Some(100000),
///     gas_refund: 0,
///     exit_status: None,
///     logs: Vec::new(),
/// };
/// ```
#[derive(Debug, Default)]
pub struct InnerContext {
    /// Represents the mutable, byte-addressable memory used during contract execution.
    /// This memory is accessible by smart contracts for reading and writing data.
    memory: Vec<u8>,
    /// Contains a tuple with the start index and length of the return data in memory.
    /// This data is returned when a VM operation completes, such as after a `RETURN` or a
    /// contract call, allowing the caller to process the output of the executed contract.
    return_data: Option<(usize, usize)>,
    /// The smart contract's bytecode being executed.
    pub program: Vec<u8>,
    /// The remaining gas for the current execution.
    gas_remaining: Option<u64>,
    /// The total gas to be refunded at the end of execution.
    gas_refund: u64,
    /// The exit status code of the VM execution.
    exit_status: Option<ExitStatusCode>,
    /// Logs captures all log entries emitted by the contract, which can be used for event tracking
    /// and off-chain data analysis. Logs are essential for notifying external observers about
    /// significant events that occurred during contract execution.
    logs: Vec<LogData>,
    /// Depth in the call stack.
    pub depth: usize,
    /// Whether the context is static.
    pub is_static: bool,
    /// Whether the context is EOF init.
    pub is_eof_init: bool,
    /// VM spec id
    pub spec_id: SpecId,
}

/// A frame of execution representing a single call within a smart contract execution context.
///
/// The `CallFrame` struct holds information about the caller of the contract and the data returned
/// from the last call made by this contract. It's part of the execution stack that is used to manage
/// nested contract calls.
///
/// # Fields:
/// - `caller`: The address of the account that initiated the call.
/// - `last_call_return_data`: The data returned by the last call executed in the current frame.
///
/// # Example Usage:
/// ```no_check
/// let call_frame = CallFrame {
///     caller: Address::from_low_u64_be(0x123),
///     last_call_return_data: vec![0x01, 0x02, 0x03],
/// };
/// ```
#[derive(Debug, Default)]
pub struct CallFrame {
    pub caller: Address,
    ctx_is_static: bool,
    last_call_return_data: Vec<u8>,
}

impl CallFrame {
    pub fn new(caller: Address) -> Self {
        Self {
            caller,
            ctx_is_static: false,
            last_call_return_data: Vec::new(),
        }
    }

    pub fn new_with_data(caller: Address, data: Vec<u8>) -> Self {
        Self {
            caller,
            ctx_is_static: false,
            last_call_return_data: data,
        }
    }
}

pub type RuntimeTransaction<DB> =
    Arc<dyn Transaction<Context = RuntimeContext<DB>, Result = anyhow::Result<ResultAndState>>>;
pub type RuntimeHost = Arc<RwLock<dyn Host>>;
pub type RuntimeDB<DB> = Arc<RwLock<DB>>;

/// The runtime context for smart contract execution, encapsulating the environment and execution state.
///
/// The `RuntimeContext` struct holds all the necessary information required during the execution of a contract.
/// It tracks the environment, execution journal, current call frame, inner execution context, and transient storage.
/// This is a core struct used in contract execution to manage the overall execution state.
///
/// # Fields:
/// - `env`: The execution environment, which contains information such as block number, gas price, and chain ID.
/// - `journal`: The journal of changes made during contract execution, used for rollback in case of failure.
/// - `call_frame`: The current call frame representing the contract call stack.
/// - `inner_context`: The inner execution context that holds memory, gas, logs, and other runtime-specific data.
/// - `transient_storage`: A temporary storage map used during execution, mapping addresses and keys to values.
pub struct RuntimeContext<DB: Database> {
    pub call_frame: CallFrame,
    pub inner_context: InnerContext,
    pub transaction: RuntimeTransaction<DB>,
    pub host: RuntimeHost,
    pub db: RuntimeDB<DB>,
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
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct LogData {
    pub topics: Vec<U256>,
    pub data: Vec<u8>,
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
pub struct Result<T> {
    /// The error, if any, encountered during execution.
    pub error: u8,
    /// The gas consumed during the execution of the function call.
    pub gas_used: u64,
    /// The result value of the function call. None indicates no value returned.
    pub value: T,
}

impl<T> Result<T> {
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

macro_rules! uint_result_ptr {
    ($result:expr) => {
        Box::into_raw(Box::new(Result::success($result)))
    };
}

/// Accessors for managing and retrieving execution results in a runtime context.
impl<DB: Database> RuntimeContext<DB> {
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
    pub fn new(
        db: RuntimeDB<DB>,
        call_frame: CallFrame,
        transaction: RuntimeTransaction<DB>,
        host: RuntimeHost,
        spec_id: SpecId,
    ) -> Self {
        Self {
            db,
            call_frame,
            inner_context: InnerContext {
                spec_id,
                ..Default::default()
            },
            transaction,
            host,
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
    /// let return_data = context.return_values();
    /// ```
    pub fn return_values(&self) -> &[u8] {
        if let Some((offset, size)) = self.inner_context.return_data {
            &self.inner_context.memory[offset..offset + size]
        } else {
            &[]
        }
    }

    /// Retrieves the logs generated during execution.
    ///
    /// This function converts the internal log data into a vector of `Log` objects, associating each log with the transaction caller.
    ///
    /// # Returns
    ///
    /// - `Vec<Log>`: A vector of logs created during execution, each containing the caller's address and the log data.
    ///
    /// # Example
    ///
    /// ```no_check
    /// let logs = context.logs();
    /// ```
    pub fn logs(&self) -> Vec<Log> {
        let host = self.host.read().unwrap();
        self.inner_context
            .logs
            .iter()
            .map(|logdata| Log {
                address: host.env().tx.caller,
                data: logdata.clone(),
            })
            .collect()
    }

    /// Retrieves the result of the execution, including gas usage, return values, and the resulting state changes.
    ///
    /// The result depends on the exit status of the execution, which can be a success, revert, or error.
    ///
    /// # Returns
    ///
    /// - `Result<ResultAndState, EVMError>`: A `Result` containing the execution result and the updated state if successful,
    ///   or an `EVMError` if the execution fails.
    ///
    /// # Example
    ///
    /// ```no_check
    /// let result = context.get_result();
    /// ```
    pub fn get_result(&self) -> anyhow::Result<ResultAndState, EVMError> {
        let host = self.host.read().unwrap();
        let gas_remaining = self.inner_context.gas_remaining.unwrap_or(0);
        let gas_initial = host.env().tx.gas_limit;
        let gas_used = gas_initial.saturating_sub(gas_remaining);
        let gas_refunded = self.inner_context.gas_refund;

        let return_values = self.return_values().to_vec();
        let exit_status = self
            .inner_context
            .exit_status
            .clone()
            .unwrap_or(ExitStatusCode::Stop);

        let result = match exit_status {
            ExitStatusCode::Return => ExecutionResult::Success {
                reason: SuccessReason::Return,
                gas_used,
                gas_refunded,
                output: Output::Call(return_values.into()),
                logs: self.logs(),
            },
            ExitStatusCode::Stop => ExecutionResult::Success {
                reason: SuccessReason::Stop,
                gas_used,
                gas_refunded,
                output: Output::Call(return_values.into()),
                logs: self.logs(),
            },
            ExitStatusCode::Revert
            | ExitStatusCode::CreateInitCodeStartingEF00
            | ExitStatusCode::InvalidEOFInitCode => ExecutionResult::Revert {
                output: return_values.into(),
                gas_used,
            },
            ExitStatusCode::CallTooDeep => ExecutionResult::Halt {
                reason: HaltReason::CallTooDeep,
                gas_used,
            },
            ExitStatusCode::OutOfFunds => ExecutionResult::Halt {
                reason: HaltReason::OutOfFunds,
                gas_used,
            },
            ExitStatusCode::OutOfGas => ExecutionResult::Halt {
                reason: HaltReason::OutOfGas(OutOfGasError::Basic),
                gas_used,
            },
            ExitStatusCode::MemoryOOG => ExecutionResult::Halt {
                reason: HaltReason::OutOfGas(OutOfGasError::Memory),
                gas_used,
            },
            ExitStatusCode::MemoryLimitOOG => ExecutionResult::Halt {
                reason: HaltReason::OutOfGas(OutOfGasError::MemoryLimit),
                gas_used,
            },
            ExitStatusCode::PrecompileOOG => ExecutionResult::Halt {
                reason: HaltReason::OutOfGas(OutOfGasError::Precompile),
                gas_used,
            },
            ExitStatusCode::InvalidOperandOOG => ExecutionResult::Halt {
                reason: HaltReason::OutOfGas(OutOfGasError::InvalidOperand),
                gas_used,
            },
            ExitStatusCode::OpcodeNotFound => ExecutionResult::Halt {
                reason: HaltReason::OpcodeNotFound,
                gas_used,
            },
            ExitStatusCode::CallNotAllowedInsideStatic => ExecutionResult::Halt {
                reason: HaltReason::CallNotAllowedInsideStatic,
                gas_used,
            },
            ExitStatusCode::StateChangeDuringStaticCall => ExecutionResult::Halt {
                reason: HaltReason::StateChangeDuringStaticCall,
                gas_used,
            },
            ExitStatusCode::InvalidFEOpcode => ExecutionResult::Halt {
                reason: HaltReason::InvalidFEOpcode,
                gas_used,
            },
            ExitStatusCode::InvalidJump => ExecutionResult::Halt {
                reason: HaltReason::InvalidJump,
                gas_used,
            },
            ExitStatusCode::NotActivated => ExecutionResult::Halt {
                reason: HaltReason::NotActivated,
                gas_used,
            },
            ExitStatusCode::StackUnderflow => ExecutionResult::Halt {
                reason: HaltReason::StackUnderflow,
                gas_used,
            },
            ExitStatusCode::StackOverflow => ExecutionResult::Halt {
                reason: HaltReason::StackOverflow,
                gas_used,
            },
            ExitStatusCode::OutOfOffset => ExecutionResult::Halt {
                reason: HaltReason::OutOfOffset,
                gas_used,
            },
            ExitStatusCode::CreateCollision => ExecutionResult::Halt {
                reason: HaltReason::CreateCollision,
                gas_used,
            },
            ExitStatusCode::OverflowPayment => ExecutionResult::Halt {
                reason: HaltReason::OverflowPayment,
                gas_used,
            },
            ExitStatusCode::PrecompileError => ExecutionResult::Halt {
                reason: HaltReason::PrecompileError,
                gas_used,
            },
            ExitStatusCode::NonceOverflow => ExecutionResult::Halt {
                reason: HaltReason::NonceOverflow,
                gas_used,
            },
            ExitStatusCode::CreateContractSizeLimit => ExecutionResult::Halt {
                reason: HaltReason::CreateContractSizeLimit,
                gas_used,
            },
            ExitStatusCode::CreateContractStartingWithEF => ExecutionResult::Halt {
                reason: HaltReason::CreateContractStartingWithEF,
                gas_used,
            },
            ExitStatusCode::CreateInitCodeSizeLimit => ExecutionResult::Halt {
                reason: HaltReason::CreateInitCodeSizeLimit,
                gas_used,
            },
            ExitStatusCode::EOFOpcodeDisabledInLegacy
            | ExitStatusCode::ReturnContractInNotInitEOF => ExecutionResult::Halt {
                reason: HaltReason::OpcodeNotFound,
                gas_used,
            },
            ExitStatusCode::EOFFunctionStackOverflow => ExecutionResult::Halt {
                reason: HaltReason::EOFFunctionStackOverflow,
                gas_used,
            },
            ExitStatusCode::EofAuxDataOverflow => ExecutionResult::Halt {
                reason: HaltReason::EofAuxDataOverflow,
                gas_used,
            },
            ExitStatusCode::EofAuxDataTooSmall => ExecutionResult::Halt {
                reason: HaltReason::EofAuxDataTooSmall,
                gas_used,
            },
            ExitStatusCode::InvalidExtCallTarget => ExecutionResult::Halt {
                reason: HaltReason::InvalidExtCallTarget,
                gas_used,
            },
            ExitStatusCode::InvalidExtDelegateCallTarget => ExecutionResult::Internal {
                result: InternalResult::InvalidExtDelegateCallTarget,
                gas_used,
            },
        };

        let host = self.host.read().unwrap();
        let mut state = self.db.read().unwrap().clone().into_state();
        let callee_address = host.env().tx.get_address();

        state.entry(callee_address).or_default().storage.extend(
            host.access_storage()
                .iter()
                .map(|(k, v)| (k.to_u256(), StorageSlot::from(v.to_u256()))),
        );

        Ok(ResultAndState { result, state })
    }
}

// System call functions
impl<DB: Database> RuntimeContext<DB> {
    pub extern "C" fn write_result(
        &mut self,
        offset: u64,
        bytes_len: u64,
        remaining_gas: u64,
        execution_result: u8,
    ) -> *mut Result<()> {
        self.inner_context.return_data = Some((offset as usize, bytes_len as usize));
        self.inner_context.gas_remaining = Some(remaining_gas);
        self.inner_context.exit_status = Some(ExitStatusCode::from_u8(execution_result));
        Box::into_raw(Box::new(Result::success(())))
    }

    pub extern "C" fn return_data_size(&mut self) -> *mut Result<u64> {
        uint_result_ptr!(self.call_frame.last_call_return_data.len() as u64)
    }

    pub extern "C" fn return_data_copy(
        &mut self,
        dest_offset: u64,
        offset: u64,
        size: u64,
    ) -> *mut Result<()> {
        Self::copy_exact(
            &mut self.inner_context.memory,
            &self.call_frame.last_call_return_data,
            dest_offset,
            offset,
            size,
        )
    }

    pub extern "C" fn call(
        &mut self,
        mut gas_to_send: u64,
        call_to_address: &Bytes32,
        value_to_transfer: &Bytes32,
        args_offset: u64,
        args_size: u64,
        ret_offset: u64,
        ret_size: u64,
        available_gas: u64,
        consumed_gas: &mut u64,
        call_type: u8,
    ) -> *mut Result<u8> {
        let callee_address = Address::from(call_to_address);
        let off = args_offset as usize;
        let size = args_size as usize;
        let calldata = Bytes::copy_from_slice(&self.inner_context.memory[off..off + size]);
        let value = value_to_transfer.to_u256();
        let (return_code, return_data) = match callee_address {
            x if x == Address::from_low_u64_be(precompiles::ECRECOVER_ADDRESS) => {
                ecrecover(&calldata, gas_to_send, consumed_gas).map_or_else(
                    |_err| (call_opcode::REVERT_RETURN_CODE, Bytes::default()),
                    |output: Bytes| (call_opcode::SUCCESS_RETURN_CODE, output),
                )
            }
            x if x == Address::from_low_u64_be(precompiles::IDENTITY_ADDRESS) => {
                identity(&calldata, gas_to_send, consumed_gas).map_or_else(
                    |_err| (call_opcode::REVERT_RETURN_CODE, Bytes::default()),
                    |output: Bytes| (call_opcode::SUCCESS_RETURN_CODE, output),
                )
            }
            x if x == Address::from_low_u64_be(precompiles::SHA2_256_ADDRESS) => {
                sha2_256(&calldata, gas_to_send, consumed_gas).map_or_else(
                    |_err| (call_opcode::REVERT_RETURN_CODE, Bytes::default()),
                    |output: Bytes| (call_opcode::SUCCESS_RETURN_CODE, output),
                )
            }
            x if x == Address::from_low_u64_be(precompiles::RIPEMD_160_ADDRESS) => {
                ripemd_160(&calldata, gas_to_send, consumed_gas).map_or_else(
                    |_err| (call_opcode::REVERT_RETURN_CODE, Bytes::default()),
                    |output: Bytes| (call_opcode::SUCCESS_RETURN_CODE, output),
                )
            }
            x if x == Address::from_low_u64_be(precompiles::MODEXP_ADDRESS) => {
                modexp(&calldata, gas_to_send, consumed_gas).map_or_else(
                    |_err| (call_opcode::REVERT_RETURN_CODE, Bytes::default()),
                    |output: Bytes| (call_opcode::SUCCESS_RETURN_CODE, output),
                )
            }
            x if x == Address::from_low_u64_be(precompiles::BLAKE2F_ADDRESS) => {
                blake2f(&calldata, gas_to_send, consumed_gas).map_or_else(
                    |_err| (call_opcode::REVERT_RETURN_CODE, Bytes::default()),
                    |output: Bytes| (call_opcode::SUCCESS_RETURN_CODE, output),
                )
            }
            _ => {
                let call_type = CallType::try_from(call_type)
                    .expect("Error while parsing CallType on call syscall");

                // Check the call depth
                if self.inner_context.depth > CALL_STACK_LIMIT {
                    *consumed_gas = 0;
                    return uint_result_ptr!(call_opcode::REVERT_RETURN_CODE);
                }

                let mut db = self.db.write().unwrap();
                // Retrieve or create the callee account in journal
                let callee_account = match db.basic(callee_address) {
                    Ok(acc) => {
                        *consumed_gas = call_opcode::WARM_MEMORY_ACCESS_COST;
                        acc.unwrap_or_else(AccountInfo::empty)
                    }
                    Err(_) => {
                        *consumed_gas = 0;
                        return uint_result_ptr!(call_opcode::REVERT_RETURN_CODE);
                    }
                };
                let host = self.host.read().unwrap();
                let caller_address = host.env().tx.get_address();
                let caller_account = db.basic(caller_address).unwrap().unwrap_or_default();

                let mut stipend = 0;
                if !value.is_zero() {
                    if caller_account.balance < value {
                        return uint_result_ptr!(call_opcode::REVERT_RETURN_CODE);
                    }
                    *consumed_gas += call_opcode::NOT_ZERO_VALUE_COST;
                    if callee_account.is_empty() {
                        *consumed_gas += call_opcode::EMPTY_CALLEE_COST;
                    }
                    if available_gas < *consumed_gas {
                        return uint_result_ptr!(call_opcode::REVERT_RETURN_CODE);
                    }
                    stipend = call_opcode::STIPEND_GAS_ADDITION;

                    let caller_balance = caller_account.balance;
                    let caller_nonce = caller_account.nonce;
                    db.set_account(
                        caller_address,
                        caller_nonce,
                        caller_balance - value,
                        Default::default(),
                    );

                    let callee_balance = callee_account.balance;
                    let callee_nonce = callee_account.nonce;
                    db.set_account(
                        callee_address,
                        callee_nonce,
                        callee_balance + value,
                        Default::default(),
                    );
                }

                let remaining_gas = available_gas - *consumed_gas;
                gas_to_send = std::cmp::min(
                    remaining_gas / call_opcode::GAS_CAP_DIVISION_FACTOR,
                    gas_to_send,
                );
                *consumed_gas += gas_to_send;
                gas_to_send += stipend;

                let this_address = host.env().tx.get_address();
                let (new_frame_caller, new_value, transact_to) = match call_type {
                    CallType::Call | CallType::StaticCall => (this_address, value, callee_address),
                    CallType::CallCode => (this_address, value, this_address),
                    CallType::DelegateCall => (
                        self.call_frame.caller,
                        Bytes32::from_u256_ref(&host.env().tx.value).to_u256(),
                        this_address,
                    ),
                };
                drop(host);
                let host = self.host.clone();
                let mut host_ref = host.write().unwrap();
                let new_env = host_ref.env_mut();
                new_env.tx.value = new_value;
                new_env.tx.transact_to = transact_to;
                new_env.tx.gas_limit = gas_to_send;
                let off = args_offset as usize;
                let size = args_size as usize;
                new_env.tx.data = Bytes::from(self.inner_context.memory[off..off + size].to_vec());
                drop(host_ref);
                let Ok(_) = db.code_by_address(callee_address) else {
                    *consumed_gas = 0;
                    return uint_result_ptr!(call_opcode::REVERT_RETURN_CODE);
                };
                drop(db);

                let is_static = self.call_frame.ctx_is_static || call_type == CallType::StaticCall;
                let call_frame = CallFrame {
                    caller: new_frame_caller,
                    ctx_is_static: is_static,
                    ..Default::default()
                };

                let mut ctx = Self::new(
                    self.db.clone(),
                    call_frame,
                    self.transaction.clone(),
                    host,
                    self.inner_context.spec_id,
                );
                ctx.inner_context.depth = self.inner_context.depth + 1;
                let result = self.transaction.run(&mut ctx, gas_to_send).unwrap().result;
                let unused_gas = gas_to_send - result.gas_used();
                *consumed_gas -= unused_gas;
                *consumed_gas -= result.gas_refunded();
                self.inner_context.depth = ctx.inner_context.depth - 1;

                let return_code = if result.is_success() {
                    call_opcode::SUCCESS_RETURN_CODE
                } else {
                    call_opcode::REVERT_RETURN_CODE
                };

                // EIP-150: Gas cost changes for IO-heavy operations
                *consumed_gas = if self.inner_context.spec_id.is_enabled_in(SpecId::TANGERINE) {
                    (*consumed_gas - *consumed_gas / 64).min(gas_to_send)
                } else {
                    gas_to_send
                };

                let output = result.into_output().unwrap_or_default();
                (return_code, output)
            }
        };

        self.call_frame.last_call_return_data.clear();
        self.call_frame
            .last_call_return_data
            .clone_from(&return_data.to_vec());
        Self::copy_exact(
            &mut self.inner_context.memory,
            &return_data,
            ret_offset,
            0,
            ret_size,
        );

        uint_result_ptr!(return_code)
    }

    fn copy_exact(
        target: &mut [u8],
        source: &[u8],
        target_offset: u64,
        source_offset: u64,
        size: u64,
    ) -> *mut Result<()> {
        let target_offset = target_offset as usize;
        let source_offset = source_offset as usize;
        let size = size as usize;

        let (source_end, overflow) = source_offset.overflowing_add(size);
        // Check bounds
        if overflow || source_end > source.len() {
            return Box::into_raw(Box::new(Result::error(
                ExitStatusCode::OutOfOffset.to_u8(),
                (),
            )));
        }
        if size + source_offset > source.len() {
            return Box::into_raw(Box::new(Result::error(
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

        Box::into_raw(Box::new(Result::success(())))
    }

    pub extern "C" fn store_in_selfbalance_ptr(
        &mut self,
        balance: &mut Bytes32,
    ) -> *mut Result<()> {
        let host = self.host.read().unwrap();
        let addr = host.env().tx.transact_to;
        let account = if addr.is_zero() {
            AccountInfo::default()
        } else {
            self.db
                .read()
                .unwrap()
                .basic(addr)
                .unwrap()
                .unwrap_or_default()
        };
        *balance = Bytes32::from_u256(account.balance);
        Box::into_raw(Box::new(Result::success(())))
    }

    pub extern "C" fn keccak256_hasher(
        &mut self,
        offset: u64,
        size: u64,
        hash_ptr: &mut Bytes32,
    ) -> *mut Result<()> {
        let data = &self.inner_context.memory[offset as usize..offset as usize + size as usize];
        let mut hasher = Keccak256::new();
        hasher.update(data);
        let result = hasher.finalize();
        *hash_ptr = Bytes32::from_be_bytes(result.into());
        Box::into_raw(Box::new(Result::success(())))
    }

    pub extern "C" fn callvalue(&self, value: &mut Bytes32) -> *mut Result<()> {
        let host = self.host.read().unwrap();
        *value = Bytes32::from_u256(host.env().tx.value);
        Box::into_raw(Box::new(Result::success(())))
    }

    pub extern "C" fn store_in_blobbasefee_ptr(&self, value: &mut u128) -> *mut Result<()> {
        let host = self.host.read().unwrap();
        *value = host.env().block.blob_gasprice.unwrap_or_default();
        Box::into_raw(Box::new(Result::success(())))
    }

    pub extern "C" fn gaslimit(&self) -> *mut Result<u64> {
        let host = self.host.read().unwrap();
        uint_result_ptr!(host.env().tx.gas_limit)
    }

    pub extern "C" fn caller(&self, value: &mut Bytes32) -> *mut Result<()> {
        value.copy_from(&self.call_frame.caller);
        Box::into_raw(Box::new(Result::success(())))
    }

    pub extern "C" fn store_in_gasprice_ptr(&self, value: &mut Bytes32) -> *mut Result<()> {
        let host = self.host.read().unwrap();
        *value = Bytes32::from(&host.env().tx.gas_price);
        Box::into_raw(Box::new(Result::success(())))
    }

    pub extern "C" fn chainid(&self) -> *mut Result<u64> {
        let host = self.host.read().unwrap();
        uint_result_ptr!(host.env().cfg.chain_id)
    }

    pub extern "C" fn calldata(&mut self) -> *mut Result<*mut u8> {
        let host = self.host.read().unwrap();
        host.env().tx.data.as_ptr() as _
    }

    pub extern "C" fn calldata_size(&self) -> u64 {
        let host = self.host.read().unwrap();
        host.env().tx.data.len() as u64
    }

    pub extern "C" fn origin(&self, address: &mut Bytes32) -> *mut Result<()> {
        let host = self.host.read().unwrap();
        address.copy_from(&host.env().tx.caller);
        Box::into_raw(Box::new(Result::success(())))
    }

    pub extern "C" fn extend_memory(&mut self, new_size: u64) -> *mut Result<*mut u8> {
        // Note the overflow on the 32-bit machine for the max memory e.g., 4GB
        let new_size = new_size as usize;
        if new_size <= self.inner_context.memory.len() {
            return Box::into_raw(Box::new(Result::success(
                self.inner_context.memory.as_mut_ptr() as _,
            )));
        }
        // Check the memory usage bound
        match self
            .inner_context
            .memory
            .try_reserve(new_size - self.inner_context.memory.len())
        {
            Ok(()) => {
                self.inner_context.memory.resize(new_size, 0);
                Box::into_raw(Box::new(Result::success(
                    self.inner_context.memory.as_mut_ptr() as _,
                )))
            }
            Err(_) => Box::into_raw(Box::new(Result::error(
                ExitStatusCode::MemoryLimitOOG.to_u8(),
                std::ptr::null_mut(),
            ))),
        }
    }

    pub extern "C" fn code_copy(
        &mut self,
        code_offset: u64,
        size: u64,
        dest_offset: u64,
    ) -> *mut Result<()> {
        let code = &self.inner_context.program;
        let code_size = code.len();
        let code_offset = code_offset as usize;
        let dest_offset = dest_offset as usize;
        let size = size as usize;
        let code_offset = code_offset.min(code_size);
        let code_end = core::cmp::min(code_offset + size, code_size);
        let code_len: usize = code_end - code_offset;
        let code_slice = &code[code_offset..code_end];
        self.inner_context.memory[dest_offset..dest_offset + code_len].copy_from_slice(code_slice);
        // Zero-fill the remaining space
        if size > code_len {
            self.inner_context.memory[dest_offset + code_len..dest_offset + size].fill(0);
        }

        Box::into_raw(Box::new(Result::success(())))
    }

    pub extern "C" fn sload(
        &mut self,
        stg_key: &Bytes32,
        stg_value: &mut Bytes32,
    ) -> *mut Result<()> {
        let mut host = self.host.write().unwrap();
        let addr = host.env().tx.transact_to;
        let result = host.get_storage(&addr, stg_key);
        *stg_value = result.value;

        let is_cold = true;
        let gas_cost = gas::sload_cost(self.inner_context.spec_id, is_cold);
        Box::into_raw(Box::new(Result::success_with_gas((), gas_cost)))
    }

    pub extern "C" fn sstore(
        &mut self,
        stg_key: &Bytes32,
        stg_value: &mut Bytes32,
    ) -> *mut Result<u64> {
        let mut host = self.host.write().unwrap();
        let addr = host.env().tx.transact_to;
        let result = host.set_storage(addr, *stg_key, *stg_value);

        let original = result.original_value.to_u256();
        let current = result.present_value.to_u256();
        let new = stg_value.to_u256();

        let is_cold = true;
        let gas_cost = gas::sstore_cost(
            self.inner_context.spec_id,
            original,
            current,
            new,
            self.inner_context.gas_remaining.unwrap_or(0),
            is_cold,
        )
        .unwrap_or(0);
        self.inner_context.gas_refund =
            gas::sstore_refund(self.inner_context.spec_id, original, current, new) as u64;

        uint_result_ptr!(gas_cost)
    }

    pub extern "C" fn append_log(&mut self, offset: u64, size: u64) -> *mut Result<()> {
        self.create_log(offset, size, vec![]);
        Box::into_raw(Box::new(Result::success(())))
    }

    pub extern "C" fn append_log_with_one_topic(
        &mut self,
        offset: u64,
        size: u64,
        topic: &Bytes32,
    ) -> *mut Result<()> {
        self.create_log(offset, size, vec![topic.to_u256()]);
        Box::into_raw(Box::new(Result::success(())))
    }

    pub extern "C" fn append_log_with_two_topics(
        &mut self,
        offset: u64,
        size: u64,
        topic1: &Bytes32,
        topic2: &Bytes32,
    ) -> *mut Result<()> {
        self.create_log(offset, size, vec![topic1.to_u256(), topic2.to_u256()]);
        Box::into_raw(Box::new(Result::success(())))
    }

    pub extern "C" fn append_log_with_three_topics(
        &mut self,
        offset: u64,
        size: u64,
        topic1: &Bytes32,
        topic2: &Bytes32,
        topic3: &Bytes32,
    ) -> *mut Result<()> {
        self.create_log(
            offset,
            size,
            vec![topic1.to_u256(), topic2.to_u256(), topic3.to_u256()],
        );
        Box::into_raw(Box::new(Result::success(())))
    }

    pub extern "C" fn append_log_with_four_topics(
        &mut self,
        offset: u64,
        size: u64,
        topic1: &Bytes32,
        topic2: &Bytes32,
        topic3: &Bytes32,
        topic4: &Bytes32,
    ) -> *mut Result<()> {
        self.create_log(
            offset,
            size,
            vec![
                topic1.to_u256(),
                topic2.to_u256(),
                topic3.to_u256(),
                topic4.to_u256(),
            ],
        );
        Box::into_raw(Box::new(Result::success(())))
    }

    pub extern "C" fn block_number(&self, number: &mut Bytes32) -> *mut Result<()> {
        let host = self.host.read().unwrap();
        *number = Bytes32::from(host.env().block.number);
        Box::into_raw(Box::new(Result::success(())))
    }

    pub extern "C" fn block_hash(&mut self, number: &mut Bytes32) -> *mut Result<()> {
        let host = self.host.read().unwrap();
        let number_as_u256 = number.to_u256();
        let hash = if number_as_u256 < host.env().block.number.saturating_sub(U256::from(256))
            || number_as_u256 >= host.env().block.number
        {
            B256::zero()
        } else {
            self.db
                .read()
                .unwrap()
                .block_hash(number_as_u256)
                .unwrap_or(B256::zero())
        };
        *number = Bytes32::from_be_bytes(hash.to_fixed_bytes());
        Box::into_raw(Box::new(Result::success(())))
    }

    fn create_log(&mut self, offset: u64, size: u64, topics: Vec<U256>) -> *mut Result<()> {
        let offset = offset as usize;
        let size = size as usize;
        let data: Vec<u8> = self.inner_context.memory[offset..offset + size].into();

        let log = LogData { data, topics };
        self.inner_context.logs.push(log);
        Box::into_raw(Box::new(Result::success(())))
    }

    pub extern "C" fn extcodesize(&mut self, address: &Bytes32) -> *mut Result<u64> {
        let size = self
            .db
            .read()
            .unwrap()
            .code_by_address(Address::from(address))
            .unwrap()
            .len();

        let is_cold = true;
        let gas_cost = gas::extcodesize_gas_cost(self.inner_context.spec_id, is_cold);

        Box::into_raw(Box::new(Result {
            gas_used: gas_cost,
            error: 0,
            value: size as u64,
        }))
    }

    #[allow(clippy::clone_on_copy)]
    pub extern "C" fn address(&mut self) -> *mut Result<*mut u8> {
        let host = self.host.read().unwrap();
        let address = host.env().tx.transact_to.clone();
        Box::into_raw(Box::new(Result::success(address.as_ptr() as *mut u8)))
    }

    pub extern "C" fn prevrandao(&self, prevrandao: &mut Bytes32) -> *mut Result<()> {
        let host = self.host.read().unwrap();
        *prevrandao = if self.inner_context.spec_id.is_enabled_in(SpecId::MERGE) {
            let randao = host.env().block.prevrandao.unwrap_or_default();
            Bytes32::from_be_bytes(randao.into())
        } else {
            host.env().block.difficulty.into()
        };
        Box::into_raw(Box::new(Result::success(())))
    }

    pub extern "C" fn coinbase(&self) -> *mut Result<*mut u8> {
        let host = self.host.read().unwrap();
        Box::into_raw(Box::new(Result::success(
            host.env().block.coinbase.as_ptr() as *mut u8,
        )))
    }

    pub extern "C" fn store_in_timestamp_ptr(&self, value: &mut Bytes32) -> *mut Result<()> {
        let host = self.host.read().unwrap();
        *value = Bytes32::from(&host.env().block.timestamp);
        Box::into_raw(Box::new(Result::success(())))
    }

    pub extern "C" fn store_in_basefee_ptr(&self, basefee: &mut Bytes32) -> *mut Result<()> {
        let host = self.host.read().unwrap();
        *basefee = Bytes32::from(&host.env().block.basefee);
        Box::into_raw(Box::new(Result::success(())))
    }

    /// This function reads an address pointer and set the balance of the address to the same pointer
    pub extern "C" fn store_in_balance(&mut self, address: &mut Bytes32) -> *mut Result<()> {
        let addr = address.to_address();
        let result = self.host.read().unwrap().get_balance(&addr);
        *address = result.value;
        let gas_cost = gas::balance_gas_cost(self.inner_context.spec_id, result.is_cold);

        Box::into_raw(Box::new(Result {
            gas_used: gas_cost,
            error: 0,
            value: (),
        }))
    }

    pub extern "C" fn blob_hash(
        &mut self,
        index: &Bytes32,
        blobhash: &mut Bytes32,
    ) -> *mut Result<()> {
        // Check if the high 12 bytes are zero, indicating a valid usize-compatible index
        if index.slice()[0..12] != [0u8; 12] {
            *blobhash = Bytes32::default();
            return Box::into_raw(Box::new(Result::success(())));
        }

        // Convert the low 20 bytes into a usize for the index
        let idx = usize::from_be_bytes(index.slice()[12..32].try_into().unwrap_or_default());
        *blobhash = self
            .host
            .read()
            .unwrap()
            .env()
            .tx
            .blob_hashes
            .get(idx)
            .cloned()
            .map(|x| Bytes32::from_be_bytes(x.into()))
            .unwrap_or_default();

        Box::into_raw(Box::new(Result::success(())))
    }

    pub extern "C" fn ext_code_copy(
        &mut self,
        address_value: &Bytes32,
        code_offset: u64,
        size: u64,
        dest_offset: u64,
    ) -> *mut Result<()> {
        let address = Address::from(address_value);
        let code = self
            .db
            .read()
            .unwrap()
            .code_by_address(address)
            .unwrap_or_default();
        let code_size = code.len();
        let code_offset = code_offset as usize;
        let dest_offset = dest_offset as usize;
        let size = size as usize;
        let code_offset = code_offset.min(code_size);
        let code_end = core::cmp::min(code_offset + size, code_size);
        let code_len = code_end - code_offset;
        let code_slice = &code[code_offset..code_end];
        self.inner_context.memory[dest_offset..dest_offset + code_len].copy_from_slice(code_slice);

        let is_cold = true;
        let gas_cost = gas::extcodecopy_gas_cost(self.inner_context.spec_id, is_cold);

        // Zero-fill the remaining space
        if size > code_len {
            self.inner_context.memory[dest_offset + code_len..dest_offset + size].fill(0);
        }
        Box::into_raw(Box::new(Result::success_with_gas((), gas_cost)))
    }

    pub extern "C" fn ext_code_hash(&mut self, address: &mut Bytes32) -> *mut Result<()> {
        let hash = match self
            .db
            .read()
            .unwrap()
            .basic(Address::from(address as &Bytes32))
        {
            Ok(Some(account_info)) => account_info.code_hash,
            _ => B256::zero(),
        };
        *address = Bytes32::from_be_bytes(hash.to_fixed_bytes());

        let is_cold = true;
        let gas_cost = gas::extcodehash_gas_cost(self.inner_context.spec_id, is_cold);

        Box::into_raw(Box::new(Result {
            gas_used: gas_cost,
            error: 0,
            value: (),
        }))
    }

    pub fn create_contract(
        &mut self,
        bytecode: &[u8],
        remaining_gas: &mut u64,
        value: U256,
        salt: Option<&Bytes32>,
    ) -> core::result::Result<Address, ExitStatusCode> {
        // Check the call depth
        if self.inner_context.depth > CALL_STACK_LIMIT {
            self.inner_context.exit_status = Some(ExitStatusCode::CallTooDeep);
            return Err(ExitStatusCode::CallTooDeep);
        }

        let host = self.host.read().unwrap();
        let size = bytecode.len();

        // Check the create init code size limit
        if size > 2 * MAX_CODE_SIZE {
            self.inner_context.exit_status = Some(ExitStatusCode::CreateInitCodeSizeLimit);
            return Err(ExitStatusCode::CreateInitCodeSizeLimit);
        }

        let sender_address = host.env().tx.get_address();
        let caller = host.env().tx.caller;
        let mut db = self.db.write().unwrap();
        let sender_account = db.basic(sender_address).unwrap().unwrap_or_default();

        let dest_addr = match salt {
            Some(s) => compute_contract_address2(sender_address, s.to_u256(), bytecode),
            _ => compute_contract_address(sender_address, sender_account.nonce),
        };
        // Check if there is already a contract stored in dest_address
        if let Ok(Some(_)) = db.basic(dest_addr) {
            self.inner_context.exit_status = Some(ExitStatusCode::CreateCollision);
            return Err(ExitStatusCode::CreateCollision);
        }
        drop(host);
        // Create sub context for the initialization code
        let host = self.host.clone();
        let mut host_ref = host.write().unwrap();
        let new_env = host_ref.env_mut();
        new_env.tx.transact_to = dest_addr;
        new_env.tx.gas_limit = *remaining_gas;
        new_env.tx.caller = caller;
        drop(host_ref);
        let code = bytecode.to_vec();
        db.insert_contract(dest_addr, code.into(), U256::ZERO);
        drop(db);
        self.call_frame = CallFrame::new(sender_address);
        self.inner_context.depth += 1;
        let tx = self.transaction.clone();
        let code_deposit_cost = (bytecode.len() as u64) * gas_cost::BYTE_DEPOSIT_COST as u64;
        // Set the gas cost
        *remaining_gas = match tx.run(self, *remaining_gas) {
            Ok(result) => {
                // Contract create success.
                code_deposit_cost + result.result.gas_used() - result.result.gas_refunded()
            }
            Err(_) => {
                // Contract create failed.
                code_deposit_cost
            }
        };
        self.inner_context.depth -= 1;
        // Check if balance is enough
        let sender_balance = match sender_account.balance.checked_sub(value) {
            Some(balance) => balance,
            None => {
                return Ok(Address::zero());
            }
        };
        // Create new contract and update sender account
        let mut db = self.db.write().unwrap();
        db.set_account(
            sender_address,
            sender_account.nonce + 1,
            sender_balance,
            Default::default(),
        );

        if self.inner_context.spec_id.is_enabled_in(SpecId::TANGERINE) {
            *remaining_gas -= *remaining_gas / 64;
        }

        Ok(dest_addr)
    }

    fn create_aux(
        &mut self,
        size: u64,
        offset: u64,
        value: &mut Bytes32,
        remaining_gas: &mut u64,
        salt: Option<&Bytes32>,
    ) -> *mut Result<u8> {
        let offset = offset as usize;
        let size = size as usize;
        let memory_len = self.inner_context.memory.len();
        if offset > memory_len {
            return Box::into_raw(Box::new(Result::success(1)));
        }
        let available_size = memory_len - offset;
        let actual_size = size.min(available_size);
        let bytecode = self.inner_context.memory[offset..offset + actual_size].to_vec();
        let value_u256 = value.to_u256();
        match self.create_contract(&bytecode, remaining_gas, value_u256, salt) {
            Ok(addr) => {
                value.copy_from(&addr);
                Box::into_raw(Box::new(Result::success(0)))
            }
            Err(err_code) => Box::into_raw(Box::new(Result::error(err_code.to_u8(), 1))),
        }
    }

    pub extern "C" fn create(
        &mut self,
        size: u64,
        offset: u64,
        value: &mut Bytes32,
        remaining_gas: &mut u64,
    ) -> *mut Result<u8> {
        self.create_aux(size, offset, value, remaining_gas, None)
    }

    pub extern "C" fn create2(
        &mut self,
        size: u64,
        offset: u64,
        value: &mut Bytes32,
        remaining_gas: &mut u64,
        salt: &Bytes32,
    ) -> *mut Result<u8> {
        self.create_aux(size, offset, value, remaining_gas, Some(salt))
    }

    pub extern "C" fn selfdestruct(&mut self, receiver_address: &Bytes32) -> *mut Result<u64> {
        let mut host = self.host.write().unwrap();
        let sender_address = host.env().tx.get_address();
        let receiver_address = Address::from(receiver_address);
        let result = host.selfdestruct(&sender_address, receiver_address);

        let gas_cost = if result.had_value && !result.target_exists {
            gas_cost::SELFDESTRUCT_DYNAMIC_GAS as u64
        } else {
            0
        };

        // EIP-3529: Reduction in refunds
        if !self.inner_context.spec_id.is_enabled_in(SpecId::LONDON) && !result.previously_destroyed
        {
            self.inner_context.gas_refund += gas_cost::SELFDESTRUCT as u64;
        }

        Box::into_raw(Box::new(Result {
            error: 0,
            gas_used: gas_cost,
            value: 0,
        }))
    }

    pub extern "C" fn read_transient_storage(
        &mut self,
        stg_key: &Bytes32,
        stg_value: &mut Bytes32,
    ) -> *mut Result<()> {
        let mut host = self.host.write().unwrap();
        let addr = host.env().tx.transact_to;
        let result = host.get_transient_storage(&addr, stg_key);
        *stg_value = result;
        Box::into_raw(Box::new(Result::success(())))
    }

    pub extern "C" fn write_transient_storage(
        &mut self,
        stg_key: &Bytes32,
        stg_value: &mut Bytes32,
    ) -> *mut Result<()> {
        let mut host = self.host.write().unwrap();
        let addr = host.env().tx.transact_to;
        host.set_transient_storage(&addr, *stg_key, *stg_value);
        Box::into_raw(Box::new(Result::success(())))
    }
}

type SymbolSignature = (&'static str, *const fn() -> ());

impl<DB: Database> RuntimeContext<DB> {
    /// Registers all the syscalls as symbols in the execution engine.
    pub fn register_symbols(&self, engine: &ExecutionEngine) {
        unsafe {
            // Global variables and syscalls with corresponding function signatures
            let symbols_and_signatures: &[SymbolSignature] = &[
                // Global variables
                (
                    symbols::CTX_IS_STATIC,
                    &self.call_frame.ctx_is_static as *const bool as *const _,
                ),
                // (
                //     symbols::DEBUG_PRINT,
                //     RuntimeContext::debug_print as *const _,
                // ),
                // Syscalls
                (
                    symbols::WRITE_RESULT,
                    RuntimeContext::<DB>::write_result as *const _,
                ),
                (
                    symbols::KECCAK256_HASHER,
                    RuntimeContext::<DB>::keccak256_hasher as *const _,
                ),
                (
                    symbols::EXTEND_MEMORY,
                    RuntimeContext::<DB>::extend_memory as *const _,
                ),
                (symbols::SLOAD, RuntimeContext::<DB>::sload as *const _),
                (symbols::SSTORE, RuntimeContext::<DB>::sstore as *const _),
                (
                    symbols::APPEND_LOG,
                    RuntimeContext::<DB>::append_log as *const _,
                ),
                (
                    symbols::APPEND_LOG_ONE_TOPIC,
                    RuntimeContext::<DB>::append_log_with_one_topic as *const _,
                ),
                (
                    symbols::APPEND_LOG_TWO_TOPICS,
                    RuntimeContext::<DB>::append_log_with_two_topics as *const _,
                ),
                (
                    symbols::APPEND_LOG_THREE_TOPICS,
                    RuntimeContext::<DB>::append_log_with_three_topics as *const _,
                ),
                (
                    symbols::APPEND_LOG_FOUR_TOPICS,
                    RuntimeContext::<DB>::append_log_with_four_topics as *const _,
                ),
                (symbols::CALL, RuntimeContext::<DB>::call as *const _),
                (
                    symbols::CALLDATA,
                    RuntimeContext::<DB>::calldata as *const _,
                ),
                (
                    symbols::CALLDATA_SIZE,
                    RuntimeContext::<DB>::calldata_size as *const _,
                ),
                (
                    symbols::CODE_COPY,
                    RuntimeContext::<DB>::code_copy as *const _,
                ),
                (symbols::ORIGIN, RuntimeContext::<DB>::origin as *const _),
                (symbols::ADDRESS, RuntimeContext::<DB>::address as *const _),
                (
                    symbols::CALLVALUE,
                    RuntimeContext::<DB>::callvalue as *const _,
                ),
                (
                    symbols::STORE_IN_BLOBBASEFEE_PTR,
                    RuntimeContext::<DB>::store_in_blobbasefee_ptr as *const _,
                ),
                (
                    symbols::EXT_CODE_SIZE,
                    RuntimeContext::<DB>::extcodesize as *const _,
                ),
                (
                    symbols::COINBASE,
                    RuntimeContext::<DB>::coinbase as *const _,
                ),
                (
                    symbols::STORE_IN_TIMESTAMP_PTR,
                    RuntimeContext::<DB>::store_in_timestamp_ptr as *const _,
                ),
                (
                    symbols::STORE_IN_BASEFEE_PTR,
                    RuntimeContext::<DB>::store_in_basefee_ptr as *const _,
                ),
                (symbols::CALLER, RuntimeContext::<DB>::caller as *const _),
                (
                    symbols::GASLIMIT,
                    RuntimeContext::<DB>::gaslimit as *const _,
                ),
                (
                    symbols::STORE_IN_GASPRICE_PTR,
                    RuntimeContext::<DB>::store_in_gasprice_ptr as *const _,
                ),
                (
                    symbols::BLOCK_NUMBER,
                    RuntimeContext::<DB>::block_number as *const _,
                ),
                (
                    symbols::PREVRANDAO,
                    RuntimeContext::<DB>::prevrandao as *const _,
                ),
                (
                    symbols::BLOB_HASH,
                    RuntimeContext::<DB>::blob_hash as *const _,
                ),
                (symbols::CHAINID, RuntimeContext::<DB>::chainid as *const _),
                (
                    symbols::STORE_IN_BALANCE,
                    RuntimeContext::<DB>::store_in_balance as *const _,
                ),
                (
                    symbols::STORE_IN_SELFBALANCE_PTR,
                    RuntimeContext::<DB>::store_in_selfbalance_ptr as *const _,
                ),
                (
                    symbols::EXT_CODE_COPY,
                    RuntimeContext::<DB>::ext_code_copy as *const _,
                ),
                (
                    symbols::BLOCK_HASH,
                    RuntimeContext::<DB>::block_hash as *const _,
                ),
                (
                    symbols::EXT_CODE_HASH,
                    RuntimeContext::<DB>::ext_code_hash as *const _,
                ),
                (symbols::CREATE, RuntimeContext::<DB>::create as *const _),
                (symbols::CREATE2, RuntimeContext::<DB>::create2 as *const _),
                (
                    symbols::RETURN_DATA_SIZE,
                    RuntimeContext::<DB>::return_data_size as *const _,
                ),
                (
                    symbols::RETURN_DATA_COPY,
                    RuntimeContext::<DB>::return_data_copy as *const _,
                ),
                (
                    symbols::SELFDESTRUCT,
                    RuntimeContext::<DB>::selfdestruct as *const _,
                ),
                (
                    symbols::TRANSIENT_STORAGE_READ,
                    RuntimeContext::<DB>::read_transient_storage as *const _,
                ),
                (
                    symbols::TRANSIENT_STORAGE_WRITE,
                    RuntimeContext::<DB>::write_transient_storage as *const _,
                ),
            ];

            for (symbol, signature) in symbols_and_signatures {
                engine.register_symbol(symbol, *signature as *mut ());
            }
        }
    }
}

/// Computes the contract address based on the sender's address and nonce.
///
/// This method follows the standard contract creation process by using the RLP encoding of the sender's address and nonce,
/// and applying a Keccak256 hash to generate the new contract address.
///
/// # Parameters
///
/// - `address`: The 160-bit sender address (usually an externally owned account).
/// - `nonce`: The nonce of the transaction that initiates the contract creation.
///
/// # Returns
///
/// - `Address`: The computed contract address.
///
/// # Example
///
/// ```no_check
/// let contract_address = RuntimeContext::compute_contract_address(sender_address, nonce);
/// ```
pub fn compute_contract_address(address: H160, nonce: u64) -> Address {
    // Compute the destination address using keccak256
    let encoded_nonce = encode_rlp_u64(nonce);
    let mut buf = Vec::new();
    buf.push(0xd5);
    buf.extend_from_slice(&encoded_nonce.len().to_be_bytes());
    buf.push(0x94);
    buf.extend_from_slice(address.as_bytes());
    buf.extend_from_slice(&encoded_nonce);

    let mut hasher = Keccak256::new();
    hasher.update(&buf);
    Address::from_slice(&hasher.finalize()[12..])
}

/// Computes the contract address using the CREATE2 opcode, which allows specifying a salt.
///
/// This method generates a contract address deterministically based on the sender's address, a salt, and the contract's initialization code.
/// This ensures that the same contract address will be generated given the same input values.
///
/// # Parameters
///
/// - `address`: The 160-bit sender address.
/// - `salt`: A 256-bit salt value, which can be chosen arbitrarily by the sender.
/// - `initialization_code`: The contract's initialization code, used to hash and form part of the address computation.
///
/// # Returns
///
/// - `Address`: The computed contract address.
///
/// # Example
///
/// ```no_check
/// let contract_address = RuntimeContext::compute_contract_address2(sender_address, salt, init_code);
/// ```
pub fn compute_contract_address2(address: H160, salt: U256, initialization_code: &[u8]) -> Address {
    // Compute the destination address using the second method
    let initialization_code_hash = {
        let mut hasher = Keccak256::new();
        hasher.update(initialization_code);
        hasher.finalize()
    };

    let mut hasher = Keccak256::new();
    let salt_bytes: [u8; 32] = salt.to_be_bytes();
    hasher.update([0xff]);
    hasher.update(address.as_bytes());
    hasher.update(salt_bytes);
    hasher.update(initialization_code_hash);

    Address::from_slice(&hasher.finalize()[12..])
}

pub fn encode_rlp_u64(number: u64) -> Vec<u8> {
    let mut buf = vec![0x80]; // RLP encoding for a single 64-bit number
    buf.extend_from_slice(&number.to_be_bytes());
    buf
}
