use crate::artifact::{Artifact, SymbolArtifact};
pub use dora_primitives::StorageSlot;
use dora_primitives::{
    Account, AccountInfo, AccountStatus, Address, B256, Bytecode, DBErrorMarker, KECCAK_EMPTY,
    U256, keccak256,
};
use rustc_hash::FxHashMap;
use std::{convert::Infallible, fmt::Debug};
use thiserror::Error;

/// The `Database` trait provides an interface for interacting with various on-chain data sources
/// related to accounts, storage, and blocks. It defines methods to retrieve basic account information,
/// contract code, storage values, and block hashes.
pub trait Database: Clone + Debug + DatabaseCommit {
    /// The type representing an error that may occur when interacting with the database.
    type Error: Debug + Sync + Send;
    type Artifact: Artifact;

    /// Retrieves basic account information for a given address.
    ///
    /// # Parameters:
    /// - `address`: The account address whose information is being queried.
    ///
    /// # Returns:
    /// - `Result<Option<AccountInfo>, Self::Error>`: A `Result` containing either an `Option` with the `AccountInfo`
    ///   or an error if the query fails. The `Option` will be `None` if the account does not exist.
    fn basic(&self, address: Address) -> Result<Option<AccountInfo>, Self::Error>;

    /// Retrieves the contract bytecode for a given code hash.
    ///
    /// # Parameters:
    /// - `code_hash`: The hash of the contract code to be retrieved.
    ///
    /// # Returns:
    /// - `Result<Bytecode, Self::Error>`: A `Result` containing either the `Bytecode` associated with the hash
    ///   or an error if the query fails.
    fn code_by_hash(&self, code_hash: B256) -> Result<Bytecode, Self::Error>;

    /// Retrieves a storage value for a given account address and storage index.
    ///
    /// # Parameters:
    /// - `address`: The address of the account whose storage is being queried.
    /// - `index`: The storage index to retrieve.
    ///
    /// # Returns:
    /// - `Result<U256, Self::Error>`: A `Result` containing either the `U256` value stored at the given index
    ///   or an error if the query fails.
    fn storage(&self, address: Address, index: U256) -> Result<U256, Self::Error>;

    /// Retrieves the block hash for a given block number.
    ///
    /// # Parameters:
    /// - `number`: The block number whose hash is being queried.
    ///
    /// # Returns:
    /// - `Result<B256, Self::Error>`: A `Result` containing either the `B256` hash of the block
    ///   or an error if the query fails.
    fn block_hash(&self, number: U256) -> Result<B256, Self::Error>;

    /// Retrieves the contract bytecode for a given address by first fetching the account information
    /// and then querying the code hash or fetching the code directly.
    ///
    /// # Parameters:
    /// - `address`: The account address whose contract bytecode is being queried.
    ///
    /// # Returns:
    /// - `Result<Bytecode, Self::Error>`: A `Result` containing either the `Bytecode` of the contract at the given
    ///   address or an error if the query fails. If the account does not exist or there is no code, an empty `Bytecode` is returned.
    ///
    /// # Default Implementation:
    /// - The method fetches the account information, retrieves the code either from the account or by querying the code hash,
    ///   and returns it. If no code is found, an empty bytecode is returned.
    fn code_by_address(&self, address: Address) -> Result<Bytecode, Self::Error> {
        let code = self
            .basic(address)?
            .and_then(|acc| acc.code.or_else(|| self.code_by_hash(acc.code_hash).ok()))
            .unwrap_or_default();
        Ok(code)
    }

    /// Inserts a contract into the specified address with the provided bytecode and balance.
    fn insert_contract(&mut self, address: Address, bytecode: Bytecode, balance: U256);

    /// Sets or updates an account in the database with the specified address, nonce, balance, and storage.
    ///
    /// If the account already exists, its nonce, balance, and storage are updated. If the account
    /// does not exist, it is created and initialized with the given values.
    fn set_account(
        &mut self,
        address: Address,
        nonce: u64,
        balance: U256,
        storage: FxHashMap<U256, U256>,
    );

