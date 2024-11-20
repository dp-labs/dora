use dora_primitives::spec::SpecId;
use num_bigint::BigUint;
pub use revmc::{op_info_map, OpcodeInfo};
use std::fmt;
use thiserror::Error;

/// An error that occurs when an invalid opcode is encountered during parsing.
/// This struct holds the invalid opcode (as a `u8`) and provides a formatted error message
/// indicating the invalid opcode in hexadecimal format.
///
/// # Example Usage:
/// ```no_check
/// let err = OpcodeParseError(0xFF);
/// println!("{}", err); // Output: "The opcode `FF` is not valid"
/// ```
///
/// # Notes:
/// - This error is triggered when the byte sequence does not match any valid opcode during
///   the parsing process, which is common when processing raw EVM bytecode.
#[derive(Error, Debug)]
#[error("The opcode `{:02X}` is not valid", self.0)]
pub struct OpcodeParseError(u8);

/// An error type that aggregates multiple `OpcodeParseError` instances. This is used when
/// multiple invalid opcodes are encountered during the parsing of bytecode.
///
/// # Fields:
/// - `0`: A vector of `OpcodeParseError` instances, each representing an individual invalid opcode.
///
/// # Example Usage:
/// ```no_check
/// let errors = vec![OpcodeParseError(0x01), OpcodeParseError(0xFF)];
/// let parse_error = ParseError(errors);
/// println!("{:?}", parse_error); // Output: ParseError([OpcodeParseError(0x01), OpcodeParseError(0xFF)])
/// ```
///
/// # Notes:
/// - This error type allows for handling cases where multiple parsing errors need to be reported,
///   such as when processing large sections of bytecode that contain multiple invalid opcodes.
#[derive(Error, Debug)]
pub struct ParseError(Vec<OpcodeParseError>);

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let opcodes: Vec<_> = self.0.iter().map(|x| format!("{:02X}", x.0)).collect();
        writeln!(f, "The following opcodes could not be parsed: ")?;
        writeln!(f, "{:#?}", opcodes)?;
        Ok(())
    }
}

macro_rules! opcodes {
    ($($name:ident = $value:expr),+ $(,)?) => {
        #[repr(u8)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum Opcode {
            $($name = $value),+
        }

        impl TryFrom<u8> for Opcode {
            type Error = OpcodeParseError;

            fn try_from(opcode: u8) -> Result<Self, Self::Error> {
                match opcode {
                    $(x if x == Opcode::$name as u8 => Ok(Opcode::$name),)+
                    _ => Err(OpcodeParseError(opcode))
                }
            }
        }
    };
}

