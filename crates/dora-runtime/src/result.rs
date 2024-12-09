use crate::{context::Log, db::DatabaseError, journaled_state::State};
use core::fmt;
use dora_primitives::{Bytes, EVMAddress as Address};
use ruint::aliases::U256;
use std::{boxed::Box, fmt::Debug, string::String};

/// Represents the result of an EVM execution along with the updated account state.
///
/// This struct holds two fields:
/// - `result`: The `ExecutionResult` indicating the status of the transaction.
/// - `state`: A `HashMap` representing the updated account state after execution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResultAndState {
    /// Status of execution, containing details of success, revert, or halt.
    pub result: ExecutionResult,
    /// Updated state of accounts after execution.
    pub state: State,
}

/// Represents the result of executing a transaction in the EVM.
///
/// This enum contains three possible outcomes:
/// - `Success`: The transaction executed successfully.
/// - `Revert`: The transaction reverted using the `REVERT` opcode, consuming only part of the gas.
/// - `Halt`: The transaction halted, consuming all available gas.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ExecutionResult {
    /// Successful execution.
    /// Contains:
    /// - `reason`: Reason for successful completion.
    /// - `gas_used`: Total gas consumed.
    /// - `gas_refunded`: Gas refunded after execution.
    /// - `output`: Output data from the transaction.
    Success {
        reason: SuccessReason,
        gas_used: u64,
        gas_refunded: u64,
        output: Output,
        logs: Vec<Log>,
    },
    /// Reverted by the `REVERT` opcode, consuming partial gas.
    /// Contains:
    /// - `gas_used`: Gas consumed before the revert.
    /// - `output`: Output data until the revert.
    Revert { gas_used: u64, output: Bytes },
    /// Halted execution, consuming all gas.
    /// Contains:
    /// - `reason`: Reason for the halt (e.g., `OutOfGas`, `OpcodeNotFound`).
    /// - `gas_used`: Total gas consumed during execution.
    Halt {
        reason: HaltReason,
        gas_limit: u64,
        gas_used: u64,
    },
}

impl ExecutionResult {
    /// Checks if the execution was successful.
    ///
    /// # Returns
    /// - `true`: If the execution resulted in `Success`.
    /// - `false`: Otherwise.
    #[inline]
    pub fn is_success(&self) -> bool {
        matches!(self, Self::Success { .. })
    }

    /// Checks if the execution resulted in a revert.
    ///
    /// # Returns
    /// - `true`: If the execution resulted in `Revert`.
    /// - `false`: Otherwise.
    #[inline]
    pub fn is_revert(&self) -> bool {
        matches!(self, Self::Revert { .. })
    }

    /// Checks if the execution resulted in a halt.
    ///
    /// # Returns
    /// - `true`: If the execution resulted in `Halt`.
    /// - `false`: Otherwise.
    #[inline]
    pub fn is_halt(&self) -> bool {
        matches!(self, Self::Halt { .. })
    }

    /// Get the halt reason
    #[inline]
    pub fn halt_reason(&self) -> Option<&HaltReason> {
        if let Self::Halt { reason, .. } = self {
            Some(reason)
        } else {
            None
        }
    }

    /// Returns the output data produced by the execution, if any.
    ///
    /// This method will return the output from both successful and reverted executions. It filters
    /// out any empty data.
    ///
    /// # Returns
    /// - `Some(&Bytes)`: If output data exists and is non-empty.
    /// - `None`: If no output data or the data is empty.
    #[inline]
    pub fn output(&self) -> Option<&Bytes> {
        match self {
            Self::Success { output, .. } => Some(output.data()),
            Self::Revert { output, .. } => Some(output),
            _ => None,
        }
        .filter(|data| !data.is_empty())
    }

    /// Returns the logs if execution is successful, or an empty list otherwise.
    pub fn logs(&self) -> &[Log] {
        match self {
            Self::Success { logs, .. } => logs,
            _ => &[],
        }
    }

    /// Consumes the execution result and returns the output data.
    ///
    /// This method consumes the `ExecutionResult` and returns the output data, if any. It works for both
    /// successful and reverted executions.
    ///
    /// # Returns
    /// - `Some(Bytes)`: If output data exists.
    /// - `None`: If there is no output data.
    #[inline]
    pub fn into_output(self) -> Option<Bytes> {
        match self {
            Self::Success { output, .. } => Some(output.into_data()),
            Self::Revert { output, .. } => Some(output),
            _ => None,
        }
    }

    /// Returns the gas used during the execution.
    ///
    /// This method provides the amount of gas consumed during execution, regardless of the result (success, revert, or halt).
    ///
    /// # Returns
    /// - `u64`: The amount of gas used.
    #[inline]
    pub fn gas_used(&self) -> u64 {
        match self {
            Self::Success { gas_used, .. }
            | Self::Revert { gas_used, .. }
            | Self::Halt { gas_used, .. } => *gas_used,
        }
    }

