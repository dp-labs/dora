use super::constants::{
    gas_cost::{
        init_code_cost, MAX_CODE_SIZE, TX_ACCESS_LIST_ADDRESS_COST,
        TX_ACCESS_LIST_STORAGE_KEY_COST, TX_BASE_COST, TX_CREATE_COST, TX_DATA_COST_PER_NON_ZERO,
        TX_DATA_COST_PER_ZERO,
    },
    MAX_BLOB_NUMBER_PER_BLOCK, VERSIONED_HASH_VERSION_KZG,
};
use super::context::U256 as DU256;
use super::result::InvalidTransaction;
use dora_primitives::{Bytes, EVMAddress as Address, B256};
use ruint::aliases::U256;

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
    pub fn validate_transaction(&mut self) -> Result<(), InvalidTransaction> {
        let is_create = matches!(self.tx.transact_to, TransactTo::Create);

        if is_create && self.tx.data.len() > 2 * MAX_CODE_SIZE {
            return Err(InvalidTransaction::CreateInitCodeSizeLimit);
        }

        if let Some(max_fee) = self.tx.max_fee_per_blob_gas {
            let price = self.block.blob_gasprice.unwrap();
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

        let create_cost = match self.tx.transact_to {
            TransactTo::Call(_) => 0,
            TransactTo::Create => TX_CREATE_COST + init_code_cost(self.tx.data.len()),
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
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct CfgEnv {
    pub chain_id: u64,
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
    pub number: U256,
    pub coinbase: Address,
    pub timestamp: U256,
    pub basefee: U256,
    pub prevrandao: Option<B256>,
    pub excess_blob_gas: Option<u64>,
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
    pub caller: Address,
    pub gas_limit: u64,
    pub gas_price: U256,
    pub transact_to: TransactTo,
    pub value: U256,
    pub data: Bytes,
    pub access_list: Vec<(Address, Vec<U256>)>,
    pub blob_hashes: Vec<B256>,
    pub max_fee_per_blob_gas: Option<U256>,
}

impl Default for TxEnv {
    fn default() -> Self {
        Self {
            caller: Address::zero(),
            gas_limit: i64::MAX as _,
            gas_price: DU256::ZERO.to_u256(),
            transact_to: TransactTo::Call(Address::zero()),
            value: DU256::ZERO.to_u256(),
            data: Bytes::new(),
            access_list: Vec::new(),
            blob_hashes: Vec::new(),
            max_fee_per_blob_gas: None,
        }
    }
}

/// Defines the destination of a transaction, either a contract call or a contract creation.
///
/// The `TransactTo` enum represents the possible destinations of a transaction. It can either be
/// a contract call with a specific address, or a contract creation where no address is specified.
///
/// # Variants:
/// - `Call(Address)`: Indicates a transaction to call a contract at a specific address.
/// - `Create`: Indicates a transaction to create a new contract.
///
/// # Example Usage:
/// ```no_check
/// let tx_to_call = TransactTo::Call(Address::from_low_u64_be(0x123));
/// let tx_to_create = TransactTo::Create;
/// ```
#[derive(Clone, Debug)]
pub enum TransactTo {
    Call(Address),
    Create,
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
    pub fn get_address(&self) -> Address {
        match self.transact_to {
            TransactTo::Call(addr) => addr,
            TransactTo::Create => Address::zero(),
        }
    }
}
