use dora_primitives::{account::EMPTY_CODE_HASH_STR, Address, Bytes32, B256};
use rustc_hash::FxHashMap;
use std::{collections::hash_map::Entry, fmt::Debug, str::FromStr};

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
    fn get_storage(&mut self, addr: &Address, key: &Bytes32) -> GetStorageResult;

    /// Sets the storage value for a given account and storage key.
    fn set_storage(&mut self, addr: Address, key: Bytes32, value: Bytes32) -> SetStorageResult;

    /// Retrieves the balance of a specified account.
    fn get_balance(&mut self, addr: &Address) -> Bytes32;

    /// Get the transient storage value of `address` at `key`.
    fn get_transient_storage(&mut self, addr: &Address, key: &Bytes32) -> Bytes32;

    /// Set the transient storage value of `address` at `key`.
    fn set_transient_storage(&mut self, addr: &Address, key: Bytes32, value: Bytes32);

    /// Retrieves the code deployed at a specified account.
    fn get_code(&mut self, addr: &Address) -> Bytes32;

    /// Retrieves the hash of the code deployed at a specified account.
    fn get_code_hash(&mut self, addr: &Address) -> Bytes32;

    /// Mark `address` to be deleted, with funds transferred to `target`.
    fn selfdestruct(&mut self, addr: &Address, target: Address) -> SelfDestructResult;

    /// Get the block hash of the given block `number`.
    fn get_block_hash(&mut self, number: u64) -> Bytes32;

    /// Emit a log owned by `address` with given `LogData`.
    fn emit_log(&mut self, log: Log);
}

/// Result of a `get_storage` action.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct GetStorageResult {
    pub value: Bytes32,
    pub is_cold: bool,
}

/// Result of a `set_storage` action.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct SetStorageResult {
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
    fn get_storage(&mut self, _addr: &Address, key: &Bytes32) -> GetStorageResult {
        match self.storage.entry(*key) {
            Entry::Occupied(entry) => GetStorageResult {
                value: *entry.get(),
                is_cold: false,
            },
            Entry::Vacant(entry) => {
                entry.insert(Bytes32::ZERO);
                GetStorageResult {
                    value: Bytes32::ZERO,
                    is_cold: true,
                }
            }
        }
    }

    #[inline]
    fn set_storage(&mut self, _addr: Address, key: Bytes32, value: Bytes32) -> SetStorageResult {
        let present = self.storage.insert(key, value);
        SetStorageResult {
            original_value: Bytes32::ZERO,
            present_value: present.unwrap_or(Bytes32::ZERO),
            is_cold: present.is_none(),
        }
    }

    #[inline]
    fn get_balance(&mut self, _addr: &Address) -> Bytes32 {
        Bytes32::ZERO
    }

    #[inline]
    fn get_transient_storage(&mut self, _addr: &Address, key: &Bytes32) -> Bytes32 {
        self.transient_storage.get(key).copied().unwrap_or_default()
    }

    #[inline]
    fn set_transient_storage(&mut self, _addr: &Address, key: Bytes32, value: Bytes32) {
        self.transient_storage.insert(key, value);
    }

    #[inline]
    fn get_code(&mut self, _addr: &Address) -> Bytes32 {
        Bytes32::ZERO
    }

    #[inline]
    fn get_code_hash(&mut self, _addr: &Address) -> Bytes32 {
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
    fn get_block_hash(&mut self, _number: u64) -> Bytes32 {
        Bytes32::ZERO
    }

    #[inline]
    fn emit_log(&mut self, log: Log) {
        self.logs.push(log)
    }
}