    /// Returns the gas refunded from the execution, if applicable.
    ///
    /// Gas refunds are only available for successful executions. If the result is not `Success`, this method returns 0.
    ///
    /// # Returns
    /// - `u64`: The amount of gas refunded, or 0 if not applicable.
    #[inline]
    pub fn gas_refunded(&self) -> u64 {
        match self {
            Self::Success { gas_refunded, .. } => *gas_refunded,
            _ => 0,
        }
    }
}

/// Represents the output of a successful transaction execution.
///
/// This enum has two variants:
/// - `Call`: Output from a regular call.
/// - `Create`: Output from contract creation, optionally returning the created contract address.
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Output {
    /// Output from a call.
    Call(Bytes),
    /// Output from a contract creation, optionally includes the created contract address.
    Create(Bytes, Option<Address>),
}

impl fmt::Debug for Output {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Output::Call(ref output) => f
                .debug_tuple("Output::Call")
                // Use the hex output format
                .field(&hex::encode(output))
                .finish(),
            Output::Create(ref output, ref address) => f
                .debug_tuple("Output::Create")
                // Use the hex output format
                .field(&hex::encode(output))
                .field(address)
                .finish(),
        }
    }
}

impl Output {
    /// Consumes the output and returns the data.
    pub fn into_data(self) -> Bytes {
        match self {
            Output::Call(data) | Output::Create(data, _) => data,
        }
    }

    /// Returns a reference to the output data.
    pub fn data(&self) -> &Bytes {
        match self {
            Output::Call(data) | Output::Create(data, _) => data,
        }
    }

    /// Returns the created address, if applicable.
    pub fn address(&self) -> Option<&Address> {
        if let Output::Create(_, address) = self {
            address.as_ref()
        } else {
            None
        }
    }
}

/// Represents errors that can occur during EVM execution.
///
/// This enum covers various error categories:
/// - `Transaction`: Errors related to transaction validation.
/// - `Header`: Errors related to block header validation.
/// - `Database`: Errors related to database operations.
/// - `Custom`: A custom error message.
/// - `Precompile`: Errors occurring within a precompiled contract.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EVMError {
    Transaction(InvalidTransaction),
    Header(InvalidHeader),
    Database(DatabaseError),
    Custom(String),
    Precompile(String),
}

impl fmt::Display for EVMError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Transaction(e) => write!(f, "transaction validation error: {}", e),
            Self::Header(e) => write!(f, "header validation error: {}", e),
            Self::Database(e) => write!(f, "database error: {}", e),
            Self::Custom(e) | Self::Precompile(e) => write!(f, "{}", e),
        }
    }
}

impl From<InvalidTransaction> for EVMError {
    fn from(value: InvalidTransaction) -> Self {
        Self::Transaction(value)
    }
}

impl From<InvalidHeader> for EVMError {
    fn from(value: InvalidHeader) -> Self {
        Self::Header(value)
    }
}

/// Errors related to transaction validation.
///
/// This enum represents various conditions that can invalidate a transaction during validation.
/// Each variant describes a specific issue or inconsistency encountered during transaction processing.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum InvalidTransaction {
    PriorityFeeGreaterThanMaxFee,
    GasPriceLessThanBasefee,
    CallerGasLimitMoreThanBlock,
    CallGasCostMoreThanGasLimit,
    RejectCallerWithCode,
    LackOfFundForMaxFee { fee: Box<U256>, balance: Box<U256> },
    OverflowPaymentInTransaction,
    NonceOverflowInTransaction,
    NonceTooHigh { tx: u64, state: u64 },
    NonceTooLow { tx: u64, state: u64 },
    CreateInitCodeSizeLimit,
    InvalidChainId,
    AccessListNotSupported,
    MaxFeePerBlobGasNotSupported,
    BlobVersionedHashesNotSupported,
    BlobGasPriceGreaterThanMax,
    EmptyBlobs,
    BlobCreateTransaction,
    TooManyBlobs { max: usize, have: usize },
    BlobVersionNotSupported,
    EofCrateShouldHaveToAddress,
}

