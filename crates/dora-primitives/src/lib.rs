use std::sync::Arc;

pub use alloy_primitives::{PrimitiveSignature, SignatureError, Signed};
pub use revm::precompile::{
    Precompile, PrecompileErrors, PrecompileOutput, PrecompileSpecId, Precompiles,
};
pub use revm::primitives::{
    address, alloy_primitives, b256, calc_blob_gasprice, calc_excess_blob_gas, eip7702,
    eof::{Eof, EofBody, TypesSection},
    fixed_bytes,
    hex::{FromHex, ToHexExt},
    keccak256, uint, Address, Authorization, AuthorizationList, Bytecode as EVMBytecode, Bytes,
    EvmStorageSlot, FixedBytes, Log, LogData, RecoveredAuthority, RecoveredAuthorization,
    SignedAuthorization, SpecId, B256, EOF_MAGIC_BYTES, GAS_PER_BLOB, I256, KECCAK_EMPTY, U256,
};

pub mod config;
pub mod spec;

/// WASM magic number `\0asm` in array form.
pub const WASM_MAGIC_BYTES: [u8; 4] = [0x00, 0x61, 0x73, 0x6D];

/// Universal bytecode definition for EVM, WASM, etc.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Bytecode {
    EVM(EVMBytecode),
    WASM(Bytes),
}

impl Bytecode {
    /// New a [Bytecode] according to the magic start bytes
    pub fn new(bytecode: Bytes) -> Self {
        if bytecode.starts_with(&WASM_MAGIC_BYTES) {
            Bytecode::WASM(bytecode)
        } else {
            Bytecode::EVM(EVMBytecode::new_raw(bytecode))
        }
    }

    /// New default EVM [Bytecode].
    #[inline]
    pub fn new_evm_default() -> Self {
        Bytecode::EVM(EVMBytecode::default())
    }

    /// New default EVM [Bytecode].
    #[inline]
    pub fn new_wasm_default() -> Self {
        Bytecode::WASM(Bytes::default())
    }

    /// Creates an empty EVM [Bytecode].
    #[inline]
    pub fn empty() -> Self {
        Self::new(Bytes::default())
    }

    /// Returns a reference to the bytecode.
    ///
    /// In case of EOF EVM bytecode, this will be the first code section.
    #[inline]
    pub fn bytecode(&self) -> &Bytes {
        match self {
            Bytecode::EVM(bytecode) => bytecode.bytecode(),
            Bytecode::WASM(bytes) => bytes,
        }
    }

    /// Returns the cloned bytes.
    #[inline]
    pub fn bytes(&self) -> Bytes {
        match self {
            Bytecode::EVM(bytecode) => bytecode.bytes(),
            Bytecode::WASM(bytes) => bytes.clone(),
        }
    }

    /// Returns reference to the EOF if bytecode is EOF.
    #[inline]
    pub fn eof(&self) -> Option<&Arc<Eof>> {
        match self {
            Bytecode::EVM(bytecode) => bytecode.eof(),
            Bytecode::WASM(_) => None,
        }
    }

    /// Returns true if bytecode is EOF.
    #[inline]
    pub const fn is_eof(&self) -> bool {
        match self {
            Bytecode::EVM(bytecode) => bytecode.is_eof(),
            Bytecode::WASM(_) => false,
        }
    }

    /// Returns true if bytecode is EIP-7702.
    #[inline]
    pub const fn is_eip7702(&self) -> bool {
        match self {
            Bytecode::EVM(bytecode) => bytecode.is_eip7702(),
            Bytecode::WASM(_) => false,
        }
    }

    /// Returns the original bytecode as a byte slice.
    #[inline]
    pub fn original_byte_slice(&self) -> &[u8] {
        match self {
            Bytecode::EVM(bytecode) => bytecode.original_byte_slice(),
            Bytecode::WASM(bytes) => bytes,
        }
    }

    /// Returns the length of the original bytes.
    #[inline]
    pub fn len(&self) -> usize {
        self.original_byte_slice().len()
    }

    /// Returns whether the bytecode is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Calculate hash of the bytecode.
    pub fn hash_slow(&self) -> B256 {
        if self.is_empty() {
            KECCAK_EMPTY
        } else {
            keccak256(self.original_byte_slice())
        }
    }

    /// Whether is EVM bytecode.
    #[inline]
    pub fn is_evm(&self) -> bool {
        matches!(self, Self::EVM(_))
    }

    /// Whether is WASM bytecode.
    #[inline]
    pub fn is_wasm(&self) -> bool {
        matches!(self, Self::WASM(_))
    }
}

impl Default for Bytecode {
    fn default() -> Self {
        Self::empty()
    }
}

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
        buffer[12..32].copy_from_slice(&value.0 .0);
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
