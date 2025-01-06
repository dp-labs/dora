use std::sync::Arc;

use dora_compiler::evm::{program::Operation, Program};
use dora_primitives::{spec::SpecId, Address, Bytecode, Bytes, Bytes32, Eof, U256};
use dora_runtime::{
    context::{Contract, Log, RuntimeContext},
    db::MemoryDB,
    env::{Env, TxKind},
    host::{DummyHost, Host},
    ExitStatusCode,
};
use num_bigint::{BigInt, BigUint};

use crate::{run_evm, run_with_context};

use super::INIT_GAS;

#[derive(Debug, Clone)]
pub(crate) struct TestResult {
    pub status: ExitStatusCode,
    pub memory: Vec<u8>,
    pub host: DummyHost,
    pub gas_used: u64,
    pub output: Vec<u8>,
}

impl TestResult {
    pub fn sload(&mut self, key: U256) -> U256 {
        let result = self
            .host
            .sload(Address::default(), Bytes32::from_u256(key))
            .unwrap_or_default();
        result.data.to_u256()
    }

    pub fn tload(&mut self, key: U256) -> U256 {
        let result = self.host.tload(Address::default(), Bytes32::from_u256(key));
        result.to_u256()
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
    let initial_gas = env.tx.gas_limit;
    let contract = Contract::new_with_env(
        &env,
        Bytecode::new_raw(
            Program {
                operations,
                code_size: 0,
                is_eof: false,
            }
            .to_opcode()
            .into(),
        ),
        None,
    );
    let mut host = DummyHost::new(env);
    let mut runtime_context = RuntimeContext::new(contract, 1, false, false, &mut host, spec_id);
    runtime_context.set_returndata(vec![0xDD; 64]);
    run_with_context::<MemoryDB>(&mut runtime_context, initial_gas).unwrap();
    TestResult {
        status: runtime_context.status(),
        memory: runtime_context.memory().to_owned(),
        output: runtime_context.return_values().to_vec(),
        gas_used: initial_gas - runtime_context.gas_remaining(),
        host,
    }
}

#[inline]
pub(crate) fn run_result_eof(eof: Eof) -> TestResult {
    run_result_with_spec_eof(eof, SpecId::PRAGUE)
}

pub(crate) fn run_result_with_spec_eof(eof: Eof, spec_id: SpecId) -> TestResult {
    let mut env = Env::default();
    env.tx.gas_limit = INIT_GAS;
    env.tx.data = Bytes::from_static(&[0xCC; 64]);
    let initial_gas = env.tx.gas_limit;
    let contract = Contract::new_with_env(&env, Bytecode::Eof(Arc::new(eof)), None);
    let mut host = DummyHost::new(env);
    let mut runtime_context = RuntimeContext::new(contract, 1, false, false, &mut host, spec_id);
    runtime_context.set_returndata(vec![0xDD; 64]);
    run_with_context::<MemoryDB>(&mut runtime_context, initial_gas).unwrap();
    TestResult {
        status: runtime_context.status(),
        memory: runtime_context.memory().to_owned(),
        output: runtime_context.return_values().to_vec(),
        gas_used: initial_gas - runtime_context.gas_remaining(),
        host,
    }
}

pub(crate) fn default_env_and_db_setup(operations: Vec<Operation>) -> (Env, MemoryDB) {
    let mut env = Env::default();
    env.tx.gas_limit = INIT_GAS;
    env.block.gas_limit = Bytes32::from(INIT_GAS).into_u256();
    let program = Program {
        operations,
        code_size: 0,
        is_eof: false,
    };
    let (address, bytecode) = (
        Address::left_padding_from(&[40]),
        Bytes::from(program.to_opcode()),
    );
    env.tx.transact_to = TxKind::Call(address);
    env.block.coinbase = Address::left_padding_from(&[80]);
    let mut db = MemoryDB::new().with_contract(address, Bytecode::new_raw(bytecode));
    db.set_balance(address, U256::from(10));
    (env, db)
}

pub(crate) fn default_env_and_db_setup_eof(eof: Eof) -> (Env, MemoryDB) {
    let mut env = Env::default();
    env.tx.gas_limit = INIT_GAS;
    env.block.gas_limit = Bytes32::from(INIT_GAS).into_u256();
    let (address, bytecode) = (
        Address::left_padding_from(&[40]),
        Bytecode::Eof(Arc::new(eof)),
    );
    env.tx.transact_to = TxKind::Call(address);
    env.block.coinbase = Address::left_padding_from(&[80]);
    let mut db = MemoryDB::new().with_contract(address, bytecode);
    db.set_balance(address, U256::from(10));
    (env, db)
}

pub(crate) fn run_program_assert_num_result(
    env: Env,
    db: MemoryDB,
    spec_id: SpecId,
    expected_result: BigUint,
) {
    let result = run_evm(env, db, spec_id).unwrap().result;
    assert!(result.is_success(), "{:?}", result);
    let result_data = BigUint::from_bytes_be(result.output().unwrap_or(&Bytes::new()));
    assert_eq!(result_data, expected_result);
}

pub(crate) fn run_program_assert_halt(env: Env, db: MemoryDB, spec_id: SpecId) {
    let result = run_evm(env, db, spec_id).unwrap().result;
    assert!(result.is_halt());
}

pub(crate) fn run_program_assert_revert(env: Env, db: MemoryDB, spec_id: SpecId) {
    let result = run_evm(env, db, spec_id).unwrap().result;
    assert!(result.is_revert());
}