impl fmt::Display for InvalidTransaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use InvalidTransaction::*;
        match self {
            PriorityFeeGreaterThanMaxFee => write!(f, "priority fee is greater than max fee"),
            GasPriceLessThanBasefee => write!(f, "gas price is less than basefee"),
            CallerGasLimitMoreThanBlock => write!(f, "caller gas limit exceeds block gas limit"),
            CallGasCostMoreThanGasLimit => write!(f, "call gas cost exceeds gas limit"),
            RejectCallerWithCode => {
                write!(f, "reject transactions from senders with deployed code")
            }
            LackOfFundForMaxFee { fee, balance } => {
                write!(
                    f,
                    "lack of funds ({} balance) for max fee ({})",
                    balance, fee
                )
            }
            OverflowPaymentInTransaction => write!(f, "overflow payment in transaction"),
            NonceOverflowInTransaction => write!(f, "nonce overflow in transaction"),
            NonceTooHigh { tx, state } => write!(f, "nonce {} too high, expected {}", tx, state),
            NonceTooLow { tx, state } => write!(f, "nonce {} too low, expected {}", tx, state),
            CreateInitCodeSizeLimit => write!(f, "create initcode size limit exceeded"),
            InvalidChainId => write!(f, "invalid chain ID"),
            AccessListNotSupported => write!(f, "access list not supported"),
            MaxFeePerBlobGasNotSupported => write!(f, "max fee per blob gas not supported"),
            BlobVersionedHashesNotSupported => write!(f, "blob versioned hashes not supported"),
            BlobGasPriceGreaterThanMax => write!(f, "blob gas price greater than max fee per blob"),
            EmptyBlobs => write!(f, "empty blobs"),
            BlobCreateTransaction => write!(f, "blob transactions cannot be create transactions"),
            TooManyBlobs { max, have } => write!(f, "too many blobs, max {}, have {}", max, have),
            BlobVersionNotSupported => write!(f, "blob version not supported"),
            EofCrateShouldHaveToAddress => write!(f, "EOF crate should have a `to` address"),
        }
    }
}

/// Errors related to block header validation.
///
/// This enum contains specific errors related to missing fields in block headers:
/// - `PrevrandaoNotSet`: The `prev_randao` field is missing.
/// - `ExcessBlobGasNotSet`: The excess blob gas field is not set.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum InvalidHeader {
    PrevrandaoNotSet,
    ExcessBlobGasNotSet,
}

impl fmt::Display for InvalidHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PrevrandaoNotSet => write!(f, "`prevrandao` not set"),
            Self::ExcessBlobGasNotSet => write!(f, "`excess_blob_gas` not set"),
        }
    }
}

/// Represents the reason a transaction successfully completed.
///
/// This enum describes various reasons for successful completion:
/// - `Stop`: The `STOP` opcode was executed.
/// - `Return`: The `RETURN` opcode was executed.
/// - `SelfDestruct`: The contract self-destructed.
/// - `EofReturnContract`: EOF contract execution returned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SuccessReason {
    Stop,
    Return,
    SelfDestruct,
    EofReturnContract,
}

/// Represents the reason for halting EVM execution.
///
/// This enum contains various reasons that can cause the EVM to halt:
/// - `OutOfGas`: Execution ran out of gas (with different subtypes).
/// - `OpcodeNotFound`: Invalid opcode encountered.
/// - `InvalidFEOpcode`: Invalid front-end opcode encountered.
/// - `InvalidJump`: Invalid jump destination encountered.
/// - `NotActivated`: The opcode or feature is not activated.
/// - `StackUnderflow`: Stack underflow during execution.
/// - `StackOverflow`: Stack overflow during execution.
/// - `OutOfOffset`: Memory access beyond bounds.
/// - `CreateCollision`: Contract creation resulted in an address collision.
/// - `PrecompileError`: Error during precompiled contract execution.
/// - `NonceOverflow`: Account nonce overflow.
/// - `CreateContractSizeLimit`: Contract size exceeds limit.
/// - `CreateContractStartingWithEF`: Contract starts with an invalid byte.
/// - `CreateInitCodeSizeLimit`: Init code size exceeds limit.
/// - `OverflowPayment`: Overflow occurred during payment.
/// - `StateChangeDuringStaticcall`: State change attempted during a static call.
/// - `CallNotAllowedInsideStatic`: Call attempted inside a static call.
/// - `OutOfFunds`: Insufficient funds for execution.
/// - `CallTooDeep`: Call stack exceeded the depth limit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HaltReason {
    OutOfGas(OutOfGasError),
    OpcodeNotFound,
    InvalidFEOpcode,
    InvalidJump,
    NotActivated,
    StackUnderflow,
    StackOverflow,
    OutOfOffset,
    CreateCollision,
    PrecompileError,
    NonceOverflow,
    /// Create init code size exceeds limit (runtime).
    CreateContractSizeLimit,
    /// Error on created contract that begins with EF
    CreateContractStartingWithEF,
    /// EIP-3860: Limit and meter initcode. Initcode size limit exceeded.
    CreateInitCodeSizeLimit,
    OverflowPayment,
    StateChangeDuringStaticcall,
    CallNotAllowedInsideStatic,
    OutOfFunds,
    CallTooDeep,
    /// Aux data overflow, new aux data is larger than u16 max size.
    EofAuxDataOverflow,
    /// Aud data is smaller then already present data size.
    EofAuxDataTooSmall,
    /// EOF Subroutine stack overflow
    EOFFunctionStackOverflow,
    /// Check for target address validity is only done inside subcall.
    InvalidExtCallTarget,
    /// Check for target address validity is only done inside sub delegate call.
    InvalidExtDelegatecallTarget,
}

