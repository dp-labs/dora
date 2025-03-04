use dora_primitives::{Address, Bytes, Bytes32, Env, Log};
use rustc_hash::FxHashMap;
use std::{collections::hash_map::Entry, fmt::Debug};

use crate::call::{CallKind, CallMessage, CallResult};
use crate::result::VMError;

pub use dora_primitives::{AccountLoad, Eip7702CodeLoad, SelfDestructResult, StateLoad};

/// The [`Host`] trait defines the interface for interacting with the Dora runtime environment.
///
/// It includes methods for account management, storage operations, balance inquiries,
/// code retrieval, logging, and access status tracking.
///
/// Implementing this trait allows a host to provide the necessary functionalities during contract execution.
pub trait Host {
    /// Returns a reference to the environment.
    fn env(&self) -> &Env;

    /// Returns a mutable reference to the environment.
    fn env_mut(&mut self) -> &mut Env;

    /// Retrieves the storage value for a given account and storage key.
    fn sload(&mut self, addr: Address, key: Bytes32) -> Option<StateLoad<Bytes32>>;

    /// Sets the storage value for a given account and storage key.
    fn sstore(
        &mut self,
        addr: Address,
        key: Bytes32,
        value: Bytes32,
    ) -> Option<StateLoad<SStoreResult>>;

    /// Get the transient storage value of `address` at `key`.
    fn tload(&mut self, addr: Address, key: Bytes32) -> Bytes32;

    /// Set the transient storage value of `address` at `key`.
    fn tstore(&mut self, addr: Address, key: Bytes32, value: Bytes32);

    /// Load account from database to JournaledState.
    fn load_account_delegated(&mut self, addr: Address) -> Option<AccountLoad>;

    /// Retrieves the balance of a specified account.
    fn balance(&mut self, addr: Address) -> Option<StateLoad<Bytes32>>;

    /// Retrieves the code deployed at a specified account.
    fn code(&mut self, addr: Address) -> Option<StateLoad<Bytes>>;

    /// Retrieves the hash of the code deployed at a specified account.
    fn code_hash(&mut self, addr: Address) -> Option<StateLoad<Bytes32>>;

    /// Mark `address` to be deleted, with funds transferred to `target`.
    fn selfdestruct(
        &mut self,
        addr: Address,
        target: Address,
    ) -> Option<StateLoad<SelfDestructResult>>;

    /// Get the block hash of the given block `number`.
    fn block_hash(&mut self, number: u64) -> Option<Bytes32>;

    /// Emit a log owned by `address` with given `LogData`.
    fn log(&mut self, log: Log);

    /// Host for the call-like insturctions e.g., `CALL`, `CREATE`, etc.
    fn call(&mut self, msg: CallMessage) -> Result<CallResult, VMError>;
}

/// Result of a `set_storage` action.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SStoreResult {
    Slot(SStoreSlot),
    Status(SStoreStatus),
}

impl Default for SStoreResult {
    fn default() -> Self {
        Self::Slot(SStoreSlot::default())
    }
}

/// Result of a `set_storage` action.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct SStoreSlot {
    /// Value of the storage when it is first read
    pub original_value: Bytes32,
    /// Current value of the storage
    pub present_value: Bytes32,
    /// New value that is set
    pub new_value: Bytes32,
}

