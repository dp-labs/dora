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
  ;; (func (export "as-call_indirect-first") (result i32)
  ;;   (block (result i32)
  ;;     (call_indirect (type $check)
  ;;       (block (result i32) (i32.const 1)) (i32.const 2) (i32.const 0)
  ;;     )
  ;;   )
  ;; )
  ;; (func (export "as-call_indirect-mid") (result i32)
  ;;   (block (result i32)
  ;;     (call_indirect (type $check)
  ;;       (i32.const 2) (block (result i32) (i32.const 1)) (i32.const 0)
  ;;     )
  ;;   )
  ;; )
  ;; (func (export "as-call_indirect-last") (result i32)
  ;;   (block (result i32)
  ;;     (call_indirect (type $check)
  ;;       (i32.const 1) (i32.const 2) (block (result i32) (i32.const 0))
  ;;     )
  ;;   )
  ;; )

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
