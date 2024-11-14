use std::sync::{Arc, RwLock};

use crate::constants::{call_opcode, gas_cost, precompiles, CallType};
use crate::host::Host;
use crate::precompiles::{blake2f, ecrecover, identity, modexp, ripemd_160, sha2_256};
use crate::{
    journal::Journal,
    result::{EVMError, ExecutionResult, HaltReason, Output, ResultAndState, SuccessReason},
};
use crate::{symbols, ExitStatusCode};
use bytes::Bytes;
use dora_primitives::db::Database;
use dora_primitives::transaction::Transaction;
use dora_primitives::{Bytes32, EVMAddress as Address, B256, H160, U256};
use melior::ExecutionEngine;
use sha3::{Digest, Keccak256};

/// Function type for the main entrypoint of the generated code.
pub type MainFunc = extern "C" fn(&mut RuntimeContext, initial_gas: u64) -> u8;

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
    /// Whether the context is static.
    pub is_static: bool,
    /// Whether the context is EOF init.
    pub is_eof_init: bool,
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
    last_call_return_data: Vec<u8>,
}

impl CallFrame {
    pub fn new(caller: Address) -> Self {
        Self {
            caller,
            last_call_return_data: Vec::new(),
        }
    }
}

pub type RuntimeTransaction =
    Arc<dyn Transaction<Context = RuntimeContext, Result = anyhow::Result<ResultAndState>>>;

