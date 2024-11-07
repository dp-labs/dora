use dora_primitives::{
    account::{Account, AccountInfo, AccountStatus, EMPTY_CODE_HASH_STR},
    db::{Database, MemoryDb, StorageSlot},
    Bytecode, EVMAddress as Address, B256, U256,
};
use rustc_hash::FxHashMap;
use sha3::{Digest, Keccak256};
use std::{collections::hash_map::Entry, str::FromStr};

/// Represents a journal entry for an individual storage slot, maintaining both its original and present values.
///
/// This is useful for tracking state changes in a storage slot during the execution of a transaction or smart contract
/// interaction.
///
/// # Fields:
/// - `original_value`: The original value of the storage slot before any modifications.
/// - `present_value`: The current value of the storage slot after possible modifications.
///
/// # Example Usage:
/// ```no_check
/// let slot = JournalStorageSlot::from(U256::from(10));
/// ```
#[derive(Clone, Default, Debug, PartialEq)]
pub struct JournalStorageSlot {
    pub original_value: U256,
    pub present_value: U256,
}

impl From<U256> for JournalStorageSlot {
    /// Creates a new `JournalStorageSlot` from a given `U256` value.
    ///
    /// Both the `original_value` and `present_value` are initialized to the provided value.
    ///
    /// # Arguments:
    /// - `value`: The initial value for the storage slot.
    ///
    /// # Returns:
    /// - `JournalStorageSlot`: A new instance with both fields set to the provided `value`.
    fn from(value: U256) -> Self {
        Self {
            original_value: value,
            present_value: value,
        }
    }
}

/// Represents a journal entry for an account, maintaining information such as nonce, balance, storage, and bytecode hash.
///
/// This structure is used to track the state of an account during transaction execution. It contains fields for
/// account-specific data and maintains the storage slots in a `HashMap`.
///
/// # Fields:
/// - `nonce`: The transaction nonce of the account.
/// - `balance`: The current balance of the account.
/// - `storage`: A map of storage slots associated with the account.
/// - `bytecode_hash`: The hash of the bytecode associated with the account.
/// - `status`: The current status of the account, such as `Created` or `Cold`.
///
/// # Example Usage:
/// ```no_check
/// let account = JournalAccount::new_created(U256::from(1000));
/// ```
#[derive(Clone, Default, Debug, PartialEq)]
pub struct JournalAccount {
    pub nonce: u64,
    pub balance: U256,
    pub storage: FxHashMap<U256, JournalStorageSlot>,
    pub bytecode_hash: B256,
    pub status: AccountStatus,
}

impl JournalAccount {
    /// Checks whether the account has any associated bytecode.
    ///
    /// This method returns `true` if the account has a non-empty bytecode hash, and `false` otherwise.
    ///
    /// # Returns:
    /// - `bool`: `true` if the account contains bytecode, otherwise `false`.
    ///
    /// # Example Usage:
    /// ```no_check
    /// if account.has_code() {
    ///     // Account has bytecode.
    /// }
    /// ```
    pub fn has_code(&self) -> bool {
        !(self.bytecode_hash == B256::zero()
            || self.bytecode_hash == B256::from_str(EMPTY_CODE_HASH_STR).unwrap_or_default())
    }

    /// Creates a new account with the provided balance and initializes it with default values.
    ///
    /// This is a convenience method for creating a newly created account with zero nonce, the provided balance,
    /// an empty storage map, and a default bytecode hash.
    ///
    /// # Arguments:
    /// - `balance`: The initial balance of the new account.
    ///
    /// # Returns:
    /// - `JournalAccount`: A new account with the provided balance and default values for other fields.
    ///
    /// # Example Usage:
    /// ```no_check
    /// let account = JournalAccount::new_created(U256::from(1000));
    /// ```
    pub fn new_created(balance: U256) -> Self {
        Self {
            nonce: 0,
            balance,
            storage: FxHashMap::default(),
            bytecode_hash: B256::from_str(EMPTY_CODE_HASH_STR).unwrap_or_default(),
            status: AccountStatus::Created,
        }
    }
}

