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
fn cmp_i32() {
    assert_snapshot!(
        r#"
(module
  (func (export "lt_s") (param $x i32) (param $y i32) (result i32) (i32.lt_s (local.get $x) (local.get $y)))
  (func (export "lt_u") (param $x i32) (param $y i32) (result i32) (i32.lt_u (local.get $x) (local.get $y)))
  (func (export "le_s") (param $x i32) (param $y i32) (result i32) (i32.le_s (local.get $x) (local.get $y)))
  (func (export "le_u") (param $x i32) (param $y i32) (result i32) (i32.le_u (local.get $x) (local.get $y)))
  (func (export "gt_s") (param $x i32) (param $y i32) (result i32) (i32.gt_s (local.get $x) (local.get $y)))
  (func (export "gt_u") (param $x i32) (param $y i32) (result i32) (i32.gt_u (local.get $x) (local.get $y)))
  (func (export "ge_s") (param $x i32) (param $y i32) (result i32) (i32.ge_s (local.get $x) (local.get $y)))
  (func (export "ge_u") (param $x i32) (param $y i32) (result i32) (i32.ge_u (local.get $x) (local.get $y)))
)
"#
    );
}

#[test]
fn cmp_i64() {
    assert_snapshot!(
        r#"
(module
  (func (export "lt_s") (param $x i64) (param $y i64) (result i32) (i64.lt_s (local.get $x) (local.get $y)))
  (func (export "lt_u") (param $x i64) (param $y i64) (result i32) (i64.lt_u (local.get $x) (local.get $y)))
  (func (export "le_s") (param $x i64) (param $y i64) (result i32) (i64.le_s (local.get $x) (local.get $y)))
  (func (export "le_u") (param $x i64) (param $y i64) (result i32) (i64.le_u (local.get $x) (local.get $y)))
  (func (export "gt_s") (param $x i64) (param $y i64) (result i32) (i64.gt_s (local.get $x) (local.get $y)))
  (func (export "gt_u") (param $x i64) (param $y i64) (result i32) (i64.gt_u (local.get $x) (local.get $y)))
  (func (export "ge_s") (param $x i64) (param $y i64) (result i32) (i64.ge_s (local.get $x) (local.get $y)))
  (func (export "ge_u") (param $x i64) (param $y i64) (result i32) (i64.ge_u (local.get $x) (local.get $y)))
)
"#
    );
}

#[test]
fn ceil_i32() {
    assert_snapshot!(
        r#"
(module
  (func (export "ceil") (param $x f32) (result f32) (f32.ceil (local.get $x)))
)
"#
    );
}

#[test]
fn ceil_i64() {
    assert_snapshot!(
        r#"
(module
   (func (export "ceil") (param $x f64) (result f64) (f64.ceil (local.get $x)))
)
"#
    );
}

#[test]
fn abs_i32() {
    assert_snapshot!(
        r#"
(module
  (func (export "abs") (param $x f32) (result f32) (f32.abs (local.get $x)))
)
"#
    );
}

#[test]
fn abs_i64() {
    assert_snapshot!(
        r#"
(module
   (func (export "abs") (param $x f64) (result f64) (f64.abs (local.get $x)))
)
"#
    );
}

#[test]
fn neg_i32() {
    assert_snapshot!(
        r#"
(module
  (func (export "neg") (param $x f32) (result f32) (f32.neg (local.get $x)))
)
"#
    );
}

#[test]
fn neg_i64() {
    assert_snapshot!(
        r#"
(module
   (func (export "neg") (param $x f64) (result f64) (f64.abs (local.get $x)))
)
"#
    );
}

#[test]
fn floor_i32() {
    assert_snapshot!(
        r#"
(module
  (func (export "floor") (param $x f32) (result f32) (f32.neg (local.get $x)))
)
"#
    );
}

#[test]
fn floor_i64() {
    assert_snapshot!(
        r#"
(module
   (func (export "floor") (param $x f64) (result f64) (f64.abs (local.get $x)))
)
"#
    );
}

#[test]
fn nearest_i32() {
    assert_snapshot!(
        r#"
(module
  (func (export "nearest") (param $x f32) (result f32) (f32.neg (local.get $x)))
)
"#
    );
}

#[test]
fn nearest_i64() {
    assert_snapshot!(
        r#"
(module
   (func (export "nearest") (param $x f64) (result f64) (f64.abs (local.get $x)))
)
"#
    );
}

#[test]
fn copysign_i32() {
    assert_snapshot!(
        r#"
(module
  (func (export "copysign") (param $x f32) (result f32) (f32.neg (local.get $x)))
)
"#
    );
}

#[test]
fn copysign_i64() {
    assert_snapshot!(
        r#"
(module
   (func (export "copysign") (param $x f64) (result f64) (f64.abs (local.get $x)))
)
"#
    );
}

#[test]
fn fib() {
    assert_snapshot!(include_str!("suites/fib.wat"));
}

#[test]
fn fib_2() {
    assert_snapshot!(include_str!("suites/fib_2.wat"));
}

#[test]
fn float_local() {
    assert_snapshot!(include_str!("suites/float_local.wat"));
}

#[test]
fn align_read_write() {
    assert_snapshot!(include_str!("suites/align_read_write.wat"));
}

#[test]
fn complex_logic() {
    assert_snapshot!(include_str!("suites/complex_logic.wat"));
}

#[test]
fn brainfuck() {
    assert_snapshot!(include_str!("suites/brainfuck.wat"));
}

