use std::collections::HashMap;
use std::str::FromStr;

use crate::{db::StorageSlot, Bytecode, B256, U256};
use bitflags::bitflags;

/// Keccak256 hash of an empty bytecode.
///
/// This constant represents the hash value for an empty bytecode, which is used to check if an account
/// has code or if it's an empty contract.
///
/// # Example:
/// ```no_check
/// assert_eq!(EMPTY_CODE_HASH_STR, "0xc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470");
/// ```
pub const EMPTY_CODE_HASH_STR: &str =
    "0xc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470";

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
#[derive(Clone, Default, PartialEq, Eq, Debug)]
pub struct AccountInfo {
    /// Account balance.
    pub balance: U256,
    /// Account nonce.
    pub nonce: u64,
    /// A Keccak256 hash of the account's contract code.
    pub code_hash: B256,
    /// The account's bytecode, if any. `None` indicates the code should be fetched when needed.
    pub code: Option<Bytecode>,
}

impl AccountInfo {
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
    pub fn is_empty(&self) -> bool {
        self.balance.is_zero()
            && self.nonce == 0
            && self.code_hash == B256::from_str(EMPTY_CODE_HASH_STR).unwrap()
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
    pub fn has_code(&self) -> bool {
        self.code_hash != B256::zero()
            && self.code_hash != B256::from_str(EMPTY_CODE_HASH_STR).unwrap()
    }
}

/// Represents an account in the blockchain, including its core information, storage, and status.
///
/// # Fields:
/// - `info`: The account's core information, including balance, nonce, and code (see `AccountInfo`).
/// - `storage`: A storage cache represented as a `HashMap` that holds the account's storage slots.
/// - `status`: The current status of the account, which may include flags indicating whether it has been modified.
///
/// # Example:
/// ```no_check
/// let account = Account::default();
/// assert!(account.info.is_empty());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Account {
    /// The account's core information (balance, nonce, code).
    pub info: AccountInfo,
    /// A cache of the account's storage, mapping storage keys to storage slots.
    pub storage: HashMap<U256, StorageSlot>,
    /// Flags representing the account's current status (e.g., whether it has been modified).
    pub status: AccountStatus,
}

impl Account {
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
    pub fn is_selfdestructed(&self) -> bool {
        self.status.contains(AccountStatus::SelfDestructed)
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
    pub fn is_created(&self) -> bool {
        self.status.contains(AccountStatus::Created)
    }

    /// Checks if the account has been marked as touched.
    ///
    /// Touched accounts are those that have been accessed or modified during the current transaction,
    /// and are relevant for cleanup or other logic within the EVM.
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
    pub fn is_touched(&self) -> bool {
        self.status.contains(AccountStatus::Touched)
    }
}

bitflags! {
    /// Status flags for an account.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct AccountStatus: u8 {
        const Loaded               = 0b00000000;
        const Created              = 0b00000001;
        const SelfDestructed       = 0b00000010;
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
            storage: HashMap::new(),
            status: AccountStatus::Loaded,
        }
    }
}
