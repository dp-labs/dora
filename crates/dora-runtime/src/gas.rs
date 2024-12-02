use crate::constants::gas_cost::{
    ACCESS_LIST_STORAGE_KEY, CALL_STIPEND, COLD_ACCOUNT_ACCESS_COST, COLD_SLOAD_COST,
    INSTANBUL_SLOAD_GAS, REFUND_SSTORE_CLEARS, SSTORE_RESET, SSTORE_SET, WARM_SSTORE_RESET,
    WARM_STORAGE_READ_COST,
};
use dora_primitives::spec::SpecId;
use dora_primitives::U256;

/// Calculates the gas cost of the `SSTORE` opcode based on the EVM specification and storage conditions.
///
/// # Parameters
/// - `spec_id`: The current EVM specification identifier.
/// - `original`: The original value of the storage slot.
/// - `current`: The current value of the storage slot.
/// - `new`: The new value being written to the storage slot.
/// - `gas`: The remaining gas available for execution.
/// - `is_cold`: Indicates whether the storage access is cold.
///
/// # Returns
/// - `Some(u64)`: The gas cost for the operation if it is valid.
/// - `None`: If the operation is invalid, such as when `gasleft` is less than the call stipend.
#[inline]
pub fn sstore_cost(
    spec_id: SpecId,
    original: U256,
    current: U256,
    new: U256,
    gas: u64,
    is_cold: bool,
) -> Option<u64> {
    // EIP-1706: Disable `SSTORE` if `gasleft` is less than call stipend.
    if spec_id.is_enabled_in(SpecId::ISTANBUL) && gas <= CALL_STIPEND {
        return None;
    }

    match spec_id {
        _ if spec_id.is_enabled_in(SpecId::BERLIN) => {
            // Berlin specification logic
            let base_cost = calculate_sstore_cost::<WARM_STORAGE_READ_COST, WARM_SSTORE_RESET>(
                original, current, new,
            );
            let additional_cost = if is_cold { COLD_SLOAD_COST } else { 0 };
            Some(base_cost + additional_cost)
        }
        _ if spec_id.is_enabled_in(SpecId::ISTANBUL) => {
            // Istanbul specification logic
            Some(calculate_sstore_cost::<INSTANBUL_SLOAD_GAS, SSTORE_RESET>(
                original, current, new,
            ))
        }
        _ => {
            // Frontier specification logic
            Some(frontier_sstore_cost(current, new))
        }
    }
}

#[inline]
fn calculate_sstore_cost<const SLOAD_GAS: u64, const SSTORE_RESET_GAS: u64>(
    original: U256,
    current: U256,
    new: U256,
) -> u64 {
    match (original == current, original.is_zero(), current == new) {
        (_, _, true) => SLOAD_GAS,                // No change in value.
        (true, true, false) => SSTORE_SET,        // New value is set from zero.
        (true, false, false) => SSTORE_RESET_GAS, // Value is reset to a non-zero value.
        _ => SLOAD_GAS,                           // Default case.
    }
}

#[inline]
fn frontier_sstore_cost(current: U256, new: U256) -> u64 {
    if current.is_zero() && !new.is_zero() {
        SSTORE_SET
    } else {
        SSTORE_RESET
    }
}

/// Calculates the refund amount for the `SSTORE` opcode based on the EVM specification.
///
/// # Parameters
/// - `spec_id`: The current EVM specification identifier.
/// - `original`: The original value of the storage slot.
/// - `current`: The current value of the storage slot.
/// - `new`: The new value being written.
///
/// # Returns
/// The refund amount as `i64`.
#[inline]
pub fn sstore_refund(spec_id: SpecId, original: U256, current: U256, new: U256) -> i64 {
    if !spec_id.is_enabled_in(SpecId::ISTANBUL) {
        return if !current.is_zero() && new.is_zero() {
            REFUND_SSTORE_CLEARS
        } else {
            0
        };
    }

    let sstore_clears_schedule = if spec_id.is_enabled_in(SpecId::LONDON) {
        (SSTORE_RESET - COLD_SLOAD_COST + ACCESS_LIST_STORAGE_KEY) as i64
    } else {
        REFUND_SSTORE_CLEARS
    };

    if current == new {
        return 0;
    }

    let mut refund = 0;

    if original == current && new.is_zero() {
        return sstore_clears_schedule;
    }

    if !original.is_zero() {
        if current.is_zero() {
            refund -= sstore_clears_schedule;
        } else if new.is_zero() {
            refund += sstore_clears_schedule;
        }
    }

    if original == new {
        let (sstore_reset_cost, sload_cost) = if spec_id.is_enabled_in(SpecId::BERLIN) {
            (SSTORE_RESET - COLD_SLOAD_COST, WARM_STORAGE_READ_COST)
        } else {
            (SSTORE_RESET, sload_cost(spec_id, false))
        };

        refund += if original.is_zero() {
            (SSTORE_SET - sload_cost) as i64
        } else {
            (sstore_reset_cost - sload_cost) as i64
        };
    }

    refund
}