/// The effect of an attempt to modify a contract storage item.
/// For the purpose of explaining the meaning of each element, the following
/// notation is used:
/// - 0 is zero value,
/// - X != 0 (X is any value other than 0),
/// - Y != 0, Y != X,  (Y is any value other than X and 0),
/// - Z != 0, Z != X, Z != X (Z is any value other than Y and X and 0),
/// - the "o -> c -> v" triple describes the change status in the context of:
///   - o: original value (cold value before a transaction started),
///   - c: current storage value,
///   - v: new storage value to be set.
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum SStoreStatus {
    /// The new/same value is assigned to the storage item without affecting the cost structure.
    Assigned = 0,
    /// A new storage item is added by changing the current clean zero to a nonzero value.
    /// `0 -> 0 -> Z`
    Added = 1,
    /// A storage item is deleted by changing the current clean nonzero to the zero value.
    /// `X -> X -> 0`
    Deleted = 2,
    /// A storage item is modified by changing the current clean nonzero to other nonzero value.
    /// `X -> X -> Z`
    Modified = 3,
    /// A storage item is added by changing the current dirty zero to a nonzero value other than the original value.
    /// `X -> 0 -> Z`
    DeletedAdded = 4,
    /// A storage item is deleted by changing the current dirty nonzero to the zero value and the original value is not zero.
    /// `X -> Y -> 0`
    ModifiedDeleted = 5,
    /// A storage item is added by changing the current dirty zero to the original value.
    /// `X -> 0 -> X`
    DeletedRestored = 6,
    /// A storage item is deleted by changing the current dirty nonzero to the original zero value.
    /// `0 -> Y -> 0`
    AddedDeleted = 7,
    /// A storage item is modified by changing the current dirty nonzero to the original nonzero value other than the current value.
    /// `X -> Y -> X`
    ModifiedRestored = 8,
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
    fn sload(&mut self, _addr: Address, key: Bytes32) -> Option<StateLoad<Bytes32>> {
        Some(match self.storage.entry(key) {
            Entry::Occupied(entry) => StateLoad::new(*entry.get(), false),
            Entry::Vacant(entry) => {
                entry.insert(Bytes32::ZERO);
                StateLoad::new(Bytes32::ZERO, true)
            }
        })
    }

    #[inline]
    fn sstore(
        &mut self,
        _addr: Address,
        key: Bytes32,
        value: Bytes32,
    ) -> Option<StateLoad<SStoreResult>> {
        let present = self.storage.insert(key, value);

        Some(StateLoad::new(
            SStoreResult::Slot(SStoreSlot {
                original_value: Bytes32::ZERO,
                present_value: present.unwrap_or(Bytes32::ZERO),
                new_value: value,
            }),
            present.is_none(),
        ))
    }

    #[inline]
    fn balance(&mut self, _addr: Address) -> Option<StateLoad<Bytes32>> {
        Some(Default::default())
    }

    #[inline]
    fn tload(&mut self, _addr: Address, key: Bytes32) -> Bytes32 {
        self.transient_storage
            .get(&key)
            .copied()
            .unwrap_or_default()
    }

    #[inline]
    fn tstore(&mut self, _addr: Address, key: Bytes32, value: Bytes32) {
        self.transient_storage.insert(key, value);
    }

    #[inline]
    fn code(&mut self, _addr: Address) -> Option<StateLoad<Bytes>> {
        Some(Default::default())
    }

    #[inline]
    fn code_hash(&mut self, _addr: Address) -> Option<StateLoad<Bytes32>> {
        Some(Default::default())
    }

    #[inline]
    fn selfdestruct(
        &mut self,
        _addr: Address,
        _target: Address,
    ) -> Option<StateLoad<SelfDestructResult>> {
        Some(Default::default())
    }

    #[inline]
    fn block_hash(&mut self, _number: u64) -> Option<Bytes32> {
        Some(Default::default())
    }

    #[inline]
    fn log(&mut self, log: Log) {
        self.logs.push(log)
    }

    #[inline]
    fn load_account_delegated(&mut self, _addr: Address) -> Option<AccountLoad> {
        Some(Default::default())
    }

    #[inline]
    fn call(&mut self, msg: CallMessage) -> Result<CallResult, VMError> {
        Ok(match msg.kind {
            CallKind::EofCreate
            | CallKind::ReturnContract
            | CallKind::Call
            | CallKind::Callcode
            | CallKind::Delegatecall
            | CallKind::Staticcall
            | CallKind::Create
            | CallKind::Create2 => CallResult::new_with_gas_limit(msg.gas_limit),
            CallKind::ExtCall | CallKind::ExtStaticcall | CallKind::ExtDelegatecall => {
                unimplemented!("{:?}", msg.kind)
            }
        })
    }
}
