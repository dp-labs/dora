use sha2::Digest;
use std::cmp::min;
use std::mem::MaybeUninit;

use super::context::with_runtime_context;
use super::ptr::GuestPtr;
use super::{
    env::{WASMEnv, WASMEnvMut},
    errors::{Escape, EscapeResult, MaybeEscape},
    memory::MemoryModel,
};
use crate::ExitStatusCode;
use crate::account::EMPTY_CODE_HASH_BYTES;
use crate::call::{CallKind, CallMessage, CallResult};
use crate::constants::CallType;
use crate::constants::env::DORA_DISABLE_CONSOLE;
use crate::context::RuntimeContext;
use dora_primitives::{
    Address, B256, Bytes32, Log, LogData, U256, as_u64_saturated, keccak256 as native_keccak256,
};
use wasmer::{Memory, MemoryAccessError, MemoryView, Pages, StoreMut, WasmPtr};

/// Represents host information including the environment, memory, and store.
pub struct HostInfo<'a> {
    /// Reference to the WASM environment.
    pub env: &'a mut WASMEnv,
    /// WASM memory instance.
    pub memory: Memory,
    /// WASM store for managing state.
    pub store: StoreMut<'a>,
}

impl<'a> HostInfo<'a> {
    /// Constructs a `HostInfo` instance from a mutable WASM environment.
    pub fn from_env(env: &'a mut WASMEnvMut<'_>) -> Result<Self, MemoryAccessError> {
        let (env, store) = env.data_and_store_mut(); // Split environment and store
        let memory = env
            .memory
            .clone()
            .ok_or(MemoryAccessError::HeapOutOfBounds)?; // Clone memory or return error
        Ok(HostInfo { env, memory, store })
    }

    /// Returns a view of the WASM memory.
    #[inline]
    pub fn view(&self) -> MemoryView {
        self.memory.view(&self.store)
    }

    /// Returns the size of the WASM memory in pages.
    #[inline]
    pub fn memory_size(&self) -> Pages {
        self.memory.ty(&self.store).minimum
    }

    /// Reads a fixed-size array of bytes from the WASM memory at the given guest pointer.
    pub fn read_fixed<const N: usize>(&self, ptr: GuestPtr) -> Result<[u8; N], MemoryAccessError> {
        let mut data = [MaybeUninit::uninit(); N]; // Initialize an uninitialized array
        self.view().read_uninit(ptr.into(), &mut data)?; // Read memory into the array
        Ok(data.map(|x| unsafe { x.assume_init() })) // Safely assume the data is initialized
    }

    /// Reads a slice of bytes from the WASM memory at the given guest pointer.
    pub fn read_slice(&self, ptr: GuestPtr, len: u32) -> Result<Vec<u8>, MemoryAccessError> {
        let len = len as usize;
        let mut data: Vec<MaybeUninit<u8>> = Vec::with_capacity(len); // Allocate uninitialized vector
        unsafe {
            data.set_len(len); // Set the length of the vector
            self.view().read_uninit(ptr.into(), &mut data)?; // Read memory into the vector
            Ok(std::mem::transmute::<Vec<MaybeUninit<u8>>, Vec<u8>>(data)) // Convert to initialized vector
        }
    }

    /// Reads an address with the fixed size 20 from the WASM memory at the given guest pointer.
    pub fn read_address(&self, ptr: GuestPtr) -> Result<Address, MemoryAccessError> {
        let address_bytes: [u8; 20] = self.read_fixed(ptr)?;
        Ok(address_bytes.into())
    }

    /// Reads an address with the fixed size 20 from the WASM memory at the given guest pointer.
    pub fn read_bytes32(&self, ptr: GuestPtr) -> Result<Bytes32, MemoryAccessError> {
        Ok(Bytes32::from_be_bytes(self.read_fixed(ptr)?))
    }

    /// Writes a 32-bit unsigned integer to the WASM memory at the given guest pointer.
    pub fn write_u32(&self, ptr: GuestPtr, x: u32) -> Result<(), MemoryAccessError> {
        let ptr: WasmPtr<u32> = WasmPtr::new(ptr.into()); // Convert to WASM pointer
        ptr.deref(&self.view()).write(x)?; // Write the value to memory
        Ok(())
    }

    /// Writes a slice of bytes to the WASM memory at the given guest pointer.
    #[inline]
    pub fn write_slice(&self, ptr: GuestPtr, src: &[u8]) -> Result<(), MemoryAccessError> {
        self.view().write(ptr.into(), src) // Write the slice to memory
    }
}

