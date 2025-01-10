//！Reference: https://github.com/WebAssembly/spec/tree/main/test/core
use crate::context::Context;
use crate::wasm::{Config, WASMCompiler};
use wasmer::wat2wasm;

macro_rules! assert_snapshot {
    ($code:expr) => {
        let context = Context::new();
        let compiler = WASMCompiler::new(&context, Config::default());
        let wasm_bytes = wat2wasm($code.as_bytes()).unwrap();
        // Compile EVM Bytecode to MLIR EVM Dialect
        let module = compiler
            .compile(&wasm_bytes)
            .expect("failed to compile program");
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
fn i32_arith() {
    assert_snapshot!(
        r#"
(module
  (func (export "add") (param $x i32) (param $y i32) (result i32) (i32.add (local.get $x) (local.get $y)))
  (func (export "sub") (param $x i32) (param $y i32) (result i32) (i32.sub (local.get $x) (local.get $y)))
  (func (export "mul") (param $x i32) (param $y i32) (result i32) (i32.mul (local.get $x) (local.get $y)))
  (func (export "div_s") (param $x i32) (param $y i32) (result i32) (i32.div_s (local.get $x) (local.get $y)))
  (func (export "div_u") (param $x i32) (param $y i32) (result i32) (i32.div_u (local.get $x) (local.get $y)))
  (func (export "rem_s") (param $x i32) (param $y i32) (result i32) (i32.rem_s (local.get $x) (local.get $y)))
  (func (export "rem_u") (param $x i32) (param $y i32) (result i32) (i32.rem_u (local.get $x) (local.get $y)))
  (func (export "and") (param $x i32) (param $y i32) (result i32) (i32.and (local.get $x) (local.get $y)))
  (func (export "or") (param $x i32) (param $y i32) (result i32) (i32.or (local.get $x) (local.get $y)))
  (func (export "xor") (param $x i32) (param $y i32) (result i32) (i32.xor (local.get $x) (local.get $y)))
  (func (export "shl") (param $x i32) (param $y i32) (result i32) (i32.shl (local.get $x) (local.get $y)))
  (func (export "shr_s") (param $x i32) (param $y i32) (result i32) (i32.shr_s (local.get $x) (local.get $y)))
  (func (export "shr_u") (param $x i32) (param $y i32) (result i32) (i32.shr_u (local.get $x) (local.get $y)))
  (func (export "rotl") (param $x i32) (param $y i32) (result i32) (i32.rotl (local.get $x) (local.get $y)))
  (func (export "rotr") (param $x i32) (param $y i32) (result i32) (i32.rotr (local.get $x) (local.get $y)))
  (func (export "clz") (param $x i32) (result i32) (i32.clz (local.get $x)))
  (func (export "ctz") (param $x i32) (result i32) (i32.ctz (local.get $x)))
  (func (export "popcnt") (param $x i32) (result i32) (i32.popcnt (local.get $x)))
  (func (export "extend8_s") (param $x i32) (result i32) (i32.extend8_s (local.get $x)))
  (func (export "extend16_s") (param $x i32) (result i32) (i32.extend16_s (local.get $x)))
  (func (export "eqz") (param $x i32) (result i32) (i32.eqz (local.get $x)))
  (func (export "eq") (param $x i32) (param $y i32) (result i32) (i32.eq (local.get $x) (local.get $y)))
  (func (export "ne") (param $x i32) (param $y i32) (result i32) (i32.ne (local.get $x) (local.get $y)))
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
fn i64_arith() {
    assert_snapshot!(
        r#"
(module
  (func (export "add") (param $x i64) (param $y i64) (result i64) (i64.add (local.get $x) (local.get $y)))
  (func (export "sub") (param $x i64) (param $y i64) (result i64) (i64.sub (local.get $x) (local.get $y)))
  (func (export "mul") (param $x i64) (param $y i64) (result i64) (i64.mul (local.get $x) (local.get $y)))
  (func (export "div_s") (param $x i64) (param $y i64) (result i64) (i64.div_s (local.get $x) (local.get $y)))
  (func (export "div_u") (param $x i64) (param $y i64) (result i64) (i64.div_u (local.get $x) (local.get $y)))
  (func (export "rem_s") (param $x i64) (param $y i64) (result i64) (i64.rem_s (local.get $x) (local.get $y)))
  (func (export "rem_u") (param $x i64) (param $y i64) (result i64) (i64.rem_u (local.get $x) (local.get $y)))
  (func (export "and") (param $x i64) (param $y i64) (result i64) (i64.and (local.get $x) (local.get $y)))
  (func (export "or") (param $x i64) (param $y i64) (result i64) (i64.or (local.get $x) (local.get $y)))
  (func (export "xor") (param $x i64) (param $y i64) (result i64) (i64.xor (local.get $x) (local.get $y)))
  (func (export "shl") (param $x i64) (param $y i64) (result i64) (i64.shl (local.get $x) (local.get $y)))
  (func (export "shr_s") (param $x i64) (param $y i64) (result i64) (i64.shr_s (local.get $x) (local.get $y)))
  (func (export "shr_u") (param $x i64) (param $y i64) (result i64) (i64.shr_u (local.get $x) (local.get $y)))
  (func (export "rotl") (param $x i64) (param $y i64) (result i64) (i64.rotl (local.get $x) (local.get $y)))
  (func (export "rotr") (param $x i64) (param $y i64) (result i64) (i64.rotr (local.get $x) (local.get $y)))
  (func (export "clz") (param $x i64) (result i64) (i64.clz (local.get $x)))
  (func (export "ctz") (param $x i64) (result i64) (i64.ctz (local.get $x)))
  (func (export "popcnt") (param $x i64) (result i64) (i64.popcnt (local.get $x)))
  (func (export "extend8_s") (param $x i64) (result i64) (i64.extend8_s (local.get $x)))
  (func (export "extend16_s") (param $x i64) (result i64) (i64.extend16_s (local.get $x)))
  (func (export "extend32_s") (param $x i64) (result i64) (i64.extend32_s (local.get $x)))
  (func (export "eqz") (param $x i64) (result i32) (i64.eqz (local.get $x)))
  (func (export "eq") (param $x i64) (param $y i64) (result i32) (i64.eq (local.get $x) (local.get $y)))
  (func (export "ne") (param $x i64) (param $y i64) (result i32) (i64.ne (local.get $x) (local.get $y)))
  (func (export "lt_s") (param $x i64) (param $y i64) (result i32) (i64.lt_s (local.get $x) (local.get $y)))
  (func (export "lt_u") (param $x i64) (param $y i64) (result i32) (i64.lt_u (local.get $x) (local.get $y)))
  (func (export "le_s") (param $x i64) (param $y i64) (result i32) (i64.le_s (local.get $x) (local.get $y)))
  (func (export "le_u") (param $x i64) (param $y i64) (result i32) (i64.le_u (local.get $x) (local.get $y)))
  (func (export "gt_s") (param $x i64) (param $y i64) (result i32) (i64.gt_s (local.get $x) (local.get $y)))
  (func (export "gt_u") (param $x i64) (param $y i64) (result i32) (i64.gt_u (local.get $x) (local.get $y)))
  (func (export "ge_s") (param $x i64) (param $y i64) (result i32) (i64.ge_s (local.get $x) (local.get $y)))
  (func (export "ge_u") (param $x i64) (param $y i64) (result i32) (i64.ge_u (local.get $x) (local.get $y)))
)"#
    );
}

#[test]
fn int_exprs() {
    assert_snapshot!(
        r#"
(module
  (func (export "i32.no_fold_cmp_s_offset") (param $x i32) (param $y i32) (result i32)
    (i32.lt_s (i32.add (local.get $x) (i32.const 1)) (i32.add (local.get $y) (i32.const 1))))
  (func (export "i32.no_fold_cmp_u_offset") (param $x i32) (param $y i32) (result i32)
    (i32.lt_u (i32.add (local.get $x) (i32.const 1)) (i32.add (local.get $y) (i32.const 1))))

  (func (export "i64.no_fold_cmp_s_offset") (param $x i64) (param $y i64) (result i32)
    (i64.lt_s (i64.add (local.get $x) (i64.const 1)) (i64.add (local.get $y) (i64.const 1))))
  (func (export "i64.no_fold_cmp_u_offset") (param $x i64) (param $y i64) (result i32)
    (i64.lt_u (i64.add (local.get $x) (i64.const 1)) (i64.add (local.get $y) (i64.const 1))))

  (func (export "i64.no_fold_wrap_extend_s") (param $x i64) (result i64)
    (i64.extend_i32_s (i32.wrap_i64 (local.get $x))))

  (func (export "i64.no_fold_wrap_extend_u") (param $x i64) (result i64)
    (i64.extend_i32_u (i32.wrap_i64 (local.get $x))))

  (func (export "i32.no_fold_shl_shr_s") (param $x i32) (result i32)
    (i32.shr_s (i32.shl (local.get $x) (i32.const 1)) (i32.const 1)))
  (func (export "i32.no_fold_shl_shr_u") (param $x i32) (result i32)
    (i32.shr_u (i32.shl (local.get $x) (i32.const 1)) (i32.const 1)))

  (func (export "i64.no_fold_shl_shr_s") (param $x i64) (result i64)
    (i64.shr_s (i64.shl (local.get $x) (i64.const 1)) (i64.const 1)))
  (func (export "i64.no_fold_shl_shr_u") (param $x i64) (result i64)
    (i64.shr_u (i64.shl (local.get $x) (i64.const 1)) (i64.const 1)))

  (func (export "i32.no_fold_div_s_mul") (param $x i32) (result i32)
    (i32.mul (i32.div_s (local.get $x) (i32.const 6)) (i32.const 6)))
  (func (export "i32.no_fold_div_u_mul") (param $x i32) (result i32)
    (i32.mul (i32.div_u (local.get $x) (i32.const 6)) (i32.const 6)))

  (func (export "i64.no_fold_div_s_mul") (param $x i64) (result i64)
    (i64.mul (i64.div_s (local.get $x) (i64.const 6)) (i64.const 6)))
  (func (export "i64.no_fold_div_u_mul") (param $x i64) (result i64)
    (i64.mul (i64.div_u (local.get $x) (i64.const 6)) (i64.const 6)))

  (func (export "i32.no_fold_div_s_self") (param $x i32) (result i32)
    (i32.div_s (local.get $x) (local.get $x)))
  (func (export "i32.no_fold_div_u_self") (param $x i32) (result i32)
    (i32.div_u (local.get $x) (local.get $x)))

  (func (export "i64.no_fold_div_s_self") (param $x i64) (result i64)
    (i64.div_s (local.get $x) (local.get $x)))
  (func (export "i64.no_fold_div_u_self") (param $x i64) (result i64)
    (i64.div_u (local.get $x) (local.get $x)))

  (func (export "i32.no_fold_rem_s_self") (param $x i32) (result i32)
    (i32.rem_s (local.get $x) (local.get $x)))
  (func (export "i32.no_fold_rem_u_self") (param $x i32) (result i32)
    (i32.rem_u (local.get $x) (local.get $x)))

  (func (export "i64.no_fold_rem_s_self") (param $x i64) (result i64)
    (i64.rem_s (local.get $x) (local.get $x)))
  (func (export "i64.no_fold_rem_u_self") (param $x i64) (result i64)
    (i64.rem_u (local.get $x) (local.get $x)))

  (func (export "i32.no_fold_mul_div_s") (param $x i32) (result i32)
    (i32.div_s (i32.mul (local.get $x) (i32.const 6)) (i32.const 6)))
  (func (export "i32.no_fold_mul_div_u") (param $x i32) (result i32)
    (i32.div_u (i32.mul (local.get $x) (i32.const 6)) (i32.const 6)))

  (func (export "i64.no_fold_mul_div_s") (param $x i64) (result i64)
    (i64.div_s (i64.mul (local.get $x) (i64.const 6)) (i64.const 6)))
  (func (export "i64.no_fold_mul_div_u") (param $x i64) (result i64)
    (i64.div_u (i64.mul (local.get $x) (i64.const 6)) (i64.const 6)))

  (func (export "i32.no_fold_div_s_2") (param $x i32) (result i32)
    (i32.div_s (local.get $x) (i32.const 2)))

  (func (export "i64.no_fold_div_s_2") (param $x i64) (result i64)
    (i64.div_s (local.get $x) (i64.const 2)))

  (func (export "i32.no_fold_rem_s_2") (param $x i32) (result i32)
    (i32.rem_s (local.get $x) (i32.const 2)))

  (func (export "i64.no_fold_rem_s_2") (param $x i64) (result i64)
    (i64.rem_s (local.get $x) (i64.const 2)))

  (func (export "i32.div_s_0") (param $x i32) (result i32)
    (i32.div_s (local.get $x) (i32.const 0)))
  (func (export "i32.div_u_0") (param $x i32) (result i32)
    (i32.div_u (local.get $x) (i32.const 0)))

  (func (export "i64.div_s_0") (param $x i64) (result i64)
    (i64.div_s (local.get $x) (i64.const 0)))
  (func (export "i64.div_u_0") (param $x i64) (result i64)
    (i64.div_u (local.get $x) (i64.const 0)))

  (func (export "i32.div_s_3") (param $x i32) (result i32)
    (i32.div_s (local.get $x) (i32.const 3)))
  (func (export "i32.div_u_3") (param $x i32) (result i32)
    (i32.div_u (local.get $x) (i32.const 3)))

  (func (export "i64.div_s_3") (param $x i64) (result i64)
    (i64.div_s (local.get $x) (i64.const 3)))
  (func (export "i64.div_u_3") (param $x i64) (result i64)
    (i64.div_u (local.get $x) (i64.const 3)))

  (func (export "i32.div_s_5") (param $x i32) (result i32)
    (i32.div_s (local.get $x) (i32.const 5)))
  (func (export "i32.div_u_5") (param $x i32) (result i32)
    (i32.div_u (local.get $x) (i32.const 5)))

  (func (export "i64.div_s_5") (param $x i64) (result i64)
    (i64.div_s (local.get $x) (i64.const 5)))
  (func (export "i64.div_u_5") (param $x i64) (result i64)
    (i64.div_u (local.get $x) (i64.const 5)))

  (func (export "i32.div_s_7") (param $x i32) (result i32)
    (i32.div_s (local.get $x) (i32.const 7)))
  (func (export "i32.div_u_7") (param $x i32) (result i32)
    (i32.div_u (local.get $x) (i32.const 7)))

  (func (export "i64.div_s_7") (param $x i64) (result i64)
    (i64.div_s (local.get $x) (i64.const 7)))
  (func (export "i64.div_u_7") (param $x i64) (result i64)
    (i64.div_u (local.get $x) (i64.const 7)))

  (func (export "i32.rem_s_3") (param $x i32) (result i32)
    (i32.rem_s (local.get $x) (i32.const 3)))
  (func (export "i32.rem_u_3") (param $x i32) (result i32)
    (i32.rem_u (local.get $x) (i32.const 3)))

  (func (export "i64.rem_s_3") (param $x i64) (result i64)
    (i64.rem_s (local.get $x) (i64.const 3)))
  (func (export "i64.rem_u_3") (param $x i64) (result i64)
    (i64.rem_u (local.get $x) (i64.const 3)))

  (func (export "i32.rem_s_5") (param $x i32) (result i32)
    (i32.rem_s (local.get $x) (i32.const 5)))
  (func (export "i32.rem_u_5") (param $x i32) (result i32)
    (i32.rem_u (local.get $x) (i32.const 5)))

  (func (export "i64.rem_s_5") (param $x i64) (result i64)
    (i64.rem_s (local.get $x) (i64.const 5)))
  (func (export "i64.rem_u_5") (param $x i64) (result i64)
    (i64.rem_u (local.get $x) (i64.const 5)))

  (func (export "i32.rem_s_7") (param $x i32) (result i32)
    (i32.rem_s (local.get $x) (i32.const 7)))
  (func (export "i32.rem_u_7") (param $x i32) (result i32)
    (i32.rem_u (local.get $x) (i32.const 7)))

  (func (export "i64.rem_s_7") (param $x i64) (result i64)
    (i64.rem_s (local.get $x) (i64.const 7)))
  (func (export "i64.rem_u_7") (param $x i64) (result i64)
    (i64.rem_u (local.get $x) (i64.const 7)))

  (func (export "i32.no_fold_div_neg1") (param $x i32) (result i32)
    (i32.div_s (local.get $x) (i32.const -1)))

  (func (export "i64.no_fold_div_neg1") (param $x i64) (result i64)
    (i64.div_s (local.get $x) (i64.const -1)))
)
"#
    );
}

#[test]
fn int_literals() {
    assert_snapshot!(
        r#"
(module
  (func (export "i32.test") (result i32) (return (i32.const 0x0bAdD00D)))
  (func (export "i32.umax") (result i32) (return (i32.const 0xffffffff)))
  (func (export "i32.smax") (result i32) (return (i32.const 0x7fffffff)))
  (func (export "i32.neg_smax") (result i32) (return (i32.const -0x7fffffff)))
  (func (export "i32.smin") (result i32) (return (i32.const -0x80000000)))
  (func (export "i32.alt_smin") (result i32) (return (i32.const 0x80000000)))
  (func (export "i32.inc_smin") (result i32) (return (i32.add (i32.const -0x80000000) (i32.const 1))))
  (func (export "i32.neg_zero") (result i32) (return (i32.const -0x0)))
  (func (export "i32.not_octal") (result i32) (return (i32.const 010)))
  (func (export "i32.unsigned_decimal") (result i32) (return (i32.const 4294967295)))
  (func (export "i32.plus_sign") (result i32) (return (i32.const +42)))

  (func (export "i64.test") (result i64) (return (i64.const 0x0CABBA6E0ba66a6e)))
  (func (export "i64.umax") (result i64) (return (i64.const 0xffffffffffffffff)))
  (func (export "i64.smax") (result i64) (return (i64.const 0x7fffffffffffffff)))
  (func (export "i64.neg_smax") (result i64) (return (i64.const -0x7fffffffffffffff)))
  (func (export "i64.smin") (result i64) (return (i64.const -0x8000000000000000)))
  (func (export "i64.alt_smin") (result i64) (return (i64.const 0x8000000000000000)))
  (func (export "i64.inc_smin") (result i64) (return (i64.add (i64.const -0x8000000000000000) (i64.const 1))))
  (func (export "i64.neg_zero") (result i64) (return (i64.const -0x0)))
  (func (export "i64.not_octal") (result i64) (return (i64.const 010)))
  (func (export "i64.unsigned_decimal") (result i64) (return (i64.const 18446744073709551615)))
  (func (export "i64.plus_sign") (result i64) (return (i64.const +42)))

  (func (export "i32-dec-sep1") (result i32) (i32.const 1_000_000))
  (func (export "i32-dec-sep2") (result i32) (i32.const 1_0_0_0))
  (func (export "i32-hex-sep1") (result i32) (i32.const 0xa_0f_00_99))
  (func (export "i32-hex-sep2") (result i32) (i32.const 0x1_a_A_0_f))

  (func (export "i64-dec-sep1") (result i64) (i64.const 1_000_000))
  (func (export "i64-dec-sep2") (result i64) (i64.const 1_0_0_0))
  (func (export "i64-hex-sep1") (result i64) (i64.const 0xa_f00f_0000_9999))
  (func (export "i64-hex-sep2") (result i64) (i64.const 0x1_a_A_0_f))
)
"#
    );
}

#[test]
fn f32_arith() {
    assert_snapshot!(
        r#"
(module
  (func (export "add") (param $x f32) (param $y f32) (result f32) (f32.add (local.get $x) (local.get $y)))
  (func (export "sub") (param $x f32) (param $y f32) (result f32) (f32.sub (local.get $x) (local.get $y)))
  (func (export "mul") (param $x f32) (param $y f32) (result f32) (f32.mul (local.get $x) (local.get $y)))
  (func (export "div") (param $x f32) (param $y f32) (result f32) (f32.div (local.get $x) (local.get $y)))
  (func (export "sqrt") (param $x f32) (result f32) (f32.sqrt (local.get $x)))
  (func (export "min") (param $x f32) (param $y f32) (result f32) (f32.min (local.get $x) (local.get $y)))
  (func (export "max") (param $x f32) (param $y f32) (result f32) (f32.max (local.get $x) (local.get $y)))
  (func (export "ceil") (param $x f32) (result f32) (f32.ceil (local.get $x)))
  (func (export "floor") (param $x f32) (result f32) (f32.floor (local.get $x)))
  (func (export "trunc") (param $x f32) (result f32) (f32.trunc (local.get $x)))
  (func (export "nearest") (param $x f32) (result f32) (f32.nearest (local.get $x)))

  (func (export "abs") (param $x f32) (result f32) (f32.abs (local.get $x)))
  (func (export "neg") (param $x f32) (result f32) (f32.neg (local.get $x)))
  (func (export "copysign") (param $x f32) (param $y f32) (result f32) (f32.copysign (local.get $x) (local.get $y)))

  (func (export "eq") (param $x f32) (param $y f32) (result i32) (f32.eq (local.get $x) (local.get $y)))
  (func (export "ne") (param $x f32) (param $y f32) (result i32) (f32.ne (local.get $x) (local.get $y)))
  (func (export "lt") (param $x f32) (param $y f32) (result i32) (f32.lt (local.get $x) (local.get $y)))
  (func (export "le") (param $x f32) (param $y f32) (result i32) (f32.le (local.get $x) (local.get $y)))
  (func (export "gt") (param $x f32) (param $y f32) (result i32) (f32.gt (local.get $x) (local.get $y)))
  (func (export "ge") (param $x f32) (param $y f32) (result i32) (f32.ge (local.get $x) (local.get $y)))
)
"#
    );
}

#[test]
fn f64_arith() {
    assert_snapshot!(
        r#"
(module
  (func (export "add") (param $x f64) (param $y f64) (result f64) (f64.add (local.get $x) (local.get $y)))
  (func (export "sub") (param $x f64) (param $y f64) (result f64) (f64.sub (local.get $x) (local.get $y)))
  (func (export "mul") (param $x f64) (param $y f64) (result f64) (f64.mul (local.get $x) (local.get $y)))
  (func (export "div") (param $x f64) (param $y f64) (result f64) (f64.div (local.get $x) (local.get $y)))
  (func (export "sqrt") (param $x f64) (result f64) (f64.sqrt (local.get $x)))
  (func (export "min") (param $x f64) (param $y f64) (result f64) (f64.min (local.get $x) (local.get $y)))
  (func (export "max") (param $x f64) (param $y f64) (result f64) (f64.max (local.get $x) (local.get $y)))
  (func (export "ceil") (param $x f64) (result f64) (f64.ceil (local.get $x)))
  (func (export "floor") (param $x f64) (result f64) (f64.floor (local.get $x)))
  (func (export "trunc") (param $x f64) (result f64) (f64.trunc (local.get $x)))
  (func (export "nearest") (param $x f64) (result f64) (f64.nearest (local.get $x)))

  (func (export "abs") (param $x f64) (result f64) (f64.abs (local.get $x)))
  (func (export "neg") (param $x f64) (result f64) (f64.neg (local.get $x)))
  (func (export "copysign") (param $x f64) (param $y f64) (result f64) (f64.copysign (local.get $x) (local.get $y)))

  (func (export "eq") (param $x f64) (param $y f64) (result i32) (f64.eq (local.get $x) (local.get $y)))
  (func (export "ne") (param $x f64) (param $y f64) (result i32) (f64.ne (local.get $x) (local.get $y)))
  (func (export "lt") (param $x f64) (param $y f64) (result i32) (f64.lt (local.get $x) (local.get $y)))
  (func (export "le") (param $x f64) (param $y f64) (result i32) (f64.le (local.get $x) (local.get $y)))
  (func (export "gt") (param $x f64) (param $y f64) (result i32) (f64.gt (local.get $x) (local.get $y)))
  (func (export "ge") (param $x f64) (param $y f64) (result i32) (f64.ge (local.get $x) (local.get $y)))
)
"#
    );
}

#[test]
fn float_literals() {
    assert_snapshot!(
        r#"
(module
  ;; f32 special values
  (func (export "f32.nan") (result i32) (i32.reinterpret_f32 (f32.const nan)))
  (func (export "f32.positive_nan") (result i32) (i32.reinterpret_f32 (f32.const +nan)))
  (func (export "f32.negative_nan") (result i32) (i32.reinterpret_f32 (f32.const -nan)))
  (func (export "f32.plain_nan") (result i32) (i32.reinterpret_f32 (f32.const nan:0x400000)))
  (func (export "f32.informally_known_as_plain_snan") (result i32) (i32.reinterpret_f32 (f32.const nan:0x200000)))
  (func (export "f32.all_ones_nan") (result i32) (i32.reinterpret_f32 (f32.const -nan:0x7fffff)))
  (func (export "f32.misc_nan") (result i32) (i32.reinterpret_f32 (f32.const nan:0x012345)))
  (func (export "f32.misc_positive_nan") (result i32) (i32.reinterpret_f32 (f32.const +nan:0x304050)))
  (func (export "f32.misc_negative_nan") (result i32) (i32.reinterpret_f32 (f32.const -nan:0x2abcde)))
  (func (export "f32.infinity") (result i32) (i32.reinterpret_f32 (f32.const inf)))
  (func (export "f32.positive_infinity") (result i32) (i32.reinterpret_f32 (f32.const +inf)))
  (func (export "f32.negative_infinity") (result i32) (i32.reinterpret_f32 (f32.const -inf)))

  ;; f32 numbers
  (func (export "f32.zero") (result i32) (i32.reinterpret_f32 (f32.const 0x0.0p0)))
  (func (export "f32.positive_zero") (result i32) (i32.reinterpret_f32 (f32.const +0x0.0p0)))
  (func (export "f32.negative_zero") (result i32) (i32.reinterpret_f32 (f32.const -0x0.0p0)))
  (func (export "f32.misc") (result i32) (i32.reinterpret_f32 (f32.const 0x1.921fb6p+2)))
  (func (export "f32.min_positive") (result i32) (i32.reinterpret_f32 (f32.const 0x1p-149)))
  (func (export "f32.min_normal") (result i32) (i32.reinterpret_f32 (f32.const 0x1p-126)))
  (func (export "f32.max_finite") (result i32) (i32.reinterpret_f32 (f32.const 0x1.fffffep+127)))
  (func (export "f32.max_subnormal") (result i32) (i32.reinterpret_f32 (f32.const 0x1.fffffcp-127)))
  (func (export "f32.trailing_dot") (result i32) (i32.reinterpret_f32 (f32.const 0x1.p10)))
  (func (export "f32.misc_int") (result i32) (i32.reinterpret_f32 (f32.const 0x12345)))
  (func (export "f32.large_int") (result i32) (i32.reinterpret_f32 (f32.const 0x1_0000_0000_0000_0000_0000)))
  (func (export "f32.min_int32") (result i32) (i32.reinterpret_f32 (f32.const -0x8000_0000)))
  (func (export "f32.min_int64") (result i32) (i32.reinterpret_f32 (f32.const -0x8000_0000_0000_0000)))

  ;; f32 in decimal format
  (func (export "f32_dec.zero") (result i32) (i32.reinterpret_f32 (f32.const 0.0e0)))
  (func (export "f32_dec.positive_zero") (result i32) (i32.reinterpret_f32 (f32.const +0.0e0)))
  (func (export "f32_dec.negative_zero") (result i32) (i32.reinterpret_f32 (f32.const -0.0e0)))
  (func (export "f32_dec.misc") (result i32) (i32.reinterpret_f32 (f32.const 6.28318548202514648)))
  (func (export "f32_dec.min_positive") (result i32) (i32.reinterpret_f32 (f32.const 1.4013e-45)))
  (func (export "f32_dec.min_normal") (result i32) (i32.reinterpret_f32 (f32.const 1.1754944e-38)))
  (func (export "f32_dec.max_subnormal") (result i32) (i32.reinterpret_f32 (f32.const 1.1754942e-38)))
  (func (export "f32_dec.max_finite") (result i32) (i32.reinterpret_f32 (f32.const 3.4028234e+38)))
  (func (export "f32_dec.trailing_dot") (result i32) (i32.reinterpret_f32 (f32.const 1.e10)))
  (func (export "f32_dec.misc_int") (result i32) (i32.reinterpret_f32 (f32.const 12345)))
  (func (export "f32_dec.large_int") (result i32) (i32.reinterpret_f32 (f32.const 100_000_000_000_000_000_000)))
  (func (export "f32_dec.min_int32") (result i32) (i32.reinterpret_f32 (f32.const -2147483648)))
  (func (export "f32_dec.min_int64") (result i32) (i32.reinterpret_f32 (f32.const -9223372036854775808)))

  ;; https://twitter.com/Archivd/status/994637336506912768
  (func (export "f32_dec.root_beer_float") (result i32) (i32.reinterpret_f32 (f32.const 1.000000119)))

  ;; f64 special values
  (func (export "f64.nan") (result i64) (i64.reinterpret_f64 (f64.const nan)))
  (func (export "f64.positive_nan") (result i64) (i64.reinterpret_f64 (f64.const +nan)))
  (func (export "f64.negative_nan") (result i64) (i64.reinterpret_f64 (f64.const -nan)))
  (func (export "f64.plain_nan") (result i64) (i64.reinterpret_f64 (f64.const nan:0x8000000000000)))
  (func (export "f64.informally_known_as_plain_snan") (result i64) (i64.reinterpret_f64 (f64.const nan:0x4000000000000)))
  (func (export "f64.all_ones_nan") (result i64) (i64.reinterpret_f64 (f64.const -nan:0xfffffffffffff)))
  (func (export "f64.misc_nan") (result i64) (i64.reinterpret_f64 (f64.const nan:0x0123456789abc)))
  (func (export "f64.misc_positive_nan") (result i64) (i64.reinterpret_f64 (f64.const +nan:0x3040506070809)))
  (func (export "f64.misc_negative_nan") (result i64) (i64.reinterpret_f64 (f64.const -nan:0x2abcdef012345)))
  (func (export "f64.infinity") (result i64) (i64.reinterpret_f64 (f64.const inf)))
  (func (export "f64.positive_infinity") (result i64) (i64.reinterpret_f64 (f64.const +inf)))
  (func (export "f64.negative_infinity") (result i64) (i64.reinterpret_f64 (f64.const -inf)))

  ;; f64 numbers
  (func (export "f64.zero") (result i64) (i64.reinterpret_f64 (f64.const 0x0.0p0)))
  (func (export "f64.positive_zero") (result i64) (i64.reinterpret_f64 (f64.const +0x0.0p0)))
  (func (export "f64.negative_zero") (result i64) (i64.reinterpret_f64 (f64.const -0x0.0p0)))
  (func (export "f64.misc") (result i64) (i64.reinterpret_f64 (f64.const 0x1.921fb54442d18p+2)))
  (func (export "f64.min_positive") (result i64) (i64.reinterpret_f64 (f64.const 0x0.0000000000001p-1022)))
  (func (export "f64.min_normal") (result i64) (i64.reinterpret_f64 (f64.const 0x1p-1022)))
  (func (export "f64.max_subnormal") (result i64) (i64.reinterpret_f64 (f64.const 0x0.fffffffffffffp-1022)))
  (func (export "f64.max_finite") (result i64) (i64.reinterpret_f64 (f64.const 0x1.fffffffffffffp+1023)))
  (func (export "f64.trailing_dot") (result i64) (i64.reinterpret_f64 (f64.const 0x1.p100)))
  (func (export "f64.misc_int") (result i64) (i64.reinterpret_f64 (f64.const 0x12345)))
  (func (export "f64.large_int") (result i64) (i64.reinterpret_f64 (f64.const 0x1_0000_0000_0000_0000_0000)))
  (func (export "f64.min_int32") (result i64) (i64.reinterpret_f64 (f64.const -0x8000_0000)))
  (func (export "f64.min_int64") (result i64) (i64.reinterpret_f64 (f64.const -0x8000_0000_0000_0000)))

  ;; f64 numbers in decimal format
  (func (export "f64_dec.zero") (result i64) (i64.reinterpret_f64 (f64.const 0.0e0)))
  (func (export "f64_dec.positive_zero") (result i64) (i64.reinterpret_f64 (f64.const +0.0e0)))
  (func (export "f64_dec.negative_zero") (result i64) (i64.reinterpret_f64 (f64.const -0.0e0)))
  (func (export "f64_dec.misc") (result i64) (i64.reinterpret_f64 (f64.const 6.28318530717958623)))
  (func (export "f64_dec.min_positive") (result i64) (i64.reinterpret_f64 (f64.const 4.94066e-324)))
  (func (export "f64_dec.min_normal") (result i64) (i64.reinterpret_f64 (f64.const 2.2250738585072012e-308)))
  (func (export "f64_dec.max_subnormal") (result i64) (i64.reinterpret_f64 (f64.const 2.2250738585072011e-308)))
  (func (export "f64_dec.max_finite") (result i64) (i64.reinterpret_f64 (f64.const 1.7976931348623157e+308)))
  (func (export "f64_dec.trailing_dot") (result i64) (i64.reinterpret_f64 (f64.const 1.e100)))
  (func (export "f64_dec.misc_int") (result i64) (i64.reinterpret_f64 (f64.const 12345)))
  (func (export "f64_dec.large_int") (result i64) (i64.reinterpret_f64 (f64.const 100_000_000_000_000_000_000)))
  (func (export "f64_dec.min_int32") (result i64) (i64.reinterpret_f64 (f64.const -2147483648)))
  (func (export "f64_dec.min_int64") (result i64) (i64.reinterpret_f64 (f64.const -9223372036854775808)))

  ;; https://twitter.com/Archivd/status/994637336506912768
  (func (export "f64_dec.root_beer_float") (result i64) (i64.reinterpret_f64 (f64.const 1.000000119)))

  (func (export "f32-dec-sep1") (result f32) (f32.const 1_000_000))
  (func (export "f32-dec-sep2") (result f32) (f32.const 1_0_0_0))
  (func (export "f32-dec-sep3") (result f32) (f32.const 100_3.141_592))
  (func (export "f32-dec-sep4") (result f32) (f32.const 99e+1_3))
  (func (export "f32-dec-sep5") (result f32) (f32.const 122_000.11_3_54E0_2_3))
  (func (export "f32-hex-sep1") (result f32) (f32.const 0xa_0f_00_99))
  (func (export "f32-hex-sep2") (result f32) (f32.const 0x1_a_A_0_f))
  (func (export "f32-hex-sep3") (result f32) (f32.const 0xa0_ff.f141_a59a))
  (func (export "f32-hex-sep4") (result f32) (f32.const 0xf0P+1_3))
  (func (export "f32-hex-sep5") (result f32) (f32.const 0x2a_f00a.1f_3_eep2_3))

  (func (export "f64-dec-sep1") (result f64) (f64.const 1_000_000))
  (func (export "f64-dec-sep2") (result f64) (f64.const 1_0_0_0))
  (func (export "f64-dec-sep3") (result f64) (f64.const 100_3.141_592))
  (func (export "f64-dec-sep4") (result f64) (f64.const 99e-1_23))
  (func (export "f64-dec-sep5") (result f64) (f64.const 122_000.11_3_54e0_2_3))
  (func (export "f64-hex-sep1") (result f64) (f64.const 0xa_f00f_0000_9999))
  (func (export "f64-hex-sep2") (result f64) (f64.const 0x1_a_A_0_f))
  (func (export "f64-hex-sep3") (result f64) (f64.const 0xa0_ff.f141_a59a))
  (func (export "f64-hex-sep4") (result f64) (f64.const 0xf0P+1_3))
  (func (export "f64-hex-sep5") (result f64) (f64.const 0x2a_f00a.1f_3_eep2_3))
)
"#
    );
}

#[test]
fn forwald() {
    assert_snapshot!(
        r#"
(module
  (func $even (export "even") (param $n i32) (result i32)
    (if (result i32) (i32.eq (local.get $n) (i32.const 0))
      (then (i32.const 1))
      (else (call $odd (i32.sub (local.get $n) (i32.const 1))))
    )
  )

  (func $odd (export "odd") (param $n i32) (result i32)
    (if (result i32) (i32.eq (local.get $n) (i32.const 0))
      (then (i32.const 0))
      (else (call $even (i32.sub (local.get $n) (i32.const 1))))
    )
  )
)
"#
    );
}

#[test]
fn float_memory() {
    assert_snapshot!(
        r#"
(module
  (memory (data "\00\00\00\00\00\00\00\f4\7f"))

  (func (export "f64.load") (result f64) (f64.load (i32.const 1)))
  (func (export "i64.load") (result i64) (i64.load (i32.const 1)))
  (func (export "f64.store") (f64.store (i32.const 1) (f64.const nan:0x4000000000000)))
  (func (export "i64.store") (i64.store (i32.const 1) (i64.const 0x7ff4000000000000)))
  (func (export "reset") (i64.store (i32.const 1) (i64.const 0)))
)
"#
    );
}

#[test]
fn global_get_and_set() {
    assert_snapshot!(
        r#"
(module
  (global $a i32 (i32.const -2))
  (global $b i64 (i64.const -5))
  (global $x (mut i32) (i32.const -12))
  (global $y (mut i64) (i64.const -15))

  (func (export "get-a") (result i32) (global.get $a))
  (func (export "get-b") (result i64) (global.get $b))
  (func (export "set-x") (param i32) (global.set $x (local.get 0)))
  (func (export "set-y") (param i64) (global.set $y (local.get 0)))
)
"#
    );
}

#[test]
fn local_get_and_set() {
    assert_snapshot!(
        r#"
(module
  (func (export "type-local-i32") (result i32) (local i32) 
    (local.set 0 (i32.const 42))
    (local.get 0)
  )
  (func (export "type-local-i64") (result i64) (local i64) 
    (local.set 0 (i64.const 42))
    (local.get 0)
  )
  (func (export "type-local-f32") (result f32) (local f32) 
    (local.set 0 (f32.const 42.0))
    (local.get 0)
  )
  (func (export "type-local-f64") (result f64) (local f64) 
    (local.set 0 (f64.const 42.0))
    (local.get 0)
  )
)
"#
    );
}

#[test]
fn local_call() {
    assert_snapshot!(
        r#"
(module
    (func $add (param $a i32) (param $b i32) (result i32)
        local.get $a
        local.get $b
        i32.add
    )
    (func $main (result i32)
        i32.const 2
        i32.const 3
        call $add
    )
)
"#
    );
}

#[test]
fn select() {
    assert_snapshot!(
        r#"
(module
  ;; Auxiliary definition
  (func $dummy)

  (func (export "type-i32") (drop (i32.ctz (return))))
  (func (export "type-i64") (drop (i64.ctz (return))))
  (func (export "type-f32") (drop (f32.neg (return))))
  (func (export "type-f64") (drop (f64.neg (return))))

  (func (export "type-i32-value") (result i32)
    (block (result i32) (i32.ctz (return (i32.const 1))))
  )
  (func (export "type-i64-value") (result i64)
    (block (result i64) (i64.ctz (return (i64.const 2))))
  )
  (func (export "type-f32-value") (result f32)
    (block (result f32) (f32.neg (return (f32.const 3))))
  )
  (func (export "type-f64-value") (result f64)
    (block (result f64) (f64.neg (return (f64.const 4))))
  )

  (func (export "nullary") (return))
  (func (export "unary") (result f64) (return (f64.const 3)))

  (func (export "as-func-first") (result i32)
    (return (i32.const 1)) (i32.const 2)
  )
  (func (export "as-func-mid") (result i32)
    (call $dummy) (return (i32.const 2)) (i32.const 3)
  )
  (func (export "as-func-last")
    (nop) (call $dummy) (return)
  )
  (func (export "as-func-value") (result i32)
    (nop) (call $dummy) (return (i32.const 3))
  )

  (func (export "as-block-first")
    (block (return) (call $dummy))
  )
  (func (export "as-block-mid")
    (block (call $dummy) (return) (call $dummy))
  )
  (func (export "as-block-last")
    (block (nop) (call $dummy) (return))
  )
  (func (export "as-block-value") (result i32)
    (block (result i32) (nop) (call $dummy) (return (i32.const 2)))
  )

  (func (export "as-loop-first") (result i32)
    (loop (result i32) (return (i32.const 3)) (i32.const 2))
  )
  (func (export "as-loop-mid") (result i32)
    (loop (result i32) (call $dummy) (return (i32.const 4)) (i32.const 2))
  )
  (func (export "as-loop-last") (result i32)
    (loop (result i32) (nop) (call $dummy) (return (i32.const 5)))
  )

  (func (export "as-br-value") (result i32)
    (block (result i32) (br 0 (return (i32.const 9))))
  )

  (func (export "as-br_if-cond")
    (block (br_if 0 (return)))
  )
  (func (export "as-br_if-value") (result i32)
    (block (result i32)
      (drop (br_if 0 (return (i32.const 8)) (i32.const 1))) (i32.const 7)
    )
  )
  (func (export "as-br_if-value-cond") (result i32)
    (block (result i32)
      (drop (br_if 0 (i32.const 6) (return (i32.const 9)))) (i32.const 7)
    )
  )

  (func (export "as-br_table-index") (result i64)
    (block (br_table 0 0 0 (return (i64.const 9)))) (i64.const -1)
  )
  (func (export "as-br_table-value") (result i32)
    (block (result i32)
      (br_table 0 0 0 (return (i32.const 10)) (i32.const 1)) (i32.const 7)
    )
  )
  (func (export "as-br_table-value-index") (result i32)
    (block (result i32)
      (br_table 0 0 (i32.const 6) (return (i32.const 11))) (i32.const 7)
    )
  )

  (func (export "as-return-value") (result i64)
    (return (return (i64.const 7)))
  )

  (func (export "as-if-cond") (result i32)
    (if (result i32)
      (return (i32.const 2)) (then (i32.const 0)) (else (i32.const 1))
    )
  )
  (func (export "as-if-then") (param i32 i32) (result i32)
    (if (result i32)
      (local.get 0) (then (return (i32.const 3))) (else (local.get 1))
    )
  )
  (func (export "as-if-else") (param i32 i32) (result i32)
    (if (result i32)
      (local.get 0) (then (local.get 1)) (else (return (i32.const 4)))
    )
  )

  (func (export "as-select-first") (param i32 i32) (result i32)
    (select (return (i32.const 5)) (local.get 0) (local.get 1))
  )
  (func (export "as-select-second") (param i32 i32) (result i32)
    (select (local.get 0) (return (i32.const 6)) (local.get 1))
  )
  (func (export "as-select-cond") (result i32)
    (select (i32.const 0) (i32.const 1) (return (i32.const 7)))
  )

  (func $f (param i32 i32 i32) (result i32) (i32.const -1))
  (func (export "as-call-first") (result i32)
    (call $f (return (i32.const 12)) (i32.const 2) (i32.const 3))
  )
  (func (export "as-call-mid") (result i32)
    (call $f (i32.const 1) (return (i32.const 13)) (i32.const 3))
  )
  (func (export "as-call-last") (result i32)
    (call $f (i32.const 1) (i32.const 2) (return (i32.const 14)))
  )

  (type $sig (func (param i32 i32 i32) (result i32)))
  (table funcref (elem $f))
  (func (export "as-call_indirect-func") (result i32)
    (call_indirect (type $sig)
      (return (i32.const 20)) (i32.const 1) (i32.const 2) (i32.const 3)
    )
  )
  (func (export "as-call_indirect-first") (result i32)
    (call_indirect (type $sig)
      (i32.const 0) (return (i32.const 21)) (i32.const 2) (i32.const 3)
    )
  )
  (func (export "as-call_indirect-mid") (result i32)
    (call_indirect (type $sig)
      (i32.const 0) (i32.const 1) (return (i32.const 22)) (i32.const 3)
    )
  )
  (func (export "as-call_indirect-last") (result i32)
    (call_indirect (type $sig)
      (i32.const 0) (i32.const 1) (i32.const 2) (return (i32.const 23))
    )
  )

  (func (export "as-local.set-value") (result i32) (local f32)
    (local.set 0 (return (i32.const 17))) (i32.const -1)
  )
  (func (export "as-local.tee-value") (result i32) (local i32)
    (local.tee 0 (return (i32.const 1)))
  )
  (global $a (mut i32) (i32.const 0))
  (func (export "as-global.set-value") (result i32)
    (global.set $a (return (i32.const 1)))
  )

  (memory 1)
  (func (export "as-load-address") (result f32)
    (f32.load (return (f32.const 1.7)))
  )
  (func (export "as-loadN-address") (result i64)
    (i64.load8_s (return (i64.const 30)))
  )

  (func (export "as-store-address") (result i32)
    (f64.store (return (i32.const 30)) (f64.const 7)) (i32.const -1)
  )
  (func (export "as-store-value") (result i32)
    (i64.store (i32.const 2) (return (i32.const 31))) (i32.const -1)
  )

  (func (export "as-storeN-address") (result i32)
    (i32.store8 (return (i32.const 32)) (i32.const 7)) (i32.const -1)
  )
  (func (export "as-storeN-value") (result i32)
    (i64.store16 (i32.const 2) (return (i32.const 33))) (i32.const -1)
  )

  (func (export "as-unary-operand") (result f32)
    (f32.neg (return (f32.const 3.4)))
  )

  (func (export "as-binary-left") (result i32)
    (i32.add (return (i32.const 3)) (i32.const 10))
  )
  (func (export "as-binary-right") (result i64)
    (i64.sub (i64.const 10) (return (i64.const 45)))
  )

  (func (export "as-test-operand") (result i32)
    (i32.eqz (return (i32.const 44)))
  )

  (func (export "as-compare-left") (result i32)
    (f64.le (return (i32.const 43)) (f64.const 10))
  )
  (func (export "as-compare-right") (result i32)
    (f32.ne (f32.const 10) (return (i32.const 42)))
  )

  (func (export "as-convert-operand") (result i32)
    (i32.wrap_i64 (return (i32.const 41)))
  )

  (func (export "as-memory.grow-size") (result i32)
    (memory.grow (return (i32.const 40)))
  )
)
"#
    );
}

#[test]
fn mem_i32_load() {
    assert_snapshot!(
        r#"
(module
  (memory 1)
  (func (export "as-select-first") (param i32 i32) (result i32)
    (select (i32.load (i32.const 0)) (local.get 0) (local.get 1))
  )
)
"#
    );
}

#[test]
fn address() {
    assert_snapshot!(
        r#"
(module
  (memory 1)
  (data (i32.const 0) "abcdefghijklmnopqrstuvwxyz")

  (func (export "8u_good1") (param $i i32) (result i32)
    (i32.load8_u offset=0 (local.get $i))                   ;; 97 'a'
  )
  (func (export "8u_good2") (param $i i32) (result i32)
    (i32.load8_u align=1 (local.get $i))                    ;; 97 'a'
  )
  (func (export "8u_good3") (param $i i32) (result i32)
    (i32.load8_u offset=1 align=1 (local.get $i))           ;; 98 'b'
  )
  (func (export "8u_good4") (param $i i32) (result i32)
    (i32.load8_u offset=2 align=1 (local.get $i))           ;; 99 'c'
  )
  (func (export "8u_good5") (param $i i32) (result i32)
    (i32.load8_u offset=25 align=1 (local.get $i))          ;; 122 'z'
  )

  (func (export "8s_good1") (param $i i32) (result i32)
    (i32.load8_s offset=0 (local.get $i))                   ;; 97 'a'
  )
  (func (export "8s_good2") (param $i i32) (result i32)
    (i32.load8_s align=1 (local.get $i))                    ;; 97 'a'
  )
  (func (export "8s_good3") (param $i i32) (result i32)
    (i32.load8_s offset=1 align=1 (local.get $i))           ;; 98 'b'
  )
  (func (export "8s_good4") (param $i i32) (result i32)
    (i32.load8_s offset=2 align=1 (local.get $i))           ;; 99 'c'
  )
  (func (export "8s_good5") (param $i i32) (result i32)
    (i32.load8_s offset=25 align=1 (local.get $i))          ;; 122 'z'
  )

  (func (export "16u_good1") (param $i i32) (result i32)
    (i32.load16_u offset=0 (local.get $i))                  ;; 25185 'ab'
  )
  (func (export "16u_good2") (param $i i32) (result i32)
    (i32.load16_u align=1 (local.get $i))                   ;; 25185 'ab'
  )
  (func (export "16u_good3") (param $i i32) (result i32)
    (i32.load16_u offset=1 align=1 (local.get $i))          ;; 25442 'bc'
  )
  (func (export "16u_good4") (param $i i32) (result i32)
    (i32.load16_u offset=2 align=2 (local.get $i))          ;; 25699 'cd'
  )
  (func (export "16u_good5") (param $i i32) (result i32)
    (i32.load16_u offset=25 align=2 (local.get $i))         ;; 122 'z\0'
  )

  (func (export "16s_good1") (param $i i32) (result i32)
    (i32.load16_s offset=0 (local.get $i))                  ;; 25185 'ab'
  )
  (func (export "16s_good2") (param $i i32) (result i32)
    (i32.load16_s align=1 (local.get $i))                   ;; 25185 'ab'
  )
  (func (export "16s_good3") (param $i i32) (result i32)
    (i32.load16_s offset=1 align=1 (local.get $i))          ;; 25442 'bc'
  )
  (func (export "16s_good4") (param $i i32) (result i32)
    (i32.load16_s offset=2 align=2 (local.get $i))          ;; 25699 'cd'
  )
  (func (export "16s_good5") (param $i i32) (result i32)
    (i32.load16_s offset=25 align=2 (local.get $i))         ;; 122 'z\0'
  )

  (func (export "32_good1") (param $i i32) (result i32)
    (i32.load offset=0 (local.get $i))                      ;; 1684234849 'abcd'
  )
  (func (export "32_good2") (param $i i32) (result i32)
    (i32.load align=1 (local.get $i))                       ;; 1684234849 'abcd'
  )
  (func (export "32_good3") (param $i i32) (result i32)
    (i32.load offset=1 align=1 (local.get $i))              ;; 1701077858 'bcde'
  )
  (func (export "32_good4") (param $i i32) (result i32)
    (i32.load offset=2 align=2 (local.get $i))              ;; 1717920867 'cdef'
  )
  (func (export "32_good5") (param $i i32) (result i32)
    (i32.load offset=25 align=4 (local.get $i))             ;; 122 'z\0\0\0'
  )

  (func (export "8u_bad") (param $i i32)
    (drop (i32.load8_u offset=4294967295 (local.get $i)))
  )
  (func (export "8s_bad") (param $i i32)
    (drop (i32.load8_s offset=4294967295 (local.get $i)))
  )
  (func (export "16u_bad") (param $i i32)
    (drop (i32.load16_u offset=4294967295 (local.get $i)))
  )
  (func (export "16s_bad") (param $i i32)
    (drop (i32.load16_s offset=4294967295 (local.get $i)))
  )
  (func (export "32_bad") (param $i i32)
    (drop (i32.load offset=4294967295 (local.get $i)))
  )
)
"#
    );
}

#[test]
fn align() {
    assert_snapshot!(
        r#"
(module
  (memory 0)
  (func (export "load_i32_8s") (drop (i32.load8_s align=1 (i32.const 0))))
  (func (export "load_i32_8u") (drop (i32.load8_u align=1 (i32.const 0))))
  (func (export "load_i32_16s") (drop (i32.load16_s align=2 (i32.const 0))))
  (func (export "load_i32_16u") (drop (i32.load16_u align=2 (i32.const 0))))
  (func (export "load_i32") (drop (i32.load align=4 (i32.const 0))))
  (func (export "load_i64_8s") (drop (i64.load8_s align=1 (i32.const 0))))
  (func (export "load_i64_8u") (drop (i64.load8_u align=1 (i32.const 0))))
  (func (export "load_i64_16s") (drop (i64.load16_s align=2 (i32.const 0))))
  (func (export "load_i64_16u") (drop (i64.load16_u align=2 (i32.const 0))))
  (func (export "load_i64_32s") (drop (i64.load32_s align=4 (i32.const 0))))
  (func (export "load_i64_32u") (drop (i64.load32_u align=4 (i32.const 0))))
  (func (export "load_i64") (drop (i64.load align=8 (i32.const 0))))
  (func (export "load_f32") (drop (f32.load align=4 (i32.const 0))))
  (func (export "load_f64") (drop (f64.load align=8 (i32.const 0))))
  (func (export "store_i32_8") (i32.store8 align=1 (i32.const 0) (i32.const 1)))
  (func (export "store_i32_16") (i32.store16 align=2 (i32.const 0) (i32.const 1)))
  (func (export "store_i32") (i32.store align=4 (i32.const 0) (i32.const 1)))
  (func (export "store_i64_8") (i64.store8 align=1 (i32.const 0) (i64.const 1)))
  (func (export "store_i64_16") (i64.store16 align=2 (i32.const 0) (i64.const 1)))
  (func (export "store_i64_32") (i64.store32 align=4 (i32.const 0) (i64.const 1)))
  (func (export "store_i64") (i64.store align=8 (i32.const 0) (i64.const 1)))
  (func (export "store_f32") (f32.store align=4 (i32.const 0) (f32.const 1.0)))
  (func (export "store_f64") (f64.store align=8 (i32.const 0) (f64.const 1.0)))
)
"#
    );
}

#[test]
fn block() {
    assert_snapshot!(
        r#"
(module
  ;; Auxiliary definition
  (memory 1)

  (func $dummy)

  (func (export "empty")
    (block)
    (block $l)
  )

  (func (export "singular") (result i32)
    (block (nop))
    (block (result i32) (i32.const 7))
  )

  (func (export "multi") (result i32)
    (block (call $dummy) (call $dummy) (call $dummy) (call $dummy))
    (block (result i32)
      (call $dummy) (call $dummy) (call $dummy) (i32.const 7) (call $dummy)
    )
    (drop)
    (block (result i32 i64 i32)
      (call $dummy) (call $dummy) (call $dummy) (i32.const 8) (call $dummy)
      (call $dummy) (call $dummy) (call $dummy) (i64.const 7) (call $dummy)
      (call $dummy) (call $dummy) (call $dummy) (i32.const 9) (call $dummy)
    )
    (drop) (drop)
  )

  (func (export "nested") (result i32)
    (block (result i32)
      (block (call $dummy) (block) (nop))
      (block (result i32) (call $dummy) (i32.const 9))
    )
  )

  (func (export "deep") (result i32)
    (block (result i32) (block (result i32)
      (block (result i32) (block (result i32)
        (block (result i32) (block (result i32)
          (block (result i32) (block (result i32)
            (block (result i32) (block (result i32)
              (block (result i32) (block (result i32)
                (block (result i32) (block (result i32)
                  (block (result i32) (block (result i32)
                    (block (result i32) (block (result i32)
                      (block (result i32) (block (result i32)
                        (block (result i32) (block (result i32)
                          (block (result i32) (block (result i32)
                            (block (result i32) (block (result i32)
                              (block (result i32) (block (result i32)
                                (block (result i32) (block (result i32)
                                  (block (result i32) (block (result i32)
                                    (block (result i32) (block (result i32)
                                      (block (result i32) (block (result i32)
                                        (block (result i32) (block (result i32)
                                          (call $dummy) (i32.const 150)
                                        ))
                                      ))
                                    ))
                                  ))
                                ))
                              ))
                            ))
                          ))
                        ))
                      ))
                    ))
                  ))
                ))
              ))
            ))
          ))
        ))
      ))
    ))
  )

  (func (export "as-select-first") (result i32)
    (select (block (result i32) (i32.const 1)) (i32.const 2) (i32.const 3))
  )
  (func (export "as-select-mid") (result i32)
    (select (i32.const 2) (block (result i32) (i32.const 1)) (i32.const 3))
  )
  (func (export "as-select-last") (result i32)
    (select (i32.const 2) (i32.const 3) (block (result i32) (i32.const 1)))
  )

  (func (export "as-loop-first") (result i32)
    (loop (result i32) (block (result i32) (i32.const 1)) (call $dummy) (call $dummy))
  )
  (func (export "as-loop-mid") (result i32)
    (loop (result i32) (call $dummy) (block (result i32) (i32.const 1)) (call $dummy))
  )
  (func (export "as-loop-last") (result i32)
    (loop (result i32) (call $dummy) (call $dummy) (block (result i32) (i32.const 1)))
  )

  (func (export "as-if-condition")
    (block (result i32) (i32.const 1)) (if (then (call $dummy)))
  )
  (func (export "as-if-then") (result i32)
    (if (result i32) (i32.const 1) (then (block (result i32) (i32.const 1))) (else (i32.const 2)))
  )
  (func (export "as-if-else") (result i32)
    (if (result i32) (i32.const 1) (then (i32.const 2)) (else (block (result i32) (i32.const 1))))
  )

  (func (export "as-br_if-first") (result i32)
    (block (result i32) (br_if 0 (block (result i32) (i32.const 1)) (i32.const 2)))
  )
  (func (export "as-br_if-last") (result i32)
    (block (result i32) (br_if 0 (i32.const 2) (block (result i32) (i32.const 1))))
  )

  (func (export "as-br_table-first") (result i32)
    (block (result i32) (block (result i32) (i32.const 1)) (i32.const 2) (br_table 0 0))
  )
  (func (export "as-br_table-last") (result i32)
    (block (result i32) (i32.const 2) (block (result i32) (i32.const 1)) (br_table 0 0))
  )

  (func $func (param i32 i32) (result i32) (local.get 0))
  (type $check (func (param i32 i32) (result i32)))
  (table funcref (elem $func))
  (func (export "as-call_indirect-first") (result i32)
    (block (result i32)
      (call_indirect (type $check)
        (block (result i32) (i32.const 1)) (i32.const 2) (i32.const 0)
      )
    )
  )
  (func (export "as-call_indirect-mid") (result i32)
    (block (result i32)
      (call_indirect (type $check)
        (i32.const 2) (block (result i32) (i32.const 1)) (i32.const 0)
      )
    )
  )
  (func (export "as-call_indirect-last") (result i32)
    (block (result i32)
      (call_indirect (type $check)
        (i32.const 1) (i32.const 2) (block (result i32) (i32.const 0))
      )
    )
  )

  (func (export "as-store-first")
    (block (result i32) (i32.const 1)) (i32.const 1) (i32.store)
  )
  (func (export "as-store-last")
    (i32.const 10) (block (result i32) (i32.const 1)) (i32.store)
  )

  (func (export "as-memory.grow-value") (result i32)
    (memory.grow (block (result i32) (i32.const 1)))
  )

  (func $f (param i32) (result i32) (local.get 0))

  (func (export "as-call-value") (result i32)
    (call $f (block (result i32) (i32.const 1)))
  )
  (func (export "as-return-value") (result i32)
    (block (result i32) (i32.const 1)) (return)
  )
  (func (export "as-drop-operand")
    (drop (block (result i32) (i32.const 1)))
  )
  (func (export "as-br-value") (result i32)
    (block (result i32) (br 0 (block (result i32) (i32.const 1))))
  )
  (func (export "as-local.set-value") (result i32)
    (local i32) (local.set 0 (block (result i32) (i32.const 1))) (local.get 0)
  )
  (func (export "as-local.tee-value") (result i32)
    (local i32) (local.tee 0 (block (result i32) (i32.const 1)))
  )
  (global $a (mut i32) (i32.const 10))
  (func (export "as-global.set-value") (result i32)
    (global.set $a (block (result i32) (i32.const 1)))
    (global.get $a)
  )

  (func (export "as-load-operand") (result i32)
    (i32.load (block (result i32) (i32.const 1)))
  )

  (func (export "as-unary-operand") (result i32)
    (i32.ctz (block (result i32) (call $dummy) (i32.const 13)))
  )
  (func (export "as-binary-operand") (result i32)
    (i32.mul
      (block (result i32) (call $dummy) (i32.const 3))
      (block (result i32) (call $dummy) (i32.const 4))
    )
  )
  (func (export "as-test-operand") (result i32)
    (i32.eqz (block (result i32) (call $dummy) (i32.const 13)))
  )
  (func (export "as-compare-operand") (result i32)
    (f32.gt
      (block (result f32) (call $dummy) (f32.const 3))
      (block (result f32) (call $dummy) (f32.const 3))
    )
  )
  (func (export "as-binary-operands") (result i32)
    (i32.mul
      (block (result i32 i32)
        (call $dummy) (i32.const 3) (call $dummy) (i32.const 4)
      )
    )
  )
  (func (export "as-compare-operands") (result i32)
    (f32.gt
      (block (result f32 f32)
        (call $dummy) (f32.const 3) (call $dummy) (f32.const 3)
      )
    )
  )
  (func (export "as-mixed-operands") (result i32)
    (block (result i32 i32)
      (call $dummy) (i32.const 3) (call $dummy) (i32.const 4)
    )
    (i32.const 5)
    (i32.add)
    (i32.mul)
  )

  (func (export "break-bare") (result i32)
    (block (br 0) (unreachable))
    (block (br_if 0 (i32.const 1)) (unreachable))
    (block (br_table 0 (i32.const 0)) (unreachable))
    (block (br_table 0 0 0 (i32.const 1)) (unreachable))
    (i32.const 19)
  )
  (func (export "break-value") (result i32)
    (block (result i32) (br 0 (i32.const 18)) (i32.const 19))
  )
  (func (export "break-multi-value") (result i32 i32 i64)
    (block (result i32 i32 i64)
      (br 0 (i32.const 18) (i32.const -18) (i64.const 18))
      (i32.const 19) (i32.const -19) (i64.const 19)
    )
  )
  (func (export "break-repeated") (result i32)
    (block (result i32)
      (br 0 (i32.const 18))
      (br 0 (i32.const 19))
      (drop (br_if 0 (i32.const 20) (i32.const 0)))
      (drop (br_if 0 (i32.const 20) (i32.const 1)))
      (br 0 (i32.const 21))
      (br_table 0 (i32.const 22) (i32.const 4))
      (br_table 0 0 0 (i32.const 23) (i32.const 1))
      (i32.const 21)
    )
  )
  (func (export "break-inner") (result i32)
    (local i32)
    (local.set 0 (i32.const 0))
    (local.set 0 (i32.add (local.get 0) (block (result i32) (block (result i32) (br 1 (i32.const 0x1))))))
    (local.set 0 (i32.add (local.get 0) (block (result i32) (block (br 0)) (i32.const 0x2))))
    (local.set 0
      (i32.add (local.get 0) (block (result i32) (i32.ctz (br 0 (i32.const 0x4)))))
    )
    (local.set 0
      (i32.add (local.get 0) (block (result i32) (i32.ctz (block (result i32) (br 1 (i32.const 0x8))))))
    )
    (local.get 0)
  )

  (func (export "param") (result i32)
    (i32.const 1)
    (block (param i32) (result i32)
      (i32.const 2)
      (i32.add)
    )
  )
  (func (export "params") (result i32)
    (i32.const 1)
    (i32.const 2)
    (block (param i32 i32) (result i32)
      (i32.add)
    )
  )
  (func (export "params-id") (result i32)
    (i32.const 1)
    (i32.const 2)
    (block (param i32 i32) (result i32 i32))
    (i32.add)
  )
  (func (export "param-break") (result i32)
    (i32.const 1)
    (block (param i32) (result i32)
      (i32.const 2)
      (i32.add)
      (br 0)
    )
  )
  (func (export "params-break") (result i32)
    (i32.const 1)
    (i32.const 2)
    (block (param i32 i32) (result i32)
      (i32.add)
      (br 0)
    )
  )
  (func (export "params-id-break") (result i32)
    (i32.const 1)
    (i32.const 2)
    (block (param i32 i32) (result i32 i32) (br 0))
    (i32.add)
  )

  (func (export "effects") (result i32)
    (local i32)
    (block
      (local.set 0 (i32.const 1))
      (local.set 0 (i32.mul (local.get 0) (i32.const 3)))
      (local.set 0 (i32.sub (local.get 0) (i32.const 5)))
      (local.set 0 (i32.mul (local.get 0) (i32.const 7)))
      (br 0)
      (local.set 0 (i32.mul (local.get 0) (i32.const 100)))
    )
    (i32.eq (local.get 0) (i32.const -14))
  )

  (type $block-sig-1 (func))
  (type $block-sig-2 (func (result i32)))
  (type $block-sig-3 (func (param $x i32)))
  (type $block-sig-4 (func (param i32 f64 i32) (result i32 f64 i32)))

  (func (export "type-use")
    (block (type $block-sig-1))
    (block (type $block-sig-2) (i32.const 0))
    (block (type $block-sig-3) (drop))
    (i32.const 0) (f64.const 0) (i32.const 0)
    (block (type $block-sig-4))
    (drop) (drop) (drop)
    (block (type $block-sig-2) (result i32) (i32.const 0))
    (block (type $block-sig-3) (param i32) (drop))
    (i32.const 0) (f64.const 0) (i32.const 0)
    (block (type $block-sig-4)
      (param i32) (param f64 i32) (result i32 f64) (result i32)
    )
    (drop) (drop) (drop)
  )
)
"#
    );
}

#[test]
fn br() {
    assert_snapshot!(
        r#"
(module
  (func $dummy)

  (func (export "type-i32") (block (drop (i32.ctz (br 0)))))
  (func (export "type-i64") (block (drop (i64.ctz (br 0)))))
  (func (export "type-f32") (block (drop (f32.neg (br 0)))))
  (func (export "type-f64") (block (drop (f64.neg (br 0)))))
  (func (export "type-i32-i32") (block (drop (i32.add (br 0)))))
  (func (export "type-i64-i64") (block (drop (i64.add (br 0)))))
  (func (export "type-f32-f32") (block (drop (f32.add (br 0)))))
  (func (export "type-f64-f64") (block (drop (f64.add (br 0)))))

  (func (export "type-i32-value") (result i32)
    (block (result i32) (i32.ctz (br 0 (i32.const 1))))
  )
  (func (export "type-i64-value") (result i64)
    (block (result i64) (i64.ctz (br 0 (i64.const 2))))
  )
  (func (export "type-f32-value") (result f32)
    (block (result f32) (f32.neg (br 0 (f32.const 3))))
  )
  (func (export "type-f64-value") (result f64)
    (block (result f64) (f64.neg (br 0 (f64.const 4))))
  )
  (func (export "type-f64-f64-value") (result f64 f64)
    (block (result f64 f64)
      (f64.add (br 0 (f64.const 4) (f64.const 5))) (f64.const 6)
    )
  )

  (func (export "as-block-first")
    (block (br 0) (call $dummy))
  )
  (func (export "as-block-mid")
    (block (call $dummy) (br 0) (call $dummy))
  )
  (func (export "as-block-last")
    (block (nop) (call $dummy) (br 0))
  )
  (func (export "as-block-value") (result i32)
    (block (result i32) (nop) (call $dummy) (br 0 (i32.const 2)))
  )

  (func (export "as-loop-first") (result i32)
    (block (result i32) (loop (result i32) (br 1 (i32.const 3)) (i32.const 2)))
  )
  (func (export "as-loop-mid") (result i32)
    (block (result i32)
      (loop (result i32) (call $dummy) (br 1 (i32.const 4)) (i32.const 2))
    )
  )
  (func (export "as-loop-last") (result i32)
    (block (result i32)
      (loop (result i32) (nop) (call $dummy) (br 1 (i32.const 5)))
    )
  )

  (func (export "as-br-value") (result i32)
    (block (result i32) (br 0 (br 0 (i32.const 9))))
  )

  (func (export "as-br_if-cond")
    (block (br_if 0 (br 0)))
  )
  (func (export "as-br_if-value") (result i32)
    (block (result i32)
      (drop (br_if 0 (br 0 (i32.const 8)) (i32.const 1))) (i32.const 7)
    )
  )
  (func (export "as-br_if-value-cond") (result i32)
    (block (result i32)
      (drop (br_if 0 (i32.const 6) (br 0 (i32.const 9)))) (i32.const 7)
    )
  )

  (func (export "as-br_table-index")
    (block (br_table 0 0 0 (br 0)))
  )
  (func (export "as-br_table-value") (result i32)
    (block (result i32)
      (br_table 0 0 0 (br 0 (i32.const 10)) (i32.const 1)) (i32.const 7)
    )
  )
  (func (export "as-br_table-value-index") (result i32)
    (block (result i32)
      (br_table 0 0 (i32.const 6) (br 0 (i32.const 11))) (i32.const 7)
    )
  )

  (func (export "as-return-value") (result i64)
    (block (result i64) (return (br 0 (i64.const 7))))
  )
  (func (export "as-return-values") (result i32 i64)
    (i32.const 2)
    (block (result i64) (return (br 0 (i32.const 1) (i64.const 7))))
  )

  (func (export "as-if-cond") (result i32)
    (block (result i32)
      (if (result i32) (br 0 (i32.const 2))
        (then (i32.const 0))
        (else (i32.const 1))
      )
    )
  )
  (func (export "as-if-then") (param i32 i32) (result i32)
    (block (result i32)
      (if (result i32) (local.get 0)
        (then (br 1 (i32.const 3)))
        (else (local.get 1))
      )
    )
  )
  (func (export "as-if-else") (param i32 i32) (result i32)
    (block (result i32)
      (if (result i32) (local.get 0)
        (then (local.get 1))
        (else (br 1 (i32.const 4)))
      )
    )
  )

  (func (export "as-select-first") (param i32 i32) (result i32)
    (block (result i32)
      (select (br 0 (i32.const 5)) (local.get 0) (local.get 1))
    )
  )
  (func (export "as-select-second") (param i32 i32) (result i32)
    (block (result i32)
      (select (local.get 0) (br 0 (i32.const 6)) (local.get 1))
    )
  )
  (func (export "as-select-cond") (result i32)
    (block (result i32)
      (select (i32.const 0) (i32.const 1) (br 0 (i32.const 7)))
    )
  )
  (func (export "as-select-all") (result i32)
    (block (result i32) (select (br 0 (i32.const 8))))
  )

  (func $f (param i32 i32 i32) (result i32) (i32.const -1))
  (func (export "as-call-first") (result i32)
    (block (result i32)
      (call $f (br 0 (i32.const 12)) (i32.const 2) (i32.const 3))
    )
  )
  (func (export "as-call-mid") (result i32)
    (block (result i32)
      (call $f (i32.const 1) (br 0 (i32.const 13)) (i32.const 3))
    )
  )
  (func (export "as-call-last") (result i32)
    (block (result i32)
      (call $f (i32.const 1) (i32.const 2) (br 0 (i32.const 14)))
    )
  )
  (func (export "as-call-all") (result i32)
    (block (result i32) (call $f (br 0 (i32.const 15))))
  )

  (type $sig (func (param i32 i32 i32) (result i32)))
  (table funcref (elem $f))
  (func (export "as-call_indirect-func") (result i32)
    (block (result i32)
      (call_indirect (type $sig)
        (br 0 (i32.const 20))
        (i32.const 1) (i32.const 2) (i32.const 3)
      )
    )
  )
  (func (export "as-call_indirect-first") (result i32)
    (block (result i32)
      (call_indirect (type $sig)
        (i32.const 0)
        (br 0 (i32.const 21)) (i32.const 2) (i32.const 3)
      )
    )
  )
  (func (export "as-call_indirect-mid") (result i32)
    (block (result i32)
      (call_indirect (type $sig)
        (i32.const 0)
        (i32.const 1) (br 0 (i32.const 22)) (i32.const 3)
      )
    )
  )
  (func (export "as-call_indirect-last") (result i32)
    (block (result i32)
      (call_indirect (type $sig)
        (i32.const 0)
        (i32.const 1) (i32.const 2) (br 0 (i32.const 23))
      )
    )
  )
  (func (export "as-call_indirect-all") (result i32)
    (block (result i32) (call_indirect (type $sig) (br 0 (i32.const 24))))
  )

  (func (export "as-local.set-value") (result i32) (local f32)
    (block (result i32) (local.set 0 (br 0 (i32.const 17))) (i32.const -1))
  )
  (func (export "as-local.tee-value") (result i32) (local i32)
    (block (result i32) (local.tee 0 (br 0 (i32.const 1))))
  )
  (global $a (mut i32) (i32.const 10))
  (func (export "as-global.set-value") (result i32)
    (block (result i32) (global.set $a (br 0 (i32.const 1))))
  )

  (memory 1)
  (func (export "as-load-address") (result f32)
    (block (result f32) (f32.load (br 0 (f32.const 1.7))))
  )
  (func (export "as-loadN-address") (result i64)
    (block (result i64) (i64.load8_s (br 0 (i64.const 30))))
  )

  (func (export "as-store-address") (result i32)
    (block (result i32)
      (f64.store (br 0 (i32.const 30)) (f64.const 7)) (i32.const -1)
    )
  )
  (func (export "as-store-value") (result i32)
    (block (result i32)
      (i64.store (i32.const 2) (br 0 (i32.const 31))) (i32.const -1)
    )
  )
  (func (export "as-store-both") (result i32)
    (block (result i32)
      (i64.store (br 0 (i32.const 32))) (i32.const -1)
    )
  )

  (func (export "as-storeN-address") (result i32)
    (block (result i32)
      (i32.store8 (br 0 (i32.const 32)) (i32.const 7)) (i32.const -1)
    )
  )
  (func (export "as-storeN-value") (result i32)
    (block (result i32)
      (i64.store16 (i32.const 2) (br 0 (i32.const 33))) (i32.const -1)
    )
  )
  (func (export "as-storeN-both") (result i32)
    (block (result i32)
      (i64.store16 (br 0 (i32.const 34))) (i32.const -1)
    )
  )

  (func (export "as-unary-operand") (result f32)
    (block (result f32) (f32.neg (br 0 (f32.const 3.4))))
  )

  (func (export "as-binary-left") (result i32)
    (block (result i32) (i32.add (br 0 (i32.const 3)) (i32.const 10)))
  )
  (func (export "as-binary-right") (result i64)
    (block (result i64) (i64.sub (i64.const 10) (br 0 (i64.const 45))))
  )
  (func (export "as-binary-both") (result i32)
    (block (result i32) (i32.add (br 0 (i32.const 46))))
  )

  (func (export "as-test-operand") (result i32)
    (block (result i32) (i32.eqz (br 0 (i32.const 44))))
  )

  (func (export "as-compare-left") (result i32)
    (block (result i32) (f64.le (br 0 (i32.const 43)) (f64.const 10)))
  )
  (func (export "as-compare-right") (result i32)
    (block (result i32) (f32.ne (f32.const 10) (br 0 (i32.const 42))))
  )
  (func (export "as-compare-both") (result i32)
    (block (result i32) (f64.le (br 0 (i32.const 44))))
  )

  (func (export "as-convert-operand") (result i32)
    (block (result i32) (i32.wrap_i64 (br 0 (i32.const 41))))
  )

  (func (export "as-memory.grow-size") (result i32)
    (block (result i32) (memory.grow (br 0 (i32.const 40))))
  )

  (func (export "nested-block-value") (result i32)
    (i32.add
      (i32.const 1)
      (block (result i32)
        (call $dummy)
        (i32.add (i32.const 4) (br 0 (i32.const 8)))
      )
    )
  )

  (func (export "nested-br-value") (result i32)
    (i32.add
      (i32.const 1)
      (block (result i32)
        (drop (i32.const 2))
        (drop
          (block (result i32)
            (drop (i32.const 4))
            (br 0 (br 1 (i32.const 8)))
          )
        )
        (i32.const 16)
      )
    )
  )

  (func (export "nested-br_if-value") (result i32)
    (i32.add
      (i32.const 1)
      (block (result i32)
        (drop (i32.const 2))
        (drop
          (block (result i32)
            (drop (i32.const 4))
            (drop (br_if 0 (br 1 (i32.const 8)) (i32.const 1)))
            (i32.const 32)
          )
        )
        (i32.const 16)
      )
    )
  )

  (func (export "nested-br_if-value-cond") (result i32)
    (i32.add
      (i32.const 1)
      (block (result i32)
        (drop (i32.const 2))
        (drop (br_if 0 (i32.const 4) (br 0 (i32.const 8))))
        (i32.const 16)
      )
    )
  )

  (func (export "nested-br_table-value") (result i32)
    (i32.add
      (i32.const 1)
      (block (result i32)
        (drop (i32.const 2))
        (drop
          (block (result i32)
            (drop (i32.const 4))
            (br_table 0 (br 1 (i32.const 8)) (i32.const 1))
          )
        )
        (i32.const 16)
      )
    )
  )

  (func (export "nested-br_table-value-index") (result i32)
    (i32.add
      (i32.const 1)
      (block (result i32)
        (drop (i32.const 2))
        (br_table 0 (i32.const 4) (br 0 (i32.const 8)))
        (i32.const 16)
      )
    )
  )
)
"#
    );
}

#[test]
fn br_if() {
    assert_snapshot!(
        r#"
(module
  (func $dummy)

  (func (export "type-i32")
    (block (drop (i32.ctz (br_if 0 (i32.const 0) (i32.const 1)))))
  )
  (func (export "type-i64")
    (block (drop (i64.ctz (br_if 0 (i64.const 0) (i32.const 1)))))
  )
  (func (export "type-f32")
    (block (drop (f32.neg (br_if 0 (f32.const 0) (i32.const 1)))))
  )
  (func (export "type-f64")
    (block (drop (f64.neg (br_if 0 (f64.const 0) (i32.const 1)))))
  )

  (func (export "type-i32-value") (result i32)
    (block (result i32) (i32.ctz (br_if 0 (i32.const 1) (i32.const 1))))
  )
  (func (export "type-i64-value") (result i64)
    (block (result i64) (i64.ctz (br_if 0 (i64.const 2) (i32.const 1))))
  )
  (func (export "type-f32-value") (result f32)
    (block (result f32) (f32.neg (br_if 0 (f32.const 3) (i32.const 1))))
  )
  (func (export "type-f64-value") (result f64)
    (block (result f64) (f64.neg (br_if 0 (f64.const 4) (i32.const 1))))
  )

  (func (export "as-block-first") (param i32) (result i32)
    (block (br_if 0 (local.get 0)) (return (i32.const 2))) (i32.const 3)
  )
  (func (export "as-block-mid") (param i32) (result i32)
    (block (call $dummy) (br_if 0 (local.get 0)) (return (i32.const 2)))
    (i32.const 3)
  )
  (func (export "as-block-last") (param i32)
    (block (call $dummy) (call $dummy) (br_if 0 (local.get 0)))
  )
  (func (export "as-block-first-value") (param i32) (result i32)
    (block (result i32)
      (drop (br_if 0 (i32.const 10) (local.get 0))) (return (i32.const 11))
    )
  )
  (func (export "as-block-mid-value") (param i32) (result i32)
    (block (result i32)
      (call $dummy)
      (drop (br_if 0 (i32.const 20) (local.get 0)))
      (return (i32.const 21))
    )
  )
  (func (export "as-block-last-value") (param i32) (result i32)
    (block (result i32)
      (call $dummy) (call $dummy) (br_if 0 (i32.const 11) (local.get 0))
    )
  )

  (func (export "as-loop-first") (param i32) (result i32)
    (block (loop (br_if 1 (local.get 0)) (return (i32.const 2)))) (i32.const 3)
  )
  (func (export "as-loop-mid") (param i32) (result i32)
    (block (loop (call $dummy) (br_if 1 (local.get 0)) (return (i32.const 2))))
    (i32.const 4)
  )
  (func (export "as-loop-last") (param i32)
    (loop (call $dummy) (br_if 1 (local.get 0)))
  )

  (func (export "as-br-value") (result i32)
    (block (result i32) (br 0 (br_if 0 (i32.const 1) (i32.const 2))))
  )

  (func (export "as-br_if-cond")
    (block (br_if 0 (br_if 0 (i32.const 1) (i32.const 1))))
  )
  (func (export "as-br_if-value") (result i32)
    (block (result i32)
      (drop (br_if 0 (br_if 0 (i32.const 1) (i32.const 2)) (i32.const 3)))
      (i32.const 4)
    )
  )
  (func (export "as-br_if-value-cond") (param i32) (result i32)
    (block (result i32)
      (drop (br_if 0 (i32.const 2) (br_if 0 (i32.const 1) (local.get 0))))
      (i32.const 4)
    )
  )

  (func (export "as-br_table-index")
    (block (br_table 0 0 0 (br_if 0 (i32.const 1) (i32.const 2))))
  )
  (func (export "as-br_table-value") (result i32)
    (block (result i32)
      (br_table 0 0 0 (br_if 0 (i32.const 1) (i32.const 2)) (i32.const 3)) (i32.const 4)
    )
  )
  (func (export "as-br_table-value-index") (result i32)
    (block (result i32)
      (br_table 0 0 (i32.const 2) (br_if 0 (i32.const 1) (i32.const 3))) (i32.const 4)
    )
  )
  (func (export "as-return-value") (result i64)
    (block (result i64) (return (br_if 0 (i64.const 1) (i32.const 2))))
  )

  (func (export "as-if-cond") (param i32) (result i32)
    (block (result i32)
      (if (result i32)
        (br_if 0 (i32.const 1) (local.get 0))
        (then (i32.const 2))
        (else (i32.const 3))
      )
    )
  )
  (func (export "as-if-then") (param i32 i32)
    (block
      (if (local.get 0) (then (br_if 1 (local.get 1))) (else (call $dummy)))
    )
  )
  (func (export "as-if-else") (param i32 i32)
    (block
      (if (local.get 0) (then (call $dummy)) (else (br_if 1 (local.get 1))))
    )
  )

  (func (export "as-select-first") (param i32) (result i32)
    (block (result i32)
      (select (br_if 0 (i32.const 3) (i32.const 10)) (i32.const 2) (local.get 0))
    )
  )
  (func (export "as-select-second") (param i32) (result i32)
    (block (result i32)
      (select (i32.const 1) (br_if 0 (i32.const 3) (i32.const 10)) (local.get 0))
    )
  )
  (func (export "as-select-cond") (result i32)
    (block (result i32)
      (select (i32.const 1) (i32.const 2) (br_if 0 (i32.const 3) (i32.const 10)))
    )
  )

  (func $f (param i32 i32 i32) (result i32) (i32.const -1))
  (func (export "as-call-first") (result i32)
    (block (result i32)
      (call $f
        (br_if 0 (i32.const 12) (i32.const 1)) (i32.const 2) (i32.const 3)
      )
    )
  )
  (func (export "as-call-mid") (result i32)
    (block (result i32)
      (call $f
        (i32.const 1) (br_if 0 (i32.const 13) (i32.const 1)) (i32.const 3)
      )
    )
  )
  (func (export "as-call-last") (result i32)
    (block (result i32)
      (call $f
        (i32.const 1) (i32.const 2) (br_if 0 (i32.const 14) (i32.const 1))
      )
    )
  )

  (func $func (param i32 i32 i32) (result i32) (local.get 0))
  (type $check (func (param i32 i32 i32) (result i32)))
  (table funcref (elem $func))
  (func (export "as-call_indirect-func") (result i32)
    (block (result i32)
      (call_indirect (type $check)
        (br_if 0 (i32.const 4) (i32.const 10))
        (i32.const 1) (i32.const 2) (i32.const 0)
      )
    )
  )
  (func (export "as-call_indirect-first") (result i32)
    (block (result i32)
      (call_indirect (type $check)
        (i32.const 1) (br_if 0 (i32.const 4) (i32.const 10)) (i32.const 2) (i32.const 0)
      )
    )
  )
  (func (export "as-call_indirect-mid") (result i32)
    (block (result i32)
      (call_indirect (type $check)
        (i32.const 1) (i32.const 2) (br_if 0 (i32.const 4) (i32.const 10)) (i32.const 0)
      )
    )
  )
  (func (export "as-call_indirect-last") (result i32)
    (block (result i32)
      (call_indirect (type $check)
        (i32.const 1) (i32.const 2) (i32.const 3) (br_if 0 (i32.const 4) (i32.const 10))
      )
    )
  )

  (func (export "as-local.set-value") (param i32) (result i32)
    (local i32)
    (block (result i32)
      (local.set 0 (br_if 0 (i32.const 17) (local.get 0)))
      (i32.const -1)
    )
  )
  (func (export "as-local.tee-value") (param i32) (result i32)
    (block (result i32)
      (local.tee 0 (br_if 0 (i32.const 1) (local.get 0)))
      (return (i32.const -1))
    )
  )
  (global $a (mut i32) (i32.const 10))
  (func (export "as-global.set-value") (param i32) (result i32)
    (block (result i32)
      (global.set $a (br_if 0 (i32.const 1) (local.get 0)))
      (return (i32.const -1))
    )
  )

  (memory 1)
  (func (export "as-load-address") (result i32)
    (block (result i32) (i32.load (br_if 0 (i32.const 1) (i32.const 1))))
  )
  (func (export "as-loadN-address") (result i32)
    (block (result i32) (i32.load8_s (br_if 0 (i32.const 30) (i32.const 1))))
  )

  (func (export "as-store-address") (result i32)
    (block (result i32)
      (i32.store (br_if 0 (i32.const 30) (i32.const 1)) (i32.const 7)) (i32.const -1)
    )
  )
  (func (export "as-store-value") (result i32)
    (block (result i32)
      (i32.store (i32.const 2) (br_if 0 (i32.const 31) (i32.const 1))) (i32.const -1)
    )
  )

  (func (export "as-storeN-address") (result i32)
    (block (result i32)
      (i32.store8 (br_if 0 (i32.const 32) (i32.const 1)) (i32.const 7)) (i32.const -1)
    )
  )
  (func (export "as-storeN-value") (result i32)
    (block (result i32)
      (i32.store16 (i32.const 2) (br_if 0 (i32.const 33) (i32.const 1))) (i32.const -1)
    )
  )

  (func (export "as-unary-operand") (result f64)
    (block (result f64) (f64.neg (br_if 0 (f64.const 1.0) (i32.const 1))))
  )
  (func (export "as-binary-left") (result i32)
    (block (result i32) (i32.add (br_if 0 (i32.const 1) (i32.const 1)) (i32.const 10)))
  )
  (func (export "as-binary-right") (result i32)
    (block (result i32) (i32.sub (i32.const 10) (br_if 0 (i32.const 1) (i32.const 1))))
  )
  (func (export "as-test-operand") (result i32)
    (block (result i32) (i32.eqz (br_if 0 (i32.const 0) (i32.const 1))))
  )
  (func (export "as-compare-left") (result i32)
    (block (result i32) (i32.le_u (br_if 0 (i32.const 1) (i32.const 1)) (i32.const 10)))
  )
  (func (export "as-compare-right") (result i32)
    (block (result i32) (i32.ne (i32.const 10) (br_if 0 (i32.const 1) (i32.const 42))))
  )

  (func (export "as-memory.grow-size") (result i32)
    (block (result i32) (memory.grow (br_if 0 (i32.const 1) (i32.const 1))))
  )

  (func (export "nested-block-value") (param i32) (result i32)
    (i32.add
      (i32.const 1)
      (block (result i32)
        (drop (i32.const 2))
        (i32.add
          (i32.const 4)
          (block (result i32)
            (drop (br_if 1 (i32.const 8) (local.get 0)))
            (i32.const 16)
          )
        )
      )
    )
  )

  (func (export "nested-br-value") (param i32) (result i32)
    (i32.add
      (i32.const 1)
      (block (result i32)
        (drop (i32.const 2))
        (br 0
          (block (result i32)
            (drop (br_if 1 (i32.const 8) (local.get 0))) (i32.const 4)
          )
        )
        (i32.const 16)
      )
    )
  )

  (func (export "nested-br_if-value") (param i32) (result i32)
    (i32.add
      (i32.const 1)
      (block (result i32)
        (drop (i32.const 2))
        (drop (br_if 0
          (block (result i32)
            (drop (br_if 1 (i32.const 8) (local.get 0))) (i32.const 4)
          )
          (i32.const 1)
        ))
        (i32.const 16)
      )
    )
  )

  (func (export "nested-br_if-value-cond") (param i32) (result i32)
    (i32.add
      (i32.const 1)
      (block (result i32)
        (drop (i32.const 2))
        (drop (br_if 0
          (i32.const 4)
          (block (result i32)
            (drop (br_if 1 (i32.const 8) (local.get 0))) (i32.const 1)
          )
        ))
        (i32.const 16)
      )
    )
  )

  (func (export "nested-br_table-value") (param i32) (result i32)
    (i32.add
      (i32.const 1)
      (block (result i32)
        (drop (i32.const 2))
        (br_table 0
          (block (result i32)
            (drop (br_if 1 (i32.const 8) (local.get 0))) (i32.const 4)
          )
          (i32.const 1)
        )
        (i32.const 16)
      )
    )
  )

  (func (export "nested-br_table-value-index") (param i32) (result i32)
    (i32.add
      (i32.const 1)
      (block (result i32)
        (drop (i32.const 2))
        (br_table 0
          (i32.const 4)
          (block (result i32)
            (drop (br_if 1 (i32.const 8) (local.get 0))) (i32.const 1)
          )
        )
        (i32.const 16)
      )
    )
  )

)
"#
    );
}

#[test]
fn br_table() {
    assert_snapshot!(
        r#"
(module
  (func $dummy)
  (func (export "type-i32")
    (block (drop (i32.ctz (br_table 0 0 (i32.const 0)))))
  )
  (func (export "type-i64")
    (block (drop (i64.ctz (br_table 0 0 (i32.const 0)))))
  )
  (func (export "type-f32")
    (block (drop (f32.neg (br_table 0 0 (i32.const 0)))))
  )
  (func (export "type-f64")
    (block (drop (f64.neg (br_table 0 0 (i32.const 0)))))
  )

  (func (export "type-i32-value") (result i32)
    (block (result i32) (i32.ctz (br_table 0 0 (i32.const 1) (i32.const 0))))
  )
  (func (export "type-i64-value") (result i64)
    (block (result i64) (i64.ctz (br_table 0 0 (i64.const 2) (i32.const 0))))
  )
  (func (export "type-f32-value") (result f32)
    (block (result f32) (f32.neg (br_table 0 0 (f32.const 3) (i32.const 0))))
  )
  (func (export "type-f64-value") (result f64)
    (block (result f64) (f64.neg (br_table 0 0 (f64.const 4) (i32.const 0))))
  )

  (func (export "empty") (param i32) (result i32)
    (block (br_table 0 (local.get 0)) (return (i32.const 21)))
    (i32.const 22)
  )
  (func (export "empty-value") (param i32) (result i32)
    (block (result i32)
      (br_table 0 (i32.const 33) (local.get 0)) (i32.const 31)
    )
  )

  (func (export "singleton") (param i32) (result i32)
    (block
      (block
        (br_table 1 0 (local.get 0))
        (return (i32.const 21))
      )
      (return (i32.const 20))
    )
    (i32.const 22)
  )

  (func (export "singleton-value") (param i32) (result i32)
    (block (result i32)
      (drop
        (block (result i32)
          (br_table 0 1 (i32.const 33) (local.get 0))
          (return (i32.const 31))
        )
      )
      (i32.const 32)
    )
  )

  (func (export "multiple") (param i32) (result i32)
    (block
      (block
        (block
          (block
            (block
              (br_table 3 2 1 0 4 (local.get 0))
              (return (i32.const 99))
            )
            (return (i32.const 100))
          )
          (return (i32.const 101))
        )
        (return (i32.const 102))
      )
      (return (i32.const 103))
    )
    (i32.const 104)
  )

  (func (export "multiple-value") (param i32) (result i32)
    (local i32)
    (local.set 1 (block (result i32)
      (local.set 1 (block (result i32)
        (local.set 1 (block (result i32)
          (local.set 1 (block (result i32)
            (local.set 1 (block (result i32)
              (br_table 3 2 1 0 4 (i32.const 200) (local.get 0))
              (return (i32.add (local.get 1) (i32.const 99)))
            ))
            (return (i32.add (local.get 1) (i32.const 10)))
          ))
          (return (i32.add (local.get 1) (i32.const 11)))
        ))
        (return (i32.add (local.get 1) (i32.const 12)))
      ))
      (return (i32.add (local.get 1) (i32.const 13)))
    ))
    (i32.add (local.get 1) (i32.const 14))
  )

  (func (export "large") (param i32) (result i32)
    (block
      (block
        (br_table
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1
          (local.get 0)
        )
        (return (i32.const -1))
      )
      (return (i32.const 0))
    )
    (return (i32.const 1))
  )

  (func (export "as-block-first")
    (block (br_table 0 0 0 (i32.const 0)) (call $dummy))
  )
  (func (export "as-block-mid")
    (block (call $dummy) (br_table 0 0 0 (i32.const 0)) (call $dummy))
  )
  (func (export "as-block-last")
    (block (nop) (call $dummy) (br_table 0 0 0 (i32.const 0)))
  )
  (func (export "as-block-value") (result i32)
    (block (result i32)
      (nop) (call $dummy) (br_table 0 0 0 (i32.const 2) (i32.const 0))
    )
  )

  (func (export "as-loop-first") (result i32)
    (loop (result i32) (br_table 1 1 (i32.const 3) (i32.const 0)) (i32.const 1))
  )
  (func (export "as-loop-mid") (result i32)
    (loop (result i32)
      (call $dummy)
      (br_table 1 1 1 (i32.const 4) (i32.const -1))
      (i32.const 2)
    )
  )
  (func (export "as-loop-last") (result i32)
    (loop (result i32)
      (nop) (call $dummy) (br_table 1 1 1 (i32.const 5) (i32.const 1))
    )
  )

  (func (export "as-br-value") (result i32)
    (block (result i32) (br 0 (br_table 0 (i32.const 9) (i32.const 0))))
  )

  (func (export "as-br_if-cond")
    (block (br_if 0 (br_table 0 0 0 (i32.const 1))))
  )
  (func (export "as-br_if-value") (result i32)
    (block (result i32)
      (drop (br_if 0 (br_table 0 (i32.const 8) (i32.const 0)) (i32.const 1)))
      (i32.const 7)
    )
  )
  (func (export "as-br_if-value-cond") (result i32)
    (block (result i32)
      (drop (br_if 0 (i32.const 6) (br_table 0 0 (i32.const 9) (i32.const 0))))
      (i32.const 7)
    )
  )

  (func (export "as-br_table-index")
    (block (br_table 0 0 0 (br_table 0 (i32.const 1))))
  )
  (func (export "as-br_table-value") (result i32)
    (block (result i32)
      (br_table 0 0 0 (br_table 0 (i32.const 10) (i32.const 0)) (i32.const 1))
      (i32.const 7)
    )
  )
  (func (export "as-br_table-value-index") (result i32)
    (block (result i32)
      (br_table 0 0 (i32.const 6) (br_table 0 (i32.const 11) (i32.const 1)))
      (i32.const 7)
    )
  )

  (func (export "as-return-value") (result i64)
    (block (result i64) (return (br_table 0 (i64.const 7) (i32.const 0))))
  )

  (func (export "as-if-cond") (result i32)
    (block (result i32)
      (if (result i32)
        (br_table 0 (i32.const 2) (i32.const 0))
        (then (i32.const 0))
        (else (i32.const 1))
      )
    )
  )
  (func (export "as-if-then") (param i32 i32) (result i32)
    (block (result i32)
      (if (result i32)
        (local.get 0)
        (then (br_table 1 (i32.const 3) (i32.const 0)))
        (else (local.get 1))
      )
    )
  )
  (func (export "as-if-else") (param i32 i32) (result i32)
    (block (result i32)
      (if (result i32)
        (local.get 0)
        (then (local.get 1))
        (else (br_table 1 0 (i32.const 4) (i32.const 0)))
      )
    )
  )

  (func (export "as-select-first") (param i32 i32) (result i32)
    (block (result i32)
      (select
        (br_table 0 (i32.const 5) (i32.const 0)) (local.get 0) (local.get 1)
      )
    )
  )
  (func (export "as-select-second") (param i32 i32) (result i32)
    (block (result i32)
      (select
        (local.get 0) (br_table 0 (i32.const 6) (i32.const 1)) (local.get 1)
      )
    )
  )
  (func (export "as-select-cond") (result i32)
    (block (result i32)
      (select
        (i32.const 0) (i32.const 1) (br_table 0 (i32.const 7) (i32.const 1))
      )
    )
  )

  (func $f (param i32 i32 i32) (result i32) (i32.const -1))
  (func (export "as-call-first") (result i32)
    (block (result i32)
      (call $f
        (br_table 0 (i32.const 12) (i32.const 1)) (i32.const 2) (i32.const 3)
      )
    )
  )
  (func (export "as-call-mid") (result i32)
    (block (result i32)
      (call $f
        (i32.const 1) (br_table 0 (i32.const 13) (i32.const 1)) (i32.const 3)
      )
    )
  )
  (func (export "as-call-last") (result i32)
    (block (result i32)
      (call $f
        (i32.const 1) (i32.const 2) (br_table 0 (i32.const 14) (i32.const 1))
      )
    )
  )

  (type $sig (func (param i32 i32 i32) (result i32)))
  (table funcref (elem $f))
  (func (export "as-call_indirect-first") (result i32)
    (block (result i32)
      (call_indirect (type $sig)
        (br_table 0 (i32.const 20) (i32.const 1)) (i32.const 1) (i32.const 2)
        (i32.const 3)
      )
    )
  )
  (func (export "as-call_indirect-mid") (result i32)
    (block (result i32)
      (call_indirect (type $sig)
        (i32.const 0) (br_table 0 (i32.const 21) (i32.const 1)) (i32.const 2)
        (i32.const 3)
      )
    )
  )
  (func (export "as-call_indirect-last") (result i32)
    (block (result i32)
      (call_indirect (type $sig)
        (i32.const 0) (i32.const 1) (br_table 0 (i32.const 22) (i32.const 1))
        (i32.const 3)
      )
    )
  )
  (func (export "as-call_indirect-func") (result i32)
    (block (result i32)
      (call_indirect (type $sig)
        (i32.const 0) (i32.const 1) (i32.const 2)
        (br_table 0 (i32.const 23) (i32.const 1))
      )
    )
  )

  (func (export "as-local.set-value") (result i32)
    (local f32)
    (block (result i32)
      (local.set 0 (br_table 0 (i32.const 17) (i32.const 1)))
      (i32.const -1)
    )
  )
  (func (export "as-local.tee-value") (result i32)
    (local i32)
    (block (result i32)
      (local.set 0 (br_table 0 (i32.const 1) (i32.const 1)))
      (i32.const -1)
    )
  )
  (global $a (mut i32) (i32.const 10))
  (func (export "as-global.set-value") (result i32)
    (block (result i32)
      (global.set $a (br_table 0 (i32.const 1) (i32.const 1)))
      (i32.const -1)
    )
  )

  (memory 1)
  (func (export "as-load-address") (result f32)
    (block (result f32) (f32.load (br_table 0 (f32.const 1.7) (i32.const 1))))
  )
  (func (export "as-loadN-address") (result i64)
    (block (result i64) (i64.load8_s (br_table 0 (i64.const 30) (i32.const 1))))
  )

  (func (export "as-store-address") (result i32)
    (block (result i32)
      (f64.store (br_table 0 (i32.const 30) (i32.const 1)) (f64.const 7))
      (i32.const -1)
    )
  )
  (func (export "as-store-value") (result i32)
    (block (result i32)
      (i64.store (i32.const 2) (br_table 0 (i32.const 31) (i32.const 1)))
      (i32.const -1)
     )
  )

  (func (export "as-storeN-address") (result i32)
    (block (result i32)
      (i32.store8 (br_table 0 (i32.const 32) (i32.const 0)) (i32.const 7))
      (i32.const -1)
    )
  )
  (func (export "as-storeN-value") (result i32)
    (block (result i32)
      (i64.store16 (i32.const 2) (br_table 0 (i32.const 33) (i32.const 0)))
      (i32.const -1)
    )
  )

  (func (export "as-unary-operand") (result f32)
    (block (result f32) (f32.neg (br_table 0 (f32.const 3.4) (i32.const 0))))
  )

  (func (export "as-binary-left") (result i32)
    (block (result i32)
      (i32.add (br_table 0 0 (i32.const 3) (i32.const 0)) (i32.const 10))
    )
  )
  (func (export "as-binary-right") (result i64)
    (block (result i64)
      (i64.sub (i64.const 10) (br_table 0 (i64.const 45) (i32.const 0)))
    )
  )

  (func (export "as-test-operand") (result i32)
    (block (result i32) (i32.eqz (br_table 0 (i32.const 44) (i32.const 0))))
  )

  (func (export "as-compare-left") (result i32)
    (block (result i32)
      (f64.le (br_table 0 0 (i32.const 43) (i32.const 0)) (f64.const 10))
    )
  )
  (func (export "as-compare-right") (result i32)
    (block (result i32)
      (f32.ne (f32.const 10) (br_table 0 (i32.const 42) (i32.const 0)))
    )
  )

  (func (export "as-convert-operand") (result i32)
    (block (result i32)
      (i32.wrap_i64 (br_table 0 (i32.const 41) (i32.const 0)))
    )
  )

  (func (export "as-memory.grow-size") (result i32)
    (block (result i32) (memory.grow (br_table 0 (i32.const 40) (i32.const 0))))
  )

  (func (export "nested-block-value") (param i32) (result i32)
    (block (result i32)
      (drop (i32.const -1))
      (i32.add
        (i32.const 1)
        (block (result i32)
          (i32.add
            (i32.const 2)
            (block (result i32)
              (drop (i32.const 4))
              (i32.add
                (i32.const 8)
                (br_table 0 1 2 (i32.const 16) (local.get 0))
              )
            )
          )
        )
      )
    )
  )

  (func (export "nested-br-value") (param i32) (result i32)
    (block (result i32)
      (i32.add
        (i32.const 1)
        (block (result i32)
          (drop (i32.const 2))
          (drop
            (block (result i32)
              (drop (i32.const 4))
              (br 0 (br_table 2 1 0 (i32.const 8) (local.get 0)))
            )
          )
          (i32.const 16)
        )
      )
    )
  )

  (func (export "nested-br_if-value") (param i32) (result i32)
    (block (result i32)
      (i32.add
        (i32.const 1)
        (block (result i32)
          (drop (i32.const 2))
          (drop
            (block (result i32)
              (drop (i32.const 4))
              (drop
                (br_if 0
                  (br_table 0 1 2 (i32.const 8) (local.get 0))
                  (i32.const 1)
                )
              )
              (i32.const 32)
            )
          )
          (i32.const 16)
        )
      )
    )
  )

  (func (export "nested-br_if-value-cond") (param i32) (result i32)
    (block (result i32)
      (i32.add
        (i32.const 1)
        (block (result i32)
          (drop (i32.const 2))
          (drop
            (br_if 0 (i32.const 4) (br_table 0 1 0 (i32.const 8) (local.get 0)))
          )
          (i32.const 16)
        )
      )
    )
  )

  (func (export "nested-br_table-value") (param i32) (result i32)
    (block (result i32)
      (i32.add
        (i32.const 1)
        (block (result i32)
          (drop (i32.const 2))
          (drop
            (block (result i32)
              (drop (i32.const 4))
              (br_table 0 (br_table 0 1 2 (i32.const 8) (local.get 0)) (i32.const 1))
              (i32.const 32)
            )
          )
          (i32.const 16)
        )
      )
    )
  )

  (func (export "nested-br_table-value-index") (param i32) (result i32)
    (block (result i32)
      (i32.add
        (i32.const 1)
        (block (result i32)
          (drop (i32.const 2))
          (br_table 0 (i32.const 4) (br_table 0 1 0 (i32.const 8) (local.get 0)))
          (i32.const 16)
        )
      )
    )
  )

  (func (export "nested-br_table-loop-block") (param i32) (result i32)
    (local.set 0
      (loop (result i32)
        (block
          (br_table 1 0 0 (local.get 0))
        )
        (i32.const 0)
      )
    )
    (loop (result i32)
      (block
        (br_table 0 1 1 (local.get 0))
      )
      (i32.const 3)
    )
  )

  (func (export "meet-externref") (param i32) (param externref) (result externref)
    (block $l1 (result externref)
      (block $l2 (result externref)
        (br_table $l1 $l2 $l1 (local.get 1) (local.get 0))
      )
    )
  )
)
"#
    );
}

#[test]
fn call() {
    assert_snapshot!(
        r#"
(module
  ;; Auxiliary definitions
  (func $const-i32 (result i32) (i32.const 0x132))
  (func $const-i64 (result i64) (i64.const 0x164))
  (func $const-f32 (result f32) (f32.const 0xf32))
  (func $const-f64 (result f64) (f64.const 0xf64))
  (func $const-i32-i64 (result i32 i64) (i32.const 0x132) (i64.const 0x164))

  (func $id-i32 (param i32) (result i32) (local.get 0))
  (func $id-i64 (param i64) (result i64) (local.get 0))
  (func $id-f32 (param f32) (result f32) (local.get 0))
  (func $id-f64 (param f64) (result f64) (local.get 0))
  (func $id-i32-f64 (param i32 f64) (result i32 f64)
    (local.get 0) (local.get 1)
  )

  (func $swap-i32-i32 (param i32 i32) (result i32 i32)
    (local.get 1) (local.get 0)
  )
  (func $swap-f32-f64 (param f32 f64) (result f64 f32)
    (local.get 1) (local.get 0)
  )
  (func $swap-f64-i32 (param f64 i32) (result i32 f64)
    (local.get 1) (local.get 0)
  )

  (func $f32-i32 (param f32 i32) (result i32) (local.get 1))
  (func $i32-i64 (param i32 i64) (result i64) (local.get 1))
  (func $f64-f32 (param f64 f32) (result f32) (local.get 1))
  (func $i64-f64 (param i64 f64) (result f64) (local.get 1))

  ;; Typing

  (func (export "type-i32") (result i32) (call $const-i32))
  (func (export "type-i64") (result i64) (call $const-i64))
  (func (export "type-f32") (result f32) (call $const-f32))
  (func (export "type-f64") (result f64) (call $const-f64))
  (func (export "type-i32-i64") (result i32 i64) (call $const-i32-i64))

  (func (export "type-first-i32") (result i32) (call $id-i32 (i32.const 32)))
  (func (export "type-first-i64") (result i64) (call $id-i64 (i64.const 64)))
  (func (export "type-first-f32") (result f32) (call $id-f32 (f32.const 1.32)))
  (func (export "type-first-f64") (result f64) (call $id-f64 (f64.const 1.64)))

  (func (export "type-second-i32") (result i32)
    (call $f32-i32 (f32.const 32.1) (i32.const 32))
  )
  (func (export "type-second-i64") (result i64)
    (call $i32-i64 (i32.const 32) (i64.const 64))
  )
  (func (export "type-second-f32") (result f32)
    (call $f64-f32 (f64.const 64) (f32.const 32))
  )
  (func (export "type-second-f64") (result f64)
    (call $i64-f64 (i64.const 64) (f64.const 64.1))
  )

  (func (export "type-all-i32-f64") (result i32 f64)
    (call $id-i32-f64 (i32.const 32) (f64.const 1.64))
  )
  (func (export "type-all-i32-i32") (result i32 i32)
    (call $swap-i32-i32 (i32.const 1) (i32.const 2))
  )
  (func (export "type-all-f32-f64") (result f64 f32)
    (call $swap-f32-f64 (f32.const 1) (f64.const 2))
  )
  (func (export "type-all-f64-i32") (result i32 f64)
    (call $swap-f64-i32 (f64.const 1) (i32.const 2))
  )

  ;; Composition

  (func (export "as-binary-all-operands") (result i32)
    (i32.add (call $swap-i32-i32 (i32.const 3) (i32.const 4)))
  )

  (func (export "as-mixed-operands") (result i32)
    (call $swap-i32-i32 (i32.const 3) (i32.const 4))
    (i32.const 5)
    (i32.add)
    (i32.mul)
  )

  (func (export "as-call-all-operands") (result i32 i32)
    (call $swap-i32-i32 (call $swap-i32-i32 (i32.const 3) (i32.const 4)))
  )

  ;; Recursion

  (func $fac (export "fac") (param i64) (result i64)
    (if (result i64) (i64.eqz (local.get 0))
      (then (i64.const 1))
      (else
        (i64.mul
          (local.get 0)
          (call $fac (i64.sub (local.get 0) (i64.const 1)))
        )
      )
    )
  )

  (func $fac-acc (export "fac-acc") (param i64 i64) (result i64)
    (if (result i64) (i64.eqz (local.get 0))
      (then (local.get 1))
      (else
        (call $fac-acc
          (i64.sub (local.get 0) (i64.const 1))
          (i64.mul (local.get 0) (local.get 1))
        )
      )
    )
  )

  (func $fib (export "fib") (param i64) (result i64)
    (if (result i64) (i64.le_u (local.get 0) (i64.const 1))
      (then (i64.const 1))
      (else
        (i64.add
          (call $fib (i64.sub (local.get 0) (i64.const 2)))
          (call $fib (i64.sub (local.get 0) (i64.const 1)))
        )
      )
    )
  )

  (func $even (export "even") (param i64) (result i32)
    (if (result i32) (i64.eqz (local.get 0))
      (then (i32.const 44))
      (else (call $odd (i64.sub (local.get 0) (i64.const 1))))
    )
  )
  (func $odd (export "odd") (param i64) (result i32)
    (if (result i32) (i64.eqz (local.get 0))
      (then (i32.const 99))
      (else (call $even (i64.sub (local.get 0) (i64.const 1))))
    )
  )

  ;; Stack exhaustion

  ;; Implementations are required to have every call consume some abstract
  ;; resource towards exhausting some abstract finite limit, such that
  ;; infinitely recursive test cases reliably trap in finite time. This is
  ;; because otherwise applications could come to depend on it on those
  ;; implementations and be incompatible with implementations that don't do
  ;; it (or don't do it under the same circumstances).

  (func $runaway (export "runaway") (call $runaway))

  (func $mutual-runaway1 (export "mutual-runaway") (call $mutual-runaway2))
  (func $mutual-runaway2 (call $mutual-runaway1))

  ;; As parameter of control constructs and instructions

  (memory 1)

  (func (export "as-select-first") (result i32)
    (select (call $const-i32) (i32.const 2) (i32.const 3))
  )
  (func (export "as-select-mid") (result i32)
    (select (i32.const 2) (call $const-i32) (i32.const 3))
  )
  (func (export "as-select-last") (result i32)
    (select (i32.const 2) (i32.const 3) (call $const-i32))
  )

  (func (export "as-if-condition") (result i32)
    (if (result i32) (call $const-i32) (then (i32.const 1)) (else (i32.const 2)))
  )

  (func (export "as-br_if-first") (result i32)
    (block (result i32) (br_if 0 (call $const-i32) (i32.const 2)))
  )
  (func (export "as-br_if-last") (result i32)
    (block (result i32) (br_if 0 (i32.const 2) (call $const-i32)))
  )

  (func (export "as-br_table-first") (result i32)
    (block (result i32) (call $const-i32) (i32.const 2) (br_table 0 0))
  )
  (func (export "as-br_table-last") (result i32)
    (block (result i32) (i32.const 2) (call $const-i32) (br_table 0 0))
  )

  (func $func (param i32 i32) (result i32) (local.get 0))
  (type $check (func (param i32 i32) (result i32)))
  (table funcref (elem $func))

  (func (export "as-store-first")
    (call $const-i32) (i32.const 1) (i32.store)
  )
  (func (export "as-store-last")
    (i32.const 10) (call $const-i32) (i32.store)
  )

  (func (export "as-memory.grow-value") (result i32)
    (memory.grow (call $const-i32))
  )
  (func (export "as-return-value") (result i32)
    (call $const-i32) (return)
  )
  (func (export "as-drop-operand")
    (call $const-i32) (drop)
  )
  (func (export "as-br-value") (result i32)
    (block (result i32) (br 0 (call $const-i32)))
  )
  (func (export "as-local.set-value") (result i32)
    (local i32) (local.set 0 (call $const-i32)) (local.get 0)
  )
  (func (export "as-local.tee-value") (result i32)
    (local i32) (local.tee 0 (call $const-i32))
  )
  (global $a (mut i32) (i32.const 10))
  (func (export "as-global.set-value") (result i32)
    (global.set $a (call $const-i32))
    (global.get $a)
  )
  (func (export "as-load-operand") (result i32)
    (i32.load (call $const-i32))
  )

  (func $dummy (param i32) (result i32) (local.get 0))
  (func $du (param f32) (result f32) (local.get 0))
  (func (export "as-unary-operand") (result f32)
    (block (result f32) (f32.sqrt (call $du (f32.const 0x0p+0))))
  )

  (func (export "as-binary-left") (result i32)
    (block (result i32) (i32.add (call $dummy (i32.const 1)) (i32.const 10)))
  )
  (func (export "as-binary-right") (result i32)
    (block (result i32) (i32.sub (i32.const 10) (call $dummy (i32.const 1))))
  )

  (func (export "as-test-operand") (result i32)
    (block (result i32) (i32.eqz (call $dummy (i32.const 1))))
  )

  (func (export "as-compare-left") (result i32)
    (block (result i32) (i32.le_u (call $dummy (i32.const 1)) (i32.const 10)))
  )
  (func (export "as-compare-right") (result i32)
    (block (result i32) (i32.ne (i32.const 10) (call $dummy (i32.const 1))))
  )

  (func (export "as-convert-operand") (result i64)
    (block (result i64) (i64.extend_i32_s (call $dummy (i32.const 1))))
  )

  ;; Test correct argument passing

  (func $return-from-long-argument-list-helper (param f32 i32 i32 f64 f32 f32 f32 f64 f32 i32 i32 f32 f64 i64 i64 i32 i64 i64 f32 i64 i64 i64 i32 f32 f32 f32 f64 f32 i32 i64 f32 f64 f64 f32 i32 f32 f32 f64 i64 f64 i32 i64 f32 f64 i32 i32 i32 i64 f64 i32 i64 i64 f64 f64 f64 f64 f64 f64 i32 f32 f64 f64 i32 i64 f32 f32 f32 i32 f64 f64 f64 f64 f64 f32 i64 i64 i32 i32 i32 f32 f64 i32 i64 f32 f32 f32 i32 i32 f32 f64 i64 f32 f64 f32 f32 f32 i32 f32 i64 i32) (result i32)
    (local.get 99)
  )

  (func (export "return-from-long-argument-list") (param i32) (result i32)
    (call $return-from-long-argument-list-helper (f32.const 0) (i32.const 0) (i32.const 0) (f64.const 0) (f32.const 0) (f32.const 0) (f32.const 0) (f64.const 0) (f32.const 0) (i32.const 0) (i32.const 0) (f32.const 0) (f64.const 0) (i64.const 0) (i64.const 0) (i32.const 0) (i64.const 0) (i64.const 0) (f32.const 0) (i64.const 0) (i64.const 0) (i64.const 0) (i32.const 0) (f32.const 0) (f32.const 0) (f32.const 0) (f64.const 0) (f32.const 0) (i32.const 0) (i64.const 0) (f32.const 0) (f64.const 0) (f64.const 0) (f32.const 0) (i32.const 0) (f32.const 0) (f32.const 0) (f64.const 0) (i64.const 0) (f64.const 0) (i32.const 0) (i64.const 0) (f32.const 0) (f64.const 0) (i32.const 0) (i32.const 0) (i32.const 0) (i64.const 0) (f64.const 0) (i32.const 0) (i64.const 0) (i64.const 0) (f64.const 0) (f64.const 0) (f64.const 0) (f64.const 0) (f64.const 0) (f64.const 0) (i32.const 0) (f32.const 0) (f64.const 0) (f64.const 0) (i32.const 0) (i64.const 0) (f32.const 0) (f32.const 0) (f32.const 0) (i32.const 0) (f64.const 0) (f64.const 0) (f64.const 0) (f64.const 0) (f64.const 0) (f32.const 0) (i64.const 0) (i64.const 0) (i32.const 0) (i32.const 0) (i32.const 0) (f32.const 0) (f64.const 0) (i32.const 0) (i64.const 0) (f32.const 0) (f32.const 0) (f32.const 0) (i32.const 0) (i32.const 0) (f32.const 0) (f64.const 0) (i64.const 0) (f32.const 0) (f64.const 0) (f32.const 0) (f32.const 0) (f32.const 0) (i32.const 0) (f32.const 0) (i64.const 0) (local.get 0))
  )
)
"#
    );
}

#[test]
fn conversions() {
    assert_snapshot!(
        r#"
(module
  (func (export "i64.extend_i32_s") (param $x i32) (result i64) (i64.extend_i32_s (local.get $x)))
  (func (export "i64.extend_i32_u") (param $x i32) (result i64) (i64.extend_i32_u (local.get $x)))
  (func (export "i32.wrap_i64") (param $x i64) (result i32) (i32.wrap_i64 (local.get $x)))
  (func (export "i32.trunc_f32_s") (param $x f32) (result i32) (i32.trunc_f32_s (local.get $x)))
  (func (export "i32.trunc_f32_u") (param $x f32) (result i32) (i32.trunc_f32_u (local.get $x)))
  (func (export "i32.trunc_f64_s") (param $x f64) (result i32) (i32.trunc_f64_s (local.get $x)))
  (func (export "i32.trunc_f64_u") (param $x f64) (result i32) (i32.trunc_f64_u (local.get $x)))
  (func (export "i64.trunc_f32_s") (param $x f32) (result i64) (i64.trunc_f32_s (local.get $x)))
  (func (export "i64.trunc_f32_u") (param $x f32) (result i64) (i64.trunc_f32_u (local.get $x)))
  (func (export "i64.trunc_f64_s") (param $x f64) (result i64) (i64.trunc_f64_s (local.get $x)))
  (func (export "i64.trunc_f64_u") (param $x f64) (result i64) (i64.trunc_f64_u (local.get $x)))
  (func (export "i32.trunc_sat_f32_s") (param $x f32) (result i32) (i32.trunc_sat_f32_s (local.get $x)))
  (func (export "i32.trunc_sat_f32_u") (param $x f32) (result i32) (i32.trunc_sat_f32_u (local.get $x)))
  (func (export "i32.trunc_sat_f64_s") (param $x f64) (result i32) (i32.trunc_sat_f64_s (local.get $x)))
  (func (export "i32.trunc_sat_f64_u") (param $x f64) (result i32) (i32.trunc_sat_f64_u (local.get $x)))
  (func (export "i64.trunc_sat_f32_s") (param $x f32) (result i64) (i64.trunc_sat_f32_s (local.get $x)))
  (func (export "i64.trunc_sat_f32_u") (param $x f32) (result i64) (i64.trunc_sat_f32_u (local.get $x)))
  (func (export "i64.trunc_sat_f64_s") (param $x f64) (result i64) (i64.trunc_sat_f64_s (local.get $x)))
  (func (export "i64.trunc_sat_f64_u") (param $x f64) (result i64) (i64.trunc_sat_f64_u (local.get $x)))
  (func (export "f32.convert_i32_s") (param $x i32) (result f32) (f32.convert_i32_s (local.get $x)))
  (func (export "f32.convert_i64_s") (param $x i64) (result f32) (f32.convert_i64_s (local.get $x)))
  (func (export "f64.convert_i32_s") (param $x i32) (result f64) (f64.convert_i32_s (local.get $x)))
  (func (export "f64.convert_i64_s") (param $x i64) (result f64) (f64.convert_i64_s (local.get $x)))
  (func (export "f32.convert_i32_u") (param $x i32) (result f32) (f32.convert_i32_u (local.get $x)))
  (func (export "f32.convert_i64_u") (param $x i64) (result f32) (f32.convert_i64_u (local.get $x)))
  (func (export "f64.convert_i32_u") (param $x i32) (result f64) (f64.convert_i32_u (local.get $x)))
  (func (export "f64.convert_i64_u") (param $x i64) (result f64) (f64.convert_i64_u (local.get $x)))
  (func (export "f64.promote_f32") (param $x f32) (result f64) (f64.promote_f32 (local.get $x)))
  (func (export "f32.demote_f64") (param $x f64) (result f32) (f32.demote_f64 (local.get $x)))
  (func (export "f32.reinterpret_i32") (param $x i32) (result f32) (f32.reinterpret_i32 (local.get $x)))
  (func (export "f64.reinterpret_i64") (param $x i64) (result f64) (f64.reinterpret_i64 (local.get $x)))
  (func (export "i32.reinterpret_f32") (param $x f32) (result i32) (i32.reinterpret_f32 (local.get $x)))
  (func (export "i64.reinterpret_f64") (param $x f64) (result i64) (i64.reinterpret_f64 (local.get $x)))
)
"#
    );
}

#[test]
fn endianness() {
    assert_snapshot!(
        r#"
(module
  (memory 1)

  ;; Stores an i16 value in little-endian-format
  (func $i16_store_little (param $address i32) (param $value i32)
    (i32.store8 (local.get $address) (local.get $value))
    (i32.store8 (i32.add (local.get $address) (i32.const 1)) (i32.shr_u (local.get $value) (i32.const 8)))
  )

  ;; Stores an i32 value in little-endian format
  (func $i32_store_little (param $address i32) (param $value i32)
    (call $i16_store_little (local.get $address) (local.get $value))
    (call $i16_store_little (i32.add (local.get $address) (i32.const 2)) (i32.shr_u (local.get $value) (i32.const 16)))
  )

  ;; Stores an i64 value in little-endian format
  (func $i64_store_little (param $address i32) (param $value i64)
    (call $i32_store_little (local.get $address) (i32.wrap_i64 (local.get $value)))
    (call $i32_store_little (i32.add (local.get $address) (i32.const 4)) (i32.wrap_i64 (i64.shr_u (local.get $value) (i64.const 32))))
  )

  ;; Loads an i16 value in little-endian format
  (func $i16_load_little (param $address i32) (result i32)
    (i32.or
      (i32.load8_u (local.get $address))
      (i32.shl (i32.load8_u (i32.add (local.get $address) (i32.const 1))) (i32.const 8))
    )
  )

  ;; Loads an i32 value in little-endian format
  (func $i32_load_little (param $address i32) (result i32)
    (i32.or
      (call $i16_load_little (local.get $address))
      (i32.shl (call $i16_load_little (i32.add (local.get $address) (i32.const 2))) (i32.const 16))
    )
  )

  ;; Loads an i64 value in little-endian format
  (func $i64_load_little (param $address i32) (result i64)
    (i64.or
      (i64.extend_i32_u (call $i32_load_little (local.get $address)))
      (i64.shl (i64.extend_i32_u (call $i32_load_little (i32.add (local.get $address) (i32.const 4)))) (i64.const 32))
    )
  )

  (func (export "i32_load16_s") (param $value i32) (result i32)
    (call $i16_store_little (i32.const 0) (local.get $value))
    (i32.load16_s (i32.const 0))
  )

  (func (export "i32_load16_u") (param $value i32) (result i32)
    (call $i16_store_little (i32.const 0) (local.get $value))
    (i32.load16_u (i32.const 0))
  )

  (func (export "i32_load") (param $value i32) (result i32)
    (call $i32_store_little (i32.const 0) (local.get $value))
    (i32.load (i32.const 0))
  )

  (func (export "i64_load16_s") (param $value i64) (result i64)
    (call $i16_store_little (i32.const 0) (i32.wrap_i64 (local.get $value)))
    (i64.load16_s (i32.const 0))
  )

  (func (export "i64_load16_u") (param $value i64) (result i64)
    (call $i16_store_little (i32.const 0) (i32.wrap_i64 (local.get $value)))
    (i64.load16_u (i32.const 0))
  )

  (func (export "i64_load32_s") (param $value i64) (result i64)
    (call $i32_store_little (i32.const 0) (i32.wrap_i64 (local.get $value)))
    (i64.load32_s (i32.const 0))
  )

  (func (export "i64_load32_u") (param $value i64) (result i64)
    (call $i32_store_little (i32.const 0) (i32.wrap_i64 (local.get $value)))
    (i64.load32_u (i32.const 0))
  )

  (func (export "i64_load") (param $value i64) (result i64)
    (call $i64_store_little (i32.const 0) (local.get $value))
    (i64.load (i32.const 0))
  )

  (func (export "f32_load") (param $value f32) (result f32)
    (call $i32_store_little (i32.const 0) (i32.reinterpret_f32 (local.get $value)))
    (f32.load (i32.const 0))
  )

  (func (export "f64_load") (param $value f64) (result f64)
    (call $i64_store_little (i32.const 0) (i64.reinterpret_f64 (local.get $value)))
    (f64.load (i32.const 0))
  )


  (func (export "i32_store16") (param $value i32) (result i32)
    (i32.store16 (i32.const 0) (local.get $value))
    (call $i16_load_little (i32.const 0))
  )

  (func (export "i32_store") (param $value i32) (result i32)
    (i32.store (i32.const 0) (local.get $value))
    (call $i32_load_little (i32.const 0))
  )

  (func (export "i64_store16") (param $value i64) (result i64)
    (i64.store16 (i32.const 0) (local.get $value))
    (i64.extend_i32_u (call $i16_load_little (i32.const 0)))
  )

  (func (export "i64_store32") (param $value i64) (result i64)
    (i64.store32 (i32.const 0) (local.get $value))
    (i64.extend_i32_u (call $i32_load_little (i32.const 0)))
  )

  (func (export "i64_store") (param $value i64) (result i64)
    (i64.store (i32.const 0) (local.get $value))
    (call $i64_load_little (i32.const 0))
  )

  (func (export "f32_store") (param $value f32) (result f32)
    (f32.store (i32.const 0) (local.get $value))
    (f32.reinterpret_i32 (call $i32_load_little (i32.const 0)))
  )

  (func (export "f64_store") (param $value f64) (result f64)
    (f64.store (i32.const 0) (local.get $value))
    (f64.reinterpret_i64 (call $i64_load_little (i32.const 0)))
  )
)
"#
    );
}

#[test]
fn fac() {
    assert_snapshot!(
        r#"
(module
  ;; Recursive factorial
  (func (export "fac-rec") (param i64) (result i64)
    (if (result i64) (i64.eq (local.get 0) (i64.const 0))
      (then (i64.const 1))
      (else
        (i64.mul (local.get 0) (call 0 (i64.sub (local.get 0) (i64.const 1))))
      )
    )
  )

  ;; Recursive factorial named
  (func $fac-rec-named (export "fac-rec-named") (param $n i64) (result i64)
    (if (result i64) (i64.eq (local.get $n) (i64.const 0))
      (then (i64.const 1))
      (else
        (i64.mul
          (local.get $n)
          (call $fac-rec-named (i64.sub (local.get $n) (i64.const 1)))
        )
      )
    )
  )

  ;; Iterative factorial
  (func (export "fac-iter") (param i64) (result i64)
    (local i64 i64)
    (local.set 1 (local.get 0))
    (local.set 2 (i64.const 1))
    (block
      (loop
        (if
          (i64.eq (local.get 1) (i64.const 0))
          (then (br 2))
          (else
            (local.set 2 (i64.mul (local.get 1) (local.get 2)))
            (local.set 1 (i64.sub (local.get 1) (i64.const 1)))
          )
        )
        (br 0)
      )
    )
    (local.get 2)
  )

  ;; Iterative factorial named
  (func (export "fac-iter-named") (param $n i64) (result i64)
    (local $i i64)
    (local $res i64)
    (local.set $i (local.get $n))
    (local.set $res (i64.const 1))
    (block $done
      (loop $loop
        (if
          (i64.eq (local.get $i) (i64.const 0))
          (then (br $done))
          (else
            (local.set $res (i64.mul (local.get $i) (local.get $res)))
            (local.set $i (i64.sub (local.get $i) (i64.const 1)))
          )
        )
        (br $loop)
      )
    )
    (local.get $res)
  )

  ;; Optimized factorial.
  (func (export "fac-opt") (param i64) (result i64)
    (local i64)
    (local.set 1 (i64.const 1))
    (block
      (br_if 0 (i64.lt_s (local.get 0) (i64.const 2)))
      (loop
        (local.set 1 (i64.mul (local.get 1) (local.get 0)))
        (local.set 0 (i64.add (local.get 0) (i64.const -1)))
        (br_if 0 (i64.gt_s (local.get 0) (i64.const 1)))
      )
    )
    (local.get 1)
  )

  ;; Iterative factorial without locals.
  (func $pick0 (param i64) (result i64 i64)
    (local.get 0) (local.get 0)
  )
  (func $pick1 (param i64 i64) (result i64 i64 i64)
    (local.get 0) (local.get 1) (local.get 0)
  )
  (func (export "fac-ssa") (param i64) (result i64)
    (i64.const 1) (local.get 0)
    (loop $l (param i64 i64) (result i64)
      (call $pick1) (call $pick1) (i64.mul)
      (call $pick1) (i64.const 1) (i64.sub)
      (call $pick0) (i64.const 0) (i64.gt_u)
      (br_if $l)
      (drop) (return)
    )
  )
)
"#
    );
}

#[test]
fn fib() {
    assert_snapshot!(
        r#"
(module
  ;; Define a function type: takes an i32 parameter and returns an i32 value
  (type $fib-type (func (param i32) (result i32)))

  ;; Recursive Fibonacci function
  (func $fib-recursive (type $fib-type)
    (param $n i32)       ;; Parameter n
    (result i32)         ;; Return type

    ;; Base cases
    (if (i32.le_u (local.get $n) (i32.const 1))
      (then
        (return (local.get $n)) ;; Return n if n <= 1
      )
    )

    ;; Recursive case: fib(n-1) + fib(n-2)
    (i32.add
      (call $fib-recursive (i32.sub (local.get $n) (i32.const 1))) ;; fib(n-1)
      (call $fib-recursive (i32.sub (local.get $n) (i32.const 2))) ;; fib(n-2)
    )
  )

  ;; Iterative Fibonacci function
  (func $fib-iterative (type $fib-type)
    (param $n i32)       ;; Parameter n
    (result i32)         ;; Return type
    (local $a i32)       ;; Local variable a
    (local $b i32)       ;; Local variable b
    (local $i i32)       ;; Local variable i
    (local $tmp i32)     ;; Local variable tmp

    ;; Initialize local variables
    (local.set $a (i32.const 0))  ;; a = 0
    (local.set $b (i32.const 1))  ;; b = 1
    (local.set $i (i32.const 0))  ;; i = 0

    ;; If n == 0, return 0 directly
    (if (i32.eq (local.get $n) (i32.const 0))
      (then
        (return (i32.const 0))
      )
    )

    ;; If n == 1, return 1 directly
    (if (i32.eq (local.get $n) (i32.const 1))
      (then
        (return (i32.const 1))
      )
    )

    ;; Loop to compute the Fibonacci sequence
    (loop $loop
      ;; Compute the next Fibonacci number
      (local.set $tmp (local.get $b))          ;; tmp = b
      (local.set $b (i32.add (local.get $a) (local.get $b))) ;; b = a + b
      (local.set $a (local.get $tmp))          ;; a = tmp

      ;; Increment the counter
      (local.set $i (i32.add (local.get $i) (i32.const 1))) ;; i++

      ;; If i < n - 1, continue the loop
      (br_if $loop (i32.lt_u (local.get $i) (i32.sub (local.get $n) (i32.const 1))))
    )

    ;; Return the result
    (local.get $b)
  )

  ;; Export the recursive Fibonacci function as "fib_recursive"
  (export "fib_recursive" (func $fib-recursive))

  ;; Export the iterative Fibonacci function as "fib_iterative"
  (export "fib_iterative" (func $fib-iterative))
)
"#
    );
}

#[test]
fn func() {
    assert_snapshot!(
        r#"
(module
  ;; Auxiliary definition
  (type $sig (func))
  (func $dummy)

  ;; Syntax

  (func)
  (func (export "f"))
  (func $f)
  (func $h (export "g"))

  (func (local))
  (func (local) (local))
  (func (local i32))
  (func (local $x i32))
  (func (local i32 f64 i64))
  (func (local i32) (local f64))
  (func (local i32 f32) (local $x i64) (local) (local i32 f64))

  (func (param))
  (func (param) (param))
  (func (param i32))
  (func (param $x i32))
  (func (param i32 f64 i64))
  (func (param i32) (param f64))
  (func (param i32 f32) (param $x i64) (param) (param i32 f64))

  (func (result))
  (func (result) (result))
  (func (result i32) (unreachable))
  (func (result i32 f64 f32) (unreachable))
  (func (result i32) (result f64) (unreachable))
  (func (result i32 f32) (result i64) (result) (result i32 f64) (unreachable))

  (type $sig-1 (func))
  (type $sig-2 (func (result i32)))
  (type $sig-3 (func (param $x i32)))
  (type $sig-4 (func (param i32 f64 i32) (result i32)))

  (func (export "type-use-1") (type $sig-1))
  (func (export "type-use-2") (type $sig-2) (i32.const 0))
  (func (export "type-use-3") (type $sig-3))
  (func (export "type-use-4") (type $sig-4) (i32.const 0))
  (func (export "type-use-5") (type $sig-2) (result i32) (i32.const 0))
  (func (export "type-use-6") (type $sig-3) (param i32))
  (func (export "type-use-7")
    (type $sig-4) (param i32) (param f64 i32) (result i32) (i32.const 0)
  )

  (func (type $sig))
  (func (type $forward))  ;; forward reference

  (func $complex
    (param i32 f32) (param $x i64) (param) (param i32)
    (result) (result i32) (result) (result i64 i32)
    (local f32) (local $y i32) (local i64 i32) (local) (local f64 i32)
    (unreachable) (unreachable)
  )
  (func $complex-sig
    (type $sig)
    (local f32) (local $y i32) (local i64 i32) (local) (local f64 i32)
    (unreachable) (unreachable)
  )

  (type $forward (func))

  ;; Typing of locals

  (func (export "local-first-i32") (result i32) (local i32 i32) (local.get 0))
  (func (export "local-first-i64") (result i64) (local i64 i64) (local.get 0))
  (func (export "local-first-f32") (result f32) (local f32 f32) (local.get 0))
  (func (export "local-first-f64") (result f64) (local f64 f64) (local.get 0))
  (func (export "local-second-i32") (result i32) (local i32 i32) (local.get 1))
  (func (export "local-second-i64") (result i64) (local i64 i64) (local.get 1))
  (func (export "local-second-f32") (result f32) (local f32 f32) (local.get 1))
  (func (export "local-second-f64") (result f64) (local f64 f64) (local.get 1))
  (func (export "local-mixed") (result f64)
    (local f32) (local $x i32) (local i64 i32) (local) (local f64 i32)
    (drop (f32.neg (local.get 0)))
    (drop (i32.eqz (local.get 1)))
    (drop (i64.eqz (local.get 2)))
    (drop (i32.eqz (local.get 3)))
    (drop (f64.neg (local.get 4)))
    (drop (i32.eqz (local.get 5)))
    (local.get 4)
  )

  ;; Typing of parameters

  (func (export "param-first-i32") (param i32 i32) (result i32) (local.get 0))
  (func (export "param-first-i64") (param i64 i64) (result i64) (local.get 0))
  (func (export "param-first-f32") (param f32 f32) (result f32) (local.get 0))
  (func (export "param-first-f64") (param f64 f64) (result f64) (local.get 0))
  (func (export "param-second-i32") (param i32 i32) (result i32) (local.get 1))
  (func (export "param-second-i64") (param i64 i64) (result i64) (local.get 1))
  (func (export "param-second-f32") (param f32 f32) (result f32) (local.get 1))
  (func (export "param-second-f64") (param f64 f64) (result f64) (local.get 1))
  (func (export "param-mixed") (param f32 i32) (param) (param $x i64) (param i32 f64 i32)
    (result f64)
    (drop (f32.neg (local.get 0)))
    (drop (i32.eqz (local.get 1)))
    (drop (i64.eqz (local.get 2)))
    (drop (i32.eqz (local.get 3)))
    (drop (f64.neg (local.get 4)))
    (drop (i32.eqz (local.get 5)))
    (local.get 4)
  )

  ;; Typing of results

  (func (export "empty"))
  (func (export "value-void") (call $dummy))
  (func (export "value-i32") (result i32) (i32.const 77))
  (func (export "value-i64") (result i64) (i64.const 7777))
  (func (export "value-f32") (result f32) (f32.const 77.7))
  (func (export "value-f64") (result f64) (f64.const 77.77))
  (func (export "value-i32-f64") (result i32 f64) (i32.const 77) (f64.const 7))
  (func (export "value-i32-i32-i32") (result i32 i32 i32)
    (i32.const 1) (i32.const 2) (i32.const 3)
  )
  (func (export "value-block-void") (block (call $dummy) (call $dummy)))
  (func (export "value-block-i32") (result i32)
    (block (result i32) (call $dummy) (i32.const 77))
  )
  (func (export "value-block-i32-i64") (result i32 i64)
    (block (result i32 i64) (call $dummy) (i32.const 1) (i64.const 2))
  )

  (func (export "return-empty") (return))
  (func (export "return-i32") (result i32) (return (i32.const 78)))
  (func (export "return-i64") (result i64) (return (i64.const 7878)))
  (func (export "return-f32") (result f32) (return (f32.const 78.7)))
  (func (export "return-f64") (result f64) (return (f64.const 78.78)))
  (func (export "return-i32-f64") (result i32 f64)
    (return (i32.const 78) (f64.const 78.78))
  )
  (func (export "return-i32-i32-i32") (result i32 i32 i32)
    (return (i32.const 1) (i32.const 2) (i32.const 3))
  )
  (func (export "return-block-i32") (result i32)
    (return (block (result i32) (call $dummy) (i32.const 77)))
  )
  (func (export "return-block-i32-i64") (result i32 i64)
    (return (block (result i32 i64) (call $dummy) (i32.const 1) (i64.const 2)))
  )

  (func (export "break-empty") (br 0))
  (func (export "break-i32") (result i32) (br 0 (i32.const 79)))
  (func (export "break-i64") (result i64) (br 0 (i64.const 7979)))
  (func (export "break-f32") (result f32) (br 0 (f32.const 79.9)))
  (func (export "break-f64") (result f64) (br 0 (f64.const 79.79)))
  (func (export "break-i32-f64") (result i32 f64)
    (br 0 (i32.const 79) (f64.const 79.79))
  )
  (func (export "break-i32-i32-i32") (result i32 i32 i32)
    (br 0 (i32.const 1) (i32.const 2) (i32.const 3))
  )
  (func (export "break-block-i32") (result i32)
    (br 0 (block (result i32) (call $dummy) (i32.const 77)))
  )
  (func (export "break-block-i32-i64") (result i32 i64)
    (br 0 (block (result i32 i64) (call $dummy) (i32.const 1) (i64.const 2)))
  )

  (func (export "break-br_if-empty") (param i32)
    (br_if 0 (local.get 0))
  )
  (func (export "break-br_if-num") (param i32) (result i32)
    (drop (br_if 0 (i32.const 50) (local.get 0))) (i32.const 51)
  )
  (func (export "break-br_if-num-num") (param i32) (result i32 i64)
    (drop (drop (br_if 0 (i32.const 50) (i64.const 51) (local.get 0))))
    (i32.const 51) (i64.const 52)
  )

  (func (export "break-br_table-empty") (param i32)
    (br_table 0 0 0 (local.get 0))
  )
  (func (export "break-br_table-num") (param i32) (result i32)
    (br_table 0 0 (i32.const 50) (local.get 0)) (i32.const 51)
  )
  (func (export "break-br_table-num-num") (param i32) (result i32 i64)
    (br_table 0 0 (i32.const 50) (i64.const 51) (local.get 0))
    (i32.const 51) (i64.const 52)
  )
  (func (export "break-br_table-nested-empty") (param i32)
    (block (br_table 0 1 0 (local.get 0)))
  )
  (func (export "break-br_table-nested-num") (param i32) (result i32)
    (i32.add
      (block (result i32)
        (br_table 0 1 0 (i32.const 50) (local.get 0)) (i32.const 51)
      )
      (i32.const 2)
    )
  )
  (func (export "break-br_table-nested-num-num") (param i32) (result i32 i32)
    (i32.add
      (block (result i32 i32)
        (br_table 0 1 0 (i32.const 50) (i32.const 51) (local.get 0))
        (i32.const 51) (i32.const -3)
      )
    )
    (i32.const 52)
  )

  ;; Large signatures

  (func (export "large-sig")
    (param i32 i64 f32 f32 i32 f64 f32 i32 i32 i32 f32 f64 f64 f64 i32 i32 f32)
    (result f64 f32 i32 i32 i32 i64 f32 i32 i32 f32 f64 f64 i32 f32 i32 f64)
    (local.get 5)
    (local.get 2)
    (local.get 0)
    (local.get 8)
    (local.get 7)
    (local.get 1)
    (local.get 3)
    (local.get 9)
    (local.get 4)
    (local.get 6)
    (local.get 13)
    (local.get 11)
    (local.get 15)
    (local.get 16)
    (local.get 14)
    (local.get 12)
  )

  ;; Default initialization of locals

  (func (export "init-local-i32") (result i32) (local i32) (local.get 0))
  (func (export "init-local-i64") (result i64) (local i64) (local.get 0))
  (func (export "init-local-f32") (result f32) (local f32) (local.get 0))
  (func (export "init-local-f64") (result f64) (local f64) (local.get 0))
)
"#
    );
}

#[test]
fn func_ptr() {
    assert_snapshot!(
        r#"
(module
  (type    (func))                           ;; 0: void -> void
  (type $S (func))                           ;; 1: void -> void
  (type    (func (param)))                   ;; 2: void -> void
  (type    (func (result i32)))              ;; 3: void -> i32
  (type    (func (param) (result i32)))      ;; 4: void -> i32
  (type $T (func (param i32) (result i32)))  ;; 5: i32 -> i32
  (type $U (func (param i32)))               ;; 6: i32 -> void

  (func $print (import "spectest" "print_i32") (type 6))

  (func (type 0))
  (func (type $S))

  (func (export "one") (type 4) (i32.const 13))
  (func (export "two") (type $T) (i32.add (local.get 0) (i32.const 1)))

  ;; Both signature and parameters are allowed (and required to match)
  ;; since this allows the naming of parameters.
  (func (export "three") (type $T) (param $a i32) (result i32)
    (i32.sub (local.get 0) (i32.const 2))
  )

  ;; N ot implemented: wasm imported functions
  ;; (func (export "four") (type $U) (call $print (local.get 0)))
)
"#
    );
}

#[test]
fn global() {
    assert_snapshot!(
        r#"
(module
  (global (import "spectest" "global_i32") i32)
  (global (import "spectest" "global_i64") i64)

  (global $a i32 (i32.const -2))
  (global (;3;) f32 (f32.const -3))
  (global (;4;) f64 (f64.const -4))
  (global $b i64 (i64.const -5))

  (global $x (mut i32) (i32.const -12))
  (global (;7;) (mut f32) (f32.const -13))
  (global (;8;) (mut f64) (f64.const -14))
  (global $y (mut i64) (i64.const -15))

  (global $z1 i32 (global.get 0))
  (global $z2 i64 (global.get 1))

  (global $r externref (ref.null extern))
  (global $mr (mut externref) (ref.null extern))
  (global funcref (ref.null func))

  (func (export "get-a") (result i32) (global.get $a))
  (func (export "get-b") (result i64) (global.get $b))
  (func (export "get-r") (result externref) (global.get $r))
  (func (export "get-mr") (result externref) (global.get $mr))
  (func (export "get-x") (result i32) (global.get $x))
  (func (export "get-y") (result i64) (global.get $y))
  (func (export "get-z1") (result i32) (global.get $z1))
  (func (export "get-z2") (result i64) (global.get $z2))
  (func (export "set-x") (param i32) (global.set $x (local.get 0)))
  (func (export "set-y") (param i64) (global.set $y (local.get 0)))
  (func (export "set-mr") (param externref) (global.set $mr (local.get 0)))

  (func (export "get-3") (result f32) (global.get 3))
  (func (export "get-4") (result f64) (global.get 4))
  (func (export "get-7") (result f32) (global.get 7))
  (func (export "get-8") (result f64) (global.get 8))
  (func (export "set-7") (param f32) (global.set 7 (local.get 0)))
  (func (export "set-8") (param f64) (global.set 8 (local.get 0)))

  ;; As the argument of control constructs and instructions

  (memory 1)

  (func $dummy)

  (func (export "as-select-first") (result i32)
    (select (global.get $x) (i32.const 2) (i32.const 3))
  )
  (func (export "as-select-mid") (result i32)
    (select (i32.const 2) (global.get $x) (i32.const 3))
  )
  (func (export "as-select-last") (result i32)
    (select (i32.const 2) (i32.const 3) (global.get $x))
  )

  (func (export "as-loop-first") (result i32)
    (loop (result i32)
      (global.get $x) (call $dummy) (call $dummy)
    )
  )
  (func (export "as-loop-mid") (result i32)
    (loop (result i32)
      (call $dummy) (global.get $x) (call $dummy)
    )
  )
  (func (export "as-loop-last") (result i32)
    (loop (result i32)
      (call $dummy) (call $dummy) (global.get $x)
    )
  )

  (func (export "as-if-condition") (result i32)
    (if (result i32) (global.get $x)
      (then (call $dummy) (i32.const 2))
      (else (call $dummy) (i32.const 3))
    )
  )
  (func (export "as-if-then") (result i32)
    (if (result i32) (i32.const 1)
      (then (global.get $x)) (else (i32.const 2))
    )
  )
  (func (export "as-if-else") (result i32)
    (if (result i32) (i32.const 0)
      (then (i32.const 2)) (else (global.get $x))
    )
  )

  (func (export "as-br_if-first") (result i32)
    (block (result i32)
      (br_if 0 (global.get $x) (i32.const 2))
      (return (i32.const 3))
    )
  )
  (func (export "as-br_if-last") (result i32)
    (block (result i32)
      (br_if 0 (i32.const 2) (global.get $x))
      (return (i32.const 3))
    )
  )

  (func (export "as-br_table-first") (result i32)
    (block (result i32)
      (global.get $x) (i32.const 2) (br_table 0 0)
    )
  )
  (func (export "as-br_table-last") (result i32)
    (block (result i32)
      (i32.const 2) (global.get $x) (br_table 0 0)
    )
  )

  (func $func (param i32 i32) (result i32) (local.get 0))
  (type $check (func (param i32 i32) (result i32)))
  (table funcref (elem $func))
  (func (export "as-call_indirect-first") (result i32)
    (block (result i32)
      (call_indirect (type $check)
        (global.get $x) (i32.const 2) (i32.const 0)
      )
    )
  )
  (func (export "as-call_indirect-mid") (result i32)
    (block (result i32)
      (call_indirect (type $check)
        (i32.const 2) (global.get $x) (i32.const 0)
      )
    )
  )
  (func (export "as-call_indirect-last") (result i32)
    (block (result i32)
      (call_indirect (type $check)
        (i32.const 2) (i32.const 0) (global.get $x)
      )
    )
  )

  (func (export "as-store-first")
    (global.get $x) (i32.const 1) (i32.store)
  )
  (func (export "as-store-last")
    (i32.const 0) (global.get $x) (i32.store)
  )
  (func (export "as-load-operand") (result i32)
    (i32.load (global.get $x))
  )
  (func (export "as-memory.grow-value") (result i32)
    (memory.grow (global.get $x))
  )

  (func $f (param i32) (result i32) (local.get 0))
  (func (export "as-call-value") (result i32)
    (call $f (global.get $x))
  )

  (func (export "as-return-value") (result i32)
    (global.get $x) (return)
  )
  (func (export "as-drop-operand")
    (drop (global.get $x))
  )
  (func (export "as-br-value") (result i32)
    (block (result i32) (br 0 (global.get $x)))
  )

  (func (export "as-local.set-value") (param i32) (result i32)
    (local.set 0 (global.get $x))
    (local.get 0)
  )
  (func (export "as-local.tee-value") (param i32) (result i32)
    (local.tee 0 (global.get $x))
  )
  (func (export "as-global.set-value") (result i32)
    (global.set $x (global.get $x))
    (global.get $x)
  )

  (func (export "as-unary-operand") (result i32)
    (i32.eqz (global.get $x))
  )
  (func (export "as-binary-operand") (result i32)
    (i32.mul
      (global.get $x) (global.get $x)
    )
  )
  (func (export "as-compare-operand") (result i32)
    (i32.gt_u
      (global.get 0) (i32.const 1)
    )
  )
)
"#
    );
}

#[test]
fn r#if() {
    assert_snapshot!(
        r#"
(module
  ;; Auxiliary definition
  (memory 1)

  (func $dummy)

  (func (export "empty") (param i32)
    (if (local.get 0) (then))
    (if (local.get 0) (then) (else))
    (if $l (local.get 0) (then))
    (if $l (local.get 0) (then) (else))
  )

  (func (export "singular") (param i32) (result i32)
    (if (local.get 0) (then (nop)))
    (if (local.get 0) (then (nop)) (else (nop)))
    (if (result i32) (local.get 0) (then (i32.const 7)) (else (i32.const 8)))
  )

  (func (export "multi") (param i32) (result i32 i32)
    (if (local.get 0) (then (call $dummy) (call $dummy) (call $dummy)))
    (if (local.get 0) (then) (else (call $dummy) (call $dummy) (call $dummy)))
    (if (result i32) (local.get 0)
      (then (call $dummy) (call $dummy) (i32.const 8) (call $dummy))
      (else (call $dummy) (call $dummy) (i32.const 9) (call $dummy))
    )
    (if (result i32 i64 i32) (local.get 0)
      (then
        (call $dummy) (call $dummy) (i32.const 1) (call $dummy)
        (call $dummy) (call $dummy) (i64.const 2) (call $dummy)
        (call $dummy) (call $dummy) (i32.const 3) (call $dummy)
      )
      (else
        (call $dummy) (call $dummy) (i32.const -1) (call $dummy)
        (call $dummy) (call $dummy) (i64.const -2) (call $dummy)
        (call $dummy) (call $dummy) (i32.const -3) (call $dummy)
      )
    )
    (drop) (drop)
  )

  (func (export "nested") (param i32 i32) (result i32)
    (if (result i32) (local.get 0)
      (then
        (if (local.get 1) (then (call $dummy) (block) (nop)))
        (if (local.get 1) (then) (else (call $dummy) (block) (nop)))
        (if (result i32) (local.get 1)
          (then (call $dummy) (i32.const 9))
          (else (call $dummy) (i32.const 10))
        )
      )
      (else
        (if (local.get 1) (then (call $dummy) (block) (nop)))
        (if (local.get 1) (then) (else (call $dummy) (block) (nop)))
        (if (result i32) (local.get 1)
          (then (call $dummy) (i32.const 10))
          (else (call $dummy) (i32.const 11))
        )
      )
    )
  )

  (func (export "as-select-first") (param i32) (result i32)
    (select
      (if (result i32) (local.get 0)
        (then (call $dummy) (i32.const 1))
        (else (call $dummy) (i32.const 0))
      )
      (i32.const 2) (i32.const 3)
    )
  )
  (func (export "as-select-mid") (param i32) (result i32)
    (select
      (i32.const 2)
      (if (result i32) (local.get 0)
        (then (call $dummy) (i32.const 1))
        (else (call $dummy) (i32.const 0))
      )
      (i32.const 3)
    )
  )
  (func (export "as-select-last") (param i32) (result i32)
    (select
      (i32.const 2) (i32.const 3)
      (if (result i32) (local.get 0)
        (then (call $dummy) (i32.const 1))
        (else (call $dummy) (i32.const 0))
      )
    )
  )

  (func (export "as-loop-first") (param i32) (result i32)
    (loop (result i32)
      (if (result i32) (local.get 0)
        (then (call $dummy) (i32.const 1))
        (else (call $dummy) (i32.const 0))
      )
      (call $dummy) (call $dummy)
    )
  )
  (func (export "as-loop-mid") (param i32) (result i32)
    (loop (result i32)
      (call $dummy)
      (if (result i32) (local.get 0)
        (then (call $dummy) (i32.const 1))
        (else (call $dummy) (i32.const 0))
      )
      (call $dummy)
    )
  )
  (func (export "as-loop-last") (param i32) (result i32)
    (loop (result i32)
      (call $dummy) (call $dummy)
      (if (result i32) (local.get 0)
        (then (call $dummy) (i32.const 1))
        (else (call $dummy) (i32.const 0))
      )
    )
  )

  (func (export "as-if-condition") (param i32) (result i32)
    (if (result i32)
      (if (result i32) (local.get 0)
        (then (i32.const 1)) (else (i32.const 0))
      )
      (then (call $dummy) (i32.const 2))
      (else (call $dummy) (i32.const 3))
    )
  )

  (func (export "as-br_if-first") (param i32) (result i32)
    (block (result i32)
      (br_if 0
        (if (result i32) (local.get 0)
          (then (call $dummy) (i32.const 1))
          (else (call $dummy) (i32.const 0))
        )
        (i32.const 2)
      )
      (return (i32.const 3))
    )
  )
  (func (export "as-br_if-last") (param i32) (result i32)
    (block (result i32)
      (br_if 0
        (i32.const 2)
        (if (result i32) (local.get 0)
          (then (call $dummy) (i32.const 1))
          (else (call $dummy) (i32.const 0))
        )
      )
      (return (i32.const 3))
    )
  )

  (func (export "as-br_table-first") (param i32) (result i32)
    (block (result i32)
      (if (result i32) (local.get 0)
        (then (call $dummy) (i32.const 1))
        (else (call $dummy) (i32.const 0))
      )
      (i32.const 2)
      (br_table 0 0)
    )
  )
  (func (export "as-br_table-last") (param i32) (result i32)
    (block (result i32)
      (i32.const 2)
      (if (result i32) (local.get 0)
        (then (call $dummy) (i32.const 1))
        (else (call $dummy) (i32.const 0))
      )
      (br_table 0 0)
    )
  )

  (func $func (param i32 i32) (result i32) (local.get 0))
  (type $check (func (param i32 i32) (result i32)))
  (table funcref (elem $func))
  (func (export "as-call_indirect-first") (param i32) (result i32)
    (block (result i32)
      (call_indirect (type $check)
        (if (result i32) (local.get 0)
          (then (call $dummy) (i32.const 1))
          (else (call $dummy) (i32.const 0))
        )
        (i32.const 2) (i32.const 0)
      )
    )
  )
  (func (export "as-call_indirect-mid") (param i32) (result i32)
    (block (result i32)
      (call_indirect (type $check)
        (i32.const 2)
        (if (result i32) (local.get 0)
          (then (call $dummy) (i32.const 1))
          (else (call $dummy) (i32.const 0))
        )
        (i32.const 0)
      )
    )
  )
  (func (export "as-call_indirect-last") (param i32) (result i32)
    (block (result i32)
      (call_indirect (type $check)
        (i32.const 2) (i32.const 0)
        (if (result i32) (local.get 0)
          (then (call $dummy) (i32.const 1))
          (else (call $dummy) (i32.const 0))
        )
      )
    )
  )

  (func (export "as-store-first") (param i32)
    (if (result i32) (local.get 0)
      (then (call $dummy) (i32.const 1))
      (else (call $dummy) (i32.const 0))
    )
    (i32.const 2)
    (i32.store)
  )
  (func (export "as-store-last") (param i32)
    (i32.const 2)
    (if (result i32) (local.get 0)
      (then (call $dummy) (i32.const 1))
      (else (call $dummy) (i32.const 0))
    )
    (i32.store)
  )

  (func (export "as-memory.grow-value") (param i32) (result i32)
    (memory.grow
      (if (result i32) (local.get 0)
        (then (i32.const 1))
        (else (i32.const 0))
      )
    )
  )

  (func $f (param i32) (result i32) (local.get 0))

  (func (export "as-call-value") (param i32) (result i32)
    (call $f
      (if (result i32) (local.get 0)
        (then (i32.const 1))
        (else (i32.const 0))
      )
    )
  )
  (func (export "as-return-value") (param i32) (result i32)
    (if (result i32) (local.get 0)
      (then (i32.const 1))
      (else (i32.const 0)))
    (return)
  )
  (func (export "as-drop-operand") (param i32)
    (drop
      (if (result i32) (local.get 0)
        (then (i32.const 1))
        (else (i32.const 0))
      )
    )
  )
  (func (export "as-br-value") (param i32) (result i32)
    (block (result i32)
      (br 0
        (if (result i32) (local.get 0)
          (then (i32.const 1))
          (else (i32.const 0))
        )
      )
    )
  )
  (func (export "as-local.set-value") (param i32) (result i32)
    (local i32)
    (local.set 0
      (if (result i32) (local.get 0)
        (then (i32.const 1))
        (else (i32.const 0))
      )
    )
    (local.get 0)
  )
  (func (export "as-local.tee-value") (param i32) (result i32)
    (local.tee 0
      (if (result i32) (local.get 0)
        (then (i32.const 1))
        (else (i32.const 0))
      )
    )
  )
  (global $a (mut i32) (i32.const 10))
  (func (export "as-global.set-value") (param i32) (result i32)
    (global.set $a
      (if (result i32) (local.get 0)
        (then (i32.const 1))
        (else (i32.const 0))
      )
    ) (global.get $a)
  )
  (func (export "as-load-operand") (param i32) (result i32)
    (i32.load
      (if (result i32) (local.get 0)
        (then (i32.const 11))
        (else (i32.const 10))
      )
    )
  )

  (func (export "as-unary-operand") (param i32) (result i32)
    (i32.ctz
      (if (result i32) (local.get 0)
        (then (call $dummy) (i32.const 13))
        (else (call $dummy) (i32.const -13))
      )
    )
  )
  (func (export "as-binary-operand") (param i32 i32) (result i32)
    (i32.mul
      (if (result i32) (local.get 0)
        (then (call $dummy) (i32.const 3))
        (else (call $dummy) (i32.const -3))
      )
      (if (result i32) (local.get 1)
        (then (call $dummy) (i32.const 4))
        (else (call $dummy) (i32.const -5))
      )
    )
  )
  (func (export "as-test-operand") (param i32) (result i32)
    (i32.eqz
      (if (result i32) (local.get 0)
        (then (call $dummy) (i32.const 13))
        (else (call $dummy) (i32.const 0))
      )
    )
  )
  (func (export "as-compare-operand") (param i32 i32) (result i32)
    (f32.gt
      (if (result f32) (local.get 0)
        (then (call $dummy) (f32.const 3))
        (else (call $dummy) (f32.const -3))
      )
      (if (result f32) (local.get 1)
        (then (call $dummy) (f32.const 4))
        (else (call $dummy) (f32.const -4))
      )
    )
  )
  (func (export "as-binary-operands") (param i32) (result i32)
    (i32.mul
      (if (result i32 i32) (local.get 0)
        (then (call $dummy) (i32.const 3) (call $dummy) (i32.const 4))
        (else (call $dummy) (i32.const 3) (call $dummy) (i32.const -4))
      )
    )
  )
  (func (export "as-compare-operands") (param i32) (result i32)
    (f32.gt
      (if (result f32 f32) (local.get 0)
        (then (call $dummy) (f32.const 3) (call $dummy) (f32.const 3))
        (else (call $dummy) (f32.const -2) (call $dummy) (f32.const -3))
      )
    )
  )
  (func (export "as-mixed-operands") (param i32) (result i32)
    (if (result i32 i32) (local.get 0)
      (then (call $dummy) (i32.const 3) (call $dummy) (i32.const 4))
      (else (call $dummy) (i32.const -3) (call $dummy) (i32.const -4))
    )
    (i32.const 5)
    (i32.add)
    (i32.mul)
  )

  (func (export "break-bare") (result i32)
    (if (i32.const 1) (then (br 0) (unreachable)))
    (if (i32.const 1) (then (br 0) (unreachable)) (else (unreachable)))
    (if (i32.const 0) (then (unreachable)) (else (br 0) (unreachable)))
    (if (i32.const 1) (then (br_if 0 (i32.const 1)) (unreachable)))
    (if (i32.const 1) (then (br_if 0 (i32.const 1)) (unreachable)) (else (unreachable)))
    (if (i32.const 0) (then (unreachable)) (else (br_if 0 (i32.const 1)) (unreachable)))
    (if (i32.const 1) (then (br_table 0 (i32.const 0)) (unreachable)))
    (if (i32.const 1) (then (br_table 0 (i32.const 0)) (unreachable)) (else (unreachable)))
    (if (i32.const 0) (then (unreachable)) (else (br_table 0 (i32.const 0)) (unreachable)))
    (i32.const 19)
  )

  (func (export "break-value") (param i32) (result i32)
    (if (result i32) (local.get 0)
      (then (br 0 (i32.const 18)) (i32.const 19))
      (else (br 0 (i32.const 21)) (i32.const 20))
    )
  )
  (func (export "break-multi-value") (param i32) (result i32 i32 i64)
    (if (result i32 i32 i64) (local.get 0)
      (then
        (br 0 (i32.const 18) (i32.const -18) (i64.const 18))
        (i32.const 19) (i32.const -19) (i64.const 19)
      )
      (else
        (br 0 (i32.const -18) (i32.const 18) (i64.const -18))
        (i32.const -19) (i32.const 19) (i64.const -19)
      )
    )
  )

  (func (export "param") (param i32) (result i32)
    (i32.const 1)
    (if (param i32) (result i32) (local.get 0)
      (then (i32.const 2) (i32.add))
      (else (i32.const -2) (i32.add))
    )
  )
  (func (export "params") (param i32) (result i32)
    (i32.const 1)
    (i32.const 2)
    (if (param i32 i32) (result i32) (local.get 0)
      (then (i32.add))
      (else (i32.sub))
    )
  )
  (func (export "params-id") (param i32) (result i32)
    (i32.const 1)
    (i32.const 2)
    (if (param i32 i32) (result i32 i32) (local.get 0) (then))
    (i32.add)
  )
  (func (export "param-break") (param i32) (result i32)
    (i32.const 1)
    (if (param i32) (result i32) (local.get 0)
      (then (i32.const 2) (i32.add) (br 0))
      (else (i32.const -2) (i32.add) (br 0))
    )
  )
  (func (export "params-break") (param i32) (result i32)
    (i32.const 1)
    (i32.const 2)
    (if (param i32 i32) (result i32) (local.get 0)
      (then (i32.add) (br 0))
      (else (i32.sub) (br 0))
    )
  )
  (func (export "params-id-break") (param i32) (result i32)
    (i32.const 1)
    (i32.const 2)
    (if (param i32 i32) (result i32 i32) (local.get 0) (then (br 0)))
    (i32.add)
  )

  (func (export "effects") (param i32) (result i32)
    (local i32)
    (if
      (block (result i32) (local.set 1 (i32.const 1)) (local.get 0))
      (then
        (local.set 1 (i32.mul (local.get 1) (i32.const 3)))
        (local.set 1 (i32.sub (local.get 1) (i32.const 5)))
        (local.set 1 (i32.mul (local.get 1) (i32.const 7)))
        (br 0)
        (local.set 1 (i32.mul (local.get 1) (i32.const 100)))
      )
      (else
        (local.set 1 (i32.mul (local.get 1) (i32.const 5)))
        (local.set 1 (i32.sub (local.get 1) (i32.const 7)))
        (local.set 1 (i32.mul (local.get 1) (i32.const 3)))
        (br 0)
        (local.set 1 (i32.mul (local.get 1) (i32.const 1000)))
      )
    )
    (local.get 1)
  )

  ;; Examples

  (func $add64_u_with_carry (export "add64_u_with_carry")
    (param $i i64) (param $j i64) (param $c i32) (result i64 i32)
    (local $k i64)
    (local.set $k
      (i64.add
        (i64.add (local.get $i) (local.get $j))
        (i64.extend_i32_u (local.get $c))
      )
    )
    (return (local.get $k) (i64.lt_u (local.get $k) (local.get $i)))
  )

  (func $add64_u_saturated (export "add64_u_saturated")
    (param i64 i64) (result i64)
    (call $add64_u_with_carry (local.get 0) (local.get 1) (i32.const 0))
    (if (param i64) (result i64)
      (then (drop) (i64.const -1))
    )
  )

  ;; Block signature syntax

  (type $block-sig-1 (func))
  (type $block-sig-2 (func (result i32)))
  (type $block-sig-3 (func (param $x i32)))
  (type $block-sig-4 (func (param i32 f64 i32) (result i32 f64 i32)))

  (func (export "type-use")
    (if (type $block-sig-1) (i32.const 1) (then))
    (if (type $block-sig-2) (i32.const 1)
      (then (i32.const 0)) (else (i32.const 2))
    )
    (if (type $block-sig-3) (i32.const 1) (then (drop)) (else (drop)))
    (i32.const 0) (f64.const 0) (i32.const 0)
    (if (type $block-sig-4) (i32.const 1) (then))
    (drop) (drop) (drop)
    (if (type $block-sig-2) (result i32) (i32.const 1)
      (then (i32.const 0)) (else (i32.const 2))
    )
    (if (type $block-sig-3) (param i32) (i32.const 1)
      (then (drop)) (else (drop))
    )
    (i32.const 0) (f64.const 0) (i32.const 0)
    (if (type $block-sig-4)
      (param i32) (param f64 i32) (result i32 f64) (result i32)
      (i32.const 1) (then)
    )
    (drop) (drop) (drop)
  )

  ;; Atypical folded condition syntax

  (func (export "atypical-condition")
    i32.const 0
    (if (then) (else))
    (if (i32.const 1) (i32.eqz) (then) (else))
  )
)
"#
    );
}

#[test]
fn imports_0() {
    assert_snapshot!(
        r#"
(module
  (func (export "func"))
  (func (export "func-i32") (param i32))
  (func (export "func-f32") (param f32))
  (func (export "func->i32") (result i32) (i32.const 22))
  (func (export "func->f32") (result f32) (f32.const 11))
  (func (export "func-i32->i32") (param i32) (result i32) (local.get 0))
  (func (export "func-i64->i64") (param i64) (result i64) (local.get 0))
  (global (export "global-i32") i32 (i32.const 55))
  (global (export "global-f32") f32 (f32.const 44))
  (global (export "global-mut-i64") (mut i64) (i64.const 66))
  (table (export "table-10-inf") 10 funcref)
  (table (export "table-10-20") 10 20 funcref)
  (memory (export "memory-2-inf") 2)
  ;; Multiple memories are not yet supported
  ;; (memory (export "memory-2-4") 2 4)
)
"#
    );
}

// TODO: wasm imported functions for call ops
// #[test]
fn imports_1() {
    assert_snapshot!(
        r#"
(module
  (type $func_i32 (func (param i32)))
  (type $func_i64 (func (param i64)))
  (type $func_f32 (func (param f32)))
  (type $func_f64 (func (param f64)))

  (import "spectest" "print_i32" (func (param i32)))
  (func (import "spectest" "print_i64") (param i64))
  (import "spectest" "print_i32" (func $print_i32 (param i32)))
  (import "spectest" "print_i64" (func $print_i64 (param i64)))
  (import "spectest" "print_f32" (func $print_f32 (param f32)))
  (import "spectest" "print_f64" (func $print_f64 (param f64)))
  (import "spectest" "print_i32_f32" (func $print_i32_f32 (param i32 f32)))
  (import "spectest" "print_f64_f64" (func $print_f64_f64 (param f64 f64)))
  (func $print_i32-2 (import "spectest" "print_i32") (param i32))
  (func $print_f64-2 (import "spectest" "print_f64") (param f64))
  (import "test" "func-i64->i64" (func $i64->i64 (param i64) (result i64)))

  (func (export "p1") (import "spectest" "print_i32") (param i32))
  (func $p (export "p2") (import "spectest" "print_i32") (param i32))
  (func (export "p3") (export "p4") (import "spectest" "print_i32") (param i32))
  (func (export "p5") (import "spectest" "print_i32") (type 0))
  (func (export "p6") (import "spectest" "print_i32") (type 0) (param i32) (result))

  (import "spectest" "print_i32" (func (type $forward)))
  (func (import "spectest" "print_i32") (type $forward))
  (type $forward (func (param i32)))

  (table funcref (elem $print_i32 $print_f64))

  (func (export "print32") (param $i i32)
    (local $x f32)
    (local.set $x (f32.convert_i32_s (local.get $i)))
    (call 0 (local.get $i))
    (call $print_i32_f32
      (i32.add (local.get $i) (i32.const 1))
      (f32.const 42)
    )
    (call $print_i32 (local.get $i))
    (call $print_i32-2 (local.get $i))
    (call $print_f32 (local.get $x))
    (call_indirect (type $func_i32) (local.get $i) (i32.const 0))
  )

  (func (export "print64") (param $i i64)
    (local $x f64)
    (local.set $x (f64.convert_i64_s (call $i64->i64 (local.get $i))))
    (call 1 (local.get $i))
    (call $print_f64_f64
      (f64.add (local.get $x) (f64.const 1))
      (f64.const 53)
    )
    (call $print_i64 (local.get $i))
    (call $print_f64 (local.get $x))
    (call $print_f64-2 (local.get $x))
    (call_indirect (type $func_f64) (local.get $x) (i32.const 1))
  )
)
"#
    );
}

#[test]
fn imports_2() {
    assert_snapshot!(
        r#"
(module
  (import "spectest" "global_i32" (global i32))
  (global (import "spectest" "global_i32") i32)

  (import "spectest" "global_i32" (global $x i32))
  (global $y (import "spectest" "global_i32") i32)

  (import "spectest" "global_i64" (global i64))
  (import "spectest" "global_f32" (global f32))
  (import "spectest" "global_f64" (global f64))

  (func (export "get-0") (result i32) (global.get 0))
  (func (export "get-1") (result i32) (global.get 1))
  (func (export "get-x") (result i32) (global.get $x))
  (func (export "get-y") (result i32) (global.get $y))
  (func (export "get-4") (result i64) (global.get 4))
  (func (export "get-5") (result f32) (global.get 5))
  (func (export "get-6") (result f64) (global.get 6))
)
"#
    );
}

#[test]
fn label() {
    assert_snapshot!(
        r#"
(module
  (func (export "block") (result i32)
    (block $exit (result i32)
      (br $exit (i32.const 1))
      (i32.const 0)
    )
  )

  (func (export "loop1") (result i32)
    (local $i i32)
    (local.set $i (i32.const 0))
    (block $exit (result i32)
      (loop $cont (result i32)
        (local.set $i (i32.add (local.get $i) (i32.const 1)))
        (if (i32.eq (local.get $i) (i32.const 5))
          (then (br $exit (local.get $i)))
        )
        (br $cont)
      )
    )
  )

  (func (export "loop2") (result i32)
    (local $i i32)
    (local.set $i (i32.const 0))
    (block $exit (result i32)
      (loop $cont (result i32)
        (local.set $i (i32.add (local.get $i) (i32.const 1)))
        (if (i32.eq (local.get $i) (i32.const 5))
          (then (br $cont))
        )
        (if (i32.eq (local.get $i) (i32.const 8))
          (then (br $exit (local.get $i)))
        )
        (local.set $i (i32.add (local.get $i) (i32.const 1)))
        (br $cont)
      )
    )
  )

  (func (export "loop3") (result i32)
    (local $i i32)
    (local.set $i (i32.const 0))
    (block $exit (result i32)
      (loop $cont (result i32)
        (local.set $i (i32.add (local.get $i) (i32.const 1)))
        (if (i32.eq (local.get $i) (i32.const 5))
          (then (br $exit (local.get $i)))
        )
        (local.get $i)
      )
    )
  )

  (func (export "loop4") (param $max i32) (result i32)
    (local $i i32)
    (local.set $i (i32.const 1))
    (block $exit (result i32)
      (loop $cont (result i32)
        (local.set $i (i32.add (local.get $i) (local.get $i)))
        (if (i32.gt_u (local.get $i) (local.get $max))
          (then (br $exit (local.get $i)))
        )
        (br $cont)
      )
    )
  )

  (func (export "loop5") (result i32)
    (i32.add
      (loop $l (result i32) (i32.const 1))
      (i32.const 1)
    )
  )

  (func (export "loop6") (result i32)
    (loop (result i32)
      (br_if 0 (i32.const 0))
      (i32.const 3)
    )
  )

  (func (export "if") (result i32)
    (local $i i32)
    (local.set $i (i32.const 0))
    (block
      (if $l
        (i32.const 1)
        (then (br $l) (local.set $i (i32.const 666)))
      )
      (local.set $i (i32.add (local.get $i) (i32.const 1)))
      (if $l
        (i32.const 1)
        (then (br $l) (local.set $i (i32.const 666)))
        (else (local.set $i (i32.const 888)))
      )
      (local.set $i (i32.add (local.get $i) (i32.const 1)))
      (if $l
        (i32.const 1)
        (then (br $l) (local.set $i (i32.const 666)))
        (else (local.set $i (i32.const 888)))
      )
      (local.set $i (i32.add (local.get $i) (i32.const 1)))
      (if $l
        (i32.const 0)
        (then (local.set $i (i32.const 888)))
        (else (br $l) (local.set $i (i32.const 666)))
      )
      (local.set $i (i32.add (local.get $i) (i32.const 1)))
      (if $l
        (i32.const 0)
        (then (local.set $i (i32.const 888)))
        (else (br $l) (local.set $i (i32.const 666)))
      )
      (local.set $i (i32.add (local.get $i) (i32.const 1)))
    )
    (local.get $i)
  )

  (func (export "if2") (result i32)
    (local $i i32)
    (local.set $i (i32.const 0))
    (block
      (if
        (i32.const 1)
        (then (br 0) (local.set $i (i32.const 666)))
      )
      (local.set $i (i32.add (local.get $i) (i32.const 1)))
      (if
        (i32.const 1)
        (then (br 0) (local.set $i (i32.const 666)))
        (else (local.set $i (i32.const 888)))
      )
      (local.set $i (i32.add (local.get $i) (i32.const 1)))
      (if
        (i32.const 1)
        (then (br 0) (local.set $i (i32.const 666)))
        (else (local.set $i (i32.const 888)))
      )
      (local.set $i (i32.add (local.get $i) (i32.const 1)))
      (if
        (i32.const 0)
        (then (local.set $i (i32.const 888)))
        (else (br 0) (local.set $i (i32.const 666)))
      )
      (local.set $i (i32.add (local.get $i) (i32.const 1)))
      (if
        (i32.const 0)
        (then (local.set $i (i32.const 888)))
        (else (br 0) (local.set $i (i32.const 666)))
      )
      (local.set $i (i32.add (local.get $i) (i32.const 1)))
    )
    (local.get $i)
  )

  (func (export "switch") (param i32) (result i32)
    (block $ret (result i32)
      (i32.mul (i32.const 10)
        (block $exit (result i32)
          (block $0
            (block $default
              (block $3
                (block $2
                  (block $1
                    (br_table $0 $1 $2 $3 $default (local.get 0))
                  ) ;; 1
                ) ;; 2
                (br $exit (i32.const 2))
              ) ;; 3
              (br $ret (i32.const 3))
            ) ;; default
          ) ;; 0
          (i32.const 5)
        )
      )
    )
  )

  (func (export "return") (param i32) (result i32)
    (block $default
      (block $1
        (block $0
          (br_table $0 $1 (local.get 0))
          (br $default)
        ) ;; 0
        (return (i32.const 0))
      ) ;; 1
    ) ;; default
    (i32.const 2)
  )

  (func (export "br_if0") (result i32)
    (local $i i32)
    (local.set $i (i32.const 0))
    (block $outer (result i32)
      (block $inner
        (br_if $inner (i32.const 0))
        (local.set $i (i32.or (local.get $i) (i32.const 0x1)))
        (br_if $inner (i32.const 1))
        (local.set $i (i32.or (local.get $i) (i32.const 0x2)))
      )
      (drop (br_if $outer
        (block (result i32)
          (local.set $i (i32.or (local.get $i) (i32.const 0x4)))
          (local.get $i)
        )
        (i32.const 0)
      ))
      (local.set $i (i32.or (local.get $i) (i32.const 0x8)))
      (drop (br_if $outer
        (block (result i32)
          (local.set $i (i32.or (local.get $i) (i32.const 0x10)))
          (local.get $i)
        )
        (i32.const 1)
      ))
      (local.set $i (i32.or (local.get $i) (i32.const 0x20))) (local.get $i)
    )
  )

  (func (export "br_if1") (result i32)
    (block $l0 (result i32)
      (drop
        (br_if $l0
          (block $l1 (result i32) (br $l1 (i32.const 1)))
          (i32.const 1)
        )
      )
      (i32.const 0)
    )
  )

  (func (export "br_if2") (result i32)
    (block $l0 (result i32)
      (if (i32.const 1)
        (then
          (drop
            (br_if $l0
              (block $l1 (result i32) (br $l1 (i32.const 1)))
              (i32.const 1)
            )
          )
        )
      )
      (i32.const 0)
    )
  )

  (func (export "br_if3") (result i32)
    (local $i1 i32)
    (drop
      (i32.add
        (block $l0 (result i32)
          (drop (br_if $l0
            (block (result i32) (local.set $i1 (i32.const 1)) (local.get $i1))
            (block (result i32) (local.set $i1 (i32.const 2)) (local.get $i1))
          ))
          (i32.const 0)
        )
        (i32.const 0)
      )
    )
    (local.get $i1)
  )

  (func (export "br") (result i32)
    (block $l0 (result i32)
      (if (i32.const 1)
        (then (br $l0 (block $l1 (result i32) (br $l1 (i32.const 1)))))
        (else (block (drop (block $l1 (result i32) (br $l1 (i32.const 1))))))
      )
      (i32.const 1)
    )
  )

  (func (export "shadowing") (result i32)
    (block $l1 (result i32) (i32.xor (br $l1 (i32.const 1)) (i32.const 2)))
  )

  (func (export "redefinition") (result i32)
    (block $l1 (result i32)
      (i32.add
        (block $l1 (result i32) (i32.const 2))
        (block $l1 (result i32) (br $l1 (i32.const 3)))
      )
    )
  )
)
"#
    );
}

#[test]
fn left_to_right() {
    assert_snapshot!(
        r#"
(module
  (memory 1)

  (type $i32_T (func (param i32 i32) (result i32)))
  (type $i64_T (func (param i64 i64) (result i32)))
  (type $f32_T (func (param f32 f32) (result i32)))
  (type $f64_T (func (param f64 f64) (result i32)))
  (table funcref
    (elem $i32_t0 $i32_t1 $i64_t0 $i64_t1 $f32_t0 $f32_t1 $f64_t0 $f64_t1)
  )

  (func $i32_t0 (type $i32_T) (i32.const -1))
  (func $i32_t1 (type $i32_T) (i32.const -2))
  (func $i64_t0 (type $i64_T) (i32.const -1))
  (func $i64_t1 (type $i64_T) (i32.const -2))
  (func $f32_t0 (type $f32_T) (i32.const -1))
  (func $f32_t1 (type $f32_T) (i32.const -2))
  (func $f64_t0 (type $f64_T) (i32.const -1))
  (func $f64_t1 (type $f64_T) (i32.const -2))

  ;; The idea is: We reset the memory, then the instruction call $*_left,
  ;; $*_right, $*_another, $*_callee (for indirect calls), and $*_bool (when a
  ;; boolean value is needed). These functions all call bump, which shifts the
  ;; memory starting at address 8 up a byte, and then store a unique value at
  ;; address 8. Then we read the 4-byte value at address 8. It should contain
  ;; the correct sequence of unique values if the calls were evaluated in the
  ;; correct order.

  (func $reset (i32.store (i32.const 8) (i32.const 0)))

  (func $bump
    (i32.store8 (i32.const 11) (i32.load8_u (i32.const 10)))
    (i32.store8 (i32.const 10) (i32.load8_u (i32.const 9)))
    (i32.store8 (i32.const 9) (i32.load8_u (i32.const 8)))
    (i32.store8 (i32.const 8) (i32.const -3)))

  (func $get (result i32) (i32.load (i32.const 8)))

  (func $i32_left (result i32) (call $bump) (i32.store8 (i32.const 8) (i32.const 1)) (i32.const 0))
  (func $i32_right (result i32) (call $bump) (i32.store8 (i32.const 8) (i32.const 2)) (i32.const 1))
  (func $i32_another (result i32) (call $bump) (i32.store8 (i32.const 8) (i32.const 3)) (i32.const 1))
  (func $i32_callee (result i32) (call $bump) (i32.store8 (i32.const 8) (i32.const 4)) (i32.const 0))
  (func $i32_bool (result i32) (call $bump) (i32.store8 (i32.const 8) (i32.const 5)) (i32.const 0))
  (func $i64_left (result i64) (call $bump) (i32.store8 (i32.const 8) (i32.const 1)) (i64.const 0))
  (func $i64_right (result i64) (call $bump) (i32.store8 (i32.const 8) (i32.const 2)) (i64.const 1))
  (func $i64_another (result i64) (call $bump) (i32.store8 (i32.const 8) (i32.const 3)) (i64.const 1))
  (func $i64_callee (result i32) (call $bump) (i32.store8 (i32.const 8) (i32.const 4)) (i32.const 2))
  (func $i64_bool (result i32) (call $bump) (i32.store8 (i32.const 8) (i32.const 5)) (i32.const 0))
  (func $f32_left (result f32) (call $bump) (i32.store8 (i32.const 8) (i32.const 1)) (f32.const 0))
  (func $f32_right (result f32) (call $bump) (i32.store8 (i32.const 8) (i32.const 2)) (f32.const 1))
  (func $f32_another (result f32) (call $bump) (i32.store8 (i32.const 8) (i32.const 3)) (f32.const 1))
  (func $f32_callee (result i32) (call $bump) (i32.store8 (i32.const 8) (i32.const 4)) (i32.const 4))
  (func $f32_bool (result i32) (call $bump) (i32.store8 (i32.const 8) (i32.const 5)) (i32.const 0))
  (func $f64_left (result f64) (call $bump) (i32.store8 (i32.const 8) (i32.const 1)) (f64.const 0))
  (func $f64_right (result f64) (call $bump) (i32.store8 (i32.const 8) (i32.const 2)) (f64.const 1))
  (func $f64_another (result f64) (call $bump) (i32.store8 (i32.const 8) (i32.const 3)) (f64.const 1))
  (func $f64_callee (result i32) (call $bump) (i32.store8 (i32.const 8) (i32.const 4)) (i32.const 6))
  (func $f64_bool (result i32) (call $bump) (i32.store8 (i32.const 8) (i32.const 5)) (i32.const 0))
  (func $i32_dummy (param i32 i32))
  (func $i64_dummy (param i64 i64))
  (func $f32_dummy (param f32 f32))
  (func $f64_dummy (param f64 f64))

  (func (export "i32_add") (result i32) (call $reset) (drop (i32.add (call $i32_left) (call $i32_right))) (call $get))
  (func (export "i32_sub") (result i32) (call $reset) (drop (i32.sub (call $i32_left) (call $i32_right))) (call $get))
  (func (export "i32_mul") (result i32) (call $reset) (drop (i32.mul (call $i32_left) (call $i32_right))) (call $get))
  (func (export "i32_div_s") (result i32) (call $reset) (drop (i32.div_s (call $i32_left) (call $i32_right))) (call $get))
  (func (export "i32_div_u") (result i32) (call $reset) (drop (i32.div_u (call $i32_left) (call $i32_right))) (call $get))
  (func (export "i32_rem_s") (result i32) (call $reset) (drop (i32.rem_s (call $i32_left) (call $i32_right))) (call $get))
  (func (export "i32_rem_u") (result i32) (call $reset) (drop (i32.rem_u (call $i32_left) (call $i32_right))) (call $get))
  (func (export "i32_and") (result i32) (call $reset) (drop (i32.and (call $i32_left) (call $i32_right))) (call $get))
  (func (export "i32_or") (result i32) (call $reset) (drop (i32.or (call $i32_left) (call $i32_right))) (call $get))
  (func (export "i32_xor") (result i32) (call $reset) (drop (i32.xor (call $i32_left) (call $i32_right))) (call $get))
  (func (export "i32_shl") (result i32) (call $reset) (drop (i32.shl (call $i32_left) (call $i32_right))) (call $get))
  (func (export "i32_shr_u") (result i32) (call $reset) (drop (i32.shr_u (call $i32_left) (call $i32_right))) (call $get))
  (func (export "i32_shr_s") (result i32) (call $reset) (drop (i32.shr_s (call $i32_left) (call $i32_right))) (call $get))
  (func (export "i32_eq") (result i32) (call $reset) (drop (i32.eq (call $i32_left) (call $i32_right))) (call $get))
  (func (export "i32_ne") (result i32) (call $reset) (drop (i32.ne (call $i32_left) (call $i32_right))) (call $get))
  (func (export "i32_lt_s") (result i32) (call $reset) (drop (i32.lt_s (call $i32_left) (call $i32_right))) (call $get))
  (func (export "i32_le_s") (result i32) (call $reset) (drop (i32.le_s (call $i32_left) (call $i32_right))) (call $get))
  (func (export "i32_lt_u") (result i32) (call $reset) (drop (i32.lt_u (call $i32_left) (call $i32_right))) (call $get))
  (func (export "i32_le_u") (result i32) (call $reset) (drop (i32.le_u (call $i32_left) (call $i32_right))) (call $get))
  (func (export "i32_gt_s") (result i32) (call $reset) (drop (i32.gt_s (call $i32_left) (call $i32_right))) (call $get))
  (func (export "i32_ge_s") (result i32) (call $reset) (drop (i32.ge_s (call $i32_left) (call $i32_right))) (call $get))
  (func (export "i32_gt_u") (result i32) (call $reset) (drop (i32.gt_u (call $i32_left) (call $i32_right))) (call $get))
  (func (export "i32_ge_u") (result i32) (call $reset) (drop (i32.ge_u (call $i32_left) (call $i32_right))) (call $get))
  (func (export "i32_store") (result i32) (call $reset) (i32.store (call $i32_left) (call $i32_right)) (call $get))
  (func (export "i32_store8") (result i32) (call $reset) (i32.store8 (call $i32_left) (call $i32_right)) (call $get))
  (func (export "i32_store16") (result i32) (call $reset) (i32.store16 (call $i32_left) (call $i32_right)) (call $get))
  (func (export "i32_call") (result i32) (call $reset) (call $i32_dummy (call $i32_left) (call $i32_right)) (call $get))
  (func (export "i32_call_indirect") (result i32) (call $reset) (drop (call_indirect (type $i32_T) (call $i32_left) (call $i32_right) (call $i32_callee))) (call $get))
  (func (export "i32_select") (result i32) (call $reset) (drop (select (call $i32_left) (call $i32_right) (call $i32_bool))) (call $get))

  (func (export "i64_add") (result i32) (call $reset) (drop (i64.add (call $i64_left) (call $i64_right))) (call $get))
  (func (export "i64_sub") (result i32) (call $reset) (drop (i64.sub (call $i64_left) (call $i64_right))) (call $get))
  (func (export "i64_mul") (result i32) (call $reset) (drop (i64.mul (call $i64_left) (call $i64_right))) (call $get))
  (func (export "i64_div_s") (result i32) (call $reset) (drop (i64.div_s (call $i64_left) (call $i64_right))) (call $get))
  (func (export "i64_div_u") (result i32) (call $reset) (drop (i64.div_u (call $i64_left) (call $i64_right))) (call $get))
  (func (export "i64_rem_s") (result i32) (call $reset) (drop (i64.rem_s (call $i64_left) (call $i64_right))) (call $get))
  (func (export "i64_rem_u") (result i32) (call $reset) (drop (i64.rem_u (call $i64_left) (call $i64_right))) (call $get))
  (func (export "i64_and") (result i32) (call $reset) (drop (i64.and (call $i64_left) (call $i64_right))) (call $get))
  (func (export "i64_or") (result i32) (call $reset) (drop (i64.or (call $i64_left) (call $i64_right))) (call $get))
  (func (export "i64_xor") (result i32) (call $reset) (drop (i64.xor (call $i64_left) (call $i64_right))) (call $get))
  (func (export "i64_shl") (result i32) (call $reset) (drop (i64.shl (call $i64_left) (call $i64_right))) (call $get))
  (func (export "i64_shr_u") (result i32) (call $reset) (drop (i64.shr_u (call $i64_left) (call $i64_right))) (call $get))
  (func (export "i64_shr_s") (result i32) (call $reset) (drop (i64.shr_s (call $i64_left) (call $i64_right))) (call $get))
  (func (export "i64_eq") (result i32) (call $reset) (drop (i64.eq (call $i64_left) (call $i64_right))) (call $get))
  (func (export "i64_ne") (result i32) (call $reset) (drop (i64.ne (call $i64_left) (call $i64_right))) (call $get))
  (func (export "i64_lt_s") (result i32) (call $reset) (drop (i64.lt_s (call $i64_left) (call $i64_right))) (call $get))
  (func (export "i64_le_s") (result i32) (call $reset) (drop (i64.le_s (call $i64_left) (call $i64_right))) (call $get))
  (func (export "i64_lt_u") (result i32) (call $reset) (drop (i64.lt_u (call $i64_left) (call $i64_right))) (call $get))
  (func (export "i64_le_u") (result i32) (call $reset) (drop (i64.le_u (call $i64_left) (call $i64_right))) (call $get))
  (func (export "i64_gt_s") (result i32) (call $reset) (drop (i64.gt_s (call $i64_left) (call $i64_right))) (call $get))
  (func (export "i64_ge_s") (result i32) (call $reset) (drop (i64.ge_s (call $i64_left) (call $i64_right))) (call $get))
  (func (export "i64_gt_u") (result i32) (call $reset) (drop (i64.gt_u (call $i64_left) (call $i64_right))) (call $get))
  (func (export "i64_ge_u") (result i32) (call $reset) (drop (i64.ge_u (call $i64_left) (call $i64_right))) (call $get))
  (func (export "i64_store") (result i32) (call $reset) (i64.store (call $i32_left) (call $i64_right)) (call $get))
  (func (export "i64_store8") (result i32) (call $reset) (i64.store8 (call $i32_left) (call $i64_right)) (call $get))
  (func (export "i64_store16") (result i32) (call $reset) (i64.store16 (call $i32_left) (call $i64_right)) (call $get))
  (func (export "i64_store32") (result i32) (call $reset) (i64.store32 (call $i32_left) (call $i64_right)) (call $get))
  (func (export "i64_call") (result i32) (call $reset) (call $i64_dummy (call $i64_left) (call $i64_right)) (call $get))
  (func (export "i64_call_indirect") (result i32) (call $reset) (drop (call_indirect (type $i64_T) (call $i64_left) (call $i64_right) (call $i64_callee))) (call $get))
  (func (export "i64_select") (result i32) (call $reset) (drop (select (call $i64_left) (call $i64_right) (call $i64_bool))) (call $get))

  (func (export "f32_add") (result i32) (call $reset) (drop (f32.add (call $f32_left) (call $f32_right))) (call $get))
  (func (export "f32_sub") (result i32) (call $reset) (drop (f32.sub (call $f32_left) (call $f32_right))) (call $get))
  (func (export "f32_mul") (result i32) (call $reset) (drop (f32.mul (call $f32_left) (call $f32_right))) (call $get))
  (func (export "f32_div") (result i32) (call $reset) (drop (f32.div (call $f32_left) (call $f32_right))) (call $get))
  (func (export "f32_copysign") (result i32) (call $reset) (drop (f32.copysign (call $f32_left) (call $f32_right))) (call $get))
  (func (export "f32_eq") (result i32) (call $reset) (drop (f32.eq (call $f32_left) (call $f32_right))) (call $get))
  (func (export "f32_ne") (result i32) (call $reset) (drop (f32.ne (call $f32_left) (call $f32_right))) (call $get))
  (func (export "f32_lt") (result i32) (call $reset) (drop (f32.lt (call $f32_left) (call $f32_right))) (call $get))
  (func (export "f32_le") (result i32) (call $reset) (drop (f32.le (call $f32_left) (call $f32_right))) (call $get))
  (func (export "f32_gt") (result i32) (call $reset) (drop (f32.gt (call $f32_left) (call $f32_right))) (call $get))
  (func (export "f32_ge") (result i32) (call $reset) (drop (f32.ge (call $f32_left) (call $f32_right))) (call $get))
  (func (export "f32_min") (result i32) (call $reset) (drop (f32.min (call $f32_left) (call $f32_right))) (call $get))
  (func (export "f32_max") (result i32) (call $reset) (drop (f32.max (call $f32_left) (call $f32_right))) (call $get))
  (func (export "f32_store") (result i32) (call $reset) (f32.store (call $i32_left) (call $f32_right)) (call $get))
  (func (export "f32_call") (result i32) (call $reset) (call $f32_dummy (call $f32_left) (call $f32_right)) (call $get))
  (func (export "f32_call_indirect") (result i32) (call $reset) (drop (call_indirect (type $f32_T) (call $f32_left) (call $f32_right) (call $f32_callee))) (call $get))
  (func (export "f32_select") (result i32) (call $reset) (drop (select (call $f32_left) (call $f32_right) (call $f32_bool))) (call $get))

  (func (export "f64_add") (result i32) (call $reset) (drop (f64.add (call $f64_left) (call $f64_right))) (call $get))
  (func (export "f64_sub") (result i32) (call $reset) (drop (f64.sub (call $f64_left) (call $f64_right))) (call $get))
  (func (export "f64_mul") (result i32) (call $reset) (drop (f64.mul (call $f64_left) (call $f64_right))) (call $get))
  (func (export "f64_div") (result i32) (call $reset) (drop (f64.div (call $f64_left) (call $f64_right))) (call $get))
  (func (export "f64_copysign") (result i32) (call $reset) (drop (f64.copysign (call $f64_left) (call $f64_right))) (call $get))
  (func (export "f64_eq") (result i32) (call $reset) (drop (f64.eq (call $f64_left) (call $f64_right))) (call $get))
  (func (export "f64_ne") (result i32) (call $reset) (drop (f64.ne (call $f64_left) (call $f64_right))) (call $get))
  (func (export "f64_lt") (result i32) (call $reset) (drop (f64.lt (call $f64_left) (call $f64_right))) (call $get))
  (func (export "f64_le") (result i32) (call $reset) (drop (f64.le (call $f64_left) (call $f64_right))) (call $get))
  (func (export "f64_gt") (result i32) (call $reset) (drop (f64.gt (call $f64_left) (call $f64_right))) (call $get))
  (func (export "f64_ge") (result i32) (call $reset) (drop (f64.ge (call $f64_left) (call $f64_right))) (call $get))
  (func (export "f64_min") (result i32) (call $reset) (drop (f64.min (call $f64_left) (call $f64_right))) (call $get))
  (func (export "f64_max") (result i32) (call $reset) (drop (f64.max (call $f64_left) (call $f64_right))) (call $get))
  (func (export "f64_store") (result i32) (call $reset) (f64.store (call $i32_left) (call $f64_right)) (call $get))
  (func (export "f64_call") (result i32) (call $reset) (call $f64_dummy (call $f64_left) (call $f64_right)) (call $get))
  (func (export "f64_call_indirect") (result i32) (call $reset) (drop (call_indirect (type $f64_T) (call $f64_left) (call $f64_right) (call $f64_callee))) (call $get))
  (func (export "f64_select") (result i32) (call $reset) (drop (select (call $f64_left) (call $f64_right) (call $f64_bool))) (call $get))

  (func (export "br_if") (result i32)
    (block (result i32)
      (call $reset)
      (drop (br_if 0 (call $i32_left) (i32.and (call $i32_right) (i32.const 0))))
      (call $get)
    )
  )
  (func (export "br_table") (result i32)
    (block $a (result i32)
      (call $reset)
      (drop
        (block $b (result i32)
          (br_table $a $b (call $i32_left) (call $i32_right))
        )
      )
      (call $get)
    )
  )
)
"#
    );
}

#[test]
fn load() {
    assert_snapshot!(
        r#"
(module
  (memory 1)

  (func (export "as-br-value") (result i32)
    (block (result i32) (br 0 (i32.load (i32.const 0))))
  )

  (func (export "as-br_if-cond")
    (block (br_if 0 (i32.load (i32.const 0))))
  )
  (func (export "as-br_if-value") (result i32)
    (block (result i32)
      (drop (br_if 0 (i32.load (i32.const 0)) (i32.const 1))) (i32.const 7)
    )
  )
  (func (export "as-br_if-value-cond") (result i32)
    (block (result i32)
      (drop (br_if 0 (i32.const 6) (i32.load (i32.const 0)))) (i32.const 7)
    )
  )

  (func (export "as-br_table-index")
    (block (br_table 0 0 0 (i32.load (i32.const 0))))
  )
  (func (export "as-br_table-value") (result i32)
    (block (result i32)
      (br_table 0 0 0 (i32.load (i32.const 0)) (i32.const 1)) (i32.const 7)
    )
  )
  (func (export "as-br_table-value-index") (result i32)
    (block (result i32)
      (br_table 0 0 (i32.const 6) (i32.load (i32.const 0))) (i32.const 7)
    )
  )

  (func (export "as-return-value") (result i32)
    (return (i32.load (i32.const 0)))
  )

  (func (export "as-if-cond") (result i32)
    (if (result i32) (i32.load (i32.const 0))
      (then (i32.const 0)) (else (i32.const 1))
    )
  )
  (func (export "as-if-then") (result i32)
    (if (result i32) (i32.const 1)
      (then (i32.load (i32.const 0))) (else (i32.const 0))
    )
  )
  (func (export "as-if-else") (result i32)
    (if (result i32) (i32.const 0)
      (then (i32.const 0)) (else (i32.load (i32.const 0)))
    )
  )

  (func (export "as-select-first") (param i32 i32) (result i32)
    (select (i32.load (i32.const 0)) (local.get 0) (local.get 1))
  )
  (func (export "as-select-second") (param i32 i32) (result i32)
    (select (local.get 0) (i32.load (i32.const 0)) (local.get 1))
  )
  (func (export "as-select-cond") (result i32)
    (select (i32.const 0) (i32.const 1) (i32.load (i32.const 0)))
  )

  (func $f (param i32 i32 i32) (result i32) (i32.const -1))
  (func (export "as-call-first") (result i32)
    (call $f (i32.load (i32.const 0)) (i32.const 2) (i32.const 3))
  )
  (func (export "as-call-mid") (result i32)
    (call $f (i32.const 1) (i32.load (i32.const 0)) (i32.const 3))
  )
  (func (export "as-call-last") (result i32)
    (call $f (i32.const 1) (i32.const 2) (i32.load (i32.const 0)))
  )

  (type $sig (func (param i32 i32 i32) (result i32)))
  (table funcref (elem $f))
  (func (export "as-call_indirect-first") (result i32)
    (call_indirect (type $sig)
      (i32.load (i32.const 0)) (i32.const 2) (i32.const 3) (i32.const 0)
    )
  )
  (func (export "as-call_indirect-mid") (result i32)
    (call_indirect (type $sig)
      (i32.const 1) (i32.load (i32.const 0)) (i32.const 3) (i32.const 0)
    )
  )
  (func (export "as-call_indirect-last") (result i32)
    (call_indirect (type $sig)
      (i32.const 1) (i32.const 2) (i32.load (i32.const 0)) (i32.const 0)
    )
  )
  (func (export "as-call_indirect-index") (result i32)
    (call_indirect (type $sig)
      (i32.const 1) (i32.const 2) (i32.const 3) (i32.load (i32.const 0))
    )
  )

  (func (export "as-local.set-value") (local i32)
    (local.set 0 (i32.load (i32.const 0)))
  )
  (func (export "as-local.tee-value") (result i32) (local i32)
    (local.tee 0 (i32.load (i32.const 0)))
  )
  (global $g (mut i32) (i32.const 0))
  (func (export "as-global.set-value") (local i32)
    (global.set $g (i32.load (i32.const 0)))
  )

  (func (export "as-load-address") (result i32)
    (i32.load (i32.load (i32.const 0)))
  )
  (func (export "as-loadN-address") (result i32)
    (i32.load8_s (i32.load (i32.const 0)))
  )

  (func (export "as-store-address")
    (i32.store (i32.load (i32.const 0)) (i32.const 7))
  )
  (func (export "as-store-value")
    (i32.store (i32.const 2) (i32.load (i32.const 0)))
  )

  (func (export "as-storeN-address")
    (i32.store8 (i32.load8_s (i32.const 0)) (i32.const 7))
  )
  (func (export "as-storeN-value")
    (i32.store16 (i32.const 2) (i32.load (i32.const 0)))
  )

  (func (export "as-unary-operand") (result i32)
    (i32.clz (i32.load (i32.const 100)))
  )

  (func (export "as-binary-left") (result i32)
    (i32.add (i32.load (i32.const 100)) (i32.const 10))
  )
  (func (export "as-binary-right") (result i32)
    (i32.sub (i32.const 10) (i32.load (i32.const 100)))
  )

  (func (export "as-test-operand") (result i32)
    (i32.eqz (i32.load (i32.const 100)))
  )

  (func (export "as-compare-left") (result i32)
    (i32.le_s (i32.load (i32.const 100)) (i32.const 10))
  )
  (func (export "as-compare-right") (result i32)
    (i32.ne (i32.const 10) (i32.load (i32.const 100)))
  )

  (func (export "as-memory.grow-size") (result i32)
    (memory.grow (i32.load (i32.const 100)))
  )
)
"#
    );
}

#[test]
fn local_tee() {
    assert_snapshot!(
        r#"
(module
  ;; Typing

  (func (export "type-local-i32") (result i32) (local i32) (local.tee 0 (i32.const 0)))
  (func (export "type-local-i64") (result i64) (local i64) (local.tee 0 (i64.const 0)))
  (func (export "type-local-f32") (result f32) (local f32) (local.tee 0 (f32.const 0)))
  (func (export "type-local-f64") (result f64) (local f64) (local.tee 0 (f64.const 0)))

  (func (export "type-param-i32") (param i32) (result i32) (local.tee 0 (i32.const 10)))
  (func (export "type-param-i64") (param i64) (result i64) (local.tee 0 (i64.const 11)))
  (func (export "type-param-f32") (param f32) (result f32) (local.tee 0 (f32.const 11.1)))
  (func (export "type-param-f64") (param f64) (result f64) (local.tee 0 (f64.const 12.2)))

  (func (export "type-mixed") (param i64 f32 f64 i32 i32) (local f32 i64 i64 f64)
    (drop (i64.eqz (local.tee 0 (i64.const 0))))
    (drop (f32.neg (local.tee 1 (f32.const 0))))
    (drop (f64.neg (local.tee 2 (f64.const 0))))
    (drop (i32.eqz (local.tee 3 (i32.const 0))))
    (drop (i32.eqz (local.tee 4 (i32.const 0))))
    (drop (f32.neg (local.tee 5 (f32.const 0))))
    (drop (i64.eqz (local.tee 6 (i64.const 0))))
    (drop (i64.eqz (local.tee 7 (i64.const 0))))
    (drop (f64.neg (local.tee 8 (f64.const 0))))
  )

  ;; Writing

  (func (export "write") (param i64 f32 f64 i32 i32) (result i64) (local f32 i64 i64 f64)
    (drop (local.tee 1 (f32.const -0.3)))
    (drop (local.tee 3 (i32.const 40)))
    (drop (local.tee 4 (i32.const -7)))
    (drop (local.tee 5 (f32.const 5.5)))
    (drop (local.tee 6 (i64.const 6)))
    (drop (local.tee 8 (f64.const 8)))
    (i64.trunc_f64_s
      (f64.add
        (f64.convert_i64_u (local.get 0))
        (f64.add
          (f64.promote_f32 (local.get 1))
          (f64.add
            (local.get 2)
            (f64.add
              (f64.convert_i32_u (local.get 3))
              (f64.add
                (f64.convert_i32_s (local.get 4))
                (f64.add
                  (f64.promote_f32 (local.get 5))
                  (f64.add
                    (f64.convert_i64_u (local.get 6))
                    (f64.add
                      (f64.convert_i64_u (local.get 7))
                      (local.get 8)
                    )
                  )
                )
              )
            )
          )
        )
      )
    )
  )

  ;; Result

  (func (export "result") (param i64 f32 f64 i32 i32) (result f64)
    (local f32 i64 i64 f64)
    (f64.add
      (f64.convert_i64_u (local.tee 0 (i64.const 1)))
      (f64.add
        (f64.promote_f32 (local.tee 1 (f32.const 2)))
        (f64.add
          (local.tee 2 (f64.const 3.3))
          (f64.add
            (f64.convert_i32_u (local.tee 3 (i32.const 4)))
            (f64.add
              (f64.convert_i32_s (local.tee 4 (i32.const 5)))
              (f64.add
                (f64.promote_f32 (local.tee 5 (f32.const 5.5)))
                (f64.add
                  (f64.convert_i64_u (local.tee 6 (i64.const 6)))
                  (f64.add
                    (f64.convert_i64_u (local.tee 7 (i64.const 0)))
                    (local.tee 8 (f64.const 8))
                  )
                )
              )
            )
          )
        )
      )
    )
  )

  (func $dummy)

  (func (export "as-block-first") (param i32) (result i32)
    (block (result i32) (local.tee 0 (i32.const 1)) (call $dummy))
  )
  (func (export "as-block-mid") (param i32) (result i32)
    (block (result i32) (call $dummy) (local.tee 0 (i32.const 1)) (call $dummy))
  )
  (func (export "as-block-last") (param i32) (result i32)
    (block (result i32) (call $dummy) (call $dummy) (local.tee 0 (i32.const 1)))
  )

  (func (export "as-loop-first") (param i32) (result i32)
    (loop (result i32) (local.tee 0 (i32.const 3)) (call $dummy))
  )
  (func (export "as-loop-mid") (param i32) (result i32)
    (loop (result i32) (call $dummy) (local.tee 0 (i32.const 4)) (call $dummy))
  )
  (func (export "as-loop-last") (param i32) (result i32)
    (loop (result i32) (call $dummy) (call $dummy) (local.tee 0 (i32.const 5)))
  )

  (func (export "as-br-value") (param i32) (result i32)
    (block (result i32) (br 0 (local.tee 0 (i32.const 9))))
  )

  (func (export "as-br_if-cond") (param i32)
    (block (br_if 0 (local.tee 0 (i32.const 1))))
  )
  (func (export "as-br_if-value") (param i32) (result i32)
    (block (result i32)
      (drop (br_if 0 (local.tee 0 (i32.const 8)) (i32.const 1))) (i32.const 7)
    )
  )
  (func (export "as-br_if-value-cond") (param i32) (result i32)
    (block (result i32)
      (drop (br_if 0 (i32.const 6) (local.tee 0 (i32.const 9)))) (i32.const 7)
    )
  )

  (func (export "as-br_table-index") (param i32)
    (block (br_table 0 0 0 (local.tee 0 (i32.const 0))))
  )
  (func (export "as-br_table-value") (param i32) (result i32)
    (block (result i32)
      (br_table 0 0 0 (local.tee 0 (i32.const 10)) (i32.const 1)) (i32.const 7)
    )
  )
  (func (export "as-br_table-value-index") (param i32) (result i32)
    (block (result i32)
      (br_table 0 0 (i32.const 6) (local.tee 0 (i32.const 11))) (i32.const 7)
    )
  )

  (func (export "as-return-value") (param i32) (result i32)
    (return (local.tee 0 (i32.const 7)))
  )

  (func (export "as-if-cond") (param i32) (result i32)
    (if (result i32) (local.tee 0 (i32.const 2))
      (then (i32.const 0)) (else (i32.const 1))
    )
  )
  (func (export "as-if-then") (param i32) (result i32)
    (if (result i32) (local.get 0)
      (then (local.tee 0 (i32.const 3))) (else (local.get 0))
    )
  )
  (func (export "as-if-else") (param i32) (result i32)
    (if (result i32) (local.get 0)
      (then (local.get 0)) (else (local.tee 0 (i32.const 4)))
    )
  )

  (func (export "as-select-first") (param i32 i32) (result i32)
    (select (local.tee 0 (i32.const 5)) (local.get 0) (local.get 1))
  )
  (func (export "as-select-second") (param i32 i32) (result i32)
    (select (local.get 0) (local.tee 0 (i32.const 6)) (local.get 1))
  )
  (func (export "as-select-cond") (param i32) (result i32)
    (select (i32.const 0) (i32.const 1) (local.tee 0 (i32.const 7)))
  )

  (func $f (param i32 i32 i32) (result i32) (i32.const -1))
  (func (export "as-call-first") (param i32) (result i32)
    (call $f (local.tee 0 (i32.const 12)) (i32.const 2) (i32.const 3))
  )
  (func (export "as-call-mid") (param i32) (result i32)
    (call $f (i32.const 1) (local.tee 0 (i32.const 13)) (i32.const 3))
  )
  (func (export "as-call-last") (param i32) (result i32)
    (call $f (i32.const 1) (i32.const 2) (local.tee 0 (i32.const 14)))
  )

  (type $sig (func (param i32 i32 i32) (result i32)))
  (table funcref (elem $f))
  (func (export "as-call_indirect-first") (param i32) (result i32)
    (call_indirect (type $sig)
      (local.tee 0 (i32.const 1)) (i32.const 2) (i32.const 3) (i32.const 0)
    )
  )
  (func (export "as-call_indirect-mid") (param i32) (result i32)
    (call_indirect (type $sig)
      (i32.const 1) (local.tee 0 (i32.const 2)) (i32.const 3) (i32.const 0)
    )
  )
  (func (export "as-call_indirect-last") (param i32) (result i32)
    (call_indirect (type $sig)
      (i32.const 1) (i32.const 2) (local.tee 0 (i32.const 3)) (i32.const 0)
    )
  )
  (func (export "as-call_indirect-index") (param i32) (result i32)
    (call_indirect (type $sig)
      (i32.const 1) (i32.const 2) (i32.const 3) (local.tee 0 (i32.const 0))
    )
  )

  (func (export "as-local.set-value") (local i32)
    (local.set 0 (local.tee 0 (i32.const 1)))
  )
  (func (export "as-local.tee-value") (param i32) (result i32)
    (local.tee 0 (local.tee 0 (i32.const 1)))
  )
  (global $g (mut i32) (i32.const 0))
  (func (export "as-global.set-value") (local i32)
    (global.set $g (local.tee 0 (i32.const 1)))
  )

  (memory 1)
  (func (export "as-load-address") (param i32) (result i32)
    (i32.load (local.tee 0 (i32.const 1)))
  )
  (func (export "as-loadN-address") (param i32) (result i32)
    (i32.load8_s (local.tee 0 (i32.const 3)))
  )

  (func (export "as-store-address") (param i32)
    (i32.store (local.tee 0 (i32.const 30)) (i32.const 7))
  )
  (func (export "as-store-value") (param i32)
    (i32.store (i32.const 2) (local.tee 0 (i32.const 1)))
  )

  (func (export "as-storeN-address") (param i32)
    (i32.store8 (local.tee 0 (i32.const 1)) (i32.const 7))
  )
  (func (export "as-storeN-value") (param i32)
    (i32.store16 (i32.const 2) (local.tee 0 (i32.const 1)))
  )

  (func (export "as-unary-operand") (param f32) (result f32)
    (f32.neg (local.tee 0 (f32.const nan:0x0f1e2)))
  )

  (func (export "as-binary-left") (param i32) (result i32)
    (i32.add (local.tee 0 (i32.const 3)) (i32.const 10))
  )
  (func (export "as-binary-right") (param i32) (result i32)
    (i32.sub (i32.const 10) (local.tee 0 (i32.const 4)))
  )

  (func (export "as-test-operand") (param i32) (result i32)
    (i32.eqz (local.tee 0 (i32.const 0)))
  )

  (func (export "as-compare-left") (param i32) (result i32)
    (i32.le_s (local.tee 0 (i32.const 43)) (i32.const 10))
  )
  (func (export "as-compare-right") (param i32) (result i32)
    (i32.ne (i32.const 10) (local.tee 0 (i32.const 42)))
  )

  (func (export "as-convert-operand") (param i64) (result i32)
    (i32.wrap_i64 (local.tee 0 (i64.const 41)))
  )

  (func (export "as-memory.grow-size") (param i32) (result i32)
    (memory.grow (local.tee 0 (i32.const 40)))
  )
)
"#
    );
}

#[test]
fn r#loop() {
    assert_snapshot!(
        r#"
(module
  (memory 1)

  (func $dummy)

  (func (export "empty")
    (loop)
    (loop $l)
  )

  (func (export "singular") (result i32)
    (loop (nop))
    (loop (result i32) (i32.const 7))
  )

  (func (export "multi") (result i32)
    (loop (call $dummy) (call $dummy) (call $dummy) (call $dummy))
    (loop (result i32) (call $dummy) (call $dummy) (i32.const 8) (call $dummy))
    (drop)
    (loop (result i32 i64 i32)
      (call $dummy) (call $dummy) (call $dummy) (i32.const 8) (call $dummy)
      (call $dummy) (call $dummy) (call $dummy) (i64.const 7) (call $dummy)
      (call $dummy) (call $dummy) (call $dummy) (i32.const 9) (call $dummy)
    )
    (drop) (drop)
  )

  (func (export "nested") (result i32)
    (loop (result i32)
      (loop (call $dummy) (block) (nop))
      (loop (result i32) (call $dummy) (i32.const 9))
    )
  )

  (func (export "deep") (result i32)
    (loop (result i32) (block (result i32)
      (loop (result i32) (block (result i32)
        (loop (result i32) (block (result i32)
          (loop (result i32) (block (result i32)
            (loop (result i32) (block (result i32)
              (loop (result i32) (block (result i32)
                (loop (result i32) (block (result i32)
                  (loop (result i32) (block (result i32)
                    (loop (result i32) (block (result i32)
                      (loop (result i32) (block (result i32)
                        (loop (result i32) (block (result i32)
                          (loop (result i32) (block (result i32)
                            (loop (result i32) (block (result i32)
                              (loop (result i32) (block (result i32)
                                (loop (result i32) (block (result i32)
                                  (loop (result i32) (block (result i32)
                                    (loop (result i32) (block (result i32)
                                      (loop (result i32) (block (result i32)
                                        (loop (result i32) (block (result i32)
                                          (loop (result i32) (block (result i32)
                                            (call $dummy) (i32.const 150)
                                          ))
                                        ))
                                      ))
                                    ))
                                  ))
                                ))
                              ))
                            ))
                          ))
                        ))
                      ))
                    ))
                  ))
                ))
              ))
            ))
          ))
        ))
      ))
    ))
  )

  (func (export "as-select-first") (result i32)
    (select (loop (result i32) (i32.const 1)) (i32.const 2) (i32.const 3))
  )
  (func (export "as-select-mid") (result i32)
    (select (i32.const 2) (loop (result i32) (i32.const 1)) (i32.const 3))
  )
  (func (export "as-select-last") (result i32)
    (select (i32.const 2) (i32.const 3) (loop (result i32) (i32.const 1)))
  )

  (func (export "as-if-condition")
    (loop (result i32) (i32.const 1)) (if (then (call $dummy)))
  )
  (func (export "as-if-then") (result i32)
    (if (result i32) (i32.const 1) (then (loop (result i32) (i32.const 1))) (else (i32.const 2)))
  )
  (func (export "as-if-else") (result i32)
    (if (result i32) (i32.const 1) (then (i32.const 2)) (else (loop (result i32) (i32.const 1))))
  )

  (func (export "as-br_if-first") (result i32)
    (block (result i32) (br_if 0 (loop (result i32) (i32.const 1)) (i32.const 2)))
  )
  (func (export "as-br_if-last") (result i32)
    (block (result i32) (br_if 0 (i32.const 2) (loop (result i32) (i32.const 1))))
  )

  (func (export "as-br_table-first") (result i32)
    (block (result i32) (loop (result i32) (i32.const 1)) (i32.const 2) (br_table 0 0))
  )
  (func (export "as-br_table-last") (result i32)
    (block (result i32) (i32.const 2) (loop (result i32) (i32.const 1)) (br_table 0 0))
  )

  (func $func (param i32 i32) (result i32) (local.get 0))
  (type $check (func (param i32 i32) (result i32)))
  (table funcref (elem $func))
  (func (export "as-call_indirect-first") (result i32)
    (block (result i32)
      (call_indirect (type $check)
        (loop (result i32) (i32.const 1)) (i32.const 2) (i32.const 0)
      )
    )
  )
  (func (export "as-call_indirect-mid") (result i32)
    (block (result i32)
      (call_indirect (type $check)
        (i32.const 2) (loop (result i32) (i32.const 1)) (i32.const 0)
      )
    )
  )
  (func (export "as-call_indirect-last") (result i32)
    (block (result i32)
      (call_indirect (type $check)
        (i32.const 1) (i32.const 2) (loop (result i32) (i32.const 0))
      )
    )
  )

  (func (export "as-store-first")
    (loop (result i32) (i32.const 1)) (i32.const 1) (i32.store)
  )
  (func (export "as-store-last")
    (i32.const 10) (loop (result i32) (i32.const 1)) (i32.store)
  )

  (func (export "as-memory.grow-value") (result i32)
    (memory.grow (loop (result i32) (i32.const 1)))
  )

  (func $f (param i32) (result i32) (local.get 0))

  (func (export "as-call-value") (result i32)
    (call $f (loop (result i32) (i32.const 1)))
  )
  (func (export "as-return-value") (result i32)
    (loop (result i32) (i32.const 1)) (return)
  )
  (func (export "as-drop-operand")
    (drop (loop (result i32) (i32.const 1)))
  )
  (func (export "as-br-value") (result i32)
    (block (result i32) (br 0 (loop (result i32) (i32.const 1))))
  )
  (func (export "as-local.set-value") (result i32)
    (local i32) (local.set 0 (loop (result i32) (i32.const 1))) (local.get 0)
  )
  (func (export "as-local.tee-value") (result i32)
    (local i32) (local.tee 0 (loop (result i32) (i32.const 1)))
  )
  (global $a (mut i32) (i32.const 0))
  (func (export "as-global.set-value") (result i32)
    (global.set $a (loop (result i32) (i32.const 1)))
    (global.get $a)
  )
  (func (export "as-load-operand") (result i32)
    (i32.load (loop (result i32) (i32.const 1)))
  )

  (func (export "as-unary-operand") (result i32)
    (i32.ctz (loop (result i32) (call $dummy) (i32.const 13)))
  )
  (func (export "as-binary-operand") (result i32)
    (i32.mul
      (loop (result i32) (call $dummy) (i32.const 3))
      (loop (result i32) (call $dummy) (i32.const 4))
    )
  )
  (func (export "as-test-operand") (result i32)
    (i32.eqz (loop (result i32) (call $dummy) (i32.const 13)))
  )
  (func (export "as-compare-operand") (result i32)
    (f32.gt
      (loop (result f32) (call $dummy) (f32.const 3))
      (loop (result f32) (call $dummy) (f32.const 3))
    )
  )
  (func (export "as-binary-operands") (result i32)
    (i32.mul
      (loop (result i32 i32)
        (call $dummy) (i32.const 3) (call $dummy) (i32.const 4)
      )
    )
  )
  (func (export "as-compare-operands") (result i32)
    (f32.gt
      (loop (result f32 f32)
        (call $dummy) (f32.const 3) (call $dummy) (f32.const 3)
      )
    )
  )
  (func (export "as-mixed-operands") (result i32)
    (loop (result i32 i32)
      (call $dummy) (i32.const 3) (call $dummy) (i32.const 4)
    )
    (i32.const 5)
    (i32.add)
    (i32.mul)
  )

  (func (export "break-bare") (result i32)
    (block (loop (br 1) (br 0) (unreachable)))
    (block (loop (br_if 1 (i32.const 1)) (unreachable)))
    (block (loop (br_table 1 (i32.const 0)) (unreachable)))
    (block (loop (br_table 1 1 1 (i32.const 1)) (unreachable)))
    (i32.const 19)
  )
  (func (export "break-value") (result i32)
    (block (result i32)
      (i32.const 0)
      (loop (param i32)
        (block (br 2 (i32.const 18)))
        (br 0 (i32.const 20))
      )
      (i32.const 19)
    )
  )
  (func (export "break-multi-value") (result i32 i32 i64)
    (block (result i32 i32 i64)
      (i32.const 0) (i32.const 0) (i64.const 0)
      (loop (param i32 i32 i64)
        (block (br 2 (i32.const 18) (i32.const -18) (i64.const 18)))
        (br 0 (i32.const 20) (i32.const -20) (i64.const 20))
      )
      (i32.const 19) (i32.const -19) (i64.const 19)
    )
  )
  (func (export "break-repeated") (result i32)
    (block (result i32)
      (loop (result i32)
        (br 1 (i32.const 18))
        (br 1 (i32.const 19))
        (drop (br_if 1 (i32.const 20) (i32.const 0)))
        (drop (br_if 1 (i32.const 20) (i32.const 1)))
        (br 1 (i32.const 21))
        (br_table 1 (i32.const 22) (i32.const 0))
        (br_table 1 1 1 (i32.const 23) (i32.const 1))
        (i32.const 21)
      )
    )
  )
  (func (export "break-inner") (result i32)
    (local i32)
    (local.set 0 (i32.const 0))
    (local.set 0 (i32.add (local.get 0) (block (result i32) (loop (result i32) (block (result i32) (br 2 (i32.const 0x1)))))))
    (local.set 0 (i32.add (local.get 0) (block (result i32) (loop (result i32) (loop (result i32) (br 2 (i32.const 0x2)))))))
    (local.set 0 (i32.add (local.get 0) (block (result i32) (loop (result i32) (block (result i32) (loop (result i32) (br 1 (i32.const 0x4))))))))
    (local.set 0 (i32.add (local.get 0) (block (result i32) (loop (result i32) (i32.ctz (br 1 (i32.const 0x8)))))))
    (local.set 0 (i32.add (local.get 0) (block (result i32) (loop (result i32) (i32.ctz (loop (result i32) (br 2 (i32.const 0x10))))))))
    (local.get 0)
  )
  (func (export "cont-inner") (result i32)
    (local i32)
    (local.set 0 (i32.const 0))
    (local.set 0 (i32.add (local.get 0) (loop (result i32) (loop (result i32) (br 1)))))
    (local.set 0 (i32.add (local.get 0) (loop (result i32) (i32.ctz (br 0)))))
    (local.set 0 (i32.add (local.get 0) (loop (result i32) (i32.ctz (loop (result i32) (br 1))))))
    (local.get 0)
  )

  (func (export "param") (result i32)
    (i32.const 1)
    (loop (param i32) (result i32)
      (i32.const 2)
      (i32.add)
    )
  )
  (func (export "params") (result i32)
    (i32.const 1)
    (i32.const 2)
    (loop (param i32 i32) (result i32)
      (i32.add)
    )
  )
  (func (export "params-id") (result i32)
    (i32.const 1)
    (i32.const 2)
    (loop (param i32 i32) (result i32 i32))
    (i32.add)
  )
  (func (export "param-break") (result i32)
    (local $x i32)
    (i32.const 1)
    (loop (param i32) (result i32)
      (i32.const 4)
      (i32.add)
      (local.tee $x)
      (local.get $x)
      (i32.const 10)
      (i32.lt_u)
      (br_if 0)
    )
  )
  (func (export "params-break") (result i32)
    (local $x i32)
    (i32.const 1)
    (i32.const 2)
    (loop (param i32 i32) (result i32)
      (i32.add)
      (local.tee $x)
      (i32.const 3)
      (local.get $x)
      (i32.const 10)
      (i32.lt_u)
      (br_if 0)
      (drop)
    )
  )
  (func (export "params-id-break") (result i32)
    (local $x i32)
    (local.set $x (i32.const 0))
    (i32.const 1)
    (i32.const 2)
    (loop (param i32 i32) (result i32 i32)
      (local.set $x (i32.add (local.get $x) (i32.const 1)))
      (br_if 0 (i32.lt_u (local.get $x) (i32.const 10)))
    )
    (i32.add)
  )

  (func $fx (export "effects") (result i32)
    (local i32)
    (block
      (loop
        (local.set 0 (i32.const 1))
        (local.set 0 (i32.mul (local.get 0) (i32.const 3)))
        (local.set 0 (i32.sub (local.get 0) (i32.const 5)))
        (local.set 0 (i32.mul (local.get 0) (i32.const 7)))
        (br 1)
        (local.set 0 (i32.mul (local.get 0) (i32.const 100)))
      )
    )
    (i32.eq (local.get 0) (i32.const -14))
  )

  (func (export "while") (param i64) (result i64)
    (local i64)
    (local.set 1 (i64.const 1))
    (block
      (loop
        (br_if 1 (i64.eqz (local.get 0)))
        (local.set 1 (i64.mul (local.get 0) (local.get 1)))
        (local.set 0 (i64.sub (local.get 0) (i64.const 1)))
        (br 0)
      )
    )
    (local.get 1)
  )

  (func (export "for") (param i64) (result i64)
    (local i64 i64)
    (local.set 1 (i64.const 1))
    (local.set 2 (i64.const 2))
    (block
      (loop
        (br_if 1 (i64.gt_u (local.get 2) (local.get 0)))
        (local.set 1 (i64.mul (local.get 1) (local.get 2)))
        (local.set 2 (i64.add (local.get 2) (i64.const 1)))
        (br 0)
      )
    )
    (local.get 1)
  )

  (func (export "nesting") (param f32 f32) (result f32)
    (local f32 f32)
    (block
      (loop
        (br_if 1 (f32.eq (local.get 0) (f32.const 0)))
        (local.set 2 (local.get 1))
        (block
          (loop
            (br_if 1 (f32.eq (local.get 2) (f32.const 0)))
            (br_if 3 (f32.lt (local.get 2) (f32.const 0)))
            (local.set 3 (f32.add (local.get 3) (local.get 2)))
            (local.set 2 (f32.sub (local.get 2) (f32.const 2)))
            (br 0)
          )
        )
        (local.set 3 (f32.div (local.get 3) (local.get 0)))
        (local.set 0 (f32.sub (local.get 0) (f32.const 1)))
        (br 0)
      )
    )
    (local.get 3)
  )

  (type $block-sig-1 (func))
  (type $block-sig-2 (func (result i32)))
  (type $block-sig-3 (func (param $x i32)))
  (type $block-sig-4 (func (param i32 f64 i32) (result i32 f64 i32)))

  (func (export "type-use")
    (loop (type $block-sig-1))
    (loop (type $block-sig-2) (i32.const 0))
    (loop (type $block-sig-3) (drop))
    (i32.const 0) (f64.const 0) (i32.const 0)
    (loop (type $block-sig-4))
    (drop) (drop) (drop)
    (loop (type $block-sig-2) (result i32) (i32.const 0))
    (loop (type $block-sig-3) (param i32) (drop))
    (i32.const 0) (f64.const 0) (i32.const 0)
    (loop (type $block-sig-4)
      (param i32) (param f64 i32) (result i32 f64) (result i32)
    )
    (drop) (drop) (drop)
  )
)
"#
    );
}

#[test]
fn memory() {
    assert_snapshot!(
        r#"
(module
  (memory 1)
  (data (i32.const 0) "ABC\a7D") (data (i32.const 20) "WASM")

  ;; Data section
  (func (export "data") (result i32)
    (i32.and
      (i32.and
        (i32.and
          (i32.eq (i32.load8_u (i32.const 0)) (i32.const 65))
          (i32.eq (i32.load8_u (i32.const 3)) (i32.const 167))
        )
        (i32.and
          (i32.eq (i32.load8_u (i32.const 6)) (i32.const 0))
          (i32.eq (i32.load8_u (i32.const 19)) (i32.const 0))
        )
      )
      (i32.and
        (i32.and
          (i32.eq (i32.load8_u (i32.const 20)) (i32.const 87))
          (i32.eq (i32.load8_u (i32.const 23)) (i32.const 77))
        )
        (i32.and
          (i32.eq (i32.load8_u (i32.const 24)) (i32.const 0))
          (i32.eq (i32.load8_u (i32.const 1023)) (i32.const 0))
        )
      )
    )
  )

  ;; Memory cast
  (func (export "cast") (result f64)
    (i64.store (i32.const 8) (i64.const -12345))
    (if
      (f64.eq
        (f64.load (i32.const 8))
        (f64.reinterpret_i64 (i64.const -12345))
      )
      (then (return (f64.const 0)))
    )
    (i64.store align=1 (i32.const 9) (i64.const 0))
    (i32.store16 align=1 (i32.const 15) (i32.const 16453))
    (f64.load align=1 (i32.const 9))
  )

  ;; Sign and zero extending memory loads
  (func (export "i32_load8_s") (param $i i32) (result i32)
    (i32.store8 (i32.const 8) (local.get $i))
    (i32.load8_s (i32.const 8))
  )
  (func (export "i32_load8_u") (param $i i32) (result i32)
    (i32.store8 (i32.const 8) (local.get $i))
    (i32.load8_u (i32.const 8))
  )
  (func (export "i32_load16_s") (param $i i32) (result i32)
    (i32.store16 (i32.const 8) (local.get $i))
    (i32.load16_s (i32.const 8))
  )
  (func (export "i32_load16_u") (param $i i32) (result i32)
    (i32.store16 (i32.const 8) (local.get $i))
    (i32.load16_u (i32.const 8))
  )
  (func (export "i64_load8_s") (param $i i64) (result i64)
    (i64.store8 (i32.const 8) (local.get $i))
    (i64.load8_s (i32.const 8))
  )
  (func (export "i64_load8_u") (param $i i64) (result i64)
    (i64.store8 (i32.const 8) (local.get $i))
    (i64.load8_u (i32.const 8))
  )
  (func (export "i64_load16_s") (param $i i64) (result i64)
    (i64.store16 (i32.const 8) (local.get $i))
    (i64.load16_s (i32.const 8))
  )
  (func (export "i64_load16_u") (param $i i64) (result i64)
    (i64.store16 (i32.const 8) (local.get $i))
    (i64.load16_u (i32.const 8))
  )
  (func (export "i64_load32_s") (param $i i64) (result i64)
    (i64.store32 (i32.const 8) (local.get $i))
    (i64.load32_s (i32.const 8))
  )
  (func (export "i64_load32_u") (param $i i64) (result i64)
    (i64.store32 (i32.const 8) (local.get $i))
    (i64.load32_u (i32.const 8))
  )
)
"#
    );
}

#[test]
fn memory_copy() {
    assert_snapshot!(
        r#"
(module
  (memory (export "memory0") 1 1)
  (data (i32.const 2) "\03\01\04\01")
  (data (i32.const 12) "\07\05\02\03\06")
  (func (export "test")
    (memory.copy (i32.const 20) (i32.const 22) (i32.const 4)))
  (func (export "load8_u") (param i32) (result i32)
    (i32.load8_u (local.get 0)))
)
"#
    );
}

#[test]
fn memory_fill() {
    assert_snapshot!(
        r#"
(module
  (memory 1 1)
  
  (func (export "checkRange") (param $from i32) (param $to i32) (param $expected i32) (result i32)
    (loop $cont
      (if (i32.eq (local.get $from) (local.get $to))
        (then
          (return (i32.const -1))))
      (if (i32.eq (i32.load8_u (local.get $from)) (local.get $expected))
        (then
          (local.set $from (i32.add (local.get $from) (i32.const 1)))
          (br $cont))))
    (return (local.get $from)))

  (func (export "test")
    (memory.fill (i32.const 0xFF00) (i32.const 0x55) (i32.const 256)))
)
"#
    );
}

#[test]
fn memory_grow() {
    assert_snapshot!(
        r#"
(module
  (memory 0)

  (func (export "load_at_zero") (result i32) (i32.load (i32.const 0)))
  (func (export "store_at_zero") (i32.store (i32.const 0) (i32.const 2)))

  (func (export "load_at_page_size") (result i32) (i32.load (i32.const 0x10000)))
  (func (export "store_at_page_size") (i32.store (i32.const 0x10000) (i32.const 3)))

  (func (export "grow") (param $sz i32) (result i32) (memory.grow (local.get $sz)))
  (func (export "size") (result i32) (memory.size))
)
"#
    );
}

#[test]
fn memory_init() {
    assert_snapshot!(
        r#"
(module
  (memory (export "memory0") 1 1)
  (data (i32.const 2) "\03\01\04\01")
  (data "\02\07\01\08")
  (data (i32.const 12) "\07\05\02\03\06")
  (data "\05\09\02\07\06")
  (func (export "test")
    (memory.init 3 (i32.const 15) (i32.const 1) (i32.const 3)))
  (func (export "load8_u") (param i32) (result i32)
    (i32.load8_u (local.get 0)))
)
"#
    );
}

#[test]
fn memory_redundancy() {
    assert_snapshot!(
        r#"
(module
  (memory 1 1)

  (func (export "zero_everything")
    (i32.store (i32.const 0) (i32.const 0))
    (i32.store (i32.const 4) (i32.const 0))
    (i32.store (i32.const 8) (i32.const 0))
    (i32.store (i32.const 12) (i32.const 0))
  )

  (func (export "test_store_to_load") (result i32)
    (i32.store (i32.const 8) (i32.const 0))
    (f32.store (i32.const 5) (f32.const -0.0))
    (i32.load (i32.const 8))
  )

  (func (export "test_redundant_load") (result i32)
    (local $t i32)
    (local $s i32)
    (local.set $t (i32.load (i32.const 8)))
    (i32.store (i32.const 5) (i32.const 0x80000000))
    (local.set $s (i32.load (i32.const 8)))
    (i32.add (local.get $t) (local.get $s))
  )

  (func (export "test_dead_store") (result f32)
    (local $t f32)
    (i32.store (i32.const 8) (i32.const 0x23232323))
    (local.set $t (f32.load (i32.const 11)))
    (i32.store (i32.const 8) (i32.const 0))
    (local.get $t)
  )

  ;; A function named "malloc" which implementations nonetheless shouldn't
  ;; assume behaves like C malloc.
  (func $malloc (export "malloc")
     (param $size i32)
     (result i32)
     (i32.const 16)
  )

  ;; Call malloc twice, but unlike C malloc, we don't get non-aliasing pointers.
  (func (export "malloc_aliasing")
     (result i32)
     (local $x i32)
     (local $y i32)
     (local.set $x (call $malloc (i32.const 4)))
     (local.set $y (call $malloc (i32.const 4)))
     (i32.store (local.get $x) (i32.const 42))
     (i32.store (local.get $y) (i32.const 43))
     (i32.load (local.get $x))
  )
)
"#
    );
}

#[test]
fn memory_size() {
    assert_snapshot!(
        r#"
(module
  (memory 1)
  (data (i32.const 0) "abcdefgh")
  (data (i32.const 0xfff8) "abcdefgh")

  (func (export "i32.load") (param $a i32) (result i32)
    (i32.load (local.get $a))
  )
  (func (export "i64.load") (param $a i32) (result i64)
    (i64.load (local.get $a))
  )
  (func (export "f32.load") (param $a i32) (result f32)
    (f32.load (local.get $a))
  )
  (func (export "f64.load") (param $a i32) (result f64)
    (f64.load (local.get $a))
  )
  (func (export "i32.load8_s") (param $a i32) (result i32)
    (i32.load8_s (local.get $a))
  )
  (func (export "i32.load8_u") (param $a i32) (result i32)
    (i32.load8_u (local.get $a))
  )
  (func (export "i32.load16_s") (param $a i32) (result i32)
    (i32.load16_s (local.get $a))
  )
  (func (export "i32.load16_u") (param $a i32) (result i32)
    (i32.load16_u (local.get $a))
  )
  (func (export "i64.load8_s") (param $a i32) (result i64)
    (i64.load8_s (local.get $a))
  )
  (func (export "i64.load8_u") (param $a i32) (result i64)
    (i64.load8_u (local.get $a))
  )
  (func (export "i64.load16_s") (param $a i32) (result i64)
    (i64.load16_s (local.get $a))
  )
  (func (export "i64.load16_u") (param $a i32) (result i64)
    (i64.load16_u (local.get $a))
  )
  (func (export "i64.load32_s") (param $a i32) (result i64)
    (i64.load32_s (local.get $a))
  )
  (func (export "i64.load32_u") (param $a i32) (result i64)
    (i64.load32_u (local.get $a))
  )
  (func (export "i32.store") (param $a i32) (param $v i32)
    (i32.store (local.get $a) (local.get $v))
  )
  (func (export "i64.store") (param $a i32) (param $v i64)
    (i64.store (local.get $a) (local.get $v))
  )
  (func (export "f32.store") (param $a i32) (param $v f32)
    (f32.store (local.get $a) (local.get $v))
  )
  (func (export "f64.store") (param $a i32) (param $v f64)
    (f64.store (local.get $a) (local.get $v))
  )
  (func (export "i32.store8") (param $a i32) (param $v i32)
    (i32.store8 (local.get $a) (local.get $v))
  )
  (func (export "i32.store16") (param $a i32) (param $v i32)
    (i32.store16 (local.get $a) (local.get $v))
  )
  (func (export "i64.store8") (param $a i32) (param $v i64)
    (i64.store8 (local.get $a) (local.get $v))
  )
  (func (export "i64.store16") (param $a i32) (param $v i64)
    (i64.store16 (local.get $a) (local.get $v))
  )
  (func (export "i64.store32") (param $a i32) (param $v i64)
    (i64.store32 (local.get $a) (local.get $v))
  )
)
"#
    );
}

#[test]
fn nop() {
    assert_snapshot!(
        r#"
(module
  ;; Auxiliary definitions
  (func $dummy)
  (func $3-ary (param i32 i32 i32) (result i32)
    local.get 0 local.get 1 local.get 2 i32.sub i32.add
  )
  (memory 1)

  (func (export "as-func-first") (result i32)
    (nop) (i32.const 1)
  )
  (func (export "as-func-mid") (result i32)
    (call $dummy) (nop) (i32.const 2)
  )
  (func (export "as-func-last") (result i32)
    (call $dummy) (i32.const 3) (nop)
  )
  (func (export "as-func-everywhere") (result i32)
    (nop) (nop) (call $dummy) (nop) (i32.const 4) (nop) (nop)
  )

  (func (export "as-drop-first") (param i32)
    (nop) (local.get 0) (drop)
  )
  (func (export "as-drop-last") (param i32)
    (local.get 0) (nop) (drop)
  )
  (func (export "as-drop-everywhere") (param i32)
    (nop) (nop) (local.get 0) (nop) (nop) (drop)
  )

  (func (export "as-select-first") (param i32) (result i32)
    (nop) (local.get 0) (local.get 0) (local.get 0) (select)
  )
  (func (export "as-select-mid1") (param i32) (result i32)
    (local.get 0) (nop) (local.get 0) (local.get 0) (select)
  )
  (func (export "as-select-mid2") (param i32) (result i32)
    (local.get 0) (local.get 0) (nop) (local.get 0) (select)
  )
  (func (export "as-select-last") (param i32) (result i32)
    (local.get 0) (local.get 0) (local.get 0) (nop) (select)
  )
  (func (export "as-select-everywhere") (param i32) (result i32)
    (nop) (local.get 0) (nop) (nop) (local.get 0)
    (nop) (nop) (local.get 0) (nop) (nop) (select)
  )

  (func (export "as-block-first") (result i32)
    (block (result i32) (nop) (i32.const 2))
  )
  (func (export "as-block-mid") (result i32)
    (block (result i32) (call $dummy) (nop) (i32.const 2))
  )
  (func (export "as-block-last") (result i32)
    (block (result i32) (nop) (call $dummy) (i32.const 3) (nop))
  )
  (func (export "as-block-everywhere") (result i32)
    (block (result i32)
      (nop) (nop) (call $dummy) (nop) (i32.const 4) (nop) (nop)
    )
  )

  (func (export "as-loop-first") (result i32)
    (loop (result i32) (nop) (i32.const 2))
  )
  (func (export "as-loop-mid") (result i32)
    (loop (result i32) (call $dummy) (nop) (i32.const 2))
  )
  (func (export "as-loop-last") (result i32)
    (loop (result i32) (call $dummy) (i32.const 3) (nop))
  )
  (func (export "as-loop-everywhere") (result i32)
    (loop (result i32)
      (nop) (nop) (call $dummy) (nop) (i32.const 4) (nop) (nop)
    )
  )

  (func (export "as-if-condition") (param i32)
    (local.get 0) (nop) (if (then (call $dummy)))
  )
  (func (export "as-if-then") (param i32)
    (if (local.get 0) (then (nop)) (else (call $dummy)))
  )
  (func (export "as-if-else") (param i32)
    (if (local.get 0) (then (call $dummy)) (else (nop)))
  )

  (func (export "as-br-first") (param i32) (result i32)
    (block (result i32) (nop) (local.get 0) (br 0))
  )
  (func (export "as-br-last") (param i32) (result i32)
    (block (result i32) (local.get 0) (nop) (br 0))
  )
  (func (export "as-br-everywhere") (param i32) (result i32)
    (block (result i32) (nop) (nop) (local.get 0) (nop) (nop) (br 0))
  )

  (func (export "as-br_if-first") (param i32) (result i32)
    (block (result i32) (nop) (local.get 0) (local.get 0) (br_if 0))
  )
  (func (export "as-br_if-mid") (param i32) (result i32)
    (block (result i32) (local.get 0) (nop) (local.get 0) (br_if 0))
  )
  (func (export "as-br_if-last") (param i32) (result i32)
    (block (result i32) (local.get 0) (local.get 0) (nop) (br_if 0))
  )
  (func (export "as-br_if-everywhere") (param i32) (result i32)
    (block (result i32)
      (nop) (nop) (local.get 0) (nop) (nop) (local.get 0) (nop) (nop)
      (br_if 0)
    )
  )

  (func (export "as-br_table-first") (param i32) (result i32)
    (block (result i32) (nop) (local.get 0) (local.get 0) (br_table 0 0))
  )
  (func (export "as-br_table-mid") (param i32) (result i32)
    (block (result i32) (local.get 0) (nop) (local.get 0) (br_table 0 0))
  )
  (func (export "as-br_table-last") (param i32) (result i32)
    (block (result i32) (local.get 0) (local.get 0) (nop) (br_table 0 0))
  )
  (func (export "as-br_table-everywhere") (param i32) (result i32)
    (block (result i32)
      (nop) (nop) (local.get 0) (nop) (nop) (local.get 0) (nop) (nop)
      (br_table 0 0)
    )
  )

  (func (export "as-return-first") (param i32) (result i32)
    (nop) (local.get 0) (return)
  )
  (func (export "as-return-last") (param i32) (result i32)
    (local.get 0) (nop) (return)
  )
  (func (export "as-return-everywhere") (param i32) (result i32)
    (nop) (nop) (local.get 0) (nop) (nop) (return)
  )

  (func (export "as-call-first") (param i32 i32 i32) (result i32)
    (nop) (local.get 0) (local.get 1) (local.get 2) (call $3-ary)
  )
  (func (export "as-call-mid1") (param i32 i32 i32) (result i32)
    (local.get 0) (nop) (local.get 1) (local.get 2) (call $3-ary)
  )
  (func (export "as-call-mid2") (param i32 i32 i32) (result i32)
    (local.get 0) (local.get 1) (nop) (local.get 2) (call $3-ary)
  )
  (func (export "as-call-last") (param i32 i32 i32) (result i32)
    (local.get 0) (local.get 1) (local.get 2) (nop) (call $3-ary)
  )
  (func (export "as-call-everywhere") (param i32 i32 i32) (result i32)
    (nop) (nop) (local.get 0) (nop) (nop) (local.get 1)
    (nop) (nop) (local.get 2) (nop) (nop) (call $3-ary)
  )

  (func (export "as-unary-first") (param i32) (result i32)
    (nop) (local.get 0) (i32.ctz)
  )
  (func (export "as-unary-last") (param i32) (result i32)
    (local.get 0) (nop) (i32.ctz)
  )
  (func (export "as-unary-everywhere") (param i32) (result i32)
    (nop) (nop) (local.get 0) (nop) (nop) (i32.ctz)
  )

  (func (export "as-binary-first") (param i32) (result i32)
    (nop) (local.get 0) (local.get 0) (i32.add)
  )
  (func (export "as-binary-mid") (param i32) (result i32)
    (local.get 0) (nop) (local.get 0) (i32.add)
  )
  (func (export "as-binary-last") (param i32) (result i32)
    (local.get 0) (local.get 0) (nop) (i32.add)
  )
  (func (export "as-binary-everywhere") (param i32) (result i32)
    (nop) (local.get 0) (nop) (nop) (local.get 0) (nop) (nop) (i32.add)
  )

  (func (export "as-test-first") (param i32) (result i32)
    (nop) (local.get 0) (i32.eqz)
  )
  (func (export "as-test-last") (param i32) (result i32)
    (local.get 0) (nop) (i32.eqz)
  )
  (func (export "as-test-everywhere") (param i32) (result i32)
    (nop) (nop) (local.get 0) (nop) (nop) i32.eqz
  )

  (func (export "as-compare-first") (param i32) (result i32)
    (nop) (local.get 0) (local.get 0) (i32.ne)
  )
  (func (export "as-compare-mid") (param i32) (result i32)
    (local.get 0) (nop) (local.get 0) (i32.ne)
  )
  (func (export "as-compare-last") (param i32) (result i32)
    (local.get 0) (local.get 0) (nop) (i32.lt_u)
  )
  (func (export "as-compare-everywhere") (param i32) (result i32)
    (nop) (local.get 0) (nop) (nop) (local.get 0) (nop) (nop) (i32.le_s)
  )

  (func (export "as-memory.grow-first") (param i32) (result i32)
    (nop) (local.get 0) (memory.grow)
  )
  (func (export "as-memory.grow-last") (param i32) (result i32)
    (local.get 0) (nop) (memory.grow)
  )
  (func (export "as-memory.grow-everywhere") (param i32) (result i32)
    (nop) (nop) (local.get 0) (nop) (nop) (memory.grow)
  )

  (func $func (param i32 i32) (result i32) (local.get 0))
  (type $check (func (param i32 i32) (result i32)))
  (table funcref (elem $func))
  (func (export "as-call_indirect-first") (result i32)
    (block (result i32)
      (nop) (i32.const 1) (i32.const 2) (i32.const 0)
      (call_indirect (type $check))
    )
  )
  (func (export "as-call_indirect-mid1") (result i32)
    (block (result i32)
      (i32.const 1) (nop) (i32.const 2) (i32.const 0)
      (call_indirect (type $check))
    )
  )
  (func (export "as-call_indirect-mid2") (result i32)
    (block (result i32)
      (i32.const 1) (i32.const 2) (nop) (i32.const 0)
      (call_indirect (type $check))
    )
  )
  (func (export "as-call_indirect-last") (result i32)
    (block (result i32)
      (i32.const 1) (i32.const 2) (i32.const 0) (nop)
      (call_indirect (type $check))
    )
  )
  (func (export "as-call_indirect-everywhere") (result i32)
    (block (result i32)
      (nop) (nop) (i32.const 1) (nop) (nop) (i32.const 2) (nop) (nop) (i32.const 0) (nop) (nop)
      (call_indirect (type $check))
    )
  )

  (func (export "as-local.set-first") (param i32) (result i32)
    (nop) (i32.const 2) (local.set 0) (local.get 0)
  )
  (func (export "as-local.set-last") (param i32) (result i32)
    (i32.const 2) (nop) (local.set 0) (local.get 0)
  )
  (func (export "as-local.set-everywhere") (param i32) (result i32)
    (nop) (nop) (i32.const 2) (nop) (nop) (local.set 0) (local.get 0)
  )

  (func (export "as-local.tee-first") (param i32) (result i32)
    (nop) (i32.const 2) (local.tee 0)
  )
  (func (export "as-local.tee-last") (param i32) (result i32)
    (i32.const 2) (nop) (local.tee 0)
  )
  (func (export "as-local.tee-everywhere") (param i32) (result i32)
    (nop) (nop) (i32.const 2) (nop) (nop) (local.tee 0)
  )

  (global $a (mut i32) (i32.const 0))
  (func (export "as-global.set-first") (result i32)
    (nop) (i32.const 2) (global.set $a) (global.get $a)
  )
  (func (export "as-global.set-last") (result i32)
    (i32.const 2) (nop) (global.set $a) (global.get $a)
  )
  (func (export "as-global.set-everywhere") (result i32)
    (nop) (nop) (i32.const 2) (nop) (nop) (global.set 0)
    (global.get $a)
  )

  (func (export "as-load-first") (param i32) (result i32)
    (nop) (local.get 0) (i32.load)
  )
  (func (export "as-load-last") (param i32) (result i32)
    (local.get 0) (nop) (i32.load)
  )
  (func (export "as-load-everywhere") (param i32) (result i32)
    (nop) (nop) (local.get 0) (nop) (nop) (i32.load)
  )

  (func (export "as-store-first") (param i32 i32)
    (nop) (local.get 0) (local.get 1) (i32.store)
  )
  (func (export "as-store-mid") (param i32 i32)
    (local.get 0) (nop) (local.get 1) (i32.store)
  )
  (func (export "as-store-last") (param i32 i32)
    (local.get 0) (local.get 1) (nop) (i32.store)
  )
  (func (export "as-store-everywhere") (param i32 i32)
    (nop) (nop) (local.get 0) (nop) (nop) (local.get 1) (nop) (nop) (i32.store)
  )
)
"#
    );
}

// #[test]
// TODO: runtime lib calls register
fn ref_func() {
    assert_snapshot!(
        r#"
(module
  (func $f (import "M" "f") (param i32) (result i32))
  (func $g (param $x i32) (result i32)
    (i32.add (local.get $x) (i32.const 1))
  )

  (global funcref (ref.func $f))
  (global funcref (ref.func $g))
  (global $v (mut funcref) (ref.func $f))

  (global funcref (ref.func $gf1))
  (global funcref (ref.func $gf2))
  (func (drop (ref.func $ff1)) (drop (ref.func $ff2)))
  (elem declare func $gf1 $ff1)
  (elem declare funcref (ref.func $gf2) (ref.func $ff2))
  (func $gf1)
  (func $gf2)
  (func $ff1)
  (func $ff2)

  (func (export "is_null-f") (result i32)
    (ref.is_null (ref.func $f))
  )
  (func (export "is_null-g") (result i32)
    (ref.is_null (ref.func $g))
  )
  (func (export "is_null-v") (result i32)
    (ref.is_null (global.get $v))
  )

  (func (export "set-f") (global.set $v (ref.func $f)))
  (func (export "set-g") (global.set $v (ref.func $g)))

  (table $t 1 funcref)
  (elem declare func $f $g)

  (func (export "call-f") (param $x i32) (result i32)
    (table.set $t (i32.const 0) (ref.func $f))
    (call_indirect $t (param i32) (result i32) (local.get $x) (i32.const 0))
  )
  (func (export "call-g") (param $x i32) (result i32)
    (table.set $t (i32.const 0) (ref.func $g))
    (call_indirect $t (param i32) (result i32) (local.get $x) (i32.const 0))
  )
  (func (export "call-v") (param $x i32) (result i32)
    (table.set $t (i32.const 0) (global.get $v))
    (call_indirect $t (param i32) (result i32) (local.get $x) (i32.const 0))
  )
)
"#
    );
}

#[test]
fn ref_is_null() {
    assert_snapshot!(
        r#"
(module
  (func $f1 (export "funcref") (param $x funcref) (result i32)
    (ref.is_null (local.get $x))
  )
  (func $f2 (export "externref") (param $x externref) (result i32)
    (ref.is_null (local.get $x))
  )

  (table $t1 2 funcref)
  (table $t2 2 externref)
  (elem (table $t1) (i32.const 1) func $dummy)
  (func $dummy)

  (func (export "init") (param $r externref)
    (table.set $t2 (i32.const 1) (local.get $r))
  )
  (func (export "deinit")
    (table.set $t1 (i32.const 1) (ref.null func))
    (table.set $t2 (i32.const 1) (ref.null extern))
  )

  (func (export "funcref-elem") (param $x i32) (result i32)
    (call $f1 (table.get $t1 (local.get $x)))
  )
  (func (export "externref-elem") (param $x i32) (result i32)
    (call $f2 (table.get $t2 (local.get $x)))
  )
)
"#
    );
}

#[test]
fn ref_null() {
    assert_snapshot!(
        r#"
(module
  (func (export "externref") (result externref) (ref.null extern))
  (func (export "funcref") (result funcref) (ref.null func))

  (global externref (ref.null extern))
  (global funcref (ref.null func))
)
"#
    );
}

#[test]
fn r#return() {
    assert_snapshot!(
        r#"
(module
  ;; Auxiliary definition
  (func $dummy)

  (func (export "type-i32") (drop (i32.ctz (return))))
  (func (export "type-i64") (drop (i64.ctz (return))))
  (func (export "type-f32") (drop (f32.neg (return))))
  (func (export "type-f64") (drop (f64.neg (return))))

  (func (export "type-i32-value") (result i32)
    (block (result i32) (i32.ctz (return (i32.const 1))))
  )
  (func (export "type-i64-value") (result i64)
    (block (result i64) (i64.ctz (return (i64.const 2))))
  )
  (func (export "type-f32-value") (result f32)
    (block (result f32) (f32.neg (return (f32.const 3))))
  )
  (func (export "type-f64-value") (result f64)
    (block (result f64) (f64.neg (return (f64.const 4))))
  )

  (func (export "nullary") (return))
  (func (export "unary") (result f64) (return (f64.const 3)))

  (func (export "as-func-first") (result i32)
    (return (i32.const 1)) (i32.const 2)
  )
  (func (export "as-func-mid") (result i32)
    (call $dummy) (return (i32.const 2)) (i32.const 3)
  )
  (func (export "as-func-last")
    (nop) (call $dummy) (return)
  )
  (func (export "as-func-value") (result i32)
    (nop) (call $dummy) (return (i32.const 3))
  )

  (func (export "as-block-first")
    (block (return) (call $dummy))
  )
  (func (export "as-block-mid")
    (block (call $dummy) (return) (call $dummy))
  )
  (func (export "as-block-last")
    (block (nop) (call $dummy) (return))
  )
  (func (export "as-block-value") (result i32)
    (block (result i32) (nop) (call $dummy) (return (i32.const 2)))
  )

  (func (export "as-loop-first") (result i32)
    (loop (result i32) (return (i32.const 3)) (i32.const 2))
  )
  (func (export "as-loop-mid") (result i32)
    (loop (result i32) (call $dummy) (return (i32.const 4)) (i32.const 2))
  )
  (func (export "as-loop-last") (result i32)
    (loop (result i32) (nop) (call $dummy) (return (i32.const 5)))
  )

  (func (export "as-br-value") (result i32)
    (block (result i32) (br 0 (return (i32.const 9))))
  )

  (func (export "as-br_if-cond")
    (block (br_if 0 (return)))
  )
  (func (export "as-br_if-value") (result i32)
    (block (result i32)
      (drop (br_if 0 (return (i32.const 8)) (i32.const 1))) (i32.const 7)
    )
  )
  (func (export "as-br_if-value-cond") (result i32)
    (block (result i32)
      (drop (br_if 0 (i32.const 6) (return (i32.const 9)))) (i32.const 7)
    )
  )

  (func (export "as-br_table-index") (result i64)
    (block (br_table 0 0 0 (return (i64.const 9)))) (i64.const -1)
  )
  (func (export "as-br_table-value") (result i32)
    (block (result i32)
      (br_table 0 0 0 (return (i32.const 10)) (i32.const 1)) (i32.const 7)
    )
  )
  (func (export "as-br_table-value-index") (result i32)
    (block (result i32)
      (br_table 0 0 (i32.const 6) (return (i32.const 11))) (i32.const 7)
    )
  )

  (func (export "as-return-value") (result i64)
    (return (return (i64.const 7)))
  )

  (func (export "as-if-cond") (result i32)
    (if (result i32)
      (return (i32.const 2)) (then (i32.const 0)) (else (i32.const 1))
    )
  )
  (func (export "as-if-then") (param i32 i32) (result i32)
    (if (result i32)
      (local.get 0) (then (return (i32.const 3))) (else (local.get 1))
    )
  )
  (func (export "as-if-else") (param i32 i32) (result i32)
    (if (result i32)
      (local.get 0) (then (local.get 1)) (else (return (i32.const 4)))
    )
  )

  (func (export "as-select-first") (param i32 i32) (result i32)
    (select (return (i32.const 5)) (local.get 0) (local.get 1))
  )
  (func (export "as-select-second") (param i32 i32) (result i32)
    (select (local.get 0) (return (i32.const 6)) (local.get 1))
  )
  (func (export "as-select-cond") (result i32)
    (select (i32.const 0) (i32.const 1) (return (i32.const 7)))
  )

  (func $f (param i32 i32 i32) (result i32) (i32.const -1))
  (func (export "as-call-first") (result i32)
    (call $f (return (i32.const 12)) (i32.const 2) (i32.const 3))
  )
  (func (export "as-call-mid") (result i32)
    (call $f (i32.const 1) (return (i32.const 13)) (i32.const 3))
  )
  (func (export "as-call-last") (result i32)
    (call $f (i32.const 1) (i32.const 2) (return (i32.const 14)))
  )

  (type $sig (func (param i32 i32 i32) (result i32)))
  (table funcref (elem $f))
  (func (export "as-call_indirect-func") (result i32)
    (call_indirect (type $sig)
      (return (i32.const 20)) (i32.const 1) (i32.const 2) (i32.const 3)
    )
  )
  (func (export "as-call_indirect-first") (result i32)
    (call_indirect (type $sig)
      (i32.const 0) (return (i32.const 21)) (i32.const 2) (i32.const 3)
    )
  )
  (func (export "as-call_indirect-mid") (result i32)
    (call_indirect (type $sig)
      (i32.const 0) (i32.const 1) (return (i32.const 22)) (i32.const 3)
    )
  )
  (func (export "as-call_indirect-last") (result i32)
    (call_indirect (type $sig)
      (i32.const 0) (i32.const 1) (i32.const 2) (return (i32.const 23))
    )
  )

  (func (export "as-local.set-value") (result i32) (local f32)
    (local.set 0 (return (i32.const 17))) (i32.const -1)
  )
  (func (export "as-local.tee-value") (result i32) (local i32)
    (local.tee 0 (return (i32.const 1)))
  )
  (global $a (mut i32) (i32.const 0))
  (func (export "as-global.set-value") (result i32)
    (global.set $a (return (i32.const 1)))
  )

  (memory 1)
  (func (export "as-load-address") (result f32)
    (f32.load (return (f32.const 1.7)))
  )
  (func (export "as-loadN-address") (result i64)
    (i64.load8_s (return (i64.const 30)))
  )

  (func (export "as-store-address") (result i32)
    (f64.store (return (i32.const 30)) (f64.const 7)) (i32.const -1)
  )
  (func (export "as-store-value") (result i32)
    (i64.store (i32.const 2) (return (i32.const 31))) (i32.const -1)
  )

  (func (export "as-storeN-address") (result i32)
    (i32.store8 (return (i32.const 32)) (i32.const 7)) (i32.const -1)
  )
  (func (export "as-storeN-value") (result i32)
    (i64.store16 (i32.const 2) (return (i32.const 33))) (i32.const -1)
  )

  (func (export "as-unary-operand") (result f32)
    (f32.neg (return (f32.const 3.4)))
  )

  (func (export "as-binary-left") (result i32)
    (i32.add (return (i32.const 3)) (i32.const 10))
  )
  (func (export "as-binary-right") (result i64)
    (i64.sub (i64.const 10) (return (i64.const 45)))
  )

  (func (export "as-test-operand") (result i32)
    (i32.eqz (return (i32.const 44)))
  )

  (func (export "as-compare-left") (result i32)
    (f64.le (return (i32.const 43)) (f64.const 10))
  )
  (func (export "as-compare-right") (result i32)
    (f32.ne (f32.const 10) (return (i32.const 42)))
  )

  (func (export "as-convert-operand") (result i32)
    (i32.wrap_i64 (return (i32.const 41)))
  )

  (func (export "as-memory.grow-size") (result i32)
    (memory.grow (return (i32.const 40)))
  )
)
"#
    );
}

#[test]
fn stack() {
    assert_snapshot!(
        r#"
(module
  (func (export "fac-expr") (param $n i64) (result i64)
    (local $i i64)
    (local $res i64)
    (local.set $i (local.get $n))
    (local.set $res (i64.const 1))
    (block $done
      (loop $loop
        (if
          (i64.eq (local.get $i) (i64.const 0))
          (then (br $done))
          (else
            (local.set $res (i64.mul (local.get $i) (local.get $res)))
            (local.set $i (i64.sub (local.get $i) (i64.const 1)))
          )
        )
        (br $loop)
      )
    )
    (local.get $res)
  )

  (func (export "fac-stack") (param $n i64) (result i64)
    (local $i i64)
    (local $res i64)
    (local.get $n)
    (local.set $i)
    (i64.const 1)
    (local.set $res)
    (block $done
      (loop $loop
        (local.get $i)
        (i64.const 0)
        (i64.eq)
        (if
          (then (br $done))
          (else
            (local.get $i)
            (local.get $res)
            (i64.mul)
            (local.set $res)
            (local.get $i)
            (i64.const 1)
            (i64.sub)
            (local.set $i)
          )
        )
        (br $loop)
      )
    )
    (local.get $res)
  )

  (func (export "fac-stack-raw") (param $n i64) (result i64)
    (local $i i64)
    (local $res i64)
    local.get $n
    local.set $i
    i64.const 1
    local.set $res
    block $done
      loop $loop
        local.get $i
        i64.const 0
        i64.eq
        if $body
          br $done
        else $body
          local.get $i
          local.get $res
          i64.mul
          local.set $res
          local.get $i
          i64.const 1
          i64.sub
          local.set $i
        end $body
        br $loop
      end $loop
    end $done
    local.get $res
  )

  (func (export "fac-mixed") (param $n i64) (result i64)
    (local $i i64)
    (local $res i64)
    (local.set $i (local.get $n))
    (local.set $res (i64.const 1))
    (block $done
      (loop $loop
        (i64.eq (local.get $i) (i64.const 0))
        (if
          (then (br $done))
          (else
            (i64.mul (local.get $i) (local.get $res))
            (local.set $res)
            (i64.sub (local.get $i) (i64.const 1))
            (local.set $i)
          )
        )
        (br $loop)
      )
    )
    (local.get $res)
  )

  (func (export "fac-mixed-raw") (param $n i64) (result i64)
    (local $i i64)
    (local $res i64)
    (local.set $i (local.get $n))
    (local.set $res (i64.const 1))
    block $done
      loop $loop
        (i64.eq (local.get $i) (i64.const 0))
        if
          br $done
        else
          (i64.mul (local.get $i) (local.get $res))
          local.set $res
          (i64.sub (local.get $i) (i64.const 1))
          local.set $i
        end
        br $loop
      end
    end
    local.get $res
  )

  (global $temp (mut i32) (i32.const 0))
  (func $add_one_to_global (result i32)
    (local i32)
    (global.set $temp (i32.add (i32.const 1) (global.get $temp)))
    (global.get $temp)
  )
  (func $add_one_to_global_and_drop
    (drop (call $add_one_to_global))
  )
  (func (export "not-quite-a-tree") (result i32)
    call $add_one_to_global
    call $add_one_to_global
    call $add_one_to_global_and_drop
    i32.add
  )
)
"#
    );
}

#[test]
fn stack_guard_page() {
    assert_snapshot!(
        r#"
(module
  (memory 1)
  (export "test-guard-page-skip" (func $test-guard-page-skip))

  (func $test-guard-page-skip
    (param $depth i32)
    (if (i32.eq (local.get $depth) (i32.const 0))
      (then (call $function-with-many-locals))
      (else (call $test-guard-page-skip (i32.sub (local.get $depth) (i32.const 1))))
    )
  )

  (func $function-with-many-locals

    ;; 1056 i64 = 8448 bytes of locals
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x000-0x007
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x008-0x00f
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x010-0x017
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x018-0x01f
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x020-0x027
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x028-0x02f
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x030-0x037
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x038-0x03f
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x040-0x047
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x048-0x04f
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x050-0x057
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x058-0x05f
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x060-0x067
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x068-0x06f
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x070-0x077
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x078-0x07f
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x080-0x087
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x088-0x08f
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x090-0x097
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x098-0x09f
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x0a0-0x0a7
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x0a8-0x0af
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x0b0-0x0b7
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x0b8-0x0bf
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x0c0-0x0c7
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x0c8-0x0cf
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x0d0-0x0d7
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x0d8-0x0df
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x0e0-0x0e7
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x0e8-0x0ef
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x0f0-0x0f7
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x0f8-0x0ff

    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x100-0x107
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x108-0x10f
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x110-0x117
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x118-0x11f
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x120-0x127
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x128-0x12f
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x130-0x137
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x138-0x13f
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x140-0x147
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x148-0x14f
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x150-0x157
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x158-0x15f
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x160-0x167
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x168-0x16f
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x170-0x177
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x178-0x17f
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x180-0x187
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x188-0x18f
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x190-0x197
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x198-0x19f
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x1a0-0x1a7
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x1a8-0x1af
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x1b0-0x1b7
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x1b8-0x1bf
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x1c0-0x1c7
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x1c8-0x1cf
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x1d0-0x1d7
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x1d8-0x1df
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x1e0-0x1e7
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x1e8-0x1ef
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x1f0-0x1f7
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x1f8-0x1ff

    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x200-0x207
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x208-0x20f
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x210-0x217
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x218-0x21f
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x220-0x227
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x228-0x22f
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x230-0x237
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x238-0x23f
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x240-0x247
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x248-0x24f
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x250-0x257
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x258-0x25f
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x260-0x267
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x268-0x26f
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x270-0x277
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x278-0x27f
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x280-0x287
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x288-0x28f
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x290-0x297
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x298-0x29f
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x2a0-0x2a7
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x2a8-0x2af
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x2b0-0x2b7
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x2b8-0x2bf
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x2c0-0x2c7
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x2c8-0x2cf
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x2d0-0x2d7
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x2d8-0x2df
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x2e0-0x2e7
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x2e8-0x2ef
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x2f0-0x2f7
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x2f8-0x2ff

    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x300-0x307
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x308-0x30f
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x310-0x317
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x318-0x31f
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x320-0x327
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x328-0x32f
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x330-0x337
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x338-0x33f
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x340-0x347
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x348-0x34f
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x350-0x357
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x358-0x35f
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x360-0x367
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x368-0x36f
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x370-0x377
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x378-0x37f
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x380-0x387
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x388-0x38f
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x390-0x397
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x398-0x39f
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x3a0-0x3a7
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x3a8-0x3af
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x3b0-0x3b7
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x3b8-0x3bf
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x3c0-0x3c7
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x3c8-0x3cf
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x3d0-0x3d7
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x3d8-0x3df
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x3e0-0x3e7
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x3e8-0x3ef
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x3f0-0x3f7
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x3f8-0x3ff

    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x400-0x407
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x408-0x40f
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x410-0x417
    (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) (local i64) ;; 0x418-0x41f

    ;; recurse first to try to make the callee access the stack below the space allocated for the locals before the locals themselves have been initialized.
    (call $function-with-many-locals)

    ;; load from memory into the locals
    (local.set 0x000 (i64.load offset=0x000 align=1 (i32.const 0)))
    (local.set 0x001 (i64.load offset=0x001 align=1 (i32.const 0)))
    (local.set 0x002 (i64.load offset=0x002 align=1 (i32.const 0)))
    (local.set 0x003 (i64.load offset=0x003 align=1 (i32.const 0)))
    (local.set 0x004 (i64.load offset=0x004 align=1 (i32.const 0)))
    (local.set 0x005 (i64.load offset=0x005 align=1 (i32.const 0)))
    (local.set 0x006 (i64.load offset=0x006 align=1 (i32.const 0)))
    (local.set 0x007 (i64.load offset=0x007 align=1 (i32.const 0)))
    (local.set 0x008 (i64.load offset=0x008 align=1 (i32.const 0)))
    (local.set 0x009 (i64.load offset=0x009 align=1 (i32.const 0)))
    (local.set 0x00a (i64.load offset=0x00a align=1 (i32.const 0)))
    (local.set 0x00b (i64.load offset=0x00b align=1 (i32.const 0)))
    (local.set 0x00c (i64.load offset=0x00c align=1 (i32.const 0)))
    (local.set 0x00d (i64.load offset=0x00d align=1 (i32.const 0)))
    (local.set 0x00e (i64.load offset=0x00e align=1 (i32.const 0)))
    (local.set 0x00f (i64.load offset=0x00f align=1 (i32.const 0)))
    (local.set 0x010 (i64.load offset=0x010 align=1 (i32.const 0)))
    (local.set 0x011 (i64.load offset=0x011 align=1 (i32.const 0)))
    (local.set 0x012 (i64.load offset=0x012 align=1 (i32.const 0)))
    (local.set 0x013 (i64.load offset=0x013 align=1 (i32.const 0)))
    (local.set 0x014 (i64.load offset=0x014 align=1 (i32.const 0)))
    (local.set 0x015 (i64.load offset=0x015 align=1 (i32.const 0)))
    (local.set 0x016 (i64.load offset=0x016 align=1 (i32.const 0)))
    (local.set 0x017 (i64.load offset=0x017 align=1 (i32.const 0)))
    (local.set 0x018 (i64.load offset=0x018 align=1 (i32.const 0)))
    (local.set 0x019 (i64.load offset=0x019 align=1 (i32.const 0)))
    (local.set 0x01a (i64.load offset=0x01a align=1 (i32.const 0)))
    (local.set 0x01b (i64.load offset=0x01b align=1 (i32.const 0)))
    (local.set 0x01c (i64.load offset=0x01c align=1 (i32.const 0)))
    (local.set 0x01d (i64.load offset=0x01d align=1 (i32.const 0)))
    (local.set 0x01e (i64.load offset=0x01e align=1 (i32.const 0)))
    (local.set 0x01f (i64.load offset=0x01f align=1 (i32.const 0)))
    (local.set 0x020 (i64.load offset=0x020 align=1 (i32.const 0)))
    (local.set 0x021 (i64.load offset=0x021 align=1 (i32.const 0)))
    (local.set 0x022 (i64.load offset=0x022 align=1 (i32.const 0)))
    (local.set 0x023 (i64.load offset=0x023 align=1 (i32.const 0)))
    (local.set 0x024 (i64.load offset=0x024 align=1 (i32.const 0)))
    (local.set 0x025 (i64.load offset=0x025 align=1 (i32.const 0)))
    (local.set 0x026 (i64.load offset=0x026 align=1 (i32.const 0)))
    (local.set 0x027 (i64.load offset=0x027 align=1 (i32.const 0)))
    (local.set 0x028 (i64.load offset=0x028 align=1 (i32.const 0)))
    (local.set 0x029 (i64.load offset=0x029 align=1 (i32.const 0)))
    (local.set 0x02a (i64.load offset=0x02a align=1 (i32.const 0)))
    (local.set 0x02b (i64.load offset=0x02b align=1 (i32.const 0)))
    (local.set 0x02c (i64.load offset=0x02c align=1 (i32.const 0)))
    (local.set 0x02d (i64.load offset=0x02d align=1 (i32.const 0)))
    (local.set 0x02e (i64.load offset=0x02e align=1 (i32.const 0)))
    (local.set 0x02f (i64.load offset=0x02f align=1 (i32.const 0)))
    (local.set 0x030 (i64.load offset=0x030 align=1 (i32.const 0)))
    (local.set 0x031 (i64.load offset=0x031 align=1 (i32.const 0)))
    (local.set 0x032 (i64.load offset=0x032 align=1 (i32.const 0)))
    (local.set 0x033 (i64.load offset=0x033 align=1 (i32.const 0)))
    (local.set 0x034 (i64.load offset=0x034 align=1 (i32.const 0)))
    (local.set 0x035 (i64.load offset=0x035 align=1 (i32.const 0)))
    (local.set 0x036 (i64.load offset=0x036 align=1 (i32.const 0)))
    (local.set 0x037 (i64.load offset=0x037 align=1 (i32.const 0)))
    (local.set 0x038 (i64.load offset=0x038 align=1 (i32.const 0)))
    (local.set 0x039 (i64.load offset=0x039 align=1 (i32.const 0)))
    (local.set 0x03a (i64.load offset=0x03a align=1 (i32.const 0)))
    (local.set 0x03b (i64.load offset=0x03b align=1 (i32.const 0)))
    (local.set 0x03c (i64.load offset=0x03c align=1 (i32.const 0)))
    (local.set 0x03d (i64.load offset=0x03d align=1 (i32.const 0)))
    (local.set 0x03e (i64.load offset=0x03e align=1 (i32.const 0)))
    (local.set 0x03f (i64.load offset=0x03f align=1 (i32.const 0)))
    (local.set 0x040 (i64.load offset=0x040 align=1 (i32.const 0)))
    (local.set 0x041 (i64.load offset=0x041 align=1 (i32.const 0)))
    (local.set 0x042 (i64.load offset=0x042 align=1 (i32.const 0)))
    (local.set 0x043 (i64.load offset=0x043 align=1 (i32.const 0)))
    (local.set 0x044 (i64.load offset=0x044 align=1 (i32.const 0)))
    (local.set 0x045 (i64.load offset=0x045 align=1 (i32.const 0)))
    (local.set 0x046 (i64.load offset=0x046 align=1 (i32.const 0)))
    (local.set 0x047 (i64.load offset=0x047 align=1 (i32.const 0)))
    (local.set 0x048 (i64.load offset=0x048 align=1 (i32.const 0)))
    (local.set 0x049 (i64.load offset=0x049 align=1 (i32.const 0)))
    (local.set 0x04a (i64.load offset=0x04a align=1 (i32.const 0)))
    (local.set 0x04b (i64.load offset=0x04b align=1 (i32.const 0)))
    (local.set 0x04c (i64.load offset=0x04c align=1 (i32.const 0)))
    (local.set 0x04d (i64.load offset=0x04d align=1 (i32.const 0)))
    (local.set 0x04e (i64.load offset=0x04e align=1 (i32.const 0)))
    (local.set 0x04f (i64.load offset=0x04f align=1 (i32.const 0)))
    (local.set 0x050 (i64.load offset=0x050 align=1 (i32.const 0)))
    (local.set 0x051 (i64.load offset=0x051 align=1 (i32.const 0)))
    (local.set 0x052 (i64.load offset=0x052 align=1 (i32.const 0)))
    (local.set 0x053 (i64.load offset=0x053 align=1 (i32.const 0)))
    (local.set 0x054 (i64.load offset=0x054 align=1 (i32.const 0)))
    (local.set 0x055 (i64.load offset=0x055 align=1 (i32.const 0)))
    (local.set 0x056 (i64.load offset=0x056 align=1 (i32.const 0)))
    (local.set 0x057 (i64.load offset=0x057 align=1 (i32.const 0)))
    (local.set 0x058 (i64.load offset=0x058 align=1 (i32.const 0)))
    (local.set 0x059 (i64.load offset=0x059 align=1 (i32.const 0)))
    (local.set 0x05a (i64.load offset=0x05a align=1 (i32.const 0)))
    (local.set 0x05b (i64.load offset=0x05b align=1 (i32.const 0)))
    (local.set 0x05c (i64.load offset=0x05c align=1 (i32.const 0)))
    (local.set 0x05d (i64.load offset=0x05d align=1 (i32.const 0)))
    (local.set 0x05e (i64.load offset=0x05e align=1 (i32.const 0)))
    (local.set 0x05f (i64.load offset=0x05f align=1 (i32.const 0)))
    (local.set 0x060 (i64.load offset=0x060 align=1 (i32.const 0)))
    (local.set 0x061 (i64.load offset=0x061 align=1 (i32.const 0)))
    (local.set 0x062 (i64.load offset=0x062 align=1 (i32.const 0)))
    (local.set 0x063 (i64.load offset=0x063 align=1 (i32.const 0)))
    (local.set 0x064 (i64.load offset=0x064 align=1 (i32.const 0)))
    (local.set 0x065 (i64.load offset=0x065 align=1 (i32.const 0)))
    (local.set 0x066 (i64.load offset=0x066 align=1 (i32.const 0)))
    (local.set 0x067 (i64.load offset=0x067 align=1 (i32.const 0)))
    (local.set 0x068 (i64.load offset=0x068 align=1 (i32.const 0)))
    (local.set 0x069 (i64.load offset=0x069 align=1 (i32.const 0)))
    (local.set 0x06a (i64.load offset=0x06a align=1 (i32.const 0)))
    (local.set 0x06b (i64.load offset=0x06b align=1 (i32.const 0)))
    (local.set 0x06c (i64.load offset=0x06c align=1 (i32.const 0)))
    (local.set 0x06d (i64.load offset=0x06d align=1 (i32.const 0)))
    (local.set 0x06e (i64.load offset=0x06e align=1 (i32.const 0)))
    (local.set 0x06f (i64.load offset=0x06f align=1 (i32.const 0)))
    (local.set 0x070 (i64.load offset=0x070 align=1 (i32.const 0)))
    (local.set 0x071 (i64.load offset=0x071 align=1 (i32.const 0)))
    (local.set 0x072 (i64.load offset=0x072 align=1 (i32.const 0)))
    (local.set 0x073 (i64.load offset=0x073 align=1 (i32.const 0)))
    (local.set 0x074 (i64.load offset=0x074 align=1 (i32.const 0)))
    (local.set 0x075 (i64.load offset=0x075 align=1 (i32.const 0)))
    (local.set 0x076 (i64.load offset=0x076 align=1 (i32.const 0)))
    (local.set 0x077 (i64.load offset=0x077 align=1 (i32.const 0)))
    (local.set 0x078 (i64.load offset=0x078 align=1 (i32.const 0)))
    (local.set 0x079 (i64.load offset=0x079 align=1 (i32.const 0)))
    (local.set 0x07a (i64.load offset=0x07a align=1 (i32.const 0)))
    (local.set 0x07b (i64.load offset=0x07b align=1 (i32.const 0)))
    (local.set 0x07c (i64.load offset=0x07c align=1 (i32.const 0)))
    (local.set 0x07d (i64.load offset=0x07d align=1 (i32.const 0)))
    (local.set 0x07e (i64.load offset=0x07e align=1 (i32.const 0)))
    (local.set 0x07f (i64.load offset=0x07f align=1 (i32.const 0)))
    (local.set 0x080 (i64.load offset=0x080 align=1 (i32.const 0)))
    (local.set 0x081 (i64.load offset=0x081 align=1 (i32.const 0)))
    (local.set 0x082 (i64.load offset=0x082 align=1 (i32.const 0)))
    (local.set 0x083 (i64.load offset=0x083 align=1 (i32.const 0)))
    (local.set 0x084 (i64.load offset=0x084 align=1 (i32.const 0)))
    (local.set 0x085 (i64.load offset=0x085 align=1 (i32.const 0)))
    (local.set 0x086 (i64.load offset=0x086 align=1 (i32.const 0)))
    (local.set 0x087 (i64.load offset=0x087 align=1 (i32.const 0)))
    (local.set 0x088 (i64.load offset=0x088 align=1 (i32.const 0)))
    (local.set 0x089 (i64.load offset=0x089 align=1 (i32.const 0)))
    (local.set 0x08a (i64.load offset=0x08a align=1 (i32.const 0)))
    (local.set 0x08b (i64.load offset=0x08b align=1 (i32.const 0)))
    (local.set 0x08c (i64.load offset=0x08c align=1 (i32.const 0)))
    (local.set 0x08d (i64.load offset=0x08d align=1 (i32.const 0)))
    (local.set 0x08e (i64.load offset=0x08e align=1 (i32.const 0)))
    (local.set 0x08f (i64.load offset=0x08f align=1 (i32.const 0)))
    (local.set 0x090 (i64.load offset=0x090 align=1 (i32.const 0)))
    (local.set 0x091 (i64.load offset=0x091 align=1 (i32.const 0)))
    (local.set 0x092 (i64.load offset=0x092 align=1 (i32.const 0)))
    (local.set 0x093 (i64.load offset=0x093 align=1 (i32.const 0)))
    (local.set 0x094 (i64.load offset=0x094 align=1 (i32.const 0)))
    (local.set 0x095 (i64.load offset=0x095 align=1 (i32.const 0)))
    (local.set 0x096 (i64.load offset=0x096 align=1 (i32.const 0)))
    (local.set 0x097 (i64.load offset=0x097 align=1 (i32.const 0)))
    (local.set 0x098 (i64.load offset=0x098 align=1 (i32.const 0)))
    (local.set 0x099 (i64.load offset=0x099 align=1 (i32.const 0)))
    (local.set 0x09a (i64.load offset=0x09a align=1 (i32.const 0)))
    (local.set 0x09b (i64.load offset=0x09b align=1 (i32.const 0)))
    (local.set 0x09c (i64.load offset=0x09c align=1 (i32.const 0)))
    (local.set 0x09d (i64.load offset=0x09d align=1 (i32.const 0)))
    (local.set 0x09e (i64.load offset=0x09e align=1 (i32.const 0)))
    (local.set 0x09f (i64.load offset=0x09f align=1 (i32.const 0)))
    (local.set 0x0a0 (i64.load offset=0x0a0 align=1 (i32.const 0)))
    (local.set 0x0a1 (i64.load offset=0x0a1 align=1 (i32.const 0)))
    (local.set 0x0a2 (i64.load offset=0x0a2 align=1 (i32.const 0)))
    (local.set 0x0a3 (i64.load offset=0x0a3 align=1 (i32.const 0)))
    (local.set 0x0a4 (i64.load offset=0x0a4 align=1 (i32.const 0)))
    (local.set 0x0a5 (i64.load offset=0x0a5 align=1 (i32.const 0)))
    (local.set 0x0a6 (i64.load offset=0x0a6 align=1 (i32.const 0)))
    (local.set 0x0a7 (i64.load offset=0x0a7 align=1 (i32.const 0)))
    (local.set 0x0a8 (i64.load offset=0x0a8 align=1 (i32.const 0)))
    (local.set 0x0a9 (i64.load offset=0x0a9 align=1 (i32.const 0)))
    (local.set 0x0aa (i64.load offset=0x0aa align=1 (i32.const 0)))
    (local.set 0x0ab (i64.load offset=0x0ab align=1 (i32.const 0)))
    (local.set 0x0ac (i64.load offset=0x0ac align=1 (i32.const 0)))
    (local.set 0x0ad (i64.load offset=0x0ad align=1 (i32.const 0)))
    (local.set 0x0ae (i64.load offset=0x0ae align=1 (i32.const 0)))
    (local.set 0x0af (i64.load offset=0x0af align=1 (i32.const 0)))
    (local.set 0x0b0 (i64.load offset=0x0b0 align=1 (i32.const 0)))
    (local.set 0x0b1 (i64.load offset=0x0b1 align=1 (i32.const 0)))
    (local.set 0x0b2 (i64.load offset=0x0b2 align=1 (i32.const 0)))
    (local.set 0x0b3 (i64.load offset=0x0b3 align=1 (i32.const 0)))
    (local.set 0x0b4 (i64.load offset=0x0b4 align=1 (i32.const 0)))
    (local.set 0x0b5 (i64.load offset=0x0b5 align=1 (i32.const 0)))
    (local.set 0x0b6 (i64.load offset=0x0b6 align=1 (i32.const 0)))
    (local.set 0x0b7 (i64.load offset=0x0b7 align=1 (i32.const 0)))
    (local.set 0x0b8 (i64.load offset=0x0b8 align=1 (i32.const 0)))
    (local.set 0x0b9 (i64.load offset=0x0b9 align=1 (i32.const 0)))
    (local.set 0x0ba (i64.load offset=0x0ba align=1 (i32.const 0)))
    (local.set 0x0bb (i64.load offset=0x0bb align=1 (i32.const 0)))
    (local.set 0x0bc (i64.load offset=0x0bc align=1 (i32.const 0)))
    (local.set 0x0bd (i64.load offset=0x0bd align=1 (i32.const 0)))
    (local.set 0x0be (i64.load offset=0x0be align=1 (i32.const 0)))
    (local.set 0x0bf (i64.load offset=0x0bf align=1 (i32.const 0)))
    (local.set 0x0c0 (i64.load offset=0x0c0 align=1 (i32.const 0)))
    (local.set 0x0c1 (i64.load offset=0x0c1 align=1 (i32.const 0)))
    (local.set 0x0c2 (i64.load offset=0x0c2 align=1 (i32.const 0)))
    (local.set 0x0c3 (i64.load offset=0x0c3 align=1 (i32.const 0)))
    (local.set 0x0c4 (i64.load offset=0x0c4 align=1 (i32.const 0)))
    (local.set 0x0c5 (i64.load offset=0x0c5 align=1 (i32.const 0)))
    (local.set 0x0c6 (i64.load offset=0x0c6 align=1 (i32.const 0)))
    (local.set 0x0c7 (i64.load offset=0x0c7 align=1 (i32.const 0)))
    (local.set 0x0c8 (i64.load offset=0x0c8 align=1 (i32.const 0)))
    (local.set 0x0c9 (i64.load offset=0x0c9 align=1 (i32.const 0)))
    (local.set 0x0ca (i64.load offset=0x0ca align=1 (i32.const 0)))
    (local.set 0x0cb (i64.load offset=0x0cb align=1 (i32.const 0)))
    (local.set 0x0cc (i64.load offset=0x0cc align=1 (i32.const 0)))
    (local.set 0x0cd (i64.load offset=0x0cd align=1 (i32.const 0)))
    (local.set 0x0ce (i64.load offset=0x0ce align=1 (i32.const 0)))
    (local.set 0x0cf (i64.load offset=0x0cf align=1 (i32.const 0)))
    (local.set 0x0d0 (i64.load offset=0x0d0 align=1 (i32.const 0)))
    (local.set 0x0d1 (i64.load offset=0x0d1 align=1 (i32.const 0)))
    (local.set 0x0d2 (i64.load offset=0x0d2 align=1 (i32.const 0)))
    (local.set 0x0d3 (i64.load offset=0x0d3 align=1 (i32.const 0)))
    (local.set 0x0d4 (i64.load offset=0x0d4 align=1 (i32.const 0)))
    (local.set 0x0d5 (i64.load offset=0x0d5 align=1 (i32.const 0)))
    (local.set 0x0d6 (i64.load offset=0x0d6 align=1 (i32.const 0)))
    (local.set 0x0d7 (i64.load offset=0x0d7 align=1 (i32.const 0)))
    (local.set 0x0d8 (i64.load offset=0x0d8 align=1 (i32.const 0)))
    (local.set 0x0d9 (i64.load offset=0x0d9 align=1 (i32.const 0)))
    (local.set 0x0da (i64.load offset=0x0da align=1 (i32.const 0)))
    (local.set 0x0db (i64.load offset=0x0db align=1 (i32.const 0)))
    (local.set 0x0dc (i64.load offset=0x0dc align=1 (i32.const 0)))
    (local.set 0x0dd (i64.load offset=0x0dd align=1 (i32.const 0)))
    (local.set 0x0de (i64.load offset=0x0de align=1 (i32.const 0)))
    (local.set 0x0df (i64.load offset=0x0df align=1 (i32.const 0)))
    (local.set 0x0e0 (i64.load offset=0x0e0 align=1 (i32.const 0)))
    (local.set 0x0e1 (i64.load offset=0x0e1 align=1 (i32.const 0)))
    (local.set 0x0e2 (i64.load offset=0x0e2 align=1 (i32.const 0)))
    (local.set 0x0e3 (i64.load offset=0x0e3 align=1 (i32.const 0)))
    (local.set 0x0e4 (i64.load offset=0x0e4 align=1 (i32.const 0)))
    (local.set 0x0e5 (i64.load offset=0x0e5 align=1 (i32.const 0)))
    (local.set 0x0e6 (i64.load offset=0x0e6 align=1 (i32.const 0)))
    (local.set 0x0e7 (i64.load offset=0x0e7 align=1 (i32.const 0)))
    (local.set 0x0e8 (i64.load offset=0x0e8 align=1 (i32.const 0)))
    (local.set 0x0e9 (i64.load offset=0x0e9 align=1 (i32.const 0)))
    (local.set 0x0ea (i64.load offset=0x0ea align=1 (i32.const 0)))
    (local.set 0x0eb (i64.load offset=0x0eb align=1 (i32.const 0)))
    (local.set 0x0ec (i64.load offset=0x0ec align=1 (i32.const 0)))
    (local.set 0x0ed (i64.load offset=0x0ed align=1 (i32.const 0)))
    (local.set 0x0ee (i64.load offset=0x0ee align=1 (i32.const 0)))
    (local.set 0x0ef (i64.load offset=0x0ef align=1 (i32.const 0)))
    (local.set 0x0f0 (i64.load offset=0x0f0 align=1 (i32.const 0)))
    (local.set 0x0f1 (i64.load offset=0x0f1 align=1 (i32.const 0)))
    (local.set 0x0f2 (i64.load offset=0x0f2 align=1 (i32.const 0)))
    (local.set 0x0f3 (i64.load offset=0x0f3 align=1 (i32.const 0)))
    (local.set 0x0f4 (i64.load offset=0x0f4 align=1 (i32.const 0)))
    (local.set 0x0f5 (i64.load offset=0x0f5 align=1 (i32.const 0)))
    (local.set 0x0f6 (i64.load offset=0x0f6 align=1 (i32.const 0)))
    (local.set 0x0f7 (i64.load offset=0x0f7 align=1 (i32.const 0)))
    (local.set 0x0f8 (i64.load offset=0x0f8 align=1 (i32.const 0)))
    (local.set 0x0f9 (i64.load offset=0x0f9 align=1 (i32.const 0)))
    (local.set 0x0fa (i64.load offset=0x0fa align=1 (i32.const 0)))
    (local.set 0x0fb (i64.load offset=0x0fb align=1 (i32.const 0)))
    (local.set 0x0fc (i64.load offset=0x0fc align=1 (i32.const 0)))
    (local.set 0x0fd (i64.load offset=0x0fd align=1 (i32.const 0)))
    (local.set 0x0fe (i64.load offset=0x0fe align=1 (i32.const 0)))
    (local.set 0x0ff (i64.load offset=0x0ff align=1 (i32.const 0)))
    (local.set 0x100 (i64.load offset=0x100 align=1 (i32.const 0)))
    (local.set 0x101 (i64.load offset=0x101 align=1 (i32.const 0)))
    (local.set 0x102 (i64.load offset=0x102 align=1 (i32.const 0)))
    (local.set 0x103 (i64.load offset=0x103 align=1 (i32.const 0)))
    (local.set 0x104 (i64.load offset=0x104 align=1 (i32.const 0)))
    (local.set 0x105 (i64.load offset=0x105 align=1 (i32.const 0)))
    (local.set 0x106 (i64.load offset=0x106 align=1 (i32.const 0)))
    (local.set 0x107 (i64.load offset=0x107 align=1 (i32.const 0)))
    (local.set 0x108 (i64.load offset=0x108 align=1 (i32.const 0)))
    (local.set 0x109 (i64.load offset=0x109 align=1 (i32.const 0)))
    (local.set 0x10a (i64.load offset=0x10a align=1 (i32.const 0)))
    (local.set 0x10b (i64.load offset=0x10b align=1 (i32.const 0)))
    (local.set 0x10c (i64.load offset=0x10c align=1 (i32.const 0)))
    (local.set 0x10d (i64.load offset=0x10d align=1 (i32.const 0)))
    (local.set 0x10e (i64.load offset=0x10e align=1 (i32.const 0)))
    (local.set 0x10f (i64.load offset=0x10f align=1 (i32.const 0)))
    (local.set 0x110 (i64.load offset=0x110 align=1 (i32.const 0)))
    (local.set 0x111 (i64.load offset=0x111 align=1 (i32.const 0)))
    (local.set 0x112 (i64.load offset=0x112 align=1 (i32.const 0)))
    (local.set 0x113 (i64.load offset=0x113 align=1 (i32.const 0)))
    (local.set 0x114 (i64.load offset=0x114 align=1 (i32.const 0)))
    (local.set 0x115 (i64.load offset=0x115 align=1 (i32.const 0)))
    (local.set 0x116 (i64.load offset=0x116 align=1 (i32.const 0)))
    (local.set 0x117 (i64.load offset=0x117 align=1 (i32.const 0)))
    (local.set 0x118 (i64.load offset=0x118 align=1 (i32.const 0)))
    (local.set 0x119 (i64.load offset=0x119 align=1 (i32.const 0)))
    (local.set 0x11a (i64.load offset=0x11a align=1 (i32.const 0)))
    (local.set 0x11b (i64.load offset=0x11b align=1 (i32.const 0)))
    (local.set 0x11c (i64.load offset=0x11c align=1 (i32.const 0)))
    (local.set 0x11d (i64.load offset=0x11d align=1 (i32.const 0)))
    (local.set 0x11e (i64.load offset=0x11e align=1 (i32.const 0)))
    (local.set 0x11f (i64.load offset=0x11f align=1 (i32.const 0)))
    (local.set 0x120 (i64.load offset=0x120 align=1 (i32.const 0)))
    (local.set 0x121 (i64.load offset=0x121 align=1 (i32.const 0)))
    (local.set 0x122 (i64.load offset=0x122 align=1 (i32.const 0)))
    (local.set 0x123 (i64.load offset=0x123 align=1 (i32.const 0)))
    (local.set 0x124 (i64.load offset=0x124 align=1 (i32.const 0)))
    (local.set 0x125 (i64.load offset=0x125 align=1 (i32.const 0)))
    (local.set 0x126 (i64.load offset=0x126 align=1 (i32.const 0)))
    (local.set 0x127 (i64.load offset=0x127 align=1 (i32.const 0)))
    (local.set 0x128 (i64.load offset=0x128 align=1 (i32.const 0)))
    (local.set 0x129 (i64.load offset=0x129 align=1 (i32.const 0)))
    (local.set 0x12a (i64.load offset=0x12a align=1 (i32.const 0)))
    (local.set 0x12b (i64.load offset=0x12b align=1 (i32.const 0)))
    (local.set 0x12c (i64.load offset=0x12c align=1 (i32.const 0)))
    (local.set 0x12d (i64.load offset=0x12d align=1 (i32.const 0)))
    (local.set 0x12e (i64.load offset=0x12e align=1 (i32.const 0)))
    (local.set 0x12f (i64.load offset=0x12f align=1 (i32.const 0)))
    (local.set 0x130 (i64.load offset=0x130 align=1 (i32.const 0)))
    (local.set 0x131 (i64.load offset=0x131 align=1 (i32.const 0)))
    (local.set 0x132 (i64.load offset=0x132 align=1 (i32.const 0)))
    (local.set 0x133 (i64.load offset=0x133 align=1 (i32.const 0)))
    (local.set 0x134 (i64.load offset=0x134 align=1 (i32.const 0)))
    (local.set 0x135 (i64.load offset=0x135 align=1 (i32.const 0)))
    (local.set 0x136 (i64.load offset=0x136 align=1 (i32.const 0)))
    (local.set 0x137 (i64.load offset=0x137 align=1 (i32.const 0)))
    (local.set 0x138 (i64.load offset=0x138 align=1 (i32.const 0)))
    (local.set 0x139 (i64.load offset=0x139 align=1 (i32.const 0)))
    (local.set 0x13a (i64.load offset=0x13a align=1 (i32.const 0)))
    (local.set 0x13b (i64.load offset=0x13b align=1 (i32.const 0)))
    (local.set 0x13c (i64.load offset=0x13c align=1 (i32.const 0)))
    (local.set 0x13d (i64.load offset=0x13d align=1 (i32.const 0)))
    (local.set 0x13e (i64.load offset=0x13e align=1 (i32.const 0)))
    (local.set 0x13f (i64.load offset=0x13f align=1 (i32.const 0)))
    (local.set 0x140 (i64.load offset=0x140 align=1 (i32.const 0)))
    (local.set 0x141 (i64.load offset=0x141 align=1 (i32.const 0)))
    (local.set 0x142 (i64.load offset=0x142 align=1 (i32.const 0)))
    (local.set 0x143 (i64.load offset=0x143 align=1 (i32.const 0)))
    (local.set 0x144 (i64.load offset=0x144 align=1 (i32.const 0)))
    (local.set 0x145 (i64.load offset=0x145 align=1 (i32.const 0)))
    (local.set 0x146 (i64.load offset=0x146 align=1 (i32.const 0)))
    (local.set 0x147 (i64.load offset=0x147 align=1 (i32.const 0)))
    (local.set 0x148 (i64.load offset=0x148 align=1 (i32.const 0)))
    (local.set 0x149 (i64.load offset=0x149 align=1 (i32.const 0)))
    (local.set 0x14a (i64.load offset=0x14a align=1 (i32.const 0)))
    (local.set 0x14b (i64.load offset=0x14b align=1 (i32.const 0)))
    (local.set 0x14c (i64.load offset=0x14c align=1 (i32.const 0)))
    (local.set 0x14d (i64.load offset=0x14d align=1 (i32.const 0)))
    (local.set 0x14e (i64.load offset=0x14e align=1 (i32.const 0)))
    (local.set 0x14f (i64.load offset=0x14f align=1 (i32.const 0)))
    (local.set 0x150 (i64.load offset=0x150 align=1 (i32.const 0)))
    (local.set 0x151 (i64.load offset=0x151 align=1 (i32.const 0)))
    (local.set 0x152 (i64.load offset=0x152 align=1 (i32.const 0)))
    (local.set 0x153 (i64.load offset=0x153 align=1 (i32.const 0)))
    (local.set 0x154 (i64.load offset=0x154 align=1 (i32.const 0)))
    (local.set 0x155 (i64.load offset=0x155 align=1 (i32.const 0)))
    (local.set 0x156 (i64.load offset=0x156 align=1 (i32.const 0)))
    (local.set 0x157 (i64.load offset=0x157 align=1 (i32.const 0)))
    (local.set 0x158 (i64.load offset=0x158 align=1 (i32.const 0)))
    (local.set 0x159 (i64.load offset=0x159 align=1 (i32.const 0)))
    (local.set 0x15a (i64.load offset=0x15a align=1 (i32.const 0)))
    (local.set 0x15b (i64.load offset=0x15b align=1 (i32.const 0)))
    (local.set 0x15c (i64.load offset=0x15c align=1 (i32.const 0)))
    (local.set 0x15d (i64.load offset=0x15d align=1 (i32.const 0)))
    (local.set 0x15e (i64.load offset=0x15e align=1 (i32.const 0)))
    (local.set 0x15f (i64.load offset=0x15f align=1 (i32.const 0)))
    (local.set 0x160 (i64.load offset=0x160 align=1 (i32.const 0)))
    (local.set 0x161 (i64.load offset=0x161 align=1 (i32.const 0)))
    (local.set 0x162 (i64.load offset=0x162 align=1 (i32.const 0)))
    (local.set 0x163 (i64.load offset=0x163 align=1 (i32.const 0)))
    (local.set 0x164 (i64.load offset=0x164 align=1 (i32.const 0)))
    (local.set 0x165 (i64.load offset=0x165 align=1 (i32.const 0)))
    (local.set 0x166 (i64.load offset=0x166 align=1 (i32.const 0)))
    (local.set 0x167 (i64.load offset=0x167 align=1 (i32.const 0)))
    (local.set 0x168 (i64.load offset=0x168 align=1 (i32.const 0)))
    (local.set 0x169 (i64.load offset=0x169 align=1 (i32.const 0)))
    (local.set 0x16a (i64.load offset=0x16a align=1 (i32.const 0)))
    (local.set 0x16b (i64.load offset=0x16b align=1 (i32.const 0)))
    (local.set 0x16c (i64.load offset=0x16c align=1 (i32.const 0)))
    (local.set 0x16d (i64.load offset=0x16d align=1 (i32.const 0)))
    (local.set 0x16e (i64.load offset=0x16e align=1 (i32.const 0)))
    (local.set 0x16f (i64.load offset=0x16f align=1 (i32.const 0)))
    (local.set 0x170 (i64.load offset=0x170 align=1 (i32.const 0)))
    (local.set 0x171 (i64.load offset=0x171 align=1 (i32.const 0)))
    (local.set 0x172 (i64.load offset=0x172 align=1 (i32.const 0)))
    (local.set 0x173 (i64.load offset=0x173 align=1 (i32.const 0)))
    (local.set 0x174 (i64.load offset=0x174 align=1 (i32.const 0)))
    (local.set 0x175 (i64.load offset=0x175 align=1 (i32.const 0)))
    (local.set 0x176 (i64.load offset=0x176 align=1 (i32.const 0)))
    (local.set 0x177 (i64.load offset=0x177 align=1 (i32.const 0)))
    (local.set 0x178 (i64.load offset=0x178 align=1 (i32.const 0)))
    (local.set 0x179 (i64.load offset=0x179 align=1 (i32.const 0)))
    (local.set 0x17a (i64.load offset=0x17a align=1 (i32.const 0)))
    (local.set 0x17b (i64.load offset=0x17b align=1 (i32.const 0)))
    (local.set 0x17c (i64.load offset=0x17c align=1 (i32.const 0)))
    (local.set 0x17d (i64.load offset=0x17d align=1 (i32.const 0)))
    (local.set 0x17e (i64.load offset=0x17e align=1 (i32.const 0)))
    (local.set 0x17f (i64.load offset=0x17f align=1 (i32.const 0)))
    (local.set 0x180 (i64.load offset=0x180 align=1 (i32.const 0)))
    (local.set 0x181 (i64.load offset=0x181 align=1 (i32.const 0)))
    (local.set 0x182 (i64.load offset=0x182 align=1 (i32.const 0)))
    (local.set 0x183 (i64.load offset=0x183 align=1 (i32.const 0)))
    (local.set 0x184 (i64.load offset=0x184 align=1 (i32.const 0)))
    (local.set 0x185 (i64.load offset=0x185 align=1 (i32.const 0)))
    (local.set 0x186 (i64.load offset=0x186 align=1 (i32.const 0)))
    (local.set 0x187 (i64.load offset=0x187 align=1 (i32.const 0)))
    (local.set 0x188 (i64.load offset=0x188 align=1 (i32.const 0)))
    (local.set 0x189 (i64.load offset=0x189 align=1 (i32.const 0)))
    (local.set 0x18a (i64.load offset=0x18a align=1 (i32.const 0)))
    (local.set 0x18b (i64.load offset=0x18b align=1 (i32.const 0)))
    (local.set 0x18c (i64.load offset=0x18c align=1 (i32.const 0)))
    (local.set 0x18d (i64.load offset=0x18d align=1 (i32.const 0)))
    (local.set 0x18e (i64.load offset=0x18e align=1 (i32.const 0)))
    (local.set 0x18f (i64.load offset=0x18f align=1 (i32.const 0)))
    (local.set 0x190 (i64.load offset=0x190 align=1 (i32.const 0)))
    (local.set 0x191 (i64.load offset=0x191 align=1 (i32.const 0)))
    (local.set 0x192 (i64.load offset=0x192 align=1 (i32.const 0)))
    (local.set 0x193 (i64.load offset=0x193 align=1 (i32.const 0)))
    (local.set 0x194 (i64.load offset=0x194 align=1 (i32.const 0)))
    (local.set 0x195 (i64.load offset=0x195 align=1 (i32.const 0)))
    (local.set 0x196 (i64.load offset=0x196 align=1 (i32.const 0)))
    (local.set 0x197 (i64.load offset=0x197 align=1 (i32.const 0)))
    (local.set 0x198 (i64.load offset=0x198 align=1 (i32.const 0)))
    (local.set 0x199 (i64.load offset=0x199 align=1 (i32.const 0)))
    (local.set 0x19a (i64.load offset=0x19a align=1 (i32.const 0)))
    (local.set 0x19b (i64.load offset=0x19b align=1 (i32.const 0)))
    (local.set 0x19c (i64.load offset=0x19c align=1 (i32.const 0)))
    (local.set 0x19d (i64.load offset=0x19d align=1 (i32.const 0)))
    (local.set 0x19e (i64.load offset=0x19e align=1 (i32.const 0)))
    (local.set 0x19f (i64.load offset=0x19f align=1 (i32.const 0)))
    (local.set 0x1a0 (i64.load offset=0x1a0 align=1 (i32.const 0)))
    (local.set 0x1a1 (i64.load offset=0x1a1 align=1 (i32.const 0)))
    (local.set 0x1a2 (i64.load offset=0x1a2 align=1 (i32.const 0)))
    (local.set 0x1a3 (i64.load offset=0x1a3 align=1 (i32.const 0)))
    (local.set 0x1a4 (i64.load offset=0x1a4 align=1 (i32.const 0)))
    (local.set 0x1a5 (i64.load offset=0x1a5 align=1 (i32.const 0)))
    (local.set 0x1a6 (i64.load offset=0x1a6 align=1 (i32.const 0)))
    (local.set 0x1a7 (i64.load offset=0x1a7 align=1 (i32.const 0)))
    (local.set 0x1a8 (i64.load offset=0x1a8 align=1 (i32.const 0)))
    (local.set 0x1a9 (i64.load offset=0x1a9 align=1 (i32.const 0)))
    (local.set 0x1aa (i64.load offset=0x1aa align=1 (i32.const 0)))
    (local.set 0x1ab (i64.load offset=0x1ab align=1 (i32.const 0)))
    (local.set 0x1ac (i64.load offset=0x1ac align=1 (i32.const 0)))
    (local.set 0x1ad (i64.load offset=0x1ad align=1 (i32.const 0)))
    (local.set 0x1ae (i64.load offset=0x1ae align=1 (i32.const 0)))
    (local.set 0x1af (i64.load offset=0x1af align=1 (i32.const 0)))
    (local.set 0x1b0 (i64.load offset=0x1b0 align=1 (i32.const 0)))
    (local.set 0x1b1 (i64.load offset=0x1b1 align=1 (i32.const 0)))
    (local.set 0x1b2 (i64.load offset=0x1b2 align=1 (i32.const 0)))
    (local.set 0x1b3 (i64.load offset=0x1b3 align=1 (i32.const 0)))
    (local.set 0x1b4 (i64.load offset=0x1b4 align=1 (i32.const 0)))
    (local.set 0x1b5 (i64.load offset=0x1b5 align=1 (i32.const 0)))
    (local.set 0x1b6 (i64.load offset=0x1b6 align=1 (i32.const 0)))
    (local.set 0x1b7 (i64.load offset=0x1b7 align=1 (i32.const 0)))
    (local.set 0x1b8 (i64.load offset=0x1b8 align=1 (i32.const 0)))
    (local.set 0x1b9 (i64.load offset=0x1b9 align=1 (i32.const 0)))
    (local.set 0x1ba (i64.load offset=0x1ba align=1 (i32.const 0)))
    (local.set 0x1bb (i64.load offset=0x1bb align=1 (i32.const 0)))
    (local.set 0x1bc (i64.load offset=0x1bc align=1 (i32.const 0)))
    (local.set 0x1bd (i64.load offset=0x1bd align=1 (i32.const 0)))
    (local.set 0x1be (i64.load offset=0x1be align=1 (i32.const 0)))
    (local.set 0x1bf (i64.load offset=0x1bf align=1 (i32.const 0)))
    (local.set 0x1c0 (i64.load offset=0x1c0 align=1 (i32.const 0)))
    (local.set 0x1c1 (i64.load offset=0x1c1 align=1 (i32.const 0)))
    (local.set 0x1c2 (i64.load offset=0x1c2 align=1 (i32.const 0)))
    (local.set 0x1c3 (i64.load offset=0x1c3 align=1 (i32.const 0)))
    (local.set 0x1c4 (i64.load offset=0x1c4 align=1 (i32.const 0)))
    (local.set 0x1c5 (i64.load offset=0x1c5 align=1 (i32.const 0)))
    (local.set 0x1c6 (i64.load offset=0x1c6 align=1 (i32.const 0)))
    (local.set 0x1c7 (i64.load offset=0x1c7 align=1 (i32.const 0)))
    (local.set 0x1c8 (i64.load offset=0x1c8 align=1 (i32.const 0)))
    (local.set 0x1c9 (i64.load offset=0x1c9 align=1 (i32.const 0)))
    (local.set 0x1ca (i64.load offset=0x1ca align=1 (i32.const 0)))
    (local.set 0x1cb (i64.load offset=0x1cb align=1 (i32.const 0)))
    (local.set 0x1cc (i64.load offset=0x1cc align=1 (i32.const 0)))
    (local.set 0x1cd (i64.load offset=0x1cd align=1 (i32.const 0)))
    (local.set 0x1ce (i64.load offset=0x1ce align=1 (i32.const 0)))
    (local.set 0x1cf (i64.load offset=0x1cf align=1 (i32.const 0)))
    (local.set 0x1d0 (i64.load offset=0x1d0 align=1 (i32.const 0)))
    (local.set 0x1d1 (i64.load offset=0x1d1 align=1 (i32.const 0)))
    (local.set 0x1d2 (i64.load offset=0x1d2 align=1 (i32.const 0)))
    (local.set 0x1d3 (i64.load offset=0x1d3 align=1 (i32.const 0)))
    (local.set 0x1d4 (i64.load offset=0x1d4 align=1 (i32.const 0)))
    (local.set 0x1d5 (i64.load offset=0x1d5 align=1 (i32.const 0)))
    (local.set 0x1d6 (i64.load offset=0x1d6 align=1 (i32.const 0)))
    (local.set 0x1d7 (i64.load offset=0x1d7 align=1 (i32.const 0)))
    (local.set 0x1d8 (i64.load offset=0x1d8 align=1 (i32.const 0)))
    (local.set 0x1d9 (i64.load offset=0x1d9 align=1 (i32.const 0)))
    (local.set 0x1da (i64.load offset=0x1da align=1 (i32.const 0)))
    (local.set 0x1db (i64.load offset=0x1db align=1 (i32.const 0)))
    (local.set 0x1dc (i64.load offset=0x1dc align=1 (i32.const 0)))
    (local.set 0x1dd (i64.load offset=0x1dd align=1 (i32.const 0)))
    (local.set 0x1de (i64.load offset=0x1de align=1 (i32.const 0)))
    (local.set 0x1df (i64.load offset=0x1df align=1 (i32.const 0)))
    (local.set 0x1e0 (i64.load offset=0x1e0 align=1 (i32.const 0)))
    (local.set 0x1e1 (i64.load offset=0x1e1 align=1 (i32.const 0)))
    (local.set 0x1e2 (i64.load offset=0x1e2 align=1 (i32.const 0)))
    (local.set 0x1e3 (i64.load offset=0x1e3 align=1 (i32.const 0)))
    (local.set 0x1e4 (i64.load offset=0x1e4 align=1 (i32.const 0)))
    (local.set 0x1e5 (i64.load offset=0x1e5 align=1 (i32.const 0)))
    (local.set 0x1e6 (i64.load offset=0x1e6 align=1 (i32.const 0)))
    (local.set 0x1e7 (i64.load offset=0x1e7 align=1 (i32.const 0)))
    (local.set 0x1e8 (i64.load offset=0x1e8 align=1 (i32.const 0)))
    (local.set 0x1e9 (i64.load offset=0x1e9 align=1 (i32.const 0)))
    (local.set 0x1ea (i64.load offset=0x1ea align=1 (i32.const 0)))
    (local.set 0x1eb (i64.load offset=0x1eb align=1 (i32.const 0)))
    (local.set 0x1ec (i64.load offset=0x1ec align=1 (i32.const 0)))
    (local.set 0x1ed (i64.load offset=0x1ed align=1 (i32.const 0)))
    (local.set 0x1ee (i64.load offset=0x1ee align=1 (i32.const 0)))
    (local.set 0x1ef (i64.load offset=0x1ef align=1 (i32.const 0)))
    (local.set 0x1f0 (i64.load offset=0x1f0 align=1 (i32.const 0)))
    (local.set 0x1f1 (i64.load offset=0x1f1 align=1 (i32.const 0)))
    (local.set 0x1f2 (i64.load offset=0x1f2 align=1 (i32.const 0)))
    (local.set 0x1f3 (i64.load offset=0x1f3 align=1 (i32.const 0)))
    (local.set 0x1f4 (i64.load offset=0x1f4 align=1 (i32.const 0)))
    (local.set 0x1f5 (i64.load offset=0x1f5 align=1 (i32.const 0)))
    (local.set 0x1f6 (i64.load offset=0x1f6 align=1 (i32.const 0)))
    (local.set 0x1f7 (i64.load offset=0x1f7 align=1 (i32.const 0)))
    (local.set 0x1f8 (i64.load offset=0x1f8 align=1 (i32.const 0)))
    (local.set 0x1f9 (i64.load offset=0x1f9 align=1 (i32.const 0)))
    (local.set 0x1fa (i64.load offset=0x1fa align=1 (i32.const 0)))
    (local.set 0x1fb (i64.load offset=0x1fb align=1 (i32.const 0)))
    (local.set 0x1fc (i64.load offset=0x1fc align=1 (i32.const 0)))
    (local.set 0x1fd (i64.load offset=0x1fd align=1 (i32.const 0)))
    (local.set 0x1fe (i64.load offset=0x1fe align=1 (i32.const 0)))
    (local.set 0x1ff (i64.load offset=0x1ff align=1 (i32.const 0)))
    (local.set 0x200 (i64.load offset=0x200 align=1 (i32.const 0)))
    (local.set 0x201 (i64.load offset=0x201 align=1 (i32.const 0)))
    (local.set 0x202 (i64.load offset=0x202 align=1 (i32.const 0)))
    (local.set 0x203 (i64.load offset=0x203 align=1 (i32.const 0)))
    (local.set 0x204 (i64.load offset=0x204 align=1 (i32.const 0)))
    (local.set 0x205 (i64.load offset=0x205 align=1 (i32.const 0)))
    (local.set 0x206 (i64.load offset=0x206 align=1 (i32.const 0)))
    (local.set 0x207 (i64.load offset=0x207 align=1 (i32.const 0)))
    (local.set 0x208 (i64.load offset=0x208 align=1 (i32.const 0)))
    (local.set 0x209 (i64.load offset=0x209 align=1 (i32.const 0)))
    (local.set 0x20a (i64.load offset=0x20a align=1 (i32.const 0)))
    (local.set 0x20b (i64.load offset=0x20b align=1 (i32.const 0)))
    (local.set 0x20c (i64.load offset=0x20c align=1 (i32.const 0)))
    (local.set 0x20d (i64.load offset=0x20d align=1 (i32.const 0)))
    (local.set 0x20e (i64.load offset=0x20e align=1 (i32.const 0)))
    (local.set 0x20f (i64.load offset=0x20f align=1 (i32.const 0)))
    (local.set 0x210 (i64.load offset=0x210 align=1 (i32.const 0)))
    (local.set 0x211 (i64.load offset=0x211 align=1 (i32.const 0)))
    (local.set 0x212 (i64.load offset=0x212 align=1 (i32.const 0)))
    (local.set 0x213 (i64.load offset=0x213 align=1 (i32.const 0)))
    (local.set 0x214 (i64.load offset=0x214 align=1 (i32.const 0)))
    (local.set 0x215 (i64.load offset=0x215 align=1 (i32.const 0)))
    (local.set 0x216 (i64.load offset=0x216 align=1 (i32.const 0)))
    (local.set 0x217 (i64.load offset=0x217 align=1 (i32.const 0)))
    (local.set 0x218 (i64.load offset=0x218 align=1 (i32.const 0)))
    (local.set 0x219 (i64.load offset=0x219 align=1 (i32.const 0)))
    (local.set 0x21a (i64.load offset=0x21a align=1 (i32.const 0)))
    (local.set 0x21b (i64.load offset=0x21b align=1 (i32.const 0)))
    (local.set 0x21c (i64.load offset=0x21c align=1 (i32.const 0)))
    (local.set 0x21d (i64.load offset=0x21d align=1 (i32.const 0)))
    (local.set 0x21e (i64.load offset=0x21e align=1 (i32.const 0)))
    (local.set 0x21f (i64.load offset=0x21f align=1 (i32.const 0)))
    (local.set 0x220 (i64.load offset=0x220 align=1 (i32.const 0)))
    (local.set 0x221 (i64.load offset=0x221 align=1 (i32.const 0)))
    (local.set 0x222 (i64.load offset=0x222 align=1 (i32.const 0)))
    (local.set 0x223 (i64.load offset=0x223 align=1 (i32.const 0)))
    (local.set 0x224 (i64.load offset=0x224 align=1 (i32.const 0)))
    (local.set 0x225 (i64.load offset=0x225 align=1 (i32.const 0)))
    (local.set 0x226 (i64.load offset=0x226 align=1 (i32.const 0)))
    (local.set 0x227 (i64.load offset=0x227 align=1 (i32.const 0)))
    (local.set 0x228 (i64.load offset=0x228 align=1 (i32.const 0)))
    (local.set 0x229 (i64.load offset=0x229 align=1 (i32.const 0)))
    (local.set 0x22a (i64.load offset=0x22a align=1 (i32.const 0)))
    (local.set 0x22b (i64.load offset=0x22b align=1 (i32.const 0)))
    (local.set 0x22c (i64.load offset=0x22c align=1 (i32.const 0)))
    (local.set 0x22d (i64.load offset=0x22d align=1 (i32.const 0)))
    (local.set 0x22e (i64.load offset=0x22e align=1 (i32.const 0)))
    (local.set 0x22f (i64.load offset=0x22f align=1 (i32.const 0)))
    (local.set 0x230 (i64.load offset=0x230 align=1 (i32.const 0)))
    (local.set 0x231 (i64.load offset=0x231 align=1 (i32.const 0)))
    (local.set 0x232 (i64.load offset=0x232 align=1 (i32.const 0)))
    (local.set 0x233 (i64.load offset=0x233 align=1 (i32.const 0)))
    (local.set 0x234 (i64.load offset=0x234 align=1 (i32.const 0)))
    (local.set 0x235 (i64.load offset=0x235 align=1 (i32.const 0)))
    (local.set 0x236 (i64.load offset=0x236 align=1 (i32.const 0)))
    (local.set 0x237 (i64.load offset=0x237 align=1 (i32.const 0)))
    (local.set 0x238 (i64.load offset=0x238 align=1 (i32.const 0)))
    (local.set 0x239 (i64.load offset=0x239 align=1 (i32.const 0)))
    (local.set 0x23a (i64.load offset=0x23a align=1 (i32.const 0)))
    (local.set 0x23b (i64.load offset=0x23b align=1 (i32.const 0)))
    (local.set 0x23c (i64.load offset=0x23c align=1 (i32.const 0)))
    (local.set 0x23d (i64.load offset=0x23d align=1 (i32.const 0)))
    (local.set 0x23e (i64.load offset=0x23e align=1 (i32.const 0)))
    (local.set 0x23f (i64.load offset=0x23f align=1 (i32.const 0)))
    (local.set 0x240 (i64.load offset=0x240 align=1 (i32.const 0)))
    (local.set 0x241 (i64.load offset=0x241 align=1 (i32.const 0)))
    (local.set 0x242 (i64.load offset=0x242 align=1 (i32.const 0)))
    (local.set 0x243 (i64.load offset=0x243 align=1 (i32.const 0)))
    (local.set 0x244 (i64.load offset=0x244 align=1 (i32.const 0)))
    (local.set 0x245 (i64.load offset=0x245 align=1 (i32.const 0)))
    (local.set 0x246 (i64.load offset=0x246 align=1 (i32.const 0)))
    (local.set 0x247 (i64.load offset=0x247 align=1 (i32.const 0)))
    (local.set 0x248 (i64.load offset=0x248 align=1 (i32.const 0)))
    (local.set 0x249 (i64.load offset=0x249 align=1 (i32.const 0)))
    (local.set 0x24a (i64.load offset=0x24a align=1 (i32.const 0)))
    (local.set 0x24b (i64.load offset=0x24b align=1 (i32.const 0)))
    (local.set 0x24c (i64.load offset=0x24c align=1 (i32.const 0)))
    (local.set 0x24d (i64.load offset=0x24d align=1 (i32.const 0)))
    (local.set 0x24e (i64.load offset=0x24e align=1 (i32.const 0)))
    (local.set 0x24f (i64.load offset=0x24f align=1 (i32.const 0)))
    (local.set 0x250 (i64.load offset=0x250 align=1 (i32.const 0)))
    (local.set 0x251 (i64.load offset=0x251 align=1 (i32.const 0)))
    (local.set 0x252 (i64.load offset=0x252 align=1 (i32.const 0)))
    (local.set 0x253 (i64.load offset=0x253 align=1 (i32.const 0)))
    (local.set 0x254 (i64.load offset=0x254 align=1 (i32.const 0)))
    (local.set 0x255 (i64.load offset=0x255 align=1 (i32.const 0)))
    (local.set 0x256 (i64.load offset=0x256 align=1 (i32.const 0)))
    (local.set 0x257 (i64.load offset=0x257 align=1 (i32.const 0)))
    (local.set 0x258 (i64.load offset=0x258 align=1 (i32.const 0)))
    (local.set 0x259 (i64.load offset=0x259 align=1 (i32.const 0)))
    (local.set 0x25a (i64.load offset=0x25a align=1 (i32.const 0)))
    (local.set 0x25b (i64.load offset=0x25b align=1 (i32.const 0)))
    (local.set 0x25c (i64.load offset=0x25c align=1 (i32.const 0)))
    (local.set 0x25d (i64.load offset=0x25d align=1 (i32.const 0)))
    (local.set 0x25e (i64.load offset=0x25e align=1 (i32.const 0)))
    (local.set 0x25f (i64.load offset=0x25f align=1 (i32.const 0)))
    (local.set 0x260 (i64.load offset=0x260 align=1 (i32.const 0)))
    (local.set 0x261 (i64.load offset=0x261 align=1 (i32.const 0)))
    (local.set 0x262 (i64.load offset=0x262 align=1 (i32.const 0)))
    (local.set 0x263 (i64.load offset=0x263 align=1 (i32.const 0)))
    (local.set 0x264 (i64.load offset=0x264 align=1 (i32.const 0)))
    (local.set 0x265 (i64.load offset=0x265 align=1 (i32.const 0)))
    (local.set 0x266 (i64.load offset=0x266 align=1 (i32.const 0)))
    (local.set 0x267 (i64.load offset=0x267 align=1 (i32.const 0)))
    (local.set 0x268 (i64.load offset=0x268 align=1 (i32.const 0)))
    (local.set 0x269 (i64.load offset=0x269 align=1 (i32.const 0)))
    (local.set 0x26a (i64.load offset=0x26a align=1 (i32.const 0)))
    (local.set 0x26b (i64.load offset=0x26b align=1 (i32.const 0)))
    (local.set 0x26c (i64.load offset=0x26c align=1 (i32.const 0)))
    (local.set 0x26d (i64.load offset=0x26d align=1 (i32.const 0)))
    (local.set 0x26e (i64.load offset=0x26e align=1 (i32.const 0)))
    (local.set 0x26f (i64.load offset=0x26f align=1 (i32.const 0)))
    (local.set 0x270 (i64.load offset=0x270 align=1 (i32.const 0)))
    (local.set 0x271 (i64.load offset=0x271 align=1 (i32.const 0)))
    (local.set 0x272 (i64.load offset=0x272 align=1 (i32.const 0)))
    (local.set 0x273 (i64.load offset=0x273 align=1 (i32.const 0)))
    (local.set 0x274 (i64.load offset=0x274 align=1 (i32.const 0)))
    (local.set 0x275 (i64.load offset=0x275 align=1 (i32.const 0)))
    (local.set 0x276 (i64.load offset=0x276 align=1 (i32.const 0)))
    (local.set 0x277 (i64.load offset=0x277 align=1 (i32.const 0)))
    (local.set 0x278 (i64.load offset=0x278 align=1 (i32.const 0)))
    (local.set 0x279 (i64.load offset=0x279 align=1 (i32.const 0)))
    (local.set 0x27a (i64.load offset=0x27a align=1 (i32.const 0)))
    (local.set 0x27b (i64.load offset=0x27b align=1 (i32.const 0)))
    (local.set 0x27c (i64.load offset=0x27c align=1 (i32.const 0)))
    (local.set 0x27d (i64.load offset=0x27d align=1 (i32.const 0)))
    (local.set 0x27e (i64.load offset=0x27e align=1 (i32.const 0)))
    (local.set 0x27f (i64.load offset=0x27f align=1 (i32.const 0)))
    (local.set 0x280 (i64.load offset=0x280 align=1 (i32.const 0)))
    (local.set 0x281 (i64.load offset=0x281 align=1 (i32.const 0)))
    (local.set 0x282 (i64.load offset=0x282 align=1 (i32.const 0)))
    (local.set 0x283 (i64.load offset=0x283 align=1 (i32.const 0)))
    (local.set 0x284 (i64.load offset=0x284 align=1 (i32.const 0)))
    (local.set 0x285 (i64.load offset=0x285 align=1 (i32.const 0)))
    (local.set 0x286 (i64.load offset=0x286 align=1 (i32.const 0)))
    (local.set 0x287 (i64.load offset=0x287 align=1 (i32.const 0)))
    (local.set 0x288 (i64.load offset=0x288 align=1 (i32.const 0)))
    (local.set 0x289 (i64.load offset=0x289 align=1 (i32.const 0)))
    (local.set 0x28a (i64.load offset=0x28a align=1 (i32.const 0)))
    (local.set 0x28b (i64.load offset=0x28b align=1 (i32.const 0)))
    (local.set 0x28c (i64.load offset=0x28c align=1 (i32.const 0)))
    (local.set 0x28d (i64.load offset=0x28d align=1 (i32.const 0)))
    (local.set 0x28e (i64.load offset=0x28e align=1 (i32.const 0)))
    (local.set 0x28f (i64.load offset=0x28f align=1 (i32.const 0)))
    (local.set 0x290 (i64.load offset=0x290 align=1 (i32.const 0)))
    (local.set 0x291 (i64.load offset=0x291 align=1 (i32.const 0)))
    (local.set 0x292 (i64.load offset=0x292 align=1 (i32.const 0)))
    (local.set 0x293 (i64.load offset=0x293 align=1 (i32.const 0)))
    (local.set 0x294 (i64.load offset=0x294 align=1 (i32.const 0)))
    (local.set 0x295 (i64.load offset=0x295 align=1 (i32.const 0)))
    (local.set 0x296 (i64.load offset=0x296 align=1 (i32.const 0)))
    (local.set 0x297 (i64.load offset=0x297 align=1 (i32.const 0)))
    (local.set 0x298 (i64.load offset=0x298 align=1 (i32.const 0)))
    (local.set 0x299 (i64.load offset=0x299 align=1 (i32.const 0)))
    (local.set 0x29a (i64.load offset=0x29a align=1 (i32.const 0)))
    (local.set 0x29b (i64.load offset=0x29b align=1 (i32.const 0)))
    (local.set 0x29c (i64.load offset=0x29c align=1 (i32.const 0)))
    (local.set 0x29d (i64.load offset=0x29d align=1 (i32.const 0)))
    (local.set 0x29e (i64.load offset=0x29e align=1 (i32.const 0)))
    (local.set 0x29f (i64.load offset=0x29f align=1 (i32.const 0)))
    (local.set 0x2a0 (i64.load offset=0x2a0 align=1 (i32.const 0)))
    (local.set 0x2a1 (i64.load offset=0x2a1 align=1 (i32.const 0)))
    (local.set 0x2a2 (i64.load offset=0x2a2 align=1 (i32.const 0)))
    (local.set 0x2a3 (i64.load offset=0x2a3 align=1 (i32.const 0)))
    (local.set 0x2a4 (i64.load offset=0x2a4 align=1 (i32.const 0)))
    (local.set 0x2a5 (i64.load offset=0x2a5 align=1 (i32.const 0)))
    (local.set 0x2a6 (i64.load offset=0x2a6 align=1 (i32.const 0)))
    (local.set 0x2a7 (i64.load offset=0x2a7 align=1 (i32.const 0)))
    (local.set 0x2a8 (i64.load offset=0x2a8 align=1 (i32.const 0)))
    (local.set 0x2a9 (i64.load offset=0x2a9 align=1 (i32.const 0)))
    (local.set 0x2aa (i64.load offset=0x2aa align=1 (i32.const 0)))
    (local.set 0x2ab (i64.load offset=0x2ab align=1 (i32.const 0)))
    (local.set 0x2ac (i64.load offset=0x2ac align=1 (i32.const 0)))
    (local.set 0x2ad (i64.load offset=0x2ad align=1 (i32.const 0)))
    (local.set 0x2ae (i64.load offset=0x2ae align=1 (i32.const 0)))
    (local.set 0x2af (i64.load offset=0x2af align=1 (i32.const 0)))
    (local.set 0x2b0 (i64.load offset=0x2b0 align=1 (i32.const 0)))
    (local.set 0x2b1 (i64.load offset=0x2b1 align=1 (i32.const 0)))
    (local.set 0x2b2 (i64.load offset=0x2b2 align=1 (i32.const 0)))
    (local.set 0x2b3 (i64.load offset=0x2b3 align=1 (i32.const 0)))
    (local.set 0x2b4 (i64.load offset=0x2b4 align=1 (i32.const 0)))
    (local.set 0x2b5 (i64.load offset=0x2b5 align=1 (i32.const 0)))
    (local.set 0x2b6 (i64.load offset=0x2b6 align=1 (i32.const 0)))
    (local.set 0x2b7 (i64.load offset=0x2b7 align=1 (i32.const 0)))
    (local.set 0x2b8 (i64.load offset=0x2b8 align=1 (i32.const 0)))
    (local.set 0x2b9 (i64.load offset=0x2b9 align=1 (i32.const 0)))
    (local.set 0x2ba (i64.load offset=0x2ba align=1 (i32.const 0)))
    (local.set 0x2bb (i64.load offset=0x2bb align=1 (i32.const 0)))
    (local.set 0x2bc (i64.load offset=0x2bc align=1 (i32.const 0)))
    (local.set 0x2bd (i64.load offset=0x2bd align=1 (i32.const 0)))
    (local.set 0x2be (i64.load offset=0x2be align=1 (i32.const 0)))
    (local.set 0x2bf (i64.load offset=0x2bf align=1 (i32.const 0)))
    (local.set 0x2c0 (i64.load offset=0x2c0 align=1 (i32.const 0)))
    (local.set 0x2c1 (i64.load offset=0x2c1 align=1 (i32.const 0)))
    (local.set 0x2c2 (i64.load offset=0x2c2 align=1 (i32.const 0)))
    (local.set 0x2c3 (i64.load offset=0x2c3 align=1 (i32.const 0)))
    (local.set 0x2c4 (i64.load offset=0x2c4 align=1 (i32.const 0)))
    (local.set 0x2c5 (i64.load offset=0x2c5 align=1 (i32.const 0)))
    (local.set 0x2c6 (i64.load offset=0x2c6 align=1 (i32.const 0)))
    (local.set 0x2c7 (i64.load offset=0x2c7 align=1 (i32.const 0)))
    (local.set 0x2c8 (i64.load offset=0x2c8 align=1 (i32.const 0)))
    (local.set 0x2c9 (i64.load offset=0x2c9 align=1 (i32.const 0)))
    (local.set 0x2ca (i64.load offset=0x2ca align=1 (i32.const 0)))
    (local.set 0x2cb (i64.load offset=0x2cb align=1 (i32.const 0)))
    (local.set 0x2cc (i64.load offset=0x2cc align=1 (i32.const 0)))
    (local.set 0x2cd (i64.load offset=0x2cd align=1 (i32.const 0)))
    (local.set 0x2ce (i64.load offset=0x2ce align=1 (i32.const 0)))
    (local.set 0x2cf (i64.load offset=0x2cf align=1 (i32.const 0)))
    (local.set 0x2d0 (i64.load offset=0x2d0 align=1 (i32.const 0)))
    (local.set 0x2d1 (i64.load offset=0x2d1 align=1 (i32.const 0)))
    (local.set 0x2d2 (i64.load offset=0x2d2 align=1 (i32.const 0)))
    (local.set 0x2d3 (i64.load offset=0x2d3 align=1 (i32.const 0)))
    (local.set 0x2d4 (i64.load offset=0x2d4 align=1 (i32.const 0)))
    (local.set 0x2d5 (i64.load offset=0x2d5 align=1 (i32.const 0)))
    (local.set 0x2d6 (i64.load offset=0x2d6 align=1 (i32.const 0)))
    (local.set 0x2d7 (i64.load offset=0x2d7 align=1 (i32.const 0)))
    (local.set 0x2d8 (i64.load offset=0x2d8 align=1 (i32.const 0)))
    (local.set 0x2d9 (i64.load offset=0x2d9 align=1 (i32.const 0)))
    (local.set 0x2da (i64.load offset=0x2da align=1 (i32.const 0)))
    (local.set 0x2db (i64.load offset=0x2db align=1 (i32.const 0)))
    (local.set 0x2dc (i64.load offset=0x2dc align=1 (i32.const 0)))
    (local.set 0x2dd (i64.load offset=0x2dd align=1 (i32.const 0)))
    (local.set 0x2de (i64.load offset=0x2de align=1 (i32.const 0)))
    (local.set 0x2df (i64.load offset=0x2df align=1 (i32.const 0)))
    (local.set 0x2e0 (i64.load offset=0x2e0 align=1 (i32.const 0)))
    (local.set 0x2e1 (i64.load offset=0x2e1 align=1 (i32.const 0)))
    (local.set 0x2e2 (i64.load offset=0x2e2 align=1 (i32.const 0)))
    (local.set 0x2e3 (i64.load offset=0x2e3 align=1 (i32.const 0)))
    (local.set 0x2e4 (i64.load offset=0x2e4 align=1 (i32.const 0)))
    (local.set 0x2e5 (i64.load offset=0x2e5 align=1 (i32.const 0)))
    (local.set 0x2e6 (i64.load offset=0x2e6 align=1 (i32.const 0)))
    (local.set 0x2e7 (i64.load offset=0x2e7 align=1 (i32.const 0)))
    (local.set 0x2e8 (i64.load offset=0x2e8 align=1 (i32.const 0)))
    (local.set 0x2e9 (i64.load offset=0x2e9 align=1 (i32.const 0)))
    (local.set 0x2ea (i64.load offset=0x2ea align=1 (i32.const 0)))
    (local.set 0x2eb (i64.load offset=0x2eb align=1 (i32.const 0)))
    (local.set 0x2ec (i64.load offset=0x2ec align=1 (i32.const 0)))
    (local.set 0x2ed (i64.load offset=0x2ed align=1 (i32.const 0)))
    (local.set 0x2ee (i64.load offset=0x2ee align=1 (i32.const 0)))
    (local.set 0x2ef (i64.load offset=0x2ef align=1 (i32.const 0)))
    (local.set 0x2f0 (i64.load offset=0x2f0 align=1 (i32.const 0)))
    (local.set 0x2f1 (i64.load offset=0x2f1 align=1 (i32.const 0)))
    (local.set 0x2f2 (i64.load offset=0x2f2 align=1 (i32.const 0)))
    (local.set 0x2f3 (i64.load offset=0x2f3 align=1 (i32.const 0)))
    (local.set 0x2f4 (i64.load offset=0x2f4 align=1 (i32.const 0)))
    (local.set 0x2f5 (i64.load offset=0x2f5 align=1 (i32.const 0)))
    (local.set 0x2f6 (i64.load offset=0x2f6 align=1 (i32.const 0)))
    (local.set 0x2f7 (i64.load offset=0x2f7 align=1 (i32.const 0)))
    (local.set 0x2f8 (i64.load offset=0x2f8 align=1 (i32.const 0)))
    (local.set 0x2f9 (i64.load offset=0x2f9 align=1 (i32.const 0)))
    (local.set 0x2fa (i64.load offset=0x2fa align=1 (i32.const 0)))
    (local.set 0x2fb (i64.load offset=0x2fb align=1 (i32.const 0)))
    (local.set 0x2fc (i64.load offset=0x2fc align=1 (i32.const 0)))
    (local.set 0x2fd (i64.load offset=0x2fd align=1 (i32.const 0)))
    (local.set 0x2fe (i64.load offset=0x2fe align=1 (i32.const 0)))
    (local.set 0x2ff (i64.load offset=0x2ff align=1 (i32.const 0)))
    (local.set 0x300 (i64.load offset=0x300 align=1 (i32.const 0)))
    (local.set 0x301 (i64.load offset=0x301 align=1 (i32.const 0)))
    (local.set 0x302 (i64.load offset=0x302 align=1 (i32.const 0)))
    (local.set 0x303 (i64.load offset=0x303 align=1 (i32.const 0)))
    (local.set 0x304 (i64.load offset=0x304 align=1 (i32.const 0)))
    (local.set 0x305 (i64.load offset=0x305 align=1 (i32.const 0)))
    (local.set 0x306 (i64.load offset=0x306 align=1 (i32.const 0)))
    (local.set 0x307 (i64.load offset=0x307 align=1 (i32.const 0)))
    (local.set 0x308 (i64.load offset=0x308 align=1 (i32.const 0)))
    (local.set 0x309 (i64.load offset=0x309 align=1 (i32.const 0)))
    (local.set 0x30a (i64.load offset=0x30a align=1 (i32.const 0)))
    (local.set 0x30b (i64.load offset=0x30b align=1 (i32.const 0)))
    (local.set 0x30c (i64.load offset=0x30c align=1 (i32.const 0)))
    (local.set 0x30d (i64.load offset=0x30d align=1 (i32.const 0)))
    (local.set 0x30e (i64.load offset=0x30e align=1 (i32.const 0)))
    (local.set 0x30f (i64.load offset=0x30f align=1 (i32.const 0)))
    (local.set 0x310 (i64.load offset=0x310 align=1 (i32.const 0)))
    (local.set 0x311 (i64.load offset=0x311 align=1 (i32.const 0)))
    (local.set 0x312 (i64.load offset=0x312 align=1 (i32.const 0)))
    (local.set 0x313 (i64.load offset=0x313 align=1 (i32.const 0)))
    (local.set 0x314 (i64.load offset=0x314 align=1 (i32.const 0)))
    (local.set 0x315 (i64.load offset=0x315 align=1 (i32.const 0)))
    (local.set 0x316 (i64.load offset=0x316 align=1 (i32.const 0)))
    (local.set 0x317 (i64.load offset=0x317 align=1 (i32.const 0)))
    (local.set 0x318 (i64.load offset=0x318 align=1 (i32.const 0)))
    (local.set 0x319 (i64.load offset=0x319 align=1 (i32.const 0)))
    (local.set 0x31a (i64.load offset=0x31a align=1 (i32.const 0)))
    (local.set 0x31b (i64.load offset=0x31b align=1 (i32.const 0)))
    (local.set 0x31c (i64.load offset=0x31c align=1 (i32.const 0)))
    (local.set 0x31d (i64.load offset=0x31d align=1 (i32.const 0)))
    (local.set 0x31e (i64.load offset=0x31e align=1 (i32.const 0)))
    (local.set 0x31f (i64.load offset=0x31f align=1 (i32.const 0)))
    (local.set 0x320 (i64.load offset=0x320 align=1 (i32.const 0)))
    (local.set 0x321 (i64.load offset=0x321 align=1 (i32.const 0)))
    (local.set 0x322 (i64.load offset=0x322 align=1 (i32.const 0)))
    (local.set 0x323 (i64.load offset=0x323 align=1 (i32.const 0)))
    (local.set 0x324 (i64.load offset=0x324 align=1 (i32.const 0)))
    (local.set 0x325 (i64.load offset=0x325 align=1 (i32.const 0)))
    (local.set 0x326 (i64.load offset=0x326 align=1 (i32.const 0)))
    (local.set 0x327 (i64.load offset=0x327 align=1 (i32.const 0)))
    (local.set 0x328 (i64.load offset=0x328 align=1 (i32.const 0)))
    (local.set 0x329 (i64.load offset=0x329 align=1 (i32.const 0)))
    (local.set 0x32a (i64.load offset=0x32a align=1 (i32.const 0)))
    (local.set 0x32b (i64.load offset=0x32b align=1 (i32.const 0)))
    (local.set 0x32c (i64.load offset=0x32c align=1 (i32.const 0)))
    (local.set 0x32d (i64.load offset=0x32d align=1 (i32.const 0)))
    (local.set 0x32e (i64.load offset=0x32e align=1 (i32.const 0)))
    (local.set 0x32f (i64.load offset=0x32f align=1 (i32.const 0)))
    (local.set 0x330 (i64.load offset=0x330 align=1 (i32.const 0)))
    (local.set 0x331 (i64.load offset=0x331 align=1 (i32.const 0)))
    (local.set 0x332 (i64.load offset=0x332 align=1 (i32.const 0)))
    (local.set 0x333 (i64.load offset=0x333 align=1 (i32.const 0)))
    (local.set 0x334 (i64.load offset=0x334 align=1 (i32.const 0)))
    (local.set 0x335 (i64.load offset=0x335 align=1 (i32.const 0)))
    (local.set 0x336 (i64.load offset=0x336 align=1 (i32.const 0)))
    (local.set 0x337 (i64.load offset=0x337 align=1 (i32.const 0)))
    (local.set 0x338 (i64.load offset=0x338 align=1 (i32.const 0)))
    (local.set 0x339 (i64.load offset=0x339 align=1 (i32.const 0)))
    (local.set 0x33a (i64.load offset=0x33a align=1 (i32.const 0)))
    (local.set 0x33b (i64.load offset=0x33b align=1 (i32.const 0)))
    (local.set 0x33c (i64.load offset=0x33c align=1 (i32.const 0)))
    (local.set 0x33d (i64.load offset=0x33d align=1 (i32.const 0)))
    (local.set 0x33e (i64.load offset=0x33e align=1 (i32.const 0)))
    (local.set 0x33f (i64.load offset=0x33f align=1 (i32.const 0)))
    (local.set 0x340 (i64.load offset=0x340 align=1 (i32.const 0)))
    (local.set 0x341 (i64.load offset=0x341 align=1 (i32.const 0)))
    (local.set 0x342 (i64.load offset=0x342 align=1 (i32.const 0)))
    (local.set 0x343 (i64.load offset=0x343 align=1 (i32.const 0)))
    (local.set 0x344 (i64.load offset=0x344 align=1 (i32.const 0)))
    (local.set 0x345 (i64.load offset=0x345 align=1 (i32.const 0)))
    (local.set 0x346 (i64.load offset=0x346 align=1 (i32.const 0)))
    (local.set 0x347 (i64.load offset=0x347 align=1 (i32.const 0)))
    (local.set 0x348 (i64.load offset=0x348 align=1 (i32.const 0)))
    (local.set 0x349 (i64.load offset=0x349 align=1 (i32.const 0)))
    (local.set 0x34a (i64.load offset=0x34a align=1 (i32.const 0)))
    (local.set 0x34b (i64.load offset=0x34b align=1 (i32.const 0)))
    (local.set 0x34c (i64.load offset=0x34c align=1 (i32.const 0)))
    (local.set 0x34d (i64.load offset=0x34d align=1 (i32.const 0)))
    (local.set 0x34e (i64.load offset=0x34e align=1 (i32.const 0)))
    (local.set 0x34f (i64.load offset=0x34f align=1 (i32.const 0)))
    (local.set 0x350 (i64.load offset=0x350 align=1 (i32.const 0)))
    (local.set 0x351 (i64.load offset=0x351 align=1 (i32.const 0)))
    (local.set 0x352 (i64.load offset=0x352 align=1 (i32.const 0)))
    (local.set 0x353 (i64.load offset=0x353 align=1 (i32.const 0)))
    (local.set 0x354 (i64.load offset=0x354 align=1 (i32.const 0)))
    (local.set 0x355 (i64.load offset=0x355 align=1 (i32.const 0)))
    (local.set 0x356 (i64.load offset=0x356 align=1 (i32.const 0)))
    (local.set 0x357 (i64.load offset=0x357 align=1 (i32.const 0)))
    (local.set 0x358 (i64.load offset=0x358 align=1 (i32.const 0)))
    (local.set 0x359 (i64.load offset=0x359 align=1 (i32.const 0)))
    (local.set 0x35a (i64.load offset=0x35a align=1 (i32.const 0)))
    (local.set 0x35b (i64.load offset=0x35b align=1 (i32.const 0)))
    (local.set 0x35c (i64.load offset=0x35c align=1 (i32.const 0)))
    (local.set 0x35d (i64.load offset=0x35d align=1 (i32.const 0)))
    (local.set 0x35e (i64.load offset=0x35e align=1 (i32.const 0)))
    (local.set 0x35f (i64.load offset=0x35f align=1 (i32.const 0)))
    (local.set 0x360 (i64.load offset=0x360 align=1 (i32.const 0)))
    (local.set 0x361 (i64.load offset=0x361 align=1 (i32.const 0)))
    (local.set 0x362 (i64.load offset=0x362 align=1 (i32.const 0)))
    (local.set 0x363 (i64.load offset=0x363 align=1 (i32.const 0)))
    (local.set 0x364 (i64.load offset=0x364 align=1 (i32.const 0)))
    (local.set 0x365 (i64.load offset=0x365 align=1 (i32.const 0)))
    (local.set 0x366 (i64.load offset=0x366 align=1 (i32.const 0)))
    (local.set 0x367 (i64.load offset=0x367 align=1 (i32.const 0)))
    (local.set 0x368 (i64.load offset=0x368 align=1 (i32.const 0)))
    (local.set 0x369 (i64.load offset=0x369 align=1 (i32.const 0)))
    (local.set 0x36a (i64.load offset=0x36a align=1 (i32.const 0)))
    (local.set 0x36b (i64.load offset=0x36b align=1 (i32.const 0)))
    (local.set 0x36c (i64.load offset=0x36c align=1 (i32.const 0)))
    (local.set 0x36d (i64.load offset=0x36d align=1 (i32.const 0)))
    (local.set 0x36e (i64.load offset=0x36e align=1 (i32.const 0)))
    (local.set 0x36f (i64.load offset=0x36f align=1 (i32.const 0)))
    (local.set 0x370 (i64.load offset=0x370 align=1 (i32.const 0)))
    (local.set 0x371 (i64.load offset=0x371 align=1 (i32.const 0)))
    (local.set 0x372 (i64.load offset=0x372 align=1 (i32.const 0)))
    (local.set 0x373 (i64.load offset=0x373 align=1 (i32.const 0)))
    (local.set 0x374 (i64.load offset=0x374 align=1 (i32.const 0)))
    (local.set 0x375 (i64.load offset=0x375 align=1 (i32.const 0)))
    (local.set 0x376 (i64.load offset=0x376 align=1 (i32.const 0)))
    (local.set 0x377 (i64.load offset=0x377 align=1 (i32.const 0)))
    (local.set 0x378 (i64.load offset=0x378 align=1 (i32.const 0)))
    (local.set 0x379 (i64.load offset=0x379 align=1 (i32.const 0)))
    (local.set 0x37a (i64.load offset=0x37a align=1 (i32.const 0)))
    (local.set 0x37b (i64.load offset=0x37b align=1 (i32.const 0)))
    (local.set 0x37c (i64.load offset=0x37c align=1 (i32.const 0)))
    (local.set 0x37d (i64.load offset=0x37d align=1 (i32.const 0)))
    (local.set 0x37e (i64.load offset=0x37e align=1 (i32.const 0)))
    (local.set 0x37f (i64.load offset=0x37f align=1 (i32.const 0)))
    (local.set 0x380 (i64.load offset=0x380 align=1 (i32.const 0)))
    (local.set 0x381 (i64.load offset=0x381 align=1 (i32.const 0)))
    (local.set 0x382 (i64.load offset=0x382 align=1 (i32.const 0)))
    (local.set 0x383 (i64.load offset=0x383 align=1 (i32.const 0)))
    (local.set 0x384 (i64.load offset=0x384 align=1 (i32.const 0)))
    (local.set 0x385 (i64.load offset=0x385 align=1 (i32.const 0)))
    (local.set 0x386 (i64.load offset=0x386 align=1 (i32.const 0)))
    (local.set 0x387 (i64.load offset=0x387 align=1 (i32.const 0)))
    (local.set 0x388 (i64.load offset=0x388 align=1 (i32.const 0)))
    (local.set 0x389 (i64.load offset=0x389 align=1 (i32.const 0)))
    (local.set 0x38a (i64.load offset=0x38a align=1 (i32.const 0)))
    (local.set 0x38b (i64.load offset=0x38b align=1 (i32.const 0)))
    (local.set 0x38c (i64.load offset=0x38c align=1 (i32.const 0)))
    (local.set 0x38d (i64.load offset=0x38d align=1 (i32.const 0)))
    (local.set 0x38e (i64.load offset=0x38e align=1 (i32.const 0)))
    (local.set 0x38f (i64.load offset=0x38f align=1 (i32.const 0)))
    (local.set 0x390 (i64.load offset=0x390 align=1 (i32.const 0)))
    (local.set 0x391 (i64.load offset=0x391 align=1 (i32.const 0)))
    (local.set 0x392 (i64.load offset=0x392 align=1 (i32.const 0)))
    (local.set 0x393 (i64.load offset=0x393 align=1 (i32.const 0)))
    (local.set 0x394 (i64.load offset=0x394 align=1 (i32.const 0)))
    (local.set 0x395 (i64.load offset=0x395 align=1 (i32.const 0)))
    (local.set 0x396 (i64.load offset=0x396 align=1 (i32.const 0)))
    (local.set 0x397 (i64.load offset=0x397 align=1 (i32.const 0)))
    (local.set 0x398 (i64.load offset=0x398 align=1 (i32.const 0)))
    (local.set 0x399 (i64.load offset=0x399 align=1 (i32.const 0)))
    (local.set 0x39a (i64.load offset=0x39a align=1 (i32.const 0)))
    (local.set 0x39b (i64.load offset=0x39b align=1 (i32.const 0)))
    (local.set 0x39c (i64.load offset=0x39c align=1 (i32.const 0)))
    (local.set 0x39d (i64.load offset=0x39d align=1 (i32.const 0)))
    (local.set 0x39e (i64.load offset=0x39e align=1 (i32.const 0)))
    (local.set 0x39f (i64.load offset=0x39f align=1 (i32.const 0)))
    (local.set 0x3a0 (i64.load offset=0x3a0 align=1 (i32.const 0)))
    (local.set 0x3a1 (i64.load offset=0x3a1 align=1 (i32.const 0)))
    (local.set 0x3a2 (i64.load offset=0x3a2 align=1 (i32.const 0)))
    (local.set 0x3a3 (i64.load offset=0x3a3 align=1 (i32.const 0)))
    (local.set 0x3a4 (i64.load offset=0x3a4 align=1 (i32.const 0)))
    (local.set 0x3a5 (i64.load offset=0x3a5 align=1 (i32.const 0)))
    (local.set 0x3a6 (i64.load offset=0x3a6 align=1 (i32.const 0)))
    (local.set 0x3a7 (i64.load offset=0x3a7 align=1 (i32.const 0)))
    (local.set 0x3a8 (i64.load offset=0x3a8 align=1 (i32.const 0)))
    (local.set 0x3a9 (i64.load offset=0x3a9 align=1 (i32.const 0)))
    (local.set 0x3aa (i64.load offset=0x3aa align=1 (i32.const 0)))
    (local.set 0x3ab (i64.load offset=0x3ab align=1 (i32.const 0)))
    (local.set 0x3ac (i64.load offset=0x3ac align=1 (i32.const 0)))
    (local.set 0x3ad (i64.load offset=0x3ad align=1 (i32.const 0)))
    (local.set 0x3ae (i64.load offset=0x3ae align=1 (i32.const 0)))
    (local.set 0x3af (i64.load offset=0x3af align=1 (i32.const 0)))
    (local.set 0x3b0 (i64.load offset=0x3b0 align=1 (i32.const 0)))
    (local.set 0x3b1 (i64.load offset=0x3b1 align=1 (i32.const 0)))
    (local.set 0x3b2 (i64.load offset=0x3b2 align=1 (i32.const 0)))
    (local.set 0x3b3 (i64.load offset=0x3b3 align=1 (i32.const 0)))
    (local.set 0x3b4 (i64.load offset=0x3b4 align=1 (i32.const 0)))
    (local.set 0x3b5 (i64.load offset=0x3b5 align=1 (i32.const 0)))
    (local.set 0x3b6 (i64.load offset=0x3b6 align=1 (i32.const 0)))
    (local.set 0x3b7 (i64.load offset=0x3b7 align=1 (i32.const 0)))
    (local.set 0x3b8 (i64.load offset=0x3b8 align=1 (i32.const 0)))
    (local.set 0x3b9 (i64.load offset=0x3b9 align=1 (i32.const 0)))
    (local.set 0x3ba (i64.load offset=0x3ba align=1 (i32.const 0)))
    (local.set 0x3bb (i64.load offset=0x3bb align=1 (i32.const 0)))
    (local.set 0x3bc (i64.load offset=0x3bc align=1 (i32.const 0)))
    (local.set 0x3bd (i64.load offset=0x3bd align=1 (i32.const 0)))
    (local.set 0x3be (i64.load offset=0x3be align=1 (i32.const 0)))
    (local.set 0x3bf (i64.load offset=0x3bf align=1 (i32.const 0)))
    (local.set 0x3c0 (i64.load offset=0x3c0 align=1 (i32.const 0)))
    (local.set 0x3c1 (i64.load offset=0x3c1 align=1 (i32.const 0)))
    (local.set 0x3c2 (i64.load offset=0x3c2 align=1 (i32.const 0)))
    (local.set 0x3c3 (i64.load offset=0x3c3 align=1 (i32.const 0)))
    (local.set 0x3c4 (i64.load offset=0x3c4 align=1 (i32.const 0)))
    (local.set 0x3c5 (i64.load offset=0x3c5 align=1 (i32.const 0)))
    (local.set 0x3c6 (i64.load offset=0x3c6 align=1 (i32.const 0)))
    (local.set 0x3c7 (i64.load offset=0x3c7 align=1 (i32.const 0)))
    (local.set 0x3c8 (i64.load offset=0x3c8 align=1 (i32.const 0)))
    (local.set 0x3c9 (i64.load offset=0x3c9 align=1 (i32.const 0)))
    (local.set 0x3ca (i64.load offset=0x3ca align=1 (i32.const 0)))
    (local.set 0x3cb (i64.load offset=0x3cb align=1 (i32.const 0)))
    (local.set 0x3cc (i64.load offset=0x3cc align=1 (i32.const 0)))
    (local.set 0x3cd (i64.load offset=0x3cd align=1 (i32.const 0)))
    (local.set 0x3ce (i64.load offset=0x3ce align=1 (i32.const 0)))
    (local.set 0x3cf (i64.load offset=0x3cf align=1 (i32.const 0)))
    (local.set 0x3d0 (i64.load offset=0x3d0 align=1 (i32.const 0)))
    (local.set 0x3d1 (i64.load offset=0x3d1 align=1 (i32.const 0)))
    (local.set 0x3d2 (i64.load offset=0x3d2 align=1 (i32.const 0)))
    (local.set 0x3d3 (i64.load offset=0x3d3 align=1 (i32.const 0)))
    (local.set 0x3d4 (i64.load offset=0x3d4 align=1 (i32.const 0)))
    (local.set 0x3d5 (i64.load offset=0x3d5 align=1 (i32.const 0)))
    (local.set 0x3d6 (i64.load offset=0x3d6 align=1 (i32.const 0)))
    (local.set 0x3d7 (i64.load offset=0x3d7 align=1 (i32.const 0)))
    (local.set 0x3d8 (i64.load offset=0x3d8 align=1 (i32.const 0)))
    (local.set 0x3d9 (i64.load offset=0x3d9 align=1 (i32.const 0)))
    (local.set 0x3da (i64.load offset=0x3da align=1 (i32.const 0)))
    (local.set 0x3db (i64.load offset=0x3db align=1 (i32.const 0)))
    (local.set 0x3dc (i64.load offset=0x3dc align=1 (i32.const 0)))
    (local.set 0x3dd (i64.load offset=0x3dd align=1 (i32.const 0)))
    (local.set 0x3de (i64.load offset=0x3de align=1 (i32.const 0)))
    (local.set 0x3df (i64.load offset=0x3df align=1 (i32.const 0)))
    (local.set 0x3e0 (i64.load offset=0x3e0 align=1 (i32.const 0)))
    (local.set 0x3e1 (i64.load offset=0x3e1 align=1 (i32.const 0)))
    (local.set 0x3e2 (i64.load offset=0x3e2 align=1 (i32.const 0)))
    (local.set 0x3e3 (i64.load offset=0x3e3 align=1 (i32.const 0)))
    (local.set 0x3e4 (i64.load offset=0x3e4 align=1 (i32.const 0)))
    (local.set 0x3e5 (i64.load offset=0x3e5 align=1 (i32.const 0)))
    (local.set 0x3e6 (i64.load offset=0x3e6 align=1 (i32.const 0)))
    (local.set 0x3e7 (i64.load offset=0x3e7 align=1 (i32.const 0)))
    (local.set 0x3e8 (i64.load offset=0x3e8 align=1 (i32.const 0)))
    (local.set 0x3e9 (i64.load offset=0x3e9 align=1 (i32.const 0)))
    (local.set 0x3ea (i64.load offset=0x3ea align=1 (i32.const 0)))
    (local.set 0x3eb (i64.load offset=0x3eb align=1 (i32.const 0)))
    (local.set 0x3ec (i64.load offset=0x3ec align=1 (i32.const 0)))
    (local.set 0x3ed (i64.load offset=0x3ed align=1 (i32.const 0)))
    (local.set 0x3ee (i64.load offset=0x3ee align=1 (i32.const 0)))
    (local.set 0x3ef (i64.load offset=0x3ef align=1 (i32.const 0)))
    (local.set 0x3f0 (i64.load offset=0x3f0 align=1 (i32.const 0)))
    (local.set 0x3f1 (i64.load offset=0x3f1 align=1 (i32.const 0)))
    (local.set 0x3f2 (i64.load offset=0x3f2 align=1 (i32.const 0)))
    (local.set 0x3f3 (i64.load offset=0x3f3 align=1 (i32.const 0)))
    (local.set 0x3f4 (i64.load offset=0x3f4 align=1 (i32.const 0)))
    (local.set 0x3f5 (i64.load offset=0x3f5 align=1 (i32.const 0)))
    (local.set 0x3f6 (i64.load offset=0x3f6 align=1 (i32.const 0)))
    (local.set 0x3f7 (i64.load offset=0x3f7 align=1 (i32.const 0)))
    (local.set 0x3f8 (i64.load offset=0x3f8 align=1 (i32.const 0)))
    (local.set 0x3f9 (i64.load offset=0x3f9 align=1 (i32.const 0)))
    (local.set 0x3fa (i64.load offset=0x3fa align=1 (i32.const 0)))
    (local.set 0x3fb (i64.load offset=0x3fb align=1 (i32.const 0)))
    (local.set 0x3fc (i64.load offset=0x3fc align=1 (i32.const 0)))
    (local.set 0x3fd (i64.load offset=0x3fd align=1 (i32.const 0)))
    (local.set 0x3fe (i64.load offset=0x3fe align=1 (i32.const 0)))
    (local.set 0x3ff (i64.load offset=0x3ff align=1 (i32.const 0)))
    (local.set 0x400 (i64.load offset=0x400 align=1 (i32.const 0)))
    (local.set 0x401 (i64.load offset=0x401 align=1 (i32.const 0)))
    (local.set 0x402 (i64.load offset=0x402 align=1 (i32.const 0)))
    (local.set 0x403 (i64.load offset=0x403 align=1 (i32.const 0)))
    (local.set 0x404 (i64.load offset=0x404 align=1 (i32.const 0)))
    (local.set 0x405 (i64.load offset=0x405 align=1 (i32.const 0)))
    (local.set 0x406 (i64.load offset=0x406 align=1 (i32.const 0)))
    (local.set 0x407 (i64.load offset=0x407 align=1 (i32.const 0)))
    (local.set 0x408 (i64.load offset=0x408 align=1 (i32.const 0)))
    (local.set 0x409 (i64.load offset=0x409 align=1 (i32.const 0)))
    (local.set 0x40a (i64.load offset=0x40a align=1 (i32.const 0)))
    (local.set 0x40b (i64.load offset=0x40b align=1 (i32.const 0)))
    (local.set 0x40c (i64.load offset=0x40c align=1 (i32.const 0)))
    (local.set 0x40d (i64.load offset=0x40d align=1 (i32.const 0)))
    (local.set 0x40e (i64.load offset=0x40e align=1 (i32.const 0)))
    (local.set 0x40f (i64.load offset=0x40f align=1 (i32.const 0)))
    (local.set 0x410 (i64.load offset=0x410 align=1 (i32.const 0)))
    (local.set 0x411 (i64.load offset=0x411 align=1 (i32.const 0)))
    (local.set 0x412 (i64.load offset=0x412 align=1 (i32.const 0)))
    (local.set 0x413 (i64.load offset=0x413 align=1 (i32.const 0)))
    (local.set 0x414 (i64.load offset=0x414 align=1 (i32.const 0)))
    (local.set 0x415 (i64.load offset=0x415 align=1 (i32.const 0)))
    (local.set 0x416 (i64.load offset=0x416 align=1 (i32.const 0)))
    (local.set 0x417 (i64.load offset=0x417 align=1 (i32.const 0)))
    (local.set 0x418 (i64.load offset=0x418 align=1 (i32.const 0)))
    (local.set 0x419 (i64.load offset=0x419 align=1 (i32.const 0)))
    (local.set 0x41a (i64.load offset=0x41a align=1 (i32.const 0)))
    (local.set 0x41b (i64.load offset=0x41b align=1 (i32.const 0)))
    (local.set 0x41c (i64.load offset=0x41c align=1 (i32.const 0)))
    (local.set 0x41d (i64.load offset=0x41d align=1 (i32.const 0)))
    (local.set 0x41e (i64.load offset=0x41e align=1 (i32.const 0)))
    (local.set 0x41f (i64.load offset=0x41f align=1 (i32.const 0)))

    ;; store the locals back to memory
    (i64.store offset=0x000 align=1 (i32.const 0) (local.get 0x000))
    (i64.store offset=0x001 align=1 (i32.const 0) (local.get 0x001))
    (i64.store offset=0x002 align=1 (i32.const 0) (local.get 0x002))
    (i64.store offset=0x003 align=1 (i32.const 0) (local.get 0x003))
    (i64.store offset=0x004 align=1 (i32.const 0) (local.get 0x004))
    (i64.store offset=0x005 align=1 (i32.const 0) (local.get 0x005))
    (i64.store offset=0x006 align=1 (i32.const 0) (local.get 0x006))
    (i64.store offset=0x007 align=1 (i32.const 0) (local.get 0x007))
    (i64.store offset=0x008 align=1 (i32.const 0) (local.get 0x008))
    (i64.store offset=0x009 align=1 (i32.const 0) (local.get 0x009))
    (i64.store offset=0x00a align=1 (i32.const 0) (local.get 0x00a))
    (i64.store offset=0x00b align=1 (i32.const 0) (local.get 0x00b))
    (i64.store offset=0x00c align=1 (i32.const 0) (local.get 0x00c))
    (i64.store offset=0x00d align=1 (i32.const 0) (local.get 0x00d))
    (i64.store offset=0x00e align=1 (i32.const 0) (local.get 0x00e))
    (i64.store offset=0x00f align=1 (i32.const 0) (local.get 0x00f))
    (i64.store offset=0x010 align=1 (i32.const 0) (local.get 0x010))
    (i64.store offset=0x011 align=1 (i32.const 0) (local.get 0x011))
    (i64.store offset=0x012 align=1 (i32.const 0) (local.get 0x012))
    (i64.store offset=0x013 align=1 (i32.const 0) (local.get 0x013))
    (i64.store offset=0x014 align=1 (i32.const 0) (local.get 0x014))
    (i64.store offset=0x015 align=1 (i32.const 0) (local.get 0x015))
    (i64.store offset=0x016 align=1 (i32.const 0) (local.get 0x016))
    (i64.store offset=0x017 align=1 (i32.const 0) (local.get 0x017))
    (i64.store offset=0x018 align=1 (i32.const 0) (local.get 0x018))
    (i64.store offset=0x019 align=1 (i32.const 0) (local.get 0x019))
    (i64.store offset=0x01a align=1 (i32.const 0) (local.get 0x01a))
    (i64.store offset=0x01b align=1 (i32.const 0) (local.get 0x01b))
    (i64.store offset=0x01c align=1 (i32.const 0) (local.get 0x01c))
    (i64.store offset=0x01d align=1 (i32.const 0) (local.get 0x01d))
    (i64.store offset=0x01e align=1 (i32.const 0) (local.get 0x01e))
    (i64.store offset=0x01f align=1 (i32.const 0) (local.get 0x01f))
    (i64.store offset=0x020 align=1 (i32.const 0) (local.get 0x020))
    (i64.store offset=0x021 align=1 (i32.const 0) (local.get 0x021))
    (i64.store offset=0x022 align=1 (i32.const 0) (local.get 0x022))
    (i64.store offset=0x023 align=1 (i32.const 0) (local.get 0x023))
    (i64.store offset=0x024 align=1 (i32.const 0) (local.get 0x024))
    (i64.store offset=0x025 align=1 (i32.const 0) (local.get 0x025))
    (i64.store offset=0x026 align=1 (i32.const 0) (local.get 0x026))
    (i64.store offset=0x027 align=1 (i32.const 0) (local.get 0x027))
    (i64.store offset=0x028 align=1 (i32.const 0) (local.get 0x028))
    (i64.store offset=0x029 align=1 (i32.const 0) (local.get 0x029))
    (i64.store offset=0x02a align=1 (i32.const 0) (local.get 0x02a))
    (i64.store offset=0x02b align=1 (i32.const 0) (local.get 0x02b))
    (i64.store offset=0x02c align=1 (i32.const 0) (local.get 0x02c))
    (i64.store offset=0x02d align=1 (i32.const 0) (local.get 0x02d))
    (i64.store offset=0x02e align=1 (i32.const 0) (local.get 0x02e))
    (i64.store offset=0x02f align=1 (i32.const 0) (local.get 0x02f))
    (i64.store offset=0x030 align=1 (i32.const 0) (local.get 0x030))
    (i64.store offset=0x031 align=1 (i32.const 0) (local.get 0x031))
    (i64.store offset=0x032 align=1 (i32.const 0) (local.get 0x032))
    (i64.store offset=0x033 align=1 (i32.const 0) (local.get 0x033))
    (i64.store offset=0x034 align=1 (i32.const 0) (local.get 0x034))
    (i64.store offset=0x035 align=1 (i32.const 0) (local.get 0x035))
    (i64.store offset=0x036 align=1 (i32.const 0) (local.get 0x036))
    (i64.store offset=0x037 align=1 (i32.const 0) (local.get 0x037))
    (i64.store offset=0x038 align=1 (i32.const 0) (local.get 0x038))
    (i64.store offset=0x039 align=1 (i32.const 0) (local.get 0x039))
    (i64.store offset=0x03a align=1 (i32.const 0) (local.get 0x03a))
    (i64.store offset=0x03b align=1 (i32.const 0) (local.get 0x03b))
    (i64.store offset=0x03c align=1 (i32.const 0) (local.get 0x03c))
    (i64.store offset=0x03d align=1 (i32.const 0) (local.get 0x03d))
    (i64.store offset=0x03e align=1 (i32.const 0) (local.get 0x03e))
    (i64.store offset=0x03f align=1 (i32.const 0) (local.get 0x03f))
    (i64.store offset=0x040 align=1 (i32.const 0) (local.get 0x040))
    (i64.store offset=0x041 align=1 (i32.const 0) (local.get 0x041))
    (i64.store offset=0x042 align=1 (i32.const 0) (local.get 0x042))
    (i64.store offset=0x043 align=1 (i32.const 0) (local.get 0x043))
    (i64.store offset=0x044 align=1 (i32.const 0) (local.get 0x044))
    (i64.store offset=0x045 align=1 (i32.const 0) (local.get 0x045))
    (i64.store offset=0x046 align=1 (i32.const 0) (local.get 0x046))
    (i64.store offset=0x047 align=1 (i32.const 0) (local.get 0x047))
    (i64.store offset=0x048 align=1 (i32.const 0) (local.get 0x048))
    (i64.store offset=0x049 align=1 (i32.const 0) (local.get 0x049))
    (i64.store offset=0x04a align=1 (i32.const 0) (local.get 0x04a))
    (i64.store offset=0x04b align=1 (i32.const 0) (local.get 0x04b))
    (i64.store offset=0x04c align=1 (i32.const 0) (local.get 0x04c))
    (i64.store offset=0x04d align=1 (i32.const 0) (local.get 0x04d))
    (i64.store offset=0x04e align=1 (i32.const 0) (local.get 0x04e))
    (i64.store offset=0x04f align=1 (i32.const 0) (local.get 0x04f))
    (i64.store offset=0x050 align=1 (i32.const 0) (local.get 0x050))
    (i64.store offset=0x051 align=1 (i32.const 0) (local.get 0x051))
    (i64.store offset=0x052 align=1 (i32.const 0) (local.get 0x052))
    (i64.store offset=0x053 align=1 (i32.const 0) (local.get 0x053))
    (i64.store offset=0x054 align=1 (i32.const 0) (local.get 0x054))
    (i64.store offset=0x055 align=1 (i32.const 0) (local.get 0x055))
    (i64.store offset=0x056 align=1 (i32.const 0) (local.get 0x056))
    (i64.store offset=0x057 align=1 (i32.const 0) (local.get 0x057))
    (i64.store offset=0x058 align=1 (i32.const 0) (local.get 0x058))
    (i64.store offset=0x059 align=1 (i32.const 0) (local.get 0x059))
    (i64.store offset=0x05a align=1 (i32.const 0) (local.get 0x05a))
    (i64.store offset=0x05b align=1 (i32.const 0) (local.get 0x05b))
    (i64.store offset=0x05c align=1 (i32.const 0) (local.get 0x05c))
    (i64.store offset=0x05d align=1 (i32.const 0) (local.get 0x05d))
    (i64.store offset=0x05e align=1 (i32.const 0) (local.get 0x05e))
    (i64.store offset=0x05f align=1 (i32.const 0) (local.get 0x05f))
    (i64.store offset=0x060 align=1 (i32.const 0) (local.get 0x060))
    (i64.store offset=0x061 align=1 (i32.const 0) (local.get 0x061))
    (i64.store offset=0x062 align=1 (i32.const 0) (local.get 0x062))
    (i64.store offset=0x063 align=1 (i32.const 0) (local.get 0x063))
    (i64.store offset=0x064 align=1 (i32.const 0) (local.get 0x064))
    (i64.store offset=0x065 align=1 (i32.const 0) (local.get 0x065))
    (i64.store offset=0x066 align=1 (i32.const 0) (local.get 0x066))
    (i64.store offset=0x067 align=1 (i32.const 0) (local.get 0x067))
    (i64.store offset=0x068 align=1 (i32.const 0) (local.get 0x068))
    (i64.store offset=0x069 align=1 (i32.const 0) (local.get 0x069))
    (i64.store offset=0x06a align=1 (i32.const 0) (local.get 0x06a))
    (i64.store offset=0x06b align=1 (i32.const 0) (local.get 0x06b))
    (i64.store offset=0x06c align=1 (i32.const 0) (local.get 0x06c))
    (i64.store offset=0x06d align=1 (i32.const 0) (local.get 0x06d))
    (i64.store offset=0x06e align=1 (i32.const 0) (local.get 0x06e))
    (i64.store offset=0x06f align=1 (i32.const 0) (local.get 0x06f))
    (i64.store offset=0x070 align=1 (i32.const 0) (local.get 0x070))
    (i64.store offset=0x071 align=1 (i32.const 0) (local.get 0x071))
    (i64.store offset=0x072 align=1 (i32.const 0) (local.get 0x072))
    (i64.store offset=0x073 align=1 (i32.const 0) (local.get 0x073))
    (i64.store offset=0x074 align=1 (i32.const 0) (local.get 0x074))
    (i64.store offset=0x075 align=1 (i32.const 0) (local.get 0x075))
    (i64.store offset=0x076 align=1 (i32.const 0) (local.get 0x076))
    (i64.store offset=0x077 align=1 (i32.const 0) (local.get 0x077))
    (i64.store offset=0x078 align=1 (i32.const 0) (local.get 0x078))
    (i64.store offset=0x079 align=1 (i32.const 0) (local.get 0x079))
    (i64.store offset=0x07a align=1 (i32.const 0) (local.get 0x07a))
    (i64.store offset=0x07b align=1 (i32.const 0) (local.get 0x07b))
    (i64.store offset=0x07c align=1 (i32.const 0) (local.get 0x07c))
    (i64.store offset=0x07d align=1 (i32.const 0) (local.get 0x07d))
    (i64.store offset=0x07e align=1 (i32.const 0) (local.get 0x07e))
    (i64.store offset=0x07f align=1 (i32.const 0) (local.get 0x07f))
    (i64.store offset=0x080 align=1 (i32.const 0) (local.get 0x080))
    (i64.store offset=0x081 align=1 (i32.const 0) (local.get 0x081))
    (i64.store offset=0x082 align=1 (i32.const 0) (local.get 0x082))
    (i64.store offset=0x083 align=1 (i32.const 0) (local.get 0x083))
    (i64.store offset=0x084 align=1 (i32.const 0) (local.get 0x084))
    (i64.store offset=0x085 align=1 (i32.const 0) (local.get 0x085))
    (i64.store offset=0x086 align=1 (i32.const 0) (local.get 0x086))
    (i64.store offset=0x087 align=1 (i32.const 0) (local.get 0x087))
    (i64.store offset=0x088 align=1 (i32.const 0) (local.get 0x088))
    (i64.store offset=0x089 align=1 (i32.const 0) (local.get 0x089))
    (i64.store offset=0x08a align=1 (i32.const 0) (local.get 0x08a))
    (i64.store offset=0x08b align=1 (i32.const 0) (local.get 0x08b))
    (i64.store offset=0x08c align=1 (i32.const 0) (local.get 0x08c))
    (i64.store offset=0x08d align=1 (i32.const 0) (local.get 0x08d))
    (i64.store offset=0x08e align=1 (i32.const 0) (local.get 0x08e))
    (i64.store offset=0x08f align=1 (i32.const 0) (local.get 0x08f))
    (i64.store offset=0x090 align=1 (i32.const 0) (local.get 0x090))
    (i64.store offset=0x091 align=1 (i32.const 0) (local.get 0x091))
    (i64.store offset=0x092 align=1 (i32.const 0) (local.get 0x092))
    (i64.store offset=0x093 align=1 (i32.const 0) (local.get 0x093))
    (i64.store offset=0x094 align=1 (i32.const 0) (local.get 0x094))
    (i64.store offset=0x095 align=1 (i32.const 0) (local.get 0x095))
    (i64.store offset=0x096 align=1 (i32.const 0) (local.get 0x096))
    (i64.store offset=0x097 align=1 (i32.const 0) (local.get 0x097))
    (i64.store offset=0x098 align=1 (i32.const 0) (local.get 0x098))
    (i64.store offset=0x099 align=1 (i32.const 0) (local.get 0x099))
    (i64.store offset=0x09a align=1 (i32.const 0) (local.get 0x09a))
    (i64.store offset=0x09b align=1 (i32.const 0) (local.get 0x09b))
    (i64.store offset=0x09c align=1 (i32.const 0) (local.get 0x09c))
    (i64.store offset=0x09d align=1 (i32.const 0) (local.get 0x09d))
    (i64.store offset=0x09e align=1 (i32.const 0) (local.get 0x09e))
    (i64.store offset=0x09f align=1 (i32.const 0) (local.get 0x09f))
    (i64.store offset=0x0a0 align=1 (i32.const 0) (local.get 0x0a0))
    (i64.store offset=0x0a1 align=1 (i32.const 0) (local.get 0x0a1))
    (i64.store offset=0x0a2 align=1 (i32.const 0) (local.get 0x0a2))
    (i64.store offset=0x0a3 align=1 (i32.const 0) (local.get 0x0a3))
    (i64.store offset=0x0a4 align=1 (i32.const 0) (local.get 0x0a4))
    (i64.store offset=0x0a5 align=1 (i32.const 0) (local.get 0x0a5))
    (i64.store offset=0x0a6 align=1 (i32.const 0) (local.get 0x0a6))
    (i64.store offset=0x0a7 align=1 (i32.const 0) (local.get 0x0a7))
    (i64.store offset=0x0a8 align=1 (i32.const 0) (local.get 0x0a8))
    (i64.store offset=0x0a9 align=1 (i32.const 0) (local.get 0x0a9))
    (i64.store offset=0x0aa align=1 (i32.const 0) (local.get 0x0aa))
    (i64.store offset=0x0ab align=1 (i32.const 0) (local.get 0x0ab))
    (i64.store offset=0x0ac align=1 (i32.const 0) (local.get 0x0ac))
    (i64.store offset=0x0ad align=1 (i32.const 0) (local.get 0x0ad))
    (i64.store offset=0x0ae align=1 (i32.const 0) (local.get 0x0ae))
    (i64.store offset=0x0af align=1 (i32.const 0) (local.get 0x0af))
    (i64.store offset=0x0b0 align=1 (i32.const 0) (local.get 0x0b0))
    (i64.store offset=0x0b1 align=1 (i32.const 0) (local.get 0x0b1))
    (i64.store offset=0x0b2 align=1 (i32.const 0) (local.get 0x0b2))
    (i64.store offset=0x0b3 align=1 (i32.const 0) (local.get 0x0b3))
    (i64.store offset=0x0b4 align=1 (i32.const 0) (local.get 0x0b4))
    (i64.store offset=0x0b5 align=1 (i32.const 0) (local.get 0x0b5))
    (i64.store offset=0x0b6 align=1 (i32.const 0) (local.get 0x0b6))
    (i64.store offset=0x0b7 align=1 (i32.const 0) (local.get 0x0b7))
    (i64.store offset=0x0b8 align=1 (i32.const 0) (local.get 0x0b8))
    (i64.store offset=0x0b9 align=1 (i32.const 0) (local.get 0x0b9))
    (i64.store offset=0x0ba align=1 (i32.const 0) (local.get 0x0ba))
    (i64.store offset=0x0bb align=1 (i32.const 0) (local.get 0x0bb))
    (i64.store offset=0x0bc align=1 (i32.const 0) (local.get 0x0bc))
    (i64.store offset=0x0bd align=1 (i32.const 0) (local.get 0x0bd))
    (i64.store offset=0x0be align=1 (i32.const 0) (local.get 0x0be))
    (i64.store offset=0x0bf align=1 (i32.const 0) (local.get 0x0bf))
    (i64.store offset=0x0c0 align=1 (i32.const 0) (local.get 0x0c0))
    (i64.store offset=0x0c1 align=1 (i32.const 0) (local.get 0x0c1))
    (i64.store offset=0x0c2 align=1 (i32.const 0) (local.get 0x0c2))
    (i64.store offset=0x0c3 align=1 (i32.const 0) (local.get 0x0c3))
    (i64.store offset=0x0c4 align=1 (i32.const 0) (local.get 0x0c4))
    (i64.store offset=0x0c5 align=1 (i32.const 0) (local.get 0x0c5))
    (i64.store offset=0x0c6 align=1 (i32.const 0) (local.get 0x0c6))
    (i64.store offset=0x0c7 align=1 (i32.const 0) (local.get 0x0c7))
    (i64.store offset=0x0c8 align=1 (i32.const 0) (local.get 0x0c8))
    (i64.store offset=0x0c9 align=1 (i32.const 0) (local.get 0x0c9))
    (i64.store offset=0x0ca align=1 (i32.const 0) (local.get 0x0ca))
    (i64.store offset=0x0cb align=1 (i32.const 0) (local.get 0x0cb))
    (i64.store offset=0x0cc align=1 (i32.const 0) (local.get 0x0cc))
    (i64.store offset=0x0cd align=1 (i32.const 0) (local.get 0x0cd))
    (i64.store offset=0x0ce align=1 (i32.const 0) (local.get 0x0ce))
    (i64.store offset=0x0cf align=1 (i32.const 0) (local.get 0x0cf))
    (i64.store offset=0x0d0 align=1 (i32.const 0) (local.get 0x0d0))
    (i64.store offset=0x0d1 align=1 (i32.const 0) (local.get 0x0d1))
    (i64.store offset=0x0d2 align=1 (i32.const 0) (local.get 0x0d2))
    (i64.store offset=0x0d3 align=1 (i32.const 0) (local.get 0x0d3))
    (i64.store offset=0x0d4 align=1 (i32.const 0) (local.get 0x0d4))
    (i64.store offset=0x0d5 align=1 (i32.const 0) (local.get 0x0d5))
    (i64.store offset=0x0d6 align=1 (i32.const 0) (local.get 0x0d6))
    (i64.store offset=0x0d7 align=1 (i32.const 0) (local.get 0x0d7))
    (i64.store offset=0x0d8 align=1 (i32.const 0) (local.get 0x0d8))
    (i64.store offset=0x0d9 align=1 (i32.const 0) (local.get 0x0d9))
    (i64.store offset=0x0da align=1 (i32.const 0) (local.get 0x0da))
    (i64.store offset=0x0db align=1 (i32.const 0) (local.get 0x0db))
    (i64.store offset=0x0dc align=1 (i32.const 0) (local.get 0x0dc))
    (i64.store offset=0x0dd align=1 (i32.const 0) (local.get 0x0dd))
    (i64.store offset=0x0de align=1 (i32.const 0) (local.get 0x0de))
    (i64.store offset=0x0df align=1 (i32.const 0) (local.get 0x0df))
    (i64.store offset=0x0e0 align=1 (i32.const 0) (local.get 0x0e0))
    (i64.store offset=0x0e1 align=1 (i32.const 0) (local.get 0x0e1))
    (i64.store offset=0x0e2 align=1 (i32.const 0) (local.get 0x0e2))
    (i64.store offset=0x0e3 align=1 (i32.const 0) (local.get 0x0e3))
    (i64.store offset=0x0e4 align=1 (i32.const 0) (local.get 0x0e4))
    (i64.store offset=0x0e5 align=1 (i32.const 0) (local.get 0x0e5))
    (i64.store offset=0x0e6 align=1 (i32.const 0) (local.get 0x0e6))
    (i64.store offset=0x0e7 align=1 (i32.const 0) (local.get 0x0e7))
    (i64.store offset=0x0e8 align=1 (i32.const 0) (local.get 0x0e8))
    (i64.store offset=0x0e9 align=1 (i32.const 0) (local.get 0x0e9))
    (i64.store offset=0x0ea align=1 (i32.const 0) (local.get 0x0ea))
    (i64.store offset=0x0eb align=1 (i32.const 0) (local.get 0x0eb))
    (i64.store offset=0x0ec align=1 (i32.const 0) (local.get 0x0ec))
    (i64.store offset=0x0ed align=1 (i32.const 0) (local.get 0x0ed))
    (i64.store offset=0x0ee align=1 (i32.const 0) (local.get 0x0ee))
    (i64.store offset=0x0ef align=1 (i32.const 0) (local.get 0x0ef))
    (i64.store offset=0x0f0 align=1 (i32.const 0) (local.get 0x0f0))
    (i64.store offset=0x0f1 align=1 (i32.const 0) (local.get 0x0f1))
    (i64.store offset=0x0f2 align=1 (i32.const 0) (local.get 0x0f2))
    (i64.store offset=0x0f3 align=1 (i32.const 0) (local.get 0x0f3))
    (i64.store offset=0x0f4 align=1 (i32.const 0) (local.get 0x0f4))
    (i64.store offset=0x0f5 align=1 (i32.const 0) (local.get 0x0f5))
    (i64.store offset=0x0f6 align=1 (i32.const 0) (local.get 0x0f6))
    (i64.store offset=0x0f7 align=1 (i32.const 0) (local.get 0x0f7))
    (i64.store offset=0x0f8 align=1 (i32.const 0) (local.get 0x0f8))
    (i64.store offset=0x0f9 align=1 (i32.const 0) (local.get 0x0f9))
    (i64.store offset=0x0fa align=1 (i32.const 0) (local.get 0x0fa))
    (i64.store offset=0x0fb align=1 (i32.const 0) (local.get 0x0fb))
    (i64.store offset=0x0fc align=1 (i32.const 0) (local.get 0x0fc))
    (i64.store offset=0x0fd align=1 (i32.const 0) (local.get 0x0fd))
    (i64.store offset=0x0fe align=1 (i32.const 0) (local.get 0x0fe))
    (i64.store offset=0x0ff align=1 (i32.const 0) (local.get 0x0ff))
    (i64.store offset=0x100 align=1 (i32.const 0) (local.get 0x100))
    (i64.store offset=0x101 align=1 (i32.const 0) (local.get 0x101))
    (i64.store offset=0x102 align=1 (i32.const 0) (local.get 0x102))
    (i64.store offset=0x103 align=1 (i32.const 0) (local.get 0x103))
    (i64.store offset=0x104 align=1 (i32.const 0) (local.get 0x104))
    (i64.store offset=0x105 align=1 (i32.const 0) (local.get 0x105))
    (i64.store offset=0x106 align=1 (i32.const 0) (local.get 0x106))
    (i64.store offset=0x107 align=1 (i32.const 0) (local.get 0x107))
    (i64.store offset=0x108 align=1 (i32.const 0) (local.get 0x108))
    (i64.store offset=0x109 align=1 (i32.const 0) (local.get 0x109))
    (i64.store offset=0x10a align=1 (i32.const 0) (local.get 0x10a))
    (i64.store offset=0x10b align=1 (i32.const 0) (local.get 0x10b))
    (i64.store offset=0x10c align=1 (i32.const 0) (local.get 0x10c))
    (i64.store offset=0x10d align=1 (i32.const 0) (local.get 0x10d))
    (i64.store offset=0x10e align=1 (i32.const 0) (local.get 0x10e))
    (i64.store offset=0x10f align=1 (i32.const 0) (local.get 0x10f))
    (i64.store offset=0x110 align=1 (i32.const 0) (local.get 0x110))
    (i64.store offset=0x111 align=1 (i32.const 0) (local.get 0x111))
    (i64.store offset=0x112 align=1 (i32.const 0) (local.get 0x112))
    (i64.store offset=0x113 align=1 (i32.const 0) (local.get 0x113))
    (i64.store offset=0x114 align=1 (i32.const 0) (local.get 0x114))
    (i64.store offset=0x115 align=1 (i32.const 0) (local.get 0x115))
    (i64.store offset=0x116 align=1 (i32.const 0) (local.get 0x116))
    (i64.store offset=0x117 align=1 (i32.const 0) (local.get 0x117))
    (i64.store offset=0x118 align=1 (i32.const 0) (local.get 0x118))
    (i64.store offset=0x119 align=1 (i32.const 0) (local.get 0x119))
    (i64.store offset=0x11a align=1 (i32.const 0) (local.get 0x11a))
    (i64.store offset=0x11b align=1 (i32.const 0) (local.get 0x11b))
    (i64.store offset=0x11c align=1 (i32.const 0) (local.get 0x11c))
    (i64.store offset=0x11d align=1 (i32.const 0) (local.get 0x11d))
    (i64.store offset=0x11e align=1 (i32.const 0) (local.get 0x11e))
    (i64.store offset=0x11f align=1 (i32.const 0) (local.get 0x11f))
    (i64.store offset=0x120 align=1 (i32.const 0) (local.get 0x120))
    (i64.store offset=0x121 align=1 (i32.const 0) (local.get 0x121))
    (i64.store offset=0x122 align=1 (i32.const 0) (local.get 0x122))
    (i64.store offset=0x123 align=1 (i32.const 0) (local.get 0x123))
    (i64.store offset=0x124 align=1 (i32.const 0) (local.get 0x124))
    (i64.store offset=0x125 align=1 (i32.const 0) (local.get 0x125))
    (i64.store offset=0x126 align=1 (i32.const 0) (local.get 0x126))
    (i64.store offset=0x127 align=1 (i32.const 0) (local.get 0x127))
    (i64.store offset=0x128 align=1 (i32.const 0) (local.get 0x128))
    (i64.store offset=0x129 align=1 (i32.const 0) (local.get 0x129))
    (i64.store offset=0x12a align=1 (i32.const 0) (local.get 0x12a))
    (i64.store offset=0x12b align=1 (i32.const 0) (local.get 0x12b))
    (i64.store offset=0x12c align=1 (i32.const 0) (local.get 0x12c))
    (i64.store offset=0x12d align=1 (i32.const 0) (local.get 0x12d))
    (i64.store offset=0x12e align=1 (i32.const 0) (local.get 0x12e))
    (i64.store offset=0x12f align=1 (i32.const 0) (local.get 0x12f))
    (i64.store offset=0x130 align=1 (i32.const 0) (local.get 0x130))
    (i64.store offset=0x131 align=1 (i32.const 0) (local.get 0x131))
    (i64.store offset=0x132 align=1 (i32.const 0) (local.get 0x132))
    (i64.store offset=0x133 align=1 (i32.const 0) (local.get 0x133))
    (i64.store offset=0x134 align=1 (i32.const 0) (local.get 0x134))
    (i64.store offset=0x135 align=1 (i32.const 0) (local.get 0x135))
    (i64.store offset=0x136 align=1 (i32.const 0) (local.get 0x136))
    (i64.store offset=0x137 align=1 (i32.const 0) (local.get 0x137))
    (i64.store offset=0x138 align=1 (i32.const 0) (local.get 0x138))
    (i64.store offset=0x139 align=1 (i32.const 0) (local.get 0x139))
    (i64.store offset=0x13a align=1 (i32.const 0) (local.get 0x13a))
    (i64.store offset=0x13b align=1 (i32.const 0) (local.get 0x13b))
    (i64.store offset=0x13c align=1 (i32.const 0) (local.get 0x13c))
    (i64.store offset=0x13d align=1 (i32.const 0) (local.get 0x13d))
    (i64.store offset=0x13e align=1 (i32.const 0) (local.get 0x13e))
    (i64.store offset=0x13f align=1 (i32.const 0) (local.get 0x13f))
    (i64.store offset=0x140 align=1 (i32.const 0) (local.get 0x140))
    (i64.store offset=0x141 align=1 (i32.const 0) (local.get 0x141))
    (i64.store offset=0x142 align=1 (i32.const 0) (local.get 0x142))
    (i64.store offset=0x143 align=1 (i32.const 0) (local.get 0x143))
    (i64.store offset=0x144 align=1 (i32.const 0) (local.get 0x144))
    (i64.store offset=0x145 align=1 (i32.const 0) (local.get 0x145))
    (i64.store offset=0x146 align=1 (i32.const 0) (local.get 0x146))
    (i64.store offset=0x147 align=1 (i32.const 0) (local.get 0x147))
    (i64.store offset=0x148 align=1 (i32.const 0) (local.get 0x148))
    (i64.store offset=0x149 align=1 (i32.const 0) (local.get 0x149))
    (i64.store offset=0x14a align=1 (i32.const 0) (local.get 0x14a))
    (i64.store offset=0x14b align=1 (i32.const 0) (local.get 0x14b))
    (i64.store offset=0x14c align=1 (i32.const 0) (local.get 0x14c))
    (i64.store offset=0x14d align=1 (i32.const 0) (local.get 0x14d))
    (i64.store offset=0x14e align=1 (i32.const 0) (local.get 0x14e))
    (i64.store offset=0x14f align=1 (i32.const 0) (local.get 0x14f))
    (i64.store offset=0x150 align=1 (i32.const 0) (local.get 0x150))
    (i64.store offset=0x151 align=1 (i32.const 0) (local.get 0x151))
    (i64.store offset=0x152 align=1 (i32.const 0) (local.get 0x152))
    (i64.store offset=0x153 align=1 (i32.const 0) (local.get 0x153))
    (i64.store offset=0x154 align=1 (i32.const 0) (local.get 0x154))
    (i64.store offset=0x155 align=1 (i32.const 0) (local.get 0x155))
    (i64.store offset=0x156 align=1 (i32.const 0) (local.get 0x156))
    (i64.store offset=0x157 align=1 (i32.const 0) (local.get 0x157))
    (i64.store offset=0x158 align=1 (i32.const 0) (local.get 0x158))
    (i64.store offset=0x159 align=1 (i32.const 0) (local.get 0x159))
    (i64.store offset=0x15a align=1 (i32.const 0) (local.get 0x15a))
    (i64.store offset=0x15b align=1 (i32.const 0) (local.get 0x15b))
    (i64.store offset=0x15c align=1 (i32.const 0) (local.get 0x15c))
    (i64.store offset=0x15d align=1 (i32.const 0) (local.get 0x15d))
    (i64.store offset=0x15e align=1 (i32.const 0) (local.get 0x15e))
    (i64.store offset=0x15f align=1 (i32.const 0) (local.get 0x15f))
    (i64.store offset=0x160 align=1 (i32.const 0) (local.get 0x160))
    (i64.store offset=0x161 align=1 (i32.const 0) (local.get 0x161))
    (i64.store offset=0x162 align=1 (i32.const 0) (local.get 0x162))
    (i64.store offset=0x163 align=1 (i32.const 0) (local.get 0x163))
    (i64.store offset=0x164 align=1 (i32.const 0) (local.get 0x164))
    (i64.store offset=0x165 align=1 (i32.const 0) (local.get 0x165))
    (i64.store offset=0x166 align=1 (i32.const 0) (local.get 0x166))
    (i64.store offset=0x167 align=1 (i32.const 0) (local.get 0x167))
    (i64.store offset=0x168 align=1 (i32.const 0) (local.get 0x168))
    (i64.store offset=0x169 align=1 (i32.const 0) (local.get 0x169))
    (i64.store offset=0x16a align=1 (i32.const 0) (local.get 0x16a))
    (i64.store offset=0x16b align=1 (i32.const 0) (local.get 0x16b))
    (i64.store offset=0x16c align=1 (i32.const 0) (local.get 0x16c))
    (i64.store offset=0x16d align=1 (i32.const 0) (local.get 0x16d))
    (i64.store offset=0x16e align=1 (i32.const 0) (local.get 0x16e))
    (i64.store offset=0x16f align=1 (i32.const 0) (local.get 0x16f))
    (i64.store offset=0x170 align=1 (i32.const 0) (local.get 0x170))
    (i64.store offset=0x171 align=1 (i32.const 0) (local.get 0x171))
    (i64.store offset=0x172 align=1 (i32.const 0) (local.get 0x172))
    (i64.store offset=0x173 align=1 (i32.const 0) (local.get 0x173))
    (i64.store offset=0x174 align=1 (i32.const 0) (local.get 0x174))
    (i64.store offset=0x175 align=1 (i32.const 0) (local.get 0x175))
    (i64.store offset=0x176 align=1 (i32.const 0) (local.get 0x176))
    (i64.store offset=0x177 align=1 (i32.const 0) (local.get 0x177))
    (i64.store offset=0x178 align=1 (i32.const 0) (local.get 0x178))
    (i64.store offset=0x179 align=1 (i32.const 0) (local.get 0x179))
    (i64.store offset=0x17a align=1 (i32.const 0) (local.get 0x17a))
    (i64.store offset=0x17b align=1 (i32.const 0) (local.get 0x17b))
    (i64.store offset=0x17c align=1 (i32.const 0) (local.get 0x17c))
    (i64.store offset=0x17d align=1 (i32.const 0) (local.get 0x17d))
    (i64.store offset=0x17e align=1 (i32.const 0) (local.get 0x17e))
    (i64.store offset=0x17f align=1 (i32.const 0) (local.get 0x17f))
    (i64.store offset=0x180 align=1 (i32.const 0) (local.get 0x180))
    (i64.store offset=0x181 align=1 (i32.const 0) (local.get 0x181))
    (i64.store offset=0x182 align=1 (i32.const 0) (local.get 0x182))
    (i64.store offset=0x183 align=1 (i32.const 0) (local.get 0x183))
    (i64.store offset=0x184 align=1 (i32.const 0) (local.get 0x184))
    (i64.store offset=0x185 align=1 (i32.const 0) (local.get 0x185))
    (i64.store offset=0x186 align=1 (i32.const 0) (local.get 0x186))
    (i64.store offset=0x187 align=1 (i32.const 0) (local.get 0x187))
    (i64.store offset=0x188 align=1 (i32.const 0) (local.get 0x188))
    (i64.store offset=0x189 align=1 (i32.const 0) (local.get 0x189))
    (i64.store offset=0x18a align=1 (i32.const 0) (local.get 0x18a))
    (i64.store offset=0x18b align=1 (i32.const 0) (local.get 0x18b))
    (i64.store offset=0x18c align=1 (i32.const 0) (local.get 0x18c))
    (i64.store offset=0x18d align=1 (i32.const 0) (local.get 0x18d))
    (i64.store offset=0x18e align=1 (i32.const 0) (local.get 0x18e))
    (i64.store offset=0x18f align=1 (i32.const 0) (local.get 0x18f))
    (i64.store offset=0x190 align=1 (i32.const 0) (local.get 0x190))
    (i64.store offset=0x191 align=1 (i32.const 0) (local.get 0x191))
    (i64.store offset=0x192 align=1 (i32.const 0) (local.get 0x192))
    (i64.store offset=0x193 align=1 (i32.const 0) (local.get 0x193))
    (i64.store offset=0x194 align=1 (i32.const 0) (local.get 0x194))
    (i64.store offset=0x195 align=1 (i32.const 0) (local.get 0x195))
    (i64.store offset=0x196 align=1 (i32.const 0) (local.get 0x196))
    (i64.store offset=0x197 align=1 (i32.const 0) (local.get 0x197))
    (i64.store offset=0x198 align=1 (i32.const 0) (local.get 0x198))
    (i64.store offset=0x199 align=1 (i32.const 0) (local.get 0x199))
    (i64.store offset=0x19a align=1 (i32.const 0) (local.get 0x19a))
    (i64.store offset=0x19b align=1 (i32.const 0) (local.get 0x19b))
    (i64.store offset=0x19c align=1 (i32.const 0) (local.get 0x19c))
    (i64.store offset=0x19d align=1 (i32.const 0) (local.get 0x19d))
    (i64.store offset=0x19e align=1 (i32.const 0) (local.get 0x19e))
    (i64.store offset=0x19f align=1 (i32.const 0) (local.get 0x19f))
    (i64.store offset=0x1a0 align=1 (i32.const 0) (local.get 0x1a0))
    (i64.store offset=0x1a1 align=1 (i32.const 0) (local.get 0x1a1))
    (i64.store offset=0x1a2 align=1 (i32.const 0) (local.get 0x1a2))
    (i64.store offset=0x1a3 align=1 (i32.const 0) (local.get 0x1a3))
    (i64.store offset=0x1a4 align=1 (i32.const 0) (local.get 0x1a4))
    (i64.store offset=0x1a5 align=1 (i32.const 0) (local.get 0x1a5))
    (i64.store offset=0x1a6 align=1 (i32.const 0) (local.get 0x1a6))
    (i64.store offset=0x1a7 align=1 (i32.const 0) (local.get 0x1a7))
    (i64.store offset=0x1a8 align=1 (i32.const 0) (local.get 0x1a8))
    (i64.store offset=0x1a9 align=1 (i32.const 0) (local.get 0x1a9))
    (i64.store offset=0x1aa align=1 (i32.const 0) (local.get 0x1aa))
    (i64.store offset=0x1ab align=1 (i32.const 0) (local.get 0x1ab))
    (i64.store offset=0x1ac align=1 (i32.const 0) (local.get 0x1ac))
    (i64.store offset=0x1ad align=1 (i32.const 0) (local.get 0x1ad))
    (i64.store offset=0x1ae align=1 (i32.const 0) (local.get 0x1ae))
    (i64.store offset=0x1af align=1 (i32.const 0) (local.get 0x1af))
    (i64.store offset=0x1b0 align=1 (i32.const 0) (local.get 0x1b0))
    (i64.store offset=0x1b1 align=1 (i32.const 0) (local.get 0x1b1))
    (i64.store offset=0x1b2 align=1 (i32.const 0) (local.get 0x1b2))
    (i64.store offset=0x1b3 align=1 (i32.const 0) (local.get 0x1b3))
    (i64.store offset=0x1b4 align=1 (i32.const 0) (local.get 0x1b4))
    (i64.store offset=0x1b5 align=1 (i32.const 0) (local.get 0x1b5))
    (i64.store offset=0x1b6 align=1 (i32.const 0) (local.get 0x1b6))
    (i64.store offset=0x1b7 align=1 (i32.const 0) (local.get 0x1b7))
    (i64.store offset=0x1b8 align=1 (i32.const 0) (local.get 0x1b8))
    (i64.store offset=0x1b9 align=1 (i32.const 0) (local.get 0x1b9))
    (i64.store offset=0x1ba align=1 (i32.const 0) (local.get 0x1ba))
    (i64.store offset=0x1bb align=1 (i32.const 0) (local.get 0x1bb))
    (i64.store offset=0x1bc align=1 (i32.const 0) (local.get 0x1bc))
    (i64.store offset=0x1bd align=1 (i32.const 0) (local.get 0x1bd))
    (i64.store offset=0x1be align=1 (i32.const 0) (local.get 0x1be))
    (i64.store offset=0x1bf align=1 (i32.const 0) (local.get 0x1bf))
    (i64.store offset=0x1c0 align=1 (i32.const 0) (local.get 0x1c0))
    (i64.store offset=0x1c1 align=1 (i32.const 0) (local.get 0x1c1))
    (i64.store offset=0x1c2 align=1 (i32.const 0) (local.get 0x1c2))
    (i64.store offset=0x1c3 align=1 (i32.const 0) (local.get 0x1c3))
    (i64.store offset=0x1c4 align=1 (i32.const 0) (local.get 0x1c4))
    (i64.store offset=0x1c5 align=1 (i32.const 0) (local.get 0x1c5))
    (i64.store offset=0x1c6 align=1 (i32.const 0) (local.get 0x1c6))
    (i64.store offset=0x1c7 align=1 (i32.const 0) (local.get 0x1c7))
    (i64.store offset=0x1c8 align=1 (i32.const 0) (local.get 0x1c8))
    (i64.store offset=0x1c9 align=1 (i32.const 0) (local.get 0x1c9))
    (i64.store offset=0x1ca align=1 (i32.const 0) (local.get 0x1ca))
    (i64.store offset=0x1cb align=1 (i32.const 0) (local.get 0x1cb))
    (i64.store offset=0x1cc align=1 (i32.const 0) (local.get 0x1cc))
    (i64.store offset=0x1cd align=1 (i32.const 0) (local.get 0x1cd))
    (i64.store offset=0x1ce align=1 (i32.const 0) (local.get 0x1ce))
    (i64.store offset=0x1cf align=1 (i32.const 0) (local.get 0x1cf))
    (i64.store offset=0x1d0 align=1 (i32.const 0) (local.get 0x1d0))
    (i64.store offset=0x1d1 align=1 (i32.const 0) (local.get 0x1d1))
    (i64.store offset=0x1d2 align=1 (i32.const 0) (local.get 0x1d2))
    (i64.store offset=0x1d3 align=1 (i32.const 0) (local.get 0x1d3))
    (i64.store offset=0x1d4 align=1 (i32.const 0) (local.get 0x1d4))
    (i64.store offset=0x1d5 align=1 (i32.const 0) (local.get 0x1d5))
    (i64.store offset=0x1d6 align=1 (i32.const 0) (local.get 0x1d6))
    (i64.store offset=0x1d7 align=1 (i32.const 0) (local.get 0x1d7))
    (i64.store offset=0x1d8 align=1 (i32.const 0) (local.get 0x1d8))
    (i64.store offset=0x1d9 align=1 (i32.const 0) (local.get 0x1d9))
    (i64.store offset=0x1da align=1 (i32.const 0) (local.get 0x1da))
    (i64.store offset=0x1db align=1 (i32.const 0) (local.get 0x1db))
    (i64.store offset=0x1dc align=1 (i32.const 0) (local.get 0x1dc))
    (i64.store offset=0x1dd align=1 (i32.const 0) (local.get 0x1dd))
    (i64.store offset=0x1de align=1 (i32.const 0) (local.get 0x1de))
    (i64.store offset=0x1df align=1 (i32.const 0) (local.get 0x1df))
    (i64.store offset=0x1e0 align=1 (i32.const 0) (local.get 0x1e0))
    (i64.store offset=0x1e1 align=1 (i32.const 0) (local.get 0x1e1))
    (i64.store offset=0x1e2 align=1 (i32.const 0) (local.get 0x1e2))
    (i64.store offset=0x1e3 align=1 (i32.const 0) (local.get 0x1e3))
    (i64.store offset=0x1e4 align=1 (i32.const 0) (local.get 0x1e4))
    (i64.store offset=0x1e5 align=1 (i32.const 0) (local.get 0x1e5))
    (i64.store offset=0x1e6 align=1 (i32.const 0) (local.get 0x1e6))
    (i64.store offset=0x1e7 align=1 (i32.const 0) (local.get 0x1e7))
    (i64.store offset=0x1e8 align=1 (i32.const 0) (local.get 0x1e8))
    (i64.store offset=0x1e9 align=1 (i32.const 0) (local.get 0x1e9))
    (i64.store offset=0x1ea align=1 (i32.const 0) (local.get 0x1ea))
    (i64.store offset=0x1eb align=1 (i32.const 0) (local.get 0x1eb))
    (i64.store offset=0x1ec align=1 (i32.const 0) (local.get 0x1ec))
    (i64.store offset=0x1ed align=1 (i32.const 0) (local.get 0x1ed))
    (i64.store offset=0x1ee align=1 (i32.const 0) (local.get 0x1ee))
    (i64.store offset=0x1ef align=1 (i32.const 0) (local.get 0x1ef))
    (i64.store offset=0x1f0 align=1 (i32.const 0) (local.get 0x1f0))
    (i64.store offset=0x1f1 align=1 (i32.const 0) (local.get 0x1f1))
    (i64.store offset=0x1f2 align=1 (i32.const 0) (local.get 0x1f2))
    (i64.store offset=0x1f3 align=1 (i32.const 0) (local.get 0x1f3))
    (i64.store offset=0x1f4 align=1 (i32.const 0) (local.get 0x1f4))
    (i64.store offset=0x1f5 align=1 (i32.const 0) (local.get 0x1f5))
    (i64.store offset=0x1f6 align=1 (i32.const 0) (local.get 0x1f6))
    (i64.store offset=0x1f7 align=1 (i32.const 0) (local.get 0x1f7))
    (i64.store offset=0x1f8 align=1 (i32.const 0) (local.get 0x1f8))
    (i64.store offset=0x1f9 align=1 (i32.const 0) (local.get 0x1f9))
    (i64.store offset=0x1fa align=1 (i32.const 0) (local.get 0x1fa))
    (i64.store offset=0x1fb align=1 (i32.const 0) (local.get 0x1fb))
    (i64.store offset=0x1fc align=1 (i32.const 0) (local.get 0x1fc))
    (i64.store offset=0x1fd align=1 (i32.const 0) (local.get 0x1fd))
    (i64.store offset=0x1fe align=1 (i32.const 0) (local.get 0x1fe))
    (i64.store offset=0x1ff align=1 (i32.const 0) (local.get 0x1ff))
    (i64.store offset=0x200 align=1 (i32.const 0) (local.get 0x200))
    (i64.store offset=0x201 align=1 (i32.const 0) (local.get 0x201))
    (i64.store offset=0x202 align=1 (i32.const 0) (local.get 0x202))
    (i64.store offset=0x203 align=1 (i32.const 0) (local.get 0x203))
    (i64.store offset=0x204 align=1 (i32.const 0) (local.get 0x204))
    (i64.store offset=0x205 align=1 (i32.const 0) (local.get 0x205))
    (i64.store offset=0x206 align=1 (i32.const 0) (local.get 0x206))
    (i64.store offset=0x207 align=1 (i32.const 0) (local.get 0x207))
    (i64.store offset=0x208 align=1 (i32.const 0) (local.get 0x208))
    (i64.store offset=0x209 align=1 (i32.const 0) (local.get 0x209))
    (i64.store offset=0x20a align=1 (i32.const 0) (local.get 0x20a))
    (i64.store offset=0x20b align=1 (i32.const 0) (local.get 0x20b))
    (i64.store offset=0x20c align=1 (i32.const 0) (local.get 0x20c))
    (i64.store offset=0x20d align=1 (i32.const 0) (local.get 0x20d))
    (i64.store offset=0x20e align=1 (i32.const 0) (local.get 0x20e))
    (i64.store offset=0x20f align=1 (i32.const 0) (local.get 0x20f))
    (i64.store offset=0x210 align=1 (i32.const 0) (local.get 0x210))
    (i64.store offset=0x211 align=1 (i32.const 0) (local.get 0x211))
    (i64.store offset=0x212 align=1 (i32.const 0) (local.get 0x212))
    (i64.store offset=0x213 align=1 (i32.const 0) (local.get 0x213))
    (i64.store offset=0x214 align=1 (i32.const 0) (local.get 0x214))
    (i64.store offset=0x215 align=1 (i32.const 0) (local.get 0x215))
    (i64.store offset=0x216 align=1 (i32.const 0) (local.get 0x216))
    (i64.store offset=0x217 align=1 (i32.const 0) (local.get 0x217))
    (i64.store offset=0x218 align=1 (i32.const 0) (local.get 0x218))
    (i64.store offset=0x219 align=1 (i32.const 0) (local.get 0x219))
    (i64.store offset=0x21a align=1 (i32.const 0) (local.get 0x21a))
    (i64.store offset=0x21b align=1 (i32.const 0) (local.get 0x21b))
    (i64.store offset=0x21c align=1 (i32.const 0) (local.get 0x21c))
    (i64.store offset=0x21d align=1 (i32.const 0) (local.get 0x21d))
    (i64.store offset=0x21e align=1 (i32.const 0) (local.get 0x21e))
    (i64.store offset=0x21f align=1 (i32.const 0) (local.get 0x21f))
    (i64.store offset=0x220 align=1 (i32.const 0) (local.get 0x220))
    (i64.store offset=0x221 align=1 (i32.const 0) (local.get 0x221))
    (i64.store offset=0x222 align=1 (i32.const 0) (local.get 0x222))
    (i64.store offset=0x223 align=1 (i32.const 0) (local.get 0x223))
    (i64.store offset=0x224 align=1 (i32.const 0) (local.get 0x224))
    (i64.store offset=0x225 align=1 (i32.const 0) (local.get 0x225))
    (i64.store offset=0x226 align=1 (i32.const 0) (local.get 0x226))
    (i64.store offset=0x227 align=1 (i32.const 0) (local.get 0x227))
    (i64.store offset=0x228 align=1 (i32.const 0) (local.get 0x228))
    (i64.store offset=0x229 align=1 (i32.const 0) (local.get 0x229))
    (i64.store offset=0x22a align=1 (i32.const 0) (local.get 0x22a))
    (i64.store offset=0x22b align=1 (i32.const 0) (local.get 0x22b))
    (i64.store offset=0x22c align=1 (i32.const 0) (local.get 0x22c))
    (i64.store offset=0x22d align=1 (i32.const 0) (local.get 0x22d))
    (i64.store offset=0x22e align=1 (i32.const 0) (local.get 0x22e))
    (i64.store offset=0x22f align=1 (i32.const 0) (local.get 0x22f))
    (i64.store offset=0x230 align=1 (i32.const 0) (local.get 0x230))
    (i64.store offset=0x231 align=1 (i32.const 0) (local.get 0x231))
    (i64.store offset=0x232 align=1 (i32.const 0) (local.get 0x232))
    (i64.store offset=0x233 align=1 (i32.const 0) (local.get 0x233))
    (i64.store offset=0x234 align=1 (i32.const 0) (local.get 0x234))
    (i64.store offset=0x235 align=1 (i32.const 0) (local.get 0x235))
    (i64.store offset=0x236 align=1 (i32.const 0) (local.get 0x236))
    (i64.store offset=0x237 align=1 (i32.const 0) (local.get 0x237))
    (i64.store offset=0x238 align=1 (i32.const 0) (local.get 0x238))
    (i64.store offset=0x239 align=1 (i32.const 0) (local.get 0x239))
    (i64.store offset=0x23a align=1 (i32.const 0) (local.get 0x23a))
    (i64.store offset=0x23b align=1 (i32.const 0) (local.get 0x23b))
    (i64.store offset=0x23c align=1 (i32.const 0) (local.get 0x23c))
    (i64.store offset=0x23d align=1 (i32.const 0) (local.get 0x23d))
    (i64.store offset=0x23e align=1 (i32.const 0) (local.get 0x23e))
    (i64.store offset=0x23f align=1 (i32.const 0) (local.get 0x23f))
    (i64.store offset=0x240 align=1 (i32.const 0) (local.get 0x240))
    (i64.store offset=0x241 align=1 (i32.const 0) (local.get 0x241))
    (i64.store offset=0x242 align=1 (i32.const 0) (local.get 0x242))
    (i64.store offset=0x243 align=1 (i32.const 0) (local.get 0x243))
    (i64.store offset=0x244 align=1 (i32.const 0) (local.get 0x244))
    (i64.store offset=0x245 align=1 (i32.const 0) (local.get 0x245))
    (i64.store offset=0x246 align=1 (i32.const 0) (local.get 0x246))
    (i64.store offset=0x247 align=1 (i32.const 0) (local.get 0x247))
    (i64.store offset=0x248 align=1 (i32.const 0) (local.get 0x248))
    (i64.store offset=0x249 align=1 (i32.const 0) (local.get 0x249))
    (i64.store offset=0x24a align=1 (i32.const 0) (local.get 0x24a))
    (i64.store offset=0x24b align=1 (i32.const 0) (local.get 0x24b))
    (i64.store offset=0x24c align=1 (i32.const 0) (local.get 0x24c))
    (i64.store offset=0x24d align=1 (i32.const 0) (local.get 0x24d))
    (i64.store offset=0x24e align=1 (i32.const 0) (local.get 0x24e))
    (i64.store offset=0x24f align=1 (i32.const 0) (local.get 0x24f))
    (i64.store offset=0x250 align=1 (i32.const 0) (local.get 0x250))
    (i64.store offset=0x251 align=1 (i32.const 0) (local.get 0x251))
    (i64.store offset=0x252 align=1 (i32.const 0) (local.get 0x252))
    (i64.store offset=0x253 align=1 (i32.const 0) (local.get 0x253))
    (i64.store offset=0x254 align=1 (i32.const 0) (local.get 0x254))
    (i64.store offset=0x255 align=1 (i32.const 0) (local.get 0x255))
    (i64.store offset=0x256 align=1 (i32.const 0) (local.get 0x256))
    (i64.store offset=0x257 align=1 (i32.const 0) (local.get 0x257))
    (i64.store offset=0x258 align=1 (i32.const 0) (local.get 0x258))
    (i64.store offset=0x259 align=1 (i32.const 0) (local.get 0x259))
    (i64.store offset=0x25a align=1 (i32.const 0) (local.get 0x25a))
    (i64.store offset=0x25b align=1 (i32.const 0) (local.get 0x25b))
    (i64.store offset=0x25c align=1 (i32.const 0) (local.get 0x25c))
    (i64.store offset=0x25d align=1 (i32.const 0) (local.get 0x25d))
    (i64.store offset=0x25e align=1 (i32.const 0) (local.get 0x25e))
    (i64.store offset=0x25f align=1 (i32.const 0) (local.get 0x25f))
    (i64.store offset=0x260 align=1 (i32.const 0) (local.get 0x260))
    (i64.store offset=0x261 align=1 (i32.const 0) (local.get 0x261))
    (i64.store offset=0x262 align=1 (i32.const 0) (local.get 0x262))
    (i64.store offset=0x263 align=1 (i32.const 0) (local.get 0x263))
    (i64.store offset=0x264 align=1 (i32.const 0) (local.get 0x264))
    (i64.store offset=0x265 align=1 (i32.const 0) (local.get 0x265))
    (i64.store offset=0x266 align=1 (i32.const 0) (local.get 0x266))
    (i64.store offset=0x267 align=1 (i32.const 0) (local.get 0x267))
    (i64.store offset=0x268 align=1 (i32.const 0) (local.get 0x268))
    (i64.store offset=0x269 align=1 (i32.const 0) (local.get 0x269))
    (i64.store offset=0x26a align=1 (i32.const 0) (local.get 0x26a))
    (i64.store offset=0x26b align=1 (i32.const 0) (local.get 0x26b))
    (i64.store offset=0x26c align=1 (i32.const 0) (local.get 0x26c))
    (i64.store offset=0x26d align=1 (i32.const 0) (local.get 0x26d))
    (i64.store offset=0x26e align=1 (i32.const 0) (local.get 0x26e))
    (i64.store offset=0x26f align=1 (i32.const 0) (local.get 0x26f))
    (i64.store offset=0x270 align=1 (i32.const 0) (local.get 0x270))
    (i64.store offset=0x271 align=1 (i32.const 0) (local.get 0x271))
    (i64.store offset=0x272 align=1 (i32.const 0) (local.get 0x272))
    (i64.store offset=0x273 align=1 (i32.const 0) (local.get 0x273))
    (i64.store offset=0x274 align=1 (i32.const 0) (local.get 0x274))
    (i64.store offset=0x275 align=1 (i32.const 0) (local.get 0x275))
    (i64.store offset=0x276 align=1 (i32.const 0) (local.get 0x276))
    (i64.store offset=0x277 align=1 (i32.const 0) (local.get 0x277))
    (i64.store offset=0x278 align=1 (i32.const 0) (local.get 0x278))
    (i64.store offset=0x279 align=1 (i32.const 0) (local.get 0x279))
    (i64.store offset=0x27a align=1 (i32.const 0) (local.get 0x27a))
    (i64.store offset=0x27b align=1 (i32.const 0) (local.get 0x27b))
    (i64.store offset=0x27c align=1 (i32.const 0) (local.get 0x27c))
    (i64.store offset=0x27d align=1 (i32.const 0) (local.get 0x27d))
    (i64.store offset=0x27e align=1 (i32.const 0) (local.get 0x27e))
    (i64.store offset=0x27f align=1 (i32.const 0) (local.get 0x27f))
    (i64.store offset=0x280 align=1 (i32.const 0) (local.get 0x280))
    (i64.store offset=0x281 align=1 (i32.const 0) (local.get 0x281))
    (i64.store offset=0x282 align=1 (i32.const 0) (local.get 0x282))
    (i64.store offset=0x283 align=1 (i32.const 0) (local.get 0x283))
    (i64.store offset=0x284 align=1 (i32.const 0) (local.get 0x284))
    (i64.store offset=0x285 align=1 (i32.const 0) (local.get 0x285))
    (i64.store offset=0x286 align=1 (i32.const 0) (local.get 0x286))
    (i64.store offset=0x287 align=1 (i32.const 0) (local.get 0x287))
    (i64.store offset=0x288 align=1 (i32.const 0) (local.get 0x288))
    (i64.store offset=0x289 align=1 (i32.const 0) (local.get 0x289))
    (i64.store offset=0x28a align=1 (i32.const 0) (local.get 0x28a))
    (i64.store offset=0x28b align=1 (i32.const 0) (local.get 0x28b))
    (i64.store offset=0x28c align=1 (i32.const 0) (local.get 0x28c))
    (i64.store offset=0x28d align=1 (i32.const 0) (local.get 0x28d))
    (i64.store offset=0x28e align=1 (i32.const 0) (local.get 0x28e))
    (i64.store offset=0x28f align=1 (i32.const 0) (local.get 0x28f))
    (i64.store offset=0x290 align=1 (i32.const 0) (local.get 0x290))
    (i64.store offset=0x291 align=1 (i32.const 0) (local.get 0x291))
    (i64.store offset=0x292 align=1 (i32.const 0) (local.get 0x292))
    (i64.store offset=0x293 align=1 (i32.const 0) (local.get 0x293))
    (i64.store offset=0x294 align=1 (i32.const 0) (local.get 0x294))
    (i64.store offset=0x295 align=1 (i32.const 0) (local.get 0x295))
    (i64.store offset=0x296 align=1 (i32.const 0) (local.get 0x296))
    (i64.store offset=0x297 align=1 (i32.const 0) (local.get 0x297))
    (i64.store offset=0x298 align=1 (i32.const 0) (local.get 0x298))
    (i64.store offset=0x299 align=1 (i32.const 0) (local.get 0x299))
    (i64.store offset=0x29a align=1 (i32.const 0) (local.get 0x29a))
    (i64.store offset=0x29b align=1 (i32.const 0) (local.get 0x29b))
    (i64.store offset=0x29c align=1 (i32.const 0) (local.get 0x29c))
    (i64.store offset=0x29d align=1 (i32.const 0) (local.get 0x29d))
    (i64.store offset=0x29e align=1 (i32.const 0) (local.get 0x29e))
    (i64.store offset=0x29f align=1 (i32.const 0) (local.get 0x29f))
    (i64.store offset=0x2a0 align=1 (i32.const 0) (local.get 0x2a0))
    (i64.store offset=0x2a1 align=1 (i32.const 0) (local.get 0x2a1))
    (i64.store offset=0x2a2 align=1 (i32.const 0) (local.get 0x2a2))
    (i64.store offset=0x2a3 align=1 (i32.const 0) (local.get 0x2a3))
    (i64.store offset=0x2a4 align=1 (i32.const 0) (local.get 0x2a4))
    (i64.store offset=0x2a5 align=1 (i32.const 0) (local.get 0x2a5))
    (i64.store offset=0x2a6 align=1 (i32.const 0) (local.get 0x2a6))
    (i64.store offset=0x2a7 align=1 (i32.const 0) (local.get 0x2a7))
    (i64.store offset=0x2a8 align=1 (i32.const 0) (local.get 0x2a8))
    (i64.store offset=0x2a9 align=1 (i32.const 0) (local.get 0x2a9))
    (i64.store offset=0x2aa align=1 (i32.const 0) (local.get 0x2aa))
    (i64.store offset=0x2ab align=1 (i32.const 0) (local.get 0x2ab))
    (i64.store offset=0x2ac align=1 (i32.const 0) (local.get 0x2ac))
    (i64.store offset=0x2ad align=1 (i32.const 0) (local.get 0x2ad))
    (i64.store offset=0x2ae align=1 (i32.const 0) (local.get 0x2ae))
    (i64.store offset=0x2af align=1 (i32.const 0) (local.get 0x2af))
    (i64.store offset=0x2b0 align=1 (i32.const 0) (local.get 0x2b0))
    (i64.store offset=0x2b1 align=1 (i32.const 0) (local.get 0x2b1))
    (i64.store offset=0x2b2 align=1 (i32.const 0) (local.get 0x2b2))
    (i64.store offset=0x2b3 align=1 (i32.const 0) (local.get 0x2b3))
    (i64.store offset=0x2b4 align=1 (i32.const 0) (local.get 0x2b4))
    (i64.store offset=0x2b5 align=1 (i32.const 0) (local.get 0x2b5))
    (i64.store offset=0x2b6 align=1 (i32.const 0) (local.get 0x2b6))
    (i64.store offset=0x2b7 align=1 (i32.const 0) (local.get 0x2b7))
    (i64.store offset=0x2b8 align=1 (i32.const 0) (local.get 0x2b8))
    (i64.store offset=0x2b9 align=1 (i32.const 0) (local.get 0x2b9))
    (i64.store offset=0x2ba align=1 (i32.const 0) (local.get 0x2ba))
    (i64.store offset=0x2bb align=1 (i32.const 0) (local.get 0x2bb))
    (i64.store offset=0x2bc align=1 (i32.const 0) (local.get 0x2bc))
    (i64.store offset=0x2bd align=1 (i32.const 0) (local.get 0x2bd))
    (i64.store offset=0x2be align=1 (i32.const 0) (local.get 0x2be))
    (i64.store offset=0x2bf align=1 (i32.const 0) (local.get 0x2bf))
    (i64.store offset=0x2c0 align=1 (i32.const 0) (local.get 0x2c0))
    (i64.store offset=0x2c1 align=1 (i32.const 0) (local.get 0x2c1))
    (i64.store offset=0x2c2 align=1 (i32.const 0) (local.get 0x2c2))
    (i64.store offset=0x2c3 align=1 (i32.const 0) (local.get 0x2c3))
    (i64.store offset=0x2c4 align=1 (i32.const 0) (local.get 0x2c4))
    (i64.store offset=0x2c5 align=1 (i32.const 0) (local.get 0x2c5))
    (i64.store offset=0x2c6 align=1 (i32.const 0) (local.get 0x2c6))
    (i64.store offset=0x2c7 align=1 (i32.const 0) (local.get 0x2c7))
    (i64.store offset=0x2c8 align=1 (i32.const 0) (local.get 0x2c8))
    (i64.store offset=0x2c9 align=1 (i32.const 0) (local.get 0x2c9))
    (i64.store offset=0x2ca align=1 (i32.const 0) (local.get 0x2ca))
    (i64.store offset=0x2cb align=1 (i32.const 0) (local.get 0x2cb))
    (i64.store offset=0x2cc align=1 (i32.const 0) (local.get 0x2cc))
    (i64.store offset=0x2cd align=1 (i32.const 0) (local.get 0x2cd))
    (i64.store offset=0x2ce align=1 (i32.const 0) (local.get 0x2ce))
    (i64.store offset=0x2cf align=1 (i32.const 0) (local.get 0x2cf))
    (i64.store offset=0x2d0 align=1 (i32.const 0) (local.get 0x2d0))
    (i64.store offset=0x2d1 align=1 (i32.const 0) (local.get 0x2d1))
    (i64.store offset=0x2d2 align=1 (i32.const 0) (local.get 0x2d2))
    (i64.store offset=0x2d3 align=1 (i32.const 0) (local.get 0x2d3))
    (i64.store offset=0x2d4 align=1 (i32.const 0) (local.get 0x2d4))
    (i64.store offset=0x2d5 align=1 (i32.const 0) (local.get 0x2d5))
    (i64.store offset=0x2d6 align=1 (i32.const 0) (local.get 0x2d6))
    (i64.store offset=0x2d7 align=1 (i32.const 0) (local.get 0x2d7))
    (i64.store offset=0x2d8 align=1 (i32.const 0) (local.get 0x2d8))
    (i64.store offset=0x2d9 align=1 (i32.const 0) (local.get 0x2d9))
    (i64.store offset=0x2da align=1 (i32.const 0) (local.get 0x2da))
    (i64.store offset=0x2db align=1 (i32.const 0) (local.get 0x2db))
    (i64.store offset=0x2dc align=1 (i32.const 0) (local.get 0x2dc))
    (i64.store offset=0x2dd align=1 (i32.const 0) (local.get 0x2dd))
    (i64.store offset=0x2de align=1 (i32.const 0) (local.get 0x2de))
    (i64.store offset=0x2df align=1 (i32.const 0) (local.get 0x2df))
    (i64.store offset=0x2e0 align=1 (i32.const 0) (local.get 0x2e0))
    (i64.store offset=0x2e1 align=1 (i32.const 0) (local.get 0x2e1))
    (i64.store offset=0x2e2 align=1 (i32.const 0) (local.get 0x2e2))
    (i64.store offset=0x2e3 align=1 (i32.const 0) (local.get 0x2e3))
    (i64.store offset=0x2e4 align=1 (i32.const 0) (local.get 0x2e4))
    (i64.store offset=0x2e5 align=1 (i32.const 0) (local.get 0x2e5))
    (i64.store offset=0x2e6 align=1 (i32.const 0) (local.get 0x2e6))
    (i64.store offset=0x2e7 align=1 (i32.const 0) (local.get 0x2e7))
    (i64.store offset=0x2e8 align=1 (i32.const 0) (local.get 0x2e8))
    (i64.store offset=0x2e9 align=1 (i32.const 0) (local.get 0x2e9))
    (i64.store offset=0x2ea align=1 (i32.const 0) (local.get 0x2ea))
    (i64.store offset=0x2eb align=1 (i32.const 0) (local.get 0x2eb))
    (i64.store offset=0x2ec align=1 (i32.const 0) (local.get 0x2ec))
    (i64.store offset=0x2ed align=1 (i32.const 0) (local.get 0x2ed))
    (i64.store offset=0x2ee align=1 (i32.const 0) (local.get 0x2ee))
    (i64.store offset=0x2ef align=1 (i32.const 0) (local.get 0x2ef))
    (i64.store offset=0x2f0 align=1 (i32.const 0) (local.get 0x2f0))
    (i64.store offset=0x2f1 align=1 (i32.const 0) (local.get 0x2f1))
    (i64.store offset=0x2f2 align=1 (i32.const 0) (local.get 0x2f2))
    (i64.store offset=0x2f3 align=1 (i32.const 0) (local.get 0x2f3))
    (i64.store offset=0x2f4 align=1 (i32.const 0) (local.get 0x2f4))
    (i64.store offset=0x2f5 align=1 (i32.const 0) (local.get 0x2f5))
    (i64.store offset=0x2f6 align=1 (i32.const 0) (local.get 0x2f6))
    (i64.store offset=0x2f7 align=1 (i32.const 0) (local.get 0x2f7))
    (i64.store offset=0x2f8 align=1 (i32.const 0) (local.get 0x2f8))
    (i64.store offset=0x2f9 align=1 (i32.const 0) (local.get 0x2f9))
    (i64.store offset=0x2fa align=1 (i32.const 0) (local.get 0x2fa))
    (i64.store offset=0x2fb align=1 (i32.const 0) (local.get 0x2fb))
    (i64.store offset=0x2fc align=1 (i32.const 0) (local.get 0x2fc))
    (i64.store offset=0x2fd align=1 (i32.const 0) (local.get 0x2fd))
    (i64.store offset=0x2fe align=1 (i32.const 0) (local.get 0x2fe))
    (i64.store offset=0x2ff align=1 (i32.const 0) (local.get 0x2ff))
    (i64.store offset=0x300 align=1 (i32.const 0) (local.get 0x300))
    (i64.store offset=0x301 align=1 (i32.const 0) (local.get 0x301))
    (i64.store offset=0x302 align=1 (i32.const 0) (local.get 0x302))
    (i64.store offset=0x303 align=1 (i32.const 0) (local.get 0x303))
    (i64.store offset=0x304 align=1 (i32.const 0) (local.get 0x304))
    (i64.store offset=0x305 align=1 (i32.const 0) (local.get 0x305))
    (i64.store offset=0x306 align=1 (i32.const 0) (local.get 0x306))
    (i64.store offset=0x307 align=1 (i32.const 0) (local.get 0x307))
    (i64.store offset=0x308 align=1 (i32.const 0) (local.get 0x308))
    (i64.store offset=0x309 align=1 (i32.const 0) (local.get 0x309))
    (i64.store offset=0x30a align=1 (i32.const 0) (local.get 0x30a))
    (i64.store offset=0x30b align=1 (i32.const 0) (local.get 0x30b))
    (i64.store offset=0x30c align=1 (i32.const 0) (local.get 0x30c))
    (i64.store offset=0x30d align=1 (i32.const 0) (local.get 0x30d))
    (i64.store offset=0x30e align=1 (i32.const 0) (local.get 0x30e))
    (i64.store offset=0x30f align=1 (i32.const 0) (local.get 0x30f))
    (i64.store offset=0x310 align=1 (i32.const 0) (local.get 0x310))
    (i64.store offset=0x311 align=1 (i32.const 0) (local.get 0x311))
    (i64.store offset=0x312 align=1 (i32.const 0) (local.get 0x312))
    (i64.store offset=0x313 align=1 (i32.const 0) (local.get 0x313))
    (i64.store offset=0x314 align=1 (i32.const 0) (local.get 0x314))
    (i64.store offset=0x315 align=1 (i32.const 0) (local.get 0x315))
    (i64.store offset=0x316 align=1 (i32.const 0) (local.get 0x316))
    (i64.store offset=0x317 align=1 (i32.const 0) (local.get 0x317))
    (i64.store offset=0x318 align=1 (i32.const 0) (local.get 0x318))
    (i64.store offset=0x319 align=1 (i32.const 0) (local.get 0x319))
    (i64.store offset=0x31a align=1 (i32.const 0) (local.get 0x31a))
    (i64.store offset=0x31b align=1 (i32.const 0) (local.get 0x31b))
    (i64.store offset=0x31c align=1 (i32.const 0) (local.get 0x31c))
    (i64.store offset=0x31d align=1 (i32.const 0) (local.get 0x31d))
    (i64.store offset=0x31e align=1 (i32.const 0) (local.get 0x31e))
    (i64.store offset=0x31f align=1 (i32.const 0) (local.get 0x31f))
    (i64.store offset=0x320 align=1 (i32.const 0) (local.get 0x320))
    (i64.store offset=0x321 align=1 (i32.const 0) (local.get 0x321))
    (i64.store offset=0x322 align=1 (i32.const 0) (local.get 0x322))
    (i64.store offset=0x323 align=1 (i32.const 0) (local.get 0x323))
    (i64.store offset=0x324 align=1 (i32.const 0) (local.get 0x324))
    (i64.store offset=0x325 align=1 (i32.const 0) (local.get 0x325))
    (i64.store offset=0x326 align=1 (i32.const 0) (local.get 0x326))
    (i64.store offset=0x327 align=1 (i32.const 0) (local.get 0x327))
    (i64.store offset=0x328 align=1 (i32.const 0) (local.get 0x328))
    (i64.store offset=0x329 align=1 (i32.const 0) (local.get 0x329))
    (i64.store offset=0x32a align=1 (i32.const 0) (local.get 0x32a))
    (i64.store offset=0x32b align=1 (i32.const 0) (local.get 0x32b))
    (i64.store offset=0x32c align=1 (i32.const 0) (local.get 0x32c))
    (i64.store offset=0x32d align=1 (i32.const 0) (local.get 0x32d))
    (i64.store offset=0x32e align=1 (i32.const 0) (local.get 0x32e))
    (i64.store offset=0x32f align=1 (i32.const 0) (local.get 0x32f))
    (i64.store offset=0x330 align=1 (i32.const 0) (local.get 0x330))
    (i64.store offset=0x331 align=1 (i32.const 0) (local.get 0x331))
    (i64.store offset=0x332 align=1 (i32.const 0) (local.get 0x332))
    (i64.store offset=0x333 align=1 (i32.const 0) (local.get 0x333))
    (i64.store offset=0x334 align=1 (i32.const 0) (local.get 0x334))
    (i64.store offset=0x335 align=1 (i32.const 0) (local.get 0x335))
    (i64.store offset=0x336 align=1 (i32.const 0) (local.get 0x336))
    (i64.store offset=0x337 align=1 (i32.const 0) (local.get 0x337))
    (i64.store offset=0x338 align=1 (i32.const 0) (local.get 0x338))
    (i64.store offset=0x339 align=1 (i32.const 0) (local.get 0x339))
    (i64.store offset=0x33a align=1 (i32.const 0) (local.get 0x33a))
    (i64.store offset=0x33b align=1 (i32.const 0) (local.get 0x33b))
    (i64.store offset=0x33c align=1 (i32.const 0) (local.get 0x33c))
    (i64.store offset=0x33d align=1 (i32.const 0) (local.get 0x33d))
    (i64.store offset=0x33e align=1 (i32.const 0) (local.get 0x33e))
    (i64.store offset=0x33f align=1 (i32.const 0) (local.get 0x33f))
    (i64.store offset=0x340 align=1 (i32.const 0) (local.get 0x340))
    (i64.store offset=0x341 align=1 (i32.const 0) (local.get 0x341))
    (i64.store offset=0x342 align=1 (i32.const 0) (local.get 0x342))
    (i64.store offset=0x343 align=1 (i32.const 0) (local.get 0x343))
    (i64.store offset=0x344 align=1 (i32.const 0) (local.get 0x344))
    (i64.store offset=0x345 align=1 (i32.const 0) (local.get 0x345))
    (i64.store offset=0x346 align=1 (i32.const 0) (local.get 0x346))
    (i64.store offset=0x347 align=1 (i32.const 0) (local.get 0x347))
    (i64.store offset=0x348 align=1 (i32.const 0) (local.get 0x348))
    (i64.store offset=0x349 align=1 (i32.const 0) (local.get 0x349))
    (i64.store offset=0x34a align=1 (i32.const 0) (local.get 0x34a))
    (i64.store offset=0x34b align=1 (i32.const 0) (local.get 0x34b))
    (i64.store offset=0x34c align=1 (i32.const 0) (local.get 0x34c))
    (i64.store offset=0x34d align=1 (i32.const 0) (local.get 0x34d))
    (i64.store offset=0x34e align=1 (i32.const 0) (local.get 0x34e))
    (i64.store offset=0x34f align=1 (i32.const 0) (local.get 0x34f))
    (i64.store offset=0x350 align=1 (i32.const 0) (local.get 0x350))
    (i64.store offset=0x351 align=1 (i32.const 0) (local.get 0x351))
    (i64.store offset=0x352 align=1 (i32.const 0) (local.get 0x352))
    (i64.store offset=0x353 align=1 (i32.const 0) (local.get 0x353))
    (i64.store offset=0x354 align=1 (i32.const 0) (local.get 0x354))
    (i64.store offset=0x355 align=1 (i32.const 0) (local.get 0x355))
    (i64.store offset=0x356 align=1 (i32.const 0) (local.get 0x356))
    (i64.store offset=0x357 align=1 (i32.const 0) (local.get 0x357))
    (i64.store offset=0x358 align=1 (i32.const 0) (local.get 0x358))
    (i64.store offset=0x359 align=1 (i32.const 0) (local.get 0x359))
    (i64.store offset=0x35a align=1 (i32.const 0) (local.get 0x35a))
    (i64.store offset=0x35b align=1 (i32.const 0) (local.get 0x35b))
    (i64.store offset=0x35c align=1 (i32.const 0) (local.get 0x35c))
    (i64.store offset=0x35d align=1 (i32.const 0) (local.get 0x35d))
    (i64.store offset=0x35e align=1 (i32.const 0) (local.get 0x35e))
    (i64.store offset=0x35f align=1 (i32.const 0) (local.get 0x35f))
    (i64.store offset=0x360 align=1 (i32.const 0) (local.get 0x360))
    (i64.store offset=0x361 align=1 (i32.const 0) (local.get 0x361))
    (i64.store offset=0x362 align=1 (i32.const 0) (local.get 0x362))
    (i64.store offset=0x363 align=1 (i32.const 0) (local.get 0x363))
    (i64.store offset=0x364 align=1 (i32.const 0) (local.get 0x364))
    (i64.store offset=0x365 align=1 (i32.const 0) (local.get 0x365))
    (i64.store offset=0x366 align=1 (i32.const 0) (local.get 0x366))
    (i64.store offset=0x367 align=1 (i32.const 0) (local.get 0x367))
    (i64.store offset=0x368 align=1 (i32.const 0) (local.get 0x368))
    (i64.store offset=0x369 align=1 (i32.const 0) (local.get 0x369))
    (i64.store offset=0x36a align=1 (i32.const 0) (local.get 0x36a))
    (i64.store offset=0x36b align=1 (i32.const 0) (local.get 0x36b))
    (i64.store offset=0x36c align=1 (i32.const 0) (local.get 0x36c))
    (i64.store offset=0x36d align=1 (i32.const 0) (local.get 0x36d))
    (i64.store offset=0x36e align=1 (i32.const 0) (local.get 0x36e))
    (i64.store offset=0x36f align=1 (i32.const 0) (local.get 0x36f))
    (i64.store offset=0x370 align=1 (i32.const 0) (local.get 0x370))
    (i64.store offset=0x371 align=1 (i32.const 0) (local.get 0x371))
    (i64.store offset=0x372 align=1 (i32.const 0) (local.get 0x372))
    (i64.store offset=0x373 align=1 (i32.const 0) (local.get 0x373))
    (i64.store offset=0x374 align=1 (i32.const 0) (local.get 0x374))
    (i64.store offset=0x375 align=1 (i32.const 0) (local.get 0x375))
    (i64.store offset=0x376 align=1 (i32.const 0) (local.get 0x376))
    (i64.store offset=0x377 align=1 (i32.const 0) (local.get 0x377))
    (i64.store offset=0x378 align=1 (i32.const 0) (local.get 0x378))
    (i64.store offset=0x379 align=1 (i32.const 0) (local.get 0x379))
    (i64.store offset=0x37a align=1 (i32.const 0) (local.get 0x37a))
    (i64.store offset=0x37b align=1 (i32.const 0) (local.get 0x37b))
    (i64.store offset=0x37c align=1 (i32.const 0) (local.get 0x37c))
    (i64.store offset=0x37d align=1 (i32.const 0) (local.get 0x37d))
    (i64.store offset=0x37e align=1 (i32.const 0) (local.get 0x37e))
    (i64.store offset=0x37f align=1 (i32.const 0) (local.get 0x37f))
    (i64.store offset=0x380 align=1 (i32.const 0) (local.get 0x380))
    (i64.store offset=0x381 align=1 (i32.const 0) (local.get 0x381))
    (i64.store offset=0x382 align=1 (i32.const 0) (local.get 0x382))
    (i64.store offset=0x383 align=1 (i32.const 0) (local.get 0x383))
    (i64.store offset=0x384 align=1 (i32.const 0) (local.get 0x384))
    (i64.store offset=0x385 align=1 (i32.const 0) (local.get 0x385))
    (i64.store offset=0x386 align=1 (i32.const 0) (local.get 0x386))
    (i64.store offset=0x387 align=1 (i32.const 0) (local.get 0x387))
    (i64.store offset=0x388 align=1 (i32.const 0) (local.get 0x388))
    (i64.store offset=0x389 align=1 (i32.const 0) (local.get 0x389))
    (i64.store offset=0x38a align=1 (i32.const 0) (local.get 0x38a))
    (i64.store offset=0x38b align=1 (i32.const 0) (local.get 0x38b))
    (i64.store offset=0x38c align=1 (i32.const 0) (local.get 0x38c))
    (i64.store offset=0x38d align=1 (i32.const 0) (local.get 0x38d))
    (i64.store offset=0x38e align=1 (i32.const 0) (local.get 0x38e))
    (i64.store offset=0x38f align=1 (i32.const 0) (local.get 0x38f))
    (i64.store offset=0x390 align=1 (i32.const 0) (local.get 0x390))
    (i64.store offset=0x391 align=1 (i32.const 0) (local.get 0x391))
    (i64.store offset=0x392 align=1 (i32.const 0) (local.get 0x392))
    (i64.store offset=0x393 align=1 (i32.const 0) (local.get 0x393))
    (i64.store offset=0x394 align=1 (i32.const 0) (local.get 0x394))
    (i64.store offset=0x395 align=1 (i32.const 0) (local.get 0x395))
    (i64.store offset=0x396 align=1 (i32.const 0) (local.get 0x396))
    (i64.store offset=0x397 align=1 (i32.const 0) (local.get 0x397))
    (i64.store offset=0x398 align=1 (i32.const 0) (local.get 0x398))
    (i64.store offset=0x399 align=1 (i32.const 0) (local.get 0x399))
    (i64.store offset=0x39a align=1 (i32.const 0) (local.get 0x39a))
    (i64.store offset=0x39b align=1 (i32.const 0) (local.get 0x39b))
    (i64.store offset=0x39c align=1 (i32.const 0) (local.get 0x39c))
    (i64.store offset=0x39d align=1 (i32.const 0) (local.get 0x39d))
    (i64.store offset=0x39e align=1 (i32.const 0) (local.get 0x39e))
    (i64.store offset=0x39f align=1 (i32.const 0) (local.get 0x39f))
    (i64.store offset=0x3a0 align=1 (i32.const 0) (local.get 0x3a0))
    (i64.store offset=0x3a1 align=1 (i32.const 0) (local.get 0x3a1))
    (i64.store offset=0x3a2 align=1 (i32.const 0) (local.get 0x3a2))
    (i64.store offset=0x3a3 align=1 (i32.const 0) (local.get 0x3a3))
    (i64.store offset=0x3a4 align=1 (i32.const 0) (local.get 0x3a4))
    (i64.store offset=0x3a5 align=1 (i32.const 0) (local.get 0x3a5))
    (i64.store offset=0x3a6 align=1 (i32.const 0) (local.get 0x3a6))
    (i64.store offset=0x3a7 align=1 (i32.const 0) (local.get 0x3a7))
    (i64.store offset=0x3a8 align=1 (i32.const 0) (local.get 0x3a8))
    (i64.store offset=0x3a9 align=1 (i32.const 0) (local.get 0x3a9))
    (i64.store offset=0x3aa align=1 (i32.const 0) (local.get 0x3aa))
    (i64.store offset=0x3ab align=1 (i32.const 0) (local.get 0x3ab))
    (i64.store offset=0x3ac align=1 (i32.const 0) (local.get 0x3ac))
    (i64.store offset=0x3ad align=1 (i32.const 0) (local.get 0x3ad))
    (i64.store offset=0x3ae align=1 (i32.const 0) (local.get 0x3ae))
    (i64.store offset=0x3af align=1 (i32.const 0) (local.get 0x3af))
    (i64.store offset=0x3b0 align=1 (i32.const 0) (local.get 0x3b0))
    (i64.store offset=0x3b1 align=1 (i32.const 0) (local.get 0x3b1))
    (i64.store offset=0x3b2 align=1 (i32.const 0) (local.get 0x3b2))
    (i64.store offset=0x3b3 align=1 (i32.const 0) (local.get 0x3b3))
    (i64.store offset=0x3b4 align=1 (i32.const 0) (local.get 0x3b4))
    (i64.store offset=0x3b5 align=1 (i32.const 0) (local.get 0x3b5))
    (i64.store offset=0x3b6 align=1 (i32.const 0) (local.get 0x3b6))
    (i64.store offset=0x3b7 align=1 (i32.const 0) (local.get 0x3b7))
    (i64.store offset=0x3b8 align=1 (i32.const 0) (local.get 0x3b8))
    (i64.store offset=0x3b9 align=1 (i32.const 0) (local.get 0x3b9))
    (i64.store offset=0x3ba align=1 (i32.const 0) (local.get 0x3ba))
    (i64.store offset=0x3bb align=1 (i32.const 0) (local.get 0x3bb))
    (i64.store offset=0x3bc align=1 (i32.const 0) (local.get 0x3bc))
    (i64.store offset=0x3bd align=1 (i32.const 0) (local.get 0x3bd))
    (i64.store offset=0x3be align=1 (i32.const 0) (local.get 0x3be))
    (i64.store offset=0x3bf align=1 (i32.const 0) (local.get 0x3bf))
    (i64.store offset=0x3c0 align=1 (i32.const 0) (local.get 0x3c0))
    (i64.store offset=0x3c1 align=1 (i32.const 0) (local.get 0x3c1))
    (i64.store offset=0x3c2 align=1 (i32.const 0) (local.get 0x3c2))
    (i64.store offset=0x3c3 align=1 (i32.const 0) (local.get 0x3c3))
    (i64.store offset=0x3c4 align=1 (i32.const 0) (local.get 0x3c4))
    (i64.store offset=0x3c5 align=1 (i32.const 0) (local.get 0x3c5))
    (i64.store offset=0x3c6 align=1 (i32.const 0) (local.get 0x3c6))
    (i64.store offset=0x3c7 align=1 (i32.const 0) (local.get 0x3c7))
    (i64.store offset=0x3c8 align=1 (i32.const 0) (local.get 0x3c8))
    (i64.store offset=0x3c9 align=1 (i32.const 0) (local.get 0x3c9))
    (i64.store offset=0x3ca align=1 (i32.const 0) (local.get 0x3ca))
    (i64.store offset=0x3cb align=1 (i32.const 0) (local.get 0x3cb))
    (i64.store offset=0x3cc align=1 (i32.const 0) (local.get 0x3cc))
    (i64.store offset=0x3cd align=1 (i32.const 0) (local.get 0x3cd))
    (i64.store offset=0x3ce align=1 (i32.const 0) (local.get 0x3ce))
    (i64.store offset=0x3cf align=1 (i32.const 0) (local.get 0x3cf))
    (i64.store offset=0x3d0 align=1 (i32.const 0) (local.get 0x3d0))
    (i64.store offset=0x3d1 align=1 (i32.const 0) (local.get 0x3d1))
    (i64.store offset=0x3d2 align=1 (i32.const 0) (local.get 0x3d2))
    (i64.store offset=0x3d3 align=1 (i32.const 0) (local.get 0x3d3))
    (i64.store offset=0x3d4 align=1 (i32.const 0) (local.get 0x3d4))
    (i64.store offset=0x3d5 align=1 (i32.const 0) (local.get 0x3d5))
    (i64.store offset=0x3d6 align=1 (i32.const 0) (local.get 0x3d6))
    (i64.store offset=0x3d7 align=1 (i32.const 0) (local.get 0x3d7))
    (i64.store offset=0x3d8 align=1 (i32.const 0) (local.get 0x3d8))
    (i64.store offset=0x3d9 align=1 (i32.const 0) (local.get 0x3d9))
    (i64.store offset=0x3da align=1 (i32.const 0) (local.get 0x3da))
    (i64.store offset=0x3db align=1 (i32.const 0) (local.get 0x3db))
    (i64.store offset=0x3dc align=1 (i32.const 0) (local.get 0x3dc))
    (i64.store offset=0x3dd align=1 (i32.const 0) (local.get 0x3dd))
    (i64.store offset=0x3de align=1 (i32.const 0) (local.get 0x3de))
    (i64.store offset=0x3df align=1 (i32.const 0) (local.get 0x3df))
    (i64.store offset=0x3e0 align=1 (i32.const 0) (local.get 0x3e0))
    (i64.store offset=0x3e1 align=1 (i32.const 0) (local.get 0x3e1))
    (i64.store offset=0x3e2 align=1 (i32.const 0) (local.get 0x3e2))
    (i64.store offset=0x3e3 align=1 (i32.const 0) (local.get 0x3e3))
    (i64.store offset=0x3e4 align=1 (i32.const 0) (local.get 0x3e4))
    (i64.store offset=0x3e5 align=1 (i32.const 0) (local.get 0x3e5))
    (i64.store offset=0x3e6 align=1 (i32.const 0) (local.get 0x3e6))
    (i64.store offset=0x3e7 align=1 (i32.const 0) (local.get 0x3e7))
    (i64.store offset=0x3e8 align=1 (i32.const 0) (local.get 0x3e8))
    (i64.store offset=0x3e9 align=1 (i32.const 0) (local.get 0x3e9))
    (i64.store offset=0x3ea align=1 (i32.const 0) (local.get 0x3ea))
    (i64.store offset=0x3eb align=1 (i32.const 0) (local.get 0x3eb))
    (i64.store offset=0x3ec align=1 (i32.const 0) (local.get 0x3ec))
    (i64.store offset=0x3ed align=1 (i32.const 0) (local.get 0x3ed))
    (i64.store offset=0x3ee align=1 (i32.const 0) (local.get 0x3ee))
    (i64.store offset=0x3ef align=1 (i32.const 0) (local.get 0x3ef))
    (i64.store offset=0x3f0 align=1 (i32.const 0) (local.get 0x3f0))
    (i64.store offset=0x3f1 align=1 (i32.const 0) (local.get 0x3f1))
    (i64.store offset=0x3f2 align=1 (i32.const 0) (local.get 0x3f2))
    (i64.store offset=0x3f3 align=1 (i32.const 0) (local.get 0x3f3))
    (i64.store offset=0x3f4 align=1 (i32.const 0) (local.get 0x3f4))
    (i64.store offset=0x3f5 align=1 (i32.const 0) (local.get 0x3f5))
    (i64.store offset=0x3f6 align=1 (i32.const 0) (local.get 0x3f6))
    (i64.store offset=0x3f7 align=1 (i32.const 0) (local.get 0x3f7))
    (i64.store offset=0x3f8 align=1 (i32.const 0) (local.get 0x3f8))
    (i64.store offset=0x3f9 align=1 (i32.const 0) (local.get 0x3f9))
    (i64.store offset=0x3fa align=1 (i32.const 0) (local.get 0x3fa))
    (i64.store offset=0x3fb align=1 (i32.const 0) (local.get 0x3fb))
    (i64.store offset=0x3fc align=1 (i32.const 0) (local.get 0x3fc))
    (i64.store offset=0x3fd align=1 (i32.const 0) (local.get 0x3fd))
    (i64.store offset=0x3fe align=1 (i32.const 0) (local.get 0x3fe))
    (i64.store offset=0x3ff align=1 (i32.const 0) (local.get 0x3ff))
    (i64.store offset=0x400 align=1 (i32.const 0) (local.get 0x400))
    (i64.store offset=0x401 align=1 (i32.const 0) (local.get 0x401))
    (i64.store offset=0x402 align=1 (i32.const 0) (local.get 0x402))
    (i64.store offset=0x403 align=1 (i32.const 0) (local.get 0x403))
    (i64.store offset=0x404 align=1 (i32.const 0) (local.get 0x404))
    (i64.store offset=0x405 align=1 (i32.const 0) (local.get 0x405))
    (i64.store offset=0x406 align=1 (i32.const 0) (local.get 0x406))
    (i64.store offset=0x407 align=1 (i32.const 0) (local.get 0x407))
    (i64.store offset=0x408 align=1 (i32.const 0) (local.get 0x408))
    (i64.store offset=0x409 align=1 (i32.const 0) (local.get 0x409))
    (i64.store offset=0x40a align=1 (i32.const 0) (local.get 0x40a))
    (i64.store offset=0x40b align=1 (i32.const 0) (local.get 0x40b))
    (i64.store offset=0x40c align=1 (i32.const 0) (local.get 0x40c))
    (i64.store offset=0x40d align=1 (i32.const 0) (local.get 0x40d))
    (i64.store offset=0x40e align=1 (i32.const 0) (local.get 0x40e))
    (i64.store offset=0x40f align=1 (i32.const 0) (local.get 0x40f))
    (i64.store offset=0x410 align=1 (i32.const 0) (local.get 0x410))
    (i64.store offset=0x411 align=1 (i32.const 0) (local.get 0x411))
    (i64.store offset=0x412 align=1 (i32.const 0) (local.get 0x412))
    (i64.store offset=0x413 align=1 (i32.const 0) (local.get 0x413))
    (i64.store offset=0x414 align=1 (i32.const 0) (local.get 0x414))
    (i64.store offset=0x415 align=1 (i32.const 0) (local.get 0x415))
    (i64.store offset=0x416 align=1 (i32.const 0) (local.get 0x416))
    (i64.store offset=0x417 align=1 (i32.const 0) (local.get 0x417))
    (i64.store offset=0x418 align=1 (i32.const 0) (local.get 0x418))
    (i64.store offset=0x419 align=1 (i32.const 0) (local.get 0x419))
    (i64.store offset=0x41a align=1 (i32.const 0) (local.get 0x41a))
    (i64.store offset=0x41b align=1 (i32.const 0) (local.get 0x41b))
    (i64.store offset=0x41c align=1 (i32.const 0) (local.get 0x41c))
    (i64.store offset=0x41d align=1 (i32.const 0) (local.get 0x41d))
    (i64.store offset=0x41e align=1 (i32.const 0) (local.get 0x41e))
    (i64.store offset=0x41f align=1 (i32.const 0) (local.get 0x41f))
  )
)
"#
    );
}

#[test]
fn store() {
    assert_snapshot!(
        r#"
(module
  (memory 1)

  (func (export "as-block-value")
    (block (i32.store (i32.const 0) (i32.const 1)))
  )
  (func (export "as-loop-value")
    (loop (i32.store (i32.const 0) (i32.const 1)))
  )

  (func (export "as-br-value")
    (block (br 0 (i32.store (i32.const 0) (i32.const 1))))
  )
  (func (export "as-br_if-value")
    (block
      (br_if 0 (i32.store (i32.const 0) (i32.const 1)) (i32.const 1))
    )
  )
  (func (export "as-br_if-value-cond")
    (block
      (br_if 0 (i32.const 6) (i32.store (i32.const 0) (i32.const 1)))
    )
  )
  (func (export "as-br_table-value")
    (block
      (br_table 0 (i32.store (i32.const 0) (i32.const 1)) (i32.const 1))
    )
  )

  (func (export "as-return-value")
    (return (i32.store (i32.const 0) (i32.const 1)))
  )

  (func (export "as-if-then")
    (if (i32.const 1) (then (i32.store (i32.const 0) (i32.const 1))))
  )
  (func (export "as-if-else")
    (if (i32.const 0) (then) (else (i32.store (i32.const 0) (i32.const 1))))
  )
)
"#
    );
}

#[test]
fn r#switch() {
    assert_snapshot!(
        r#"
(module
  ;; Statement switch
  (func (export "stmt") (param $i i32) (result i32)
    (local $j i32)
    (local.set $j (i32.const 100))
    (block $switch
      (block $7
        (block $default
          (block $6
            (block $5
              (block $4
                (block $3
                  (block $2
                    (block $1
                      (block $0
                        (br_table $0 $1 $2 $3 $4 $5 $6 $7 $default
                          (local.get $i)
                        )
                      ) ;; 0
                      (return (local.get $i))
                    ) ;; 1
                    (nop)
                    ;; fallthrough
                  ) ;; 2
                  ;; fallthrough
                ) ;; 3
                (local.set $j (i32.sub (i32.const 0) (local.get $i)))
                (br $switch)
              ) ;; 4
              (br $switch)
            ) ;; 5
            (local.set $j (i32.const 101))
            (br $switch)
          ) ;; 6
          (local.set $j (i32.const 101))
          ;; fallthrough
        ) ;; default
        (local.set $j (i32.const 102))
      ) ;; 7
      ;; fallthrough
    )
    (return (local.get $j))
  )

  ;; Expression switch
  (func (export "expr") (param $i i64) (result i64)
    (local $j i64)
    (local.set $j (i64.const 100))
    (return
      (block $switch (result i64)
        (block $7
          (block $default
            (block $4
              (block $5
                (block $6
                  (block $3
                    (block $2
                      (block $1
                        (block $0
                          (br_table $0 $1 $2 $3 $4 $5 $6 $7 $default
                            (i32.wrap_i64 (local.get $i))
                          )
                        ) ;; 0
                        (return (local.get $i))
                      ) ;; 1
                      (nop)
                      ;; fallthrough
                    ) ;; 2
                    ;; fallthrough
                  ) ;; 3
                  (br $switch (i64.sub (i64.const 0) (local.get $i)))
                ) ;; 6
                (local.set $j (i64.const 101))
                ;; fallthrough
              ) ;; 4
              ;; fallthrough
            ) ;; 5
            ;; fallthrough
          ) ;; default
          (br $switch (local.get $j))
        ) ;; 7
        (i64.const -5)
      )
    )
  )

  ;; Argument switch
  (func (export "arg") (param $i i32) (result i32)
    (return
      (block $2 (result i32)
        (i32.add (i32.const 10)
          (block $1 (result i32)
            (i32.add (i32.const 100)
              (block $0 (result i32)
                (i32.add (i32.const 1000)
                  (block $default (result i32)
                    (br_table $0 $1 $2 $default
                      (i32.mul (i32.const 2) (local.get $i))
                      (i32.and (i32.const 3) (local.get $i))
                    )
                  )
                )
              )
            )
          )
        )
      )
    )
  )

  ;; Corner cases
  (func (export "corner") (result i32)
    (block
      (br_table 0 (i32.const 0))
    )
    (i32.const 1)
  )
)
"#
    );
}

#[test]
fn table_copy() {
    assert_snapshot!(
        r#"
(module
  (type (func (result i32)))  ;; type #0
  (import "a" "ef0" (func (result i32)))    ;; index 0
  (import "a" "ef1" (func (result i32)))
  (import "a" "ef2" (func (result i32)))
  (import "a" "ef3" (func (result i32)))
  (import "a" "ef4" (func (result i32)))    ;; index 4
  (table $t0 30 30 funcref)
  (table $t1 30 30 funcref)
  (elem (table $t0) (i32.const 2) func 3 1 4 1)
  (elem funcref
    (ref.func 2) (ref.func 7) (ref.func 1) (ref.func 8))
  (elem (table $t0) (i32.const 12) func 7 5 2 3 6)
  (elem funcref
    (ref.func 5) (ref.func 9) (ref.func 2) (ref.func 7) (ref.func 6))
  (elem (table $t1) (i32.const 3) func 1 3 1 4)
  (elem (table $t1) (i32.const 11) func 6 3 2 5 7)
  (func (result i32) (i32.const 5))  ;; index 5
  (func (result i32) (i32.const 6))
  (func (result i32) (i32.const 7))
  (func (result i32) (i32.const 8))
  (func (result i32) (i32.const 9))  ;; index 9
  (func (export "test")
    (nop))
  (func (export "check_t0") (param i32) (result i32)
    (call_indirect $t0 (type 0) (local.get 0)))
  (func (export "check_t1") (param i32) (result i32)
    (call_indirect $t1 (type 0) (local.get 0)))
)
"#
    );
}

#[test]
fn table_fill() {
    assert_snapshot!(
        r#"
(module
  (table $t 10 externref)

  (func (export "fill") (param $i i32) (param $r externref) (param $n i32)
    (table.fill $t (local.get $i) (local.get $r) (local.get $n))
  )

  (func (export "fill-abbrev") (param $i i32) (param $r externref) (param $n i32)
    (table.fill (local.get $i) (local.get $r) (local.get $n))
  )

  (func (export "get") (param $i i32) (result externref)
    (table.get $t (local.get $i))
  )
)
"#
    );
}

#[test]
fn table_get() {
    assert_snapshot!(
        r#"
(module
  (table $t2 2 externref)
  (table $t3 3 funcref)
  (elem (table $t3) (i32.const 1) func $dummy)
  (func $dummy)

  (func (export "init") (param $r externref)
    (table.set $t2 (i32.const 1) (local.get $r))
    (table.set $t3 (i32.const 2) (table.get $t3 (i32.const 1)))
  )

  (func (export "get-externref") (param $i i32) (result externref)
    (table.get (local.get $i))
  )
  (func $f3 (export "get-funcref") (param $i i32) (result funcref)
    (table.get $t3 (local.get $i))
  )

  (func (export "is_null-funcref") (param $i i32) (result i32)
    (ref.is_null (call $f3 (local.get $i)))
  )
)
"#
    );
}

#[test]
fn table_grow() {
    assert_snapshot!(
        r#"
(module
  (table $t 0 externref)

  (func (export "get") (param $i i32) (result externref) (table.get $t (local.get $i)))
  (func (export "set") (param $i i32) (param $r externref) (table.set $t (local.get $i) (local.get $r)))

  (func (export "grow") (param $sz i32) (param $init externref) (result i32)
    (table.grow $t (local.get $init) (local.get $sz))
  )
  (func (export "grow-abbrev") (param $sz i32) (param $init externref) (result i32)
    (table.grow (local.get $init) (local.get $sz))
  )
  (func (export "size") (result i32) (table.size $t))
)
"#
    );
}

#[test]
fn table_init() {
    assert_snapshot!(
        r#"
(module
  (type (func (result i32)))  ;; type #0
  (import "a" "ef0" (func (result i32)))    ;; index 0
  (import "a" "ef1" (func (result i32)))
  (import "a" "ef2" (func (result i32)))
  (import "a" "ef3" (func (result i32)))
  (import "a" "ef4" (func (result i32)))    ;; index 4
  (table $t0 30 30 funcref)
  (table $t1 30 30 funcref)
  (elem (table $t0) (i32.const 2) func 3 1 4 1)
  (elem funcref
    (ref.func 2) (ref.func 7) (ref.func 1) (ref.func 8))
  (elem (table $t0) (i32.const 12) func 7 5 2 3 6)
  (elem funcref
    (ref.func 5) (ref.func 9) (ref.func 2) (ref.func 7) (ref.func 6))
  (func (result i32) (i32.const 5))  ;; index 5
  (func (result i32) (i32.const 6))
  (func (result i32) (i32.const 7))
  (func (result i32) (i32.const 8))
  (func (result i32) (i32.const 9))  ;; index 9
  (func (export "test")
    (table.init $t0 1 (i32.const 7) (i32.const 0) (i32.const 4)))
)
"#
    );
}

#[test]
fn table_set() {
    assert_snapshot!(
        r#"
(module
  (table $t2 1 externref)
  (table $t3 2 funcref)
  (elem (table $t3) (i32.const 1) func $dummy)
  (func $dummy)

  (func (export "get-externref") (param $i i32) (result externref)
    (table.get $t2 (local.get $i))
  )
  (func $fx (export "get-funcref") (param $i i32) (result funcref)
    (table.get $t3 (local.get $i))
  )

  (func (export "set-externref") (param $i i32) (param $r externref)
    (table.set (local.get $i) (local.get $r))
  )
  (func (export "set-funcref") (param $i i32) (param $r funcref)
    (table.set $t3 (local.get $i) (local.get $r))
  )
  (func (export "set-funcref-from") (param $i i32) (param $j i32)
    (table.set $t3 (local.get $i) (table.get $t3 (local.get $j)))
  )

  (func (export "is_null-funcref") (param $i i32) (result i32)
    (ref.is_null (call $fx (local.get $i)))
  )
)
"#
    );
}

#[test]
fn table_size() {
    assert_snapshot!(
        r#"
(module
  (table $t0 0 externref)
  (table $t1 1 externref)
  (table $t2 0 2 externref)
  (table $t3 3 8 externref)

  (func (export "size-t0") (result i32) table.size)
  (func (export "size-t1") (result i32) (table.size $t1))
  (func (export "size-t2") (result i32) (table.size $t2))
  (func (export "size-t3") (result i32) (table.size $t3))

  (func (export "grow-t0") (param $sz i32)
    (drop (table.grow $t0 (ref.null extern) (local.get $sz)))
  )
  (func (export "grow-t1") (param $sz i32)
    (drop (table.grow $t1 (ref.null extern) (local.get $sz)))
  )
  (func (export "grow-t2") (param $sz i32)
    (drop (table.grow $t2 (ref.null extern) (local.get $sz)))
  )
  (func (export "grow-t3") (param $sz i32)
    (drop (table.grow $t3 (ref.null extern) (local.get $sz)))
  )
)
"#
    );
}

#[test]
fn r#type() {
    assert_snapshot!(
        r#"
(module
  (type (func))
  (type $t (func))

  (type (func (param i32)))
  (type (func (param $x i32)))
  (type (func (result i32)))
  (type (func (param i32) (result i32)))
  (type (func (param $x i32) (result i32)))

  (type (func (param f32 f64)))
  (type (func (result i64 f32)))
  (type (func (param i32 i64) (result f32 f64)))

  (type (func (param f32) (param f64)))
  (type (func (param $x f32) (param f64)))
  (type (func (param f32) (param $y f64)))
  (type (func (param $x f32) (param $y f64)))
  (type (func (result i64) (result f32)))
  (type (func (param i32) (param i64) (result f32) (result f64)))
  (type (func (param $x i32) (param $y i64) (result f32) (result f64)))

  (type (func (param f32 f64) (param $x i32) (param f64 i32 i32)))
  (type (func (result i64 i64 f32) (result f32 i32)))
  (type
    (func (param i32 i32) (param i64 i32) (result f32 f64) (result f64 i32))
  )

  (type (func (param) (param $x f32) (param) (param) (param f64 i32) (param)))
  (type
    (func (result) (result) (result i64 i64) (result) (result f32) (result))
  )
  (type
    (func
      (param i32 i32) (param i64 i32) (param) (param $x i32) (param)
      (result) (result f32 f64) (result f64 i32) (result)
    )
  )
)
"#
    );
}

#[test]
fn unreachable() {
    assert_snapshot!(
        r#"
(module
  ;; Auxiliary definitions
  (func $dummy)
  (func $dummy3 (param i32 i32 i32))

  (func (export "type-i32") (result i32) (unreachable))
  (func (export "type-i64") (result i64) (unreachable))
  (func (export "type-f32") (result f32) (unreachable))
  (func (export "type-f64") (result f64) (unreachable))

  (func (export "as-func-first") (result i32)
    (unreachable) (i32.const -1)
  )
  (func (export "as-func-mid") (result i32)
    (call $dummy) (unreachable) (i32.const -1)
  )
  (func (export "as-func-last")
    (call $dummy) (unreachable)
  )
  (func (export "as-func-value") (result i32)
    (call $dummy) (unreachable)
  )

  (func (export "as-block-first") (result i32)
    (block (result i32) (unreachable) (i32.const 2))
  )
  (func (export "as-block-mid") (result i32)
    (block (result i32) (call $dummy) (unreachable) (i32.const 2))
  )
  (func (export "as-block-last")
    (block (nop) (call $dummy) (unreachable))
  )
  (func (export "as-block-value") (result i32)
    (block (result i32) (nop) (call $dummy) (unreachable))
  )
  (func (export "as-block-broke") (result i32)
    (block (result i32) (call $dummy) (br 0 (i32.const 1)) (unreachable))
  )

  (func (export "as-loop-first") (result i32)
    (loop (result i32) (unreachable) (i32.const 2))
  )
  (func (export "as-loop-mid") (result i32)
    (loop (result i32) (call $dummy) (unreachable) (i32.const 2))
  )
  (func (export "as-loop-last")
    (loop (nop) (call $dummy) (unreachable))
  )
  (func (export "as-loop-broke") (result i32)
    (block (result i32)
      (loop (result i32) (call $dummy) (br 1 (i32.const 1)) (unreachable))
    )
  )

  (func (export "as-br-value") (result i32)
    (block (result i32) (br 0 (unreachable)))
  )

  (func (export "as-br_if-cond")
    (block (br_if 0 (unreachable)))
  )
  (func (export "as-br_if-value") (result i32)
    (block (result i32)
      (drop (br_if 0 (unreachable) (i32.const 1))) (i32.const 7)
    )
  )
  (func (export "as-br_if-value-cond") (result i32)
    (block (result i32)
      (drop (br_if 0 (i32.const 6) (unreachable))) (i32.const 7)
    )
  )

  (func (export "as-br_table-index")
    (block (br_table 0 0 0 (unreachable)))
  )
  (func (export "as-br_table-value") (result i32)
    (block (result i32)
      (br_table 0 0 0 (unreachable) (i32.const 1)) (i32.const 7)
    )
  )
  (func (export "as-br_table-value-2") (result i32)
    (block (result i32)
      (block (result i32) (br_table 0 1 (unreachable) (i32.const 1)))
    )
  )
  (func (export "as-br_table-value-index") (result i32)
    (block (result i32)
      (br_table 0 0 (i32.const 6) (unreachable)) (i32.const 7)
    )
  )
  (func (export "as-br_table-value-and-index") (result i32)
    (block (result i32) (br_table 0 0 (unreachable)) (i32.const 8))
  )

  (func (export "as-return-value") (result i64)
    (return (unreachable))
  )

  (func (export "as-if-cond") (result i32)
    (if (result i32) (unreachable) (then (i32.const 0)) (else (i32.const 1)))
  )
  (func (export "as-if-then") (param i32 i32) (result i32)
    (if (result i32) (local.get 0) (then (unreachable)) (else (local.get 1)))
  )
  (func (export "as-if-else") (param i32 i32) (result i32)
    (if (result i32) (local.get 0) (then (local.get 1)) (else (unreachable)))
  )
  (func (export "as-if-then-no-else") (param i32 i32) (result i32)
    (if (local.get 0) (then (unreachable))) (local.get 1)
  )

  (func (export "as-select-first") (param i32 i32) (result i32)
    (select (unreachable) (local.get 0) (local.get 1))
  )
  (func (export "as-select-second") (param i32 i32) (result i32)
    (select (local.get 0) (unreachable) (local.get 1))
  )
  (func (export "as-select-cond") (result i32)
    (select (i32.const 0) (i32.const 1) (unreachable))
  )

  (func (export "as-call-first")
    (call $dummy3 (unreachable) (i32.const 2) (i32.const 3))
  )
  (func (export "as-call-mid")
    (call $dummy3 (i32.const 1) (unreachable) (i32.const 3))
  )
  (func (export "as-call-last")
    (call $dummy3 (i32.const 1) (i32.const 2) (unreachable))
  )

  (type $sig (func (param i32 i32 i32)))
  (table funcref (elem $dummy3))

  (func (export "as-local.set-value") (local f32)
    (local.set 0 (unreachable))
  )
  (func (export "as-local.tee-value") (result f32) (local f32)
    (local.tee 0 (unreachable))
  )
  (global $a (mut f32) (f32.const 0))
  (func (export "as-global.set-value") (result f32)
    (global.set $a (unreachable))
  )

  (memory 1)
  (func (export "as-load-address") (result f32)
    (f32.load (unreachable))
  )
  (func (export "as-loadN-address") (result i64)
    (i64.load8_s (unreachable))
  )

  (func (export "as-store-address")
    (f64.store (unreachable) (f64.const 7))
  )
  (func (export "as-store-value")
    (i64.store (i32.const 2) (unreachable))
  )

  (func (export "as-storeN-address")
    (i32.store8 (unreachable) (i32.const 7))
  )
  (func (export "as-storeN-value")
    (i64.store16 (i32.const 2) (unreachable))
  )

  (func (export "as-unary-operand") (result f32)
    (f32.neg (unreachable))
  )

  (func (export "as-binary-left") (result i32)
    (i32.add (unreachable) (i32.const 10))
  )
  (func (export "as-binary-right") (result i64)
    (i64.sub (i64.const 10) (unreachable))
  )

  (func (export "as-test-operand") (result i32)
    (i32.eqz (unreachable))
  )

  (func (export "as-compare-left") (result i32)
    (f64.le (unreachable) (f64.const 10))
  )
  (func (export "as-compare-right") (result i32)
    (f32.ne (f32.const 10) (unreachable))
  )

  (func (export "as-convert-operand") (result i32)
    (i32.wrap_i64 (unreachable))
  )

  (func (export "as-memory.grow-size") (result i32)
    (memory.grow (unreachable))
  )
)
"#
    );
}

#[test]
fn unwind() {
    assert_snapshot!(
        r#"
(module
  (func (export "func-unwind-by-unreachable")
    (i32.const 3) (i64.const 1) (unreachable)
  )
  (func (export "func-unwind-by-br")
    (i32.const 3) (i64.const 1) (br 0)
  )
  (func (export "func-unwind-by-br-value") (result i32)
    (i32.const 3) (i64.const 1) (br 0 (i32.const 9))
  )
  (func (export "func-unwind-by-br_if")
    (i32.const 3) (i64.const 1) (drop (drop (br_if 0 (i32.const 1))))
  )
  (func (export "func-unwind-by-br_if-value") (result i32)
    (i32.const 3) (i64.const 1) (drop (drop (br_if 0 (i32.const 9) (i32.const 1))))
  )
  (func (export "func-unwind-by-br_table")
    (i32.const 3) (i64.const 1) (br_table 0 (i32.const 0))
  )
  (func (export "func-unwind-by-br_table-value") (result i32)
    (i32.const 3) (i64.const 1) (br_table 0 (i32.const 9) (i32.const 0))
  )
  (func (export "func-unwind-by-return") (result i32)
    (i32.const 3) (i64.const 1) (return (i32.const 9))
  )

  (func (export "block-unwind-by-unreachable")
    (block (i32.const 3) (i64.const 1) (unreachable))
  )
  (func (export "block-unwind-by-br") (result i32)
    (block (i32.const 3) (i64.const 1) (br 0)) (i32.const 9)
  )
  (func (export "block-unwind-by-br-value") (result i32)
    (block (result i32) (i32.const 3) (i64.const 1) (br 0 (i32.const 9)))
  )
  (func (export "block-unwind-by-br_if") (result i32)
    (block (i32.const 3) (i64.const 1) (drop (drop (br_if 0 (i32.const 1))))) (i32.const 9)
  )
  (func (export "block-unwind-by-br_if-value") (result i32)
    (block (result i32)
      (i32.const 3) (i64.const 1) (drop (drop (br_if 0 (i32.const 9) (i32.const 1))))
    )
  )
  (func (export "block-unwind-by-br_table") (result i32)
    (block (i32.const 3) (i64.const 1) (br_table 0 (i32.const 0))) (i32.const 9)
  )
  (func (export "block-unwind-by-br_table-value") (result i32)
    (block (result i32)
      (i32.const 3) (i64.const 1) (br_table 0 (i32.const 9) (i32.const 0))
    )
  )
  (func (export "block-unwind-by-return") (result i32)
    (block (result i32) (i32.const 3) (i64.const 1) (return (i32.const 9)))
  )

  (func (export "block-nested-unwind-by-unreachable") (result i32)
    (block (result i32) (i32.const 3) (block (i64.const 1) (unreachable)))
  )
  (func (export "block-nested-unwind-by-br") (result i32)
    (block (i32.const 3) (block (i64.const 1) (br 1)) (drop)) (i32.const 9)
  )
  (func (export "block-nested-unwind-by-br-value") (result i32)
    (block (result i32)
      (i32.const 3) (block (i64.const 1) (br 1 (i32.const 9)))
    )
  )
  (func (export "block-nested-unwind-by-br_if") (result i32)
    (block (i32.const 3) (block (i64.const 1) (drop (br_if 1 (i32.const 1)))) (drop)) (i32.const 9)
  )
  (func (export "block-nested-unwind-by-br_if-value") (result i32)
    (block (result i32)
      (i32.const 3) (block (i64.const 1) (drop (drop (br_if 1 (i32.const 9) (i32.const 1)))))
    )
  )
  (func (export "block-nested-unwind-by-br_table") (result i32)
    (block
      (i32.const 3) (block (i64.const 1) (br_table 1 (i32.const 1)))
      (drop)
    )
    (i32.const 9)
  )
  (func (export "block-nested-unwind-by-br_table-value") (result i32)
    (block (result i32)
      (i32.const 3)
      (block (i64.const 1) (br_table 1 (i32.const 9) (i32.const 1)))
    )
  )
  (func (export "block-nested-unwind-by-return") (result i32)
    (block (result i32)
      (i32.const 3) (block (i64.const 1) (return (i32.const 9)))
    )
  )

  (func (export "unary-after-unreachable") (result i32)
    (f32.const 0) (unreachable) (i64.eqz)
  )
  (func (export "unary-after-br") (result i32)
    (block (result i32) (f32.const 0) (br 0 (i32.const 9)) (i64.eqz))
  )
  (func (export "unary-after-br_if") (result i32)
    (block (result i32)
      (i64.const 0) (drop (br_if 0 (i32.const 9) (i32.const 1))) (i64.eqz)
    )
  )
  (func (export "unary-after-br_table") (result i32)
    (block (result i32)
      (f32.const 0) (br_table 0 0 (i32.const 9) (i32.const 0)) (i64.eqz)
    )
  )
  (func (export "unary-after-return") (result i32)
    (f32.const 0) (return (i32.const 9)) (i64.eqz)
  )

  (func (export "binary-after-unreachable") (result i32)
    (f32.const 0) (f64.const 1) (unreachable) (i64.eq)
  )
  (func (export "binary-after-br") (result i32)
    (block (result i32)
      (f32.const 0) (f64.const 1) (br 0 (i32.const 9)) (i64.eq)
    )
  )
  (func (export "binary-after-br_if") (result i32)
    (block (result i32)
      (i64.const 0) (i64.const 1) (drop (br_if 0 (i32.const 9) (i32.const 1)))
      (i64.eq)
    )
  )
  (func (export "binary-after-br_table") (result i32)
    (block (result i32)
      (f32.const 0) (f64.const 1) (br_table 0 (i32.const 9) (i32.const 0))
      (i64.eq)
    )
  )
  (func (export "binary-after-return") (result i32)
    (f32.const 0) (f64.const 1) (return (i32.const 9)) (i64.eq)
  )

  (func (export "select-after-unreachable") (result i32)
    (f32.const 0) (f64.const 1) (i64.const 0) (unreachable) (select)
  )
  (func (export "select-after-br") (result i32)
    (block (result i32)
      (f32.const 0) (f64.const 1) (i64.const 0) (br 0 (i32.const 9)) (select)
    )
  )
  (func (export "select-after-br_if") (result i32)
    (block (result i32)
      (i32.const 0) (i32.const 1) (i32.const 0)
      (drop (br_if 0 (i32.const 9) (i32.const 1)))
      (select)
    )
  )
  (func (export "select-after-br_table") (result i32)
    (block (result i32)
      (f32.const 0) (f64.const 1) (i64.const 0)
      (br_table 0 (i32.const 9) (i32.const 0))
      (select)
    )
  )
  (func (export "select-after-return") (result i32)
    (f32.const 0) (f64.const 1) (i64.const 1) (return (i32.const 9)) (select)
  )

  (func (export "block-value-after-unreachable") (result i32)
    (block (result i32) (f32.const 0) (unreachable))
  )
  (func (export "block-value-after-br") (result i32)
    (block (result i32) (f32.const 0) (br 0 (i32.const 9)))
  )
  (func (export "block-value-after-br_if") (result i32)
    (block (result i32)
      (i32.const 0) (drop (br_if 0 (i32.const 9) (i32.const 1)))
    )
  )
  (func (export "block-value-after-br_table") (result i32)
    (block (result i32)
      (f32.const 0) (br_table 0 0 (i32.const 9) (i32.const 0))
    )
  )
  (func (export "block-value-after-return") (result i32)
    (block (result i32) (f32.const 0) (return (i32.const 9)))
  )

  (func (export "loop-value-after-unreachable") (result i32)
    (loop (result i32) (f32.const 0) (unreachable))
  )
  (func (export "loop-value-after-br") (result i32)
    (block (result i32) (loop (result i32) (f32.const 0) (br 1 (i32.const 9))))
  )
  (func (export "loop-value-after-br_if") (result i32)
    (block (result i32)
      (loop (result i32)
        (i32.const 0) (drop (br_if 1 (i32.const 9) (i32.const 1)))
      )
    )
  )

  (func (export "loop-value-after-br_table") (result i32)
    (block (result i32)
      (loop (result i32)
        (f32.const 0) (br_table 1 1 (i32.const 9) (i32.const 0))
      )
    )
  )
  (func (export "loop-value-after-return") (result i32)
    (loop (result i32) (f32.const 0) (return (i32.const 9)))
  )
)
"#
    );
}