    /// Retrieves the contract bytecode artifact for a given code hash.
    ///
    /// # Parameters:
    /// - `code_hash`: The hash of the contract code to be retrieved.
    ///
    /// # Returns:
    /// - `Result<Option<Self::Artifact>, Self::Error>`: A `Result` containing either
    ///   the `Self::Artifact` associated with the hash or an error if the query fails.
    fn get_artifact(&self, code_hash: B256) -> Result<Option<Self::Artifact>, Self::Error>;

    /// Sets or updates an account in the database with the specified address and the contract artifact.
    fn set_artifact(&mut self, code_hash: B256, artifact: Self::Artifact);

    /// Converts the current state of the database into a collection of `Account` objects.
    fn into_state(self) -> FxHashMap<Address, Account>;
}

/// An error that occurs during database access operations.
///
/// This error is typically encountered when there is a failure or inconsistency
/// in interacting with the in-memory or persistent database.
///
/// It provides a uniform way to handle errors within the storage layer.
///
/// # Example:
/// ```
/// use dora_runtime::db::DatabaseError;
/// let error = DatabaseError;
/// assert_eq!("Error during database access", error.to_string());
/// ```
#[derive(Error, Debug, Clone, Hash, PartialEq, Eq)]
#[error("Error during database access")]
pub struct DatabaseError;

impl DBErrorMarker for DatabaseError {}

/// Database commit interface.
pub trait DatabaseCommit {
    /// Commit changes to the database.
    fn commit(&mut self, changes: FxHashMap<Address, Account>);
}

/// An in-memory database for storing account information, contract bytecodes, and block hashes.
///
/// The [`MemoryDB`] struct is a simple, non-persistent database that holds blockchain data in memory,
/// including accounts, contracts, and block hashes.
///
/// It is typically used for testing, simulations, or lightweight applications where persistence isn't necessary.
///
/// # Fields:
/// - `accounts`: Stores account information mapped by address.
/// - `contracts`: Stores contract bytecode mapped by code hash.
/// - `block_hashes`: Stores block numbers and their corresponding hashes.
///
/// # Example Usage:
/// ```no_run
/// use dora_primitives::{B256, U256};
/// use dora_runtime::db::MemoryDB;
/// let mut db = MemoryDB::new();
/// db.insert_block_hash(U256::from(1), B256::from_slice(&[0; 32]));
/// ```
#[derive(Clone, Debug, Default)]
pub struct MemoryDB {
    accounts: FxHashMap<Address, DbAccount>,
    contracts: FxHashMap<B256, Bytecode>,
    artifacts: FxHashMap<B256, SymbolArtifact>,
    block_hashes: FxHashMap<U256, B256>,
}

unsafe impl Send for MemoryDB {}
unsafe impl Sync for MemoryDB {}

impl MemoryDB {
    /// Creates a new `MemoryDB` instance.
    ///
    /// This function returns an empty in-memory database for accounts, contracts,
    /// and block hashes.
    ///
    /// # Example:
    /// ```no_check
    /// let db = MemoryDB::new();
    /// ```
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts a block hash for a given block number.
    ///
    /// This function associates a block number (`U256`) with a block hash (`B256`).
    /// It can be used to simulate block storage within the in-memory database.
    ///
    /// # Example:
    /// ```no_check
    /// let mut db = MemoryDB::new();
    /// db.insert_block_hash(U256::from(1), B256::from_slice(&[0x12; 32]));
    /// ```
    #[inline]
    pub fn insert_block_hash(&mut self, number: U256, hash: B256) {
        self.block_hashes.insert(number, hash);
    }