/// Gets the ETH balance in wei of the account at the given address.
pub fn account_balance(
    mut env: WASMEnvMut,
    address: GuestPtr, // *const u8
    dest: GuestPtr,    // *mut u8
) -> MaybeEscape {
    let host = HostInfo::from_env(&mut env)?;
    let address = host.read_address(address)?;
    let data = with_runtime_context(|runtime_context| {
        runtime_context
            .host
            .balance(address)
            .unwrap_or_default()
            .data
    });
    host.write_slice(dest, &data.to_be_bytes())?;
    Ok(())
}

/// Gets a subset of the code from the account at the given address.
pub fn account_code(
    mut env: WASMEnvMut,
    address: GuestPtr, // *const u8,
    offset: u32,       // usize,
    size: u32,         // usize,
    dest: GuestPtr,    // *mut u8,
) -> EscapeResult<u32> {
    let host = HostInfo::from_env(&mut env)?;
    let address = host.read_address(address)?;
    let code = with_runtime_context(|runtime_context| {
        runtime_context
            .host
            .code(address)
            .unwrap_or_default()
            .data
            .to_vec()
    });
    let code_slice = data_slice(&code, offset, size);
    host.write_slice(dest, code_slice)?;
    Ok(code_slice.len() as u32)
}

/// Gets the size of the code in bytes at the given address.
pub fn account_code_size(
    mut env: WASMEnvMut,
    address: GuestPtr, // *const u8
) -> EscapeResult<u32> {
    let host = HostInfo::from_env(&mut env)?;
    let address = host.read_address(address)?;
    let size = with_runtime_context(|runtime_context| {
        runtime_context
            .host
            .code(address)
            .unwrap_or_default()
            .data
            .len()
    });
    Ok(size as u32)
}

/// Gets the code hash of the account at the given address.
pub fn account_codehash(
    mut env: WASMEnvMut,
    address: GuestPtr, // *const u8,
    dest: GuestPtr,    // *mut u8
) -> MaybeEscape {
    let host = HostInfo::from_env(&mut env)?;
    let address = host.read_address(address)?;
    let hash = with_runtime_context(|runtime_context| {
        runtime_context
            .host
            .code_hash(address)
            .unwrap_or_default()
            .data
            .to_be_bytes()
    });
    host.write_slice(dest, &hash)?;
    Ok(())
}

/// Load storage slot, if storage is not present inside the account then it will be loaded from database.
pub fn sload(
    mut env: WASMEnvMut,
    key: GuestPtr,  // *const u8,
    dest: GuestPtr, // *mut u8
) -> MaybeEscape {
    let host = HostInfo::from_env(&mut env)?;
    let key = host.read_bytes32(key)?;
    let value = with_runtime_context(|runtime_context| {
        let target_address = runtime_context.contract.target_address;
        runtime_context
            .host
            .sload(target_address, key)
            .unwrap_or_default()
            .data
    });
    host.write_slice(dest, &value.to_be_bytes())?;
    Ok(())
}

/// Storage change of storage slot, before storing `sload` will be called for that slot.
pub fn sstore(
    mut env: WASMEnvMut,
    key: GuestPtr,   // *const u8,
    value: GuestPtr, // *const u8
) -> MaybeEscape {
    let host = HostInfo::from_env(&mut env)?;
    let key = host.read_bytes32(key)?;
    let value = host.read_bytes32(value)?;
    with_runtime_context(|runtime_context| {
        let target_address = runtime_context.contract.target_address;
        runtime_context
            .host
            .sstore(target_address, key, value)
            .unwrap_or_default();
    });
    Ok(())
}

/// Returns the transient storage value.
pub fn tload(
    mut env: WASMEnvMut,
    key: GuestPtr,  // *const u8,
    dest: GuestPtr, // *mut u8
) -> MaybeEscape {
    let host = HostInfo::from_env(&mut env)?;
    let key = host.read_bytes32(key)?;
    let value = with_runtime_context(|runtime_context| {
        let target_address = runtime_context.contract.target_address;
        runtime_context.host.tload(target_address, key)
    });
    host.write_slice(dest, &value.to_be_bytes())?;
    Ok(())
}

/// Stores the transient storage value.
pub fn tstore(
    mut env: WASMEnvMut,
    key: GuestPtr,   // *const u8,
    value: GuestPtr, // *const u8
) -> MaybeEscape {
    let host = HostInfo::from_env(&mut env)?;
    let key = host.read_bytes32(key)?;
    let value = host.read_bytes32(value)?;
    with_runtime_context(|runtime_context| {
        let target_address = runtime_context.contract.target_address;
        runtime_context.host.tstore(target_address, key, value);
    });
    Ok(())
}

