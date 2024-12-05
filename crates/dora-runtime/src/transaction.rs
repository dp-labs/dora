use std::fmt::Debug;

/// A trait representing a blockchain transaction that can be executed within a specified context.
///
/// This trait defines the core structure for executing transactions, including the context required for execution
/// and the type of result that should be returned upon completion. It provides an interface for running the transaction
/// with a mutable context and an initial gas limit.
///
/// # Associated Types
/// - `Context`: The type representing the context required as input for transaction execution (e.g., environment settings, call frames).
/// - `Result`: The type representing the outcome of executing the transaction, allowing for both successful and error results.
///
/// # Required Methods
///
/// - `run(&self, ctx: &mut Self::Context, initial_gas: u64) -> Self::Result`
///
///   Executes the transaction in the specified mutable context with a given initial gas limit.
///
///   # Parameters
///   - `ctx`: A mutable reference to the execution context in which the transaction operates, which may include state, environment, and other execution-related data.
///   - `initial_gas`: The initial amount of gas allocated for the transaction, controlling the transaction's execution limits.
///
///   # Returns
///   - `Self::Result`: The result of the transaction execution, which may include a state result and/or error information.
pub trait Transaction: Debug {
    type Context;
    type Result;

    fn run(&self, ctx: &mut Self::Context, initial_gas: u64) -> Self::Result;
}

/// Transaction types of all Ethereum transaction.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TransactionType {
    /// Legacy transaction type.
    Legacy,
    /// EIP-2930 Access List transaction type.
    Eip2930,
    /// EIP-1559 Fee market change transaction type.
    Eip1559,
    /// EIP-4844 Blob transaction type.
    Eip4844,
    /// EIP-7702 Set EOA account code transaction type.
    Eip7702,
    /// Custom type means that transaction trait was extend and have custom types.
    Custom,
}
