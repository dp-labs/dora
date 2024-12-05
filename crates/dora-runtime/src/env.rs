use std::cmp::min;

use crate::transaction::TransactionType;

use super::constants::{
    gas_cost::{
        init_code_cost, MAX_CODE_SIZE, TX_ACCESS_LIST_ADDRESS_COST,
        TX_ACCESS_LIST_STORAGE_KEY_COST, TX_BASE_COST, TX_CREATE_COST, TX_DATA_COST_PER_NON_ZERO,
        TX_DATA_COST_PER_ZERO,
    },
    MAX_BLOB_NUMBER_PER_BLOCK, VERSIONED_HASH_VERSION_KZG,
};
use super::result::InvalidTransaction;
use dora_primitives::{Bytes, EVMAddress as Address, B256, U256};

/// Represents the execution environment for the EVM, including block, transaction, and EVM configuration.
///
/// The `Env` struct contains the environment in which the Ethereum Virtual Machine (EVM) operates. It holds
/// configuration settings specific to the EVM, the current block being processed, and the transaction being executed.
///
/// # Fields:
/// - `cfg`: EVM configuration, including chain ID.
/// - `block`: Information about the current block, such as block number, timestamp, and coinbase.
/// - `tx`: Details about the current transaction, including caller, gas limit, and transaction type.
///
/// # Example Usage:
/// ```no_check
/// let env = Env::default();
/// ```
#[derive(Clone, Debug, Default)]
pub struct Env {
    /// EVM configuration.
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
        let is_create = self.tx.transact_to.is_zero();

        if is_create && self.tx.data.len() > 2 * MAX_CODE_SIZE {
            return Err(InvalidTransaction::CreateInitCodeSizeLimit);
        }

        if let Some(max_fee) = self.tx.max_fee_per_blob_gas {
            let price = self.block.blob_gasprice.unwrap_or_default();
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

    /// Calculates the intrinsic cost based on the transaction data.
    fn calculate_intrinsic_cost(&self) -> u64 {
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

        let create_cost = if !self.tx.transact_to.is_zero() {
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
}

/// Configuration settings for the EVM, including chain ID.
///
/// The `CfgEnv` struct holds the configuration details specific to the EVM instance, such as the chain ID.
/// It allows the EVM to distinguish between different blockchain networks and environments.
///
/// # Fields:
/// - `chain_id`: The unique identifier for the blockchain network (e.g., 1 for Ethereum mainnet).
///
/// # Example Usage:
/// ```no_check
/// let cfg_env = CfgEnv { chain_id: 1 };
/// ```
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct CfgEnv {
    /// Chain ID of the EVM, it will be compared to the transaction's Chain ID.
    /// Chain ID is introduced EIP-155
    pub chain_id: u64,
    /// If some it will effects EIP-170: Contract code size limit. Useful to increase this because of tests.
    /// By default it is 0x6000 (~25kb).
    pub limit_contract_code_size: Option<usize>,
    /// Skips the nonce validation against the account's nonce.
    pub disable_nonce_check: bool,
    /// A hard memory limit in bytes beyond which [crate::result::OutOfGasError::Memory] cannot be resized.
    /// Defaults to `2^32 - 1` bytes per EIP-1985.
    pub memory_limit: u64,
}

impl Default for CfgEnv {
    fn default() -> Self {
        Self {
            chain_id: 1,
            limit_contract_code_size: None,
            disable_nonce_check: false,
            memory_limit: (1 << 32) - 1,
        }
    }
}

/// Information about the current block being processed in the EVM, including block number and timestamp.
///
/// The `BlockEnv` struct holds various parameters related to the current block being executed in the EVM.
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
    /// Coinbase or miner or address that created and signed the block.
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
    /// The excess blob gas of the block.
    ///
    /// [EIP-4844](https://eips.ethereum.org/EIPS/eip-4844)
    pub excess_blob_gas: Option<u64>,
    /// The calculated blob gas price based on the `excess_blob_gas`.
    ///
    /// [EIP-4844](https://eips.ethereum.org/EIPS/eip-4844)
    pub blob_gasprice: Option<u128>,
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
    pub transact_to: Address,
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
}

pub type AccessListItem = (Address, Vec<B256>);

impl Default for TxEnv {
    fn default() -> Self {
        Self {
            caller: Address::zero(),
            gas_limit: i64::MAX as _,
            gas_price: U256::ZERO,
            transact_to: Address::zero(),
            value: U256::ZERO,
            data: Bytes::new(),
            nonce: 0,
            access_list: Vec::new(),
            blob_hashes: Vec::new(),
            tx_type: TransactionType::Legacy,
            chain_id: Some(1),
            gas_priority_fee: None,
            max_fee_per_blob_gas: None,
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
        self.transact_to
    }

    #[inline]
    pub fn effective_gas_price(&self, base_fee: U256) -> U256 {
        let (max_fee, max_priority_fee) = match self.tx_type {
            TransactionType::Legacy => return U256::from(self.gas_price),
            TransactionType::Eip2930 => return U256::from(self.gas_price),
            TransactionType::Eip1559 => (self.gas_price, self.gas_priority_fee.unwrap_or_default()),
            TransactionType::Eip4844 => (self.gas_price, self.gas_priority_fee.unwrap_or_default()),
            TransactionType::Eip7702 => (self.gas_price, self.gas_priority_fee.unwrap_or_default()),
            TransactionType::Custom => unimplemented!("Custom tx not supported"),
        };

        min(max_fee, base_fee + max_priority_fee)
    }
}