opcodes! {
    STOP = 0x00,
    ADD = 0x01,
    MUL = 0x02,
    SUB = 0x03,
    DIV = 0x04,
    SDIV = 0x05,
    MOD = 0x06,
    SMOD = 0x07,
    ADDMOD = 0x08,
    MULMOD = 0x09,
    EXP = 0x0A,
    SIGNEXTEND = 0x0B,
    // unused 0x0C-0x0F
    LT = 0x10,
    GT = 0x11,
    SLT = 0x12,
    SGT = 0x13,
    EQ = 0x14,
    ISZERO = 0x15,
    AND = 0x16,
    OR = 0x17,
    XOR = 0x18,
    NOT = 0x19,
    BYTE = 0x1A,
    SHL = 0x1B,
    SHR = 0x1C,
    SAR = 0x1D,
    // unused 0x1E-0x1F
    KECCAK256 = 0x20,
    // unused 0x21-0x2F
    ADDRESS = 0x30,
    BALANCE = 0x31,
    ORIGIN = 0x32,
    CALLER = 0x33,
    CALLVALUE = 0x34,
    CALLDATALOAD = 0x35,
    CALLDATASIZE = 0x36,
    CALLDATACOPY = 0x37,
    CODESIZE = 0x38,
    CODECOPY = 0x39,
    GASPRICE = 0x3A,
    EXTCODESIZE = 0x3B,
    EXTCODECOPY = 0x3C,
    RETURNDATASIZE = 0x3D,
    RETURNDATACOPY = 0x3E,
    EXTCODEHASH = 0x3F,
    BLOCKHASH = 0x40,
    COINBASE = 0x41,
    TIMESTAMP = 0x42,
    NUMBER = 0x43,
    PREVRANDAO = 0x44,
    GASLIMIT = 0x45,
    CHAINID = 0x46,
    SELFBALANCE = 0x47,
    BASEFEE = 0x48,
    BLOBHASH = 0x49,
    BLOBBASEFEE = 0x4A,
    // unused 0x4B-0x4F
    POP = 0x50,
    MLOAD = 0x51,
    MSTORE = 0x52,
    MSTORE8 = 0x53,
    SLOAD = 0x54,
    SSTORE = 0x55,
    JUMP = 0x56,
    JUMPI = 0x57,
    PC = 0x58,
    MSIZE = 0x59,
    GAS = 0x5A,
    JUMPDEST = 0x5B,
    TLOAD = 0x5C,
    TSTORE = 0x5D,
    MCOPY = 0x5E,
    PUSH0 = 0x5F,
    PUSH1 = 0x60,
    PUSH2 = 0x61,
    PUSH3 = 0x62,
    PUSH4 = 0x63,
    PUSH5 = 0x64,
    PUSH6 = 0x65,
    PUSH7 = 0x66,
    PUSH8 = 0x67,
    PUSH9 = 0x68,
    PUSH10 = 0x69,
    PUSH11 = 0x6A,
    PUSH12 = 0x6B,
    PUSH13 = 0x6C,
    PUSH14 = 0x6D,
    PUSH15 = 0x6E,
    PUSH16 = 0x6F,
    PUSH17 = 0x70,
    PUSH18 = 0x71,
    PUSH19 = 0x72,
    PUSH20 = 0x73,
    PUSH21 = 0x74,
    PUSH22 = 0x75,
    PUSH23 = 0x76,
    PUSH24 = 0x77,
    PUSH25 = 0x78,
    PUSH26 = 0x79,
    PUSH27 = 0x7A,
    PUSH28 = 0x7B,
    PUSH29 = 0x7C,
    PUSH30 = 0x7D,
    PUSH31 = 0x7E,
    PUSH32 = 0x7F,
    DUP1 = 0x80,
    DUP2 = 0x81,
    DUP3 = 0x82,
    DUP4 = 0x83,
    DUP5 = 0x84,
    DUP6 = 0x85,
    DUP7 = 0x86,
    DUP8 = 0x87,
    DUP9 = 0x88,
    DUP10 = 0x89,
    DUP11 = 0x8A,
    DUP12 = 0x8B,
    DUP13 = 0x8C,
    DUP14 = 0x8D,
    DUP15 = 0x8E,
    DUP16 = 0x8F,
    SWAP1 = 0x90,
    SWAP2 = 0x91,
    SWAP3 = 0x92,
    SWAP4 = 0x93,
    SWAP5 = 0x94,
    SWAP6 = 0x95,
    SWAP7 = 0x96,
    SWAP8 = 0x97,
    SWAP9 = 0x98,
    SWAP10 = 0x99,
    SWAP11 = 0x9A,
    SWAP12 = 0x9B,
    SWAP13 = 0x9C,
    SWAP14 = 0x9D,
    SWAP15 = 0x9E,
    SWAP16 = 0x9F,
    LOG0 = 0xA0,
    LOG1 = 0xA1,
    LOG2 = 0xA2,
    LOG3 = 0xA3,
    LOG4 = 0xA4,
    // unused 0xA5-0xEF
    CREATE = 0xF0,
    CALL = 0xF1,
    CALLCODE = 0xF2,
    RETURN = 0xF3,
    DELEGATECALL = 0xF4,
    CREATE2 = 0xF5,
    // unused 0xF6-0xF9
    RETURNDATALOAD = 0xF7,
    STATICCALL = 0xFA,
    // unused 0xFB-0xFC
    REVERT = 0xFD,
    INVALID = 0xFE,
    SELFDESTRUCT = 0xFF,
}

