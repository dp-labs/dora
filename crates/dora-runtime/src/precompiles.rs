use crate::constants::precompiles::{sha2_256_dynamic_cost, SHA2_256_COST};
use dora_primitives::{Bytes, PrecompileOutput};
use revm_precompile::{
    blake2::run as blake2_run, hash::ripemd160_run, identity::identity_run, modexp::berlin_run,
    secp256k1::ec_recover_run, PrecompileError, PrecompileErrors,
};
use sha2::{Digest, Sha256};

/// Current implementation serves as a baseline:
/// + Five functions reuse the REVM implementation
/// + `sha2_256` utilizes the `sha2` crate with the ASM feature enabled
/// + Future performance optimizations may consider porting high-performance SIMD implementations
///   ECRECOVER precompile implementation.
#[inline]
pub fn ecrecover(calldata: &Bytes, gas_limit: u64) -> Result<PrecompileOutput, PrecompileErrors> {
    ec_recover_run(calldata, gas_limit)
}

/// IDENTITY precompile implementation.
#[inline]
pub fn identity(calldata: &Bytes, gas_limit: u64) -> Result<PrecompileOutput, PrecompileErrors> {
    identity_run(calldata, gas_limit)
}

/// SHA-256 precompile implementation.
pub fn sha2_256(calldata: &Bytes, gas_limit: u64) -> Result<PrecompileOutput, PrecompileErrors> {
    let gas_cost = SHA2_256_COST + sha2_256_dynamic_cost(calldata.len() as u64);
    if gas_limit < gas_cost {
        return Err(PrecompileErrors::Error(PrecompileError::OutOfGas));
    }
    let mut hasher = Sha256::new();
    hasher.update(calldata);
    let hash = hasher.finalize();
    Ok(PrecompileOutput {
        bytes: Bytes::copy_from_slice(&hash),
        gas_used: gas_cost,
    })
}

/// RIPEMD-160 precompile implementation.
#[inline]
pub fn ripemd_160(calldata: &Bytes, gas_limit: u64) -> Result<PrecompileOutput, PrecompileErrors> {
    ripemd160_run(calldata, gas_limit)
}

/// Modular exponentiation precompile (MOD_EXP) implementation.
#[inline]
pub fn modexp(calldata: &Bytes, gas_limit: u64) -> Result<PrecompileOutput, PrecompileErrors> {
    berlin_run(calldata, gas_limit)
}

/// BLAKE2 F precompile implementation.
#[inline]
pub fn blake2f(calldata: &Bytes, gas_limit: u64) -> Result<PrecompileOutput, PrecompileErrors> {
    blake2_run(calldata, gas_limit)
}