/// Gets the block hash for a given block number.
pub fn block_hash(
    mut env: WASMEnvMut,
    number: u64,    // u64,
    dest: GuestPtr, // *mut u8
) -> MaybeEscape {
    let host = HostInfo::from_env(&mut env)?;
    let hash = with_runtime_context(|runtime_context| runtime_context.host.block_hash(number))
        .unwrap_or_default();
    host.write_slice(dest, &hash.to_be_bytes())?;
    Ok(())
}

/// Gets the block prevrandao value.
pub fn block_prevrandao(
    mut env: WASMEnvMut,
    dest: GuestPtr, // *mut u8
) -> MaybeEscape {
    let host = HostInfo::from_env(&mut env)?;
    let prevrandao =
        with_runtime_context(|runtime_context| runtime_context.host.env().block.prevrandao)
            .unwrap_or_default();
    host.write_slice(dest, &prevrandao.0)?;
    Ok(())
}

/// Gets the basefee of the current block.
pub fn block_basefee(
    mut env: WASMEnvMut,
    dest: GuestPtr, // *mut u8
) -> MaybeEscape {
    let host = HostInfo::from_env(&mut env)?;
    let basefee = with_runtime_context(|runtime_context| runtime_context.host.env().block.basefee);
    host.write_slice(dest, &basefee.to_be_bytes_vec())?;
    Ok(())
}

/// Gets the basefee of the current block.
pub fn block_blobbasefee(
    mut env: WASMEnvMut,
    dest: GuestPtr, // *mut u8
) -> MaybeEscape {
    let host = HostInfo::from_env(&mut env)?;
    let basefee = with_runtime_context(|runtime_context| {
        runtime_context
            .host
            .env()
            .block
            .blob_excess_gas_and_price
            .clone()
            .unwrap_or_default()
            .blob_gasprice
    });
    let basefee = Bytes32::from(basefee);
    host.write_slice(dest, &basefee.to_be_bytes())?;
    Ok(())
}

/// Gets the coinbase of the current block.
pub fn block_coinbase(
    mut env: WASMEnvMut,
    dest: GuestPtr, // *mut u8
) -> MaybeEscape {
    let host = HostInfo::from_env(&mut env)?;
    let coinbase =
        with_runtime_context(|runtime_context| runtime_context.host.env().block.coinbase);
    host.write_slice(dest, &coinbase.0.0)?;
    Ok(())
}

/// Gets the gas limit of the current block.
pub fn block_gas_limit(mut _env: WASMEnvMut) -> EscapeResult<u64> {
    let gas_limit =
        with_runtime_context(|runtime_context| runtime_context.host.env().block.gas_limit);
    Ok(as_u64_saturated!(gas_limit))
}

/// Gets the number of ancestor blocks of this block (block height).
pub fn block_number(mut _env: WASMEnvMut) -> EscapeResult<u64> {
    let block_number =
        with_runtime_context(|runtime_context| runtime_context.host.env().block.number);
    Ok(as_u64_saturated!(block_number))
}

/// Gets the timestamp of the block in seconds since the UNIX epoch.
pub fn block_timestamp(mut _env: WASMEnvMut) -> EscapeResult<u64> {
    let block_timestamp =
        with_runtime_context(|runtime_context| runtime_context.host.env().block.timestamp);
    Ok(as_u64_saturated!(block_timestamp))
}

/// Gets the unique chain identifier.
pub fn chainid(mut _env: WASMEnvMut) -> EscapeResult<u64> {
    let chainid = with_runtime_context(|runtime_context| runtime_context.host.env().cfg.chain_id);
    Ok(chainid)
}

/// Gets the unique chain identifier.
pub fn chainid_u128_dest(mut env: WASMEnvMut, dest: GuestPtr) -> MaybeEscape {
    let host = HostInfo::from_env(&mut env)?;
    let chainid = with_runtime_context(|runtime_context| runtime_context.host.env().cfg.chain_id);
    host.write_slice(dest, &(chainid as u128).to_le_bytes())?;
    Ok(())
}

/// Calls the contract at the given address with options for passing value and to limit the
/// amount of gas supplied.
pub fn call(
    mut env: WASMEnvMut,
    contract: GuestPtr, // *const u8,
    calldata: GuestPtr, // *const u8,
    calldata_len: u32,  // usize,
    value: GuestPtr,    // *const u8,
    gas_limit: u64,
    return_data_len: GuestPtr, // *mut usize,
) -> EscapeResult<u8> {
    let host = HostInfo::from_env(&mut env)?;
    let to = host.read_address(contract)?;
    let value = host.read_bytes32(value)?.to_u256();
    let call_data = host.read_slice(calldata, calldata_len)?;
    let result = with_runtime_context(|runtime_context| {
        intern_call(
            runtime_context,
            CallType::Call,
            to,
            value,
            call_data,
            gas_limit,
        )
    })?;
    host.write_u32(return_data_len, result.1)?;
    Ok(result.0)
}