/// Calculates the `SLOAD` cost based on the EVM specification.
///
/// # Parameters
/// - `spec_id`: The current EVM specification identifier.
/// - `is_cold`: Indicates whether the storage access is cold.
///
/// # Returns
/// The gas cost for the operation.
#[inline]
pub const fn sload_cost(spec_id: SpecId, is_cold: bool) -> u64 {
    if spec_id.is_enabled_in(SpecId::BERLIN) {
        if is_cold {
            COLD_SLOAD_COST
        } else {
            WARM_STORAGE_READ_COST
        }
    } else if spec_id.is_enabled_in(SpecId::ISTANBUL) {
        INSTANBUL_SLOAD_GAS
    } else if spec_id.is_enabled_in(SpecId::TANGERINE) {
        200
    } else {
        50
    }
}

/// Calculates the gas cost for the `EXTCODESIZE` opcode based on the EVM specification.
///
/// The gas cost depends on the specification version and whether the account access is considered
/// "cold" (not accessed recently) or "warm" (already accessed in the current execution context).
///
/// ### Parameters:
/// - `spec_id`: The current specification version.
/// - `is_cold`: Boolean indicating whether the account access is cold.
///
/// ### Returns:
/// The gas cost for the `EXTCODESIZE` operation.
///
#[inline]
pub const fn extcodesize_gas_cost(spec_id: SpecId, is_cold: bool) -> u64 {
    if spec_id.is_enabled_in(SpecId::BERLIN) {
        if is_cold {
            COLD_ACCOUNT_ACCESS_COST
        } else {
            WARM_STORAGE_READ_COST
        }
    } else if spec_id.is_enabled_in(SpecId::TANGERINE) {
        700
    } else {
        20
    }
}

/// Computes the gas cost for the `EXTCODECOPY` operation based on the Ethereum specification.
///
/// # Parameters
/// - `spec_id`: The `SpecId` representing the Ethereum specification version.
/// - `is_cold`: A boolean indicating whether the operation is accessing a cold account (`true`)
///   or a warm account (`false`).
///
/// # Returns
/// - A `u64` value representing the gas cost for the `EXTCODECOPY` operation
///
/// # Notes
/// - This function leverages inline execution for performance optimization.
/// - Gas costs vary depending on the Ethereum specification version to maintain compatibility
///   with historical and updated protocol rules.
///
#[inline]
pub const fn extcodecopy_gas_cost(spec_id: SpecId, is_cold: bool) -> u64 {
    if spec_id.is_enabled_in(SpecId::BERLIN) {
        if is_cold {
            COLD_ACCOUNT_ACCESS_COST
        } else {
            WARM_STORAGE_READ_COST
        }
    } else if spec_id.is_enabled_in(SpecId::TANGERINE) {
        700
    } else {
        20
    }
}

/// Calculates the balance store gas cost for an EVM operation based on the specification version and access type.
///
/// The gas cost varies depending on the EVM specification and whether the operation involves
/// a "cold" or "warm" account access.
///
/// ### Parameters:
/// - `spec_id`: The current EVM specification version.
/// - `is_cold`: Boolean indicating whether the access is cold.
///
/// ### Returns:
/// The gas cost for the operation.
///
#[inline]
pub const fn balance_gas_cost(spec_id: SpecId, is_cold: bool) -> u64 {
    if spec_id.is_enabled_in(SpecId::BERLIN) {
        if is_cold {
            COLD_ACCOUNT_ACCESS_COST
        } else {
            WARM_STORAGE_READ_COST
        }
    } else if spec_id.is_enabled_in(SpecId::ISTANBUL) {
        700
    } else if spec_id.is_enabled_in(SpecId::TANGERINE) {
        400
    } else {
        20
    }
}

/// Calculates the gas cost for the `EXTCODEHASH` opcode based on the specification version
/// and access type.
///
/// The gas cost depends on the EVM specification version and whether the access is "cold" or "warm."
///
/// ### Parameters:
/// - `spec_id`: The current EVM specification version.
/// - `is_cold`: Boolean indicating whether the access is cold.
///
/// ### Returns:
/// The gas cost for the `EXTCODEHASH` operation.
///
#[inline]
pub const fn extcodehash_gas_cost(spec_id: SpecId, is_cold: bool) -> u64 {
    if spec_id.is_enabled_in(SpecId::BERLIN) {
        if is_cold {
            COLD_ACCOUNT_ACCESS_COST
        } else {
            WARM_STORAGE_READ_COST
        }
    } else if spec_id.is_enabled_in(SpecId::ISTANBUL) {
        700
    } else {
        400
    }
}