pub type RuntimeHost = Arc<RwLock<dyn Host>>;

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
///
/// # Example Usage:
/// ```no_check
/// let context = RuntimeContext {
///     env: Env::default(),
///     journal: Journal::new(),
///     call_frame: CallFrame::default(),
///     inner_context: InnerContext::default(),
///     transient_storage: FxHashMap::default(),
/// };
/// ```
#[derive(Debug)]
pub struct RuntimeContext {
    pub journal: Journal,
    pub call_frame: CallFrame,
    pub inner_context: InnerContext,
    pub transaction: RuntimeTransaction,
    pub host: RuntimeHost,
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

/// Accessors for managing and retrieving execution results in a runtime context.
impl RuntimeContext {
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
        journal: Journal,
        call_frame: CallFrame,
        transaction: RuntimeTransaction,
        host: RuntimeHost,
    ) -> Self {
        Self {
            journal,
            call_frame,
            inner_context: Default::default(),
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
        let host = self.host.write().unwrap();
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
    pub fn get_result(&self) -> Result<ResultAndState, EVMError> {
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
            .unwrap_or(ExitStatusCode::Default);

        let result = match exit_status {
            ExitStatusCode::Return => ExecutionResult::Success {
                reason: SuccessReason::Return,
                gas_used,
                gas_refunded,
                output: Output::Call(return_values.into()),
            },
            ExitStatusCode::Stop => ExecutionResult::Success {
                reason: SuccessReason::Stop,
                gas_used,
                gas_refunded,
                output: Output::Call(return_values.into()),
            },
            ExitStatusCode::Revert => ExecutionResult::Revert {
                output: return_values.into(),
                gas_used,
            },
            ExitStatusCode::Error | ExitStatusCode::Default => ExecutionResult::Halt {
                reason: HaltReason::OpcodeNotFound,
                gas_used,
            },
        };

        let state = self.journal.into_state();
        Ok(ResultAndState { result, state })
    }
}

// Debug functions
impl RuntimeContext {
    pub extern "C" fn debug_print(val: i32) {
        println!("dora debug value: {val}");
    }
}

// System call functions

impl RuntimeContext {
    pub extern "C" fn write_result(
        &mut self,
        offset: u32,
        bytes_len: u32,
        remaining_gas: u64,
        execution_result: u8,
    ) {
        self.inner_context.return_data = Some((offset as usize, bytes_len as usize));
        self.inner_context.gas_remaining = Some(remaining_gas);
        self.inner_context.exit_status = Some(ExitStatusCode::from_u8(execution_result));
    }

    pub extern "C" fn get_return_data_size(&mut self) -> u32 {
        self.call_frame.last_call_return_data.len() as _
    }

    pub extern "C" fn copy_return_data_into_memory(
        &mut self,
        dest_offset: u32,
        offset: u32,
        size: u32,
    ) {
        Self::copy_exact(
            &mut self.inner_context.memory,
            &self.call_frame.last_call_return_data,
            dest_offset,
            offset,
            size,
        );
    }

    pub extern "C" fn call(
        &mut self,
        mut gas_to_send: u64,
        call_to_address: &Bytes32,
        value_to_transfer: &Bytes32,
        args_offset: u32,
        args_size: u32,
        ret_offset: u32,
        ret_size: u32,
        available_gas: u64,
        consumed_gas: &mut u64,
        call_type: u8,
    ) -> u8 {
        let callee_address = Address::from(call_to_address);
        let off = args_offset as usize;
        let size = args_size as usize;
        let calldata = Bytes::copy_from_slice(&self.inner_context.memory[off..off + size]);
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
                // Retrieve or create the callee account in journal
                let callee_account = match self.journal.get_account(&callee_address) {
                    Some(acc) => {
                        *consumed_gas = call_opcode::WARM_MEMORY_ACCESS_COST;
                        acc
                    }
                    None => return call_opcode::REVERT_RETURN_CODE,
                };

                let host = self.host.read().unwrap();
                let caller = host.env().tx.caller;
                let caller_address = host.env().tx.get_address();
                let caller_account = self
                    .journal
                    .get_account(&caller_address)
                    .unwrap_or_default();

                let mut stipend = 0;
                if !value_to_transfer.as_u256().is_zero() {
                    if caller_account.balance < *value_to_transfer.as_u256() {
                        return call_opcode::REVERT_RETURN_CODE;
                    }
                    *consumed_gas += call_opcode::NOT_ZERO_VALUE_COST;
                    if callee_account.is_empty() {
                        *consumed_gas += call_opcode::EMPTY_CALLEE_COST;
                    }
                    if available_gas < *consumed_gas {
                        return call_opcode::REVERT_RETURN_CODE;
                    }
                    stipend = call_opcode::STIPEND_GAS_ADDITION;

                    let _ = self.journal.set_balance(
                        &caller_address,
                        caller_account.balance - value_to_transfer.as_u256(),
                    );
                    let _ = self.journal.set_balance(
                        &callee_address,
                        callee_account.balance + value_to_transfer.as_u256(),
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
                    CallType::Call | CallType::StaticCall => {
                        (this_address, value_to_transfer.to_u256(), callee_address)
                    }
                    CallType::CallCode => (this_address, value_to_transfer.to_u256(), this_address),
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
                new_env.tx.caller = caller;
                let off = args_offset as usize;
                let size = args_size as usize;
                new_env.tx.data = Bytes::from(self.inner_context.memory[off..off + size].to_vec());
                drop(host_ref);

                let journal = self.journal.eject_base();
                let call_frame = CallFrame::new(new_frame_caller);
                let mut ctx = Self::new(journal, call_frame, self.transaction.clone(), host);
                let result = self.transaction.run(&mut ctx, gas_to_send).unwrap().result;
                let unused_gas = gas_to_send - result.gas_used();
                *consumed_gas -= unused_gas;
                *consumed_gas -= result.gas_refunded();

                let return_code = if result.is_success() {
                    self.journal.extend_from_successful(ctx.journal);
                    call_opcode::SUCCESS_RETURN_CODE
                } else {
                    self.journal.extend_from_reverted(ctx.journal);
                    call_opcode::REVERT_RETURN_CODE
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

        return_code
    }

    fn copy_exact(
        target: &mut [u8],
        source: &[u8],
        target_offset: u32,
        source_offset: u32,
        size: u32,
    ) {
        let target_offset = target_offset as usize;
        let source_offset = source_offset as usize;
        let size = size as usize;

        // Check bounds
        if size + target_offset > target.len() {
            eprintln!("ERROR: Target offset and size exceed target length");
            return;
        }

        if size + source_offset > source.len() {
            eprintln!("ERROR: Source offset and size exceed source length");
            return;
        }

        // Calculate bytes to copy
        let available_target_space = target.len() - target_offset;
        let available_source_bytes = source.len() - source_offset;
        let bytes_to_copy = size.min(available_target_space).min(available_source_bytes);

        // Perform the copy
        target[target_offset..target_offset + bytes_to_copy]
            .copy_from_slice(&source[source_offset..source_offset + bytes_to_copy]);
    }

    pub extern "C" fn store_in_selfbalance_ptr(&mut self, balance: &mut Bytes32) {
        let host = self.host.read().unwrap();
        let account = self
            .journal
            .get_account(&host.env().tx.transact_to)
            .unwrap_or_default();
        *balance = Bytes32::from_u256(account.balance);
    }

    pub extern "C" fn keccak256_hasher(&mut self, offset: u32, size: u32, hash_ptr: &mut Bytes32) {
        let data = &self.inner_context.memory[offset as usize..offset as usize + size as usize];
        let mut hasher = Keccak256::new();
        hasher.update(data);
        let result = hasher.finalize();
        *hash_ptr = Bytes32::from_be_bytes(result.into());
    }

    pub extern "C" fn store_in_callvalue_ptr(&self, value: &mut Bytes32) {
        let host = self.host.read().unwrap();
        *value = Bytes32::from_u256(host.env().tx.value);
    }

    pub extern "C" fn store_in_blobbasefee_ptr(&self, value: &mut u128) {
        let host = self.host.read().unwrap();
        *value = host.env().block.blob_gasprice.unwrap_or_default();
    }

    pub extern "C" fn get_gaslimit(&self) -> u64 {
        let host = self.host.read().unwrap();
        host.env().tx.gas_limit
    }

    pub extern "C" fn store_in_caller_ptr(&self, value: &mut Bytes32) {
        value.copy_from(&self.call_frame.caller);
    }

    pub extern "C" fn store_in_gasprice_ptr(&self, value: &mut Bytes32) {
        let host = self.host.read().unwrap();
        *value = Bytes32::from(&host.env().tx.gas_price);
    }

    pub extern "C" fn get_chainid(&self) -> u64 {
        let host = self.host.read().unwrap();
        host.env().cfg.chain_id
    }

    pub extern "C" fn get_calldata_ptr(&mut self) -> *const u8 {
        let host = self.host.read().unwrap();
        host.env().tx.data.as_ptr()
    }

    pub extern "C" fn get_calldata_size_syscall(&self) -> u32 {
        let host = self.host.read().unwrap();
        host.env().tx.data.len() as u32
    }

    pub extern "C" fn get_origin(&self, address: &mut Bytes32) {
        let host = self.host.read().unwrap();
        address.copy_from(&host.env().tx.caller);
    }

    pub extern "C" fn extend_memory(&mut self, new_size: u32) -> *mut u8 {
        let new_size = new_size as usize;
        if new_size <= self.inner_context.memory.len() {
            return self.inner_context.memory.as_mut_ptr();
        }

        match self
            .inner_context
            .memory
            .try_reserve(new_size - self.inner_context.memory.len())
        {
            Ok(()) => {
                self.inner_context.memory.resize(new_size, 0);
                self.inner_context.memory.as_mut_ptr()
            }
            Err(err) => {
                eprintln!("Failed to reserve memory: {err}");
                std::ptr::null_mut()
            }
        }
    }

    pub extern "C" fn copy_code_to_memory(
        &mut self,
        code_offset: u32,
        size: u32,
        dest_offset: u32,
    ) {
        let code_size = self.inner_context.program.len();
        let code_offset = code_offset as usize;
        let size = size as usize;
        let dest_offset = dest_offset as usize;

        let size = size.min(code_size.saturating_sub(code_offset));
        let code_slice = match self
            .inner_context
            .program
            .get(code_offset..code_offset + size)
        {
            Some(slice) => slice,
            None => {
                eprintln!("Error on copy_code_to_memory");
                return;
            }
        };

        self.inner_context.memory[dest_offset..dest_offset + size].copy_from_slice(code_slice);
    }

    pub extern "C" fn read_storage(&mut self, stg_key: &Bytes32, stg_value: &mut Bytes32) {
        let mut host = self.host.write().unwrap();
        let addr = host.env().tx.transact_to;
        let result = host.get_storage(&addr, stg_key);
        *stg_value = result.value;
    }

    pub extern "C" fn write_storage(&mut self, stg_key: &Bytes32, stg_value: &mut Bytes32) -> i64 {
        let mut host = self.host.write().unwrap();
        let addr = host.env().tx.transact_to;
        let result = host.set_storage(addr, *stg_key, *stg_value);

        // Dynamic gas cost
        let original = result.original_value.to_u256();
        let current = result.present_value.to_u256();
        let value = stg_value.to_u256();

        // Compute the gas cost
        let mut gas_cost: i64 = if original.is_zero() && current.is_zero() && current != value {
            20_000
        } else if original == current && current != value {
            2_900
        } else {
            100
        };

        if result.is_cold {
            gas_cost += 2_100; // Extra cost for cold storage
        }

        // Compute the gas refund
        let gas_refund: i64 = match (original.is_zero(), current.is_zero(), value.is_zero()) {
            (false, false, true) => 4_800, // Reset non-zero to zero
            (false, true, false) if value == original => -2_000, // Undo reset to zero into original
            (false, true, false) => -4_800, // Undo reset to zero
            (true, false, true) => 19_900, // Reset back to zero
            (true, false, false) if current != value && original == value => 2_800, // Reset to original
            _ => 0,
        };

        if gas_refund > 0 {
            self.inner_context.gas_refund += gas_refund as u64;
        } else {
            self.inner_context.gas_refund -= gas_refund.unsigned_abs();
        };

        gas_cost
    }

    pub extern "C" fn append_log(&mut self, offset: u32, size: u32) {
        self.create_log(offset, size, vec![]);
    }

    pub extern "C" fn append_log_with_one_topic(
        &mut self,
        offset: u32,
        size: u32,
        topic: &Bytes32,
    ) {
        self.create_log(offset, size, vec![topic.to_u256()]);
    }

    pub extern "C" fn append_log_with_two_topics(
        &mut self,
        offset: u32,
        size: u32,
        topic1: &Bytes32,
        topic2: &Bytes32,
    ) {
        self.create_log(offset, size, vec![topic1.to_u256(), topic2.to_u256()]);
    }

    pub extern "C" fn append_log_with_three_topics(
        &mut self,
        offset: u32,
        size: u32,
        topic1: &Bytes32,
        topic2: &Bytes32,
        topic3: &Bytes32,
    ) {
        self.create_log(
            offset,
            size,
            vec![topic1.to_u256(), topic2.to_u256(), topic3.to_u256()],
        );
    }

    pub extern "C" fn append_log_with_four_topics(
        &mut self,
        offset: u32,
        size: u32,
        topic1: &Bytes32,
        topic2: &Bytes32,
        topic3: &Bytes32,
        topic4: &Bytes32,
    ) {
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
    }

    pub extern "C" fn get_block_number(&self, number: &mut Bytes32) {
        let host = self.host.read().unwrap();
        *number = Bytes32::from(host.env().block.number);
    }

    pub extern "C" fn get_block_hash(&mut self, number: &mut Bytes32) {
        let host = self.host.read().unwrap();
        let number_as_u256 = number.to_u256();
        let hash = if number_as_u256 < host.env().block.number.saturating_sub(U256::from(256))
            || number_as_u256 >= host.env().block.number
        {
            B256::zero()
        } else {
            self.journal.get_block_hash(&number_as_u256)
        };
        *number = Bytes32::from_be_bytes(hash.to_fixed_bytes());
    }

    fn create_log(&mut self, offset: u32, size: u32, topics: Vec<U256>) {
        let offset = offset as usize;
        let size = size as usize;
        let data: Vec<u8> = self.inner_context.memory[offset..offset + size].into();

        let log = LogData { data, topics };
        self.inner_context.logs.push(log);
    }

    pub extern "C" fn get_codesize_from_address(&mut self, address: &Bytes32) -> u64 {
        self.journal.code_by_address(&Address::from(address)).len() as _
    }

    #[allow(clippy::clone_on_copy)]
    pub extern "C" fn get_address_ptr(&mut self) -> *const u8 {
        let host = self.host.read().unwrap();
        let address = host.env().tx.transact_to.clone();
        address.as_ptr()
    }

    pub extern "C" fn get_prevrandao(&self, prevrandao: &mut Bytes32) {
        let host = self.host.read().unwrap();
        let randao = host.env().block.prevrandao.unwrap_or_default();
        *prevrandao = Bytes32::from_be_bytes(randao.into());
    }

    pub extern "C" fn get_coinbase_ptr(&self) -> *const u8 {
        let host = self.host.read().unwrap();
        host.env().block.coinbase.as_ptr()
    }

    pub extern "C" fn store_in_timestamp_ptr(&self, value: &mut Bytes32) {
        let host = self.host.read().unwrap();
        *value = Bytes32::from(&host.env().block.timestamp);
    }

    pub extern "C" fn store_in_basefee_ptr(&self, basefee: &mut Bytes32) {
        let host = self.host.read().unwrap();
        *basefee = Bytes32::from(&host.env().block.basefee);
    }

    pub extern "C" fn store_in_balance(&mut self, address: &Bytes32, balance: &mut Bytes32) {
        if !address.is_valid_eth_address() {
            *balance = Bytes32::ZERO;
            return;
        }

        let addr = Address::from(address);
        if let Some(a) = self.journal.get_account(&addr) {
            *balance = Bytes32::from_u256(a.balance);
        } else {
            *balance = Bytes32::ZERO;
        }
    }

    pub extern "C" fn get_blob_hash_at_index(&mut self, index: &Bytes32, blobhash: &mut Bytes32) {
        // Check if the high 12 bytes are zero, indicating a valid usize-compatible index
        if index.slice()[0..12] != [0u8; 12] {
            *blobhash = Bytes32::default();
            return;
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
    }

    pub extern "C" fn copy_ext_code_to_memory(
        &mut self,
        address_value: &Bytes32,
        code_offset: u32,
        size: u32,
        dest_offset: u32,
    ) {
        let address = Address::from(address_value);
        let code = self.journal.code_by_address(&address);
        let code_size = code.len();
        let code_offset = code_offset as usize;
        let dest_offset = dest_offset as usize;
        // Note the IOB error
        let code_offset = code_offset.min(code_size);
        // Determine the amount of code to copy and perform the copy
        let code_to_copy_size = code_size.saturating_sub(code_offset);
        let code_slice = &code[code_offset..code_offset + code_to_copy_size];
        self.inner_context.memory[dest_offset..dest_offset + code_to_copy_size]
            .copy_from_slice(code_slice);
        // Zero-fill the remaining space
        if size as usize > code_to_copy_size {
            self.inner_context.memory[dest_offset + code_to_copy_size..dest_offset + size as usize]
                .fill(0);
        }
    }

    pub extern "C" fn get_code_hash(&mut self, address: &mut Bytes32) {
        let hash = match self
            .journal
            .get_account(&Address::from(address as &Bytes32))
        {
            Some(account_info) => account_info.code_hash,
            _ => B256::zero(),
        };
        *address = Bytes32::from_be_bytes(hash.to_fixed_bytes());
    }

    pub fn create_contract(
        &mut self,
        bytecode: &[u8],
        remaining_gas: &mut u64,
        value: U256,
        salt: Option<&Bytes32>,
    ) -> Option<Address> {
        let host = self.host.read().unwrap();
        let size = bytecode.len();
        let minimum_word_size = ((size + 31) / 32) as u64;
        let sender_address = host.env().tx.get_address();
        let caller = host.env().tx.caller;
        let db = &mut self.journal.db;
        let sender_account = db.basic(sender_address).unwrap().unwrap_or_default();

        let (dest_addr, hash_cost) = match salt {
            Some(s) => (
                Self::compute_contract_address2(sender_address, s.to_u256(), bytecode),
                minimum_word_size * gas_cost::HASH_WORD_COST as u64,
            ),
            _ => (
                Self::compute_contract_address(sender_address, sender_account.nonce),
                0,
            ),
        };
        // Check if there is already a contract stored in dest_address
        if let Ok(Some(_)) = db.basic(dest_addr) {
            return None;
        }
        drop(host);
        // Create sub context for the initialization code
        // TODO: Add call depth 1024 check
        let host = self.host.clone();
        let mut host_ref = host.write().unwrap();
        let new_env = host_ref.env_mut();
        new_env.tx.transact_to = dest_addr;
        new_env.tx.gas_limit = *remaining_gas;
        new_env.tx.caller = caller;
        drop(host_ref);
        let code = bytecode.to_vec();
        let journal = Journal::new(db.clone().with_contract(dest_addr, code.into()));
        let call_frame = CallFrame::new(sender_address);
        let mut ctx = Self::new(journal, call_frame, self.transaction.clone(), host);
        let result = self
            .transaction
            .run(&mut ctx, *remaining_gas)
            .unwrap()
            .result;
        let _output = result.output().cloned().unwrap_or_default();
        // Set the gas cost
        let init_code_cost = minimum_word_size * gas_cost::INIT_WORD_COST as u64;
        let code_deposit_cost = (bytecode.len() as u64) * gas_cost::BYTE_DEPOSIT_COST as u64;
        let gas_cost = init_code_cost + code_deposit_cost + hash_cost + result.gas_used()
            - result.gas_refunded();
        *remaining_gas = gas_cost;

        // Check if balance is enough
        let sender_balance = sender_account.balance.checked_sub(value)?;
        // Create new contract and update sender account
        db.insert_contract(dest_addr, bytecode.to_vec().into(), value);
        db.set_account(
            sender_address,
            sender_account.nonce + 1,
            sender_balance,
            Default::default(),
        );
        // TODO: add dest_addr as warm in the access list
        Some(dest_addr)
    }

    fn create_aux(
        &mut self,
        size: u32,
        offset: u32,
        value: &mut Bytes32,
        remaining_gas: &mut u64,
        salt: Option<&Bytes32>,
    ) -> u8 {
        let offset = offset as usize;
        let size = size as usize;
        let bytecode = self.inner_context.memory[offset..offset + size].to_vec();
        let value_u256 = value.to_u256();
        match self.create_contract(&bytecode, remaining_gas, value_u256, salt) {
            Some(addr) => {
                value.copy_from(&addr);
                0
            }
            None => 1,
        }
    }

    pub extern "C" fn create(
        &mut self,
        size: u32,
        offset: u32,
        value: &mut Bytes32,
        remaining_gas: &mut u64,
    ) -> u8 {
        self.create_aux(size, offset, value, remaining_gas, None)
    }

    pub extern "C" fn create2(
        &mut self,
        size: u32,
        offset: u32,
        value: &mut Bytes32,
        remaining_gas: &mut u64,
        salt: &Bytes32,
    ) -> u8 {
        self.create_aux(size, offset, value, remaining_gas, Some(salt))
    }

    pub extern "C" fn selfdestruct(&mut self, receiver_address: &Bytes32) -> u64 {
        let mut host = self.host.write().unwrap();
        let sender_address = host.env().tx.get_address();
        let receiver_address = Address::from(receiver_address);
        let result = host.selfdestruct(&sender_address, receiver_address);
        if result.had_value && !result.target_exists {
            gas_cost::SELFDESTRUCT_DYNAMIC_GAS as u64
        } else {
            0
        }
    }

    pub extern "C" fn read_transient_storage(
        &mut self,
        stg_key: &Bytes32,
        stg_value: &mut Bytes32,
    ) {
        let mut host = self.host.write().unwrap();
        let addr = host.env().tx.transact_to;
        let result = host.get_transient_storage(&addr, stg_key);
        *stg_value = result;
    }

    pub extern "C" fn write_transient_storage(
        &mut self,
        stg_key: &Bytes32,
        stg_value: &mut Bytes32,
    ) {
        let mut host = self.host.write().unwrap();
        let addr = host.env().tx.transact_to;
        host.set_transient_storage(&addr, *stg_key, *stg_value);
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
        let encoded_nonce = Self::encode_rlp_u64(nonce);
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
    pub fn compute_contract_address2(
        address: H160,
        salt: U256,
        initialization_code: &[u8],
    ) -> Address {
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
}

type SymbolSignature = (&'static str, *const fn() -> ());

impl RuntimeContext {
    /// Registers all the syscalls as symbols in the execution engine.
    pub fn register_symbols(&self, engine: &ExecutionEngine) {
        unsafe {
            // Global variables and syscalls with corresponding function signatures
            let symbols_and_signatures: &[SymbolSignature] = &[
                (
                    symbols::DEBUG_PRINT,
                    RuntimeContext::debug_print as *const _,
                ),
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
                (
                    symbols::STORAGE_READ,
                    RuntimeContext::read_storage as *const _,
                ),
                (
                    symbols::STORAGE_WRITE,
                    RuntimeContext::write_storage as *const _,
                ),
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
                (
                    symbols::GET_CALLDATA_PTR,
                    RuntimeContext::get_calldata_ptr as *const _,
                ),
                (
                    symbols::GET_CALLDATA_SIZE,
                    RuntimeContext::get_calldata_size_syscall as *const _,
                ),
                (
                    symbols::COPY_CODE_TO_MEMORY,
                    RuntimeContext::copy_code_to_memory as *const _,
                ),
                (symbols::GET_ORIGIN, RuntimeContext::get_origin as *const _),
                (
                    symbols::GET_ADDRESS_PTR,
                    RuntimeContext::get_address_ptr as *const _,
                ),
                (
                    symbols::STORE_IN_CALLVALUE_PTR,
                    RuntimeContext::store_in_callvalue_ptr as *const _,
                ),
                (
                    symbols::STORE_IN_BLOBBASEFEE_PTR,
                    RuntimeContext::store_in_blobbasefee_ptr as *const _,
                ),
                (
                    symbols::GET_CODESIZE_FROM_ADDRESS,
                    RuntimeContext::get_codesize_from_address as *const _,
                ),
                (
                    symbols::GET_COINBASE_PTR,
                    RuntimeContext::get_coinbase_ptr as *const _,
                ),
                (
                    symbols::STORE_IN_TIMESTAMP_PTR,
                    RuntimeContext::store_in_timestamp_ptr as *const _,
                ),
                (
                    symbols::STORE_IN_BASEFEE_PTR,
                    RuntimeContext::store_in_basefee_ptr as *const _,
                ),
                (
                    symbols::STORE_IN_CALLER_PTR,
                    RuntimeContext::store_in_caller_ptr as *const _,
                ),
                (
                    symbols::GET_GASLIMIT,
                    RuntimeContext::get_gaslimit as *const _,
                ),
                (
                    symbols::STORE_IN_GASPRICE_PTR,
                    RuntimeContext::store_in_gasprice_ptr as *const _,
                ),
                (
                    symbols::GET_BLOCK_NUMBER,
                    RuntimeContext::get_block_number as *const _,
                ),
                (
                    symbols::GET_PREVRANDAO,
                    RuntimeContext::get_prevrandao as *const _,
                ),
                (
                    symbols::GET_BLOB_HASH_AT_INDEX,
                    RuntimeContext::get_blob_hash_at_index as *const _,
                ),
                (
                    symbols::GET_CHAINID,
                    RuntimeContext::get_chainid as *const _,
                ),
                (
                    symbols::STORE_IN_BALANCE,
                    RuntimeContext::store_in_balance as *const _,
                ),
                (
                    symbols::STORE_IN_SELFBALANCE_PTR,
                    RuntimeContext::store_in_selfbalance_ptr as *const _,
                ),
                (
                    symbols::COPY_EXT_CODE_TO_MEMORY,
                    RuntimeContext::copy_ext_code_to_memory as *const _,
                ),
                (
                    symbols::GET_BLOCK_HASH,
                    RuntimeContext::get_block_hash as *const _,
                ),
                (
                    symbols::GET_CODE_HASH,
                    RuntimeContext::get_code_hash as *const _,
                ),
                (symbols::CREATE, RuntimeContext::create as *const _),
                (symbols::CREATE2, RuntimeContext::create2 as *const _),
                (
                    symbols::GET_RETURN_DATA_SIZE,
                    RuntimeContext::get_return_data_size as *const _,
                ),
                (
                    symbols::COPY_RETURN_DATA_INTO_MEMORY,
                    RuntimeContext::copy_return_data_into_memory as *const _,
                ),
                (
                    symbols::SELFDESTRUCT,
                    RuntimeContext::selfdestruct as *const _,
                ),
                (
                    symbols::TRANSIENT_STORAGE_READ,
                    RuntimeContext::read_transient_storage as *const _,
                ),
                (
                    symbols::TRANSIENT_STORAGE_WRITE,
                    RuntimeContext::write_transient_storage as *const _,
                ),
            ];

            for (symbol, signature) in symbols_and_signatures {
                engine.register_symbol(symbol, *signature as *mut ());
            }
        }
    }
}
