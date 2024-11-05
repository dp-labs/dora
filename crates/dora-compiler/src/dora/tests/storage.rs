use dora_primitives::config::OptimizationLevel;
use melior::ir::Module;
use melior::ExecutionEngine;

use crate::pass::run;
use crate::{context::Context, dora::storage::STORAGE_MEMORY_MAP_CODE};

use crate::evm::program::Operation;
use crate::evm::program::Program;
use crate::evm::EVMCompiler;
use crate::Compiler;
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
            .compile(&program, &())
            .expect("failed to compile program");
        crate::evm::pass::run(&context.mlir_context, &mut module.mlir_module).unwrap();
        crate::dora::pass::run_storage_pass(&context.mlir_context, &mut module.mlir_module)
            .unwrap();
        let op = module.module().as_operation();
        assert!(op.verify());
        insta::assert_snapshot!(op);
    };
}

#[test]
fn test_storage_memory_cache_code() {
    let context = Context::new();
    let mut module = Module::parse(&context.mlir_context, STORAGE_MEMORY_MAP_CODE).unwrap();
    assert!(module.as_operation().verify());
    run(&context.mlir_context, &mut module).unwrap();
    let _engine = ExecutionEngine::new(&module, OptimizationLevel::default() as usize, &[], false);
    assert!(module.as_operation().verify());
}

#[test]
fn test_storage_pass() {
    let operations = vec![
        Operation::Push((32_u8, BigUint::from(100_u64))),
        Operation::Push((32_u8, BigUint::from(0_u64))),
        Operation::Sstore,
        Operation::Push((32_u8, BigUint::from(0_u64))),
        Operation::Sload,
        // Return result
        Operation::Push0,
        Operation::Mstore,
        Operation::Push((1, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    assert_snapshot!(operations);
}