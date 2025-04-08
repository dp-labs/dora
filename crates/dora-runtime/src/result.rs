use crate::{db::DatabaseError, journaled_state::State};
use core::fmt;
pub use dora_primitives::{ExecutionResult, HaltReason, OutOfGasError, Output, SuccessReason};
use dora_primitives::{InvalidHeader, InvalidTransaction};
use std::{fmt::Debug, string::String};

/// Represents the result of an VM execution along with the updated account state.
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

/// Represents errors that can occur during VM execution.
///
/// This enum covers various error categories:
/// - `Transaction`: Errors related to transaction validation.
/// - `Header`: Errors related to block header validation.
/// - `Database`: Errors related to database operations.
/// - `Custom`: A custom error message.
/// - `Precompile`: Errors occurring within a precompiled contract.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VMError {
    Transaction(InvalidTransaction),
    Header(InvalidHeader),
    Database(DatabaseError),
    Compile(String),
    Precompile(String),
    Handler(String),
}

impl fmt::Display for VMError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Transaction(e) => write!(f, "transaction validation error: {}", e),
            Self::Header(e) => write!(f, "header validation error: {}", e),
            Self::Database(e) => write!(f, "database error: {}", e),
            Self::Handler(e) => write!(f, "handler error: {}", e),
            Self::Compile(e) => write!(f, "compile error: {}", e),
            Self::Precompile(e) => write!(f, "{}", e),
        }
    }
}

impl From<InvalidTransaction> for VMError {
    fn from(value: InvalidTransaction) -> Self {
        Self::Transaction(value)
    }
}

impl From<InvalidHeader> for VMError {
    fn from(value: InvalidHeader) -> Self {
        Self::Header(value)
    }
}

impl From<DatabaseError> for VMError {
    fn from(value: DatabaseError) -> Self {
        VMError::Database(value)
    }
}
