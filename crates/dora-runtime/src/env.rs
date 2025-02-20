//! Reference: [revm](https://github.com/bluealloy/revm)

use std::cmp::min;

use crate::{
    gas,
    result::{InvalidHeader, VMError},
    transaction::TransactionType,
};

use super::constants::{
    gas_cost::{
        init_code_cost, MAX_CODE_SIZE, TX_ACCESS_LIST_ADDRESS_COST,
        TX_ACCESS_LIST_STORAGE_KEY_COST, TX_BASE_COST, TX_CREATE_COST, TX_DATA_COST_PER_NON_ZERO,
        TX_DATA_COST_PER_ZERO,
    },
    MAX_BLOB_NUMBER_PER_BLOCK, VERSIONED_HASH_VERSION_KZG,
};
use super::result::InvalidTransaction;
use dora_primitives::{
    calc_blob_gasprice, Address, Bytes, SignedAuthorization, SpecId, B256, GAS_PER_BLOB, U256,
};

/// Represents the execution environment for the EVM/WASM, including block, transaction, and VM configuration.
/// It holds configuration settings specific to the VM, the current block being processed, and the transaction
/// being executed.
///
/// # Fields:
/// - `cfg`: VM configuration, including chain ID.
/// - `block`: Information about the current block, such as block number, timestamp, and coinbase.
/// - `tx`: Details about the current transaction, including caller, gas limit, and transaction type.
///
/// # Example Usage:
/// ```no_check
/// let env = Env::default();
/// ```
#[derive(Clone, Debug, Default)]
pub struct Env {
    /// VM configuration.
    pub cfg: CfgEnv,
    /// Block information.
    pub block: BlockEnv,
    /// Transaction information.
    pub tx: TxEnv,
}

impl Env {
    /// Consumes the intrinsic gas cost from the transaction's available gas limit.
    ///
    /// The intrinsic cost includes base transaction costs (e.g., for data, signature) that are deducted before any execution begins.
    /// This method checks whether the transaction has enough gas to cover the intrinsic cost. If there is sufficient gas, the intrinsic
    /// cost is subtracted from the gas limit; otherwise, an error is returned.
    ///
    /// # Returns
    ///
    /// - `Ok(())`: If the gas limit is sufficient to cover the intrinsic cost.
    /// - `Err(InvalidTransaction::CallGasCostMoreThanGasLimit)`: If the intrinsic cost exceeds the transaction's gas limit.
    ///
    /// # Example
    ///
    /// ```no_check
    /// env.consume_intrinsic_cost()?;
    /// ```
    pub fn consume_intrinsic_cost(&mut self) -> Result<(), InvalidTransaction> {
        let intrinsic_cost = self.calculate_intrinsic_cost();
        if self.tx.gas_limit >= intrinsic_cost {
            self.tx.gas_limit -= intrinsic_cost;
            Ok(())
        } else {
            Err(InvalidTransaction::CallGasCostMoreThanGasLimit)
        }
    }

    #[inline]
    pub fn effective_gas_price(&self) -> U256 {
        self.tx.effective_gas_price(self.block.basefee)
    }

