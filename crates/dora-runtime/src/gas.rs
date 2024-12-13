//! Reference: [revm](https://github.com/bluealloy/revm)
use crate::constants::gas_cost::{
    ACCESS_LIST_ADDRESS, ACCESS_LIST_STORAGE_KEY, CALLVALUE, CALL_STIPEND,
    COLD_ACCOUNT_ACCESS_COST, COLD_SLOAD_COST, INITCODE_WORD_COST, INSTANBUL_SLOAD_GAS, NEWACCOUNT,
    REFUND_SSTORE_CLEARS, SSTORE_RESET, SSTORE_SET, TRANSACTION_ZERO_DATA, WARM_SLOAD_COST,
    WARM_SSTORE_RESET,
};
use crate::env::AccessListItem;
use crate::host::{
    AccountLoad, CodeLoad, SStoreResult, SStoreStatus, SelfDestructResult, StateLoad,
};
use dora_primitives::eip7702::PER_EMPTY_ACCOUNT_COST;
use dora_primitives::spec::SpecId;
use dora_primitives::U256;

#[inline]
pub fn sstore_cost(spec_id: SpecId, result: &SStoreResult, gas: u64, is_cold: bool) -> Option<u64> {
    // EIP-1706: Disable `SSTORE` if `gasleft` is less than call stipend.
    if spec_id.is_enabled_in(SpecId::ISTANBUL) && gas <= CALL_STIPEND {
        return None;
    }
    if spec_id.is_enabled_in(SpecId::BERLIN) {
        // Berlin specification logic
        let base_cost = match result {
            SStoreResult::Slot(slot) => {
                calculate_sstore_slot_cost::<WARM_SLOAD_COST, WARM_SSTORE_RESET>(
                    slot.original_value.to_u256(),
                    slot.present_value.to_u256(),
                    slot.new_value.to_u256(),
                )
            }
            SStoreResult::Status(status) => {
                calculate_sstore_status_cost::<WARM_SLOAD_COST, WARM_SSTORE_RESET>(status)
            }
        };
        let additional_cost = if is_cold { COLD_SLOAD_COST } else { 0 };
        Some(base_cost + additional_cost)
    } else if spec_id.is_enabled_in(SpecId::ISTANBUL) {
        // Istanbul specification logic
        Some(match result {
            SStoreResult::Slot(slot) => {
                calculate_sstore_slot_cost::<INSTANBUL_SLOAD_GAS, SSTORE_RESET>(
                    slot.original_value.to_u256(),
                    slot.present_value.to_u256(),
                    slot.new_value.to_u256(),
                )
            }
            SStoreResult::Status(status) => {
                calculate_sstore_status_cost::<INSTANBUL_SLOAD_GAS, SSTORE_RESET>(status)
            }
        })
    } else {
        // Frontier specification logic
        Some(match result {
            SStoreResult::Slot(slot) => {
                frontier_sstore_slot_cost(slot.present_value.to_u256(), slot.new_value.to_u256())
            }
            SStoreResult::Status(status) => frontier_sstore_status_cost(status),
        })
    }
}

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
pub fn sstore_slot_cost(
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
    if spec_id.is_enabled_in(SpecId::BERLIN) {
        // Berlin specification logic
        let base_cost = calculate_sstore_slot_cost::<WARM_SLOAD_COST, WARM_SSTORE_RESET>(
            original, current, new,
        );
        let additional_cost = if is_cold { COLD_SLOAD_COST } else { 0 };
        Some(base_cost + additional_cost)
    } else if spec_id.is_enabled_in(SpecId::ISTANBUL) {
        // Istanbul specification logic
        Some(calculate_sstore_slot_cost::<
            INSTANBUL_SLOAD_GAS,
            SSTORE_RESET,
        >(original, current, new))
    } else {
        // Frontier specification logic
        Some(frontier_sstore_slot_cost(current, new))
    }
}