impl From<AccountInfo> for JournalAccount {
    /// Converts an `AccountInfo` into a `JournalAccount`.
    ///
    /// This method transfers the relevant fields from `AccountInfo` into a new `JournalAccount` instance.
    ///
    /// # Arguments:
    /// - `info`: The `AccountInfo` instance to convert.
    ///
    /// # Returns:
    /// - `JournalAccount`: A new account created from the `AccountInfo` fields.
    fn from(info: AccountInfo) -> Self {
        Self {
            nonce: info.nonce,
            balance: info.balance,
            storage: Default::default(),
            bytecode_hash: info.code_hash,
            status: AccountStatus::Cold,
        }
    }
}

impl From<&JournalAccount> for AccountInfo {
    /// Converts a reference to a `JournalAccount` into an `AccountInfo`.
    ///
    /// This method extracts the relevant information from the `JournalAccount` and returns an `AccountInfo`.
    ///
    /// # Arguments:
    /// - `acc`: A reference to the `JournalAccount` to convert.
    ///
    /// # Returns:
    /// - `AccountInfo`: A new `AccountInfo` created from the `JournalAccount`.
    fn from(acc: &JournalAccount) -> Self {
        Self {
            balance: acc.balance,
            nonce: acc.nonce,
            code_hash: acc.bytecode_hash,
            code: None,
        }
    }
}

type AccountState = FxHashMap<Address, JournalAccount>;
type ContractState = FxHashMap<B256, Bytecode>;

/// A struct that manages account and contract states in memory, tracking changes during execution.
///
/// The `Journal` stores accounts, contracts, and block hashes and optionally interacts with a `MemoryDb` to persist
/// the data. This is typically used during the execution of transactions or smart contracts.
///
/// # Fields:
/// - `accounts`: A map of addresses to their corresponding journaled account state.
/// - `contracts`: A map of contract bytecode hashes to their respective bytecode.
/// - `block_hashes`: A map of block numbers to their corresponding block hashes.
/// - `db`: An optional reference to a mutable `MemoryDb`, used to persist changes if necessary.
///
/// # Example Usage:
/// ```no_check
/// let mut journal = Journal::new(memory_db);
/// journal.new_account(address, U256::from(1000));
/// journal.new_contract(contract_address, bytecode, U256::from(500));
/// ```
#[derive(Default, Debug)]
pub struct Journal {
    accounts: AccountState,
    contracts: ContractState,
    block_hashes: FxHashMap<U256, B256>,
    pub db: MemoryDb,
}

impl Journal {
    /// Creates a new `Journal` with a reference to a mutable `MemoryDb`.
    ///
    /// This initializes an empty journal, optionally connected to a `MemoryDb` for database interactions.
    ///
    /// # Arguments:
    /// - `db`: A mutable reference to a `MemoryDb`.
    ///
    /// # Returns:
    /// - `Self`: A new `Journal` instance.
    ///
    /// # Example Usage:
    /// ```no_check
    /// let mut journal = Journal::new(memory_db);
    /// ```
    pub fn new(db: MemoryDb) -> Self {
        Self {
            db,
            ..Default::default()
        }
    }

    #[inline]
    pub fn cloned_db(&self) -> MemoryDb {
        self.db.clone()
    }

    /// Adds a new account to the journal with the given address and initial balance.
    ///
    /// If the account already exists, an error is returned.
    ///
    /// # Arguments:
    /// - `address`: The address of the new account.
    /// - `balance`: The initial balance of the new account.
    ///
    /// # Returns:
    /// - `Result<(), String>`: `Ok` if the account is successfully added, or an error message if it already exists.
    ///
    /// # Example Usage:
    /// ```no_check
    /// match journal.new_account(address, U256::from(1000)) {
    ///     Ok(()) => println!("Account created."),
    ///     Err(e) => println!("Error: {}", e),
    /// }
    /// ```
    pub fn new_account(&mut self, address: Address, balance: U256) -> Result<(), String> {
        if self.accounts.contains_key(&address) {
            return Err("Account already exists.".into());
        }
        let account = JournalAccount::new_created(balance);
        self.accounts.insert(address, account);
        Ok(())
    }