macro_rules! operations {
    ($(($variant:ident, $opcode:ident)),* $(,)?) => {
        #[derive(Debug, Clone)]
        pub enum Operation {
            $(
                $variant,
            )*
            PC { pc: usize },
            Jumpdest { pc: usize },
            Push((u8, BigUint)),
            Dup(u8),
            Swap(u8),
            Log(u8),
        }

        impl Operation {
            pub fn to(&self) -> Vec<u8> {
                match self {
                    $(
                        Operation::$variant => vec![Opcode::$opcode as u8],
                    )*
                    Operation::PC { .. } => vec![Opcode::PC as u8],
                    Operation::Jumpdest { .. } => vec![Opcode::JUMPDEST as u8],
                    Operation::Push((n, x)) => {
                        if !(1..=32).contains(n) {
                            panic!("Invalid PUSH size: {}", n);
                        }
                        let len = 1 + *n as usize;
                        let mut opcode_bytes = vec![0; len];
                        opcode_bytes[0] = Opcode::PUSH0 as u8 + n;
                        let bytes = x.to_bytes_be();
                        opcode_bytes[len - bytes.len()..].copy_from_slice(&bytes);
                        opcode_bytes
                    }
                    Operation::Dup(n) => {
                        if !(1..=16).contains(n) {
                            panic!("Invalid DUP index: {}", n);
                        }
                        vec![Opcode::DUP1 as u8 + n - 1]
                    },
                    Operation::Swap(n) => {
                        if !(1..=16).contains(n) {
                            panic!("Invalid SWAP index: {}", n);
                        }
                        vec![Opcode::SWAP1 as u8 + n - 1]
                    },
                    Operation::Log(n) => {
                        if !(0..=4).contains(n) {
                            panic!("Invalid LOG index: {}", n);
                        }
                        vec![Opcode::LOG0 as u8 + n]
                    }
                }
            }

            pub fn from(opcode: Opcode, opcodes: &[u8], pc: &mut usize) -> Result<Self, OpcodeParseError> {
                let op = match opcode {
                    $(
                        Opcode::$opcode => Operation::$variant,
                    )*
                    opcode if Opcode::PUSH1 as u8 <= opcode as u8 && opcode as u8 <= Opcode::PUSH32 as u8 => {
                            *pc += 1;
                            let n = (opcode as u8 - Opcode::PUSH0 as u8) as usize;
                            let x = opcodes[*pc..std::cmp::min(opcodes.len(), *pc + n)]
                                .try_into()
                                .unwrap();
                            *pc += n - 1;
                            Operation::Push((n as u8, BigUint::from_bytes_be(x)))
                    }
                    opcode if Opcode::DUP1 as u8 <= opcode as u8 && opcode as u8 <= Opcode::DUP16 as u8 => {
                        let index = (opcode as u8 - Opcode::DUP1 as u8 + 1) as u8;
                        Operation::Dup(index)
                    }
                    opcode if Opcode::SWAP1 as u8 <= opcode as u8 && opcode as u8 <= Opcode::SWAP16 as u8 => {
                        let index = (opcode as u8 - Opcode::SWAP1 as u8 + 1) as u8;
                        Operation::Swap(index)
                    }
                    opcode if Opcode::LOG0 as u8 <= opcode as u8 && opcode as u8 <= Opcode::LOG4 as u8 => {
                        let index = (opcode as u8 - Opcode::LOG0 as u8) as u8;
                        Operation::Log(index)
                    }
                    Opcode::PC => Operation::PC { pc: *pc },
                    Opcode::JUMPDEST => Operation::Jumpdest { pc: *pc },
                    _ => return Err(OpcodeParseError(opcode as u8)),
                };
                Ok(op)
            }
        }
    };
}

