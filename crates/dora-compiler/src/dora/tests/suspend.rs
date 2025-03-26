use crate::Compiler;
use crate::context::Context;
use crate::evm::program::{Operation, Program};
use crate::evm::{EVMCompileOptions, EVMCompiler};
use dora_primitives::SpecId;
use num_bigint::BigUint;

macro_rules! assert_snapshot {
    ($operations:expr) => {
        assert_snapshot!($operations, false)
    };
    ($operations:expr, $is_eof:expr) => {
        let program = Program::from_operations($operations, $is_eof);
        let spec_id = if $is_eof {
            SpecId::OSAKA
        } else {
            SpecId::CANCUN
        };
        let context = Context::new();
        let compiler = EVMCompiler::new(
            &context,
            EVMCompileOptions::default().suspend(true).spec_id(spec_id),
        );
        let mut module = compiler
            .compile(&program)
            .expect("failed to compile program");
        crate::evm::pass::run(&context.mlir_context, &mut module.mlir_module).unwrap();
        crate::dora::pass::run(
            &context.mlir_context,
            &mut module.mlir_module,
            &crate::dora::pass::PassOptions {
                code_size: program.code_size(),
                ..Default::default()
            },
        )
        .unwrap();
        let op = module.module().as_operation();
        assert!(op.verify());
        insta::assert_snapshot!(op);
    };
}

#[test]
fn empty() {
    assert_snapshot!(vec![]);
}

#[test]
fn push_push_add() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, 10_u8.into())),
        Operation::Push((1_u8, 10_u8.into())),
        Operation::Add,
    ]);
}

#[test]
fn push_call() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, 10000_u32.into())),
        Operation::Push((1_u8, 0x1000_u32.into())),
        Operation::Push((1_u8, 1_u32.into())),
        Operation::Push((1_u8, 32_u32.into())),
        Operation::Push((1_u8, 32_u32.into())),
        Operation::Push((1_u8, 64_u32.into())),
        Operation::Push((1_u8, 64_u32.into())),
        Operation::Call,
    ]);
}

#[test]
fn push_mstore_create_call_0() {
    assert_snapshot!(vec![
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
    ]);
}

#[test]
fn push_mstore_create_call_1() {
    assert_snapshot!(vec![
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
    ]);
}

#[test]
fn push_return() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ]);
}

#[test]
fn push_return_large_data() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, 64_u8.into())),
        Operation::Push0,
        Operation::Return,
    ]);
}

#[test]
fn push_store_return() {
    assert_snapshot!(vec![
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
    ]);
}

#[test]
fn push_delegatecall() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, 7000_u32.into())),
        Operation::Push((1_u8, 0x3000_u32.into())),
        Operation::Push((1_u8, 32_u32.into())),
        Operation::Push((1_u8, 32_u32.into())),
        Operation::Push((1_u8, 64_u32.into())),
        Operation::Push((1_u8, 64_u32.into())),
        Operation::Delegatecall,
    ]);
}

#[test]
fn push_mstore_create_delegatecall() {
    assert_snapshot!(vec![
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
    ]);
}
