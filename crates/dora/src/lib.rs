#[cfg(test)]
mod tests;

use dora_compiler::{
    context::Context,
    dora,
    evm::{self, program::Program, CompileOptions, EVMCompiler},
    pass, Compiler,
};
use dora_primitives::{spec::SpecId, Address, Bytecode};
use dora_runtime::context::RuntimeContext;
use dora_runtime::env::Env;
use dora_runtime::executor::Executor;
use dora_runtime::{
    artifact::Artifact,
    call::CallResult,
    context::VMContext,
    handler::{Frame, Handler},
    result::EVMError,
    vm::VM,
};
use dora_runtime::{
    db::{Database, MemoryDB},
    result::ResultAndState,
};
use std::sync::Arc;

/// Run the EVM environment with the given state database and return the execution result and final state.
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
pub fn run_evm<DB: Database + 'static>(
    env: Env,
    db: DB,
    spec_id: SpecId,
) -> Result<ResultAndState, EVMError> {
    VM::new(VMContext::new(
        db,
        env,
        spec_id,
        Handler {
            call_frame: Arc::new(call_frame),
        },
    ))
    .transact()
}

/// Default frame calling hanlder, using dora compiler and runtime to run EVM contract.
pub fn call_frame<DB: Database>(
    frame: Frame,
    ctx: &mut VMContext<'_, DB>,
) -> Result<CallResult, EVMError> {
    let code_hash = frame.contract.hash.unwrap_or_default();
    let spec_id = ctx.spec_id();
    let artifact = ctx.db.get_artifact(code_hash);
    let mut runtime_context = RuntimeContext::new(frame.contract, ctx, spec_id);
    if let Ok(Some(artifact)) = artifact {
        artifact.execute(&mut runtime_context, frame.gas_limit);
        Ok(CallResult::new_with_runtime_context_and_gas_limit(
            &runtime_context,
            frame.gas_limit,
        ))
    } else {
        let artifact = run_with_context::<DB>(&mut runtime_context, frame.gas_limit)
            .map_err(|e| EVMError::Custom(e.to_string()))?;
        let result =
            CallResult::new_with_runtime_context_and_gas_limit(&runtime_context, frame.gas_limit);
        drop(runtime_context);
        ctx.db.set_artifact(code_hash, artifact);
        Ok(result)
    }
}

/// Run transaction with the runtime context.
pub fn run_with_context<DB: Database>(
    runtime_context: &mut RuntimeContext,
    initial_gas: u64,
) -> anyhow::Result<DB::Artifact> {
    // Compile the contract code
    let program = Program::from_opcodes(
        &runtime_context.contract.code,
        runtime_context.inner.spec_id,
    );
    let context = Context::new();
    let compiler = EVMCompiler::new(&context);
    let mut module = compiler.compile(
        &program,
        &(),
        &CompileOptions {
            spec_id: runtime_context.inner.spec_id,
            ..Default::default()
        },
    )?;
    // Lowering the EVM dialect to MLIR builtin dialects.
    evm::pass::run(&context.mlir_context, &mut module.mlir_module)?;
    dora::pass::run(
        &context.mlir_context,
        &mut module.mlir_module,
        &dora::pass::PassOptions {
            program_code_size: program.code_size,
            spec_id: runtime_context.inner.spec_id,
            ..Default::default()
        },
    )?;
    pass::run(&context.mlir_context, &mut module.mlir_module)?;
    debug_assert!(module.mlir_module.as_operation().verify());
    let executor = Executor::new(module.module(), runtime_context, Default::default());
    executor.execute(runtime_context, initial_gas);
    Ok(DB::Artifact::new(executor))
}

/// Run hex-encoded EVM bytecode with custom calldata and return the execution result and final state.
///
/// # Arguments
///
/// * `program` - A string representing the hex-encoded EVM bytecode.
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
pub fn run_evm_bytecode_with_calldata(
    program: &str,
    calldata: &str,
    initial_gas: u64,
    spec_id: SpecId,
) -> anyhow::Result<ResultAndState> {
    let opcodes = hex::decode(program)?;
    let calldata = hex::decode(calldata)?;
    let address = Address::from_low_u64_be(40);
    let mut env = Env::default();
    env.tx.transact_to = address;
    env.tx.gas_limit = initial_gas;
    env.tx.data = Bytecode::from(calldata);
    env.tx.caller = Address::from_low_u64_le(10000);
    let db = MemoryDB::new().with_contract(address, Bytecode::from(opcodes));
    run_evm(env, db, spec_id).map_err(|err| anyhow::anyhow!(err))
}
