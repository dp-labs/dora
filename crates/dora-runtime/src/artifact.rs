use crate::{
    call::CallResult,
    constants::MAIN_ENTRYPOINT,
    context::{Contract, EVMMainFunc, RuntimeContext},
    executor::{ExecuteKind, Executor},
    host::DummyHost,
    stack::Stack,
    wasm::context::{set_runtime_context, with_runtime_context},
};
use anyhow::{anyhow, Result};
use dora_primitives::SpecId;
use std::fmt::Debug;
use std::panic::{catch_unwind, AssertUnwindSafe};
use wasmer_vm::VMContext;

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
    fn execute(&self, runtime_context: RuntimeContext) -> Result<CallResult>;
}

/// A memory artifact that represents a compiled symbol as a raw pointer.
/// This implementation of Artifact is designed for efficient in-memory execution
/// of compiled bytecode.
#[derive(Default, Debug, Clone)]
pub struct SymbolArtifact {
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
        Self { executor }
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
    fn execute(&self, mut context: RuntimeContext) -> Result<CallResult> {
        let ptr = self.executor.get_main_entrypoint_ptr();
        if ptr.is_null() {
            return Err(anyhow::anyhow!("function main not found"));
        }
        match &self.executor.kind {
            ExecuteKind::EVM => {
                let mut initial_gas = context.gas_limit();
                let func: EVMMainFunc = unsafe { std::mem::transmute(ptr) };
                func(&mut context, &mut initial_gas, &mut Stack::new(), &mut 0);
                Ok(CallResult {
                    status: context.status(),
                    gas_limit: context.gas_limit(),
                    gas_remaining: context.gas_remaining(),
                    gas_refunded: context.gas_refunded(),
                    output: context.return_bytes(),
                    create_address: None,
                })
            }
            ExecuteKind::WASM(_) => {
                // Note: default WASM entrypoint is `fn main() -> ()`, no args and return values.
                let (_, result): ((), _) =
                    self.execute_wasm_func_with_context_result(MAIN_ENTRYPOINT, (), context)?;
                Ok(result)
            }
        }
    }
}

impl SymbolArtifact {
    /// Executes a WASM function by name with the given arguments.
    ///
    /// # Arguments
    /// * `name` - The name of the WASM function to execute.
    /// * `args` - The arguments to pass to the WASM function.
    ///
    /// # Returns
    /// * `Result<Ret>` - The result of the WASM function execution, or an error if the function fails.
    ///
    /// # Safety
    /// This function uses `unsafe` to transmute a function pointer, which is inherently unsafe.
    /// Ensure that the function pointer is valid and that the arguments and return types match the expected types.
    #[inline]
    pub fn execute_wasm_func<Args, Ret>(&self, name: &str, args: Args) -> Result<Ret>
    where
        Args: Sized,
        Ret: Sized,
    {
        let mut host = DummyHost::default();
        self.execute_wasm_func_with_context(
            name,
            args,
            RuntimeContext::new(
                Contract::default(),
                1,
                false,
                false,
                &mut host,
                SpecId::default(),
                u64::MAX,
            ),
        )
    }

    /// Executes a WASM function by name with the given arguments.
    ///
    /// # Arguments
    /// * `name` - The name of the WASM function to execute.
    /// * `args` - The arguments to pass to the WASM function.
    ///
    /// # Returns
    /// * `Result<Ret>` - The result of the WASM function execution, or an error if the function fails.
    ///
    /// # Safety
    /// This function uses `unsafe` to transmute a function pointer, which is inherently unsafe.
    /// Ensure that the function pointer is valid and that the arguments and return types match the expected types.
    #[inline]
    pub fn execute_wasm_func_with_calldata<Args, Ret, T>(
        &self,
        name: &str,
        args: Args,
        calldata: T,
    ) -> Result<Ret>
    where
        Args: Sized,
        Ret: Sized,
        T: AsRef<[u8]>,
    {
        let mut host = DummyHost::default();
        self.execute_wasm_func_with_context(
            name,
            args,
            RuntimeContext::new(
                Contract::new_with_calldata(calldata),
                1,
                false,
                false,
                &mut host,
                SpecId::default(),
                u64::MAX,
            ),
        )
    }

    /// Executes the WASM compiled code represented by this artifact with the runtime context.
    ///
    /// # Safety
    /// This function uses `unsafe` to transmute a function pointer, which is inherently unsafe.
    /// Ensure that the function pointer is valid and that the arguments and return types match the expected types.
    #[inline]
    pub fn execute_wasm_func_with_context<Args, Ret>(
        &self,
        name: &str,
        args: Args,
        runtime_context: RuntimeContext,
    ) -> Result<Ret>
    where
        Args: Sized,
        Ret: Sized,
    {
        let (ret, _) = self.execute_wasm_func_with_context_result(name, args, runtime_context)?;
        Ok(ret)
    }

    /// Executes the WASM compiled code represented by this artifact with the runtime context and result.
    ///
    /// # Safety
    /// This function uses `unsafe` to transmute a function pointer, which is inherently unsafe.
    /// Ensure that the function pointer is valid and that the arguments and return types match the expected types.
    pub fn execute_wasm_func_with_context_result<Args, Ret>(
        &self,
        name: &str,
        args: Args,
        runtime_context: RuntimeContext,
    ) -> Result<(Ret, CallResult)>
    where
        Args: Sized,
        Ret: Sized,
    {
        let closure: Box<dyn FnOnce() -> Result<(Ret, CallResult)>> = Box::new(move || {
            let func_ptr = self.executor.lookup(name);
            if func_ptr.is_null() {
                return Err(anyhow::anyhow!(
                    "function {name} not found in the WASM module"
                ));
            }
            match &self.executor.kind {
                ExecuteKind::EVM => Err(anyhow!(
                    "The compiled code kind is EVM, and it's not WASM kind"
                )),
                ExecuteKind::WASM(vm_inst) => set_runtime_context(runtime_context, || {
                    let func: fn(*mut VMContext, Args) -> Ret =
                        unsafe { std::mem::transmute(func_ptr) };
                    let func_result = func(vm_inst.read().vmctx_ptr(), args);
                    let call_result = with_runtime_context(|runtime_context| {
                        CallResult::new_with_runtime_context(runtime_context)
                    });
                    Ok((func_result, call_result))
                }),
            }
        });
        let prev_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        let result = catch_unwind(AssertUnwindSafe(closure));
        std::panic::set_hook(prev_hook);
        match result {
            Ok(result) => result,
            Err(err) => Err(anyhow::anyhow!(crate::wasm::trap::err_to_str(err))),
        }
    }
}
