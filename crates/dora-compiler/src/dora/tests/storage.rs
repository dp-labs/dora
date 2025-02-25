use crate::Compiler;
use crate::evm::program::Operation;
use crate::evm::{EVMCompileOptions, EVMCompiler, Program};
use crate::pass::run;
use crate::{context::Context, dora::storage::STORAGE_MEMORY_MAP_CODE};
use dora_primitives::config::OptimizationLevel;
use melior::ExecutionEngine;
use melior::ir::Module;
use num_bigint::BigUint;

macro_rules! assert_snapshot {
    ($operations:expr) => {
        assert_snapshot!($operations, false)
    };
    ($operations:expr, $is_eof:expr) => {
        let program = Program::from_operations($operations, $is_eof);
        let context = Context::new();
        let compiler = EVMCompiler::new(
            &context,
            EVMCompileOptions {
                inline: true,
                ..Default::default()
            },
        );
        let mut module = compiler
            .compile(&program)
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
        Operation::SStore,
        Operation::Push((32_u8, BigUint::from(0_u64))),
        Operation::SLoad,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ];
    assert_snapshot!(operations);
}
