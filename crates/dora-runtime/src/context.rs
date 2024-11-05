use crate::constants::gas_cost;
use crate::{
    env::{Env, TransactTo},
    journal::Journal,
    result::{EVMError, ExecutionResult, HaltReason, Output, ResultAndState, SuccessReason},
};
use crate::{symbols, ExitStatusCode};
use dora_primitives::account::{AccountInfo, AccountStatus};
use dora_primitives::{EVMAddress as Address, B256, H160, U256 as EU256};
use melior::ExecutionEngine;
use sha3::{Digest, Keccak256};
use std::collections::HashMap;

/// Function type for the main entrypoint of the generated code.
pub type MainFunc = extern "C" fn(&mut RuntimeContext, initial_gas: u64) -> u8;

/// A 256-bit unsigned integer represented as two 128-bit components.
///
/// The `U256` struct is useful for representing large integers in blockchain applications,
/// especially for Ethereum-related calculations where 256-bit values are common.
///
/// # Fields:
/// - `lo`: The lower 128 bits of the 256-bit number.
/// - `hi`: The higher 128 bits of the 256-bit number.
///
/// # Example Usage:
/// ```no_check
/// let number = U256 { lo: 0, hi: 1 };
/// assert_eq!(number.hi, 1);
/// assert_eq!(number.lo, 0);
/// ```
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(C, align(16))]
pub struct U256 {
    pub lo: u128,
    pub hi: u128,
}

impl U256 {
    pub fn from_fixed_be_bytes(bytes: [u8; 32]) -> Self {
        let hi = u128::from_be_bytes(bytes[0..16].try_into().unwrap());
        let lo = u128::from_be_bytes(bytes[16..32].try_into().unwrap());
        U256 { hi, lo }
    }

    pub fn copy_from(&mut self, value: &Address) {
        let mut buffer = [0u8; 32];
        buffer[12..32].copy_from_slice(&value.0);
        self.lo = u128::from_be_bytes(buffer[16..32].try_into().unwrap());
        self.hi = u128::from_be_bytes(buffer[0..16].try_into().unwrap());
    }

    pub fn to_primitive_u256(&self) -> EU256 {
        (EU256::from(self.hi) << 128) + self.lo
    }

    pub fn to_le_bytes(&self) -> [u8; 32] {
        let mut bytes = [0u8; 32];
        let lo_bytes = self.lo.to_le_bytes();
        bytes[0..16].copy_from_slice(&lo_bytes);
        let hi_bytes = self.hi.to_le_bytes();
        bytes[16..32].copy_from_slice(&hi_bytes);
        bytes
    }

    pub fn to_be_bytes(&self) -> [u8; 32] {
        let mut bytes = [0u8; 32];
        let hi_bytes = self.hi.to_be_bytes();
        bytes[0..16].copy_from_slice(&hi_bytes);
        let lo_bytes = self.lo.to_be_bytes();
        bytes[16..32].copy_from_slice(&lo_bytes);
        bytes
    }

    pub fn zero() -> U256 {
        U256 { lo: 0, hi: 0 }
    }
}

