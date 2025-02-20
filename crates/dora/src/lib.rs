#[cfg(test)]
mod tests;

pub use dora_compiler as compiler;
pub use dora_ir as ir;
pub use dora_primitives as primitives;
pub use dora_runtime as runtime;

pub use dora_compiler::{
    context::Context,
    dora,
    evm::{self, program::Program, EVMCompileOptions, EVMCompiler},
    pass,
    wasm::{self, WASMCompileOptions, WASMCompiler},
    Compiler,
};
pub use dora_primitives::{spec::SpecId, Bytecode, Bytes, Bytes32, EVMBytecode, WASMBytecode};
pub use dora_runtime::executor::{ExecuteKind, Executor};
pub use dora_runtime::{
    artifact::Artifact,
    call::CallResult,
    context::VMContext,
    handler::{Frame, Handler},
    result::{ExecutionResult, VMError},
    vm::VM,
};
pub use dora_runtime::{context::RuntimeContext, env::TxKind};
pub use dora_runtime::{
    db::{Database, MemoryDB},
    result::ResultAndState,
};
pub use dora_runtime::{env::Env, stack::Stack};
use std::sync::Arc;

/// Run EVM or WASM with the environment configuration for the execution, given state database and return the execution result and final state.
///
/// # Arguments
///
/// * `env` - The environment configuration for the execution (e.g., gas limit, transaction data, etc.).
/// * `db` - A mutable reference to the `MemoryDB` that simulates the contract storage and state database.
///
/// # Returns
///
/// Returns `ResultAndState`, containing the execution result and the final state after execution.
///
/// # Errors
///
/// Returns an error if the program fails to execute or if the bytecode or address is invalid.
#[inline]
pub fn run<DB: Database + 'static>(
    env: Env,
    db: DB,
    spec_id: SpecId,
) -> Result<ExecutionResult, VMError> {
    VM::new(VMContext::new(db, env, spec_id, compile_handler())).transact_commit()
}

/// Compile Handler for the VM.
#[inline]
pub fn compile_handler<'a, DB: Database + 'a>() -> Handler<'a, DB> {
    Handler {
        call_handler: Arc::new(compile_call_handler),
    }
}

/// Default frame calling hanlder, using dora compiler and runtime to run EVM and WASM contract.
fn compile_call_handler<DB: Database>(
    frame: Frame,
    ctx: &mut VMContext<'_, DB>,
) -> Result<CallResult, VMError> {
    let code_hash = frame.contract.hash.unwrap_or_default();
    let spec_id = ctx.spec_id();
    let artifact = ctx.db.get_artifact(code_hash);
    let artifact = if let Ok(Some(artifact)) = artifact {
        artifact
    } else {
        let artifact = build_artifact::<DB>(&frame.contract.code, ctx.spec_id())
            .map_err(|e| VMError::Compile(e.to_string()))?;
        ctx.db.set_artifact(code_hash, artifact.clone());
        artifact
    };
    let runtime_context = RuntimeContext::new(
        frame.contract,
        frame.depth,
        frame.is_static,
        frame.is_eof_init,
        ctx,
        spec_id,
        frame.gas_limit,
    );
    artifact
        .execute(runtime_context)
        .map_err(|err| VMError::Handler(err.to_string()))
}

/// Run hex-encoded EVM or WASM bytecode with custom calldata and return the execution result and final state.
///
/// # Arguments
///
/// * `program` - A string representing the hex-encoded EVM or WASM bytecode.
/// * `calldata` - A byte slice containing the custom calldata to use for the execution.
/// * `initial_gas` - The initial amount of gas allocated for the execution.
///
/// # Returns
///
/// Returns `ResultAndState`, containing the execution result and the final state after execution.
///
/// # Errors
///
/// Returns an error if the bytecode fails to decode or execute.
pub fn run_bytecode_hex(
    program: &str,
    calldata: &str,
    initial_gas: u64,
    spec_id: SpecId,
) -> anyhow::Result<ExecutionResult> {
    let opcodes = hex::decode(program)?;
    let calldata = hex::decode(calldata)?;
    let address = Bytes32::from(40_u32).to_address();
    let mut env = Env::default();
    env.tx.transact_to = TxKind::Call(address);
    env.tx.gas_limit = initial_gas;
    env.tx.data = Bytes::from(calldata);
    env.tx.caller = Bytes32::from(10000_u32).to_address();
    let db = MemoryDB::new().with_contract(address, Bytecode::new(Bytes::from(opcodes)));
    run(env, db, spec_id).map_err(|err| anyhow::anyhow!(err))
}

/// Run transaction with the runtime context.
pub fn run_with_context<DB: Database>(
    runtime_context: RuntimeContext,
) -> anyhow::Result<CallResult> {
    let artifact: DB::Artifact = build_artifact::<DB>(
        &runtime_context.contract.code,
        runtime_context.inner.spec_id,
    )?;
    artifact.execute(runtime_context)
}

/// Build the EVM or WASM bytecode to the native artifact.
pub fn build_artifact<DB: Database>(
    code: &Bytecode,
    spec_id: SpecId,
) -> anyhow::Result<DB::Artifact> {
    match code {
        Bytecode::EVM(code) => build_evm_artifact::<DB>(code, spec_id),
        Bytecode::WASM(code) => build_wasm_artifact::<DB>(code),
    }
}

/// Build the EVM bytecode to the artifact
pub fn build_evm_artifact<DB: Database>(
    code: &EVMBytecode,
    spec_id: SpecId,
) -> anyhow::Result<DB::Artifact> {
    // Compile the contract code
    let program = Program::from_opcodes(code.bytecode(), code.eof().cloned());
    let context = Context::new();
    let compiler = EVMCompiler::new(
        &context,
        EVMCompileOptions {
            spec_id,
            ..Default::default()
        },
    );
    let mut module = compiler.compile(&program)?;
    // Lowering the EVM dialect to MLIR builtin dialects.
    evm::pass::run(&context.mlir_context, &mut module.mlir_module)?;
    dora::pass::run(
        &context.mlir_context,
        &mut module.mlir_module,
        &dora::pass::PassOptions {
            code_size: program.code_size(),
            spec_id,
            ..Default::default()
        },
    )?;
    pass::run(&context.mlir_context, &mut module.mlir_module)?;
    debug_assert!(module.mlir_module.as_operation().verify());
    let executor = Executor::new(module.module(), Default::default(), ExecuteKind::EVM);
    Ok(DB::Artifact::new(executor))
}

/// Build WASM opcode to the artifact
pub fn build_wasm_artifact<DB: Database>(code: &WASMBytecode) -> anyhow::Result<DB::Artifact> {
    let context = Context::new();
    let compiler = WASMCompiler::new(&context, WASMCompileOptions::default());
    // Compile WASM Bytecode to MLIR WASM Dialect
    let mut module = compiler.compile(code)?;
    let instance = compiler.build_instance(code)?;
    // Lowering the WASM dialect to the Dora dialect.
    wasm::pass::run(&context.mlir_context, &mut module.mlir_module)?;
    // Lowering the Dora dialect to MLIR builtin dialects.
    dora::pass::run(
        &context.mlir_context,
        &mut module.mlir_module,
        &dora::pass::PassOptions {
            code_size: code.len() as u32,
            ..Default::default()
        },
    )?;
    pass::run(&context.mlir_context, &mut module.mlir_module)?;
    debug_assert!(module.mlir_module.as_operation().verify());

    let executor = Executor::new(
        module.module(),
        Default::default(),
        ExecuteKind::new_wasm(instance),
    );
    Ok(DB::Artifact::new(executor))
}
