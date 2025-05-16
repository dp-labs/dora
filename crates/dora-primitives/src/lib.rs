use std::cmp::min;

pub use alloy_primitives::{
    Signature, SignatureError, Signed, Uint,
    map::{Entry, HashMap, HashSet},
};
pub use indexmap::{IndexMap, IndexSet, map::Entry as IndexMapEntry};
pub use revm::bytecode::{
    Bytecode as EVMBytecode,
    eip7702::Eip7702Bytecode,
    eof::{CodeInfo as EofCodeInfo, EOF_MAGIC_BYTES, EOF_MAGIC_HASH, Eof, EofBody},
    opcode::{OpCode, OpCodeInfo},
};
use revm::context::Block;
pub use revm::context::{
    BlockEnv, CfgEnv, Journal, JournalEntry, JournalEntryTr, JournalOutput, JournalTr, TxEnv,
};
pub use revm::context_interface::{
    block::{BlobExcessGasAndPrice, calc_blob_gasprice, calc_excess_blob_gas},
    cfg::Cfg,
    context::{SStoreResult, SelfDestructResult},
    journaled_state::{AccountLoad, JournalCheckpoint, StateLoad, TransferError},
    result::{
        ExecutionResult, HaltReason, InvalidHeader, InvalidTransaction, OutOfGasError, Output,
        ResultAndState, SuccessReason,
    },
    transaction::{
        AccessList, AccessListItem, Authorization, AuthorizationTr, RecoveredAuthority,
        RecoveredAuthorization, SignedAuthorization, TransactionType,
    },
};
pub use revm::database::{DBErrorMarker, Database, DatabaseCommit, DatabaseRef};
pub use revm::precompile::{PrecompileError, PrecompileOutput, PrecompileSpecId, Precompiles};
pub use revm::primitives::{
    Address, B256, BLOCK_HASH_HISTORY, Bytes, FixedBytes, I256, KECCAK_EMPTY, Log, LogData, TxKind,
    U256, address, alloy_primitives, b256,
    constants::MAX_INITCODE_SIZE,
    eip4844::{self, GAS_PER_BLOB},
    eip7702::{self, PER_AUTH_BASE_COST, PER_EMPTY_ACCOUNT_COST},
    fixed_bytes,
    hex::{FromHex, ToHexExt},
    keccak256, uint,
};
pub use revm::state::{
    Account, AccountInfo, AccountStatus, EvmState, EvmStorageSlot as StorageSlot,
};

pub mod config;
pub mod spec;

pub use config::OptimizationLevel;
pub use spec::{SpecId, SpecName};

/// Converts a [U256] value to a [u64], saturating to [MAX][u64] if the value is too large.
#[macro_export]
macro_rules! as_u64_saturated {
    ($v:expr) => {
        match $v.as_limbs() {
            x => {
                if (x[1] == 0) & (x[2] == 0) & (x[3] == 0) {
                    x[0]
                } else {
                    u64::MAX
                }
            }
        }
    };
}

/// Converts a [U256] value to a [usize], saturating to [MAX][usize] if the value is too large.
#[macro_export]
macro_rules! as_usize_saturated {
    ($v:expr) => {
        usize::try_from($crate::as_u64_saturated!($v)).unwrap_or(usize::MAX)
    };
}

/// Converts a [U256] value to a [isize], saturating to [MAX][isize] if the value is too large.
#[macro_export]
macro_rules! as_isize_saturated {
    ($v:expr) => {
        isize::try_from($crate::as_u64_saturated!($v)).unwrap_or(isize::MAX)
    };
}