/// Calls code at a given address.
pub fn call_code(
    mut env: WASMEnvMut,
    gas_limit: u64,
    contract: GuestPtr, // *const u8,
    calldata: GuestPtr, // *const u8,
    calldata_len: u32,  // usize,
) -> EscapeResult<u8> {
    let host = HostInfo::from_env(&mut env)?;
    let to = host.read_address(contract)?;
    let value = U256::ZERO;
    let call_data = host.read_slice(calldata, calldata_len)?;
    let result = with_runtime_context(|runtime_context| {
        intern_call(
            runtime_context,
            CallType::Callcode,
            to,
            value,
            call_data,
            gas_limit,
        )
    })?;
    Ok(result.0)
}

/// Performs a delegate call to a contract at a given address.
pub fn call_delegate(
    mut env: WASMEnvMut,
    gas_limit: u64,
    contract: GuestPtr, // *const u8,
    calldata: GuestPtr, // *const u8,
    calldata_len: u32,  // usize,
) -> EscapeResult<u8> {
    let host = HostInfo::from_env(&mut env)?;
    let to = host.read_address(contract)?;
    let value = U256::ZERO;
    let call_data = host.read_slice(calldata, calldata_len)?;
    let result = with_runtime_context(|runtime_context| {
        intern_call(
            runtime_context,
            CallType::Delegatecall,
            to,
            value,
            call_data,
            gas_limit,
        )
    })?;
    Ok(result.0)
}

/// Performs a static call to a contract at a given address.
pub fn call_static(
    mut env: WASMEnvMut,
    gas_limit: u64,
    contract: GuestPtr, // *const u8,
    calldata: GuestPtr, // *const u8,
    calldata_len: u32,  // usize,
) -> EscapeResult<u8> {
    let host = HostInfo::from_env(&mut env)?;
    let to = host.read_address(contract)?;
    let value = U256::ZERO;
    let call_data = host.read_slice(calldata, calldata_len)?;
    let result = with_runtime_context(|runtime_context| {
        intern_call(
            runtime_context,
            CallType::Staticcall,
            to,
            value,
            call_data,
            gas_limit,
        )
    })?;
    Ok(result.0)
}

/// Calls the contract at the given address with options for passing value and to limit the
/// amount of gas supplied.
pub fn call_contract(
    mut env: WASMEnvMut,
    gas_limit: u64,
    contract: GuestPtr, // *const u8,
    value: GuestPtr,    // *const u8,
    calldata: GuestPtr, // *const u8,
    calldata_len: u32,  // usize,
) -> EscapeResult<u8> {
    let host = HostInfo::from_env(&mut env)?;
    let to = host.read_address(contract)?;
    let value = host.read_bytes32(value)?.to_u256();
    let call_data = host.read_slice(calldata, calldata_len)?;
    let result = with_runtime_context(|runtime_context| {
        intern_call(
            runtime_context,
            CallType::Call,
            to,
            value,
            call_data,
            gas_limit,
        )
    })?;
    Ok(result.0)
}

/// Delegate calls the contract at the given address, with the option to limit the amount of
/// gas supplied.
pub fn delegate_call(
    mut env: WASMEnvMut,
    contract: GuestPtr, // *const u8,
    calldata: GuestPtr, // *const u8,
    calldata_len: u32,  // usize,
    gas_limit: u64,
    return_data_len: GuestPtr, // *mut usize,
) -> EscapeResult<u8> {
    let host = HostInfo::from_env(&mut env)?;
    let to = host.read_address(contract)?;
    let call_data = host.read_slice(calldata, calldata_len)?;
    let result = with_runtime_context(|runtime_context| {
        intern_call(
            runtime_context,
            CallType::Delegatecall,
            to,
            U256::ZERO,
            call_data,
            gas_limit,
        )
    })?;
    host.write_u32(return_data_len, result.1)?;
    Ok(result.0)
}

/// Static calls the contract at the given address, with the option to limit the amount of gas
/// supplied.
pub fn static_call(
    mut env: WASMEnvMut,
    contract: GuestPtr, // *const u8,
    calldata: GuestPtr, // *const u8,
    calldata_len: u32,  // usize,
    gas_limit: u64,
    return_data_len: GuestPtr, // *mut usize
) -> EscapeResult<u8> {
    let host = HostInfo::from_env(&mut env)?;
    let to = host.read_address(contract)?;
    let call_data = host.read_slice(calldata, calldata_len)?;
    let result = with_runtime_context(|runtime_context| {
        intern_call(
            runtime_context,
            CallType::Staticcall,
            to,
            U256::ZERO,
            call_data,
            gas_limit,
        )
    })?;
    host.write_u32(return_data_len, result.1)?;
    Ok(result.0)
}

