use crate::constants::precompiles::{sha2_256_dynamic_cost, SHA2_256_COST};
use bytes::Bytes;
use revm_precompile::{
    blake2::run as blake2_run, hash::ripemd160_run, identity::identity_run, modexp::berlin_run,
    secp256k1::ec_recover_run, PrecompileErrors,
};
use sha2::{Digest, Sha256};
use std::mem::transmute;

/// Current implementation serves as a baseline:
/// + Five functions reuse the REVM implementation
/// + `sha2_256` utilizes the `sha2` crate with the ASM feature enabled
/// + Future performance optimizations may consider porting high-performance SIMD implementations
type PError = PrecompileErrors;

/// ECRECOVER precompile implementation.
pub fn ecrecover(
    calldata: &Bytes,
    gas_limit: u64,
    consumed_gas: &mut u64,
) -> Result<Bytes, PError> {
    let calldata = unsafe { transmute::<&bytes::Bytes, &revm_precompile::Bytes>(calldata) };
    let output = ec_recover_run(calldata, gas_limit)?;
    *consumed_gas += output.gas_used;
    Ok(output.bytes.into())
}

/// IDENTITY precompile implementation.
pub fn identity(calldata: &Bytes, gas_limit: u64, consumed_gas: &mut u64) -> Result<Bytes, PError> {
    let calldata = unsafe { transmute::<&bytes::Bytes, &revm_precompile::Bytes>(calldata) };
    let output = identity_run(calldata, gas_limit)?;
    *consumed_gas += output.gas_used;
    Ok(output.bytes.into())
}

/// SHA-256 precompile implementation.
pub fn sha2_256(calldata: &Bytes, gas_limit: u64, consumed_gas: &mut u64) -> Result<Bytes, PError> {
    let gas_cost = SHA2_256_COST + sha2_256_dynamic_cost(calldata.len() as u64);
    if gas_limit < gas_cost {
        return Ok(Bytes::new());
    }

    let mut hasher = Sha256::new();
    hasher.update(calldata);
    let hash = hasher.finalize();
    *consumed_gas += gas_cost;
    Ok(Bytes::copy_from_slice(&hash))
}

/// RIPEMD-160 precompile implementation.
pub fn ripemd_160(
    calldata: &Bytes,
    gas_limit: u64,
    consumed_gas: &mut u64,
) -> Result<Bytes, PError> {
    let calldata = unsafe { transmute::<&bytes::Bytes, &revm_precompile::Bytes>(calldata) };
    let output = ripemd160_run(calldata, gas_limit)?;
    *consumed_gas += output.gas_used;
    Ok(output.bytes.into())
}

/// Modular exponentiation precompile (MOD_EXP) implementation.
pub fn modexp(calldata: &Bytes, gas_limit: u64, consumed_gas: &mut u64) -> Result<Bytes, PError> {
    let calldata = unsafe { transmute::<&bytes::Bytes, &revm_precompile::Bytes>(calldata) };
    let output = berlin_run(calldata, gas_limit)?;
    *consumed_gas += output.gas_used;
    Ok(output.bytes.into())
}

/// BLAKE2 F precompile implementation.
pub fn blake2f(calldata: &Bytes, gas_limit: u64, consumed_gas: &mut u64) -> Result<Bytes, PError> {
    let calldata = unsafe { transmute::<&bytes::Bytes, &revm_precompile::Bytes>(calldata) };
    let output = blake2_run(calldata, gas_limit)?;
    *consumed_gas += output.gas_used;
    Ok(output.bytes.into())
}
