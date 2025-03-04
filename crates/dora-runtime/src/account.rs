//! Reference: [revm](https://github.com/bluealloy/revm)

use crate::db::{DbAccount, StorageSlot};
use bitflags::bitflags;
use dora_primitives::{B256, Bytecode, KECCAK_EMPTY, SpecId, U256};
use rustc_hash::FxHashMap;

/// Represents the core information of an account, including its balance, nonce, code hash, and optional bytecode.
///
/// # Fields:
/// - `balance`: The balance of the account, represented as a `U256`.
/// - `nonce`: The nonce of the account, represented as a `u64`.
/// - `code_hash`: A 256-bit hash of the account's contract code.
/// - `code`: Optional bytecode. If `None`, the bytecode will be fetched by the code hash when required.
///
/// # Methods:
/// - `is_empty`: Checks if the account is empty. An account is considered empty if its balance is zero, its nonce is zero,
///   and its `code_hash` matches the hash of an empty bytecode.
/// - `has_code`: Checks if the account contains code. Returns `true` if the `code_hash` is not zero and does not match
///   the empty bytecode hash.
///
/// # Example:
/// ```no_check
/// let account_info = AccountInfo::default();
/// assert!(account_info.is_empty());
/// assert!(!account_info.has_code());
/// ```
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AccountInfo {
    /// Account balance.
    pub balance: U256,
    /// Account nonce.
    pub nonce: u64,
    /// A Keccak256 hash of the account's contract code.
    pub code_hash: B256,
    /// The account's EVM or WASM bytecode, if any. `None` indicates the code should be fetched when needed.
    pub code: Option<Bytecode>,
}

impl Default for AccountInfo {
    fn default() -> Self {
        Self {
            balance: U256::ZERO,
            code_hash: KECCAK_EMPTY,
            code: Some(Bytecode::default()),
            nonce: 0,
        }
    }
}

impl AccountInfo {
    /// Construct an empty account info.
    #[inline]
    pub fn empty() -> AccountInfo {
        DbAccount::empty().into()
    }

    /// Determines if the account is empty, based on its balance, nonce, and code hash.
    ///
    /// # Returns:
    /// - `true` if the account is empty (balance is zero, nonce is zero, and the code hash matches the empty bytecode hash).
    /// - `false` otherwise.
    ///
    /// # Example:
    /// ```no_check
    /// let account_info = AccountInfo::default();
    /// assert!(account_info.is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.balance.is_zero() && self.nonce == 0 && self.code_hash == KECCAK_EMPTY
    }

    /// Checks if the account has contract code associated with it.
    ///
    /// # Returns:
    /// - `true` if the `code_hash` is non-zero and does not match the empty bytecode hash.
    /// - `false` if the account has no code.
    ///
    /// # Example:
    /// ```no_check
    /// let account_info = AccountInfo::default();
    /// assert!(!account_info.has_code());
    /// ```
    #[inline]
    pub fn has_code(&self) -> bool {
        self.code_hash != B256::ZERO && self.code_hash != KECCAK_EMPTY
    }
}

/// Represents an account in the blockchain, including its core information, storage, and status.
///
/// # Fields:
/// - `info`: The account's core information, including balance, nonce, and code (see `AccountInfo`).
/// - `storage`: A storage cache represented as a `HashMap` that holds the account's storage slots.
/// - `status`: The current status of the account, which may include flags indicating whether it has been modified.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Account {
    /// The account's core information (balance, nonce, code).
    pub info: AccountInfo,
    /// A cache of the account's storage, mapping storage keys to storage slots.
    pub storage: FxHashMap<U256, StorageSlot>,
    /// Flags representing the account's current status (e.g., whether it has been modified).
    pub status: AccountStatus,
}

impl Account {
    /// Create a new account and mark it as non existing.
    #[inline]
    pub fn new_not_existing() -> Self {
        Self {
            status: AccountStatus::LoadedAsNotExisting,
            ..Default::default()
        }
    }

    /// New empty account with the storage.
    #[inline]
    pub fn new_empty_with_storage(storage: FxHashMap<U256, StorageSlot>) -> Self {
        Self {
            storage,
            ..Default::default()
        }
    }

    /// Check if account is empty and check if empty state before spurious dragon hardfork.
    pub fn state_clear_aware_is_empty(&self, spec: SpecId) -> bool {
        if SpecId::enabled(spec, SpecId::SPURIOUS_DRAGON) {
            self.is_empty()
        } else {
            let loaded_not_existing = self.is_loaded_as_not_existing();
            let is_not_touched = !self.is_touched();
            loaded_not_existing && is_not_touched
        }
    }

