use crate::constants::MAIN_ENTRYPOINT;
use crate::context::{MainFunc, RuntimeContext};
use dora_primitives::config::OptimizationLevel;
use melior::ir::Module;
use melior::ExecutionEngine;

/// A struct that wraps around the MLIR-based execution engine for executing compiled EVM code.
///
/// The `Executor` is responsible for managing the execution engine and invoking the main entry point of the compiled
/// code. It serves as the core execution unit, executing the EVM bytecode compiled via MLIR within the provided
/// `RuntimeContext`.
///
/// # Fields:
/// - `engine`: An instance of `ExecutionEngine`, used to execute the compiled EVM bytecode.
///
/// # Example Usage:
/// ```no_check
/// let executor = Executor::new(&module, &runtime_ctx, OptimizationLevel::Default);
/// let result = executor.execute(&mut runtime_ctx, initial_gas);
/// ```
pub struct Executor {
    engine: ExecutionEngine,
}

impl Executor {
    /// Creates a new `Executor` instance by initializing the `ExecutionEngine` with the provided MLIR `module` and
    /// `OptimizationLevel`. Registers necessary runtime symbols in the execution engine.
    ///
    /// This method constructs an `ExecutionEngine` for the given compiled module and optimization level,
    /// and registers symbols required by the `RuntimeContext`.
    ///
    /// # Arguments:
    /// - `module`: A reference to the MLIR `Module` containing the compiled EVM bytecode.
    /// - `runtime_ctx`: A reference to the `RuntimeContext`, which provides the execution environment.
    /// - `opt_level`: The optimization level to be applied during the execution of the bytecode.
    ///
    /// # Returns:
    /// - `Self`: A new `Executor` instance.
    ///
    /// # Example Usage:
    /// ```no_check
    /// let executor = Executor::new(&module, &runtime_ctx, OptimizationLevel::Aggressive);
    /// ```
    pub fn new(
        module: &Module,
        runtime_ctx: &RuntimeContext,
        opt_level: OptimizationLevel,
    ) -> Self {
        let engine = ExecutionEngine::new(module, opt_level as usize, &[], false);
        runtime_ctx.register_symbols(&engine);
        Self { engine }
    }

    /// Executes the main entry point of the compiled EVM code.
    ///
    /// This function calls the main entry point of the compiled module with the provided `RuntimeContext`
    /// and initial gas amount. It retrieves the main entry function pointer and executes it with the supplied
    /// parameters.
    ///
    /// # Arguments:
    /// - `context`: A mutable reference to the `RuntimeContext`, which holds the environment for the execution.
    /// - `initial_gas`: The initial amount of gas available for the execution.
    ///
    /// # Returns:
    /// - `u8`: The result code from executing the main entry point, typically indicating success or failure.
    ///
    /// # Example Usage:
    /// ```no_check
    /// let result_code = executor.execute(&mut runtime_ctx, initial_gas);
    /// ```
    pub fn execute(&self, context: &mut RuntimeContext, initial_gas: u64) -> u8 {
        let main_fn = self.get_main_entrypoint();
        main_fn(context, initial_gas)
    }

    /// Retrieves the main entry point function from the execution engine.
    ///
    /// This function constructs the main entry point's symbol name in the format `_mlir_ciface_<MAIN_ENTRYPOINT>`
    /// and looks it up in the execution engine. It then converts the function pointer into the expected function
    /// signature (`MainFunc`).
    ///
    /// # Returns:
    /// - `MainFunc`: A function pointer to the main entry point of the compiled module.
    ///
    /// # Safety:
    /// The function pointer is assumed to be valid and to conform to the expected signature of `MainFunc`.
    ///
    /// # Example Usage:
    /// ```no_check
    /// let main_fn = executor.get_main_entrypoint();
    /// ```
    pub fn get_main_entrypoint(&self) -> MainFunc {
        let function_name = format!("_mlir_ciface_{MAIN_ENTRYPOINT}");
        let fptr = self.engine.lookup(&function_name);
        // SAFETY: We're assuming the function pointer is valid and matches the MainFunc signature.
        unsafe { std::mem::transmute(fptr) }
    }
}