    /// Validates the transaction based on various constraints such as transaction type, size limits, and gas fees.
    ///
    /// This method checks the transaction's validity against several criteria, including:
    /// - Whether the transaction is a contract creation and if the initialization code exceeds the maximum allowed size.
    /// - Validation of blob-specific fields (e.g., gas price, blob hashes) for transactions involving blobs.
    ///
    /// # Returns
    ///
    /// - `Ok(())`: If the transaction passes all validation checks.
    /// - `Err(InvalidTransaction)`: If any validation rule is violated, such as:
    ///   - `CreateInitCodeSizeLimit`: If contract creation's initialization code exceeds the size limit.
    ///   - `BlobGasPriceGreaterThanMax`: If the blob gas price exceeds the maximum fee allowed.
    ///   - `EmptyBlobs`: If the transaction involves blobs but no blob hashes are provided.
    ///   - `BlobCreateTransaction`: If the transaction is a contract creation, which is not allowed with blobs.
    ///   - `BlobVersionNotSupported`: If an unsupported versioned hash is detected in the blob hashes.
    ///   - `TooManyBlobs`: If the number of blob hashes exceeds the allowed maximum per block.
    ///
    /// # Example
    ///
    /// ```no_check
    /// env.validate_transaction()?;
    /// ```
    pub fn validate_transaction(&self) -> Result<(), InvalidTransaction> {
        let is_create = self.tx.transact_to.is_create();

        if is_create && self.tx.data.len() > 2 * MAX_CODE_SIZE {
            return Err(InvalidTransaction::CreateInitCodeSizeLimit);
        }

        if let Some(max_fee) = self.tx.max_fee_per_blob_gas {
            let price = self
                .block
                .blob_excess_gas_and_price
                .clone()
                .unwrap_or_default()
                .blob_gasprice;
            if U256::from(price) > max_fee {
                return Err(InvalidTransaction::BlobGasPriceGreaterThanMax);
            }
            if self.tx.blob_hashes.is_empty() {
                return Err(InvalidTransaction::EmptyBlobs);
            }
            if is_create {
                return Err(InvalidTransaction::BlobCreateTransaction);
            }

            for blob in &self.tx.blob_hashes {
                if blob[0] != VERSIONED_HASH_VERSION_KZG {
                    return Err(InvalidTransaction::BlobVersionNotSupported);
                }
            }

            let num_blobs = self.tx.blob_hashes.len();
            if num_blobs > MAX_BLOB_NUMBER_PER_BLOCK as usize {
                return Err(InvalidTransaction::TooManyBlobs {
                    have: num_blobs,
                    max: MAX_BLOB_NUMBER_PER_BLOCK as usize,
                });
            }
        }
        Ok(())
    }

    /// Validate the block environment.
    #[inline]
    pub fn validate_block_env(&self, spec_id: SpecId) -> Result<(), InvalidHeader> {
        // `prevrandao` is required for the merge
        if spec_id.is_enabled_in(SpecId::MERGE) && self.block.prevrandao.is_none() {
            return Err(InvalidHeader::PrevrandaoNotSet);
        }
        // `excess_blob_gas` is required for Cancun
        if spec_id.is_enabled_in(SpecId::CANCUN) && self.block.blob_excess_gas_and_price.is_none() {
            return Err(InvalidHeader::ExcessBlobGasNotSet);
        }
        Ok(())
    }

    /// Validate initial transaction gas.
    pub fn validate_initial_tx_gas(&self, spec_id: SpecId) -> Result<u64, VMError> {
        let is_create = self.tx.transact_to.is_create();
        let authorization_list_num = if self.tx.tx_type == TransactionType::Eip7702 {
            self.tx.authorization_list_len() as u64
        } else {
            0
        };
        let initial_gas_cost = gas::validate_initial_tx_gas(
            spec_id,
            &self.tx.data,
            is_create,
            &self.tx.access_list,
            authorization_list_num,
        );
        // Additional check to see if limit is big enough to cover initial gas.
        if initial_gas_cost > self.tx.gas_limit {
            return Err(VMError::Transaction(
                InvalidTransaction::CallGasCostMoreThanGasLimit,
            ));
        }
        Ok(initial_gas_cost)
    }

    /// Calculates the intrinsic cost based on the transaction data.
    pub fn calculate_intrinsic_cost(&self) -> u64 {
        let data_cost: u64 = self
            .tx
            .data
            .iter()
            .map(|byte| {
                if *byte == 0 {
                    TX_DATA_COST_PER_ZERO
                } else {
                    TX_DATA_COST_PER_NON_ZERO
                }
            })
            .sum();

        let create_cost = if !self.tx.transact_to.is_create() {
            0
        } else {
            TX_CREATE_COST + init_code_cost(self.tx.data.len())
        };

        let access_list_cost: u64 = self
            .tx
            .access_list
            .iter()
            .map(|(_, keys)| {
                TX_ACCESS_LIST_ADDRESS_COST + keys.len() as u64 * TX_ACCESS_LIST_STORAGE_KEY_COST
            })
            .sum();

        TX_BASE_COST + data_cost + create_cost + access_list_cost
    }