impl From<&U256> for Address {
    fn from(value: &U256) -> Self {
        // Create an address from the last 20 bytes of the 256-bit U256.
        let hi_bytes = value.hi.to_be_bytes();
        let lo_bytes = value.lo.to_be_bytes();
        let address = [&hi_bytes[12..16], &lo_bytes[..]].concat();
        Address::from_slice(&address)
    }
}

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
    memory: Vec<u8>,
    return_data: Option<(usize, usize)>,
    pub program: Vec<u8>,
    gas_remaining: Option<u64>,
    gas_refund: u64,
    exit_status: Option<ExitStatusCode>,
    logs: Vec<LogData>,
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
///     transient_storage: HashMap::new(),
/// };
/// ```
#[derive(Debug)]
pub struct RuntimeContext<'c> {
    pub env: Env,
    pub journal: Journal<'c>,
    pub call_frame: CallFrame,
    pub inner_context: InnerContext,
    pub transient_storage: HashMap<(Address, EU256), EU256>,
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
///     topics: vec![U256::from(0x123), U256::from(0x456)],
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
impl<'c> RuntimeContext<'c> {
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
    pub fn new(env: Env, journal: Journal<'c>, call_frame: CallFrame) -> Self {
        Self {
            env,
            journal,
            call_frame,
            inner_context: Default::default(),
            transient_storage: Default::default(),
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
        self.inner_context
            .logs
            .iter()
            .map(|logdata| Log {
                address: self.env.tx.caller,
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
        let gas_remaining = self.inner_context.gas_remaining.unwrap_or(0);
        let gas_initial = self.env.tx.gas_limit;
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
impl<'c> RuntimeContext<'c> {
    pub extern "C" fn debug_print(val: i32) {
        println!("dora debug value: {val}");
    }
}

// System call functions

impl<'c> RuntimeContext<'c> {
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
        _gas_to_send: u64,
        _call_to_address: &U256,
        _value_to_transfer: &U256,
        _args_offset: u32,
        _args_size: u32,
        _ret_offset: u32,
        _ret_size: u32,
        _available_gas: u64,
        _consumed_gas: &mut u64,
        _call_type: u8,
    ) -> u8 {
        unimplemented!()
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

    pub extern "C" fn store_in_selfbalance_ptr(&mut self, balance: &mut U256) {
        let account = match self.env.tx.transact_to {
            TransactTo::Call(address) => self.journal.get_account(&address).unwrap_or_default(),
            TransactTo::Create => AccountInfo::default(), // This branch should never happen
        };
        balance.hi = (account.balance >> 128).low_u128();
        balance.lo = account.balance.low_u128();
    }

    pub extern "C" fn keccak256_hasher(&mut self, offset: u32, size: u32, hash_ptr: &mut U256) {
        let data = &self.inner_context.memory[offset as usize..offset as usize + size as usize];
        let mut hasher = Keccak256::new();
        hasher.update(data);
        let result = hasher.finalize();
        *hash_ptr = U256::from_fixed_be_bytes(result.into());
    }

    pub extern "C" fn store_in_callvalue_ptr(&self, value: &mut U256) {
        let aux = &self.env.tx.value;
        value.lo = aux.low_u128();
        value.hi = (aux >> 128).low_u128();
    }

    pub extern "C" fn store_in_blobbasefee_ptr(&self, value: &mut u128) {
        *value = self.env.block.blob_gasprice.unwrap_or_default();
    }

    pub extern "C" fn get_gaslimit(&self) -> u64 {
        self.env.tx.gas_limit
    }

    pub extern "C" fn store_in_caller_ptr(&self, value: &mut U256) {
        value.copy_from(&self.call_frame.caller);
    }

    pub extern "C" fn store_in_gasprice_ptr(&self, value: &mut U256) {
        let aux = &self.env.tx.gas_price;
        value.lo = aux.low_u128();
        value.hi = (aux >> 128).low_u128();
    }

    pub extern "C" fn get_chainid(&self) -> u64 {
        self.env.cfg.chain_id
    }

    pub extern "C" fn get_calldata_ptr(&mut self) -> *const u8 {
        self.env.tx.data.as_ptr()
    }

    pub extern "C" fn get_calldata_size_syscall(&self) -> u32 {
        self.env.tx.data.len() as u32
    }

    pub extern "C" fn get_origin(&self, address: &mut U256) {
        address.copy_from(&self.env.tx.caller);
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

    pub extern "C" fn read_storage(&mut self, stg_key: &U256, stg_value: &mut U256) {
        let address = self.env.tx.get_address();
        let key = stg_key.to_primitive_u256();

        let result = self
            .journal
            .read_storage(&address, &key)
            .unwrap_or_default()
            .present_value;
        stg_value.hi = (result >> 128).low_u128();
        stg_value.lo = result.low_u128();
    }

    pub extern "C" fn write_storage(&mut self, stg_key: &U256, stg_value: &mut U256) -> i64 {
        let key = stg_key.to_primitive_u256();
        let value = stg_value.to_primitive_u256();

        let address = match self.env.tx.transact_to {
            TransactTo::Call(address) => address,
            TransactTo::Create => return 0, // Storage should not be written on Create
        };

        let is_cold = !self.journal.is_key_warm(&address, &key);
        let slot = self.journal.read_storage(&address, &key);
        self.journal.write_storage(&address, key, value);

        let (original, current) = match slot {
            Some(slot) => (slot.original_value, slot.present_value),
            None => (value, value),
        };

        // Compute the gas cost
        let mut gas_cost: i64 = if original.is_zero() && current.is_zero() && current != value {
            20_000
        } else if original == current && current != value {
            2_900
        } else {
            100
        };

        if is_cold {
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

    pub extern "C" fn append_log_with_one_topic(&mut self, offset: u32, size: u32, topic: &U256) {
        self.create_log(offset, size, vec![*topic]);
    }

    pub extern "C" fn append_log_with_two_topics(
        &mut self,
        offset: u32,
        size: u32,
        topic1: &U256,
        topic2: &U256,
    ) {
        self.create_log(offset, size, vec![*topic1, *topic2]);
    }

    pub extern "C" fn append_log_with_three_topics(
        &mut self,
        offset: u32,
        size: u32,
        topic1: &U256,
        topic2: &U256,
        topic3: &U256,
    ) {
        self.create_log(offset, size, vec![*topic1, *topic2, *topic3]);
    }

    pub extern "C" fn append_log_with_four_topics(
        &mut self,
        offset: u32,
        size: u32,
        topic1: &U256,
        topic2: &U256,
        topic3: &U256,
        topic4: &U256,
    ) {
        self.create_log(offset, size, vec![*topic1, *topic2, *topic3, *topic4]);
    }

    pub extern "C" fn get_block_number(&self, number: &mut U256) {
        let block_number = self.env.block.number;

        number.hi = (block_number >> 128).low_u128();
        number.lo = block_number.low_u128();
    }

    pub extern "C" fn get_block_hash(&mut self, number: &mut U256) {
        let number_as_u256 = number.to_primitive_u256();
        let hash = if number_as_u256 < self.env.block.number.saturating_sub(EU256::from(256))
            || number_as_u256 >= self.env.block.number
        {
            B256::zero()
        } else {
            self.journal.get_block_hash(&number_as_u256)
        };
        let (hi, lo) = hash.as_bytes().split_at(16);
        number.lo = u128::from_be_bytes(lo.try_into().unwrap());
        number.hi = u128::from_be_bytes(hi.try_into().unwrap());
    }

    fn create_log(&mut self, offset: u32, size: u32, topics: Vec<U256>) {
        let offset = offset as usize;
        let size = size as usize;
        let data: Vec<u8> = self.inner_context.memory[offset..offset + size].into();

        let log = LogData { data, topics };
        self.inner_context.logs.push(log);
    }

    pub extern "C" fn get_codesize_from_address(&mut self, address: &U256) -> u64 {
        self.journal.code_by_address(&Address::from(address)).len() as _
    }

    #[allow(clippy::clone_on_copy)]
    pub extern "C" fn get_address_ptr(&mut self) -> *const u8 {
        // Just keep alive using the `clone()` function, the address stack value maybe freed by Rust in the release mode.
        let address = self.env.tx.get_address().clone();
        address.as_ptr()
    }

    pub extern "C" fn get_prevrandao(&self, prevrandao: &mut U256) {
        let randao = self.env.block.prevrandao.unwrap_or_default();
        *prevrandao = U256::from_fixed_be_bytes(randao.into());
    }

    pub extern "C" fn get_coinbase_ptr(&self) -> *const u8 {
        self.env.block.coinbase.as_ptr()
    }

    pub extern "C" fn store_in_timestamp_ptr(&self, value: &mut U256) {
        let aux = &self.env.block.timestamp;
        value.hi = (aux >> 128).low_u128();
        value.lo = aux.low_u128();
    }

    pub extern "C" fn store_in_basefee_ptr(&self, basefee: &mut U256) {
        basefee.hi = (self.env.block.basefee >> 128).low_u128();
        basefee.lo = self.env.block.basefee.low_u128();
    }

    pub extern "C" fn store_in_balance(&mut self, address: &U256, balance: &mut U256) {
        // Ensure address is a valid 20-byte Ethereum address
        if (address.hi >> 32) != 0 {
            balance.hi = 0;
            balance.lo = 0;
            return;
        }

        let address = Address::from_slice(
            &[
                &address.hi.to_be_bytes()[12..16],
                &address.lo.to_be_bytes()[..],
            ]
            .concat(),
        );

        if let Some(a) = self.journal.get_account(&address) {
            balance.hi = (a.balance >> 128).low_u128();
            balance.lo = a.balance.low_u128();
        } else {
            balance.hi = 0;
            balance.lo = 0;
        }
    }

    pub extern "C" fn get_blob_hash_at_index(&mut self, index: &U256, blobhash: &mut U256) {
        if index.hi != 0 {
            *blobhash = U256::default();
            return;
        }
        *blobhash = usize::try_from(index.lo)
            .ok()
            .and_then(|idx| self.env.tx.blob_hashes.get(idx).cloned())
            .map(|x| U256::from_fixed_be_bytes(x.into()))
            .unwrap_or_default();
    }

    pub extern "C" fn copy_ext_code_to_memory(
        &mut self,
        address_value: &U256,
        code_offset: u32,
        size: u32,
        dest_offset: u32,
    ) {
        let address = Address::from(address_value);
        let code = self.journal.code_by_address(&address);
        let code_size = code.len();
        let code_offset = code_offset as usize;
        let dest_offset = dest_offset as usize;

        // Determine the amount of code to copy and perform the copy
        let code_to_copy_size = code_size.saturating_sub(code_offset);
        let code_slice = &code[code_offset..code_offset + code_to_copy_size];
        self.inner_context.memory[dest_offset..dest_offset + code_to_copy_size]
            .copy_from_slice(code_slice);

        // Zero-fill the remaining space
        let padding_size = size as usize - code_to_copy_size;
        if padding_size > 0 {
            self.inner_context.memory[dest_offset + code_to_copy_size..dest_offset + size as usize]
                .fill(0);
        }
    }

    pub extern "C" fn get_code_hash(&mut self, address: &mut U256) {
        let hash = match self.journal.get_account(&Address::from(address as &U256)) {
            Some(account_info) => account_info.code_hash,
            _ => B256::zero(),
        };
        *address = U256::from_fixed_be_bytes(hash.to_fixed_bytes());
    }

    fn create_aux(
        &mut self,
        _size: u32,
        _offset: u32,
        _value: &mut U256,
        _remaining_gas: &mut u64,
        _salt: Option<&U256>,
    ) -> u8 {
        unimplemented!()
    }

    pub extern "C" fn create(
        &mut self,
        size: u32,
        offset: u32,
        value: &mut U256,
        remaining_gas: &mut u64,
    ) -> u8 {
        self.create_aux(size, offset, value, remaining_gas, None)
    }

    pub extern "C" fn create2(
        &mut self,
        size: u32,
        offset: u32,
        value: &mut U256,
        remaining_gas: &mut u64,
        salt: &U256,
    ) -> u8 {
        self.create_aux(size, offset, value, remaining_gas, Some(salt))
    }

    pub extern "C" fn selfdestruct(&mut self, receiver_address: &U256) -> u64 {
        let sender_address = self.env.tx.get_address();
        let receiver_address = Address::from(receiver_address);
        let sender_balance = self
            .journal
            .get_account(&sender_address)
            .unwrap_or_default()
            .balance;

        let receiver_is_empty = if let Some(receiver) = self.journal.get_account(&receiver_address)
        {
            let _ = self
                .journal
                .set_balance(&receiver_address, receiver.balance + sender_balance);
            receiver.is_empty()
        } else {
            let _ = self.journal.new_account(receiver_address, sender_balance);
            true
        };

        let _ = self.journal.set_balance(&sender_address, EU256::zero());
        if self.journal.get_account(&sender_address).is_some() {
            let _ = self
                .journal
                .set_status(&sender_address, AccountStatus::SelfDestructed);
        }

        if !sender_balance.is_zero() && receiver_is_empty {
            gas_cost::SELFDESTRUCT_DYNAMIC_GAS as u64
        } else {
            0
        }
    }

    pub extern "C" fn read_transient_storage(&mut self, stg_key: &U256, stg_value: &mut U256) {
        let key = stg_key.to_primitive_u256();
        let address = self.env.tx.get_address();
        let result = self
            .transient_storage
            .get(&(address, key))
            .cloned()
            .unwrap_or(EU256::zero());

        stg_value.hi = (result >> 128).low_u128();
        stg_value.lo = result.low_u128();
    }

    pub extern "C" fn write_transient_storage(&mut self, stg_key: &U256, stg_value: &mut U256) {
        let address = self.env.tx.get_address();
        let key = stg_key.to_primitive_u256();
        let value = stg_value.to_primitive_u256();
        self.transient_storage.insert((address, key), value);
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
        salt: EU256,
        initialization_code: &[u8],
    ) -> Address {
        // Compute the destination address using the second method
        let initialization_code_hash = {
            let mut hasher = Keccak256::new();
            hasher.update(initialization_code);
            hasher.finalize()
        };

        let mut hasher = Keccak256::new();
        let mut salt_bytes = [0; 32];
        salt.to_big_endian(&mut salt_bytes);
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

impl<'c> RuntimeContext<'c> {
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