operations!(
    (Stop, STOP),
    (Add, ADD),
    (Mul, MUL),
    (Sub, SUB),
    (Div, DIV),
    (Sdiv, SDIV),
    (Mod, MOD),
    (SMod, SMOD),
    (Addmod, ADDMOD),
    (Mulmod, MULMOD),
    (Exp, EXP),
    (SignExtend, SIGNEXTEND),
    (Lt, LT),
    (Gt, GT),
    (Slt, SLT),
    (Sgt, SGT),
    (Eq, EQ),
    (IsZero, ISZERO),
    (And, AND),
    (Or, OR),
    (Xor, XOR),
    (Not, NOT),
    (Byte, BYTE),
    (Shl, SHL),
    (Shr, SHR),
    (Sar, SAR),
    (Keccak256, KECCAK256),
    (Address, ADDRESS),
    (Balance, BALANCE),
    (Origin, ORIGIN),
    (Caller, CALLER),
    (Callvalue, CALLVALUE),
    (CalldataLoad, CALLDATALOAD),
    (CallDataSize, CALLDATASIZE),
    (CallDataCopy, CALLDATACOPY),
    (Codesize, CODESIZE),
    (Codecopy, CODECOPY),
    (Gasprice, GASPRICE),
    (ExtcodeCopy, EXTCODECOPY),
    (ReturnDataSize, RETURNDATASIZE),
    (ReturnDataCopy, RETURNDATACOPY),
    (ReturnDataLoad, RETURNDATALOAD),
    (ExtcodeHash, EXTCODEHASH),
    (BlockHash, BLOCKHASH),
    (ExtcodeSize, EXTCODESIZE),
    (Coinbase, COINBASE),
    (Timestamp, TIMESTAMP),
    (Number, NUMBER),
    (Prevrandao, PREVRANDAO),
    (Gaslimit, GASLIMIT),
    (Chainid, CHAINID),
    (SelfBalance, SELFBALANCE),
    (Basefee, BASEFEE),
    (BlobHash, BLOBHASH),
    (BlobBaseFee, BLOBBASEFEE),
    (Pop, POP),
    (Mload, MLOAD),
    (Mstore, MSTORE),
    (Mstore8, MSTORE8),
    (Sload, SLOAD),
    (Sstore, SSTORE),
    (Jump, JUMP),
    (Jumpi, JUMPI),
    (Msize, MSIZE),
    (Gas, GAS),
    (Tload, TLOAD),
    (Tstore, TSTORE),
    (Mcopy, MCOPY),
    (Push0, PUSH0),
    (Create, CREATE),
    (Call, CALL),
    (CallCode, CALLCODE),
    (Return, RETURN),
    (DelegateCall, DELEGATECALL),
    (Create2, CREATE2),
    (StaticCall, STATICCALL),
    (Revert, REVERT),
    (Invalid, INVALID),
    (SelfDestruct, SELFDESTRUCT),
);

pub struct CompileOptions {
    pub spec_id: SpecId,
}

/// Represents a program that has been parsed and is ready for execution. The `Program` struct
/// holds a list of operations and the total code size of the bytecode it represents.
///
/// # Fields:
/// - `operations`: A vector of `Operation` structs, each representing an individual parsed operation
///   from the bytecode.
/// - `code_size`: The size of the bytecode, typically measured in bytes, representing the total length
///   of the code that was parsed.
///
/// # Example Usage:
/// ```no_check
/// use dora_compiler::evm::program::{Operation, Program};
/// use num_bigint::BigUint;
///
/// let program = Program {
///     operations: vec![
///         Operation::Push((1_u8, BigUint::from(1_u8))),
///         Operation::Push((1_u8, BigUint::from(1_u8))),
///         Operation::Add,
///     ],
///     code_size: 5,
/// };
/// println!("Program has {} operations and a code size of {} bytes", program.operations.len(), program.code_size);
/// ```
///
/// # Notes:
/// - The `Program` struct is used as the core representation of a parsed and compiled EVM bytecode.
///   It encapsulates the operations that will be executed by the EVM, along with metadata such as
///   the total size of the bytecode.
#[derive(Debug, Clone)]
pub struct Program {
    /// A vector of operations parsed from the bytecode.
    pub operations: Vec<Operation>,

