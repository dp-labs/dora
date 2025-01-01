use crate::context::Context;
use crate::evm::program::{Operation, Program};
use crate::evm::{CompileOptions, EVMCompiler};
use crate::Compiler;
use dora_primitives::Bytecode;
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

#[test]
fn program_from_eof_bytecode() {
    let bytecode = hex_literal::hex!("ef0001010004020001013c04006d00008000056080806040526004361015e100035f80fd5f3560e01c9081633fb5c1cb14e100e081638381f58a14e1009c5063d09de08a14e100045fe0ffd534e100875f7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc360112e1005c5f547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8114e100086001015f555f80f37f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5f80fd5f80fd34e100335f7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc360112e100086020905f548152f35f80fd5f80fd34e1003460207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc360112e100086004355f555f80f35f80fd5f80fda3646970667358221220cc6570f0f9fb641c08c7d9d1c36810bd19987855bcd8f9ccde7cfcd8670b41fa6c6578706572696d656e74616cf564736f6c63782c302e382e32372d646576656c6f702e323032342e372e32342b636f6d6d69742e64353139363430342e6d6f64006b");
    let bytecode = Bytecode::new_raw(bytecode.into());
    let program: Program = bytecode.into();
    insta::assert_snapshot!(format!("{:#?}", program.operations));
}