    /// Calculates the maximum [EIP-4844] `data_fee` of the transaction.
    ///
    /// This is used for ensuring that the user has at least enough funds to pay the
    /// `max_fee_per_blob_gas * total_blob_gas`, on top of regular gas costs.
    ///
    /// See EIP-4844:
    /// <https://github.com/ethereum/EIPs/blob/master/EIPS/eip-4844.md#execution-layer-validation>
    #[inline]
    pub fn calc_max_data_fee(&self) -> Option<U256> {
        self.tx.calc_max_data_fee()
    }

    /// Calculates the [EIP-4844] `data_fee` of the transaction.
    ///
    /// Returns `None` if `Cancun` is not enabled.
    ///
    /// [EIP-4844](https://eips.ethereum.org/EIPS/eip-4844)
    #[inline]
    pub fn calc_data_fee(&self) -> Option<U256> {
        if self.tx.tx_type == TransactionType::Eip4844 {
            let blob_gas = U256::from(self.tx.total_blob_gas());
            let blob_gas_price = U256::from(
                self.block
                    .blob_excess_gas_and_price
                    .clone()
                    .unwrap_or_default()
                    .blob_gasprice,
            );
            return Some(blob_gas_price.saturating_mul(blob_gas));
        }
        None
    }
}

/// Configuration settings for the VM, including chain ID.
///
/// The `CfgEnv` struct holds the configuration details specific to the VM instance, such as the chain ID.
/// It allows the VM to distinguish between different blockchain networks and environments.
///
/// # Fields:
/// - `chain_id`: The unique identifier for the blockchain network (e.g., 1 for Ethereum mainnet).
///
/// # Example Usage:
/// ```no_check
/// let cfg_env = CfgEnv { chain_id: 1 };
/// ```
#[derive(Clone, Debug)]
pub struct CfgEnv {
    /// Chain ID of the VM, it will be compared to the transaction's Chain ID.
    /// Chain ID is introduced EIP-155
    pub chain_id: u64,
    /// If some it will effects EIP-170: Contract code size limit. Useful to increase this because of tests.
    /// By default it is 0x6000 (~25kb).
    pub limit_contract_code_size: Option<usize>,
    /// A hard memory limit in bytes beyond which [crate::result::OutOfGasError::Memory] cannot be resized.
    /// Defaults to `2^32 - 1` bytes per EIP-1985.
    pub memory_limit: u64,
    /// Skips the nonce validation against the account's nonce.
    pub disable_nonce_check: bool,
    /// Skips balance checks if true. Adds transaction cost to balance to ensure execution doesn't fail.
    pub disable_balance_check: bool,
}

impl Default for CfgEnv {
    fn default() -> Self {
        Self {
            chain_id: 1,
            limit_contract_code_size: None,
            memory_limit: (1 << 32) - 1,
            disable_nonce_check: false,
            disable_balance_check: false,
        }
    }
}

impl CfgEnv {
    /// Returns max code size from [`Self::limit_contract_code_size`] if set
    /// or default [`MAX_CODE_SIZE`] value.
    #[inline]
    pub fn max_code_size(&self) -> usize {
        self.limit_contract_code_size.unwrap_or(MAX_CODE_SIZE)
    }

    #[inline]
    pub const fn is_nonce_check_disabled(&self) -> bool {
        self.disable_nonce_check
    }

    #[inline]
    pub fn is_balance_check_disabled(&self) -> bool {
        self.disable_balance_check
    }
}

