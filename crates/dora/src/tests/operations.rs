use std::str::FromStr;

use crate::{run_with_context, tests::INIT_GAS};
use dora_compiler::evm::program::{Operation, Program};
use dora_primitives::spec::SpecId;
use dora_primitives::{Address, Bytecode, Bytes, Bytes32, Eof, EofBody, B256, U256};
use dora_runtime::account::EMPTY_CODE_HASH_BYTES;
use dora_runtime::context::Contract;
use dora_runtime::env::TxKind;
use dora_runtime::host::DummyHost;
use dora_runtime::{context::RuntimeContext, db::MemoryDB, env::Env};
use num_bigint::{BigInt, BigUint};

use super::utils::{
    biguint_256_from_bigint, default_env_and_db_setup, default_env_and_db_setup_eof,
    run_program_assert_halt, run_program_assert_num_result, run_program_assert_revert,
};

const CREATE_ADDRESS_U256_STR: &str = "1145609038113382871769568181405607467656660548686";

#[test]
fn add() {
    let (a, b) = (BigUint::from(10_u8), BigUint::from(10_u8));
    let operations = vec![
        Operation::Push((1_u8, b.clone())),
        Operation::Push((1_u8, a.clone())),
        Operation::Add,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, a + b);
}

#[test]
fn add_overflow_u64() {
    let (a, b) = (BigUint::from(u64::MAX), BigUint::from(1_u8));
    let operations = vec![
        Operation::Push((32_u8, b.clone())),
        Operation::Push((32_u8, a.clone())),
        Operation::Add,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, a + b);
}

#[test]
fn add_overflow_u128() {
    let (a, b) = (BigUint::from(u128::MAX), BigUint::from(1_u8));
    let operations = vec![
        Operation::Push((32_u8, b.clone())),
        Operation::Push((32_u8, a.clone())),
        Operation::Add,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, a + b);
}

#[test]
fn mul() {
    let (a, b) = (BigUint::from(10_u8), BigUint::from(10_u8));
    let operations = vec![
        Operation::Push((1_u8, b.clone())),
        Operation::Push((1_u8, a.clone())),
        Operation::Mul,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, a * b);
}

#[test]
fn mul_large() {
    let (a, b) = (BigUint::from(123456789_u128), BigUint::from(987654321_u128));
    let operations = vec![
        Operation::Push((32_u8, b.clone())),
        Operation::Push((32_u8, a.clone())),
        Operation::Mul,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, a * b);
}

#[test]
fn mul_overflow() {
    let (a, b) = (BigUint::from(u64::MAX), BigUint::from(2_u64));
    let operations = vec![
        Operation::Push((32_u8, b.clone())),
        Operation::Push((32_u8, a.clone())),
        Operation::Mul,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, a * b);
}

#[test]
fn sub() {
    let (a, b) = (BigUint::from(10_u8), BigUint::from(5_u8));
    let operations = vec![
        Operation::Push((1_u8, b.clone())),
        Operation::Push((1_u8, a.clone())),
        Operation::Sub,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, a - b);
}