    /// Sets the balance of an account at the specified address.
    ///
    /// If the account does not exist, it is created and initialized with an empty nonce and storage.
    ///
    /// # Parameters
    ///
    /// - `address`: The address of the account to update.
    /// - `balance`: The new balance to set for the account.
    ///
    /// # Example
    ///
    /// ```no_check
    /// db.set_balance(address, U256::from(1000));
    /// ```
    #[inline]
    pub fn set_balance(&mut self, address: Address, balance: U256) {
        let account = self.accounts.entry(address).or_insert(DbAccount::empty());
        account.balance = balance;
    }

    /// Retrieves the balance of an account at the specified address, if it exists.
    ///
    /// # Parameters
    ///
    /// - `address`: The address of the account to query.
    ///
    /// # Returns
    ///
    /// - `Option<U256>`: The balance of the account, or `None` if the account does not exist.
    ///
    /// # Example
    ///
    /// ```no_check
    /// let balance = db.get_balance(address);
    /// ```
    #[inline]
    pub fn get_balance(&self, address: Address) -> Option<U256> {
        self.accounts.get(&address).map(|acc| acc.balance)
    }

    /// Checks if the account at the specified address has been created.
    ///
    /// # Parameters
    ///
    /// - `address`: The address of the account to check.
    ///
    /// # Returns
    ///
    /// - `bool`: `true` if the account has the `Created` status, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```no_check
    /// let is_created = db.address_is_created(address);
    /// ```
    #[inline]
    pub fn address_is_created(&self, address: Address) -> bool {
        self.accounts
            .get(&address)
            .map(|acc| acc.status.contains(AccountStatus::Created))
            .unwrap_or(false)
    }

    /// Sets the status of an account at the specified address.
    ///
    /// If the account does not exist, it is created and initialized with an empty nonce and storage.
    ///
    /// # Parameters
    ///
    /// - `address`: The address of the account to update.
    /// - `status`: The new status to set for the account.
    ///
    /// # Example
    ///
    /// ```no_check
    /// db.set_status(address, AccountStatus::Cold);
    /// ```
    #[inline]
    pub fn set_status(&mut self, address: Address, status: AccountStatus) {
        let account = self.accounts.entry(address).or_insert(DbAccount::empty());
        account.status = status;
    }

    /// Returns a modified version of the current database, with a contract added to the specified address.
    ///
    /// The method calls `insert_contract` to add the contract and then returns `self`.
    ///
    /// # Parameters
    ///
    /// - `address`: The address of the account that will hold the contract.
    /// - `bytecode`: The bytecode of the contract to insert.
    ///
    /// # Returns
    ///
    /// - `Self`: The modified instance of the `MemoryDB`.
    ///
    /// # Example
    ///
    /// ```no_check
    /// db.with_contract(address, bytecode);
    /// ```
    #[inline]
    pub fn with_contract(mut self, address: Address, bytecode: Bytecode) -> Self {
        self.insert_contract(address, bytecode, U256::ZERO);
        self
    }

    /// Writes a storage value to the account at the specified address and key.
    ///
    /// If the account does not exist, it is created and initialized with an empty storage map.
    ///
    /// # Parameters
    ///
    /// - `address`: The address of the account to update.
    /// - `key`: The storage key to write the value to.
    /// - `value`: The value to store at the given key.
    ///
    /// # Example
    ///
    /// ```no_check
    /// db.sstore(address, key, value);
    /// ```
    #[inline]
    pub fn sstore(&mut self, address: Address, key: U256, value: U256) {
        let account = self.accounts.entry(address).or_insert(DbAccount::empty());
        account.storage.insert(key, value);
    }

    /// Reads a storage value from the account at the specified address and key.
    ///
    /// # Parameters
    ///
    /// - `address`: The address of the account to query.
    /// - `key`: The storage key to retrieve the value from.
    ///
    /// # Returns
    ///
    /// - `U256`: The storage value stored at the key, or `U256::ZERO` if not found.
    ///
    /// # Example
    ///
    /// ```no_check
    /// let value = db.sload(address, key);
    /// ```
    #[inline]
    pub fn sload(&self, address: Address, key: U256) -> U256 {
        self.accounts
            .get(&address)
            .and_then(|acc| acc.storage.get(&key).cloned())
            .unwrap_or_default()
    }