/// Self-destructs the current contract and sends any remaining balance to the specified address.
pub fn selfdestruct(
    mut env: WASMEnvMut,
    address: GuestPtr, // *const u8,
) -> MaybeEscape {
    let host = HostInfo::from_env(&mut env)?;
    let address = host.read_address(address)?;
    with_runtime_context(|runtime_context| {
        runtime_context
            .host
            .selfdestruct(runtime_context.contract.target_address, address)
    });
    Ok(())
}

/// Gets the address of the current program.
pub fn contract_address(
    mut env: WASMEnvMut,
    address: GuestPtr, // *mut u8
) -> MaybeEscape {
    let host = HostInfo::from_env(&mut env)?;
    let target_address =
        with_runtime_context(|runtime_context| runtime_context.contract.target_address);
    host.write_slice(address, target_address.as_slice())?;
    Ok(())
}

/// Deploys a new contract using the init code provided.
pub fn create(
    mut env: WASMEnvMut,
    code: GuestPtr,            // *const u8,
    code_len: u32,             // usize,
    endowment: GuestPtr,       // *const u8,
    contract: GuestPtr,        // *mut u8,
    revert_data_len: GuestPtr, // *mut usize,
) -> MaybeEscape {
    let host = HostInfo::from_env(&mut env)?;
    let code = host.read_slice(code, code_len)?;
    let value = host.read_bytes32(endowment)?.to_u256();
    let result = with_runtime_context(|runtime_context| {
        intern_create(runtime_context, code, value, None, u64::MAX)
    })?;
    host.write_slice(contract, result.0.as_slice())?;
    host.write_u32(revert_data_len, result.1)?;
    Ok(())
}

/// Deploys a new contract using the init code provided.
#[allow(clippy::too_many_arguments)]
pub fn create_contract(
    mut env: WASMEnvMut,
    endowment: GuestPtr, // *const u8,
    code: GuestPtr,      // *const u8,
    code_len: u32,       // usize,
    _data: GuestPtr,     // *const u8,
    _data_len: u32,      // usize,
    salt: GuestPtr,      // *const u8,
    is_create2: u32,     // bool,
    contract: GuestPtr,  // *mut u8,
) -> EscapeResult<u32> {
    let host = HostInfo::from_env(&mut env)?;
    let code = host.read_slice(code, code_len)?;
    let value = host.read_bytes32(endowment)?.to_u256();
    let salt = host.read_bytes32(salt)?.to_b256();
    let result = with_runtime_context(|runtime_context| {
        intern_create(
            runtime_context,
            code,
            value,
            if is_create2 > 0 { Some(salt) } else { None },
            u64::MAX,
        )
    })?;
    host.write_slice(contract, result.0.as_slice())?;
    Ok(result.1)
}

/// Deploys a new contract using the init code provided.
pub fn create2(
    mut env: WASMEnvMut,
    code: GuestPtr,            // *const u8,
    code_len: u32,             // usize,
    endowment: GuestPtr,       // *const u8,
    salt: GuestPtr,            // *const u8,
    contract: GuestPtr,        // *mut u8,
    revert_data_len: GuestPtr, // *mut usize,
) -> MaybeEscape {
    let host = HostInfo::from_env(&mut env)?;
    let code = host.read_slice(code, code_len)?;
    let value = host.read_bytes32(endowment)?.to_u256();
    let salt = host.read_bytes32(salt)?.to_b256();
    let result = with_runtime_context(|runtime_context| {
        intern_create(runtime_context, code, value, Some(salt), u64::MAX)
    })?;
    host.write_slice(contract, result.0.as_slice())?;
    host.write_u32(revert_data_len, result.1)?;
    Ok(())
}

/// Emits the log with the given number of topics and data.
pub fn emit_log(
    mut env: WASMEnvMut,
    data: GuestPtr, // *const u8,
    len: u32,       // usize,
    topics: u32,    // usize
) -> MaybeEscape {
    debug_assert!(topics <= 4);
    let host = HostInfo::from_env(&mut env)?;
    let data = host.read_slice(data, len)?;
    with_runtime_context(|runtime_context| {
        runtime_context.host.log(Log {
            address: runtime_context.contract.target_address,
            data: LogData::new_unchecked(vec![], data.into()),
        });
    });
    Ok(())
}