#[test]
fn block() {
    assert_snapshot!(include_str!("suites/block.wat"));
}

#[test]
fn br() {
    assert_snapshot!(include_str!("suites/br.wat"));
}

#[test]
fn br_if() {
    assert_snapshot!(include_str!("suites/br_if.wat"));
}

#[test]
fn br_table() {
    assert_snapshot!(include_str!("suites/br_table.wat"));
}

#[test]
fn bulk() {
    assert_snapshot!(include_str!("suites/bulk.wat"));
}

#[test]
fn call() {
    assert_snapshot!(include_str!("suites/call.wat"));
}

#[test]
fn call_indirect() {
    assert_snapshot!(include_str!("suites/call_indirect.wat"));
}

#[test]
fn conversions() {
    assert_snapshot!(include_str!("suites/conversions.wat"));
}

#[test]
fn counter_contract() {
    assert_snapshot!(include_str!("suites/counter.wat"));
}

#[test]
fn endianness() {
    assert_snapshot!(include_str!("suites/endianness.wat"));
}

#[test]
fn fac() {
    assert_snapshot!(include_str!("suites/fac.wat"));
}

#[test]
fn f32_arith() {
    assert_snapshot!(include_str!("suites/f32_arith.wat"));
}

#[test]
fn f64_arith() {
    assert_snapshot!(include_str!("suites/f64_arith.wat"));
}

#[test]
fn float_exprs() {
    assert_snapshot!(include_str!("suites/float_exprs.wat"));
}

#[test]
fn float_literals() {
    assert_snapshot!(include_str!("suites/float_literals.wat"));
}

#[test]
fn float_memory() {
    assert_snapshot!(include_str!("suites/float_memory.wat"));
}

#[test]
fn forward() {
    assert_snapshot!(include_str!("suites/forward.wat"));
}

#[test]
fn func() {
    assert_snapshot!(include_str!("suites/func.wat"));
}

#[test]
fn func_ptr() {
    assert_snapshot!(include_str!("suites/func_ptr.wat"));
}

#[test]
fn global() {
    assert_snapshot!(include_str!("suites/global.wat"));
}

#[test]
fn global_get_and_set() {
    assert_snapshot!(include_str!("suites/global_get_and_set.wat"));
}

#[test]
fn global_value() {
    assert_snapshot!(include_str!("suites/global_value.wat"));
}

#[test]
fn i32_arith() {
    assert_snapshot!(include_str!("suites/i32_arith.wat"));
}

#[test]
fn i64_arith() {
    assert_snapshot!(include_str!("suites/i64_arith.wat"));
}

#[test]
fn r#if() {
    assert_snapshot!(include_str!("suites/if.wat"));
}

#[test]
fn int_exprs() {
    assert_snapshot!(include_str!("suites/int_exprs.wat"));
}

#[test]
fn int_literals() {
    assert_snapshot!(include_str!("suites/int_literals.wat"));
}

#[test]
fn label() {
    assert_snapshot!(include_str!("suites/label.wat"));
}

#[test]
fn left_to_right() {
    assert_snapshot!(include_str!("suites/left_to_right.wat"));
}

#[test]
fn load() {
    assert_snapshot!(include_str!("suites/load.wat"));
}

#[test]
fn local_call() {
    assert_snapshot!(include_str!("suites/local_call.wat"));
}

#[test]
fn local_get_and_set() {
    assert_snapshot!(include_str!("suites/local_get_and_set.wat"));
}

#[test]
fn local_tee() {
    assert_snapshot!(include_str!("suites/local_tee.wat"));
}

#[test]
fn r#loop() {
    assert_snapshot!(include_str!("suites/loop.wat"));
}

#[test]
fn mem_i32_load() {
    assert_snapshot!(include_str!("suites/mem_i32_load.wat"));
}

#[test]
fn memory() {
    assert_snapshot!(include_str!("suites/memory.wat"));
}

#[test]
fn memory_copy() {
    assert_snapshot!(include_str!("suites/memory_copy.wat"));
}

#[test]
fn memory_fill() {
    assert_snapshot!(include_str!("suites/memory_fill.wat"));
}

#[test]
fn memory_grow() {
    assert_snapshot!(include_str!("suites/memory_grow.wat"));
}

#[test]
fn memory_init() {
    assert_snapshot!(include_str!("suites/memory_init.wat"));
}

#[test]
fn memory_redundancy() {
    assert_snapshot!(include_str!("suites/memory_redundancy.wat"));
}

#[test]
fn memory_load_store() {
    assert_snapshot!(include_str!("suites/memory_load_store.wat"));
}

#[test]
fn memory_size() {
    assert_snapshot!(include_str!("suites/memory_size.wat"));
}

#[test]
fn nop() {
    assert_snapshot!(include_str!("suites/nop.wat"));
}

#[test]
fn ref_func() {
    assert_snapshot!(include_str!("suites/ref_func.wat"));
}

#[test]
fn ref_is_null() {
    assert_snapshot!(include_str!("suites/ref_is_null.wat"));
}

#[test]
fn ref_null() {
    assert_snapshot!(include_str!("suites/ref_null.wat"));
}

#[test]
fn r#return() {
    assert_snapshot!(include_str!("suites/return.wat"));
}

#[test]
fn select() {
    assert_snapshot!(include_str!("suites/select.wat"));
}

#[test]
fn stack() {
    assert_snapshot!(include_str!("suites/stack.wat"));
}