    /// Adds a new contract to the journal with the given address, bytecode, and balance.
    ///
    /// This function computes the bytecode hash and inserts the contract into the journal.
    ///
    /// # Arguments:
    /// - `address`: The address of the contract.
    /// - `bytecode`: The bytecode of the contract to be added.
    /// - `balance`: The initial balance of the contract account.
    ///
    /// # Example Usage:
    /// ```no_check
    /// journal.new_contract(contract_address, bytecode, U256::from(500));
    /// ```
    pub fn new_contract(&mut self, address: Address, bytecode: Bytecode, balance: U256) {
        let mut hasher = Keccak256::new();
        hasher.update(&bytecode);
        let hash = B256::from_slice(&hasher.finalize());

        let account = JournalAccount {
            bytecode_hash: hash,
            balance,
            nonce: 1,
            status: AccountStatus::Created,
            ..Default::default()
        };

        self.accounts.insert(address, account);
        self.contracts.insert(hash, bytecode);
    }

    /// Sets the balance for the specified account.
    ///
    /// This function updates the balance of an account and marks the account as "touched."
    ///
    /// # Parameters
    /// - `address`: The address of the account.
    /// - `balance`: The new balance to set for the account.
    ///
    /// # Returns
    /// - `Ok(())`: If the account is found and the balance is successfully updated.
    /// - `Err(String)`: If the account is not found.
    pub fn set_balance(&mut self, address: &Address, balance: U256) -> Result<(), String> {
        if let Some(acc) = self._get_account_mut(address) {
            acc.balance = balance;
            acc.status |= AccountStatus::Touched;
            Ok(())
        } else {
            Err("Account not found.".into())
        }
    }

    /// Sets the nonce for the specified account.
    ///
    /// This function updates the nonce of an account and marks the account as "touched."
    ///
    /// # Parameters
    /// - `address`: The address of the account.
    /// - `nonce`: The new nonce value to set.
    ///
    /// # Returns
    /// - `Ok(())`: If the account is found and the nonce is successfully updated.
    /// - `Err(String)`: If the account is not found.
    pub fn set_nonce(&mut self, address: &Address, nonce: u64) -> Result<(), String> {
        if let Some(acc) = self._get_account_mut(address) {
            acc.nonce = nonce;
            acc.status |= AccountStatus::Touched;
            Ok(())
        } else {
            Err("Account not found.".into())
        }
    }

    /// Updates the status of an account.
    ///
    /// This function modifies the account's status by applying a bitwise OR operation with the provided status flag.
    ///
    /// # Parameters
    /// - `address`: The address of the account.
    /// - `status`: The new status flag to apply.
    ///
    /// # Returns
    /// - `Ok(())`: If the account is found and the status is successfully updated.
    /// - `Err(String)`: If the account is not found.
    pub fn set_status(&mut self, address: &Address, status: AccountStatus) -> Result<(), String> {
        if let Some(acc) = self._get_account_mut(address) {
            acc.status |= status;
            Ok(())
        } else {
            Err("Account not found.".into())
        }
    }

    /// Retrieves the account information for a given address.
    ///
    /// Returns the account details as an `AccountInfo` if the account exists, or `None` if the account is not found.
    ///
    /// # Parameters
    /// - `address`: The address of the account to retrieve.
    ///
    /// # Returns
    /// - `Some(AccountInfo)`: If the account exists.
    /// - `None`: If the account does not exist.
    pub fn get_account(&mut self, address: &Address) -> Option<AccountInfo> {
        self._get_account(address).map(AccountInfo::from)
    }