/// Emits the log with the given number of topics and data.
#[allow(clippy::too_many_arguments)]
pub fn emit_log_event(
    mut env: WASMEnvMut,
    data: GuestPtr, // *const u8,
    len: u32,       // usize,
    topics: u32,    // usize
    topic1: GuestPtr,
    topic2: GuestPtr,
    topic3: GuestPtr,
    topic4: GuestPtr,
) -> MaybeEscape {
    debug_assert!(topics <= 4);
    let host = HostInfo::from_env(&mut env)?;
    let data = host.read_slice(data, len)?;
    let topics = match topics {
        0 => vec![],
        1 => vec![host.read_fixed(topic1)?.into()],
        2 => vec![
            host.read_fixed(topic1)?.into(),
            host.read_fixed(topic2)?.into(),
        ],
        3 => vec![
            host.read_fixed(topic1)?.into(),
            host.read_fixed(topic2)?.into(),
            host.read_fixed(topic3)?.into(),
        ],
        4 => vec![
            host.read_fixed(topic1)?.into(),
            host.read_fixed(topic2)?.into(),
            host.read_fixed(topic3)?.into(),
            host.read_fixed(topic4)?.into(),
        ],
        _ => return Err(Escape::Exit(ExitStatusCode::FatalExternalError.to_u8())),
    };
    with_runtime_context(|runtime_context| {
        runtime_context.host.log(Log {
            address: runtime_context.contract.target_address,
            data: LogData::new_unchecked(topics, data.into()),
        });
    });
    Ok(())
}

/// Gets the amount of gas left after paying for the cost of this hostio.
pub fn gas_left(mut _env: WASMEnvMut) -> EscapeResult<u64> {
    let gas = with_runtime_context(|runtime_context| runtime_context.gas_remaining());
    Ok(gas)
}

/// Gets the limit gas of the execution.
pub fn gas_limit() -> u64 {
    with_runtime_context(|runtime_context| runtime_context.gas_limit())
}

/// Gets the address of the account that called the program.
pub fn msg_sender(
    mut env: WASMEnvMut,
    sender: GuestPtr, // *mut u8
) -> MaybeEscape {
    let host = HostInfo::from_env(&mut env)?;
    let msg_sender = with_runtime_context(|runtime_context| runtime_context.contract.caller);
    host.write_slice(sender, msg_sender.as_slice())?;
    Ok(())
}

/// Get the ETH value in wei sent to the program.
pub fn msg_value(
    mut env: WASMEnvMut,
    value: GuestPtr, // *mut u8
) -> MaybeEscape {
    let host = HostInfo::from_env(&mut env)?;
    let msg_value = with_runtime_context(|runtime_context| runtime_context.contract.call_value);
    host.write_slice(value, &msg_value.to_be_bytes_vec())?;
    Ok(())
}

/// Efficiently computes the [`keccak256`] hash of the given preimage.
pub fn keccak256(
    mut env: WASMEnvMut,
    bytes: GuestPtr,  // *const u8,
    len: u32,         // usize,
    output: GuestPtr, // *mut u8
) -> MaybeEscape {
    let host = HostInfo::from_env(&mut env)?;
    let data = host.read_slice(bytes, len)?;
    let hash = if len == 0 {
        EMPTY_CODE_HASH_BYTES
    } else {
        *native_keccak256(data)
    };
    host.write_slice(output, &hash)?;
    Ok(())
}

/// Efficiently computes the [`sha256`] hash of the given preimage.
pub fn sha256(
    mut env: WASMEnvMut,
    bytes: GuestPtr,  // *const u8,
    len: u32,         // usize,
    output: GuestPtr, // *mut u8
) -> MaybeEscape {
    let host = HostInfo::from_env(&mut env)?;
    let data = host.read_slice(bytes, len)?;
    let hash = sha2::Sha256::digest(data.as_slice());
    host.write_slice(output, &hash)?;
    Ok(())
}

/// Copies the contract code into a buffer.
pub fn code_copy(
    mut env: WASMEnvMut,
    dest: GuestPtr, // *mut u8,
    offset: u32,    // usize,
    size: u32,      // usize,
) -> MaybeEscape {
    let host = HostInfo::from_env(&mut env)?;
    let code = with_runtime_context(|runtime_context| runtime_context.contract.code.clone());
    let code_slice = data_slice(code.bytecode(), offset, size);
    host.write_slice(dest, code_slice)?;
    Ok(())
}

/// Gets the size of the contract code.
pub fn code_size(mut _env: WASMEnvMut) -> EscapeResult<u32> {
    let code_size = with_runtime_context(|runtime_context| runtime_context.contract.code.len());
    Ok(code_size as u32)
}

/// Reads the program calldata.
pub fn call_data_copy(
    mut env: WASMEnvMut,
    dest: GuestPtr, // *mut u8
) -> MaybeEscape {
    let host = HostInfo::from_env(&mut env)?;
    let data = with_runtime_context(|runtime_context| runtime_context.contract.input.clone());
    host.write_slice(dest, data_slice(&data, 0, data.len() as u32))?;
    Ok(())
}