/// Information about the current block being processed in the VM, including block number and timestamp.
///
/// The `BlockEnv` struct holds various parameters related to the current block being executed in the VM.
/// This includes details like the block number, the address of the block's coinbase (miner), the timestamp,
/// and more advanced fields like blob gas prices.
///
/// # Fields:
/// - `number`: The block number.
/// - `coinbase`: The address of the miner or validator producing the block.
/// - `timestamp`: The block's timestamp in seconds since the Unix epoch.
/// - `basefee`: The base gas fee for transactions in the block.
/// - `prevrandao`: An optional 256-bit random value used for randomness in the block.
/// - `excess_blob_gas`: Optional blob gas excess for blob-carrying transactions (EIP-4844).
/// - `blob_gasprice`: Optional price per blob gas unit (EIP-4844).
///
/// # Example Usage:
/// ```no_check
/// let block_env = BlockEnv {
///     number: U256::from(100000),
///     coinbase: Address::from_low_u64_be(0x123),
///     timestamp: U256::from(1625247600),
///     basefee: U256::from(1000),
///     prevrandao: None,
///     excess_blob_gas: Some(100),
///     blob_gasprice: Some(200),
/// };
/// ```
#[derive(Clone, Debug, Default)]
pub struct BlockEnv {
    /// The number of ancestor blocks of this block (block height).
    pub number: U256,
    /// Coinbase or miner address that created and signed the block.
    pub coinbase: Address,
    /// The timestamp of the block in seconds since the UNIX epoch.
    pub timestamp: U256,
    /// The gas limit of the block.
    pub gas_limit: U256,
    /// The base fee per gas.
    ///
    /// [EIP-1559](https://eips.ethereum.org/EIPS/eip-1559)
    pub basefee: U256,
    /// The difficulty of the block.
    ///
    /// Unused after the Paris (AKA the merge) upgrade, and replaced by `prevrandao`.
    pub difficulty: U256,
    /// The output of the randomness beacon provided by the beacon chain.
    /// Replaces `difficulty` after the Paris (AKA the merge) upgrade with [EIP-4399].
    ///
    /// [EIP-4399](https://eips.ethereum.org/EIPS/eip-4399)
    pub prevrandao: Option<B256>,
    /// Excess blob gas and blob gasprice.
    pub blob_excess_gas_and_price: Option<BlobExcessGasAndPrice>,
}

impl BlockEnv {
    /// Takes `blob_excess_gas` saves it inside env.
    pub fn set_blob_excess_gas_and_price(&mut self, excess_blob_gas: u64, is_prague: bool) {
        self.blob_excess_gas_and_price =
            Some(BlobExcessGasAndPrice::new(excess_blob_gas, is_prague));
    }
}

/// Structure holding block blob excess gas and it calculates blob fee.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct BlobExcessGasAndPrice {
    /// The excess blob gas of the block.
    pub excess_blob_gas: u64,
    /// The calculated blob gas price based on the `excess_blob_gas`, See [calc_blob_gasprice]
    pub blob_gasprice: u128,
}

impl BlobExcessGasAndPrice {
    /// Creates a new instance by calculating the blob gas price with [`calc_blob_gasprice`].
    pub fn new(excess_blob_gas: u64, is_prague: bool) -> Self {
        let blob_gasprice = calc_blob_gasprice(excess_blob_gas, is_prague);
        Self {
            excess_blob_gas,
            blob_gasprice,
        }
    }
}

