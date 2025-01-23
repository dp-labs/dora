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
fn fib() {
    assert_snapshot!(
        r#"
(module
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
)
"#
    );
}

#[test]
fn align_read_write() {
    assert_snapshot!(
        r#"
(module
  (memory 1)

  ;; $default: natural alignment, $1: align=1, $2: align=2, $4: align=4, $8: align=8

  (func (export "f32_align_switch") (param i32) (result f32)
    (local f32 f32)
    (local.set 1 (f32.const 10.0))
    (block $4
      (block $2
        (block $1
          (block $default
            (block $0
              (br_table $0 $default $1 $2 $4 (local.get 0))
            ) ;; 0
            (f32.store (i32.const 0) (local.get 1))
            (local.set 2 (f32.load (i32.const 0)))
            (br $4)
          ) ;; default
          (f32.store align=1 (i32.const 0) (local.get 1))
          (local.set 2 (f32.load align=1 (i32.const 0)))
          (br $4)
        ) ;; 1
        (f32.store align=2 (i32.const 0) (local.get 1))
        (local.set 2 (f32.load align=2 (i32.const 0)))
        (br $4)
      ) ;; 2
      (f32.store align=4 (i32.const 0) (local.get 1))
      (local.set 2 (f32.load align=4 (i32.const 0)))
    ) ;; 4
    (local.get 2)
  )

  (func (export "f64_align_switch") (param i32) (result f64)
    (local f64 f64)
    (local.set 1 (f64.const 10.0))
    (block $8
      (block $4
        (block $2
          (block $1
            (block $default
              (block $0
                (br_table $0 $default $1 $2 $4 $8 (local.get 0))
              ) ;; 0
              (f64.store (i32.const 0) (local.get 1))
              (local.set 2 (f64.load (i32.const 0)))
              (br $8)
            ) ;; default
            (f64.store align=1 (i32.const 0) (local.get 1))
            (local.set 2 (f64.load align=1 (i32.const 0)))
            (br $8)
          ) ;; 1
          (f64.store align=2 (i32.const 0) (local.get 1))
          (local.set 2 (f64.load align=2 (i32.const 0)))
          (br $8)
        ) ;; 2
        (f64.store align=4 (i32.const 0) (local.get 1))
        (local.set 2 (f64.load align=4 (i32.const 0)))
        (br $8)
      ) ;; 4
      (f64.store align=8 (i32.const 0) (local.get 1))
      (local.set 2 (f64.load align=8 (i32.const 0)))
    ) ;; 8
    (local.get 2)
  )

  ;; $8s: i32/i64.load8_s, $8u: i32/i64.load8_u, $16s: i32/i64.load16_s, $16u: i32/i64.load16_u, $32: i32.load
  ;; $32s: i64.load32_s, $32u: i64.load32_u, $64: i64.load

  (func (export "i32_align_switch") (param i32 i32) (result i32)
    (local i32 i32)
    (local.set 2 (i32.const 10))
    (block $32
      (block $16u
        (block $16s
          (block $8u
            (block $8s
              (block $0
                (br_table $0 $8s $8u $16s $16u $32 (local.get 0))
              ) ;; 0
              (if (i32.eq (local.get 1) (i32.const 0))
                (then
                  (i32.store8 (i32.const 0) (local.get 2))
                  (local.set 3 (i32.load8_s (i32.const 0)))
                )
              )
              (if (i32.eq (local.get 1) (i32.const 1))
                (then
                  (i32.store8 align=1 (i32.const 0) (local.get 2))
                  (local.set 3 (i32.load8_s align=1 (i32.const 0)))
                )
              )
              (br $32)
            ) ;; 8s
            (if (i32.eq (local.get 1) (i32.const 0))
              (then
                (i32.store8 (i32.const 0) (local.get 2))
                (local.set 3 (i32.load8_u (i32.const 0)))
              )
            )
            (if (i32.eq (local.get 1) (i32.const 1))
              (then
                (i32.store8 align=1 (i32.const 0) (local.get 2))
                (local.set 3 (i32.load8_u align=1 (i32.const 0)))
              )
            )
            (br $32)
          ) ;; 8u
          (if (i32.eq (local.get 1) (i32.const 0))
            (then
              (i32.store16 (i32.const 0) (local.get 2))
              (local.set 3 (i32.load16_s (i32.const 0)))
            )
          )
          (if (i32.eq (local.get 1) (i32.const 1))
            (then
              (i32.store16 align=1 (i32.const 0) (local.get 2))
              (local.set 3 (i32.load16_s align=1 (i32.const 0)))
            )
          )
          (if (i32.eq (local.get 1) (i32.const 2))
            (then
              (i32.store16 align=2 (i32.const 0) (local.get 2))
              (local.set 3 (i32.load16_s align=2 (i32.const 0)))
            )
          )
          (br $32)
        ) ;; 16s
        (if (i32.eq (local.get 1) (i32.const 0))
          (then
            (i32.store16 (i32.const 0) (local.get 2))
            (local.set 3 (i32.load16_u (i32.const 0)))
          )
        )
        (if (i32.eq (local.get 1) (i32.const 1))
          (then
            (i32.store16 align=1 (i32.const 0) (local.get 2))
            (local.set 3 (i32.load16_u align=1 (i32.const 0)))
          )
        )
        (if (i32.eq (local.get 1) (i32.const 2))
          (then
            (i32.store16 align=2 (i32.const 0) (local.get 2))
            (local.set 3 (i32.load16_u align=2 (i32.const 0)))
          )
        )
        (br $32)
      ) ;; 16u
      (if (i32.eq (local.get 1) (i32.const 0))
        (then
          (i32.store (i32.const 0) (local.get 2))
          (local.set 3 (i32.load (i32.const 0)))
        )
      )
      (if (i32.eq (local.get 1) (i32.const 1))
        (then
          (i32.store align=1 (i32.const 0) (local.get 2))
          (local.set 3 (i32.load align=1 (i32.const 0)))
        )
      )
      (if (i32.eq (local.get 1) (i32.const 2))
        (then
          (i32.store align=2 (i32.const 0) (local.get 2))
          (local.set 3 (i32.load align=2 (i32.const 0)))
        )
      )
      (if (i32.eq (local.get 1) (i32.const 4))
        (then
          (i32.store align=4 (i32.const 0) (local.get 2))
          (local.set 3 (i32.load align=4 (i32.const 0)))
        )
      )
    ) ;; 32
    (local.get 3)
  )

  (func (export "i64_align_switch") (param i32 i32) (result i64)
    (local i64 i64)
    (local.set 2 (i64.const 10))
    (block $64
      (block $32u
        (block $32s
          (block $16u
            (block $16s
              (block $8u
                (block $8s
                  (block $0
                    (br_table $0 $8s $8u $16s $16u $32s $32u $64 (local.get 0))
                  ) ;; 0
                  (if (i32.eq (local.get 1) (i32.const 0))
                    (then
                      (i64.store8 (i32.const 0) (local.get 2))
                      (local.set 3 (i64.load8_s (i32.const 0)))
                    )
                  )
                  (if (i32.eq (local.get 1) (i32.const 1))
                    (then
                      (i64.store8 align=1 (i32.const 0) (local.get 2))
                      (local.set 3 (i64.load8_s align=1 (i32.const 0)))
                    )
                  )
                  (br $64)
                ) ;; 8s
                (if (i32.eq (local.get 1) (i32.const 0))
                  (then
                    (i64.store8 (i32.const 0) (local.get 2))
                    (local.set 3 (i64.load8_u (i32.const 0)))
                  )
                )
                (if (i32.eq (local.get 1) (i32.const 1))
                  (then
                    (i64.store8 align=1 (i32.const 0) (local.get 2))
                    (local.set 3 (i64.load8_u align=1 (i32.const 0)))
                  )
                )
                (br $64)
              ) ;; 8u
              (if (i32.eq (local.get 1) (i32.const 0))
                (then
                  (i64.store16 (i32.const 0) (local.get 2))
                  (local.set 3 (i64.load16_s (i32.const 0)))
                )
              )
              (if (i32.eq (local.get 1) (i32.const 1))
                (then
                  (i64.store16 align=1 (i32.const 0) (local.get 2))
                  (local.set 3 (i64.load16_s align=1 (i32.const 0)))
                )
              )
              (if (i32.eq (local.get 1) (i32.const 2))
                (then
                  (i64.store16 align=2 (i32.const 0) (local.get 2))
                  (local.set 3 (i64.load16_s align=2 (i32.const 0)))
                )
              )
              (br $64)
            ) ;; 16s
            (if (i32.eq (local.get 1) (i32.const 0))
              (then
                (i64.store16 (i32.const 0) (local.get 2))
                (local.set 3 (i64.load16_u (i32.const 0)))
              )
            )
            (if (i32.eq (local.get 1) (i32.const 1))
              (then
                (i64.store16 align=1 (i32.const 0) (local.get 2))
                (local.set 3 (i64.load16_u align=1 (i32.const 0)))
              )
            )
            (if (i32.eq (local.get 1) (i32.const 2))
              (then
                (i64.store16 align=2 (i32.const 0) (local.get 2))
                (local.set 3 (i64.load16_u align=2 (i32.const 0)))
              )
            )
            (br $64)
          ) ;; 16u
          (if (i32.eq (local.get 1) (i32.const 0))
            (then
              (i64.store32 (i32.const 0) (local.get 2))
              (local.set 3 (i64.load32_s (i32.const 0)))
            )
          )
          (if (i32.eq (local.get 1) (i32.const 1))
            (then
              (i64.store32 align=1 (i32.const 0) (local.get 2))
              (local.set 3 (i64.load32_s align=1 (i32.const 0)))
            )
          )
          (if (i32.eq (local.get 1) (i32.const 2))
            (then
              (i64.store32 align=2 (i32.const 0) (local.get 2))
              (local.set 3 (i64.load32_s align=2 (i32.const 0)))
            )
          )
          (if (i32.eq (local.get 1) (i32.const 4))
            (then
              (i64.store32 align=4 (i32.const 0) (local.get 2))
              (local.set 3 (i64.load32_s align=4 (i32.const 0)))
            )
          )
          (br $64)
        ) ;; 32s
        (if (i32.eq (local.get 1) (i32.const 0))
          (then
            (i64.store32 (i32.const 0) (local.get 2))
            (local.set 3 (i64.load32_u (i32.const 0)))
          )
        )
        (if (i32.eq (local.get 1) (i32.const 1))
          (then
            (i64.store32 align=1 (i32.const 0) (local.get 2))
            (local.set 3 (i64.load32_u align=1 (i32.const 0)))
          )
        )
        (if (i32.eq (local.get 1) (i32.const 2))
          (then
            (i64.store32 align=2 (i32.const 0) (local.get 2))
            (local.set 3 (i64.load32_u align=2 (i32.const 0)))
          )
        )
        (if (i32.eq (local.get 1) (i32.const 4))
          (then
            (i64.store32 align=4 (i32.const 0) (local.get 2))
            (local.set 3 (i64.load32_u align=4 (i32.const 0)))
          )
        )
        (br $64)
      ) ;; 32u
      (if (i32.eq (local.get 1) (i32.const 0))
        (then
          (i64.store (i32.const 0) (local.get 2))
          (local.set 3 (i64.load (i32.const 0)))
        )
      )
      (if (i32.eq (local.get 1) (i32.const 1))
        (then
          (i64.store align=1 (i32.const 0) (local.get 2))
          (local.set 3 (i64.load align=1 (i32.const 0)))
        )
      )
      (if (i32.eq (local.get 1) (i32.const 2))
        (then
          (i64.store align=2 (i32.const 0) (local.get 2))
          (local.set 3 (i64.load align=2 (i32.const 0)))
        )
      )
      (if (i32.eq (local.get 1) (i32.const 4))
        (then
          (i64.store align=4 (i32.const 0) (local.get 2))
          (local.set 3 (i64.load align=4 (i32.const 0)))
        )
      )
      (if (i32.eq (local.get 1) (i32.const 8))
        (then
          (i64.store align=8 (i32.const 0) (local.get 2))
          (local.set 3 (i64.load align=8 (i32.const 0)))
        )
      )
    ) ;; 64
    (local.get 3)
  )
)
"#
    );
}