    /// Retrieves the bytecode for a contract by address.
    ///
    /// This function checks if the account has associated bytecode, and attempts to load it either from cached
    /// contracts or by querying the database. Returns the bytecode if found, otherwise returns default bytecode.
    ///
    /// # Parameters
    /// - `address`: The address of the account to get the bytecode for.
    ///
    /// # Returns
    /// - `Bytecode`: The bytecode of the contract, or default bytecode if none is found.
    pub fn code_by_address(&mut self, address: &Address) -> Bytecode {
        let acc = match self._get_account(address) {
            Some(acc) if acc.has_code() => acc,
            _ => return Bytecode::default(),
        };

        let hash = acc.bytecode_hash;
        self.contracts
            .get(&hash)
            .cloned()
            .unwrap_or_else(|| self.db.code_by_hash(hash).unwrap_or_default())
    }

    /// Checks if an account is considered "warm" (not cold) for gas metering purposes.
    ///
    /// Warm accounts are those that have been accessed in the current execution context, while cold accounts incur
    /// higher gas costs for access.
    ///
    /// # Parameters
    /// - `address`: The address of the account to check.
    ///
    /// # Returns
    /// - `true`: If the account is warm.
    /// - `false`: If the account is cold or does not exist.
    pub fn is_account_warm(&self, address: &Address) -> bool {
        self.accounts
            .get(address)
            .map(|acc| !acc.status.contains(AccountStatus::Cold))
            .unwrap_or(false)
    }

    /// Prefetches the account from storage and caches it in the journal.
    ///
    /// This method is a no-op if the account is already present in the journal.
    ///
    /// # Parameters
    /// - `address`: The address of the account to prefetch.
    pub fn prefetch_account(&mut self, address: &Address) {
        let _ = self._get_account(address);
    }

    /// Prefetches specific storage keys for an account.
    ///
    /// This method fetches and caches the storage slots for the given keys from the database, storing them in the journal.
    ///
    /// # Parameters
    /// - `address`: The address of the account.
    /// - `keys`: The storage keys to prefetch.
    pub fn prefetch_account_keys(&mut self, address: &Address, keys: &[U256]) {
        if self._get_account(address).is_none() {
            return;
        };
        let slots: FxHashMap<U256, JournalStorageSlot> = keys
            .iter()
            .map(|key| (*key, self._fetch_storage_from_db(address, key)))
            .collect();

        let acc = self._get_account_mut(address).unwrap();
        acc.storage.extend(slots);
    }

    /// Checks if a storage key for an account is considered "warm."
    ///
    /// # Parameters
    /// - `address`: The address of the account.
    /// - `key`: The storage key to check.
    ///
    /// # Returns
    /// - `true`: If the storage key is warm.
    /// - `false`: If the storage key is cold or not found.
    pub fn is_key_warm(&self, address: &Address, key: &U256) -> bool {
        self.accounts
            .get(address)
            .and_then(|acc| acc.storage.get(key))
            .is_some()
    }

    /// Reads a storage slot for the given account and key.
    ///
    /// This method retrieves the storage value for the specified key, caching it in the journal if necessary.
    ///
    /// # Parameters
    /// - `address`: The address of the account.
    /// - `key`: The storage key to read.
    ///
    /// # Returns
    /// - `Some(JournalStorageSlot)`: The storage value if found.
    /// - `None`: If the account or key is not found.
    pub fn read_storage(&mut self, address: &Address, key: &U256) -> Option<JournalStorageSlot> {
        if let Some(acc) = self._get_account(address) {
            let slot = acc
                .storage
                .get(key)
                .cloned()
                .unwrap_or_else(|| self._fetch_storage_from_db(address, key));

            if let Some(acc_mut) = self._get_account_mut(address) {
                acc_mut.storage.insert(*key, slot.clone());
            }
            return Some(slot);
        }
        None
    }

    /// Writes a storage slot for the given account and key.
    ///
    /// Updates the present value of the storage slot and marks the account as "touched."
    ///
    /// # Parameters
    /// - `address`: The address of the account.
    /// - `key`: The storage key to write.
    /// - `value`: The new value to set.
    pub fn write_storage(&mut self, address: &Address, key: U256, value: U256) {
        let acc = self._get_account(address).unwrap(); //TODO handle error here
        let mut slot = acc
            .storage
            .get(&key)
            .cloned()
            .unwrap_or(self._fetch_storage_from_db(address, &key));

        slot.present_value = value;
        let acc = self._get_account_mut(address).unwrap();
        acc.storage.insert(key, slot.clone());
        acc.status |= AccountStatus::Touched;
    }