/// Contains information about the transaction being processed by the EVM, including sender and gas details.
///
/// The `TxEnv` struct represents the environment configuration specific to the transaction currently being executed.
/// It includes details such as the transaction's sender (`caller`), gas limit, gas price, and other fields relevant
/// to the transaction's execution.
///
/// # Fields:
/// - `caller`: The address of the account initiating the transaction.
/// - `gas_limit`: The maximum amount of gas allowed for the transaction's execution.
/// - `gas_price`: The price per unit of gas for the transaction.
/// - `transact_to`: The destination of the transaction, which can either be a contract call or a contract creation.
/// - `value`: The amount of Ether transferred with the transaction.
/// - `data`: The transaction's input data.
/// - `access_list`: A list of addresses and storage keys accessed during the transaction.
/// - `blob_hashes`: A list of blob hashes for blob-carrying transactions (EIP-4844).
/// - `max_fee_per_blob_gas`: Optional maximum fee per blob gas unit (EIP-4844).
///
/// # Example Usage:
/// ```no_check
/// let tx_env = TxEnv::default();
/// assert_eq!(tx_env.caller, Address::zero());
/// assert_eq!(tx_env.gas_limit, i64::MAX as u64);
/// ```
#[derive(Clone, Debug)]
pub struct TxEnv {
    /// The type of the transaction.
    pub tx_type: TransactionType,
    /// The caller aka Author aka transaction signer of the transaction.
    pub caller: Address,
    /// The gas limit of the transaction.
    pub gas_limit: u64,
    /// The gas price of the transaction.
    pub gas_price: U256,
    /// The destination of the transaction.
    pub transact_to: TxKind,
    /// The value sent to `transact_to`.
    pub value: U256,
    /// The data of the transaction.
    pub data: Bytes,
    /// The nonce of the transaction.
    pub nonce: u64,
    /// A list of addresses and storage keys that the transaction plans to access.
    ///
    /// [EIP-2930](https://eips.ethereum.org/EIPS/eip-2930)
    pub access_list: Vec<AccessListItem>,
    /// The list of blob versioned hashes. Per EIP there should be at least
    ///
    /// [EIP-4844](https://eips.ethereum.org/EIPS/eip-4844)
    pub blob_hashes: Vec<B256>,
    /// The max fee per blob gas.
    ///
    /// [EIP-4844](https://eips.ethereum.org/EIPS/eip-4844)
    pub max_fee_per_blob_gas: Option<U256>,
    /// The chain ID of the transaction.
    ///
    /// [EIP-155](https://eips.ethereum.org/EIPS/eip-155)
    pub chain_id: Option<u64>,
    /// The priority fee per gas.
    ///
    /// [EIP-1559](https://eips.ethereum.org/EIPS/eip-1559)
    pub gas_priority_fee: Option<U256>,
    /// List of authorizations, that contains the signature that authorizes this
    /// caller to place the code to signer account.
    ///
    /// Set EOA account code for one transaction
    ///
    /// [EIP-Set EOA account code for one transaction](https://eips.ethereum.org/EIPS/eip-7702)
    pub authorization_list: Vec<SignedAuthorization>,
}

pub type AccessListItem = (Address, Vec<B256>);

impl Default for TxEnv {
    fn default() -> Self {
        Self {
            caller: Address::ZERO,
            gas_limit: i64::MAX as _,
            gas_price: U256::ZERO,
            transact_to: TxKind::Create,
            value: U256::ZERO,
            data: Bytes::new(),
            nonce: 0,
            access_list: Vec::new(),
            blob_hashes: Vec::new(),
            tx_type: TransactionType::Legacy,
            chain_id: Some(1),
            gas_priority_fee: None,
            max_fee_per_blob_gas: None,
            authorization_list: vec![],
        }
    }
}

impl TxEnv {
    /// Returns the address of the destination of the transaction.
    ///
    /// This function returns the destination address for a `Call` transaction. If the transaction
    /// is for contract creation (`Create`), it returns `Address::zero()` as a placeholder.
    ///
    /// # Returns:
    /// - `Address`: The address of the contract to call or `Address::zero()` for contract creation.
    ///
    /// # Example Usage:
    /// ```no_check
    /// let tx_env = TxEnv::default();
    /// assert_eq!(tx_env.get_address(), Address::zero());
    /// ```
    #[inline]
    pub fn get_address(&self) -> Address {
        match self.transact_to {
            TxKind::Create => Address::default(),
            TxKind::Call(address) => address,
        }
    }

