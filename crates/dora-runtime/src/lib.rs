pub mod account;
pub mod artifact;
pub mod call;
pub mod constants;
pub mod context;
pub mod db;
pub mod executor;
pub mod gas;
pub mod handler;
pub mod host;
pub mod journaled_state;
pub mod result;
pub mod stack;
pub mod symbols;
pub mod vm;
pub mod wasm;

pub use account::{Account, AccountInfo, AccountStatus};
pub use artifact::{Artifact, SymbolArtifact};
pub use call::{CallKind, CallMessage, CallResult, CallType, CallTypeParseError, ExtCallType};
pub use context::{Contract, RuntimeContext, VMContext};
pub use db::{Database, DatabaseCommit, MemoryDB};
pub use executor::{ExecuteKind, ExecutionEngine, Executor, RUNTIME_STACK_SIZE};
pub use host::{DummyHost, Host};
pub use result::{ExecutionResult, HaltReason, ResultAndState, VMError};
pub use stack::Stack;
pub use vm::VM;

#[repr(u8)]
#[derive(Debug, Clone)]
pub enum ExitStatusCode {
    Continue = 0,
    /* Success Codes */
    /// Encountered a `RETURN` opcode
    Return = 1,
    /// Encountered a `STOP` opcode
    Stop = 2,
    /// Self-destruct the current contract.
    Selfdestruct = 3,

    /* Revert Codes */
    /// Revert the transaction.
    Revert = 0x10,
    /// Exceeded maximum call depth.
    CallTooDeep,
    /// Insufficient funds for transfer.
    OutOfFunds,
    /// Revert if `CREATE`/`CREATE2` starts with `0xEF00`.
    CreateInitCodeStartingEF00,
    /// Invalid EVM Object Format (EOF) init code.
    InvalidEOFInitCode,
    /// `ExtDelegatecall` calling a non EOF contract.
    InvalidExtDelegatecallTarget,

    /* Error Codes */
    /// Out of gas error.
    OutOfGas = 0x50,
    /// Out of gas error encountered during memory expansion.
    MemoryOOG,
    /// The memory limit of the VM has been exceeded.
    MemoryLimitOOG,
    /// Out of gas error encountered during the execution of a precompiled contract.
    PrecompileOOG,
    /// Out of gas error encountered while calling an invalid operand.
    InvalidOperandOOG,
    /// Unknown or invalid opcode.
    OpcodeNotFound,
    /// Invalid `CALL` with value transfer in static context.
    CallNotAllowedInsideStatic,
    /// Invalid state modification in static call.
    StateChangeDuringStaticcall,
    /// An undefined bytecode value encountered during execution.
    InvalidFEOpcode,
    /// Invalid jump destination. Dynamic jumps points to invalid not jumpdest opcode.
    InvalidJump,
    /// The feature or opcode is not activated in this version of the VM.
    NotActivated,
    /// Attempting to pop a value from an empty stack.
    StackUnderflow,
    /// Attempting to push a value onto a full stack.
    StackOverflow,
    /// Invalid memory or storage offset.
    OutOfOffset,
    /// Address collision during contract creation.
    CreateCollision,
    /// Payment amount overflow.
    OverflowPayment,
    /// Error in precompiled contract execution.
    PrecompileError,
    /// Nonce overflow.
    NonceOverflow,
    /// Exceeded contract size limit during creation.
    CreateContractSizeLimit,
    /// Created contract starts with invalid bytes (`0xEF`).
    CreateContractStartingWithEF,
    /// Exceeded init code size limit (EIP-3860:  Limit and meter initcode).
    CreateInitCodeSizeLimit,
    /// `RETURNCONTRACT` called outside init EOF code.
    ReturnContractInNotInitEOF,
    /// Legacy contract is calling opcode that is enabled only in EOF.
    EOFOpcodeDisabledInLegacy,
    /// Stack overflow in EOF subroutine function calls.
    EOFFunctionStackOverflow,
    /// Aux data overflow, new aux data is larger than `u16` max size.
    EofAuxDataOverflow,
    /// Aux data is smaller then already present data size.
    EofAuxDataTooSmall,
    /// `EXT*CALL` target address needs to be padded with 0s.
    InvalidExtCallTarget,
    /// Fatal extenal error such as host errors.
    FatalExternalError,
}

