use crate::constants::MAIN_ENTRYPOINT;
use crate::context::{MainFunc, RuntimeContext, Stack};
use dora_primitives::config::OptimizationLevel;
use melior::ir::Module;
use melior::StringRef;
use mlir_sys::{
    mlirExecutionEngineCreate, mlirExecutionEngineDestroy, mlirExecutionEngineLookup,
    mlirExecutionEngineRegisterSymbol, MlirExecutionEngine,
};
use std::fmt::Debug;
use std::rc::Rc;

/// The stack size at runtime, used for recursive program execution to prevent stack overflow
pub const RUNTIME_STACK_SIZE: usize = 64 * 1024 * 1024;

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
#[derive(Default, Debug, Clone)]
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
    pub fn new(module: &Module, opt_level: OptimizationLevel) -> Self {
        let engine = ExecutionEngine::new(module, opt_level as usize, &[], false);
        RuntimeContext::register_symbols(&engine);
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
    pub fn execute(
        &self,
        context: &mut RuntimeContext,
        initial_gas: &mut u64,
        stack: &mut Stack,
        stack_size: &mut u64,
    ) -> u8 {
        let main_fn = self.get_main_entrypoint();
        main_fn(context, initial_gas, stack, stack_size)
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
        let fptr = self.get_main_entrypoint_ptr();
        // SAFETY: We're assuming the function pointer is valid and matches the MainFunc signature.
        unsafe { std::mem::transmute(fptr) }
    }

    /// Retrieves the main entry point function pointer from the execution engine.
    pub fn get_main_entrypoint_ptr(&self) -> *mut () {
        let function_name = format!("_mlir_ciface_{MAIN_ENTRYPOINT}");
        self.engine.lookup(&function_name)
    }
}

/// A shared reference execution engine for the artifact cache.
#[derive(Debug)]
pub struct ExecutionEngine {
    raw: Rc<MlirExecutionEngine>,
}

impl Clone for ExecutionEngine {
    fn clone(&self) -> Self {
        ExecutionEngine {
            raw: Rc::clone(&self.raw),
        }
    }
}

impl Default for ExecutionEngine {
    fn default() -> Self {
        Self {
            raw: Rc::new(MlirExecutionEngine {
                ptr: std::ptr::null_mut(),
            }),
        }
    }
}

impl ExecutionEngine {
    /// Creates an execution engine.
    pub fn new(
        module: &Module,
        optimization_level: usize,
        shared_library_paths: &[&str],
        enable_object_dump: bool,
    ) -> Self {
        Self {
            raw: unsafe {
                Rc::new(mlirExecutionEngineCreate(
                    module.to_raw(),
                    optimization_level as i32,
                    shared_library_paths.len() as i32,
                    shared_library_paths
                        .iter()
                        .map(|&string| StringRef::new(string).to_raw())
                        .collect::<Vec<_>>()
                        .as_ptr(),
                    enable_object_dump,
                ))
            },
        }
    }

    /// Searches a symbol in a module and returns a pointer to it.
    #[inline]
    pub fn lookup(&self, name: &str) -> *mut () {
        unsafe { mlirExecutionEngineLookup(*self.raw, StringRef::new(name).to_raw()) as *mut () }
    }

    /// Register a symbol. This symbol will be accessible to the JIT'd codes.
    ///
    /// # Safety
    ///
    /// This function makes a pointer accessible to the execution engine. If a
    /// given pointer is invalid or misaligned, calling this function might
    /// result in undefined behavior.
    #[inline]
    pub unsafe fn register_symbol(&self, name: &str, ptr: *mut ()) {
        mlirExecutionEngineRegisterSymbol(*self.raw, StringRef::new(name).to_raw(), ptr as _);
    }
}

impl Drop for ExecutionEngine {
    fn drop(&mut self) {
        if Rc::strong_count(&self.raw) == 1 {
            unsafe { mlirExecutionEngineDestroy(*self.raw) }
        }
    }
}
