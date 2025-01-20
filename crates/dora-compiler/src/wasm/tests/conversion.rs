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
fn unreachable() {
    assert_snapshot!(
        r#"
(module
    (func (export "break-bare") (result i32)
    (block (br 0) (unreachable))
    (block (br_if 0 (i32.const 1)) (unreachable))
    (block (br_table 0 (i32.const 0)) (unreachable))
    (block (br_table 0 0 0 (i32.const 1)) (unreachable))
    (i32.const 19)
  )
)
"#
    );
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
fn sum_with_export_functions() {
    assert_snapshot!(
        r#"
(module
  (func $sum_f (param $x i32) (param $y i32) (result i32)
    local.get $x
    local.get $y
    i32.add)

  (func (export "main") 
    (call $sum_f (i32.const 2) (i32.const 3))
  )
)
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
fn rotl_i32() {
    assert_snapshot!(
        r#"
(module
  (func (export "rotl") (param $x i32) (param $y i32) (result i32) (i32.rotl (local.get $x) (local.get $y)))
)
"#
    );
}

#[test]
fn rotl_i64() {
    assert_snapshot!(
        r#"
(module
  (func (export "rotl") (param $x i64) (param $y i64) (result i64) (i64.rotl (local.get $x) (local.get $y)))
)
"#
    );
}

#[test]
fn rotr_i32() {
    assert_snapshot!(
        r#"
(module
  (func (export "rotr") (param $x i32) (param $y i32) (result i32) (i32.rotr (local.get $x) (local.get $y)))
)
"#
    );
}

#[test]
fn rotr_i64() {
    assert_snapshot!(
        r#"
(module
  (func (export "rotr") (param $x i64) (param $y i64) (result i64) (i64.rotr (local.get $x) (local.get $y)))
)
"#
    );
}

#[test]
fn clz_i32() {
    assert_snapshot!(
        r#"
(module
  (func (export "clz") (param $x i32) (result i32) (i32.clz (local.get $x)))
)
"#
    );
}

#[test]
fn clz_i64() {
    assert_snapshot!(
        r#"
(module
  (func (export "clz") (param $x i64) (result i64) (i64.clz (local.get $x)))
)
"#
    );
}

#[test]
fn ctz_i32() {
    assert_snapshot!(
        r#"
(module
  (func (export "ctz") (param $x i32) (result i32) (i32.ctz (local.get $x)))
)
"#
    );
}

#[test]
fn ctz_i64() {
    assert_snapshot!(
        r#"
(module
  (func (export "ctz") (param $x i64) (result i64) (i64.ctz (local.get $x)))
)
"#
    );
}

#[test]
fn popcnt_i32() {
    assert_snapshot!(
        r#"
(module
  (func (export "popcnt") (param $x i32) (result i32) (i32.popcnt (local.get $x)))
)
"#
    );
}

#[test]
fn popcnt_i64() {
    assert_snapshot!(
        r#"
(module
  (func (export "popcnt") (param $x i64) (result i64) (i64.popcnt (local.get $x)))
)
"#
    );
}

#[test]
fn extend_i32() {
    assert_snapshot!(
        r#"
(module
  (func (export "extend8_s") (param $x i32) (result i32) (i32.extend8_s (local.get $x)))
  (func (export "extend16_s") (param $x i32) (result i32) (i32.extend16_s (local.get $x)))
)
"#
    );
}

#[test]
fn extend_i64() {
    assert_snapshot!(
        r#"
(module
  (func (export "extend8_s") (param $x i64) (result i64) (i64.extend8_s (local.get $x)))
  (func (export "extend16_s") (param $x i64) (result i64) (i64.extend16_s (local.get $x)))
  (func (export "extend32_s") (param $x i64) (result i64) (i64.extend32_s (local.get $x)))
)
"#
    );
}

#[test]
fn eq_i32() {
    assert_snapshot!(
        r#"
(module
  (func (export "eqz") (param $x i32) (result i32) (i32.eqz (local.get $x)))
  (func (export "eq") (param $x i32) (param $y i32) (result i32) (i32.eq (local.get $x) (local.get $y)))
  (func (export "ne") (param $x i32) (param $y i32) (result i32) (i32.ne (local.get $x) (local.get $y)))
)
"#
    );
}

#[test]
fn eq_i64() {
    assert_snapshot!(
        r#"
(module
  (func (export "eqz") (param $x i64) (result i32) (i64.eqz (local.get $x)))
  (func (export "eq") (param $x i64) (param $y i64) (result i32) (i64.eq (local.get $x) (local.get $y)))
  (func (export "ne") (param $x i64) (param $y i64) (result i32) (i64.ne (local.get $x) (local.get $y)))
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