#[inline]
fn calculate_sstore_slot_cost<const SLOAD_GAS: u64, const SSTORE_RESET_GAS: u64>(
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
fn calculate_sstore_status_cost<const SLOAD_GAS: u64, const SSTORE_RESET_GAS: u64>(
    status: &SStoreStatus,
) -> u64 {
    match status {
        SStoreStatus::Added => SSTORE_SET, // New value is set from zero.
        SStoreStatus::Modified | SStoreStatus::Deleted => SSTORE_RESET_GAS, // Value is reset to a non-zero value.
        _ => SLOAD_GAS,                                                     // Default case.
    }
}

#[inline]
fn frontier_sstore_slot_cost(current: U256, new: U256) -> u64 {
    if current.is_zero() && !new.is_zero() {
        SSTORE_SET
    } else {
        SSTORE_RESET
    }
}

#[inline]
fn frontier_sstore_status_cost(status: &SStoreStatus) -> u64 {
    if matches!(status, SStoreStatus::Added) {
        SSTORE_SET
    } else {
        SSTORE_RESET
    }
}

/// Calculates the refund amount for the `SSTORE` opcode based on the EVM specification.
#[inline]
pub fn sstore_refund(spec_id: SpecId, result: &SStoreResult) -> i64 {
    match result {
        SStoreResult::Slot(slot) => sstore_slot_refund(
            spec_id,
            slot.original_value.to_u256(),
            slot.present_value.to_u256(),
            slot.new_value.to_u256(),
        ),
        SStoreResult::Status(status) => sstore_status_refund(spec_id, status),
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
pub fn sstore_slot_refund(spec_id: SpecId, original: U256, current: U256, new: U256) -> i64 {
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
    if original == current && new.is_zero() {
        return sstore_clears_schedule;
    }
    let mut refund: i64 = 0;
    if !original.is_zero() {
        if current.is_zero() {
            refund -= sstore_clears_schedule;
        } else if new.is_zero() {
            refund += sstore_clears_schedule;
        }
    }

    if original == new {
        let (sstore_reset_cost, sload_cost) = if spec_id.is_enabled_in(SpecId::BERLIN) {
            (SSTORE_RESET - COLD_SLOAD_COST, WARM_SLOAD_COST)
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

/// Calculates the refund amount for the `SSTORE` opcode based on the EVM specification.
///
/// # Parameters
/// - `spec_id`: The current EVM specification identifier.
/// - `status`: The status of the `SSTORE` opcode.
///
/// # Returns
/// The refund amount as `i64`.
pub fn sstore_status_refund(spec_id: SpecId, status: &SStoreStatus) -> i64 {
    if !spec_id.is_enabled_in(SpecId::ISTANBUL) {
        return if matches!(
            status,
            SStoreStatus::Deleted | SStoreStatus::AddedDeleted | SStoreStatus::ModifiedDeleted
        ) {
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
    match status {
        SStoreStatus::Assigned | SStoreStatus::Added | SStoreStatus::Modified => 0,
        SStoreStatus::Deleted | SStoreStatus::ModifiedDeleted => sstore_clears_schedule,
        SStoreStatus::DeletedAdded => -sstore_clears_schedule,
        SStoreStatus::DeletedRestored
        | SStoreStatus::ModifiedRestored
        | SStoreStatus::AddedDeleted => {
            let (sstore_reset_cost, sload_cost) = if spec_id.is_enabled_in(SpecId::BERLIN) {
                (SSTORE_RESET - COLD_SLOAD_COST, WARM_SLOAD_COST)
            } else {
                (SSTORE_RESET, sload_cost(spec_id, false))
            };
            if matches!(status, SStoreStatus::AddedDeleted) {
                (SSTORE_SET - sload_cost) as i64
            } else {
                (sstore_reset_cost - sload_cost) as i64
            }
        }
    }
}

/// Returns number of words what would fit to provided number of bytes,
/// i.e. it rounds up the number bytes to number of words.
#[inline]
pub const fn num_words(len: u64) -> u64 {
    len.saturating_add(31) / 32
}

/// Calculate the cost of buffer per word.
#[inline]
pub const fn cost_per_word(len: u64, multiple: u64) -> Option<u64> {
    multiple.checked_mul(num_words(len))
}

/// EIP-3860: Limit and meter initcode
///
/// Apply extra gas cost of 2 for every 32-byte chunk of initcode.
///
/// This cannot overflow as the initcode length is assumed to be checked.
#[inline]
pub const fn initcode_cost(len: u64) -> Option<u64> {
    cost_per_word(len, INITCODE_WORD_COST)
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
            WARM_SLOAD_COST
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
pub const fn extcodesize_gas_cost(spec_id: SpecId, load: CodeLoad<()>) -> u64 {
    if spec_id.is_enabled_in(SpecId::BERLIN) {
        warm_cold_cost_with_delegation(load)
    } else if spec_id.is_enabled_in(SpecId::TANGERINE) {
        700
    } else {
        20
    }
}

#[inline]
pub const fn warm_cold_cost(is_cold: bool) -> u64 {
    if is_cold {
        COLD_ACCOUNT_ACCESS_COST
    } else {
        WARM_SLOAD_COST
    }
}

pub const fn warm_cold_cost_with_delegation(load: CodeLoad<()>) -> u64 {
    let mut gas = warm_cold_cost(load.state_load.is_cold);
    if let Some(is_cold) = load.is_delegate_account_cold {
        gas += warm_cold_cost(is_cold);
    }
    gas
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
pub const fn extcodecopy_gas_cost(spec_id: SpecId, load: CodeLoad<()>) -> u64 {
    if spec_id.is_enabled_in(SpecId::BERLIN) {
        warm_cold_cost_with_delegation(load)
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
        warm_cold_cost(is_cold)
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
pub const fn extcodehash_gas_cost(spec_id: SpecId, load: CodeLoad<()>) -> u64 {
    if spec_id.is_enabled_in(SpecId::BERLIN) {
        warm_cold_cost_with_delegation(load)
    } else if spec_id.is_enabled_in(SpecId::ISTANBUL) {
        700
    } else {
        400
    }
}

/// `SELFDESTRUCT` opcode cost calculation.
///
/// Calculates the gas cost for the `SELFDESTRUCT` opcode based on the specification version
/// and the self destruct result returned by the host.
///
/// ### Parameters:
/// - `spec_id`: The current EVM specification version.
/// - `res`: The self destruct result returned by the host.
///
/// ### Returns:
/// The gas cost for the `SELFDESTRUCT` operation.
///
#[inline]
pub const fn selfdestruct_cost(spec_id: SpecId, res: &StateLoad<SelfDestructResult>) -> u64 {
    // EIP-161: State trie clearing (invariant-preserving alternative)
    let should_charge_topup = if spec_id.is_enabled_in(SpecId::SPURIOUS_DRAGON) {
        res.data.had_value && !res.data.target_exists
    } else {
        !res.data.target_exists
    };

    // EIP-150: Gas cost changes for IO-heavy operations
    let selfdestruct_gas_topup = if spec_id.is_enabled_in(SpecId::TANGERINE) && should_charge_topup
    {
        25000
    } else {
        0
    };

    // EIP-150: Gas cost changes for IO-heavy operations
    let selfdestruct_gas = if spec_id.is_enabled_in(SpecId::TANGERINE) {
        5000
    } else {
        0
    };

    let mut gas = selfdestruct_gas + selfdestruct_gas_topup;
    if spec_id.is_enabled_in(SpecId::BERLIN) && res.is_cold {
        gas += COLD_ACCOUNT_ACCESS_COST
    }
    gas
}

/// Calculate call gas cost for the call instruction.
#[inline]
pub const fn call_cost(spec_id: SpecId, transfers_value: bool, account_load: AccountLoad) -> u64 {
    // Account access.
    let mut gas = if spec_id.is_enabled_in(SpecId::BERLIN) {
        warm_cold_cost_with_delegation(account_load.code_load)
    } else if spec_id.is_enabled_in(SpecId::TANGERINE) {
        // EIP-150: Gas cost changes for IO-heavy operations
        700
    } else {
        40
    };
    // transfer value cost
    if transfers_value {
        gas += CALLVALUE;
    }
    if account_load.is_empty {
        if spec_id.is_enabled_in(SpecId::SPURIOUS_DRAGON) {
            if transfers_value {
                gas += NEWACCOUNT;
            }
        } else {
            gas += NEWACCOUNT;
        }
    }

    gas
}

/// Initial gas that is deducted for transaction to be included.
/// Initial gas contains initial stipend gas, gas for access list and input data.
pub fn validate_initial_tx_gas(
    spec_id: SpecId,
    input: &[u8],
    is_create: bool,
    access_list: &[AccessListItem],
    authorization_list_num: u64,
) -> u64 {
    let mut initial_gas = 0;
    let zero_data_len = input.iter().filter(|v| **v == 0).count() as u64;
    let non_zero_data_len = input.len() as u64 - zero_data_len;

    // initdate stipend
    initial_gas += zero_data_len * TRANSACTION_ZERO_DATA;
    // EIP-2028: Transaction data gas cost reduction
    initial_gas += non_zero_data_len
        * if spec_id.is_enabled_in(SpecId::ISTANBUL) {
            16
        } else {
            68
        };

    // get number of access list account and storages.
    let (account_num, storage_num): (usize, usize) = (
        access_list.len(),
        access_list.iter().map(|i| i.1.len()).sum(),
    );
    initial_gas += account_num as u64 * ACCESS_LIST_ADDRESS;
    initial_gas += storage_num as u64 * ACCESS_LIST_STORAGE_KEY;

    // base stipend
    initial_gas += if is_create {
        if spec_id.is_enabled_in(SpecId::HOMESTEAD) {
            // EIP-2: Homestead Hard-fork Changes
            53000
        } else {
            21000
        }
    } else {
        21000
    };

    // EIP-3860: Limit and meter initcode
    // Init code stipend for bytecode analysis
    if spec_id.is_enabled_in(SpecId::SHANGHAI) && is_create {
        initial_gas += initcode_cost(input.len() as u64).unwrap_or_default();
    }

    // EIP-7702
    if spec_id.is_enabled_in(SpecId::PRAGUE) {
        initial_gas += authorization_list_num * PER_EMPTY_ACCOUNT_COST;
    }

    initial_gas
}