    /// The total size of the bytecode (in bytes).
    pub code_size: u32,
}

impl Program {
    /// Constructs a `Program` from a slice of opcodes, checking for errors during parsing.
    ///
    /// This method attempts to parse the provided opcodes into operations and calculates
    /// the total code size. If any opcodes fail to parse, a `ParseError` is returned containing
    /// the failed opcodes.
    ///
    /// # Parameters
    /// * `opcodes` - A slice of bytes representing the opcodes to be parsed.
    ///
    /// # Returns
    /// - `Ok(Self)` - If all opcodes are successfully parsed.
    /// - `Err(ParseError)` - If any opcodes fail to parse, containing the list of failed opcodes.
    pub fn from_opcode_checked(opcodes: &[u8]) -> Result<Self, ParseError> {
        let (operations, failed_opcodes) = Self::parse_operations(opcodes);
        let code_size = Self::calculate_code_size(&operations);

        if failed_opcodes.is_empty() {
            Ok(Self {
                operations,
                code_size,
            })
        } else {
            Err(ParseError(failed_opcodes))
        }
    }

    /// Constructs a `Program` from a slice of opcodes without error checking.
    ///
    /// This method parses the provided opcodes into operations and calculates the total
    /// code size. It does not check for any parsing errors.
    ///
    /// # Parameters
    /// * `opcodes` - A slice of bytes representing the opcodes to be parsed.
    ///
    /// # Returns
    /// A `Program` instance constructed from the parsed operations.
    pub fn from_opcode(opcodes: &[u8]) -> Self {
        let (operations, _) = Self::parse_operations(opcodes);
        let code_size = Self::calculate_code_size(&operations);

        Self {
            operations,
            code_size,
        }
    }

    /// Converts the `Program` into a vector of opcodes.
    ///
    /// This method serializes the operations in the program back into their corresponding
    /// byte representations, returning a vector of opcodes.
    ///
    /// # Returns
    /// A vector of bytes representing the opcodes of the program.
    pub fn to_opcode(&self) -> Vec<u8> {
        self.operations
            .iter()
            .flat_map(Operation::to)
            .collect::<Vec<u8>>()
    }

    fn parse_operations(opcodes: &[u8]) -> (Vec<Operation>, Vec<OpcodeParseError>) {
        let mut operations = vec![];
        let mut failed_opcodes = vec![];
        let mut pc = 0;

        while pc < opcodes.len() {
            match Self::parse_operation(opcodes, pc) {
                Ok((op, new_pc)) => {
                    operations.push(op);
                    pc = new_pc;
                }
                Err(e) => {
                    operations.push(Operation::Invalid);
                    failed_opcodes.push(e);
                    pc += 1;
                }
            }
        }

        (operations, failed_opcodes)
    }

    fn parse_operation(
        bytecode: &[u8],
        mut pc: usize,
    ) -> Result<(Operation, usize), OpcodeParseError> {
        let opcode = Opcode::try_from(bytecode[pc])?;
        let op = Operation::from(opcode, bytecode, &mut pc)?;
        pc += 1;
        Ok((op, pc))
    }

    fn calculate_code_size(operations: &[Operation]) -> u32 {
        operations
            .iter()
            .map(|op| match op {
                Operation::Push((size, _)) => (size + 1) as u32,
                _ => 1,
            })
            .sum()
    }
}

impl From<Vec<Operation>> for Program {
    fn from(operations: Vec<Operation>) -> Self {
        let code_size = Self::calculate_code_size(&operations);
        Self {
            operations,
            code_size,
        }
    }
}

