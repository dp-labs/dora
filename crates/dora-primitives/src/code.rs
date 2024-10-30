/// The `Program` trait provides an interface for creating a program instance from bytecode.
pub trait Program {
    /// Creates a new program instance from the provided bytecode.
    ///
    /// # Parameters:
    /// - `bytecode`: A slice of bytes representing the compiled program code.
    ///
    /// # Returns:
    /// - `Self`: A new instance of the program.
    fn from_bytecode(bytecode: &[u8]) -> Self;
}

/// The `Executor` trait defines the interface for executing a program in a specific context.
pub trait Executor {
    /// The type representing the execution context.
    type Context;

    /// Executes the program in the provided context, consuming a specified amount of initial gas.
    ///
    /// # Parameters:
    /// - `context`: A mutable reference to the execution context.
    /// - `initial_gas`: The initial amount of gas available for execution.
    ///
    /// # Returns:
    /// - `u8`: A result code indicating the success or failure of the execution.
    ///   Typically, `0` represents success, and other values indicate different types of failure.
    fn execute(&self, context: &mut &Self::Context, initial_gas: u64) -> u8;
}

/// The `ExecutorFactory` trait defines an interface for creating executor instances that can execute programs.
pub trait ExecutorFactory {
    /// The type of executor produced by the factory.
    type ExecutorType: Executor;

    /// The type of module (or program) that can be executed by the executor.
    type Module;

    /// Creates a new executor for the provided module and context, with an optional optimization level.
    ///
    /// # Parameters:
    /// - `module`: The module (program) to be executed by the executor.
    /// - `context`: The execution context in which the executor will run.
    /// - `optimization_level`: A level of optimization to apply to the executor (e.g., 0 for no optimization,
    ///   higher values for more aggressive optimizations).
    ///
    /// # Returns:
    /// - `Self::ExecutorType`: A new executor instance configured with the provided module, context, and optimization level.
    fn create(
        &self,
        module: &Self::Module,
        context: &<Self::ExecutorType as Executor>::Context,
        optimization_level: usize,
    ) -> Self::ExecutorType;
}
