use std::fmt;
use std::mem::transmute;
use crate::constants::precompiles::{sha2_256_dynamic_cost, SHA2_256_COST};
use bytes::Bytes as rowBytes;
use revm_precompile::{
    blake2::run as blake2_run, hash::ripemd160_run, identity::identity_run, modexp::berlin_run,
    secp256k1::ec_recover_run, PrecompileErrors,
};
use sha2::{Digest, Sha256};

/// Current implementation serves as a baseline:
/// + Five functions reuse the REVM implementation
/// + `sha2_256` utilizes the `sha2` crate with the ASM feature enabled
/// + Future performance optimizations may consider porting high-performance SIMD implementations
pub struct PError(PrecompileErrors);

impl fmt::Debug for PError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PError({:?})", self.0)
    }
}

/// ECRECOVER precompile implementation.
pub fn ecrecover(
    calldata: &rowBytes,
    gas_limit: u64,
    consumed_gas: &mut u64,
) -> Result<rowBytes, PError> {
    let calldata = unsafe { transmute(calldata) };
    let output = ec_recover_run(calldata, gas_limit).unwrap();
    *consumed_gas += output.gas_used;
    Ok(output.bytes.into())
}

/// IDENTITY precompile implementation.
pub fn identity(calldata: &rowBytes, gas_limit: u64, consumed_gas: &mut u64) -> rowBytes {
    let calldata = unsafe { transmute(calldata) };
    let output = identity_run(calldata, gas_limit).unwrap();
    *consumed_gas += output.gas_used;
    output.bytes.into()
}

/// SHA-256 precompile implementation.
pub fn sha2_256(calldata: &rowBytes, gas_limit: u64, consumed_gas: &mut u64) -> rowBytes {
    let gas_cost = SHA2_256_COST + sha2_256_dynamic_cost(calldata.len() as u64);
    if gas_limit < gas_cost {
        return rowBytes::new();
    }

    *consumed_gas += gas_cost;
    let mut hasher = Sha256::new();
    hasher.update(calldata);
    let hash = hasher.finalize();
    rowBytes::copy_from_slice(&hash)
}

/// RIPEMD-160 precompile implementation.
pub fn ripemd_160(calldata: &rowBytes, gas_limit: u64, consumed_gas: &mut u64) -> rowBytes {
    let calldata = unsafe { transmute(calldata) };
    let output = ripemd160_run(calldata, gas_limit).unwrap();
    *consumed_gas += output.gas_used;
    output.bytes.into()
}

/// Modular exponentiation precompile (MOD_EXP) implementation.
pub fn modexp(calldata: &rowBytes, gas_limit: u64, consumed_gas: &mut u64) -> rowBytes {
    let calldata = unsafe { transmute(calldata) };
    let output = berlin_run(calldata, gas_limit).unwrap();
    *consumed_gas += output.gas_used;
    output.bytes.into()
}

/// BLAKE2 F precompile implementation.
pub fn blake2f(
    calldata: &rowBytes,
    gas_limit: u64,
    consumed_gas: &mut u64,
) -> Result<rowBytes, PError> {
    let calldata = unsafe { transmute(calldata) };
    let output = blake2_run(calldata, gas_limit).map_err(PError)?;
    *consumed_gas += output.gas_used;
    Ok(output.bytes.into())
}