/// Returns the number of input and output stack elements of the given opcode.
pub const fn stack_io(op: &Operation) -> (u8, u8) {
    match op {
        Operation::Stop => (0, 0),
        Operation::Add => (2, 1),
        Operation::Mul => (2, 1),
        Operation::Sub => (2, 1),
        Operation::Div => (2, 1),
        Operation::Sdiv => (2, 1),
        Operation::Mod => (2, 1),
        Operation::SMod => (2, 1),
        Operation::Addmod => (3, 1),
        Operation::Mulmod => (3, 1),
        Operation::Exp => (2, 1),
        Operation::SignExtend => (2, 1),
        Operation::Lt => (2, 1),
        Operation::Gt => (2, 1),
        Operation::Slt => (2, 1),
        Operation::Sgt => (2, 1),
        Operation::Eq => (2, 1),
        Operation::IsZero => (1, 1),
        Operation::And => (2, 1),
        Operation::Or => (2, 1),
        Operation::Xor => (2, 1),
        Operation::Not => (1, 1),
        Operation::Byte => (2, 1),
        Operation::Shl => (2, 1),
        Operation::Shr => (2, 1),
        Operation::Sar => (2, 1),
        Operation::Keccak256 => (2, 1),
        Operation::Address => (0, 1),
        Operation::Balance => (1, 1),
        Operation::Origin => (0, 1),
        Operation::Caller => (0, 1),
        Operation::Callvalue => (0, 1),
        Operation::CalldataLoad => (1, 1),
        Operation::CallDataSize => (0, 1),
        Operation::CallDataCopy => (3, 0),
        Operation::Codesize => (0, 1),
        Operation::Codecopy => (3, 0),
        Operation::Gasprice => (0, 1),
        Operation::ExtcodeCopy => (4, 0),
        Operation::ReturnDataSize => (0, 1),
        Operation::ReturnDataCopy => (3, 0),
        Operation::ReturnDataLoad => (1, 1),
        Operation::ExtcodeHash => (1, 1),
        Operation::BlockHash => (1, 1),
        Operation::ExtcodeSize => (1, 1),
        Operation::Coinbase => (0, 1),
        Operation::Timestamp => (0, 1),
        Operation::Number => (0, 1),
        Operation::Prevrandao => (0, 1),
        Operation::Gaslimit => (0, 1),
        Operation::Chainid => (0, 1),
        Operation::SelfBalance => (0, 1),
        Operation::Basefee => (0, 1),
        Operation::BlobHash => (1, 1),
        Operation::BlobBaseFee => (0, 1),
        Operation::Pop => (1, 0),
        Operation::Mload => (1, 1),
        Operation::Mstore => (2, 0),
        Operation::Mstore8 => (2, 0),
        Operation::Sload => (1, 1),
        Operation::Sstore => (2, 0),
        Operation::Jump => (1, 0),
        Operation::Jumpi => (2, 0),
        Operation::Msize => (0, 1),
        Operation::Gas => (0, 1),
        Operation::Tload => (1, 1),
        Operation::Tstore => (2, 0),
        Operation::Mcopy => (3, 0),
        Operation::Push0 => (0, 1),
        Operation::Create => (3, 1),
        Operation::Call => (7, 1),
        Operation::CallCode => (7, 1),
        Operation::Return => (2, 0),
        Operation::DelegateCall => (6, 1),
        Operation::Create2 => (4, 1),
        Operation::StaticCall => (6, 1),
        Operation::Revert => (2, 0),
        Operation::Invalid => (0, 0),
        Operation::SelfDestruct => (1, 0),
        Operation::PC { .. } => (0, 1),
        Operation::Jumpdest { .. } => (0, 0),
        Operation::Push(_) => (0, 1),
        Operation::Dup(_) => (0, 1),
        Operation::Swap(n) => (*n + 1, *n + 1),
        Operation::Log(n) => (*n + 2, 0),
    }
}