#[test]
fn sub_underflow() {
    let (a, b) = (BigUint::from(0_u8), BigUint::from(10_u8));
    let operations = vec![
        Operation::Push((1_u8, b.clone())),
        Operation::Push((1_u8, a.clone())),
        Operation::Sub,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(
        env,
        db,
        SpecId::CANCUN,
        BigUint::from(2_u8).pow(256_u32) - BigUint::from(10_u8),
    );
}

#[test]
fn sub_underflow_u64() {
    let (a, b) = (BigUint::from(0_u64), BigUint::from(1_u64));
    let operations = vec![
        Operation::Push((32_u8, b.clone())),
        Operation::Push((32_u8, a.clone())),
        Operation::Sub,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(
        env,
        db,
        SpecId::CANCUN,
        BigUint::from(2_u8).pow(256_u32) - BigUint::from(1_u8),
    );
}

#[test]
fn div() {
    let (a, b) = (BigUint::from(10_u8), BigUint::from(5_u8));
    let operations = vec![
        Operation::Push((1_u8, b.clone())),
        Operation::Push((1_u8, a.clone())),
        Operation::Div,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, a / b);
}

#[test]
fn div_by_zero() {
    let (a, b) = (BigUint::from(0_u8), BigUint::from(10_u8));
    let operations = vec![
        Operation::Push((1_u8, b.clone())),
        Operation::Push((1_u8, a.clone())),
        Operation::Div,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 0_u8.into());
}

#[test]
fn div_by_one() {
    let (a, b) = (BigUint::from(1_u8), BigUint::from(10_u8));
    let operations = vec![
        Operation::Push((1_u8, b.clone())),
        Operation::Push((1_u8, a.clone())),
        Operation::Div,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 0_u8.into());
}

#[test]
fn div_by_eleven() {
    let (a, b) = (BigUint::from(11_u8), BigUint::from(10_u8));
    let operations = vec![
        Operation::Push((1_u8, b.clone())),
        Operation::Push((1_u8, a.clone())),
        Operation::Div,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 1_u8.into());
}

#[test]
fn div_zero() {
    let (a, b) = (BigUint::from(10_u8), BigUint::from(0_u8));
    let operations = vec![
        Operation::Push((1_u8, b.clone())),
        Operation::Push((1_u8, a.clone())),
        Operation::Div,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 0_u8.into());
}

#[test]
fn umod() {
    let (a, b) = (BigUint::from(2_u8), BigUint::from(6_u8));
    let operations = vec![
        Operation::Push((1_u8, b.clone())),
        Operation::Push((1_u8, a.clone())),
        Operation::Mod,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 2_u8.into());
}

#[test]
fn umod_by_zero() {
    let (a, b) = (BigUint::from(0_u8), BigUint::from(10_u8));
    let operations = vec![
        Operation::Push((1_u8, b.clone())),
        Operation::Push((1_u8, a.clone())),
        Operation::Mod,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 0_u8.into());
}

#[test]
fn umod_by_one() {
    let (a, b) = (BigUint::from(1_u8), BigUint::from(10_u8));
    let operations = vec![
        Operation::Push((1_u8, b.clone())),
        Operation::Push((1_u8, a.clone())),
        Operation::Mod,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 1_u8.into());
}

#[test]
fn smod_by_zero() {
    let (a, b) = (BigUint::from(0_u8), BigUint::from(10_u8));
    let operations = vec![
        Operation::Push((1_u8, b.clone())),
        Operation::Push((1_u8, a.clone())),
        Operation::SMod,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 0_u8.into());
}

#[test]
fn umod_zero() {
    let (a, b) = (BigUint::from(10_u8), BigUint::from(0_u8));
    let operations = vec![
        Operation::Push((1_u8, b.clone())),
        Operation::Push((1_u8, a.clone())),
        Operation::Mod,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 0_u8.into());
}

#[test]
fn smod_zero() {
    let (a, b) = (BigUint::from(10_u8), BigUint::from(0_u8));
    let operations = vec![
        Operation::Push((1_u8, b.clone())),
        Operation::Push((1_u8, a.clone())),
        Operation::SMod,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 0_u8.into());
}

#[test]
fn sdiv_positive() {
    let (a, b) = (BigUint::from(10_u8), BigUint::from(5_u8));
    let operations = vec![
        Operation::Push((1_u8, b.clone())),
        Operation::Push((1_u8, a.clone())),
        Operation::SDiv,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, a / b);
}

#[test]
fn sdiv_negative() {
    let operations = vec![
        Operation::Push((
            32_u8,
            BigUint::from_bytes_be(&[
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF,
            ]),
        )),
        Operation::Push((
            32_u8,
            BigUint::from_bytes_be(&[
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF,
            ]),
        )),
        Operation::SDiv,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 1_u8.into());
}

#[test]
fn modulus() {
    let (a, b) = (BigUint::from(10_u8), BigUint::from(3_u8));
    let operations = vec![
        Operation::Push((1_u8, b.clone())),
        Operation::Push((1_u8, a.clone())),
        Operation::Mod,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, a % b);
}

#[test]
fn modulus_large_numbers() {
    let (a, b) = (BigUint::from(17_u8), BigUint::from(5_u8));
    let operations = vec![
        Operation::Push((1_u8, b.clone())),
        Operation::Push((1_u8, a.clone())),
        Operation::Mod,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, a % b);
}

#[test]
fn smod() {
    let (a, b) = (BigUint::from(10_u8), BigUint::from(3_u8));
    let operations = vec![
        Operation::Push((1_u8, b.clone())),
        Operation::Push((1_u8, a.clone())),
        Operation::SMod,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, a % b);
}

#[test]
fn smod_negative() {
    let (a, b) = (
        biguint_256_from_bigint(BigInt::from(-8_i64)),
        biguint_256_from_bigint(BigInt::from(-3_i64)),
    );
    let operations = vec![
        Operation::Push((32_u8, b.clone())),
        Operation::Push((32_u8, a.clone())),
        Operation::SMod,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(
        env,
        db,
        SpecId::CANCUN,
        biguint_256_from_bigint(BigInt::from(-2_i8)),
    );
}

#[test]
fn addmod() {
    let (a, b, den) = (
        BigUint::from(1_u8),
        BigUint::from(2_u8),
        BigUint::from(3_u32),
    );
    let operations = vec![
        Operation::Push((1_u8, den.clone())),
        Operation::Push((1_u8, b.clone())),
        Operation::Push((1_u8, a.clone())),
        Operation::AddMod,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, (a + b) % den);
}

#[test]
fn addmod_large_mod() {
    let (a, b, den) = (
        BigUint::from(u128::MAX),
        BigUint::from(u128::MAX),
        BigUint::from(100_u8),
    );
    let operations = vec![
        Operation::Push((32_u8, den.clone())),
        Operation::Push((32_u8, b.clone())),
        Operation::Push((32_u8, a.clone())),
        Operation::AddMod,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, (a + b) % den);
}

#[test]
fn mulmod() {
    let (a, b, den) = (
        BigUint::from(10_u32),
        BigUint::from(10_u32),
        BigUint::from(8_u32),
    );
    let operations = vec![
        Operation::Push((1_u8, den.clone())),
        Operation::Push((1_u8, b.clone())),
        Operation::Push((1_u8, a.clone())),
        Operation::MulMod,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, (a * b) % den);
}

#[test]
fn mulmod_zero_mod() {
    let (a, b, den) = (
        BigUint::from(2_u8),
        BigUint::from(2_u8),
        BigUint::from(0_u8),
    );
    let operations = vec![
        Operation::Push((32_u8, den.clone())),
        Operation::Push((32_u8, b.clone())),
        Operation::Push((32_u8, a.clone())),
        Operation::MulMod,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 0_u8.into());
}

#[test]
fn exp() {
    let (a, b) = (BigUint::from(3_u8), 3_u32);
    let operations = vec![
        Operation::Push((1_u8, b.into())),
        Operation::Push((1_u8, a.clone())),
        Operation::Exp,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, a.pow(b));
}

#[test]
fn exp_large_base() {
    let (a, b) = (BigUint::from(123456789_u64), 5_u32);
    let operations = vec![
        Operation::Push((4_u8, b.into())),
        Operation::Push((a.bits() as u8 / 8 + 1, a.clone())),
        Operation::Exp,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, a.pow(b));
}

#[test]
fn exp_large_u256_exponent() {
    let (a, b) = (
        0xff_u8.into(),
        BigUint::from_str(
            "102161150204658159326162171757797299165741800222807601117528975009918212890625",
        )
        .unwrap(),
    );
    let operations = vec![
        Operation::Push((32, b)),
        Operation::Push((1_u8, a)),
        Operation::Exp,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, BigUint::from_str(
        &U256::from(0xFF).pow(U256::from_str("102161150204658159326162171757797299165741800222807601117528975009918212890625").unwrap()).to_string()
    ).unwrap());
}

#[test]
fn exp_edge_case() {
    let (a, b) = (BigUint::from(u128::MAX), 1_u32);
    let operations = vec![
        Operation::Push((1_u8, b.into())),
        Operation::Push((a.bits() as u8 / 8 + 1, a.clone())),
        Operation::Exp,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, a.pow(b));
}

#[test]
fn signextend() {
    let (a, b) = (BigUint::from(0_u8), BigUint::from(0xFF_u8));
    let operations = vec![
        Operation::Push((1_u8, b.clone())),
        Operation::Push((1_u8, a.clone())),
        Operation::SignExtend,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(
        env,
        db,
        SpecId::CANCUN,
        biguint_256_from_bigint(BigInt::from(-1_i8)),
    );
}

#[test]
fn lt() {
    let (a, b) = (BigUint::from(9_u8), BigUint::from(10_u8));
    let operations = vec![
        Operation::Push((1_u8, b.clone())),
        Operation::Push((1_u8, a.clone())),
        Operation::Lt,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, (a < b).into());
}

#[test]
fn gt() {
    let (a, b) = (BigUint::from(9_u8), BigUint::from(10_u8));
    let operations = vec![
        Operation::Push((1_u8, b.clone())),
        Operation::Push((1_u8, a.clone())),
        Operation::Gt,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, (a > b).into());
}

#[test]
fn eq_true() {
    let (a, b) = (BigUint::from(10_u8), BigUint::from(10_u8));
    let operations = vec![
        Operation::Push((1_u8, b.clone())),
        Operation::Push((1_u8, a.clone())),
        Operation::Eq,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, (a == b).into());
}

#[test]
fn eq_false() {
    let (a, b) = (BigUint::from(10_u8), BigUint::from(5_u8));
    let operations = vec![
        Operation::Push((1_u8, b.clone())),
        Operation::Push((1_u8, a.clone())),
        Operation::Eq,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, (a == b).into());
}

#[test]
fn iszero_true() {
    let operations = vec![
        Operation::Push0,
        Operation::IsZero,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 1_u8.into());
}

#[test]
fn iszero_false() {
    let operations = vec![
        Operation::Push((1_u8, 10_u8.into())),
        Operation::IsZero,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 0_u8.into());
}

#[test]
fn and_identical_non_zero() {
    let operations = vec![
        Operation::Push((1_u8, BigUint::from_bytes_be(&[0xFF]))),
        Operation::Push((1_u8, BigUint::from_bytes_be(&[0xFF]))),
        Operation::And,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 0xFF_u8.into());
}

#[test]
fn and_zero_with_non_zero() {
    let operations = vec![
        Operation::Push0,
        Operation::Push((1_u8, BigUint::from_bytes_be(&[0xFF]))),
        Operation::And,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 0_u32.into());
}

#[test]
fn and_zero_with_large() {
    let operations = vec![
        Operation::Push((
            32_u8,
            BigUint::from_bytes_be(&[
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF,
            ]),
        )),
        Operation::Push((32_u8, 0_u32.into())),
        Operation::And,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 0_u32.into());
}

#[test]
fn or_identical_non_zero() {
    let operations = vec![
        Operation::Push((1_u8, BigUint::from_bytes_be(&[0xFF]))),
        Operation::Push((1_u8, BigUint::from_bytes_be(&[0xFF]))),
        Operation::Or,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 0xFF_u8.into());
}

#[test]
fn or_zero_with_non_zero() {
    let operations = vec![
        Operation::Push0,
        Operation::Push((1_u8, BigUint::from_bytes_be(&[0xFF]))),
        Operation::Or,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 0xFF_u8.into());
}

#[test]
fn or_zero_with_large() {
    let operations = vec![
        Operation::Push0,
        Operation::Push((
            32_u8,
            BigUint::from_bytes_be(&[
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF,
            ]),
        )),
        Operation::Or,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(
        env,
        db,
        SpecId::CANCUN,
        BigUint::from_bytes_be(&[
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF,
        ]),
    );
}

#[test]
fn xor_non_zero() {
    let operations = vec![
        Operation::Push((1_u8, BigUint::from_bytes_be(&[0xF]))),
        Operation::Push((1_u8, BigUint::from_bytes_be(&[0xF0]))),
        Operation::Xor,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 0xFF_u8.into());
}

#[test]
fn xor_zero_with_non_zero() {
    let operations = vec![
        Operation::Push0,
        Operation::Push((1_u8, BigUint::from_bytes_be(&[0xFF]))),
        Operation::Xor,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 0xFF_u8.into());
}

#[test]
fn xor_large_with_zero() {
    let a = BigUint::from_bytes_be(&[
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF,
    ]);
    let operations = vec![
        Operation::Push((32_u8, a.clone())),
        Operation::Push((32_u8, 0_u32.into())),
        Operation::Xor,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, a);
}

#[test]
fn not() {
    let operations = vec![
        Operation::Push0,
        Operation::Not,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(
        env,
        db,
        SpecId::CANCUN,
        BigUint::from_bytes_be(&[
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff,
        ]),
    );
}

#[test]
fn byte() {
    let operations = vec![
        Operation::Push((32_u8, BigUint::from_bytes_be(&[0xff; 32]))),
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Byte,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 0_u8.into());
}

#[test]
fn shl() {
    let operations = vec![
        Operation::Push((1_u8, 1_u8.into())),
        Operation::Push((1_u8, 1_u8.into())),
        Operation::Shl,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 2_u8.into());
}

#[test]
fn shr() {
    let operations = vec![
        Operation::Push((1_u8, 1_u8.into())),
        Operation::Push((1_u8, 1_u8.into())),
        Operation::Shr,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 0_u8.into());
}

#[test]
fn sar() {
    let operations = vec![
        Operation::Push((1_u8, 2_u8.into())),
        Operation::Push((1_u8, 1_u8.into())),
        Operation::Sar,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 1_u8.into());
}

#[test]
fn keccak256_empty_bytes() {
    let operations = vec![
        Operation::Push((1_u8, 0x00_u8.into())),
        Operation::Push((1_u8, 0x00_u8.into())),
        Operation::Keccak256,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let expected = BigUint::parse_bytes(
        b"c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470",
        16,
    )
    .unwrap();
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, expected);
}

#[test]
fn keccak256_padded_data() {
    let operations = vec![
        Operation::Push((
            32_u8,
            BigUint::from_bytes_be(&[
                0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00,
            ]),
        )),
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 4_u8.into())),
        Operation::Push0,
        Operation::Keccak256,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let expected = BigUint::parse_bytes(
        b"29045a592007d0c246ef02c2223570da9522d0cf0f73282c79a1bc8f0bb2c238",
        16,
    )
    .unwrap();
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, expected);
}

#[test]
fn keccak256_single_byte() {
    let operations = vec![
        Operation::Push((1_u8, 0x04_u8.into())),
        Operation::Push((1_u8, 0x00_u8.into())),
        Operation::Keccak256,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let expected = BigUint::parse_bytes(
        b"e8e77626586f73b955364c7b4bbf0bb7f7685ebd40e852b164633a4acbd3244c",
        16,
    )
    .unwrap();
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, expected);
}

#[test]
fn address() {
    let operations = vec![
        Operation::Address,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 40_u8.into());
}

#[test]
fn balance() {
    let operations = vec![
        Operation::Push0,
        Operation::Balance,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 0_u8.into());
}

#[test]
fn origin() {
    let operations = vec![
        Operation::Origin,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 0_u8.into());
}

#[test]
fn caller() {
    let addr = Address::left_padding_from(&[40]);
    let mut value = Bytes32::ZERO;
    value.copy_from(&addr);
    let operations = vec![
        Operation::Caller,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (mut env, db) = default_env_and_db_setup(operations);
    env.tx.caller = addr;
    env.tx.nonce = 1;
    run_program_assert_num_result(
        env,
        db,
        SpecId::CANCUN,
        BigUint::from_bytes_le(&value.to_le_bytes()),
    );
}

#[test]
fn callvalue() {
    let operations = vec![
        Operation::CallValue,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 0_u8.into());
}

#[test]
fn calldataload_zero_offset() {
    let operations = vec![
        Operation::Push0,
        Operation::CalldataLoad,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 0_u8.into());
}

#[test]
fn calldataload_non_zero_offset() {
    let operations = vec![
        Operation::Push((1_u8, 100_u8.into())),
        Operation::CalldataLoad,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 0_u8.into());
}

#[test]
fn calldatasize() {
    let operations = vec![
        Operation::CalldataSize,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 0_u8.into());
}

#[test]
fn calldatacopy() {
    let operations = vec![
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Push0,
        Operation::CalldataCopy,
        // Return result
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 0_u8.into());
}

#[test]
fn calldatacopy_small_range() {
    let operations = vec![
        Operation::Push0,
        Operation::Push((1_u8, 10_u8.into())),
        Operation::Push((1_u8, 20_u8.into())),
        Operation::CalldataCopy,
        // Return result
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 0_u8.into());
}

#[test]
fn calldatacopy_large_range() {
    let operations = vec![
        Operation::Push0,
        Operation::Push((1_u8, 100_u8.into())),
        Operation::Push((1_u8, 10_u8.into())),
        Operation::CalldataCopy,
        // Return result
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 0_u8.into());
}

#[test]
fn calldatacopy_out_of_range() {
    let operations = vec![
        Operation::Push((1_u8, 1_u8.into())),
        Operation::Push((1_u8, 1_u8.into())),
        Operation::Push((1_u8, 1_u8.into())),
        Operation::CalldataCopy,
        // Return result
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 0_u8.into());
}

#[test]
fn codesize() {
    let operations = vec![
        Operation::CodeSize,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 7_u8.into());
}

#[test]
fn codesize_with_push_pop() {
    let operations = vec![
        Operation::Push0,
        Operation::Pop,
        Operation::CodeSize,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 9_u8.into());
}

#[test]
fn codecopy() {
    let operations = vec![
        Operation::Push0,
        Operation::Push0,
        Operation::Push((1_u8, 10_u8.into())),
        Operation::CodeCopy,
        // Return result
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 0_u8.into());
}

#[test]
fn codecopy_with_large_value() {
    let a = BigUint::from_bytes_be(&[
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    ]);
    let operations = vec![
        Operation::Push((a.bits() as u8 / 8 + 1, a.clone())),
        Operation::Push((32_u8, 0_u8.into())),
        Operation::Pop,
        Operation::Pop,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Push0,
        Operation::CodeCopy,
        // Return result
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(
        env,
        db,
        SpecId::CANCUN,
        BigUint::from_str(
            "50208493039807347493768565078611515464268940091219712940177602988105906782208",
        )
        .unwrap(),
    );
}

#[test]
fn codecopy_partial() {
    let operations = vec![
        Operation::Push0,
        Operation::Push((1_u8, 5_u8.into())),
        Operation::Push((1_u8, 5_u8.into())),
        Operation::CodeCopy,
        // Return result
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 0_u8.into());
}

#[test]
fn codecopy_large_offset() {
    let operations = vec![
        Operation::Push0,
        Operation::Push((1_u8, 50_u8.into())),
        Operation::Push((1_u8, 10_u8.into())),
        Operation::CodeCopy,
        // Return result
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 0_u8.into());
}

#[test]
fn gasprice() {
    let operations = vec![
        Operation::GasPrice,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 0_u8.into());
}

#[test]
fn test_extcodesize() {
    let operations = vec![
        Operation::Push((
            32_u8,
            BigUint::from_bytes_be(&[
                0x7F, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF,
            ]),
        )),
        Operation::Push0,
        Operation::MStore,
        Operation::Push((
            32_u8,
            BigUint::from_bytes_be(&[
                0x7F, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF,
            ]),
        )),
        Operation::Push((1_u8, 32_u8.into())),
        Operation::MStore,
        Operation::Push((1_u8, 41_u8.into())),
        Operation::Push0,
        Operation::Push0,
        Operation::Create,
        Operation::ExtCodeSize,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    // 40 is the sender address
    let _created_address = Address::left_padding_from(&[40]).create(1);
    // _created_address is the deployed contract address
    // 41 is the deployed contract code size.
    run_program_assert_num_result(env, db, SpecId::CANCUN, 0_u8.into());
}

#[test]
fn extcodesize_nonexistent_address() {
    let operations = vec![
        Operation::Push((20_u8, BigUint::from_bytes_be(&[0xde, 0xad, 0xbe, 0xef]))),
        Operation::ExtCodeSize,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 0_u8.into());
}

#[test]
fn extcodesize_zero_address() {
    let operations = vec![
        Operation::Push((20_u8, BigUint::from_bytes_be(&[0x00, 0x00, 0x00, 0x00]))),
        Operation::ExtCodeSize,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 0_u8.into());
}

#[test]
fn extcodecopy_full() {
    let operations = vec![
        Operation::Push((
            32_u8,
            BigUint::from_bytes_be(&[
                0x7F, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF,
            ]),
        )),
        Operation::Push0,
        Operation::MStore,
        Operation::Push((
            32_u8,
            BigUint::from_bytes_be(&[
                0x7F, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF,
            ]),
        )),
        Operation::Push((1_u8, 32_u8.into())),
        Operation::MStore,
        Operation::Push((1_u8, 41_u8.into())),
        Operation::Push0,
        Operation::Push0,
        Operation::Create,
        Operation::Push0,
        Operation::Push0,
        Operation::MStore,
        Operation::Push0,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Push0,
        // Dup the created contract address value
        Operation::Dup(4),
        Operation::ExtCodeCopy,
        // Return result, the top of stack top is the created contract address
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    let created_address = Address::left_padding_from(&[40]).create(1);
    run_program_assert_num_result(
        env,
        db,
        SpecId::CANCUN,
        BigUint::from_bytes_be(&created_address.0 .0),
    );
}

#[test]
fn extcodecopy_specific_length() {
    let operations = vec![
        Operation::Push0,
        Operation::Push0,
        Operation::Push((1_u8, 10_u8.into())),
        Operation::Push((20_u8, BigUint::from_bytes_be(&[0xde, 0xad, 0xbe, 0xef]))),
        Operation::ExtCodeCopy,
        // Return result
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 0_u8.into());
}

#[test]
fn extcodecopy_partial() {
    let operations = vec![
        Operation::Push0,
        Operation::Push((1_u8, 5_u8.into())),
        Operation::Push((1_u8, 5_u8.into())),
        Operation::Push((20_u8, BigUint::from_bytes_be(&[0xde, 0xad, 0xbe, 0xef]))),
        Operation::ExtCodeCopy,
        // Return result
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 0_u8.into());
}

#[test]
fn extcodecopy_out_of_bounds() {
    let operations = vec![
        // extcodecopy size
        Operation::Push0,
        // extcodecopy offset
        Operation::Push((1_u8, 50_u8.into())),
        // extcodecopy dest offset
        Operation::Push((1_u8, 10_u8.into())),
        // extcodecopy address
        Operation::Push((20_u8, BigUint::from_bytes_be(&[0xde, 0xad, 0xbe, 0xef]))),
        Operation::ExtCodeCopy,
        // Return result
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 0_u8.into());
}

#[test]
fn returndatasize() {
    let operations = vec![
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push((4_u8, BigUint::from_bytes_be(&[0xFF, 0xFF, 0xFF, 0xFF]))),
        Operation::Push((4_u8, BigUint::from_bytes_be(&[0xFF, 0xFF, 0xFF, 0xFF]))),
        Operation::Staticcall,
        Operation::ReturndataSize,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 0_u8.into());
}

#[test]
fn returndatacopy() {
    let calldata_size = 32_u8;
    let operations = vec![
        // size
        Operation::Push((1_u8, calldata_size.into())),
        // offset
        Operation::Push0,
        // Dest offset
        Operation::Push0,
        Operation::ReturndataCopy,
        Operation::Push0,
        Operation::MLoad,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let mut env = Env::default();
    env.tx.gas_limit = INIT_GAS;
    env.tx.transact_to = TxKind::Call(Address::left_padding_from(&[40]));
    env.block.coinbase = Address::left_padding_from(&[80]);
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
    let mut runtime_context =
        RuntimeContext::new(contract, 1, false, false, &mut host, SpecId::CANCUN);
    runtime_context.set_returndata(vec![0; calldata_size as usize]);
    run_with_context::<MemoryDB>(&mut runtime_context, INIT_GAS).unwrap();
    let status = runtime_context.status();
    assert!(status.is_ok());
}

#[test]
fn returndatacopy_offset_size_adjustments() {
    let operations = vec![
        // size
        Operation::Push0,
        // offset
        Operation::Push0,
        // Dest offset
        Operation::Push0,
        Operation::ReturndataCopy,
        Operation::Push0,
        Operation::MLoad,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 0_u8.into());
}

#[test]
fn returndatacopy_out_of_bounds_with_empty_calldata() {
    let operations = vec![
        Operation::Push0,
        Operation::Push((1_u8, 50_u8.into())),
        Operation::Push((1_u8, 10_u8.into())),
        Operation::ReturndataCopy,
        Operation::Push0,
        Operation::MLoad,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_halt(env, db, SpecId::CANCUN);
}

#[test]
fn returndatacopy_out_of_bounds() {
    let calldata_size = 32_u8;
    let operations = vec![
        // size
        Operation::Push((1_u8, calldata_size.into())),
        // offset
        Operation::Push0,
        // Dest offset
        Operation::Push0,
        Operation::ReturndataCopy,
        Operation::Push0,
        Operation::MLoad,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let mut env = Env::default();
    env.tx.gas_limit = INIT_GAS;
    env.tx.transact_to = TxKind::Call(Address::left_padding_from(&[40]));
    env.block.coinbase = Address::left_padding_from(&[80]);
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
    let mut runtime_context =
        RuntimeContext::new(contract, 1, false, false, &mut host, SpecId::CANCUN);
    runtime_context.set_returndata(vec![0; (calldata_size - 10) as usize]);
    run_with_context::<MemoryDB>(&mut runtime_context, INIT_GAS).unwrap();
    let status = runtime_context.status();
    assert!(status.is_error());
}

#[test]
fn extcodehash() {
    let operations = vec![
        Operation::Push((
            13_u8,
            BigUint::from_bytes_be(&[
                0x63, 0xFF, 0xFF, 0xFF, 0xFF, 0x60, 0x00, 0x52, 0x60, 0x04, 0x60, 0x00, 0xF3,
            ]),
        )),
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 13_u8.into())),
        Operation::Push0,
        Operation::Push0,
        Operation::Create,
        Operation::ExtCodeHash,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(
        env,
        db,
        SpecId::CANCUN,
        BigUint::from_str(
            "89477152217924674838424037953991966239322087453347756267410168184682657981552",
        )
        .unwrap(),
    );
}

#[test]
fn extcodehash_nonexistent_address() {
    let operations = vec![
        Operation::Push((20_u8, BigUint::from_bytes_be(&[0xde, 0xad, 0xbe, 0xef]))),
        Operation::ExtCodeHash,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 0_u8.into());
}

#[test]
fn extcodehash_empty_address() {
    let operations = vec![
        Operation::Push((20_u8, BigUint::from_bytes_be(&[0x00, 0x00, 0x00, 0x00]))),
        Operation::ExtCodeHash,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(
        env,
        db,
        SpecId::CANCUN,
        BigUint::from_bytes_be(&EMPTY_CODE_HASH_BYTES),
    );
}

#[test]
fn blockhash_invalid_block_number() {
    let operations = vec![
        Operation::Push((32_u8, 59942354_u32.into())),
        Operation::BlockHash,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 0_u8.into());
}

#[test]
fn blockhash_previous_block() {
    let block_number = 1_u8;
    let block_hash: u32 = 209433;
    let current_block_number = 3_u8;
    let expected_block_hash = BigUint::from(block_hash);
    let operations = vec![
        Operation::Push((32, block_number.into())),
        Operation::BlockHash,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (mut env, mut db) = default_env_and_db_setup(operations);
    env.block.number = U256::from(current_block_number);
    db.insert_block_hash(
        U256::from(block_number),
        B256::left_padding_from(&block_hash.to_be_bytes()),
    );
    run_program_assert_num_result(env, db, SpecId::CANCUN, expected_block_hash);
}

#[test]
fn coinbase() {
    let operations = vec![
        Operation::Coinbase,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 80_u8.into());
}

#[test]
fn timestamp() {
    let operations = vec![
        Operation::Timestamp,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 0_u8.into());
}

#[test]
fn number() {
    let operations = vec![
        Operation::Number,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 0_u8.into());
}

#[test]
fn prevrandao() {
    let operations = vec![
        Operation::Prevrandao,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 0_u8.into());
}

#[test]
fn gaslimit() {
    let operations = vec![
        Operation::GasLimit,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, INIT_GAS.into());
}

#[test]
fn chainid() {
    let operations = vec![
        Operation::Chainid,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 1_u8.into());
}

#[test]
fn selfbalance() {
    let operations = vec![
        Operation::SelfBalance,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 10_u8.into());
}

#[test]
fn basefee() {
    let operations = vec![
        Operation::BaseFee,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 0_u8.into());
}

#[test]
fn blobhash() {
    let operations = vec![
        Operation::Push((3_u8, 21000u32.into())),
        Operation::BlobHash,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 0_u8.into());
}

#[test]
fn blobbasefee() {
    let operations = vec![
        Operation::BlobBaseFee,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 0_u8.into());
}

#[test]
fn push_pop() {
    let operations = vec![
        Operation::Push((3_u8, 125985_u32.into())),
        Operation::Pop,
        Operation::Push((3_u8, 125986_u32.into())),
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 125986_u32.into());
}

#[test]
fn push_pop_1() {
    let operations = vec![
        Operation::Push((1_u8, 42_u8.into())),
        Operation::Pop,
        Operation::Push((1_u8, 24_u8.into())),
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 24_u8.into());
}

#[test]
fn push_multiple_pop() {
    let operations = vec![
        Operation::Push((1_u8, 1_u8.into())),
        Operation::Push((1_u8, 2_u8.into())),
        Operation::Push((1_u8, 3_u8.into())),
        Operation::Pop,
        Operation::Pop,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 1_u8.into());
}

#[test]
fn push_stack_depth() {
    let operations = vec![
        Operation::Push((1_u8, 10_u8.into())),
        Operation::Push((1_u8, 20_u8.into())),
        Operation::Push((1_u8, 30_u8.into())),
        Operation::Push((1_u8, 40_u8.into())),
        Operation::Pop,
        Operation::Pop,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 20_u8.into());
}

#[test]
fn mstore_mload() {
    let operations = vec![
        Operation::Push((
            32_u8,
            BigUint::from_bytes_be(&[
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0xFF,
            ]),
        )),
        Operation::Push0,
        Operation::MStore,
        Operation::Push0,
        Operation::MLoad,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(
        env,
        db,
        SpecId::CANCUN,
        BigUint::from_bytes_be(&[
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0xFF,
        ]),
    );
}

#[test]
fn mstore_mload_1() {
    let operations = vec![
        Operation::Push((32_u8, 1024_u64.into())),
        Operation::Push((32_u8, 321_u64.into())),
        Operation::MStore,
        Operation::Push((32_u8, 321_u64.into())),
        Operation::MLoad,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 1024_u64.into());
}

#[test]
fn mstore_1() {
    let operations = vec![
        Operation::Push0,
        Operation::Push((32_u8, 42_u64.into())),
        Operation::MStore,
        // Return result
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 0_u32.into());
}

#[test]
fn mstore_2() {
    let operations = vec![
        Operation::Push0,
        Operation::Push((32_u8, 42_u64.into())),
        Operation::MStore,
        Operation::Push0,
        Operation::Push((32_u8, 99_u64.into())),
        Operation::MStore,
        // Return result
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 0_u32.into());
}

#[test]
fn mstore() {
    let operations = vec![
        Operation::Push((
            32_u8,
            BigUint::from_bytes_be(&[
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0xFF,
            ]),
        )),
        Operation::Push0,
        Operation::MStore,
        // Return result
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(
        env,
        db,
        SpecId::CANCUN,
        BigUint::from_bytes_be(&[
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0xFF,
        ]),
    );
}

#[test]
fn mstore_high_address() {
    let operations = vec![
        Operation::Push((32_u8, 1024_u64.into())),
        Operation::Push((32_u8, 123_u64.into())),
        Operation::MStore,
        // Return result
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 0_u32.into());
}

#[test]
fn mload_1() {
    let operations = vec![
        Operation::Push0,
        Operation::Push((32_u8, 42_u64.into())),
        Operation::MStore,
        Operation::Push0,
        Operation::MLoad,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 0_u64.into());
}

#[test]
fn mload_uninitialized() {
    let operations = vec![
        Operation::Push0,
        Operation::MLoad,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 0_u32.into());
}

#[test]
fn mstore8() {
    let operations = vec![
        // Note only one 0xFF will be stored into the memory
        Operation::Push((32_u8, BigUint::from_bytes_be(&[0xFF; 32]))),
        Operation::Push0,
        Operation::MStore8,
        Operation::Push0,
        Operation::MLoad,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let mut expect_bytes: Vec<u8> = vec![0x00; 31];
    expect_bytes.push(0xFF);
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(
        env,
        db,
        SpecId::CANCUN,
        BigUint::from_bytes_le(&expect_bytes),
    );
}

#[test]
fn mstore_mcopy_mload_with_zero_address_arbitrary_size() {
    let value = BigUint::from(1_u8) << 24;
    let value1 = BigUint::from(2_u8) << 24;
    let operations = vec![
        Operation::Push((32_u8, value1)),
        Operation::Push((1_u8, 32_u8.into())),
        Operation::MStore,
        Operation::Push((32_u8, value)),
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 4_u8.into())),
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::MCopy,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::MLoad,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let expected_result: BigUint = (16777216_u32 * 2).into();
    let mut result_bytes = [0_u8; 32];
    if expected_result != BigUint::ZERO {
        let bytes = expected_result.to_bytes_be();
        result_bytes[32 - bytes.len()..].copy_from_slice(&bytes);
    }
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 33554432_u64.into());
}

#[test]
fn sload() {
    let operations = vec![
        Operation::Push((1_u8, 46_u8.into())),
        Operation::Push0,
        Operation::SStore,
        Operation::Push0,
        Operation::SLoad,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 46_u8.into());
}

#[test]
fn sload_1() {
    let operations = vec![
        Operation::Push((32_u8, 0_u64.into())),
        Operation::Push((32_u8, 400_u64.into())),
        Operation::SStore,
        Operation::Push((32_u8, 0_u64.into())),
        Operation::SLoad,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 0_u64.into());
}

#[test]
fn sstore() {
    let operations = vec![
        Operation::Push((32_u8, 100_u64.into())),
        Operation::Push((32_u8, 0_u64.into())),
        Operation::SStore,
        Operation::Push((32_u8, 0_u64.into())),
        Operation::SLoad,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 100_u64.into());
}

#[test]
fn sstore_1() {
    let operations = vec![
        Operation::Push((2_u8, BigUint::from_bytes_be(&[0xFF, 0xFF]))),
        Operation::Push0,
        Operation::SStore,
        Operation::Push0,
        Operation::SLoad,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 65535_u32.into());
}

#[test]
fn sstore_multiple_slots() {
    let operations = vec![
        Operation::Push((32_u8, 1_u64.into())),
        Operation::Push((32_u8, 500_u64.into())),
        Operation::SStore,
        Operation::Push((32_u8, 2_u64.into())),
        Operation::Push((32_u8, 600_u64.into())),
        Operation::SStore,
        Operation::Push((32_u8, 600_u64.into())),
        Operation::SLoad,
        Operation::Push((32_u8, 500_u64.into())),
        Operation::SLoad,
        Operation::Add,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    // 3 means 2 + 1 from the storage slot 600 and slot 500
    run_program_assert_num_result(env, db, SpecId::CANCUN, 3_u8.into());
}

#[test]
fn sstore_high_slot() {
    let key = BigUint::from(2_u64).pow(255) - 1_u64;
    let operations = vec![
        Operation::Push((32_u8, 777_u64.into())),
        Operation::Push((32_u8, key.clone())),
        Operation::SStore,
        Operation::Push((32_u8, key)),
        Operation::SLoad,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 777_u64.into());
}

#[test]
fn sload_uninitialized() {
    let operations = vec![
        Operation::Push((32_u8, 0_u64.into())),
        Operation::SLoad,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 0_u32.into());
}

#[test]
fn sload_high_slot() {
    let key = BigUint::from(2_u64).pow(256) - 1_u64;
    let operations = vec![
        Operation::Push((32_u8, key.clone())),
        Operation::Push((32_u8, 123_u64.into())),
        Operation::SStore,
        Operation::Push((32_u8, key)),
        Operation::SLoad,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 0_u32.into());
}

#[test]
fn sload_multiple_slots() {
    let operations = vec![
        Operation::Push((32_u8, 0_u64.into())),
        Operation::Push((32_u8, 100_u64.into())),
        Operation::SStore,
        Operation::Push((32_u8, 1_u64.into())),
        Operation::Push((32_u8, 200_u64.into())),
        Operation::SStore,
        Operation::Push((32_u8, 0_u64.into())),
        Operation::SLoad,
        Operation::Push((32_u8, 1_u64.into())),
        Operation::SLoad,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 0_u8.into());
}

#[test]
fn jump() {
    let (a, b) = (5_u8, 10_u8);
    let pc: usize = 7;
    let operations = vec![
        Operation::Push((1_u8, a.into())),
        Operation::Push((1_u8, (pc as u8).into())),
        Operation::Jump,
        Operation::Push((1_u8, b.into())),
        Operation::Jumpdest { pc },
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, a.into());
}

#[test]
fn jumpi_with_false_condition() {
    let (a, b) = (5_u8, 10_u8);
    let condition: BigUint = BigUint::from(0_u8);
    let pc: usize = 9;
    let operations = vec![
        Operation::Push((1_u8, a.into())),
        Operation::Push((1_u8, condition)),
        Operation::Push((1_u8, (pc as u8).into())),
        Operation::JumpI,
        Operation::Push((1_u8, b.into())),
        Operation::Jumpdest { pc },
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, b.into());
}

#[test]
fn jumpi_does_not_revert_if_pc_is_wrong_but_branch_is_not_taken() {
    let (a, b) = (5_u8, 10_u8);
    let condition: BigUint = BigUint::from(0_u8);
    let pc: usize = 9;
    let operations = vec![
        Operation::Push((1_u8, a.into())),
        Operation::Push((1_u8, condition)),
        Operation::Push((1_u8, (pc as u8).into())),
        Operation::JumpI,
        Operation::Push((1_u8, b.into())),
        Operation::Jumpdest { pc },
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, b.into());
}

#[test]
fn jumpdest() {
    let expected = 5_u8;
    let operations = vec![
        Operation::Jumpdest { pc: 0 },
        Operation::Push((1_u8, expected.into())),
        Operation::Jumpdest { pc: 34 },
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, expected.into());
}

#[test]
fn pc() {
    let operations = vec![
        Operation::PC { pc: 0 },
        Operation::PC { pc: 1 },
        Operation::Jumpdest { pc: 2 },
        Operation::PC { pc: 3 },
        Operation::Push((1_u8, 1_u8.into())),
        Operation::PC { pc: 6 },
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 6_u8.into());
}

#[test]
fn mload_misze() {
    let operations = vec![
        Operation::MSize,
        Operation::Push0,
        Operation::MLoad,
        Operation::Pop,
        Operation::MSize,
        Operation::Push((1_u8, 39_u8.into())),
        Operation::MLoad,
        Operation::Pop,
        Operation::MSize,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 96_u8.into());
}

#[test]
fn gas() {
    let operations = vec![
        Operation::Gas,
        Operation::Push((4_u8, 21000_u32.into())),
        Operation::GasLimit,
        Operation::Sub,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, (INIT_GAS - 21000).into());
}

#[test]
fn tstore_tload() {
    let operations = vec![
        Operation::Push((1_u8, 46_u8.into())),
        Operation::Push0,
        Operation::TStore,
        Operation::Push0,
        Operation::TLoad,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 46_u8.into());
}

#[test]
fn tstore_0() {
    let operations = vec![
        Operation::Push((2_u8, BigUint::from_bytes_be(&[0xFF, 0xFF]))),
        Operation::Push0,
        Operation::TStore,
        Operation::Push0,
        Operation::TLoad,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(
        env,
        db,
        SpecId::CANCUN,
        BigUint::from_bytes_be(&[0xFF, 0xFF]),
    );
}

#[test]
fn tstore_1() {
    let operations = vec![
        Operation::Push((2_u8, BigUint::from_bytes_be(&[0xFF, 0xFF]))),
        Operation::Push((2_u8, 8965u32.into())),
        Operation::TStore,
        Operation::Push((2_u8, 8965u32.into())),
        Operation::TLoad,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(
        env,
        db,
        SpecId::CANCUN,
        BigUint::from_bytes_be(&[0xFF, 0xFF]),
    );
}

#[test]
fn mstore_mcopy() {
    let operations = vec![
        Operation::Push((
            32_u8,
            BigUint::from_bytes_be(&[
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0xFE,
            ]),
        )),
        Operation::Push((1_u8, 32_u8.into())),
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::MCopy,
        Operation::Push0,
        Operation::MLoad,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 254_u8.into());
}

#[test]
fn mcopy_large_size_overflow() {
    let operations = vec![
        Operation::Push((1_u8, 1_u8.into())),
        Operation::Push0,
        Operation::SStore,
        Operation::Push((1_u8, 17_u8.into())),
        Operation::Push((1_u8, 64_u8.into())),
        Operation::CalldataLoad,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::CalldataLoad,
        Operation::Push0,
        Operation::CalldataLoad,
        Operation::Push((1_u8, 22_u8.into())),
        Operation::Jump,
        Operation::Jumpdest { pc: 17 },
        Operation::MSize,
        Operation::Push0,
        Operation::SStore,
        Operation::Stop,
        Operation::Jumpdest { pc: 22 },
        Operation::MCopy,
        Operation::Jump,
    ];
    let (mut env, db) = default_env_and_db_setup(operations);
    env.tx.data = hex_literal::hex!("00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff").to_vec().into();
    run_program_assert_halt(env, db, SpecId::CANCUN);
}

#[test]
fn push_dup1() {
    let operations = vec![
        Operation::Push((1_u8, 1_u8.into())),
        Operation::Dup(1),
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 1_u8.into());
}

#[test]
fn push_dup2() {
    let operations = vec![
        Operation::Push((1_u8, 1_u8.into())),
        Operation::Push0,
        Operation::Dup(2),
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 1_u8.into());
}

#[test]
fn push_dup3() {
    let operations = vec![
        Operation::Push((1_u8, 1_u8.into())),
        Operation::Push0,
        Operation::Push0,
        Operation::Dup(3),
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 1_u8.into());
}

#[test]
fn push_dup4() {
    let operations = vec![
        Operation::Push((1_u8, 1_u8.into())),
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Dup(4),
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 1_u8.into());
}

#[test]
fn push_dup5() {
    let operations = vec![
        Operation::Push((1_u8, 1_u8.into())),
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Dup(5),
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 1_u8.into());
}

#[test]
fn push_dup6() {
    let operations = vec![
        Operation::Push((1_u8, 1_u8.into())),
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Dup(6),
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 1_u8.into());
}

#[test]
fn push_dup7() {
    let operations = vec![
        Operation::Push((1_u8, 1_u8.into())),
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Dup(7),
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 1_u8.into());
}

#[test]
fn push_dup8() {
    let operations = vec![
        Operation::Push((1_u8, 1_u8.into())),
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Dup(8),
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 1_u8.into());
}

#[test]
fn push_dup9() {
    let operations = vec![
        Operation::Push((1_u8, 1_u8.into())),
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Dup(9),
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 1_u8.into());
}

#[test]
fn push_dup10() {
    let operations = vec![
        Operation::Push((1_u8, 1_u8.into())),
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Dup(10),
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 1_u8.into());
}

#[test]
fn push_dup11() {
    let operations = vec![
        Operation::Push((1_u8, 1_u8.into())),
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Dup(11),
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 1_u8.into());
}

#[test]
fn push_dup12() {
    let operations = vec![
        Operation::Push((1_u8, 1_u8.into())),
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Dup(12),
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 1_u8.into());
}

#[test]
fn push_dup13() {
    let operations = vec![
        Operation::Push((1_u8, 1_u8.into())),
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Dup(13),
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 1_u8.into());
}

#[test]
fn push_dup14() {
    let operations = vec![
        Operation::Push((1_u8, 1_u8.into())),
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Dup(14),
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 1_u8.into());
}

#[test]
fn push_dup15() {
    let operations = vec![
        Operation::Push((1_u8, 1_u8.into())),
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Dup(15),
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 1_u8.into());
}

#[test]
fn push_dup16() {
    let operations = vec![
        Operation::Push((1_u8, 1_u8.into())),
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Dup(16),
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 1_u8.into());
}

#[test]
fn swap1() {
    let operations = vec![
        Operation::Push((1_u8, 2_u8.into())),
        Operation::Push((1_u8, 1_u8.into())),
        Operation::Swap(1),
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 2_u8.into());
}

#[test]
fn swap2() {
    let operations = vec![
        Operation::Push((1_u8, 2_u8.into())),
        Operation::Push0,
        Operation::Push((1_u8, 1_u8.into())),
        Operation::Swap(2),
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 2_u8.into());
}

#[test]
fn swap3() {
    let operations = vec![
        Operation::Push((1_u8, 2_u8.into())),
        Operation::Push0,
        Operation::Push0,
        Operation::Push((1_u8, 1_u8.into())),
        Operation::Swap(3),
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 2_u8.into());
}

#[test]
fn swap4() {
    let operations = vec![
        Operation::Push((1_u8, 2_u8.into())),
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push((1_u8, 1_u8.into())),
        Operation::Swap(4),
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 2_u8.into());
}

#[test]
fn swap5() {
    let operations = vec![
        Operation::Push((1_u8, 2_u8.into())),
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push((1_u8, 1_u8.into())),
        Operation::Swap(5),
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 2_u8.into());
}

#[test]
fn swap6() {
    let operations = vec![
        Operation::Push((1_u8, 2_u8.into())),
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push((1_u8, 1_u8.into())),
        Operation::Swap(6),
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 2_u8.into());
}

#[test]
fn swap7() {
    let operations = vec![
        Operation::Push((1_u8, 2_u8.into())),
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push((1_u8, 1_u8.into())),
        Operation::Swap(7),
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 2_u8.into());
}

#[test]
fn swap8() {
    let operations = vec![
        Operation::Push((1_u8, 2_u8.into())),
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push((1_u8, 1_u8.into())),
        Operation::Swap(8),
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 2_u8.into());
}

#[test]
fn swap9() {
    let operations = vec![
        Operation::Push((1_u8, 2_u8.into())),
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push((1_u8, 1_u8.into())),
        Operation::Swap(9),
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 2_u8.into());
}

#[test]
fn swap10() {
    let operations = vec![
        Operation::Push((1_u8, 2_u8.into())),
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push((1_u8, 1_u8.into())),
        Operation::Swap(10),
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 2_u8.into());
}

#[test]
fn swap11() {
    let operations = vec![
        Operation::Push((1_u8, 2_u8.into())),
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push((1_u8, 1_u8.into())),
        Operation::Swap(11),
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 2_u8.into());
}

#[test]
fn swap12() {
    let operations = vec![
        Operation::Push((1_u8, 2_u8.into())),
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push((1_u8, 1_u8.into())),
        Operation::Swap(12),
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 2_u8.into());
}

#[test]
fn swap13() {
    let operations = vec![
        Operation::Push((1_u8, 2_u8.into())),
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push((1_u8, 1_u8.into())),
        Operation::Swap(13),
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 2_u8.into());
}

#[test]
fn swap14() {
    let operations = vec![
        Operation::Push((1_u8, 2_u8.into())),
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push((1_u8, 1_u8.into())),
        Operation::Swap(14),
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 2_u8.into());
}

#[test]
fn swap15() {
    let operations = vec![
        Operation::Push((1_u8, 2_u8.into())),
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push((1_u8, 1_u8.into())),
        Operation::Swap(15),
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 2_u8.into());
}

#[test]
fn swap16() {
    let operations = vec![
        Operation::Push((1_u8, 2_u8.into())),
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push((1_u8, 1_u8.into())),
        Operation::Swap(16),
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 2_u8.into());
}

#[test]
fn create() {
    let operations = vec![
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Create,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    // The expect result is the empty code hash address.
    run_program_assert_num_result(
        env,
        db,
        SpecId::CANCUN,
        BigUint::from_str(CREATE_ADDRESS_U256_STR).unwrap(),
    );
}

#[test]
fn create_1() {
    let operations = vec![
        Operation::Push0,
        Operation::Push0,
        // value is 9, the sender account balance is 10,
        Operation::Push((1_u8, 9_u8.into())),
        Operation::Create,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    // The expect result is the empty code hash address.
    run_program_assert_num_result(
        env,
        db,
        SpecId::CANCUN,
        BigUint::from_str(CREATE_ADDRESS_U256_STR).unwrap(),
    );
}

#[test]
fn create_2() {
    let operations = vec![
        Operation::Push((1_u8, 10_u8.into())),
        Operation::Push0,
        Operation::Push((1_u8, 20_u8.into())),
        // value is 20, the sender account balance is 10 and it is not enough,
        // thus the create process will be halt
        Operation::Create,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 0_u8.into());
}

#[test]
fn create_3() {
    let operations = vec![
        Operation::Push((
            32_u8,
            BigUint::from_bytes_be(&[
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            ]),
        )),
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Push0,
        Operation::Create,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(
        env,
        db,
        SpecId::CANCUN,
        BigUint::from_str(CREATE_ADDRESS_U256_STR).unwrap(),
    );
}

#[test]
fn create_with_value() {
    let operations = vec![
        Operation::Push((1_u8, 100_u8.into())),
        Operation::Push0,
        // value is 9, the sender account balance is 10,
        Operation::Push((1_u8, 9_u8.into())),
        Operation::Create,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(
        env,
        db,
        SpecId::CANCUN,
        BigUint::from_str(CREATE_ADDRESS_U256_STR).unwrap(),
    );
}

#[test]
fn create2_with_salt() {
    let operations = vec![
        Operation::Push((32_u8, 0x1234_u16.into())),
        Operation::Push((1_u8, 1_u8.into())),
        Operation::Push((1_u8, 20_u8.into())),
        Operation::Push((1_u8, 10_u8.into())),
        Operation::Create2,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(
        env,
        db,
        SpecId::CANCUN,
        BigUint::from_str("1298672851206845405429649291545422093257887715444").unwrap(),
    );
}

#[test]
fn create2_with_large_salt() {
    let operations = vec![
        Operation::Push((
            32_u8,
            BigUint::from_bytes_be(&[
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF, 0xFF, 0xFF,
            ]),
        )),
        Operation::Push((1_u8, 1_u8.into())),
        Operation::Push((1_u8, 64_u8.into())),
        Operation::Push((1_u8, 50_u8.into())),
        Operation::Create2,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 0_u8.into());
}

#[test]
fn create_too_init_code_size_limit_halt() {
    let operations = vec![
        Operation::Push((4_u8, 0x16000_u32.into())),
        Operation::Push0,
        Operation::Push0,
        Operation::Create,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_halt(env, db, SpecId::CANCUN);
}

#[test]
fn log0() {
    let operations = vec![
        Operation::Push((1_u8, 0x40_u8.into())),
        Operation::Push((1_u8, 0x20_u8.into())),
        Operation::Log(0),
        // Return result
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 0_u8.into());
}

#[test]
fn log1() {
    let operations = vec![
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push((1_u8, 10_u8.into())),
        Operation::Push((1_u8, 0x01_u8.into())),
        Operation::Log(1),
        // Return result
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 0_u8.into());
}

#[test]
fn log2() {
    let operations = vec![
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push((1_u8, 10_u8.into())),
        Operation::Push((1_u8, 0x01_u8.into())),
        Operation::Push((1_u8, 0x02_u8.into())),
        Operation::Log(2),
        // Return result
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 0_u8.into());
}

#[test]
fn log3() {
    let operations = vec![
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push((1_u8, 10_u8.into())),
        Operation::Push((1_u8, 0x01_u8.into())),
        Operation::Push((1_u8, 0x02_u8.into())),
        Operation::Push((1_u8, 0x03_u8.into())),
        Operation::Log(3),
        // Return resul
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 0_u8.into());
}

#[test]
fn log4() {
    let operations = vec![
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push((1_u8, 10_u8.into())),
        Operation::Push((1_u8, 0x01_u8.into())),
        Operation::Push((1_u8, 0x02_u8.into())),
        Operation::Push((1_u8, 0x03_u8.into())),
        Operation::Push((1_u8, 0x04_u8.into())),
        Operation::Log(4),
        // Return result
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 0_u8.into());
}

#[test]
fn dataload_zero_offset() {
    let operations = vec![
        Operation::Push0,
        Operation::DataLoad,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];

    let program = Program {
        operations,
        code_size: 0,
        is_eof: true,
    };

    let eof = Eof::new(EofBody {
        code_section: vec![Bytes::from(program.to_opcode())],
        data_section: Bytes::new(),
        ..EofBody::default()
    });

    let (env, db) = default_env_and_db_setup_eof(eof);
    run_program_assert_num_result(env, db, SpecId::OSAKA, 0_u8.into());
}

#[test]
fn call() {
    let operations = vec![
        Operation::Push((4_u8, 10000_u32.into())),
        Operation::Push((4_u8, 0x1000_u32.into())),
        Operation::Push((1_u8, 1_u32.into())),
        Operation::Push((1_u8, 32_u32.into())),
        Operation::Push((1_u8, 32_u32.into())),
        Operation::Push((1_u8, 64_u32.into())),
        Operation::Push((1_u8, 64_u32.into())),
        Operation::Call,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 0_u8.into());
}

#[test]
fn call_1() {
    let operations = vec![
        Operation::Push((
            17_u8,
            BigUint::from_bytes_be(&[
                0x67, 0x60, 0x00, 0x35, 0x60, 0x07, 0x57, 0xFE, 0x5B, 0x60, 0x00, 0x52, 0x60, 0x08,
                0x60, 0x18, 0xF3,
            ]),
        )),
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 17_u8.into())),
        Operation::Push((1_u8, 15_u8.into())),
        Operation::Push0,
        Operation::Create,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Dup(6),
        Operation::Push((2_u8, BigUint::from_bytes_be(&[0xFF, 0xFF]))),
        Operation::Call,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 0_u8.into());
}

#[test]
fn call_2() {
    let operations = vec![
        Operation::Push((
            17_u8,
            BigUint::from_bytes_be(&[
                0x67, 0x60, 0x00, 0x35, 0x60, 0x07, 0x57, 0xFE, 0x5B, 0x60, 0x00, 0x52, 0x60, 0x08,
                0x60, 0x18, 0xF3,
            ]),
        )),
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 17_u8.into())),
        Operation::Push((1_u8, 15_u8.into())),
        Operation::Push0,
        Operation::Create,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Dup(6),
        Operation::Push((2_u8, BigUint::from_bytes_be(&[0xFF, 0xFF]))),
        Operation::Call,
        Operation::Push0,
        Operation::Push0,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Push0,
        Operation::Dup(7),
        Operation::Push((2_u8, BigUint::from_bytes_be(&[0xFF, 0xFF]))),
        Operation::Call,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 1_u8.into());
}

#[test]
fn call_insufficient_value() {
    let operations = vec![
        Operation::Push((
            17_u8,
            BigUint::from_bytes_be(&[
                0x67, 0x60, 0x00, 0x35, 0x60, 0x07, 0x57, 0xFE, 0x5B, 0x60, 0x00, 0x52, 0x60, 0x08,
                0x60, 0x18, 0xF3,
            ]),
        )),
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 17_u8.into())),
        Operation::Push((1_u8, 15_u8.into())),
        Operation::Push0,
        Operation::Create,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Dup(6),
        Operation::Push((2_u8, BigUint::from_bytes_be(&[0xFF, 0xFF]))),
        Operation::Call,
        Operation::Push0,
        Operation::Push0,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Push((1_u8, 200_u8.into())),
        Operation::Dup(7),
        Operation::Push((2_u8, BigUint::from_bytes_be(&[0xF0, 0x0F]))),
        Operation::Call,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 0_u8.into());
}

#[test]
fn callcode() {
    let operations = vec![
        Operation::Push((32_u8, 5000_u32.into())),
        Operation::Push((32_u8, 0x2000_u32.into())),
        Operation::Push((1_u8, 0_u32.into())),
        Operation::Push((1_u8, 32_u32.into())),
        Operation::Push((1_u8, 32_u32.into())),
        Operation::Push((1_u8, 64_u32.into())),
        Operation::Push((1_u8, 64_u32.into())),
        Operation::Callcode,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 0_u8.into());
}

#[test]
fn callcode_1() {
    let operations = vec![
        Operation::Push((
            17_u8,
            BigUint::from_bytes_be(&[
                0x67, 0x60, 0x00, 0x35, 0x60, 0x07, 0x57, 0xFE, 0x5B, 0x60, 0x00, 0x52, 0x60, 0x08,
                0x60, 0x18, 0xF3,
            ]),
        )),
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 17_u8.into())),
        Operation::Push((1_u8, 15_u8.into())),
        Operation::Push0,
        Operation::Create,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Dup(6),
        Operation::Push((2_u8, BigUint::from_bytes_be(&[0xFF, 0xFF]))),
        Operation::Callcode,
        Operation::Push((1_u8, 1_u8.into())),
        Operation::Push0,
        Operation::SStore,
        Operation::Push0,
        Operation::Push0,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Push0,
        Operation::Dup(7),
        Operation::Push((2_u8, BigUint::from_bytes_be(&[0xFF, 0xFF]))),
        Operation::Callcode,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 1_u8.into());
}

#[test]
fn rreturn() {
    let operations = vec![
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 0_u8.into());
}

#[test]
fn return_large_data() {
    let operations = vec![
        Operation::Push((1_u8, 64_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 0_u8.into());
}

#[test]
fn store_return() {
    let operations = vec![
        Operation::Push((
            32_u8,
            BigUint::from_bytes_be(&[
                0xFF, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            ]),
        )),
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 2_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 0_u8.into());
}

#[test]
fn delegatecall() {
    let operations = vec![
        Operation::Push((4_u8, 7000_u32.into())),
        Operation::Push((4_u8, 0x3000_u32.into())),
        Operation::Push((1_u8, 32_u32.into())),
        Operation::Push((1_u8, 32_u32.into())),
        Operation::Push((1_u8, 64_u32.into())),
        Operation::Push((1_u8, 64_u32.into())),
        Operation::Delegatecall,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 1_u8.into());
}

#[test]
fn delegatecall_1() {
    let operations = vec![
        Operation::Push((
            17_u8,
            BigUint::from_bytes_be(&[
                0x67, 0x60, 0x00, 0x35, 0x60, 0x07, 0x57, 0xFE, 0x5B, 0x60, 0x00, 0x52, 0x60, 0x08,
                0x60, 0x18, 0xF3,
            ]),
        )),
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 17_u8.into())),
        Operation::Push((1_u8, 15_u8.into())),
        Operation::Push0,
        Operation::Create,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Dup(5),
        Operation::Push((2_u8, BigUint::from_bytes_be(&[0xFF, 0xFF]))),
        Operation::Delegatecall,
        Operation::Push((1_u8, 1_u8.into())),
        Operation::Push0,
        Operation::SStore,
        Operation::Push0,
        Operation::Push0,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Dup(6),
        Operation::Push((2_u8, BigUint::from_bytes_be(&[0xFF, 0xFF]))),
        Operation::Delegatecall,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 1_u8.into());
}

#[test]
fn staticcall() {
    let operations = vec![
        Operation::Push((4_u8, 8000_u32.into())),
        Operation::Push((4_u8, 0x4000_u32.into())),
        Operation::Push((1_u8, 32_u32.into())),
        Operation::Push((1_u8, 32_u32.into())),
        Operation::Push((1_u8, 64_u32.into())),
        Operation::Push((1_u8, 64_u32.into())),
        Operation::Staticcall,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 1_u8.into());
}

// #[test]
// fn returndataload() {
//     let operations = vec![
//         Operation::Push((2_u8, 0_u8.into())),
//         // Note that RETURNDATALOAD is not found in the CANCUN spec.
//         Operation::ReturndataLoad,
//         // Return result
//         Operation::Push0,
//         Operation::MStore,
//         Operation::Push((1_u8, 32_u8.into())),
//         Operation::Push0,
//         Operation::Return,
//     ];
//     let (env, db) = default_env_and_db_setup(operations, true);
//     run_program_assert_halt(env, db, SpecId::OSAKA);
// }

#[test]
fn revert() {
    let operations = vec![
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Revert,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_revert(env, db, SpecId::CANCUN);
}

#[test]
fn revert_large_data() {
    let operations = vec![
        Operation::Push((1_u8, 64_u8.into())),
        Operation::Push0,
        Operation::Revert,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_revert(env, db, SpecId::CANCUN);
}

#[test]
fn mstore_revert() {
    let operations = vec![
        Operation::Push((
            32_u8,
            BigUint::from_bytes_be(&[
                0xFF, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            ]),
        )),
        Operation::Push((32_u8, 0_u8.into())),
        Operation::MStore,
        Operation::Push((1_u8, 2_u8.into())),
        Operation::Push0,
        Operation::Revert,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_revert(env, db, SpecId::CANCUN);
}

#[test]
fn invalid() {
    let operations = vec![Operation::Invalid];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_halt(env, db, SpecId::CANCUN);
}

#[test]
fn stop() {
    let operations = vec![Operation::Stop];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 0_u8.into());
}

#[test]
fn selfdestruct() {
    let operations = vec![
        Operation::Push((
            20_u8,
            BigUint::from_bytes_be(&[
                0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0, 0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC,
                0xDE, 0xF0, 0x12, 0x34, 0x56, 0x78,
            ]),
        )),
        Operation::Selfdestruct,
        Operation::Push0,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 0_u8.into());
}

#[test]
fn selfdestruct_zero_address() {
    let operations = vec![
        Operation::Push((1_u8, 0_u32.into())),
        Operation::Selfdestruct,
        Operation::Push0,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    let (env, db) = default_env_and_db_setup(operations);
    run_program_assert_num_result(env, db, SpecId::CANCUN, 0_u8.into());
}
