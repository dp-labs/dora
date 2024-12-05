use dora_primitives::{Address, Bytes32, B256};
use rustc_hash::FxHashMap;
use std::{collections::hash_map::Entry, fmt::Debug, str::FromStr};

use crate::account::EMPTY_CODE_HASH_STR;
use crate::{context::Log, env::Env};

/// The `Host` trait defines the interface for interacting with the Dora runtime environment.
/// It includes methods for account management, storage operations, balance inquiries,
/// code retrieval, logging, and access status tracking. Implementing this trait allows
/// a host to provide the necessary functionalities during contract execution.
pub trait Host: Debug {
    /// Returns a reference to the environment.
    fn env(&self) -> &Env;

    /// Returns a mutable reference to the environment.
    fn env_mut(&mut self) -> &mut Env;

    /// Retrieves the storage value for a given account and storage key.
    fn sload(&mut self, addr: &Address, key: &Bytes32) -> SLoadResult;

    /// Sets the storage value for a given account and storage key.
    fn sstore(&mut self, addr: Address, key: Bytes32, value: Bytes32) -> SStoreResult;

    /// Get the transient storage value of `address` at `key`.
    fn tload(&mut self, addr: &Address, key: &Bytes32) -> Bytes32;

    /// Set the transient storage value of `address` at `key`.
    fn tstore(&mut self, addr: &Address, key: Bytes32, value: Bytes32);

    /// Access all storage in the journa that is used to merge into DB.
    fn storage(&self) -> FxHashMap<Bytes32, Bytes32>;

    /// Get access status for the specific account
    fn access_storage(&self, addr: Address, key: Bytes32) -> AccessStatus;

    /// Get access status for the specific account
    fn access_account(&self, addr: Address) -> AccessStatus;

    /// Retrieves the balance of a specified account.
    fn balance(&self, addr: &Address) -> BalanceResult;

    /// Retrieves the code deployed at a specified account.
    fn code(&mut self, addr: &Address) -> Bytes32;

    /// Retrieves the hash of the code deployed at a specified account.
    fn code_hash(&self, addr: &Address) -> Bytes32;

    /// Mark `address` to be deleted, with funds transferred to `target`.
    fn selfdestruct(&mut self, addr: &Address, target: Address) -> SelfDestructResult;

    /// Get the block hash of the given block `number`.
    fn block_hash(&mut self, number: u64) -> Bytes32;

    /// Emit a log owned by `address` with given `LogData`.
    fn log(&mut self, log: Log);
}

/// Result of a `get_storage` action.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct SLoadResult {
    pub value: Bytes32,
    pub is_cold: bool,
}

/// Result of a `get_storage` action.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct BalanceResult {
    pub value: Bytes32,
    pub is_cold: bool,
}

/// Result of a `set_storage` action.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct SStoreResult {
    pub original_value: Bytes32,
    pub present_value: Bytes32,
    pub is_cold: bool,
}

/// Result of a `selfdestruct` action.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct SelfDestructResult {
    pub had_value: bool,
    pub target_exists: bool,
    pub previously_destroyed: bool,
    pub is_cold: bool,
}

/// Access status per EIP-2929: Gas cost increases for state access opcodes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AccessStatus {
    Cold = 0,
    Warm = 1,
}

impl AccessStatus {
    #[inline]
    pub fn is_cold(&self) -> bool {
        matches!(self, AccessStatus::Cold)
    }

    #[inline]
    pub fn is_warm(&self) -> bool {
        matches!(self, AccessStatus::Warm)
    }
}

/// A dummy [Host] implementation for testing.
#[derive(Debug, Clone, Default)]
pub struct DummyHost {
    pub env: Env,
    pub storage: FxHashMap<Bytes32, Bytes32>,
    pub transient_storage: FxHashMap<Bytes32, Bytes32>,
    pub logs: Vec<Log>,
}

impl DummyHost {
    #[inline]
    pub fn new(env: Env) -> Self {
        Self {
            env,
            ..Default::default()
        }
    }
}

impl Host for DummyHost {
    #[inline]
    fn env(&self) -> &Env {
        &self.env
    }

    #[inline]
    fn env_mut(&mut self) -> &mut Env {
        &mut self.env
    }

    #[inline]
    fn sload(&mut self, _addr: &Address, key: &Bytes32) -> SLoadResult {
        match self.storage.entry(*key) {
            Entry::Occupied(entry) => SLoadResult {
                value: *entry.get(),
                is_cold: false,
            },
            Entry::Vacant(entry) => {
                entry.insert(Bytes32::ZERO);
                SLoadResult {
                    value: Bytes32::ZERO,
                    is_cold: true,
                }
            }
        }
    }

    #[inline]
    fn sstore(&mut self, _addr: Address, key: Bytes32, value: Bytes32) -> SStoreResult {
        let present = self.storage.insert(key, value);
        SStoreResult {
            original_value: Bytes32::ZERO,
            present_value: present.unwrap_or(Bytes32::ZERO),
            is_cold: present.is_none(),
        }
    }

    #[inline]
    fn balance(&self, _addr: &Address) -> BalanceResult {
        BalanceResult {
            value: Bytes32::ZERO,
            is_cold: true,
        }
    }

    #[inline]
    fn tload(&mut self, _addr: &Address, key: &Bytes32) -> Bytes32 {
        self.transient_storage.get(key).copied().unwrap_or_default()
    }

    #[inline]
    fn tstore(&mut self, _addr: &Address, key: Bytes32, value: Bytes32) {
        self.transient_storage.insert(key, value);
    }

    #[inline]
    fn code(&mut self, _addr: &Address) -> Bytes32 {
        Bytes32::ZERO
    }

    #[inline]
    fn code_hash(&self, _addr: &Address) -> Bytes32 {
        Bytes32::from_be_bytes(
            *B256::from_str(EMPTY_CODE_HASH_STR)
                .unwrap_or_default()
                .as_fixed_bytes(),
        )
    }

    #[inline]
    fn selfdestruct(&mut self, _addr: &Address, _target: Address) -> SelfDestructResult {
        SelfDestructResult::default()
    }

    #[inline]
    fn block_hash(&mut self, _number: u64) -> Bytes32 {
        Bytes32::ZERO
    }

    #[inline]
    fn log(&mut self, log: Log) {
        self.logs.push(log)
    }

    #[inline]
    fn storage(&self) -> FxHashMap<Bytes32, Bytes32> {
        self.storage.clone()
    }

    #[inline]
    fn access_storage(&self, _addr: Address, _key: Bytes32) -> AccessStatus {
        AccessStatus::Cold
    }

    #[inline]
    fn access_account(&self, _addr: Address) -> AccessStatus {
        AccessStatus::Cold
    }
}