/// `const` Option `?`.
#[macro_export]
macro_rules! tri {
    ($e:expr) => {
        match $e {
            Some(v) => v,
            None => return None,
        }
    };
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Env {
    pub block: BlockEnv,
    pub tx: TxEnv,
    pub cfg: CfgEnv,
}

impl Env {
    /// Gas price for the transaction.
    /// It is only applicable for Legacy and EIP-2930 transactions.
    /// For Eip1559 it is max_fee_per_gas.
    #[inline]
    pub fn gas_price(&self) -> u128 {
        self.tx.gas_price
    }

    /// Returns maximum fee that can be paid for the transaction.
    pub fn max_fee_per_gas(&self) -> u128 {
        self.gas_price()
    }

    /// Maximum priority fee per gas.
    #[inline]
    pub fn max_priority_fee_per_gas(&self) -> Option<u128> {
        self.tx.gas_priority_fee
    }

    /// Returns vector of fixed size hash(32 bytes)
    ///
    /// Note : EIP-4844 transaction field.
    #[inline]
    pub fn blob_versioned_hashes(&self) -> &[B256] {
        self.tx.blob_hashes.as_slice()
    }

    /// Max fee per data gas
    ///
    /// Note : EIP-4844 transaction field.
    #[inline]
    pub fn max_fee_per_blob_gas(&self) -> u128 {
        self.tx.max_fee_per_blob_gas
    }

    /// See [EIP-4844] and [`calc_blob_gasprice`].
    ///
    /// Returns `None` if `Cancun` is not enabled.
    ///
    /// [EIP-4844]: https://eips.ethereum.org/EIPS/eip-4844
    #[inline]
    pub fn blob_gasprice(&self) -> Option<u128> {
        self.block
            .blob_excess_gas_and_price
            .map(|a| a.blob_gasprice)
    }

    /// Total gas for all blobs. Max number of blocks is already checked
    /// so we dont need to check for overflow.
    #[inline]
    pub fn total_blob_gas(&self) -> u64 {
        GAS_PER_BLOB * self.blob_versioned_hashes().len() as u64
    }

    /// Calculates the maximum [EIP-4844] `data_fee` of the transaction.
    ///
    /// This is used for ensuring that the user has at least enough funds to pay the
    /// `max_fee_per_blob_gas * total_blob_gas`, on top of regular gas costs.
    ///
    /// See EIP-4844:
    /// <https://github.com/ethereum/EIPs/blob/master/EIPS/eip-4844.md#execution-layer-validation>
    pub fn calc_max_data_fee(&self) -> U256 {
        let blob_gas = U256::from(self.total_blob_gas());
        let max_blob_fee = U256::from(self.max_fee_per_blob_gas());
        max_blob_fee.saturating_mul(blob_gas)
    }

    /// Returns effective gas price is gas price field for Legacy and Eip2930 transaction.
    ///
    /// While for transactions after Eip1559 it is minimum of max_fee and `base + max_priority_fee`.
    pub fn effective_gas_price(&self) -> u128 {
        if self.tx.tx_type == TransactionType::Legacy as u8
            || self.tx.tx_type == TransactionType::Eip2930 as u8
        {
            return self.gas_price();
        }
        let base_fee = self.block.basefee() as u128;
        let max_fee = self.gas_price();
        let Some(max_priority_fee) = self.max_priority_fee_per_gas() else {
            return max_fee;
        };
        min(max_fee, base_fee.saturating_add(max_priority_fee))
    }
}

/// WASM bytecode is a normal bytes that start with the magic bytes `\0asm`.
pub type WASMBytecode = Bytes;

/// WASM magic number `\0asm` in array form.
pub const WASM_MAGIC_BYTES: [u8; 4] = [0x00, 0x61, 0x73, 0x6D];

pub trait IsWASMBytecode {
    /// Returns true if the bytecode is WASM.
    fn is_wasm(&self) -> bool;
}

pub trait EmptyBytecode {
    /// Returns true if the bytecode is empty.
    fn empty() -> Self;
}

impl IsWASMBytecode for Bytes {
    fn is_wasm(&self) -> bool {
        self.len() >= 4 && self[0..4] == WASM_MAGIC_BYTES
    }
}

impl IsWASMBytecode for &[u8] {
    fn is_wasm(&self) -> bool {
        self.len() >= 4 && self[0..4] == WASM_MAGIC_BYTES
    }
}

impl IsWASMBytecode for EVMBytecode {
    fn is_wasm(&self) -> bool {
        self.original_byte_slice().is_wasm()
    }
}

impl EmptyBytecode for EVMBytecode {
    fn empty() -> Self {
        EVMBytecode::new_raw(Default::default())
    }
}

/// Universal bytecode definition for EVM, WASM, etc.
pub type Bytecode = EVMBytecode;

/// A fixed size array of 32 bytes for storing 256-bit EVM values.
///
/// `U256` provides methods for conversion between different byte orders and offers several
/// trait implementations for efficient conversions to and from other integer types. It is
/// aligned to 8 bytes and represented as an array of 32 bytes.
///
/// # Examples
///
/// ```no_check
/// let num = Bytes32::ZERO;
/// let be_bytes = num.to_be_bytes();
/// let from_bytes = Bytes32::from_be_bytes(be_bytes);
/// assert_eq!(num, from_bytes);
/// ```
///
/// # Fields
///
/// - `ZERO`: Constant representing the zero value for `Bytes32`.
///
/// # Methods
///
/// - `from_ne_bytes`: Creates a `Bytes32` from native-endian bytes.
/// - `from_be_bytes`: Creates a `Bytes32` from big-endian bytes.
/// - `from_le_bytes`: Creates a `Bytes32` from little-endian bytes.
/// - `to_ne_bytes`: Converts the integer to a byte array in native byte order.
/// - `to_be_bytes`: Converts the integer to a byte array in big-endian byte order.
/// - `to_le_bytes`: Converts the integer to a byte array in little-endian byte order.
///
/// # Trait Implementations
///
/// Implements `Clone`, `Copy`, `Debug`, `Default`, `Eq`, `Hash`, `Ord`, `PartialEq`, `PartialOrd`.
///
/// Includes conversion implementations from various integer types (`bool`, `u8`, `u16`, `u32`,
/// `u64`, `usize`, `u128`) through `impl_conversions!` macro, and allows conversion
/// to and from an external 256-bit type, `U256`.
///
/// # Safety
///
/// Some methods (e.g., `from_u256` on little-endian platforms) rely on `unsafe` transmutation
/// for efficient internal representation and conversion.
#[repr(C, align(8))]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Bytes32([u8; 32]);

macro_rules! impl_conversions {
    ($($ty:ty),*) => {
        $(
            impl From<$ty> for Bytes32 {
                #[inline]
                fn from(value: $ty) -> Self {
                    Self::from_u256(U256::from(value))
                }
            }

            impl From<&$ty> for Bytes32 {
                #[inline]
                fn from(value: &$ty) -> Self {
                    Self::from(*value)
                }
            }

            impl From<&mut $ty> for Bytes32 {
                #[inline]
                fn from(value: &mut $ty) -> Self {
                    Self::from(*value)
                }
            }

            impl TryFrom<Bytes32> for $ty {
                type Error = ();

                #[inline]
                fn try_from(value: Bytes32) -> Result<Self, Self::Error> {
                    value.to_u256().try_into().map_err(drop)
                }
            }

            impl TryFrom<&Bytes32> for $ty {
                type Error = ();

                #[inline]
                fn try_from(value: &Bytes32) -> Result<Self, Self::Error> {
                    (*value).try_into()
                }
            }

            impl TryFrom<&mut Bytes32> for $ty {
                type Error = ();

                #[inline]
                fn try_from(value: &mut Bytes32) -> Result<Self, Self::Error> {
                    (*value).try_into()
                }
            }
        )*
    };
}

impl_conversions!(bool, u8, u16, u32, u64, usize, u128);

impl From<U256> for Bytes32 {
    #[inline]
    fn from(value: U256) -> Self {
        Self::from_u256(value)
    }
}

impl From<&U256> for Bytes32 {
    #[inline]
    fn from(value: &U256) -> Self {
        Self::from(*value)
    }
}

impl From<&mut U256> for Bytes32 {
    #[inline]
    fn from(value: &mut U256) -> Self {
        Self::from(*value)
    }
}

impl From<B256> for Bytes32 {
    #[inline]
    fn from(value: B256) -> Self {
        Self::from_be_bytes(value.0)
    }
}

impl Bytes32 {
    /// The zero value.
    pub const ZERO: Self = Self([0; 32]);

    /// Creates a new value from native-endian bytes.
    #[inline]
    pub const fn from_ne_bytes(x: [u8; 32]) -> Self {
        Self(x)
    }

    /// Creates a new value from big-endian bytes.
    #[inline]
    pub fn from_be_bytes(x: [u8; 32]) -> Self {
        Self::from_be(Self(x))
    }

    /// Creates a new value from little-endian bytes.
    #[inline]
    pub fn from_le_bytes(x: [u8; 32]) -> Self {
        Self::from_le(Self(x))
    }

    /// Converts an integer from big endian to the target's endianness.
    #[inline]
    pub fn from_be(x: Self) -> Self {
        #[cfg(target_endian = "little")]
        return x.swap_bytes();
        #[cfg(target_endian = "big")]
        return x;
    }

    /// Converts an integer from little endian to the target's endianness.
    #[inline]
    pub fn from_le(x: Self) -> Self {
        #[cfg(target_endian = "little")]
        return x;
        #[cfg(target_endian = "big")]
        return x.swap_bytes();
    }

    /// Converts a [`U256`].
    #[inline]
    pub const fn from_u256(value: U256) -> Self {
        #[cfg(target_endian = "little")]
        return unsafe { core::mem::transmute::<U256, Self>(value) };
        #[cfg(target_endian = "big")]
        return Self(value.to_be_bytes());
    }

    /// Converts a [`U256`] reference to a [`U256`].
    #[inline]
    #[cfg(target_endian = "little")]
    pub const fn from_u256_ref(value: &U256) -> &Self {
        unsafe { &*(value as *const U256 as *const Self) }
    }

    /// Converts a [`U256`] mutable reference to a [`U256`].
    #[inline]
    #[cfg(target_endian = "little")]
    pub fn from_u256_mut(value: &mut U256) -> &mut Self {
        unsafe { &mut *(value as *mut U256 as *mut Self) }
    }

    /// Return the memory representation of this integer as a byte array in big-endian (network)
    /// byte order.
    #[inline]
    pub fn to_be_bytes(self) -> [u8; 32] {
        self.to_be().to_ne_bytes()
    }

    /// Return the memory representation of this integer as a byte array in little-endian byte
    /// order.
    #[inline]
    pub fn to_le_bytes(self) -> [u8; 32] {
        self.to_le().to_ne_bytes()
    }

    /// Return the memory representation of this integer as a byte array in native byte order.
    #[inline]
    pub const fn to_ne_bytes(self) -> [u8; 32] {
        self.0
    }

    /// Converts `self` to big endian from the target's endianness.
    #[inline]
    pub fn to_be(self) -> Self {
        #[cfg(target_endian = "little")]
        return self.swap_bytes();
        #[cfg(target_endian = "big")]
        return self;
    }

    /// Converts `self` to little endian from the target's endianness.
    #[inline]
    pub fn to_le(self) -> Self {
        #[cfg(target_endian = "little")]
        return self;
        #[cfg(target_endian = "big")]
        return self.swap_bytes();
    }

    /// Reverses the byte order of the integer.
    #[inline]
    pub fn swap_bytes(mut self) -> Self {
        self.0.reverse();
        self
    }

    /// Creates a new value from native-endian bytes.
    #[inline]
    pub const fn slice(&self) -> &[u8] {
        &self.0
    }

    /// Casts this value to a [`U256`]. This is a no-op on little-endian systems.
    #[cfg(target_endian = "little")]
    #[inline]
    pub const fn as_u256(&self) -> &U256 {
        unsafe { &*(self as *const Self as *const U256) }
    }

    /// Casts this value to a [`U256`]. This is a no-op on little-endian systems.
    #[cfg(target_endian = "little")]
    #[inline]
    pub fn as_u256_mut(&mut self) -> &mut U256 {
        unsafe { &mut *(self as *mut Self as *mut U256) }
    }

    /// Converts this value to a [`U256`]. This is a simple copy on little-endian systems.
    #[inline]
    pub const fn to_u256(&self) -> U256 {
        #[cfg(target_endian = "little")]
        return *self.as_u256();
        #[cfg(target_endian = "big")]
        return Bytes32::from_be_bytes(self.0);
    }

    /// Converts this value to a [`B256`]. This is a simple copy on little-endian systems.
    #[inline]
    pub fn to_b256(&self) -> B256 {
        B256::from_slice(&self.to_be_bytes())
    }

    /// Converts this value to a [`U256`]. This is a no-op on little-endian systems.
    #[inline]
    pub const fn into_u256(self) -> U256 {
        #[cfg(target_endian = "little")]
        return unsafe { core::mem::transmute::<Self, U256>(self) };
        #[cfg(target_endian = "big")]
        return Bytes32::from_be_bytes(self.0);
    }

    /// Copy an address into the bytes32.
    #[inline]
    pub fn copy_from(&mut self, value: &Address) {
        let mut buffer = [0u8; 32];
        buffer[12..32].copy_from_slice(&value.0.0);
        *self = Bytes32::from_be_bytes(buffer);
    }

    /// Converts this byte32 value to an [`Address`]  with 20 bytes.
    #[inline]
    pub fn to_address(self) -> Address {
        Address::from_slice(&self.to_be_bytes()[12..])
    }
}

impl Bytes32 {
    /// Checks if this `U256` value represents a valid 20-byte Ethereum address.
    ///
    /// A valid Ethereum address is stored in the lower 20 bytes of this `U256` value,
    /// meaning that the higher 12 bytes of this `U256` must be zero.
    ///
    /// # Returns
    ///
    /// `true` if the high 12 bytes are zero, indicating a valid Ethereum address.
    /// `false` otherwise.
    pub fn is_valid_eth_address(&self) -> bool {
        let bytes = self.to_be_bytes();
        bytes[0..12] == [0u8; 12]
    }
}

impl From<&Bytes32> for Address {
    fn from(value: &Bytes32) -> Self {
        // Create an address from the last 20 bytes of the 256-bit U256.
        let bytes = value.to_be_bytes();
        Address::from_slice(&bytes[12..])
    }
}
