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
    assert_snapshot!(include_str!("suites/sum.wat"));
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
fn int_exprs() {
    assert_snapshot!(include_str!("suites/int_exprs.wat"));
}

#[test]
fn int_literals() {
    assert_snapshot!(include_str!("suites/int_literals.wat"));
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
fn float_literals() {
    assert_snapshot!(include_str!("suites/float_literals.wat"));
}

#[test]
fn forward() {
    assert_snapshot!(include_str!("suites/forward.wat"));
}

#[test]
fn float_memory() {
    assert_snapshot!(include_str!("suites/float_memory.wat"));
}

#[test]
fn global_get_and_set() {
    assert_snapshot!(include_str!("suites/global_get_and_set.wat"));
}

#[test]
fn local_get_and_set() {
    assert_snapshot!(include_str!("suites/local_get_and_set.wat"));
}

#[test]
fn local_call() {
    assert_snapshot!(include_str!("suites/local_call.wat"));
}

#[test]
fn select() {
    assert_snapshot!(include_str!("suites/select.wat"));
}

#[test]
fn mem_i32_load() {
    assert_snapshot!(include_str!("suites/mem_i32_load.wat"));
}

#[test]
fn address() {
    assert_snapshot!(include_str!("suites/address.wat"));
}

#[test]
fn align() {
    assert_snapshot!(include_str!("suites/align.wat"));
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
fn call() {
    assert_snapshot!(include_str!("suites/call.wat"));
}

#[test]
fn conversions() {
    assert_snapshot!(include_str!("suites/conversions.wat"));
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
fn fib() {
    assert_snapshot!(include_str!("suites/fib.wat"));
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
fn simple_if() {
    assert_snapshot!(include_str!("suites/simple_if.wat"));
}

#[test]
fn r#if() {
    assert_snapshot!(include_str!("suites/if.wat"));
}

#[test]
fn imports_0() {
    assert_snapshot!(include_str!("suites/imports_0.wat"));
}

#[test]
fn imports_1() {
    assert_snapshot!(include_str!("suites/imports_1.wat"));
}

#[test]
fn imports_2() {
    assert_snapshot!(include_str!("suites/imports_2.wat"));
}

#[test]
fn imports_3() {
    assert_snapshot!(include_str!("suites/imports_3.wat"));
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
fn local_tee() {
    assert_snapshot!(include_str!("suites/local_tee.wat"));
}

#[test]
fn r#loop() {
    assert_snapshot!(include_str!("suites/loop.wat"));
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
fn stack() {
    assert_snapshot!(include_str!("suites/stack.wat"));
}

#[test]
fn stack_guard_page() {
    assert_snapshot!(include_str!("suites/stack_guard_page.wat"));
}

#[test]
fn store() {
    assert_snapshot!(include_str!("suites/store.wat"));
}

#[test]
fn r#switch() {
    assert_snapshot!(include_str!("suites/switch.wat"));
}

#[test]
fn table_copy() {
    assert_snapshot!(include_str!("suites/table_copy.wat"));
}

#[test]
fn table_fill() {
    assert_snapshot!(include_str!("suites/table_fill.wat"));
}

#[test]
fn table_get() {
    assert_snapshot!(include_str!("suites/table_get.wat"));
}

#[test]
fn table_grow() {
    assert_snapshot!(include_str!("suites/table_grow.wat"));
}

#[test]
fn table_init() {
    assert_snapshot!(include_str!("suites/table_init.wat"));
}

#[test]
fn table_set() {
    assert_snapshot!(include_str!("suites/table_set.wat"));
}

#[test]
fn table_size() {
    assert_snapshot!(include_str!("suites/table_size.wat"));
}

#[test]
fn r#type() {
    assert_snapshot!(include_str!("suites/type.wat"));
}

#[test]
fn unreachable() {
    assert_snapshot!(include_str!("suites/unreachable.wat"));
}

#[test]
fn unwind() {
    assert_snapshot!(include_str!("suites/unwind.wat"));
}