    #[inline]
    pub fn effective_gas_price(&self, base_fee: U256) -> U256 {
        let (max_fee, max_priority_fee) = match self.tx_type {
            TransactionType::Legacy => return self.gas_price,
            TransactionType::Eip2930 => return self.gas_price,
            TransactionType::Eip1559 => (self.gas_price, self.gas_priority_fee.unwrap_or_default()),
            TransactionType::Eip4844 => (self.gas_price, self.gas_priority_fee.unwrap_or_default()),
            TransactionType::Eip7702 => (self.gas_price, self.gas_priority_fee.unwrap_or_default()),
            TransactionType::Custom => unimplemented!("Custom tx not supported"),
        };

        min(max_fee, base_fee + max_priority_fee)
    }

    /// Maximum fee that can be paid for the transaction.
    pub fn max_fee(&self) -> U256 {
        match self.tx_type {
            TransactionType::Legacy => self.gas_price,
            TransactionType::Eip2930 => self.gas_price,
            TransactionType::Eip1559 => self.gas_price,
            TransactionType::Eip4844 => self.gas_price,
            TransactionType::Eip7702 => self.gas_price,
            TransactionType::Custom => unimplemented!("Custom tx not supported"),
        }
    }

    /// Calculates the maximum [EIP-4844] `data_fee` of the transaction.
    ///
    /// This is used for ensuring that the user has at least enough funds to pay the
    /// `max_fee_per_blob_gas * total_blob_gas`, on top of regular gas costs.
    ///
    /// See EIP-4844:
    /// <https://github.com/ethereum/EIPs/blob/master/EIPS/eip-4844.md#execution-layer-validation>
    pub fn calc_max_data_fee(&self) -> Option<U256> {
        if self.tx_type == TransactionType::Eip4844 {
            let blob_gas = U256::from(self.total_blob_gas());
            let max_blob_fee = self.max_fee_per_blob_gas.unwrap_or_default();
            return Some(max_blob_fee.saturating_mul(blob_gas));
        }
        None
    }

    /// Total gas for all blobs. Max number of blocks is already checked
    /// so we dont need to check for overflow.
    #[inline]
    pub fn total_blob_gas(&self) -> u64 {
        GAS_PER_BLOB * self.blob_hashes.len() as u64
    }

    /// Returns length of the authorization list.
    #[inline]
    pub fn authorization_list_len(&self) -> usize {
        if self.tx_type == TransactionType::Eip7702 {
            self.authorization_list.len()
        } else {
            0
        }
    }
}

/// The `to` field of a transaction. Either a target address, or empty for a
/// contract creation.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum TxKind {
    /// A transaction that creates a contract.
    #[default]
    Create,
    /// A transaction that calls a contract or transfer.
    Call(Address),
}

impl From<Option<Address>> for TxKind {
    /// Creates a `TxKind::Call` with the `Some` address, `None` otherwise.
    #[inline]
    fn from(value: Option<Address>) -> Self {
        match value {
            None => Self::Create,
            Some(addr) => Self::Call(addr),
        }
    }
}

impl From<Address> for TxKind {
    /// Creates a `TxKind::Call` with the given address.
    #[inline]
    fn from(value: Address) -> Self {
        Self::Call(value)
    }
}

impl TxKind {
    /// Returns the address of the contract that will be called or will receive the transfer.
    pub const fn to(&self) -> Option<&Address> {
        match self {
            Self::Create => None,
            Self::Call(to) => Some(to),
        }
    }

    /// Returns true if the transaction is a contract creation.
    #[inline]
    pub const fn is_create(&self) -> bool {
        matches!(self, Self::Create)
    }

    /// Returns true if the transaction is a contract call.
    #[inline]
    pub const fn is_call(&self) -> bool {
        matches!(self, Self::Call(_))
    }
}