impl HaltReason {
    #[inline]
    pub fn is_stack_overflow(&self) -> bool {
        matches!(self, HaltReason::StackOverflow)
    }

    #[inline]
    pub fn is_stack_underflow(&self) -> bool {
        matches!(self, HaltReason::StackUnderflow)
    }

    #[inline]
    pub fn is_out_of_gas(&self) -> bool {
        matches!(self, HaltReason::OutOfGas(_))
    }

    #[inline]
    pub fn is_opcode_not_found(&self) -> bool {
        matches!(self, HaltReason::OpcodeNotFound)
    }

    #[inline]
    pub fn is_invalid_jump(&self) -> bool {
        matches!(self, HaltReason::InvalidJump)
    }
}

/// Represents out-of-gas errors during EVM execution.
///
/// This enum specifies various out-of-gas scenarios:
/// - `Basic`: Generic out-of-gas error during execution.
/// - `MemoryLimit`: Memory expansion exceeded the EVM's memory limit.
/// - `Memory`: Insufficient gas for memory expansion.
/// - `Precompile`: Out-of-gas in a precompiled contract.
/// - `Create`: Out-of-gas during contract creation.
/// - `InvalidOperand`: Out-of-gas according to the operand such as overflow.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OutOfGasError {
    /// Basic Out-of-Gas error during execution.
    Basic,
    /// Out-of-Gas due to memory expansion exceeding the EVM's memory limit.
    MemoryLimit,
    /// Out-of-Gas due to insufficient gas for memory expansion.
    Memory,
    /// Out-of-Gas triggered by a precompile.
    Precompile,
    /// Out-of-Gas during contract creation.
    Create,
    // When performing something that takes a U256 and casts down to a u64, if its too large this would fire
    // i.e. in `as_usize_or_fail`
    InvalidOperand,
}

impl fmt::Display for HaltReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HaltReason::OutOfGas(err) => write!(f, "Out of gas: {}", err),
            HaltReason::OpcodeNotFound => write!(f, "Opcode not found"),
            HaltReason::InvalidFEOpcode => write!(f, "Invalid FE opcode"),
            HaltReason::InvalidJump => write!(f, "Invalid jump"),
            HaltReason::NotActivated => write!(f, "Not activated"),
            HaltReason::StackUnderflow => write!(f, "Stack underflow"),
            HaltReason::StackOverflow => write!(f, "Stack overflow"),
            HaltReason::OutOfOffset => write!(f, "Out of offset"),
            HaltReason::CreateCollision => write!(f, "Create collision"),
            HaltReason::PrecompileError => write!(f, "Precompile error"),
            HaltReason::NonceOverflow => write!(f, "Nonce overflow"),
            HaltReason::CreateContractSizeLimit => {
                write!(f, "Create contract size exceeds limit")
            }
            HaltReason::CreateContractStartingWithEF => {
                write!(f, "Created contract starts with EF")
            }
            HaltReason::CreateInitCodeSizeLimit => write!(f, "Initcode size limit exceeded"),
            HaltReason::OverflowPayment => write!(f, "Overflow payment"),
            HaltReason::StateChangeDuringStaticcall => {
                write!(f, "State change during static call")
            }
            HaltReason::CallNotAllowedInsideStatic => {
                write!(f, "Call not allowed inside static")
            }
            HaltReason::OutOfFunds => write!(f, "Out of funds"),
            HaltReason::CallTooDeep => write!(f, "Call too deep"),
            HaltReason::EofAuxDataOverflow => write!(f, "EOF aux data overflow"),
            HaltReason::EofAuxDataTooSmall => write!(f, "EOF aux data too small"),
            HaltReason::EOFFunctionStackOverflow => write!(f, "EOF function stack overflow"),
            HaltReason::InvalidExtCallTarget => write!(f, "Invalid external call target"),
            HaltReason::InvalidExtDelegatecallTarget => {
                write!(f, "Invalid external delegatecall target")
            }
        }
    }
}

impl fmt::Display for OutOfGasError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OutOfGasError::Basic => write!(f, "Basic out-of-gas error during execution"),
            OutOfGasError::MemoryLimit => write!(f, "Out-of-gas: memory expansion exceeded limit"),
            OutOfGasError::Memory => write!(f, "Out-of-gas: insufficient gas for memory expansion"),
            OutOfGasError::Precompile => write!(f, "Out-of-gas triggered by a precompile"),
            OutOfGasError::Create => write!(f, "Out-of-gas during contract creation"),
            OutOfGasError::InvalidOperand => {
                write!(f, "Out-of-gas: invalid operand for operation")
            }
        }
    }
}
