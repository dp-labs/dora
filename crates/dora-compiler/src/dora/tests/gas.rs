use crate::context::Context;
use crate::dora::gas::GasOptions;
use crate::evm::program::CompileOptions;
use crate::evm::program::Operation;
use crate::evm::program::Program;
use crate::evm::EVMCompiler;
use crate::Compiler;
use dora_primitives::spec::SpecId;
use num_bigint::BigUint;

macro_rules! assert_snapshot {
    ($operations:expr) => {
        let program = Program {
            operations: $operations,
            code_size: 0,
        };
        let context = Context::new();
        let compiler = EVMCompiler::new(&context);
        let mut module = compiler
            .compile(
                &program,
                &(),
                &CompileOptions {
                    spec_id: SpecId::CANCUN,
                },
            )
            .expect("failed to compile program");

        crate::evm::pass::run(&context.mlir_context, &mut module.mlir_module).unwrap();
        crate::dora::pass::run_gas_pass(
            &context.mlir_context,
            &mut module.mlir_module,
            GasOptions {
                spec_id: SpecId::CANCUN,
                limit_contract_code_size: None,
            },
        )
        .unwrap();
        let op = module.module().as_operation();
        assert!(op.verify());
        insta::assert_snapshot!(op);
    };
}

#[test]
fn gas_push_push_add() {
    let (a, b) = (BigUint::from(3_u8), 3_u32);
    let operations = vec![
        Operation::Push((1_u8, BigUint::from(b))),
        Operation::Push((1_u8, a.clone())),
        Operation::Add,
    ];
    assert_snapshot!(operations);
}

#[test]
fn gas_push_push_exp() {
    let (a, b) = (2_u32, 3_u32);
    let operations = vec![
        Operation::Push((1_u8, BigUint::from(b))),
        Operation::Push((1_u8, BigUint::from(a))),
        Operation::Exp,
    ];
    assert_snapshot!(operations);
}

#[test]
fn gas_push_push_add_sub() {
    let (a, b) = (BigUint::from(3_u8), 3_u32);
    let operations = vec![
        Operation::Push((1_u8, BigUint::from(b))),
        Operation::Push((1_u8, a.clone())),
        Operation::Add,
        Operation::Push((1_u8, a.clone())),
        Operation::Sub,
        Operation::Push((2_u8, a.clone())),
        Operation::Mul,
    ];
    assert_snapshot!(operations);
}

#[test]
fn gas_call_data_copy() {
    let operations = vec![
        Operation::Push((1_u8, BigUint::from(32_u8))),
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::CallDataCopy,
    ];
    assert_snapshot!(operations);
}

#[test]
fn gas_create() {
    let operations = vec![
        Operation::Push((1_u8, BigUint::from(41_u8))),
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Create,
    ];
    assert_snapshot!(operations);
}