/// Reads the program calldata with offset and size
pub fn call_data_copy_with_size(
    mut env: WASMEnvMut,
    dest: GuestPtr, // *mut u8
    offset: u32,    // usize,
    size: u32,      // usize
) -> MaybeEscape {
    let host = HostInfo::from_env(&mut env)?;
    let data = with_runtime_context(|runtime_context| runtime_context.contract.input.clone());
    host.write_slice(dest, data_slice(&data, offset, size))?;
    Ok(())
}

/// Gets the size of the program calldata.
pub fn call_data_size(mut _env: WASMEnvMut) -> EscapeResult<u32> {
    let data = with_runtime_context(|runtime_context| runtime_context.contract.input.clone());
    Ok(data.len() as u32)
}

/// Copies the bytes of the last VM call or deployment return result.
pub fn return_data_copy(
    mut env: WASMEnvMut,
    dest: GuestPtr, // *mut u8,
    offset: u32,    // usize,
    size: u32,      // usize
) -> EscapeResult<u32> {
    let host = HostInfo::from_env(&mut env)?;
    let data = with_runtime_context(|runtime_context| runtime_context.return_bytes());
    let data = data_slice(&data, offset, size);
    host.write_slice(dest, data)?;
    Ok(data.len() as u32)
}

/// Copies the bytes of the last VM call or deployment return result.
pub fn return_data_copy_without_return_size(
    env: WASMEnvMut,
    dest: GuestPtr, // *mut u8,
    offset: u32,    // usize,
    size: u32,      // usize
) -> MaybeEscape {
    return_data_copy(env, dest, offset, size)?;
    Ok(())
}

/// Copies the contract code into a buffer.
pub fn external_code_copy(
    env: WASMEnvMut,
    address: GuestPtr, // *const u8,
    dest: GuestPtr,    // *mut u8,
    offset: u32,       // usize,
    size: u32,         // usize,
) -> MaybeEscape {
    account_code(env, address, offset, size, dest)?;
    Ok(())
}

/// Writes the final return data. If not called before the program exists, the return data will
/// be 0 bytes long. Note that this hostio does not cause the program to exit.
pub fn write_result(
    mut env: WASMEnvMut,
    data: GuestPtr, // *const u8,
    len: u32,       // usize
) -> MaybeEscape {
    let host = HostInfo::from_env(&mut env)?;
    let data = host.read_slice(data, len)?;
    with_runtime_context(|runtime_context| {
        runtime_context.set_returndata(data);
    });
    Ok(())
}

/// Reverts the program, which will cause the program to exit with a status code of `1`.
pub fn revert(
    env: WASMEnvMut,
    data: GuestPtr, // *const u8,
    len: u32,       // usize
) -> MaybeEscape {
    write_result(env, data, len)?;
    with_runtime_context(|runtime_context| {
        runtime_context.set_exit_status(ExitStatusCode::Revert);
    });
    Ok(())
}

/// Prints a 32-bit integer to the console, which can be either signed or unsigned.
/// Only available in debug mode.
pub fn debug_i32(_env: WASMEnvMut, value: i32) {
    println!("{value}");
}

/// Prints a 64-bit integer to the console, which can be either signed or unsigned.
/// Only available in debug mode.
pub fn debug_i64(_env: WASMEnvMut, value: i64) {
    println!("{value}");
}

/// Prints a UTF-8 encoded string to the console. Only available in debug mode.
pub fn debug_bytes(mut env: WASMEnvMut, bytes: GuestPtr, len: u32) -> MaybeEscape {
    if std::env::var(DORA_DISABLE_CONSOLE).is_err() {
        let host = HostInfo::from_env(&mut env)?;
        let data = host.read_slice(bytes, len)?;
        let string = String::from_utf8_lossy(&data);
        println!("{string}");
    }
    Ok(())
}

/// Returns the length of the last VM call or deployment return result.
pub fn return_data_size(mut _env: WASMEnvMut) -> u32 {
    with_runtime_context(|runtime_context| runtime_context.return_data_size() as u32)
}

/// Gets the gas price in wei per gas.
pub fn gas_price(
    mut env: WASMEnvMut,
    gas_price: GuestPtr, // *mut u8
) -> MaybeEscape {
    let host = HostInfo::from_env(&mut env)?;
    let price = with_runtime_context(|runtime_context| runtime_context.host.env().tx.gas_price);
    host.write_slice(gas_price, &price.to_be_bytes_vec())?;
    Ok(())
}

/// Gets the top-level sender of the transaction.
pub fn tx_origin(
    mut env: WASMEnvMut,
    origin: GuestPtr, // *mut u8
) -> MaybeEscape {
    let host = HostInfo::from_env(&mut env)?;
    let caller = with_runtime_context(|runtime_context| runtime_context.host.env().tx.caller);
    host.write_slice(origin, caller.as_slice())?;
    Ok(())
}

