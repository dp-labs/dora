#![allow(unused)]

use super::{
    env::WASMEnvMut,
    errors::{Escape, EscapeResult, MaybeEscape},
};

/// Gets the ETH balance in wei of the account at the given address.
pub fn account_balance(
    env: WASMEnvMut,
    address: i32, // *const u8
    dest: i32,    // *mut u8
) -> MaybeEscape {
    Ok(())
}

/// Gets a subset of the code from the account at the given address.
pub fn account_code(
    env: WASMEnvMut,
    address: i32, // *const u8,
    offset: i32,  // usize,
    size: i32,    // usize,
    dest: i32,    // *mut u8,
) -> EscapeResult<i32> {
    Ok(0)
}

/// Gets the size of the code in bytes at the given address.
pub fn account_code_size(
    env: WASMEnvMut,
    address: i32, // *const u8
) -> EscapeResult<i32> {
    Ok(0)
}

/// Gets the code hash of the account at the given address.
pub fn account_codehash(
    env: WASMEnvMut,
    address: i32, // *const u8,
    dest: i32,    // *mut u8
) -> MaybeEscape {
    Ok(())
}

/// Load storage slot, if storage is not present inside the account then it will be loaded from database.
pub fn sload(
    env: WASMEnvMut,
    address: i32, // *const u8,
    index: i32,   // *const u8,
    dest: i32,    // *mut u8
) -> MaybeEscape {
    Ok(())
}

/// Storage change of storage slot, before storing `sload` will be called for that slot.
pub fn sstore(
    env: WASMEnvMut,
    address: i32, // *const u8,
    index: i32,   // *const u8,
    value: i32,   // *const u8
) -> MaybeEscape {
    Ok(())
}

/// Returns the transient storage value.
pub fn tload(
    env: WASMEnvMut,
    address: i32, // *const u8,
    index: i32,   // *const u8,
    dest: i32,    // *mut u8
) -> MaybeEscape {
    Ok(())
}

/// Stores the transient storage value.
pub fn tstore(
    env: WASMEnvMut,
    address: i32, // *const u8,
    index: i32,   // *const u8,
    value: i32,   // *const u8
) -> MaybeEscape {
    Ok(())
}

/// Gets the basefee of the current block.
pub fn block_basefee(
    env: WASMEnvMut,
    basefee: i32, // *mut u8
) -> MaybeEscape {
    Ok(())
}

/// Gets the coinbase of the current block.
pub fn block_coinbase(
    env: WASMEnvMut,
    coinbase: i32, // *mut u8
) -> MaybeEscape {
    Ok(())
}

/// Gets the gas limit of the current block.
pub fn block_gas_limit(env: WASMEnvMut) -> u64 {
    0
}

/// Gets the number of ancestor blocks of this block (block height).
pub fn block_number(env: WASMEnvMut) -> u64 {
    0
}

/// Gets the timestamp of the block in seconds since the UNIX epoch.
pub fn block_timestamp(env: WASMEnvMut) -> u64 {
    0
}

/// Gets the unique chain identifier.
pub fn chainid(env: WASMEnvMut) -> EscapeResult<u64> {
    Ok(0)
}

/// Calls the contract at the given address with options for passing value and to limit the
/// amount of gas supplied.
pub fn call(
    env: WASMEnvMut,
    contract: i32,     // *const u8,
    calldata: i32,     // *const u8,
    calldata_len: i32, // usize,
    value: i32,        // *const u8,
    gas: u64,
    return_data_len: i32, // *mut usize,
) -> EscapeResult<u8> {
    Ok(0)
}

/// Delegate calls the contract at the given address, with the option to limit the amount of
/// gas supplied.
pub fn delegate_call(
    env: WASMEnvMut,
    contract: i32,     // *const u8,
    calldata: i32,     // *const u8,
    calldata_len: i32, // usize,
    gas: u64,
    return_data_len: i32, // *mut usize,
) -> EscapeResult<u8> {
    Ok(0)
}

/// Static calls the contract at the given address, with the option to limit the amount of gas
/// supplied.
pub fn static_call(
    env: WASMEnvMut,
    contract: i32,     // *const u8,
    calldata: i32,     // *const u8,
    calldata_len: i32, // usize,
    gas: u64,
    return_data_len: i32, // *mut usize
) -> EscapeResult<u8> {
    Ok(0)
}

/// Gets the address of the current program.
pub fn contract_address(
    env: WASMEnvMut,
    address: i32, // *mut u8
) {
}

/// Deploys a new contract using the init code provided.
pub fn create(
    env: WASMEnvMut,
    code: i32,            // *const u8,
    code_len: i32,        // usize,
    endowment: i32,       // *const u8,
    contract: i32,        // *mut u8,
    revert_data_len: i32, // *mut usize,
) {
}

/// Deploys a new contract using the init code provided.
pub fn create2(
    env: WASMEnvMut,
    code: i32,            // *const u8,
    code_len: i32,        // usize,
    endowment: i32,       // *const u8,
    salt: i32,            // *const u8,
    contract: i32,        // *mut u8,
    revert_data_len: i32, // *mut usize,
) {
}

/// Emits an EVM log with the given number of topics and data.
pub fn emit_log(
    env: WASMEnvMut,
    data: i32,   // *const u8,
    len: i32,    // usize,
    topics: i32, // usize
) {
}

/// Gets the amount of gas left after paying for the cost of this hostio.
pub fn gas_left(env: WASMEnvMut) -> u64 {
    0
}

/// Gets the address of the account that called the program.
pub fn msg_sender(
    env: WASMEnvMut,
    sender: i32, // *mut u8
) {
}

/// Get the ETH value in wei sent to the program.
pub fn msg_value(
    env: WASMEnvMut,
    value: i32, // *mut u8
) {
}

/// Efficiently computes the [`keccak256`] hash of the given preimage.
pub fn keccak256(
    env: WASMEnvMut,
    bytes: i32,  // *const u8,
    len: i32,    // usize,
    output: i32, // *mut u8
) {
}

/// Reads the program calldata.
pub fn call_data_copy(
    env: WASMEnvMut,
    dest: i32, // *mut u8
) {
}

/// Copies the bytes of the last EVM call or deployment return result.
pub fn return_data_copy(
    env: WASMEnvMut,
    dest: i32,   // *mut u8,
    offset: i32, // usize,
    size: i32,   // usize
) -> i32 {
    0
}

/// Writes the final return data. If not called before the program exists, the return data will
/// be 0 bytes long. Note that this hostio does not cause the program to exit.
pub fn write_result(
    env: WASMEnvMut,
    data: i32, // *const u8,
    len: i32,  // usize
) {
}

/// Returns the length of the last EVM call or deployment return result.
pub fn return_data_size(env: WASMEnvMut) -> i32 {
    0
}

/// Gets the gas price in wei per gas.
pub fn gas_price(
    env: WASMEnvMut,
    gas_price: i32, // *mut u8
) {
}

/// Gets the top-level sender of the transaction.
pub fn tx_origin(
    env: WASMEnvMut,
    origin: i32, // *mut u8
) {
}
