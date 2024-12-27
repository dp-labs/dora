use crate::context::Context;
use crate::evm::program::{Operation, Program};
use crate::evm::{CompileOptions, EVMCompiler};
use crate::Compiler;
use num_bigint::BigUint;

macro_rules! assert_snapshot {
    ($operations:expr) => {
        assert_snapshot!($operations, false)
    };
    ($operations:expr, $is_eof:expr) => {
        let program = Program {
            operations: $operations,
            code_size: 0,
            is_eof: $is_eof,
        };
        let context = Context::new();
        let compiler = EVMCompiler::new(&context);
        // Compile EVM Bytecode to MLIR EVM Dialect
        let module = compiler
            .compile(&program, &(), &CompileOptions::default())
            .expect("failed to compile program");
        insta::assert_snapshot!(module.module().as_operation());
    };
}

#[test]
fn push_push_add() {
    let (a, b) = (BigUint::from(11_u8), BigUint::from(31_u8));
    let operations = vec![
        Operation::Push((1_u8, a.clone())),
        Operation::Push((1_u8, b.clone())),
        Operation::Add,
    ];
    assert_snapshot!(operations);
}

#[test]
fn push_push_sub() {
    let (a, b) = (BigUint::from(11_u8), BigUint::from(5_u8));
    let operations = vec![
        Operation::Push((1_u8, a.clone())),
        Operation::Push((1_u8, b.clone())),
        Operation::Sub,
    ];
    assert_snapshot!(operations);
}

#[test]
fn push_push_mul_div() {
    let (a, b) = (BigUint::from(1_u8), BigUint::from(2_u8));
    let operations = vec![
        Operation::Push((1_u8, a.clone())),
        Operation::Push((1_u8, b.clone())),
        Operation::Mul,
        Operation::Push((1_u8, b.clone())),
        Operation::Div,
    ];
    assert_snapshot!(operations);
}

#[test]
fn push0_pop() {
    let operations = vec![Operation::Push0, Operation::Pop];
    assert_snapshot!(operations);
}

#[test]
fn push0_iszero_pop() {
    let operations = vec![Operation::Push0, Operation::IsZero, Operation::Pop];
    assert_snapshot!(operations);
}
