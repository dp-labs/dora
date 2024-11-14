#[cfg(test)]
mod tests;

use anyhow::Result;
use bytes::Bytes;
use dora_compiler::{
    context::Context,
    dora,
    evm::{self, program::Program, EVMCompiler},
    pass, Compiler,
};
use dora_primitives::transaction::Transaction;
use dora_primitives::{
    db::{Database, MemoryDb},
    Address, Bytecode,
};
use dora_runtime::executor::Executor;
use dora_runtime::journal::Journal;
use dora_runtime::result::ResultAndState;
use dora_runtime::{context::CallFrame, env::Env};
use dora_runtime::{context::RuntimeContext, host::DummyHost};
use std::{
    hint::black_box,
    sync::{Arc, RwLock},
};

/// Run the EVM environment with the given state database and return the execution result and final state.
///
/// # Arguments
///
/// * `env` - The environment configuration for the execution (e.g., gas limit, transaction data, etc.).
/// * `db` - A mutable reference to the `MemoryDb` that simulates the contract storage and state database.
///
/// # Returns
///
/// Returns `ResultAndState`, containing the execution result and the final state after execution.
///
/// # Errors
///
/// Returns an error if the program fails to execute or if the bytecode or address is invalid.
pub fn run_evm(env: Env, db: MemoryDb) -> Result<ResultAndState> {
    let mut runtime_context = RuntimeContext::new(
        Journal::new(db),
        CallFrame::new(env.tx.caller),
        Arc::new(EVMTransaction),
        Arc::new(RwLock::new(DummyHost::new(env))),
    );
    run_with_context(&mut runtime_context)
}

/// Run transaction with the runtime context.
fn run_with_context(runtime_context: &mut RuntimeContext) -> Result<ResultAndState> {
    let host = runtime_context.host.read().unwrap();
    let code_address = host.env().tx.get_address();
    let mut remaining_gas = host.env().tx.gas_limit;
    if code_address.is_zero() {
        let value = host.env().tx.value;
        let data = host.env().tx.data.clone();
        drop(host);
        runtime_context.create_contract(&data, &mut remaining_gas, value, None)?;
    } else {
        drop(host);
        let opcodes = runtime_context.journal.db.code_by_address(code_address)?;
        let program = Program::from_opcode(&opcodes);
        let context = Context::new();
        let compiler = EVMCompiler::new(&context);
        let mut module = compiler.compile(&program, &())?;
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
    }
    runtime_context.get_result().map_err(|e| anyhow::anyhow!(e))
}

/// A specific implementation of the `Transaction` trait for executing EVM (Ethereum Virtual Machine) transactions.
///
/// `EVMTransaction` uses `RuntimeContext` for its execution context and returns a `Result` containing
/// `ResultAndState` after execution. This struct is designed to handle Ethereum-style transaction processing
/// by setting the initial gas limit and cloning the database state before running the EVM.
///
/// # Example
/// ```no_check
/// use dora_primitives::transaction::Transaction;
/// use dora_runtime::context::RuntimeContext;
/// use dora::EVMTransaction;
///
/// let mut ctx = RuntimeContext::new();
/// let evm_tx = EVMTransaction::default();
/// let result = evm_tx.run(&mut ctx, 21_000);
/// ```
#[derive(Debug, Default)]
pub struct EVMTransaction;

impl Transaction for EVMTransaction {
    type Context = RuntimeContext;
    type Result = Result<ResultAndState>;

    #[inline]
    fn run(&self, ctx: &mut Self::Context, _initial_gas: u64) -> Self::Result {
        run_with_context(ctx)
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
    calldata: &[u8],
    initial_gas: u64,
) -> Result<ResultAndState> {
    let opcodes = hex::decode(program)?;
    let program = Program::from_opcode(&opcodes);
    run_evm_program(&program, calldata, initial_gas)
}

/// Run EVM bytecode from a hex-encoded string and return the execution result and final state.
///
/// # Arguments
///
/// * `program` - A string representing the hex-encoded EVM bytecode.
/// * `data` - A 32-bit integer that will be converted into calldata for the execution.
/// * `initial_gas` - The initial amount of gas allocated for the execution.
///
/// # Returns
///
/// Returns `ResultAndState`, containing the execution result and the final state after execution.
///
/// # Errors
///
/// Returns an error if the bytecode fails to decode or execute.
pub fn run_evm_bytecode(program: &str, data: u32, initial_gas: u64) -> Result<ResultAndState> {
    let mut calldata = vec![0x00; 32];
    calldata[28..32].copy_from_slice(&data.to_be_bytes());
    run_evm_bytecode_with_calldata(program, &calldata, initial_gas)
}

/// Run an EVM program and return the execution result and final state.
///
/// # Arguments
///
/// * `program` - A reference to the `Program` struct representing the EVM program to execute.
/// * `calldata` - A byte slice representing the calldata for the transaction.
/// * `initial_gas` - The initial amount of gas allocated for the execution.
///
/// # Returns
///
/// Returns `ResultAndState`, containing the execution result and the final state after execution.
///
/// # Errors
///
/// Returns an error if the program fails to compile, transform, or execute.
pub fn run_evm_program(
    program: &Program,
    calldata: &[u8],
    initial_gas: u64,
) -> Result<ResultAndState> {
    let context = Context::new();
    let compiler = EVMCompiler::new(&context);
    let mut module = compiler.compile(program, &())?;
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
    let (address, bytecode) = (
        Address::from_low_u64_be(40),
        Bytecode::from(program.to_opcode()),
    );
    // New the EVM run environment.
    let mut env: Env = Default::default();
    env.tx.gas_limit = 999_999;
    env.tx.data = Bytes::from(calldata.to_vec());
    env.tx.transact_to = address;
    let journal = Journal::new(MemoryDb::default().with_contract(address, bytecode));
    let mut context = RuntimeContext::new(
        journal,
        CallFrame::new(Address::from_low_u64_le(10000)),
        Arc::new(EVMTransaction),
        Arc::new(RwLock::new(DummyHost::new(env))),
    );
    let executor = Executor::new(module.module(), &context, Default::default());
    black_box(executor.execute(black_box(&mut context), black_box(initial_gas)));
    context.get_result().map_err(|e| anyhow::anyhow!(e))
}