    /// Checks if the account is marked for self-destruction.
    ///
    /// # Returns:
    /// - `true` if the account has been flagged for self-destruction, meaning it will be removed at the end of the transaction.
    /// - `false` otherwise.
    ///
    /// # Example:
    /// ```no_check
    /// let account = Account::default();
    /// assert!(!account.is_selfdestructed());
    /// ```
    #[inline]
    pub fn is_selfdestructed(&self) -> bool {
        self.status.contains(AccountStatus::Selfdestructed)
    }

    /// Checks if the account was newly created.
    ///
    /// # Returns:
    /// - `true` if the account has been marked as newly created during the current transaction.
    /// - `false` otherwise.
    ///
    /// # Example:
    /// ```no_check
    /// let account = Account::default();
    /// assert!(!account.is_created());
    /// ```
    #[inline]
    pub fn is_created(&self) -> bool {
        self.status.contains(AccountStatus::Created)
    }

    /// Mark account as touched.
    #[inline]
    pub fn mark_touch(&mut self) {
        self.status |= AccountStatus::Touched;
    }

    /// Unmark the touch flag.
    #[inline]
    pub fn unmark_touch(&mut self) {
        self.status -= AccountStatus::Touched;
    }

    /// Checks if the account has been marked as touched.
    ///
    /// Touched accounts are those that have been accessed or modified during the current transaction,
    /// and are relevant for cleanup or other logic within the VM.
    ///
    /// # Returns:
    /// - `true` if the account is marked as touched.
    /// - `false` otherwise.
    ///
    /// # Example:
    /// ```no_check
    /// let account = Account::default();
    /// assert!(!account.is_touched());
    /// ```
    #[inline]
    pub fn is_touched(&self) -> bool {
        self.status.contains(AccountStatus::Touched)
    }

    /// Mark account as self destructed.
    #[inline]
    pub fn mark_selfdestruct(&mut self) {
        self.status |= AccountStatus::Selfdestructed;
    }

    /// Unmark account as self destructed.
    #[inline]
    pub fn unmark_selfdestruct(&mut self) {
        self.status -= AccountStatus::Selfdestructed;
    }

    /// Mark account as newly created.
    #[inline]
    pub fn mark_created(&mut self) {
        self.status |= AccountStatus::Created;
    }

    /// Unmark created flag.
    #[inline]
    pub fn unmark_created(&mut self) {
        self.status -= AccountStatus::Created;
    }

    /// Mark account as cold.
    #[inline]
    pub fn mark_cold(&mut self) {
        self.status |= AccountStatus::Cold;
    }

    /// Mark account as warm and return true if it was previously cold.
    #[inline]
    pub fn mark_warm(&mut self) -> bool {
        if self.status.contains(AccountStatus::Cold) {
            self.status -= AccountStatus::Cold;
            true
        } else {
            false
        }
    }

    /// Is account loaded as not existing from database
    /// This is needed for pre spurious dragon hardforks where
    /// existing and empty were two separate states.
    #[inline]
    pub fn is_loaded_as_not_existing(&self) -> bool {
        self.status.contains(AccountStatus::LoadedAsNotExisting)
    }

    /// Is account empty, check if nonce and balance are zero and code is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.info.is_empty()
    }

    /// Returns an iterator over the storage slots that have been changed.
    #[inline]
    pub fn changed_storage_slots(&self) -> impl Iterator<Item = (&U256, &StorageSlot)> {
        self.storage.iter().filter(|(_, slot)| slot.is_changed())
    }
}

bitflags! {
    /// Status flags for an account.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct AccountStatus: u8 {
        const Loaded               = 0b00000000;
        const Created              = 0b00000001;
        const Selfdestructed       = 0b00000010;
        const Touched              = 0b00000100;
        const LoadedAsNotExisting  = 0b00001000;  // Pre-EIP-161 state.
        const Cold                 = 0b00100000;
    }
}

impl Default for AccountStatus {
    fn default() -> Self {
        Self::Loaded
    }
}

impl From<AccountInfo> for Account {
    fn from(info: AccountInfo) -> Self {
        Self {
            info,
            storage: FxHashMap::default(),
            status: AccountStatus::Loaded,
        }
    }
}
