use dora_compiler::evm::{Program, program::Operation};
use dora_primitives::{Address, Bytecode, Bytes, Env, Log, TxKind, U256, spec::SpecId};
use dora_runtime::{
    ExitStatusCode,
    context::{Contract, RuntimeContext},
    db::MemoryDB,
    host::{DummyHost, Host},
};
use num_bigint::{BigInt, BigUint};

use crate::{run, run_with_context};

use super::INIT_GAS;

#[derive(Debug, Clone)]
pub(crate) struct TestResult {
    pub status: ExitStatusCode,
    pub host: DummyHost,
    pub gas_used: u64,
    pub output: Vec<u8>,
}

impl TestResult {
    pub fn sload(&mut self, key: U256) -> U256 {
        let result = self.host.sload(Address::default(), key).unwrap_or_default();
        result.data
    }

    pub fn tload(&mut self, key: U256) -> U256 {
        self.host.tload(Address::default(), key)
    }

    pub fn logs(&self) -> &[Log] {
        &self.host.logs
    }

    pub fn gas_used(&self) -> u64 {
        self.gas_used
    }

    pub fn output(&self) -> Bytes {
        self.output.to_owned().into()
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
    let mut env = Env::default();
    env.tx.gas_limit = INIT_GAS;
    env.tx.data = Bytes::from_static(&[0xCC; 64]);
    env.cfg.spec = spec_id;
    let initial_gas = env.tx.gas_limit;
    let contract = Contract::new_with_env(
        &env,
        Bytecode::new_raw(Program::operations_to_opcode(&operations).into()),
        None,
    );
    let mut host = DummyHost::new(env);
    let mut runtime_context =
        RuntimeContext::new(contract, 1, false, false, &mut host, spec_id, initial_gas);
    runtime_context.set_returndata(vec![0xDD; 64]);
    let result = run_with_context::<MemoryDB>(runtime_context).unwrap();
    TestResult {
        status: result.status,
        output: result.output.to_vec(),
        gas_used: initial_gas - result.gas_remaining,
        host,
    }
}

pub(crate) fn default_env_and_db_setup(operations: Vec<Operation>) -> (Env, MemoryDB) {
    let mut env = Env::default();
    env.tx.gas_limit = INIT_GAS;
    env.block.gas_limit = INIT_GAS;
    let program = Program::from_operations(operations, false);
    let (address, bytecode) = (
        Address::left_padding_from(&[40]),
        Bytes::from(program.to_opcode()),
    );
    env.tx.kind = TxKind::Call(address);
    env.block.beneficiary = Address::left_padding_from(&[80]);
    env.cfg.spec = SpecId::CANCUN;
    let mut db = MemoryDB::new().with_contract(address, Bytecode::new_raw(bytecode));
    db.set_balance(address, U256::from(10));
    (env, db)
}

/// Asserts program execution result as `success`.
///
/// Asserts returned [`BigUint`] number result.
pub(crate) fn run_program_assert_num_result(env: Env, db: MemoryDB, expected_result: BigUint) {
    let result = run(env, db).unwrap();
    assert!(result.is_success(), "{:?}", result);
    let result_data = BigUint::from_bytes_be(result.output().unwrap_or(&Bytes::new()));
    assert_eq!(result_data, expected_result);
}

/// Asserts program execution result as `success`.
///
/// Asserts returned [`Bytes`] result.
pub(crate) fn run_program_assert_bytes_result(env: Env, db: MemoryDB, expected_result: Bytes) {
    let result = run(env, db).unwrap();
    assert!(result.is_success(), "{:?}", result);
    let result_data = result.output().unwrap_or(&Bytes::new()).clone();
    assert_eq!(result_data, expected_result);
}

/// Asserts program execution result as `halt`.
pub(crate) fn run_program_assert_halt(env: Env, db: MemoryDB) {
    let result = run(env, db).unwrap();
    assert!(result.is_halt());
}

/// Asserts program execution result as `revert`.
pub(crate) fn run_program_assert_revert(env: Env, db: MemoryDB) {
    let result = run(env, db).unwrap();
    assert!(!result.is_halt());
    assert!(!result.is_success());
}
