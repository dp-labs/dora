//！Reference: https://github.com/WebAssembly/spec/tree/main/test/core
use crate::context::Context;
use crate::wasm::{Config, WASMCompiler};
use wasmer::wat2wasm;

macro_rules! assert_snapshot {
    ($code:expr) => {
        let context = Context::new();
        let compiler = WASMCompiler::new(&context, Config::default());
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
fn sum() {
    assert_snapshot!(
        r#"
(module
  (type $sum_t (func (param i32 i32) (result i32)))
  (func $sum_f (type $sum_t) (param $x i32) (param $y i32) (result i32)
    local.get $x
    local.get $y
    i32.add)
  (export "sum" (func $sum_f)))
"#
    );
}

#[test]
fn and_i32() {
    assert_snapshot!(
        r#"
(module
  (func (export "and") (param $x i32) (param $y i32) (result i32) (i32.and (local.get $x) (local.get $y)))
)
"#
    );
}

#[test]
fn and_i64() {
    assert_snapshot!(
        r#"
(module
  (func (export "and") (param $x i64) (param $y i64) (result i64) (i64.and (local.get $x) (local.get $y)))
)
"#
    );
}

#[test]
fn or_i32() {
    assert_snapshot!(
        r#"
(module
  (func (export "or") (param $x i32) (param $y i32) (result i32) (i32.or (local.get $x) (local.get $y)))
)
"#
    );
}

#[test]
fn or_i64() {
    assert_snapshot!(
        r#"
(module
  (func (export "or") (param $x i64) (param $y i64) (result i64) (i64.or (local.get $x) (local.get $y)))
)
"#
    );
}

#[test]
fn xor_i32() {
    assert_snapshot!(
        r#"
(module
  (func (export "xor") (param $x i32) (param $y i32) (result i32) (i32.xor (local.get $x) (local.get $y)))
)
"#
    );
}

#[test]
fn xor_i64() {
    assert_snapshot!(
        r#"
(module
  (func (export "xor") (param $x i64) (param $y i64) (result i64) (i64.xor (local.get $x) (local.get $y)))
)
"#
    );
}

#[test]
fn shl_i32() {
    assert_snapshot!(
        r#"
(module
  (func (export "shl") (param $x i32) (param $y i32) (result i32) (i32.shl (local.get $x) (local.get $y)))
)
"#
    );
}

#[test]
fn shl_i64() {
    assert_snapshot!(
        r#"
(module
  (func (export "shl") (param $x i64) (param $y i64) (result i64) (i64.shl (local.get $x) (local.get $y)))
)
"#
    );
}

#[test]
fn shr_s_i32() {
    assert_snapshot!(
        r#"
(module
  (func (export "shr_s") (param $x i32) (param $y i32) (result i32) (i32.shr_s (local.get $x) (local.get $y)))
)
"#
    );
}

#[test]
fn shr_s_i64() {
    assert_snapshot!(
        r#"
(module
  (func (export "shr_s") (param $x i64) (param $y i64) (result i64) (i64.shr_s (local.get $x) (local.get $y)))
)
"#
    );
}

#[test]
fn shr_u_i32() {
    assert_snapshot!(
        r#"
(module
  (func (export "shr_u") (param $x i32) (param $y i32) (result i32) (i32.shr_u (local.get $x) (local.get $y)))
)
"#
    );
}

#[test]
fn shr_u_i64() {
    assert_snapshot!(
        r#"
(module
  (func (export "shr_u") (param $x i64) (param $y i64) (result i64) (i64.shr_u (local.get $x) (local.get $y)))
)
"#
    );
}

#[test]
fn select() {
    assert_snapshot!(
        r#"
(module
    (func $select (param $cond i32) (param $a i32) (param $b i32) (result i32)
        local.get $cond
        local.get $a
        local.get $b
        select
    )
    (export "select" (func $select))
)
"#
    );
}