    /// Retrieves the block hash for a given block number.
    ///
    /// If the block hash is not already cached, it queries the database and caches the result.
    ///
    /// # Parameters
    /// - `number`: The block number to retrieve the hash for.
    ///
    /// # Returns
    /// - `B256`: The block hash, or a default value if the hash is not found.
    pub fn get_block_hash(&mut self, number: &U256) -> B256 {
        if let Some(hash) = self.block_hashes.get(number) {
            return *hash;
        }
        let block_hash = self.db.block_hash(*number).unwrap_or_default();

        self.block_hashes.insert(*number, block_hash);
        block_hash
    }

    pub fn into_state(&self) -> FxHashMap<Address, Account> {
        self.accounts
            .iter()
            .map(|(address, acc)| {
                let code = if acc.has_code() {
                    self.contracts.get(&acc.bytecode_hash).cloned()
                } else {
                    None
                };

                let storage = acc
                    .storage
                    .iter()
                    .map(|(&key, slot)| {
                        (
                            key,
                            StorageSlot {
                                original_value: slot.original_value,
                                present_value: slot.present_value,
                                is_cold: false,
                            },
                        )
                    })
                    .collect();

                (
                    *address,
                    Account {
                        info: AccountInfo {
                            balance: acc.balance,
                            nonce: acc.nonce,
                            code_hash: acc.bytecode_hash,
                            code,
                        },
                        storage,
                        status: acc.status,
                    },
                )
            })
            .collect()
    }

    /// Ejects the base journal state by creating a new journal instance.
    ///
    /// This method returns a new `Journal` containing clones of the current journal's accounts, contracts,
    /// block hashes, and the database reference. The original database reference in the current journal is taken.
    ///
    /// # Returns
    /// - `Journal`: A new journal instance with the base state.
    pub fn eject_base(&mut self) -> Self {
        Self {
            accounts: self.accounts.clone(),
            contracts: self.contracts.clone(),
            block_hashes: self.block_hashes.clone(),
            db: self.db.clone(),
        }
    }

    /// Extends the current journal with the state from a successful execution.
    ///
    /// This function replaces the accounts, contracts, and block hashes in the current journal with those from the
    /// `other` journal. It also updates the database reference with the one from `other`.
    ///
    /// # Parameters
    /// - `other`: The `Journal` instance containing the state to merge from a successful execution.
    pub fn extend_from_successful(&mut self, other: Journal) {
        self.accounts = other.accounts;
        self.contracts = other.contracts;
        self.block_hashes = other.block_hashes;
        self.db = other.db;
    }

    /// Extends the current journal with the state from a reverted execution.
    ///
    /// This function only updates the database reference from the `other` journal, leaving accounts, contracts,
    /// and block hashes unchanged.
    ///
    /// # Parameters
    /// - `other`: The `Journal` instance containing the state to merge from a reverted execution.
    pub fn extend_from_reverted(&mut self, other: Journal) {
        self.db = other.db;
    }

    fn _get_account(&mut self, address: &Address) -> Option<&JournalAccount> {
        self._get_account_mut(address).map(|acc| &*acc)
    }

    fn _get_account_mut(&mut self, address: &Address) -> Option<&mut JournalAccount> {
        match self.accounts.entry(*address) {
            Entry::Occupied(e) => Some(e.into_mut()),
            Entry::Vacant(e) => {
                let acc = self.db.basic(*address).ok().flatten()?;
                let mut journal_acc = JournalAccount::from(acc);
                journal_acc.status = AccountStatus::Loaded;
                Some(e.insert(journal_acc))
            }
        }
    }

    fn _fetch_storage_from_db(&mut self, address: &Address, key: &U256) -> JournalStorageSlot {
        self.db
            .storage(*address, *key)
            .ok()
            .map(JournalStorageSlot::from)
            .unwrap_or_default()
    }
}
