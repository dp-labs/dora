#![allow(clippy::arc_with_non_send_sync)]

use crate::constants::MAIN_ENTRYPOINT;
use crate::context::{EVMMainFunc, RuntimeContext, WASMMainFunc};
use crate::wasm::WASMInstance;
use dora_primitives::config::OptimizationLevel;
use melior::ir::Module;
use melior::StringRef;
use mlir_sys::{
    mlirExecutionEngineCreate, mlirExecutionEngineDestroy, mlirExecutionEngineLookup,
    mlirExecutionEngineRegisterSymbol, MlirExecutionEngine,
};
use parking_lot::RwLock;
use std::fmt::Debug;
use std::sync::Arc;

/// The stack size at runtime, used for recursive program execution to prevent stack overflow
pub const RUNTIME_STACK_SIZE: usize = 64 * 1024 * 1024;

/// A struct that wraps around the MLIR-based execution engine for executing compiled EVM/WASM bytecode.
///
/// The `Executor` is responsible for managing the execution engine and invoking the main entry point of the compiled
/// code. It serves as the core execution unit, executing the EVM/WASM bytecode compiled via MLIR within the provided
/// `RuntimeContext`.
#[derive(Default, Debug, Clone)]
pub struct Executor {
    engine: ExecutionEngine,
    pub(crate) kind: ExecuteKind,
}

/// Runtime engine execute kind including EVM, WASM, etc.
#[derive(Default, Debug, Clone)]
pub enum ExecuteKind {
    #[default]
    EVM,
    WASM(Arc<RwLock<WASMInstance>>),
}

unsafe impl Send for ExecuteKind {}
unsafe impl Sync for ExecuteKind {}

impl ExecuteKind {
    pub fn new_wasm(instance: WASMInstance) -> ExecuteKind {
        ExecuteKind::WASM(Arc::new(RwLock::new(instance)))
    }
}

impl Executor {
    /// Creates a new `Executor` instance by initializing the `ExecutionEngine` with the provided MLIR `module` and
    /// `OptimizationLevel`. Registers necessary runtime symbols in the execution engine.
    ///
    /// This method constructs an `ExecutionEngine` for the given compiled module and optimization level,
    /// and registers symbols required by the `RuntimeContext`.
    ///
    /// # Arguments:
    /// - `module`: A reference to the MLIR `Module` containing the compiled EVM/WASM bytecode.
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
    pub fn new(module: &Module, opt_level: OptimizationLevel, kind: ExecuteKind) -> Self {
        let engine = ExecutionEngine::new(module, opt_level as usize, &[], false);
        match kind {
            ExecuteKind::EVM => RuntimeContext::register_evm_symbols(&engine),
            ExecuteKind::WASM(_) => RuntimeContext::register_wasm_symbols(&engine),
        }
        Self { engine, kind }
    }

    /// Retrieves the EVM main entry point function from the execution engine.
    ///
    /// This function constructs the main entry point's symbol name in the format `_mlir_ciface_<MAIN_ENTRYPOINT>`
    /// and looks it up in the execution engine. It then converts the function pointer into the expected function
    /// signature (`EVMMainFunc`).
    ///
    /// # Returns:
    /// - `EVMMainFunc`: A function pointer to the main entry point of the compiled module.
    ///
    /// # Safety:
    /// The function pointer is assumed to be valid and to conform to the expected signature of `EVMMainFunc`.
    pub fn get_evm_main_entrypoint(&self) -> EVMMainFunc {
        let fptr = self.get_main_entrypoint_ptr();
        // SAFETY: We're assuming the function pointer is valid and matches the MainFunc signature.
        unsafe { std::mem::transmute(fptr) }
    }

    /// Retrieves the WASM main entry point function from the execution engine.
    ///
    /// This function constructs the main entry point's symbol name in the format `_mlir_ciface_<MAIN_ENTRYPOINT>`
    /// and looks it up in the execution engine. It then converts the function pointer into the expected function
    /// signature (`WASMMainFunc`).
    ///
    /// # Returns:
    /// - `WASMMainFunc`: A function pointer to the main entry point of the compiled module.
    ///
    /// # Safety:
    /// The function pointer is assumed to be valid and to conform to the expected signature of `WASMMainFunc`.
    ///
    /// # Example Usage:
    /// ```no_check
    /// let main_fn = executor.get_main_entrypoint();
    /// ```
    pub fn get_wasm_main_entrypoint(&self) -> WASMMainFunc {
        let fptr = self.get_main_entrypoint_ptr();
        // SAFETY: We're assuming the function pointer is valid and matches the MainFunc signature.
        unsafe { std::mem::transmute(fptr) }
    }

    /// Retrieves the main entry point function pointer from the execution engine.
    pub fn get_main_entrypoint_ptr(&self) -> *mut () {
        let function_name = format!("_mlir_ciface_{MAIN_ENTRYPOINT}");
        self.engine.lookup(&function_name)
    }

    /// Searches a symbol in a module and returns a pointer to it.
    #[inline]
    pub fn lookup(&self, name: &str) -> *mut () {
        self.engine.lookup(name)
    }
}

/// A shared reference execution engine for the artifact cache.
#[derive(Debug)]
pub struct ExecutionEngine {
    raw: Arc<MlirExecutionEngine>,
}

unsafe impl Send for ExecutionEngine {}
unsafe impl Sync for ExecutionEngine {}

impl Clone for ExecutionEngine {
    fn clone(&self) -> Self {
        ExecutionEngine {
            raw: Arc::clone(&self.raw),
        }
    }
}

impl Default for ExecutionEngine {
    fn default() -> Self {
        Self {
            raw: Arc::new(MlirExecutionEngine {
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
                Arc::new(mlirExecutionEngineCreate(
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
        if Arc::strong_count(&self.raw) == 1 {
            unsafe { mlirExecutionEngineDestroy(*self.raw) }
        }
    }
}