    #[inline]
    pub fn store_contract(&mut self, account: &AccountInfo) {
        if let Some(code) = account.code.as_ref() {
            self.contracts
                .entry(account.code_hash)
                .or_insert_with(|| code.clone());
        }
    }
}

impl Database for MemoryDB {
    type Error = Infallible;
    type Artifact = SymbolArtifact;

    fn basic(&self, address: Address) -> Result<Option<AccountInfo>, Self::Error> {
        Ok(self.accounts.get(&address).cloned().map(AccountInfo::from))
    }

    fn code_by_hash(&self, code_hash: B256) -> Result<Bytecode, Self::Error> {
        Ok(self.contracts.get(&code_hash).cloned().unwrap_or_default())
    }

    fn storage(&self, address: Address, index: U256) -> Result<U256, Self::Error> {
        Ok(self.sload(address, index))
    }

    fn block_hash(&self, number: U256) -> Result<B256, Self::Error> {
        Ok(self.block_hashes.get(&number).cloned().unwrap_or_default())
    }

    fn insert_contract(&mut self, address: Address, bytecode: Bytecode, balance: U256) {
        let hash = keccak256(bytecode.original_byte_slice());
        let account = DbAccount {
            bytecode_hash: hash,
            balance,
            nonce: 1,
            status: AccountStatus::Created,
            ..Default::default()
        };

        self.accounts.insert(address, account);
        self.contracts.insert(hash, bytecode);
    }

    fn set_account(
        &mut self,
        address: Address,
        nonce: u64,
        balance: U256,
        storage: FxHashMap<U256, U256>,
    ) {
        let account = self.accounts.entry(address).or_insert(DbAccount::empty());
        account.nonce = nonce;
        account.balance = balance;
        account.storage = storage;
    }

    fn get_artifact(&self, code_hash: B256) -> Result<Option<Self::Artifact>, Self::Error> {
        Ok(self.artifacts.get(&code_hash).cloned())
    }

    fn set_artifact(&mut self, code_hash: B256, artifact: Self::Artifact) {
        self.artifacts.insert(code_hash, artifact);
    }

    fn into_state(self) -> FxHashMap<Address, Account> {
        self.accounts
            .into_iter()
            .map(|(address, db_account)| {
                (
                    address,
                    Account {
                        info: AccountInfo::from(db_account.clone()),
                        storage: db_account
                            .storage
                            .into_iter()
                            .map(|(k, v)| (k, StorageSlot::new(v)))
                            .collect(),
                        status: db_account.status,
                    },
                )
            })
            .collect()
    }
}

impl DatabaseCommit for MemoryDB {
    /// Commits a set of changes to the in-memory database.
    ///
    /// The changes consist of a map of addresses and their corresponding `Account` objects.
    /// Each account's balance, nonce, status, and storage are updated accordingly. If the account
    /// is created but self-destructed or untouched, it is skipped.
    ///
    /// # Parameters
    ///
    /// - `changes`: A `HashMap<Address, Account>` containing the changes to be committed.
    ///
    /// # Example
    ///
    /// ```no_check
    /// db.commit(changes);
    /// ```
    fn commit(&mut self, changes: FxHashMap<Address, Account>) {
        for (address, account) in changes {
            if !account.is_touched() {
                continue;
            }
            if account.is_selfdestructed() {
                let db_account = self.accounts.entry(address).or_default();
                db_account.storage.clear();
                db_account.status = AccountStatus::LoadedAsNotExisting;
                db_account.balance = U256::ZERO;
                db_account.nonce = 0;
                db_account.bytecode_hash = KECCAK_EMPTY;
                continue;
            }
            let is_newly_created = account.is_created();
            self.store_contract(&account.info);

            let db_account = self
                .accounts
                .entry(address)
                .or_insert_with(DbAccount::empty);
            db_account.nonce = account.info.nonce;
            db_account.balance = account.info.balance;
            db_account.status = account.status;
            db_account.bytecode_hash = account.info.code_hash;
            db_account.account_state = if is_newly_created {
                db_account.storage.clear();
                AccountState::StorageCleared
            } else if db_account.account_state.is_storage_cleared() {
                AccountState::StorageCleared
            } else {
                AccountState::Touched
            };
            db_account.storage.extend(
                account
                    .storage
                    .into_iter()
                    .map(|(key, value)| (key, value.present_value)),
            );
        }
    }
}

