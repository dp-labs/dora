#[cfg(test)]
mod tests;

use anyhow::Result;
use dora_compiler::{
    context::Context,
    dora,
    evm::{self, program::Program, CompileOptions, EVMCompiler},
    pass, Compiler,
};
use dora_primitives::{spec::SpecId, Address, Bytecode};
use dora_runtime::artifact::Artifact;
use dora_runtime::executor::Executor;
use dora_runtime::{context::CallFrame, env::Env};
use dora_runtime::{context::RuntimeContext, host::DummyHost};
use dora_runtime::{
    db::{Database, MemoryDB},
    result::ResultAndState,
    transaction::Transaction,
};
use std::sync::{Arc, RwLock};

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
pub fn run_evm<DB: Database + 'static>(
    mut env: Env,
    db: DB,
    spec_id: SpecId,
) -> Result<ResultAndState> {
    env.validate_transaction().map_err(|e| anyhow::anyhow!(e))?;
    env.consume_intrinsic_cost()
        .map_err(|e| anyhow::anyhow!(e))?;
    let mut runtime_context = RuntimeContext::new(
        Arc::new(RwLock::new(db)),
        CallFrame::new(env.tx.caller),
        Arc::new(EVMTransaction::<DB>::new()),
        Arc::new(RwLock::new(DummyHost::new(env))),
        spec_id,
    );
    run_with_context(&mut runtime_context)
}

/// Run transaction with the runtime context.
pub fn run_with_context<DB: Database>(
    runtime_context: &mut RuntimeContext<DB>,
) -> Result<ResultAndState> {
    let host = runtime_context.host.read().unwrap();
    let env = host.env();
    let code_address = env.tx.get_address();
    let mut remaining_gas = env.tx.gas_limit;
    if code_address.is_zero() {
        let value = env.tx.value;
        let data = env.tx.data.clone();
        drop(host);
        // Here we get the conrtact create error from the runtime context.
        let _ = runtime_context.create_contract(&data, &mut remaining_gas, value, None);
    } else {
        // Fetch the bytecode artifact if exists
        let db = runtime_context.db.read().unwrap();
        let acc = db
            .basic(code_address)
            .map_err(|e| anyhow::anyhow!(format!("{e:?}")))?;
        let code_hash = acc.map(|acc| acc.code_hash).unwrap_or_default();
        let artifact = db.get_artifact(code_hash);
        drop(host);
        if let Ok(Some(artifact)) = artifact {
            drop(db);
            artifact.execute(runtime_context, remaining_gas);
        } else {
            let opcodes = db
                .code_by_hash(code_hash)
                .map_err(|e| anyhow::anyhow!(format!("{e:?}")))?;
            drop(db);
            // Compile the contract code
            let program = Program::from_opcode(&opcodes);
            let context = Context::new();
            let compiler = EVMCompiler::new(&context);
            let mut module = compiler.compile(
                &program,
                &(),
                &CompileOptions {
                    spec_id: runtime_context.inner_context.spec_id,
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
                },
            )?;
            pass::run(&context.mlir_context, &mut module.mlir_module)?;
            debug_assert!(module.mlir_module.as_operation().verify());
            let executor = Executor::new(module.module(), runtime_context, Default::default());
            executor.execute(runtime_context, remaining_gas);
            runtime_context
                .db
                .write()
                .unwrap()
                .set_artifact(code_hash, DB::Artifact::new(executor));
        }
    }
    runtime_context.get_result().map_err(|e| anyhow::anyhow!(e))
}

/// A specific implementation of the `Transaction` trait for executing EVM (Ethereum Virtual Machine) transactions.
///
/// `EVMTransaction` uses `RuntimeContext` for its execution context and returns a `Result` containing
/// `ResultAndState` after execution. This struct is designed to handle Ethereum-style transaction processing
/// by setting the initial gas limit and cloning the database state before running the EVM.
#[derive(Debug)]
pub struct EVMTransaction<DB: Database>(std::marker::PhantomData<DB>);

impl<DB: Database> EVMTransaction<DB> {
    #[inline]
    pub fn new() -> Self {
        Self(std::marker::PhantomData)
    }
}

impl<DB: Database> Default for EVMTransaction<DB> {
    fn default() -> Self {
        Self::new()
    }
}

impl<DB: Database> Transaction for EVMTransaction<DB> {
    type Context = RuntimeContext<DB>;
    type Result = Result<ResultAndState>;

    #[inline]
    fn run(&self, ctx: &mut Self::Context, _initial_gas: u64) -> Self::Result {
        run_with_context::<DB>(ctx)
    }
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
) -> Result<ResultAndState> {
    let opcodes = hex::decode(program)?;
    let calldata = hex::decode(calldata)?;
    let address = Address::from_low_u64_be(40);
    let mut env = Env::default();
    env.tx.transact_to = address;
    env.tx.gas_limit = initial_gas;
    env.tx.data = Bytecode::from(calldata);
    env.tx.caller = Address::from_low_u64_le(10000);
    let db = MemoryDB::new().with_contract(address, Bytecode::from(opcodes));
    run_evm(env, db, spec_id)
}