#[test]
fn complex_logic() {
    assert_snapshot!(
        r#"
(module
  (table (;0;) 60 60 funcref)
  (memory (;0;) 17)
  (global (;0;) (mut i32) i32.const 1048576)

  (type (;0;) (func (param i32)))
  (type (;1;) (func (param i32 i32 i32)))
  (type (;2;) (func (param i32 i32)))
  (type (;3;) (func (param i32 i32 i32) (result i32)))
  (type (;4;) (func (param i64 i32 i32) (result i32)))
  (func (;0;) (type 3) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32)
    block ;; label = @1
      block ;; label = @2
        local.get 2
        i32.const 16
        i32.ge_u
        br_if 0 (;@2;)
        local.get 0
        local.set 3
        br 1 (;@1;)
      end
      local.get 0
      i32.const 0
      local.get 0
      i32.sub
      i32.const 3
      i32.and
      local.tee 4
      i32.add
      local.set 5
      block ;; label = @2
        local.get 4
        i32.eqz
        br_if 0 (;@2;)
        local.get 0
        local.set 3
        local.get 1
        local.set 6
        loop ;; label = @3
          local.get 3
          local.get 6
          i32.load8_u
          i32.store8
          local.get 6
          i32.const 1
          i32.add
          local.set 6
          local.get 3
          i32.const 1
          i32.add
          local.tee 3
          local.get 5
          i32.lt_u
          br_if 0 (;@3;)
        end
      end
      local.get 5
      local.get 2
      local.get 4
      i32.sub
      local.tee 7
      i32.const -4
      i32.and
      local.tee 8
      i32.add
      local.set 3
      block ;; label = @2
        block ;; label = @3
          local.get 1
          local.get 4
          i32.add
          local.tee 9
          i32.const 3
          i32.and
          i32.eqz
          br_if 0 (;@3;)
          local.get 8
          i32.const 1
          i32.lt_s
          br_if 1 (;@2;)
          local.get 9
          i32.const 3
          i32.shl
          local.tee 6
          i32.const 24
          i32.and
          local.set 2
          local.get 9
          i32.const -4
          i32.and
          local.tee 10
          i32.const 4
          i32.add
          local.set 1
          i32.const 0
          local.get 6
          i32.sub
          i32.const 24
          i32.and
          local.set 4
          local.get 10
          i32.load
          local.set 6
          loop ;; label = @4
            local.get 5
            local.get 6
            local.get 2
            i32.shr_u
            local.get 1
            i32.load
            local.tee 6
            local.get 4
            i32.shl
            i32.or
            i32.store
            local.get 1
            i32.const 4
            i32.add
            local.set 1
            local.get 5
            i32.const 4
            i32.add
            local.tee 5
            local.get 3
            i32.lt_u
            br_if 0 (;@4;)
            br 2 (;@2;)
          end
        end
        local.get 8
        i32.const 1
        i32.lt_s
        br_if 0 (;@2;)
        local.get 9
        local.set 1
        loop ;; label = @3
          local.get 5
          local.get 1
          i32.load
          i32.store
          local.get 1
          i32.const 4
          i32.add
          local.set 1
          local.get 5
          i32.const 4
          i32.add
          local.tee 5
          local.get 3
          i32.lt_u
          br_if 0 (;@3;)
        end
      end
      local.get 7
      i32.const 3
      i32.and
      local.set 2
      local.get 9
      local.get 8
      i32.add
      local.set 1
    end
    block ;; label = @1
      local.get 2
      i32.eqz
      br_if 0 (;@1;)
      local.get 3
      local.get 2
      i32.add
      local.set 5
      loop ;; label = @2
        local.get 3
        local.get 1
        i32.load8_u
        i32.store8
        local.get 1
        i32.const 1
        i32.add
        local.set 1
        local.get 3
        i32.const 1
        i32.add
        local.tee 3
        local.get 5
        i32.lt_u
        br_if 0 (;@2;)
      end
    end
    local.get 0
  )
  (func (;1;) (type 3) (param i32 i32 i32) (result i32)
    (local i32 i32 i32)
    i32.const 0
    local.set 3
    block ;; label = @1
      local.get 2
      i32.eqz
      br_if 0 (;@1;)
      block ;; label = @2
        loop ;; label = @3
          local.get 0
          i32.load8_u
          local.tee 4
          local.get 1
          i32.load8_u
          local.tee 5
          i32.ne
          br_if 1 (;@2;)
          local.get 0
          i32.const 1
          i32.add
          local.set 0
          local.get 1
          i32.const 1
          i32.add
          local.set 1
          local.get 2
          i32.const -1
          i32.add
          local.tee 2
          i32.eqz
          br_if 2 (;@1;)
          br 0 (;@3;)
        end
      end
      local.get 4
      local.get 5
      i32.sub
      local.set 3
    end
    local.get 3
  )
  (func (;2;) (type 3) (param i32 i32 i32) (result i32)
    local.get 0
    local.get 1
    local.get 2
    call 1
  )
  (func (;3;) (type 3) (param i32 i32 i32) (result i32)
    local.get 0
    local.get 1
    local.get 2
    call 2
  )
  (func (;4;) (type 4) (param i64 i32 i32) (result i32)
    (local i32 i32 i64 i32 i32 i32)
    global.get 0
    i32.const 48
    i32.sub
    local.tee 3
    global.set 0
    i32.const 39
    local.set 4
    block ;; label = @1
      block ;; label = @2
        local.get 0
        i64.const 10000
        i64.ge_u
        br_if 0 (;@2;)
        local.get 0
        local.set 5
        br 1 (;@1;)
      end
      i32.const 39
      local.set 4
      loop ;; label = @2
        local.get 3
        i32.const 9
        i32.add
        local.get 4
        i32.add
        local.tee 6
        i32.const -4
        i32.add
        local.get 0
        local.get 0
        i64.const 10000
        i64.div_u
        local.tee 5
        i64.const 10000
        i64.mul
        i64.sub
        i32.wrap_i64
        local.tee 7
        i32.const 65535
        i32.and
        i32.const 100
        i32.div_u
        local.tee 8
        i32.const 1
        i32.shl
        i32.const 1050004
        i32.add
        i32.load16_u align=1
        i32.store16 align=1
        local.get 6
        i32.const -2
        i32.add
        local.get 7
        local.get 8
        i32.const 100
        i32.mul
        i32.sub
        i32.const 65535
        i32.and
        i32.const 1
        i32.shl
        i32.const 1050004
        i32.add
        i32.load16_u align=1
        i32.store16 align=1
        local.get 4
        i32.const -4
        i32.add
        local.set 4
        local.get 0
        i64.const 99999999
        i64.gt_u
        local.set 6
        local.get 5
        local.set 0
        local.get 6
        br_if 0 (;@2;)
      end
    end
    block ;; label = @1
      local.get 5
      i32.wrap_i64
      local.tee 6
      i32.const 99
      i32.le_u
      br_if 0 (;@1;)
      local.get 3
      i32.const 9
      i32.add
      local.get 4
      i32.const -2
      i32.add
      local.tee 4
      i32.add
      local.get 5
      i32.wrap_i64
      local.tee 6
      local.get 6
      i32.const 65535
      i32.and
      i32.const 100
      i32.div_u
      local.tee 6
      i32.const 100
      i32.mul
      i32.sub
      i32.const 65535
      i32.and
      i32.const 1
      i32.shl
      i32.const 1050004
      i32.add
      i32.load16_u align=1
      i32.store16 align=1
    end
    block ;; label = @1
      block ;; label = @2
        local.get 6
        i32.const 10
        i32.lt_u
        br_if 0 (;@2;)
        local.get 3
        i32.const 9
        i32.add
        local.get 4
        i32.const -2
        i32.add
        local.tee 4
        i32.add
        local.get 6
        i32.const 1
        i32.shl
        i32.const 1050004
        i32.add
        i32.load16_u align=1
        i32.store16 align=1
        br 1 (;@1;)
      end
      local.get 3
      i32.const 9
      i32.add
      local.get 4
      i32.const -1
      i32.add
      local.tee 4
      i32.add
      local.get 6
      i32.const 48
      i32.or
      i32.store8
    end
    local.get 2
    local.get 1

  )
)
"#
    );
}

#[test]
fn counter_contract() {
    assert_snapshot!(include_str!("suites/counter.wat"));
}
