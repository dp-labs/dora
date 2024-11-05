mod runners;
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
use dora_primitives::{
    db::{Database, MemoryDb},
    Address, Bytecode,
};
use dora_runtime::executor::Executor;
use dora_runtime::journal::Journal;
use dora_runtime::result::ResultAndState;
use dora_runtime::{context::CallFrame, env::Env};
use dora_runtime::{context::RuntimeContext, env::TransactTo};
use std::hint::black_box;

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
    env.tx.transact_to = TransactTo::Call(address);
    let mut db = MemoryDb::default().with_contract(address, bytecode);
    let journal = Journal::new(&mut db);
    let mut context = RuntimeContext::new(
        env,
        journal,
        CallFrame::new(Address::from_low_u64_le(10000)),
    );
    let executor = Executor::new(module.module(), &context, Default::default());
    black_box(executor.execute(black_box(&mut context), black_box(initial_gas)));
    context.get_result().map_err(|e| anyhow::anyhow!(e))
}

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
pub fn run_evm(env: Env, db: &mut MemoryDb) -> Result<ResultAndState> {
    let code_address = env.tx.get_address();
    let opcodes = db.code_by_address(code_address)?;
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
    let journal = Journal::new(db);
    let gas_limit = env.tx.gas_limit;
    let mut context = RuntimeContext::new(
        env,
        journal,
        CallFrame::new(Address::from_low_u64_le(10000)),
    );
    let executor = Executor::new(module.module(), &context, Default::default());
    black_box(executor.execute(black_box(&mut context), black_box(gas_limit)));
    context.get_result().map_err(|e| anyhow::anyhow!(e))
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
