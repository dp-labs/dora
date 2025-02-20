//！Reference: https://github.com/WebAssembly/spec/tree/main/test/core
use crate::context::Context;
use crate::wasm::{WASMCompileOptions, WASMCompiler};
use wasmer::wat2wasm;

macro_rules! assert_snapshot {
    ($code:expr) => {
        let context = Context::new();
        let compiler =
            WASMCompiler::new(&context, WASMCompileOptions::default().gas_metering(true));
        let wasm_bytes = wat2wasm($code.as_bytes()).unwrap();
        // Compile WASM Bytecode to MLIR EVM Dialect
        let mut module = compiler
            .compile(&wasm_bytes)
            .expect("failed to compile program");
        // Lowering the WASM dialect to the Dora dialect.
        crate::wasm::pass::run(&context.mlir_context, &mut module.mlir_module)
            .expect("failed to run conversion pass");
        // Lowering the Dora dialect to MLIR builtin dialects.
        crate::dora::pass::run(
            &context.mlir_context,
            &mut module.mlir_module,
            &crate::dora::pass::PassOptions {
                code_size: wasm_bytes.len() as u32,
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
fn console_fib() {
    assert_snapshot!(include_str!("suites/console_fib.wat"));
}

#[test]
fn fib() {
    assert_snapshot!(include_str!("suites/fib.wat"));
}

#[test]
fn counter_contract() {
    assert_snapshot!(include_str!("suites/counter.wat"));
}