impl ExitStatusCode {
    #[inline(always)]
    pub fn to_u8(self) -> u8 {
        self as u8
    }
    pub fn from_u8(value: u8) -> Self {
        match value {
            x if x == Self::Continue.to_u8() => Self::Continue,
            x if x == Self::Return.to_u8() => Self::Return,
            x if x == Self::Stop.to_u8() => Self::Stop,
            x if x == Self::Selfdestruct.to_u8() => Self::Selfdestruct,
            x if x == Self::Revert.to_u8() => Self::Revert,
            x if x == Self::CallTooDeep.to_u8() => Self::CallTooDeep,
            x if x == Self::OutOfFunds.to_u8() => Self::OutOfFunds,
            x if x == Self::CreateInitCodeStartingEF00.to_u8() => Self::CreateInitCodeStartingEF00,
            x if x == Self::InvalidEOFInitCode.to_u8() => Self::InvalidEOFInitCode,
            x if x == Self::InvalidExtDelegatecallTarget.to_u8() => {
                Self::InvalidExtDelegatecallTarget
            }
            x if x == Self::OutOfGas.to_u8() => Self::OutOfGas,
            x if x == Self::MemoryOOG.to_u8() => Self::MemoryOOG,
            x if x == Self::MemoryLimitOOG.to_u8() => Self::MemoryLimitOOG,
            x if x == Self::PrecompileOOG.to_u8() => Self::PrecompileOOG,
            x if x == Self::InvalidOperandOOG.to_u8() => Self::InvalidOperandOOG,
            x if x == Self::OpcodeNotFound.to_u8() => Self::OpcodeNotFound,
            x if x == Self::CallNotAllowedInsideStatic.to_u8() => Self::CallNotAllowedInsideStatic,
            x if x == Self::StateChangeDuringStaticcall.to_u8() => {
                Self::StateChangeDuringStaticcall
            }
            x if x == Self::InvalidFEOpcode.to_u8() => Self::InvalidFEOpcode,
            x if x == Self::InvalidJump.to_u8() => Self::InvalidJump,
            x if x == Self::NotActivated.to_u8() => Self::NotActivated,
            x if x == Self::StackUnderflow.to_u8() => Self::StackUnderflow,
            x if x == Self::StackOverflow.to_u8() => Self::StackOverflow,
            x if x == Self::OutOfOffset.to_u8() => Self::OutOfOffset,
            x if x == Self::CreateCollision.to_u8() => Self::CreateCollision,
            x if x == Self::OverflowPayment.to_u8() => Self::OverflowPayment,
            x if x == Self::PrecompileError.to_u8() => Self::PrecompileError,
            x if x == Self::NonceOverflow.to_u8() => Self::NonceOverflow,
            x if x == Self::CreateContractSizeLimit.to_u8() => Self::CreateContractSizeLimit,
            x if x == Self::CreateContractStartingWithEF.to_u8() => {
                Self::CreateContractStartingWithEF
            }
            x if x == Self::CreateInitCodeSizeLimit.to_u8() => Self::CreateInitCodeSizeLimit,
            x if x == Self::ReturnContractInNotInitEOF.to_u8() => Self::ReturnContractInNotInitEOF,
            x if x == Self::EOFOpcodeDisabledInLegacy.to_u8() => Self::EOFOpcodeDisabledInLegacy,
            x if x == Self::EOFFunctionStackOverflow.to_u8() => Self::EOFFunctionStackOverflow,
            x if x == Self::EofAuxDataOverflow.to_u8() => Self::EofAuxDataOverflow,
            x if x == Self::EofAuxDataTooSmall.to_u8() => Self::EofAuxDataTooSmall,
            x if x == Self::InvalidExtCallTarget.to_u8() => Self::InvalidExtCallTarget,
            _ => Self::Return,
        }
    }

    #[inline]
    pub fn is_ok(&self) -> bool {
        matches!(
            self,
            ExitStatusCode::Return | ExitStatusCode::Stop | ExitStatusCode::Selfdestruct
        )
    }

    #[inline]
    pub fn is_revert(&self) -> bool {
        matches!(
            self,
            ExitStatusCode::Revert
                | ExitStatusCode::CallTooDeep
                | ExitStatusCode::OutOfFunds
                | ExitStatusCode::CreateInitCodeStartingEF00
                | ExitStatusCode::InvalidEOFInitCode
                | ExitStatusCode::InvalidExtDelegatecallTarget
        )
    }

    #[inline]
    pub fn is_error(&self) -> bool {
        matches!(
            self,
            ExitStatusCode::OutOfGas
                | ExitStatusCode::MemoryOOG
                | ExitStatusCode::MemoryLimitOOG
                | ExitStatusCode::PrecompileOOG
                | ExitStatusCode::InvalidOperandOOG
                | ExitStatusCode::OpcodeNotFound
                | ExitStatusCode::CallNotAllowedInsideStatic
                | ExitStatusCode::StateChangeDuringStaticcall
                | ExitStatusCode::InvalidFEOpcode
                | ExitStatusCode::InvalidJump
                | ExitStatusCode::NotActivated
                | ExitStatusCode::StackUnderflow
                | ExitStatusCode::StackOverflow
                | ExitStatusCode::OutOfOffset
                | ExitStatusCode::CreateCollision
                | ExitStatusCode::OverflowPayment
                | ExitStatusCode::PrecompileError
                | ExitStatusCode::NonceOverflow
                | ExitStatusCode::CreateContractSizeLimit
                | ExitStatusCode::CreateContractStartingWithEF
                | ExitStatusCode::CreateInitCodeSizeLimit
                | ExitStatusCode::ReturnContractInNotInitEOF
                | ExitStatusCode::EOFOpcodeDisabledInLegacy
                | ExitStatusCode::EOFFunctionStackOverflow
                | ExitStatusCode::EofAuxDataOverflow
                | ExitStatusCode::EofAuxDataTooSmall
                | ExitStatusCode::InvalidExtCallTarget
        )
    }

    #[inline]
    pub fn is_stack_overflow(&self) -> bool {
        matches!(self, ExitStatusCode::StackOverflow)
    }

    #[inline]
    pub fn is_stack_underflow(&self) -> bool {
        matches!(self, ExitStatusCode::StackUnderflow)
    }

    #[inline]
    pub fn is_out_of_gas(&self) -> bool {
        matches!(
            self,
            ExitStatusCode::OutOfGas
                | ExitStatusCode::MemoryOOG
                | ExitStatusCode::PrecompileOOG
                | ExitStatusCode::MemoryLimitOOG
                | ExitStatusCode::InvalidOperandOOG
        )
    }

    #[inline]
    pub fn is_opcode_not_found(&self) -> bool {
        matches!(self, ExitStatusCode::OpcodeNotFound)
    }

    #[inline]
    pub fn is_invalid_jump(&self) -> bool {
        matches!(self, ExitStatusCode::InvalidJump)
    }
}
