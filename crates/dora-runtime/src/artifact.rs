use crate::{
    context::{EVMMainFunc, RuntimeContext, Stack, WASMMainFunc},
    executor::{ExecuteKind, Executor},
};
use std::fmt::Debug;

/// Artifact represents an abstraction of a compilation product for EVM/WASM bytecode.
/// This versatile concept can be implemented in various forms, including intermediate
/// compiled code, JIT memory format, or AOT binary executable/library/object format.
/// The compilation artifact is a crucial component of `AccountInfo<A: Artifact>` and
/// is stored in conjunction with the pre-compilation `code` and its corresponding `code_hash`.
///
/// Upon compilation of an account's bytecode, the resulting artifact is stored within
/// that account. This approach offers a significant advantage: when a transaction is
/// subsequently directed to that account, the pre-compiled code can be executed directly,
/// eliminating the need for recompilation. This optimization enhances efficiency and
/// reduces computational overhead in blockchain operations.
pub trait Artifact: Default + Debug + Clone {
    /// Creates an artifact from an executor.
    /// This method is responsible for transforming the executor's state into a compiled artifact.
    ///
    /// # Parameters
    /// * `executor`: The Executor instance containing the compiled code and related information.
    ///
    /// # Returns
    /// A new instance of the Artifact implementation.
    fn new(executor: Executor) -> Self;
    /// Executes the artifact with the given runtime context and initial gas.
    /// This method runs the compiled code represented by the artifact.
    ///
    /// # Parameters
    /// * `runtime_context`: Mutable reference to the runtime context, which may be modified during execution.
    /// * `initial_gas`: The initial amount of gas available for this execution.
    ///
    /// # Returns
    /// A u8 value, typically representing an execution status or error code.
    fn execute(
        &self,
        runtime_context: &mut RuntimeContext,
        initial_gas: &mut u64,
        stack: &mut Stack,
        stack_size: &mut u64,
    ) -> u8;
}

/// A memory artifact that represents a compiled symbol as a raw pointer.
/// This implementation of Artifact is designed for efficient in-memory execution
/// of compiled bytecode.
#[derive(Default, Debug, Clone)]
pub struct SymbolArtifact {
    /// The raw pointer to the entry point of the compiled symbol.
    /// This pointer represents the memory address where the compiled code begins.
    entry_ptr: usize,
    /// An instance of the Executor that produced this artifact.
    /// This field ensures that the Executor is kept alive as long as the SymbolArtifact exists,
    /// preventing the compiled code from being deallocated prematurely from the memory.
    executor: Executor,
}

impl Artifact for SymbolArtifact {
    /// Creates a new SymbolArtifact from an Executor.
    /// This method extracts the main entry point of the compiled code from the Executor
    /// and stores it along with the Executor itself.
    ///
    /// # Parameters
    /// * `executor`: The Executor instance containing the compiled code.
    ///
    /// # Returns
    /// A new SymbolArtifact instance.
    #[inline]
    fn new(executor: Executor) -> Self {
        Self {
            entry_ptr: executor.get_main_entrypoint_ptr() as usize,
            executor,
        }
    }

    /// Executes the compiled code represented by this artifact.
    /// This method demonstrates the primary advantage of the SymbolArtifact:
    /// direct execution of pre-compiled code without any additional compilation step.
    ///
    /// # Parameters
    /// * `runtime_context`: A mutable reference to the RuntimeContext, which provides
    ///   the execution environment and may be modified during execution.
    /// * `initial_gas`: The initial amount of gas allocated for this execution.
    ///
    /// # Returns
    /// A u8 value representing the execution result or error code.
    ///
    /// # Safety
    /// This method uses unsafe Rust features to convert a raw pointer to a function pointer.
    /// It assumes that the stored entry_ptr is valid and points to a correctly compiled
    /// function matching the MainFunc<DB> signature. Incorrect use could lead to undefined behavior.
    #[inline]
    fn execute(
        &self,
        runtime_context: &mut RuntimeContext,
        initial_gas: &mut u64,
        stack: &mut Stack,
        stack_size: &mut u64,
    ) -> u8 {
        match self.executor.kind {
            ExecuteKind::EVM => {
                let ptr = self.entry_ptr as *mut ();
                let func: EVMMainFunc = unsafe { std::mem::transmute(ptr) };
                func(runtime_context, initial_gas, stack, stack_size)
            }
            ExecuteKind::WASM(vm_ctx) => {
                let ptr = self.entry_ptr as *mut ();
                let func: WASMMainFunc = unsafe { std::mem::transmute(ptr) };
                func(vm_ctx);
                0
            }
        }
    }
}