fn intern_call(
    runtime_context: &mut RuntimeContext<'_>,
    call_type: CallType,
    to: Address,
    value: U256,
    call_data: Vec<u8>,
    gas_limit: u64,
) -> EscapeResult<(u8, u32)> {
    // Load account and calculate gas cost.
    let mut account_load = match runtime_context.host.load_account_delegated(to) {
        Some(account_load) => account_load,
        None => return Err(Escape::Exit(ExitStatusCode::FatalExternalError.to_u8())),
    };
    if call_type != CallType::Call {
        account_load.is_empty = false;
    }
    let call_msg = CallMessage {
        kind: call_type.into(),
        input: call_data.into(),
        value: if call_type == CallType::Delegatecall {
            runtime_context.contract.call_value
        } else {
            value
        },
        depth: runtime_context.inner.depth as u32,
        gas_limit,
        caller: if call_type == CallType::Delegatecall {
            runtime_context.contract.caller
        } else {
            runtime_context.contract.target_address
        },
        salt: None,
        recipient: if matches!(call_type, CallType::Delegatecall | CallType::Callcode) {
            runtime_context.contract.target_address
        } else {
            to
        },
        code_address: to,
        is_static: runtime_context.inner.is_static || call_type == CallType::Staticcall,
        is_eof_init: false,
        validate_eof: true,
    };
    let call_result = runtime_context
        .host
        .call(call_msg)
        .unwrap_or_else(|_| CallResult::new_with_gas_limit(gas_limit));
    let output_size = call_result.output.len() as u32;
    runtime_context.set_returndata(call_result.output.to_vec());
    // Check the error message. 0 denotes success, 1 denotes failure.
    if call_result.status.is_ok() {
        Ok((0, output_size))
    } else {
        Ok((1, output_size))
    }
}

fn intern_create(
    runtime_context: &mut RuntimeContext<'_>,
    code: Vec<u8>,
    value: U256,
    salt: Option<B256>,
    gas_limit: u64,
) -> EscapeResult<(Address, u32)> {
    let call_msg = CallMessage {
        kind: if salt.is_some() {
            CallKind::Create2
        } else {
            CallKind::Create
        },
        input: code.into(),
        value,
        depth: runtime_context.inner.depth as u32,
        gas_limit,
        caller: runtime_context.contract.target_address,
        salt,
        recipient: Address::default(),
        code_address: Address::default(),
        is_static: runtime_context.inner.is_static,
        is_eof_init: false,
        validate_eof: true,
    };
    let call_result = match runtime_context.host.call(call_msg) {
        Ok(result) => result,
        Err(_) => {
            return Err(Escape::Exit(ExitStatusCode::FatalExternalError.to_u8()));
        }
    };
    let return_data = if call_result.status.is_revert() {
        call_result.output.to_vec()
    } else {
        Vec::new()
    };
    let return_data_size = return_data.len() as u32;
    runtime_context.set_returndata(return_data);

    // Check the error message.
    if call_result.status.is_ok() {
        // Set created address to the value.
        Ok((
            call_result.create_address.unwrap_or_default(),
            return_data_size,
        ))
    } else {
        Ok((Address::ZERO, return_data_size))
    }
}

fn data_slice(data: &[u8], offset: u32, size: u32) -> &[u8] {
    let offset = offset as usize;
    let size = size as usize;
    if size != 0 {
        let offset = offset.min(data.len());
        let end = min(offset + size, data.len());
        unsafe { data.get_unchecked(offset..end) }
    } else {
        &[]
    }
}

// Mocked Arbitrum Stylus Host Functions, just for test
// Reference: https://github.com/OffchainLabs/stylus-sdk-rs/blob/main/stylus-sdk/src/hostio.rs

/// Whether the current call is reentrant.
pub fn msg_reentrant(mut _env: WASMEnvMut) -> u32 {
    0
}

pub fn tx_ink_price(mut _env: WASMEnvMut) -> u32 {
    0
}

/// Pay for the WASM memory grow gas cost
pub fn pay_for_memory_grow(mut _env: WASMEnvMut, pages: u16) -> MaybeEscape {
    if pages == 0 {
        Ok(())
    } else {
        let model = MemoryModel::default();
        let gas = model.gas_cost(pages, 0, 0);
        with_runtime_context(|runtime_context| {
            let gas_limit = runtime_context.gas_limit();
            if gas > gas_limit {
                Err(Escape::OutOfGas)
            } else {
                runtime_context.update_gas_limit(gas_limit);
                Ok(())
            }
        })
    }
}

pub fn storage_flush_cache(mut _env: WASMEnvMut, _clear: u32) {
    // Nothing to do here
}
