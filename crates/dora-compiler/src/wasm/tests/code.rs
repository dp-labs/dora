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
