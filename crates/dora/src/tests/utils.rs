use std::{
    ops::Deref,
    sync::{Arc, RwLock},
};

use bytes::Bytes;
use dora_compiler::evm::{program::Operation, Program};
use dora_primitives::{spec::SpecId, Address, Bytecode, U256};
use dora_runtime::{
    context::{CallFrame, RuntimeContext},
    db::MemoryDB,
    env::Env,
    host::DummyHost,
    result::ExecutionResult,
};
use num_bigint::{BigInt, BigUint};

use crate::{run_evm, run_with_context, EVMTransaction};

use super::INIT_GAS;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct TestResult {
    pub result: ExecutionResult,
    pub memory: Vec<u8>,
}

impl Deref for TestResult {
    type Target = ExecutionResult;

    fn deref(&self) -> &Self::Target {
        &self.result
    }
}

pub(crate) fn biguint_256_from_bigint(value: BigInt) -> BigUint {
    if value >= BigInt::ZERO {
        value.magnitude().clone()
    } else {
        let bytes = value.to_signed_bytes_be();
        let mut buffer: Vec<u8> = vec![255_u8; 32];
        let finish = 32;
        let start = finish - bytes.len();
        buffer[start..finish].copy_from_slice(&bytes);
        BigUint::from_bytes_be(&buffer)
    }
}

#[inline]
pub(crate) fn run_result(operations: Vec<Operation>) -> TestResult {
    run_result_with_spec(operations, SpecId::CANCUN)
}

pub(crate) fn run_result_with_spec(operations: Vec<Operation>, spec_id: SpecId) -> TestResult {
    let (mut env, db) = default_env_and_db_setup(operations);
    env.tx.data = Bytes::from_static(&[0xCC; 64]);
    let mut runtime_context = RuntimeContext::new(
        Arc::new(RwLock::new(db)),
        CallFrame::new_with_data(env.tx.caller, vec![0xDD; 64]),
        Arc::new(EVMTransaction::<MemoryDB>::new()),
        Arc::new(RwLock::new(DummyHost::new(env))),
        spec_id,
    );
    run_with_context(&mut runtime_context).unwrap();
    let result = runtime_context.get_result().unwrap().result;
    TestResult {
        result,
        memory: runtime_context.memory().to_owned(),
    }
}

pub(crate) fn default_env_and_db_setup(operations: Vec<Operation>) -> (Env, MemoryDB) {
    let mut env = Env::default();
    env.tx.gas_limit = INIT_GAS;
    let program = Program::from(operations);
    let (address, bytecode) = (
        Address::from_low_u64_be(40),
        Bytecode::from(program.to_opcode()),
    );
    env.tx.transact_to = address;
    env.block.coinbase = Address::from_low_u64_be(80);
    let mut db = MemoryDB::new().with_contract(address, bytecode);
    db.set_balance(address, U256::from(10));
    (env, db)
}

pub(crate) fn run_program_assert_num_result(env: Env, db: MemoryDB, expected_result: BigUint) {
    let result = run_evm(env, db, SpecId::CANCUN).unwrap().result;
    assert!(result.is_success(), "{:?}", result);
    let result_data = BigUint::from_bytes_be(result.output().unwrap_or(&Bytes::new()));
    assert_eq!(result_data, expected_result);
}

pub(crate) fn run_program_assert_halt(env: Env, db: MemoryDB) {
    let result = run_evm(env, db, SpecId::CANCUN).unwrap().result;
    assert!(result.is_halt());
}

pub(crate) fn run_program_assert_revert(env: Env, db: MemoryDB) {
    let result = run_evm(env, db, SpecId::CANCUN).unwrap().result;
    assert!(result.is_revert());
}