/// Represents an account in the in-memory database, including its nonce, balance, storage, and bytecode.
///
/// The `DbAccount` struct stores account information, such as the nonce, balance, and storage state. It also
/// includes the account's bytecode hash and status flags. This struct is designed to hold both contract
/// and user accounts in the blockchain.
///
/// # Fields:
/// - `nonce`: The number of transactions sent from the account.
/// - `balance`: The account's balance.
/// - `storage`: A `HashMap` representing the account's key-value storage slots.
/// - `bytecode_hash`: The hash of the account's contract bytecode, if applicable.
/// - `status`: Flags representing the account's status (e.g., created, self-destructed).
///
/// # Example:
/// ```no_check
/// let account = DbAccount {
///     nonce: 1,
///     balance: U256::from(1000),
///     storage: FxHashMap::new(),
///     bytecode_hash: B256::zero(),
///     status: AccountStatus::default(),
/// };
/// ```
#[derive(Clone, Default, Debug, PartialEq)]
pub struct DbAccount {
    pub nonce: u64,
    pub balance: U256,
    pub storage: FxHashMap<U256, U256>,
    pub bytecode_hash: B256,
    pub status: AccountStatus,
    /// If account is selfdestructed or newly created, storage will be cleared.
    pub account_state: AccountState,
}

impl DbAccount {
    /// Creates an empty account with a zero balance, nonce, and empty storage.
    ///
    /// This is useful for initializing new accounts or resetting existing ones.
    ///
    /// # Example:
    /// ```no_check
    /// let empty_account = DbAccount::empty();
    /// ```
    pub fn empty() -> Self {
        DbAccount {
            nonce: 0,
            balance: U256::ZERO,
            storage: FxHashMap::default(),
            bytecode_hash: KECCAK_EMPTY,
            status: AccountStatus::Created,
            account_state: AccountState::None,
        }
    }
}

/// Converts a `DbAccount` into an `AccountInfo` object, which represents the basic information
/// of an account without the storage or status flags.
///
/// This conversion is useful when transferring account data between different database layers or
/// when storage and status flags are not needed.
///
/// # Example:
/// ```no_check
/// let db_account = DbAccount::empty();
/// let account_info: AccountInfo = db_account.into();
/// ```
impl From<DbAccount> for AccountInfo {
    fn from(db_account: DbAccount) -> Self {
        Self {
            balance: db_account.balance,
            nonce: db_account.nonce,
            code_hash: db_account.bytecode_hash,
            code: None,
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub enum AccountState {
    /// Before Spurious Dragon hardfork there was a difference between empty and not existing.
    /// And we are flagging it here.
    NotExisting,
    /// VM touched this account. For newer hardfork this means it can be cleared/removed from state.
    Touched,
    /// VM cleared storage of this account, mostly by selfdestruct, we don't ask database for storage slots
    /// and assume they are U256::ZERO
    StorageCleared,
    /// VM didn't interacted with this account
    #[default]
    None,
}

impl AccountState {
    /// Returns `true` if VM cleared storage of this account.
    #[inline]
    pub fn is_storage_cleared(&self) -> bool {
        matches!(self, AccountState::StorageCleared)
    }
}
