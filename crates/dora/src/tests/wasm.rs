//！Reference: https://github.com/WebAssembly/spec/tree/main/test/core

use crate::build_wasm_artifact;
use crate::MemoryDB;
use anyhow::Result;
use wasmer::wat2wasm;

macro_rules! build_wasm_code {
    ($code:ident, $artifact:ident) => {
        let wasm_code = wat2wasm($code).unwrap();
        let $artifact = build_wasm_artifact::<MemoryDB>(&wasm_code.to_vec().into()).unwrap();
    };
    ($code:ident, $artifact:ident, $runtime_context:ident, $gas:ident) => {
        let wasm_code = wat2wasm($code).unwrap();
        let $artifact = build_wasm_artifact::<MemoryDB>(&wasm_code.to_vec().into()).unwrap();
        // Run WASM code with env.
        let env = Env::default();
        let mut host = DummyHost::new(env);
        let $runtime_context = RuntimeContext::new(
            Contract::new_with_env(&host.env, Bytecode::new(wasm_code.to_vec().into()), None),
            1,
            false,
            false,
            &mut host,
            SpecId::CANCUN,
        );
        let $gas = INIT_GAS;
    };
}

macro_rules! generate_test_cases {
    ($artifact:expr, [ $(($func_name:expr, $arg:expr, $expect:expr, $ty:ty)),* $(,)? ]) => {
        $(
            {
                let result: $ty = $artifact.execute_wasm_func($func_name, $arg)?;
                assert_eq!(result, $expect, "Function: {} {:?} test failed.", $func_name, $arg);
            }
        )*
    };
}

// TODO: fix host api calling panic on macos.
#[test]
#[cfg(target_os = "linux")]
fn test_wasm_brainfuck_with_host_functions() {
    use crate::tests::INIT_GAS;
    use dora_primitives::Bytecode;
    use dora_primitives::SpecId;
    use dora_runtime::context::Contract;
    use dora_runtime::context::RuntimeContext;
    use dora_runtime::env::Env;
    use dora_runtime::host::DummyHost;

    let code = include_bytes!("../../../dora-compiler/src/wasm/tests/suites/brainfuck.wat");
    build_wasm_code!(code, artifact, runtime_context, gas);
    let result: i32 = artifact
        .execute_wasm_func_with_context("user_entrypoint", 0, runtime_context, gas)
        .unwrap();
    assert_eq!(result, 0);
}

#[test]
fn test_wasm_sum() -> Result<()> {
    let code = include_bytes!("../../../dora-compiler/src/wasm/tests/suites/sum.wat");
    build_wasm_code!(code, artifact);
    generate_test_cases!(&artifact, [("main", (), (), ()),]);
    Ok(())
}

#[test]
fn test_wasm_fib() -> Result<()> {
    let code = include_bytes!("../../../dora-compiler/src/wasm/tests/suites/fib.wat");
    build_wasm_code!(code, artifact);
    generate_test_cases!(
        &artifact,
        [
            ("fib-recursive", 0_i64, 0_i64, i64),
            ("fib-recursive", 1_i64, 1_i64, i64),
            ("fib-recursive", 2_i64, 1_i64, i64),
            ("fib-recursive", 5_i64, 5_i64, i64),
            ("fib-recursive", 6_i64, 8_i64, i64),
            ("fib-iterative", 0_i64, 0_i64, i64),
            ("fib-iterative", 1_i64, 1_i64, i64),
            ("fib-iterative", 2_i64, 1_i64, i64),
            ("fib-iterative", 5_i64, 5_i64, i64),
            ("fib-iterative", 6_i64, 8_i64, i64),
            ("fib-iterative", 100_i64, 3314859971_i64, i64),
        ]
    );
    Ok(())
}

#[test]
fn test_wasm_fib_2() -> Result<()> {
    let code = include_bytes!("../../../dora-compiler/src/wasm/tests/suites/fib_2.wat");
    build_wasm_code!(code, artifact);
    generate_test_cases!(
        &artifact,
        [
            ("fib", 0_i64, 1_i64, i64),
            ("fib", 1_i64, 1_i64, i64),
            ("fib", 2_i64, 2_i64, i64),
            ("fib", 3_i64, 3_i64, i64),
            ("fib", 4_i64, 5_i64, i64),
            ("fib", 5_i64, 8_i64, i64),
            ("fib", 6_i64, 13_i64, i64),
            ("fib", 7_i64, 21_i64, i64),
        ]
    );
    Ok(())
}

#[test]
fn test_wasm_global_value() -> Result<()> {
    let code = include_bytes!("../../../dora-compiler/src/wasm/tests/suites/global_value.wat");
    build_wasm_code!(code, artifact);
    generate_test_cases!(&artifact, [("user_entrypoint", 10, 10 + 255 + 255, i32),]);
    Ok(())
}

#[test]
fn test_wasm_address() -> Result<()> {
    let code = include_bytes!("../../../dora-compiler/src/wasm/tests/suites/address.wat");
    build_wasm_code!(code, artifact);
    generate_test_cases!(
        &artifact,
        [
            // i32.load_8u
            ("8u_good1", 0, 97, i32),
            ("8u_good2", 0, 97, i32),
            ("8u_good3", 0, 98, i32),
            ("8u_good4", 0, 99, i32),
            ("8u_good5", 0, 122, i32),
            // i32.load_8s
            ("8s_good1", 0, 97, i32),
            ("8s_good2", 0, 97, i32),
            ("8s_good3", 0, 98, i32),
            ("8s_good4", 0, 99, i32),
            ("8s_good5", 0, 122, i32),
            // i32.load_16u
            ("16u_good1", 0, 25185, i32),
            ("16u_good2", 0, 25185, i32),
            ("16u_good3", 0, 25442, i32),
            ("16u_good4", 0, 25699, i32),
            ("16u_good5", 0, 122, i32),
            // i32.load_16s
            ("16s_good1", 0, 25185, i32),
            ("16s_good2", 0, 25185, i32),
            ("16s_good3", 0, 25442, i32),
            ("16s_good4", 0, 25699, i32),
            ("16s_good5", 0, 122, i32),
            // i32.load
            ("32_good1", 0, 1684234849, i32),
            ("32_good2", 0, 1684234849, i32),
            ("32_good3", 0, 1701077858, i32),
            ("32_good4", 0, 1717920867, i32),
            ("32_good5", 0, 122, i32),
            // i32.load_8u
            ("8u_good1", 65507, 0, i32),
            ("8u_good2", 65507, 0, i32),
            ("8u_good3", 65507, 0, i32),
            ("8u_good4", 65507, 0, i32),
            ("8u_good5", 65507, 0, i32),
        ]
    );
    // TODO: Out of bounds memory access error deal
    // assert!(artifact.execute_wasm_func("32_good5", 65508).is_err());
    // assert!(artifact.execute_wasm_func("8u_good3", -1).is_err());
    // assert!(artifact.execute_wasm_func("8u_bad", 0).is_err());
    Ok(())
}

#[test]
fn test_wasm_align_read_write() -> Result<()> {
    let code = include_bytes!("../../../dora-compiler/src/wasm/tests/suites/align_read_write.wat");
    build_wasm_code!(code, artifact);
    generate_test_cases!(
        &artifact,
        [
            ("f32_align", (), 10.0_f32, f32),
            ("f32_align_switch", 0, 10.0_f32, f32),
            ("f32_align_switch", 1, 10.0_f32, f32),
            ("f32_align_switch", 2, 10.0_f32, f32),
            ("f32_align_switch", 3, 10.0_f32, f32),
            ("f64_align_switch", 0, 10.0_f64, f64),
            ("f64_align_switch", 1, 10.0_f64, f64),
            ("f64_align_switch", 2, 10.0_f64, f64),
            ("f64_align_switch", 3, 10.0_f64, f64),
            ("f64_align_switch", 4, 10.0_f64, f64),
            ("i32_align_switch", (0, 0), 10, i32),
            ("i32_align_switch", (0, 1), 10, i32),
            ("i32_align_switch", (1, 0), 10, i32),
            ("i32_align_switch", (1, 1), 10, i32),
            ("i32_align_switch", (2, 0), 10, i32),
            ("i32_align_switch", (2, 1), 10, i32),
            ("i32_align_switch", (2, 2), 10, i32),
            ("i32_align_switch", (3, 0), 10, i32),
            ("i32_align_switch", (3, 1), 10, i32),
            ("i32_align_switch", (3, 2), 10, i32),
            ("i32_align_switch", (4, 0), 10, i32),
            ("i32_align_switch", (4, 1), 10, i32),
            ("i32_align_switch", (4, 2), 10, i32),
            ("i32_align_switch", (4, 4), 10, i32),
            ("i64_align_switch", (0, 0), 10, i64),
            ("i64_align_switch", (0, 1), 10, i64),
            ("i64_align_switch", (1, 0), 10, i64),
            ("i64_align_switch", (1, 1), 10, i64),
            ("i64_align_switch", (2, 0), 10, i64),
            ("i64_align_switch", (2, 1), 10, i64),
            ("i64_align_switch", (2, 2), 10, i64),
            ("i64_align_switch", (3, 0), 10, i64),
            ("i64_align_switch", (3, 1), 10, i64),
            ("i64_align_switch", (3, 2), 10, i64),
            ("i64_align_switch", (4, 0), 10, i64),
            ("i64_align_switch", (4, 1), 10, i64),
            ("i64_align_switch", (4, 2), 10, i64),
            ("i64_align_switch", (4, 4), 10, i64),
            ("i64_align_switch", (5, 0), 10, i64),
            ("i64_align_switch", (5, 1), 10, i64),
            ("i64_align_switch", (5, 2), 10, i64),
            ("i64_align_switch", (5, 4), 10, i64),
            ("i64_align_switch", (6, 0), 10, i64),
            ("i64_align_switch", (6, 1), 10, i64),
            ("i64_align_switch", (6, 2), 10, i64),
            ("i64_align_switch", (6, 4), 10, i64),
            ("i64_align_switch", (6, 8), 10, i64),
        ]
    );
    Ok(())
}

// TODO: test errors on macos.
#[test]
#[cfg(target_os = "linux")]
fn test_wasm_block() -> Result<()> {
    let code = include_bytes!("../../../dora-compiler/src/wasm/tests/suites/block.wat");
    build_wasm_code!(code, artifact);
    generate_test_cases!(
        &artifact,
        [
            ("empty", (), (), ()),
            ("singular", (), 7, i32),
            ("multi", (), 8, i32),
            ("nested", (), 9, i32),
            ("deep", (), 150, i32),
            ("as-select-first", (), 1, i32),
            ("as-select-mid", (), 2, i32),
            ("as-select-last", (), 2, i32),
            ("as-loop-first", (), 1, i32),
            ("as-loop-mid", (), 1, i32),
            ("as-loop-last", (), 1, i32),
            ("as-if-condition", (), (), ()),
            ("as-if-then", (), 1, i32),
            ("as-if-else", (), 2, i32),
            ("as-br_if-first", (), 1, i32),
            ("as-br_if-last", (), 2, i32),
            ("as-br_table-first", (), 1, i32),
            ("as-br_table-last", (), 2, i32),
            ("as-call_indirect-first", (), 1, i32),
            ("as-call_indirect-mid", (), 2, i32),
            ("as-call_indirect-last", (), 1, i32),
            ("as-store-first", (), (), ()),
            ("as-store-last", (), (), ()),
            ("as-memory.grow-value", (), 1, i32),
            ("as-call-value", (), 1, i32),
            ("as-return-value", (), 1, i32),
            ("as-drop-operand", (), (), ()),
            ("as-br-value", (), 1, i32),
            ("as-local.set-value", (), 1, i32),
            ("as-local.tee-value", (), 1, i32),
            ("as-global.set-value", (), 1, i32),
            ("as-load-operand", (), 1, i32),
            ("as-unary-operand", (), 0, i32),
            ("as-binary-operand", (), 12, i32),
            ("as-test-operand", (), 0, i32),
            ("as-compare-operand", (), 0, i32),
            ("as-binary-operands", (), 12, i32),
            ("as-compare-operands", (), 0, i32),
            ("as-mixed-operands", (), 27, i32),
            ("break-bare", (), 19, i32),
            ("break-value", (), 18, i32),
            ("break-multi-value", (18, -18), 18, i32),
            ("break-repeated", (), 18, i32),
            ("break-inner", (), 0xF, i32),
            ("param", (), 3, i32),
            ("params", (), 3, i32),
            ("params-id", (), 3, i32),
            ("param-break", (), 3, i32),
            ("params-break", (), 3, i32),
            ("params-id-break", (), 3, i32),
            ("effects", (), 1, i32),
            ("type-use", (), (), ()),
        ]
    );
    Ok(())
}

#[test]
fn test_wasm_br() -> Result<()> {
    let code = include_bytes!("../../../dora-compiler/src/wasm/tests/suites/br.wat");
    build_wasm_code!(code, artifact);
    generate_test_cases!(
        &artifact,
        [
            ("type-i32", (), (), ()),
            ("type-i64", (), (), ()),
            ("type-f32", (), (), ()),
            ("type-f64", (), (), ()),
            ("type-i32-i32", (), (), ()),
            ("type-i64-i64", (), (), ()),
            ("type-f32-f32", (), (), ()),
            ("type-f64-f64", (), (), ()),
            ("type-i32-value", (), 1_i32, i32),
            ("type-i64-value", (), 2_i64, i64),
            ("type-f32-value", (), 3_f32, f32),
            ("type-f64-value", (), 4_f64, f64),
            ("type-f64-f64-value", (), (4_f64, 5_f64), (f64, f64)),
            ("as-block-first", (), (), ()),
            ("as-block-mid", (), (), ()),
            ("as-block-last", (), (), ()),
            ("as-block-value", (), 2_i32, i32),
            ("as-loop-first", (), (), ()),
            ("as-loop-mid", (), (), ()),
            ("as-loop-last", (), (), ()),
            ("as-br-value", (), (), ()),
            ("as-br_if-cond", (), (), ()),
            ("as-br_if-value", (), 8_i32, i32),
            ("as-br_if-value-cond", (), 9_i32, i32),
            ("as-br_table-index", (), (), ()),
            ("as-br_table-value", (), 10_i32, i32),
            ("as-br_table-value-index", (), 11_i32, i32),
            ("as-return-value", (), 7_i32, i32),
            ("as-return-values", (), (2_i32, 7_i32), (i32, i32)),
            ("as-if-cond", (), 2_i32, i32),
            ("as-if-then", (1_i32, 6_i32), 3_i32, i32),
            ("as-if-then", (0_i32, 6_i32), 6_i32, i32),
            ("as-if-else", (0_i32, 6_i32), 4_i32, i32),
            ("as-if-else", (1_i32, 6_i32), 6_i32, i32),
            ("as-select-first", (0_i32, 6_i32), 5_i32, i32),
            ("as-select-first", (1_i32, 6_i32), 5_i32, i32),
            ("as-select-second", (0_i32, 6_i32), 6_i32, i32),
            ("as-select-second", (1_i32, 6_i32), 6_i32, i32),
            ("as-select-cond", 7_i32, (), ()),
            ("as-select-all", 8_i32, (), ()),
            ("as-call-first", 12_i32, (), ()),
            ("as-call-mid", 13_i32, (), ()),
            ("as-call-last", 14_i32, (), ()),
            ("as-call-all", 15_i32, (), ()),
            ("as-call_indirect-func", 20_i32, (), ()),
            ("as-call_indirect-first", 21_i32, (), ()),
            ("as-call_indirect-mid", 22_i32, (), ()),
            ("as-call_indirect-last", 23_i32, (), ()),
            ("as-call_indirect-all", 24_i32, (), ()),
            ("as-local.set-value", 17_i32, (), ()),
            ("as-local.tee-value", 1_i32, (), ()),
            ("as-local.set-value", 1_i32, (), ()),
            ("as-load-address", (), 1.7_f32, f32),
            ("as-loadN-address", (), 30_i32, i32),
            ("as-store-address", (), 30_i32, i32),
            ("as-store-value", (), 31_i32, i32),
            ("as-store-both", (), 32_i32, i32),
            ("as-storeN-address", (), 32_i32, i32),
            ("as-storeN-value", (), 33_i32, i32),
            ("as-storeN-both", (), 34_i32, i32),
            ("as-unary-operand", (), 3.4_f32, f32),
            ("as-binary-left", (), 3_i32, i32),
            ("as-binary-right", (), 45_i64, i64),
            ("as-binary-both", (), 46_i32, i32),
            ("as-test-operand", (), 44_i32, i32),
            ("as-compare-left", (), 43_i32, i32),
            ("as-compare-right", (), 42_i32, i32),
            ("as-compare-both", (), 44_i32, i32),
            ("as-convert-operand", (), 41_i32, i32),
            ("as-memory.grow-size", (), 40_i32, i32),
            ("nested-block-value", (), 9_i32, i32),
            ("nested-br-value", (), 9_i32, i32),
            ("nested-br_if-value", (), 9_i32, i32),
            ("nested-br_if-value-cond", (), 9_i32, i32),
            ("nested-br_table-value", (), 9_i32, i32),
            ("nested-br_table-value-index", (), 9_i32, i32),
        ]
    );
    Ok(())
}

#[test]
fn test_wasm_br_if() -> Result<()> {
    let code = include_bytes!("../../../dora-compiler/src/wasm/tests/suites/br_if.wat");
    build_wasm_code!(code, artifact);
    generate_test_cases!(
        &artifact,
        [
            ("type-i32", (), (), ()),
            ("type-i64", (), (), ()),
            ("type-f32", (), (), ()),
            ("type-f64", (), (), ()),
            ("type-i32-value", (), 1_i32, i32),
            ("type-i64-value", (), 2_i64, i64),
            ("type-f32-value", (), 3_f32, f32),
            ("type-f64-value", (), 4_f64, f64),
            ("as-block-first", 0_i32, 2_i32, i32),
            ("as-block-first", 1_i32, 3_i32, i32),
            ("as-block-mid", 0_i32, 2_i32, i32),
            ("as-block-mid", 1_i32, 3_i32, i32),
            ("as-block-last", 0_i32, (), ()),
            ("as-block-last", 1_i32, (), ()),
            ("as-block-first-value", 0_i32, 11_i32, i32),
            ("as-block-first-value", 1_i32, 10_i32, i32),
            ("as-block-mid-value", 0_i32, 21_i32, i32),
            ("as-block-mid-value", 1_i32, 20_i32, i32),
            ("as-block-last-value", 0_i32, 11_i32, i32),
            ("as-block-last-value", 1_i32, 11_i32, i32),
            ("as-loop-first", 0_i32, 2_i32, i32),
            ("as-loop-first", 1_i32, 3_i32, i32),
            ("as-loop-mid", 0_i32, 2_i32, i32),
            ("as-loop-mid", 1_i32, 4_i32, i32),
            ("as-loop-last", 0_i32, (), ()),
            ("as-loop-last", 1_i32, (), ()),
            ("as-br-value", 1_i32, (), ()),
            ("as-br_if-cond", (), (), ()),
            ("as-br_if-value", 1_i32, (), ()),
            ("as-br_if-value-cond", 0_i32, 2_i32, i32),
            ("as-br_if-value-cond", 1_i32, 1_i32, i32),
            ("as-br_table-index", (), (), ()),
            ("as-br_table-value", 1_i32, (), ()),
            ("as-br_table-value-index", 1_i32, (), ()),
            ("as-return-value", 1_i32, (), ()),
            ("as-if-cond", 0, 2, i32),
            ("as-if-cond", 1, 1, i32),
            ("as-if-then", (0, 0), (), ()),
            ("as-if-then", (4, 0), (), ()),
            ("as-if-then", (0, 1), (), ()),
            ("as-if-then", (4, 1), (), ()),
            ("as-if-else", (0, 0), (), ()),
            ("as-if-else", (3, 0), (), ()),
            ("as-if-else", (0, 1), (), ()),
            ("as-if-else", (3, 1), (), ()),
            ("as-select-first", 0, 3, i32),
            ("as-select-first", 1, 3, i32),
            ("as-select-second", 0, 3, i32),
            ("as-select-second", 1, 3, i32),
            ("as-select-cond", 3, (), ()),
            ("as-call-first", 12, (), ()),
            ("as-call-mid", 13, (), ()),
            ("as-call-last", 14, (), ()),
            ("as-call_indirect-func", 4, (), ()),
            ("as-call_indirect-first", 4, (), ()),
            ("as-call_indirect-mid", 4, (), ()),
            ("as-call_indirect-last", 4, (), ()),
            ("as-local.set-value", 0_i32, -1_i32, i32),
            ("as-local.set-value", 1_i32, 17_i32, i32),
            ("as-local.tee-value", 0_i32, -1_i32, i32),
            ("as-local.tee-value", 1_i32, 1_i32, i32),
            ("as-global.set-value", 0_i32, -1_i32, i32),
            ("as-global.set-value", 1_i32, 1_i32, i32),
            ("as-load-address", 1_i32, (), ()),
            ("as-loadN-address", 30_i32, (), ()),
            ("as-store-address", 30_i32, (), ()),
            ("as-store-value", 31_i32, (), ()),
            ("as-storeN-address", 32_i32, (), ()),
            ("as-storeN-value", 33_i32, (), ()),
            ("as-unary-operand", 1.0_f64, (), ()),
            ("as-binary-left", 1_i32, (), ()),
            ("as-binary-right", 1_i32, (), ()),
            ("as-test-operand", 0_i32, (), ()),
            ("as-compare-left", 1_i32, (), ()),
            ("as-compare-right", 1_i32, (), ()),
            ("as-memory.grow-size", 1_i32, (), ()),
            ("nested-block-value", 0_i32, 21_i32, i32),
            ("nested-block-value", 1_i32, 9_i32, i32),
            ("nested-br-value", 0_i32, 5_i32, i32),
            ("nested-br-value", 1_i32, 9_i32, i32),
            ("nested-br_if-value", 0_i32, 5_i32, i32),
            ("nested-br_if-value", 1_i32, 9_i32, i32),
            ("nested-br_if-value-cond", 0_i32, 5_i32, i32),
            ("nested-br_if-value-cond", 1_i32, 9_i32, i32),
            ("nested-br_table-value", 0_i32, 5_i32, i32),
            ("nested-br_table-value", 1_i32, 9_i32, i32),
            ("nested-br_table-value-index", 0_i32, 5_i32, i32),
            ("nested-br_table-value-index", 1_i32, 9_i32, i32),
        ]
    );
    Ok(())
}

#[test]
fn test_wasm_br_table() -> Result<()> {
    let code = include_bytes!("../../../dora-compiler/src/wasm/tests/suites/br_table.wat");
    build_wasm_code!(code, artifact);
    generate_test_cases!(
        &artifact,
        [
            ("type-i32", (), (), ()),
            ("type-i64", (), (), ()),
            ("type-f32", (), (), ()),
            ("type-f64", (), (), ()),
            ("type-i32-value", (), 1_i32, i32),
            ("type-i64-value", (), 2_i64, i64),
            ("type-f32-value", (), 3_f32, f32),
            ("type-f64-value", (), 4_f64, f64),
            ("empty", 0, 22, i32),
            ("empty", 1, 22, i32),
            ("empty", 11, 22, i32),
            ("empty", -1, 22, i32),
            ("empty", -100, 22, i32),
            ("empty", 0xffffffffu32 as i32, 22, i32),
            ("empty-value", 0, 33, i32),
            ("empty-value", 1, 33, i32),
            ("empty-value", 11, 33, i32),
            ("empty-value", -1, 33, i32),
            ("empty-value", -100, 33, i32),
            ("empty-value", 0xffffffffu32 as i32, 33, i32),
            ("singleton", 0, 22, i32),
            ("singleton", 1, 20, i32),
            ("singleton", 11, 20, i32),
            ("singleton", -1, 20, i32),
            ("singleton", -100, 20, i32),
            ("singleton", 0xffffffffu32 as i32, 20, i32),
            ("multiple", 0, 103, i32),
            ("multiple", 1, 102, i32),
            ("multiple", 2, 101, i32),
            ("multiple", 3, 100, i32),
            ("multiple", 4, 104, i32),
            ("multiple", 5, 104, i32),
            ("multiple", 6, 104, i32),
            ("multiple", 10, 104, i32),
            ("multiple", -1, 104, i32),
            ("multiple", 0xffffffffu32 as i32, 104, i32),
            ("multiple-value", 0, 213, i32),
            ("multiple-value", 1, 212, i32),
            ("multiple-value", 2, 211, i32),
            ("multiple-value", 3, 210, i32),
            ("multiple-value", 4, 214, i32),
            ("multiple-value", 5, 214, i32),
            ("multiple-value", 6, 214, i32),
            ("multiple-value", 10, 214, i32),
            ("multiple-value", -1, 214, i32),
            ("multiple-value", 0xffffffffu32 as i32, 214, i32),
            ("large", 0, 0, i32),
            ("large", 1, 1, i32),
            ("large", 100, 0, i32),
            ("large", 101, 1, i32),
            ("large", 10000, 0, i32),
            ("large", 10001, 1, i32),
            ("large", 1000000, 1, i32),
            ("large", 1000001, 1, i32),
            ("as-block-first", (), (), ()),
            ("as-block-mid", (), (), ()),
            ("as-block-last", (), (), ()),
            ("as-block-value", 2, (), ()),
            ("as-loop-first", 2, (), ()),
            ("as-loop-mid", 3, (), ()),
            ("as-loop-last", 4, (), ()),
            ("as-br-value", 9, (), ()),
            ("as-br_if-cond", (), (), ()),
            ("as-br_if-value", 8, (), ()),
            ("as-br_if-value-cond", 9, (), ()),
            ("as-br_table-index", (), (), ()),
            ("as-br_table-value", 10, (), ()),
            ("as-br_table-value-index", 11, (), ()),
            ("as-return-value", 7, (), ()),
            ("as-if-cond", 7, (), ()),
            ("as-if-then", (1, 6), 3, i32),
            ("as-if-then", (0, 6), 6, i32),
            ("as-if-else", (0, 6), 4, i32),
            ("as-if-else", (1, 6), 6, i32),
            ("as-select-first", (0, 6), 5, i32),
            ("as-select-first", (1, 6), 5, i32),
            ("as-select-second", (0, 6), 6, i32),
            ("as-select-second", (1, 6), 6, i32),
            ("as-select-cond", 7, (), ()),
            ("as-call-first", 12, (), ()),
            ("as-call-mid", 13, (), ()),
            ("as-call-last", 14, (), ()),
            ("as-call_indirect-first", 20, (), ()),
            ("as-call_indirect-mid", 21, (), ()),
            ("as-call_indirect-last", 22, (), ()),
            ("as-call_indirect-func", 23, (), ()),
            ("as-local.set-value", 17, (), ()),
            ("as-local.tee-value", 1, (), ()),
            ("as-global.set-value", 1, (), ()),
            ("as-load-address", 1.7_f32, (), ()),
            ("as-loadN-address", 30_i32, (), ()),
            ("as-store-address", 30_i32, (), ()),
            ("as-store-value", 31_i32, (), ()),
            ("as-storeN-address", 32_i32, (), ()),
            ("as-storeN-value", 33_i32, (), ()),
            ("as-unary-operand", 3.4_f64, (), ()),
            ("as-binary-left", 3_i32, (), ()),
            ("as-binary-right", 45_i32, (), ()),
            ("as-test-operand", 44_i32, (), ()),
            ("as-compare-left", 43_i32, (), ()),
            ("as-compare-right", 42_i32, (), ()),
            ("as-convert-operand", 41_i32, (), ()),
            ("as-memory.grow-size", 40, (), ()),
            ("nested-block-value", 0_i32, 19_i32, i32),
            ("nested-block-value", 1_i32, 17_i32, i32),
            ("nested-block-value", 2_i32, 16_i32, i32),
            ("nested-block-value", 10_i32, 16_i32, i32),
            ("nested-block-value", -1_i32, 16_i32, i32),
            ("nested-block-value", 100000_i32, 16_i32, i32),
            ("nested-br-value", 0_i32, 8_i32, i32),
            ("nested-br-value", 1_i32, 9_i32, i32),
            ("nested-br-value", 2_i32, 17_i32, i32),
            ("nested-br-value", 11_i32, 17_i32, i32),
            ("nested-br-value", -4_i32, 17_i32, i32),
            ("nested-br-value", 10213210_i32, 17_i32, i32),
            ("nested-br_if-value", 0_i32, 17_i32, i32),
            ("nested-br_if-value", 1_i32, 9_i32, i32),
            ("nested-br_if-value", 2_i32, 8_i32, i32),
            ("nested-br_if-value", 9_i32, 8_i32, i32),
            ("nested-br_if-value", -9_i32, 8_i32, i32),
            ("nested-br_if-value", 999999_i32, 8_i32, i32),
            ("nested-br_if-value-cond", 0_i32, 9_i32, i32),
            ("nested-br_if-value-cond", 1_i32, 8_i32, i32),
            ("nested-br_if-value-cond", 2_i32, 9_i32, i32),
            ("nested-br_if-value-cond", 9_i32, 9_i32, i32),
            ("nested-br_if-value-cond", -1000000_i32, 9_i32, i32),
            ("nested-br_if-value-cond", 9423975_i32, 9_i32, i32),
            ("nested-br_table-value", 0_i32, 17_i32, i32),
            ("nested-br_table-value", 1_i32, 9_i32, i32),
            ("nested-br_table-value", 2_i32, 8_i32, i32),
            ("nested-br_table-value", 9_i32, 8_i32, i32),
            ("nested-br_table-value", -9_i32, 8_i32, i32),
            ("nested-br_table-value", 999999_i32, 8_i32, i32),
            ("nested-br_table-value-index", 0_i32, 9_i32, i32),
            ("nested-br_table-value-index", 1_i32, 8_i32, i32),
            ("nested-br_table-value-index", 2_i32, 9_i32, i32),
            ("nested-br_table-value-index", 3_i32, 9_i32, i32),
            ("nested-br_table-value-index", -1000000_i32, 9_i32, i32),
            ("nested-br_table-value-index", 9423975_i32, 9_i32, i32),
            ("nested-br_table-loop-block", 1_i32, 3_i32, i32),
        ]
    );
    Ok(())
}

#[test]
fn test_wasm_bulk() -> Result<()> {
    let code = include_bytes!("../../../dora-compiler/src/wasm/tests/suites/bulk.wat");
    build_wasm_code!(code, artifact);
    generate_test_cases!(
        &artifact,
        [
            // Basic fill test.
            ("fill", (1_i32, 3_i32), (), ()),
            ("load8_u", 0, 0, i32),
            ("load8_u", 1, 0xFF, i32),
            ("load8_u", 2, 0xFF, i32),
            ("load8_u", 3, 0xFF, i32),
            ("load8_u", 4, 0, i32),
            // Fill all of memory
            ("fill_all", (), (), ()),
            // Succeed when writing 0 bytes at the end of the region.
            ("fill_end_of_memory", (), (), ()),
        ]
    );
    Ok(())
}

#[test]
fn test_wasm_call() -> Result<()> {
    let code = include_bytes!("../../../dora-compiler/src/wasm/tests/suites/call.wat");
    build_wasm_code!(code, artifact);
    generate_test_cases!(
        &artifact,
        [
            ("type-i32", 0x132, (), ()),
            ("type-i64", 0x164_i64, (), ()),
            ("type-f32", 0xf32_f32, (), ()),
            ("type-f64", 0xf64_f64, (), ()),
            ("type-i32-i64", (0x132, 0x164), (), ()),
            ("type-first-i32", 32, (), ()),
            ("type-first-i64", 64, (), ()),
            ("type-first-f32", 1.32_f32, (), ()),
            ("type-first-f64", 1.64_f64, (), ()),
            ("type-second-i32", 32, (), ()),
            ("type-second-i64", 64, (), ()),
            ("type-second-f32", 32_f32, (), ()),
            ("type-second-f64", 64.1_f64, (), ()),
            ("type-all-i32-f64", (32, 1.64_f64), (), ()),
            ("type-all-i32-i32", (2, 1), (), ()),
            ("type-all-f32-f64", (2_f32, 1_f64), (), ()),
            ("type-all-f64-i32", (1_i32, 1.64_f64), (), ()),
            ("as-binary-all-operands", 7, (), ()),
            ("as-mixed-operands", 32, (), ()),
            ("as-call-all-operands", (3, 4), (), ()),
            ("fac", 0_i64, 1_i64, i64),
            ("fac", 1_i64, 1_i64, i64),
            ("fac", 5_i64, 120_i64, i64),
            ("fac", 25_i64, 7034535277573963776_i64, i64),
            ("fac-acc", (0_i64, 1_i64), 1_i64, i64),
            ("fac-acc", (1_i64, 1_i64), 1_i64, i64),
            ("fac-acc", (5_i64, 1_i64), 120_i64, i64),
            ("fac-acc", (25_i64, 1_i64), 7034535277573963776_i64, i64),
            ("fib", 0_i64, 1_i64, i64),
            ("fib", 1_i64, 1_i64, i64),
            ("fib", 2_i64, 2_i64, i64),
            ("fib", 5_i64, 8_i64, i64),
            ("fib", 20_i64, 10946_i64, i64),
            ("even", 0_i64, 44_i64, i64),
            ("even", 1_i64, 99_i64, i64),
            ("even", 100_i64, 44_i64, i64),
            ("even", 77_i64, 99_i64, i64),
            ("odd", 0_i64, 99_i64, i64),
            ("odd", 1_i64, 44_i64, i64),
            ("odd", 200_i64, 99_i64, i64),
            ("odd", 77_i64, 44_i64, i64),
            ("as-select-first", 0x132, (), ()),
            ("as-select-mid", 2, (), ()),
            ("as-select-last", 2, (), ()),
            ("as-if-condition", 1, (), ()),
            ("as-br_if-first", 0x132, (), ()),
            ("as-br_if-last", 2, (), ()),
            ("as-br_table-first", 0x132, (), ()),
            ("as-br_table-last", 2, (), ()),
            ("as-store-first", (), (), ()),
            ("as-store-last", (), (), ()),
            ("as-memory.grow-value", 1, (), ()),
            ("as-return-value", 0x132, (), ()),
            ("as-drop-operand", (), (), ()),
            ("as-br-value", 0x132, (), ()),
            ("as-local.set-value", 0x132, (), ()),
            ("as-local.tee-value", 0x132, (), ()),
            ("as-global.set-value", 0x132, (), ()),
            ("as-load-operand", 1, (), ()),
            ("as-unary-operand", 0x0_f32, (), ()),
            ("as-binary-left", 11_i32, (), ()),
            ("as-binary-right", 9_i32, (), ()),
            ("as-test-operand", 0_i32, (), ()),
            ("as-compare-left", 1_i32, (), ()),
            ("as-compare-right", 1_i32, (), ()),
            ("as-convert-operand", 1_i32, (), ()),
            ("return-from-long-argument-list", 42_i32, 42_i32, i32),
        ]
    );
    Ok(())
}

#[test]
fn test_wasm_call_indirect() -> Result<()> {
    let code = include_bytes!("../../../dora-compiler/src/wasm/tests/suites/call_indirect.wat");
    build_wasm_code!(code, artifact);
    generate_test_cases!(
        &artifact,
        [
            ("type-i32", (), 0x132, i32),
            // TODO: call indirect error deal.
            // ("type-i64", (), 0x164, i64),
            // ("type-f32", (), 0xf32 as f32, f32),
            // ("type-f64", (), 0xf64 as f64, f64),
            // ("type-index", (), 100, i64),
        ]
    );
    Ok(())
}

#[test]
fn test_wasm_conversions() -> Result<()> {
    let code = include_bytes!("../../../dora-compiler/src/wasm/tests/suites/conversions.wat");
    build_wasm_code!(code, artifact);
    generate_test_cases!(
        &artifact,
        [
            ("i64.extend_i32_s", 0_i32, 0_i64, i64),
            ("i64.extend_i32_s", 10000_i32, 10000_i64, i64),
            ("i64.extend_i32_s", -10000_i32, -10000_i64, i64),
            ("i64.extend_i32_s", -1_i32, -1_i64, i64),
            (
                "i64.extend_i32_s",
                0x7fffffff_i32,
                0x000000007fffffff_u64 as i64,
                i64
            ),
            (
                "i64.extend_i32_s",
                0x80000000_u32 as i32,
                0xffffffff80000000_u64 as i64,
                i64
            ),
            ("i64.extend_i32_u", 0_i32, 0_i64, i64),
            ("i64.extend_i32_u", 10000_i32, 10000_i64, i64),
            ("i64.extend_i32_u", -10000_i32, 0x00000000ffffd8f0_i64, i64),
            ("i64.extend_i32_u", -1_i32, 0xffffffff_i64, i64),
            (
                "i64.extend_i32_u",
                0x7fffffff_i32,
                0x000000007fffffff_u64 as i64,
                i64
            ),
            (
                "i64.extend_i32_u",
                0x80000000_u32 as i32,
                0x0000000080000000_u64 as i64,
                i64
            ),
            ("i32.wrap_i64", -1_i64, -1, i32),
            ("i32.wrap_i64", -100000_i64, -100000, i32),
            ("i32.wrap_i64", 0x80000000_i64, 0x80000000_u32 as i32, i32),
            (
                "i32.wrap_i64",
                0xffffffff7fffffff_u64 as i64,
                0x7fffffff,
                i32
            ),
            (
                "i32.wrap_i64",
                0xffffffff00000000_u64 as i64,
                0x00000000,
                i32
            ),
            (
                "i32.wrap_i64",
                0xfffffffeffffffff_u64 as i64,
                0xffffffff_u32 as i32,
                i32
            ),
            (
                "i32.wrap_i64",
                0xffffffff00000001_u64 as i64,
                0x00000001,
                i32
            ),
            ("i32.wrap_i64", 0_i64, 0, i32),
            (
                "i32.wrap_i64",
                1311768467463790320_i64,
                0x9abcdef0_u32 as i32,
                i32
            ),
            (
                "i32.wrap_i64",
                0x00000000ffffffff_i64,
                0xffffffff_u32 as i32,
                i32
            ),
            ("i32.wrap_i64", 0x0000000100000000_i64, 0x00000000, i32),
            ("i32.wrap_i64", 0x0000000100000001_i64, 0x00000001, i32),
            // TODO: add more tests for conversions.
        ]
    );
    Ok(())
}

#[test]
fn test_wasm_endiannes() -> Result<()> {
    let code = include_bytes!("../../../dora-compiler/src/wasm/tests/suites/endianness.wat");
    build_wasm_code!(code, artifact);
    generate_test_cases!(
        &artifact,
        [
            ("i32_load16_s", -1_i32, -1_i32, i32),
            ("i32_load16_s", -4242_i32, -4242_i32, i32),
            ("i32_load16_s", 42_i32, 42_i32, i32),
            ("i32_load16_s", 0x3210_i32, 0x3210_i32, i32),
            ("i32_load16_u", -1_i32, 0xFFFF_i32, i32),
            ("i32_load16_u", -4242_i32, 61294_i32, i32),
            ("i32_load16_u", 42_i32, 42_i32, i32),
            ("i32_load16_u", 0xCAFE_i32, 0xCAFE_i32, i32),
            ("i32_load", -1_i32, -1_i32, i32),
            ("i32_load", -42424242_i32, -42424242_i32, i32),
            ("i32_load", 42424242_i32, 42424242_i32, i32),
            (
                "i32_load",
                0xABAD1DEA_u32 as i32,
                0xABAD1DEA_u32 as i32,
                i32
            ),
            ("i64_load16_s", -1_i64, -1_i64, i64),
            ("i64_load16_s", -4242_i64, -4242_i64, i64),
            ("i64_load16_s", 42_i64, 42_i64, i64),
            ("i64_load16_s", 0x3210_i64, 0x3210_i64, i64),
            ("i64_load16_u", -1_i64, 0xFFFF_i64, i64),
            ("i64_load16_u", -4242_i64, 61294_i64, i64),
            ("i64_load16_u", 42_i64, 42_i64, i64),
            ("i64_load16_u", 0xCAFE_i64, 0xCAFE_i64, i64),
            ("i64_load32_s", -1_i64, -1_i64, i64),
            ("i64_load32_s", -42424242_i64, -42424242_i64, i64),
            ("i64_load32_s", 42424242_i64, 42424242_i64, i64),
            ("i64_load32_s", 0x12345678_i64, 0x12345678_i64, i64),
            ("i64_load32_u", -1_i64, 0xFFFFFFFF_u64 as i64, i64),
            ("i64_load32_u", -42424242_i64, 4252543054_i64, i64),
            ("i64_load32_u", 42424242_i64, 42424242_i64, i64),
            ("i64_load32_u", 0xABAD1DEA_i64, 0xABAD1DEA_i64, i64),
            ("i64_load", -1_i64, -1_i64, i64),
            ("i64_load", -42424242_i64, -42424242_i64, i64),
            ("i64_load", 0xABAD1DEA_i64, 0xABAD1DEA_i64, i64),
            (
                "i64_load",
                0xABADCAFEDEAD1DEA_u64 as i64,
                0xABADCAFEDEAD1DEA_u64 as i64,
                i64
            ),
            ("f32_load", -1_f32, -1_f32, f32),
            ("f32_load", 1234e-5_f32, 1234e-5_f32, f32),
            ("f32_load", 4242.4242_f32, 4242.4242_f32, f32),
            ("f64_load", -1_f64, -1_f64, f64),
            ("f64_load", 1234e-5_f64, 1234e-5_f64, f64),
            ("f64_load", 4242.4242_f64, 4242.4242_f64, f64),
            ("i32_store16", -1_i32, 0xFFFF_i32, i32),
            ("i32_store16", -4242_i32, 61294_i32, i32),
            ("i32_store16", 42_i32, 42_i32, i32),
            ("i32_store16", 0xCAFE_i32, 0xCAFE_i32, i32),
            ("i32_store", -1_i32, -1_i32, i32),
            ("i32_store", -4242_i32, -4242_i32, i32),
            ("i32_store", 42_i32, 42_i32, i32),
            ("i32_store", 0x3210_i32, 0x3210_i32, i32),
            ("i64_store16", -1_i64, 0xFFFF_i64, i64),
            ("i64_store16", -4242_i64, 61294_i64, i64),
            ("i64_store16", 42_i64, 42_i64, i64),
            ("i64_store16", 0xCAFE_i64, 0xCAFE_i64, i64),
            ("i64_store32", -1_i64, 0xFFFFFFFF_u64 as i64, i64),
            ("i64_store32", -42424242_i64, 4252543054_i64, i64),
            ("i64_store32", 42424242_i64, 42424242_i64, i64),
            ("i64_store32", 0xABAD1DEA_i64, 0xABAD1DEA_i64, i64),
            ("i64_store", -1_i64, -1_i64, i64),
            ("i64_store", -42424242_i64, -42424242_i64, i64),
            ("i64_store", 42424242_i64, 42424242_i64, i64),
            ("i64_store", 0x12345678_i64, 0x12345678_i64, i64),
            ("f32_store", -1_f32, -1_f32, f32),
            ("f32_store", 1234e-5_f32, 1234e-5_f32, f32),
            ("f32_store", 4242.4242_f32, 4242.4242_f32, f32),
            ("f64_store", -1_f64, -1_f64, f64),
            ("f64_store", 1234e-5_f64, 1234e-5_f64, f64),
            ("f64_store", 4242.4242_f64, 4242.4242_f64, f64),
        ]
    );
    Ok(())
}

#[test]
fn test_wasm_fac() -> Result<()> {
    let code = include_bytes!("../../../dora-compiler/src/wasm/tests/suites/fac.wat");
    build_wasm_code!(code, artifact);
    generate_test_cases!(
        &artifact,
        [
            ("fac-rec", 25_i64, 7034535277573963776_i64, i64),
            ("fac-iter", 25_i64, 7034535277573963776_i64, i64),
            ("fac-rec-named", 25_i64, 7034535277573963776_i64, i64),
            ("fac-iter-named", 25_i64, 7034535277573963776_i64, i64),
            ("fac-opt", 25_i64, 7034535277573963776_i64, i64),
            ("fac-ssa", 25_i64, 7034535277573963776_i64, i64),
        ]
    );
    Ok(())
}

#[test]
fn test_wasm_float_literals() -> Result<()> {
    let code = include_bytes!("../../../dora-compiler/src/wasm/tests/suites/float_literals.wat");
    build_wasm_code!(code, artifact);
    generate_test_cases!(
        &artifact,
        [
            ("f32.nan", (), 0x7fc00000, i32),
            ("f32.positive_nan", (), 0x7fc00000, i32),
            ("f32.negative_nan", (), 0xffc00000_u32 as i32, i32),
            ("f32.plain_nan", (), 0x7fc00000, i32),
            ("f32.informally_known_as_plain_snan", (), 0x7fa00000, i32),
            ("f32.all_ones_nan", (), 0xffffffff_u32 as i32, i32),
            ("f32.misc_nan", (), 0x7f812345, i32),
            ("f32.misc_positive_nan", (), 0x7fb04050, i32),
            ("f32.misc_negative_nan", (), 0xffaabcde_u32 as i32, i32),
            ("f32.infinity", (), 0x7f800000, i32),
            ("f32.positive_infinity", (), 0x7f800000, i32),
            ("f32.negative_infinity", (), 0xff800000_u32 as i32, i32),
            ("f32.zero", (), 0, i32),
            ("f32.positive_zero", (), 0, i32),
            ("f32.negative_zero", (), 0x80000000_u32 as i32, i32),
            ("f32.misc", (), 0x40c90fdb, i32),
            ("f32.min_positive", (), 1, i32),
            ("f32.min_normal", (), 0x800000, i32),
            ("f32.max_subnormal", (), 0x7fffff, i32),
            ("f32.max_finite", (), 0x7f7fffff, i32),
            ("f32.trailing_dot", (), 0x44800000, i32),
            ("f32.misc_int", (), 0x4791a280, i32),
            ("f32.large_int", (), 0x67800000_u32 as i32, i32),
            ("f32.min_int32", (), 0xcf000000_u32 as i32, i32),
            ("f32.min_int64", (), 0xdf000000_u32 as i32, i32),
            ("f32_dec.zero", (), 0, i32),
            ("f32_dec.positive_zero", (), 0, i32),
            ("f32_dec.negative_zero", (), 0x80000000_u32 as i32, i32),
            ("f32_dec.misc", (), 0x40c90fdb, i32),
            ("f32_dec.min_positive", (), 1, i32),
            ("f32_dec.min_normal", (), 0x800000, i32),
            ("f32_dec.max_subnormal", (), 0x7fffff, i32),
            ("f32_dec.max_finite", (), 0x7f7fffff, i32),
            ("f32_dec.trailing_dot", (), 0x501502f9, i32),
            ("f32_dec.root_beer_float", (), 0x3f800001, i32),
            ("f32_dec.misc_int", (), 0x4640e400, i32),
            ("f32_dec.large_int", (), 0x60ad78ec, i32),
            ("f32_dec.min_int32", (), 0xcf000000_u32 as i32, i32),
            ("f32_dec.min_int64", (), 0xdf000000_u32 as i32, i32),
            ("f64.nan", (), 0x7ff8000000000000, i64),
            ("f64.positive_nan", (), 0x7ff8000000000000, i64),
            ("f64.negative_nan", (), 0xfff8000000000000_u64 as i64, i64),
            ("f64.plain_nan", (), 0x7ff8000000000000, i64),
            (
                "f64.informally_known_as_plain_snan",
                (),
                0x7ff4000000000000,
                i64
            ),
            ("f64.all_ones_nan", (), 0xffffffffffffffff_u64 as i64, i64),
            ("f64.misc_nan", (), 0x7ff0123456789abc, i64),
            ("f64.misc_positive_nan", (), 0x7ff3040506070809, i64),
            (
                "f64.misc_negative_nan",
                (),
                0xfff2abcdef012345_u64 as i64,
                i64
            ),
            ("f64.infinity", (), 0x7ff0000000000000, i64),
            ("f64.positive_infinity", (), 0x7ff0000000000000, i64),
            (
                "f64.negative_infinity",
                (),
                0xfff0000000000000_u64 as i64,
                i64
            ),
            ("f64.zero", (), 0, i64),
            ("f64.positive_zero", (), 0, i64),
            ("f64.negative_zero", (), 0x8000000000000000_u64 as i64, i64),
            ("f64.misc", (), 0x401921fb54442d18, i64),
            ("f64.min_positive", (), 1, i64),
            ("f64.min_normal", (), 0x10000000000000, i64),
            ("f64.max_subnormal", (), 0xfffffffffffff, i64),
            ("f64.max_finite", (), 0x7fefffffffffffff, i64),
            ("f64.trailing_dot", (), 0x4630000000000000, i64),
            ("f64.misc_int", (), 0x40f2345000000000, i64),
            ("f64.large_int", (), 0x44f0000000000000, i64),
            ("f64.min_int32", (), 0xc1e0000000000000_u64 as i64, i64),
            ("f64.min_int64", (), 0xc3e0000000000000_u64 as i64, i64),
            ("f64_dec.zero", (), 0, i64),
            ("f64_dec.positive_zero", (), 0, i64),
            (
                "f64_dec.negative_zero",
                (),
                0x8000000000000000_u64 as i64,
                i64
            ),
            ("f64_dec.misc", (), 0x401921fb54442d18, i64),
            ("f64_dec.min_positive", (), 1, i64),
            ("f64_dec.min_normal", (), 0x10000000000000, i64),
            ("f64_dec.max_subnormal", (), 0xfffffffffffff, i64),
            ("f64_dec.max_finite", (), 0x7fefffffffffffff, i64),
            ("f64_dec.trailing_dot", (), 0x54b249ad2594c37d, i64),
            ("f64_dec.root_beer_float", (), 0x3ff000001ff19e24, i64),
            ("f64_dec.misc_int", (), 0x40c81c8000000000, i64),
            ("f64_dec.large_int", (), 0x4415af1d78b58c40, i64),
            ("f64_dec.min_int32", (), 0xc1e0000000000000_u64 as i64, i64),
            ("f64_dec.min_int64", (), 0xc3e0000000000000_u64 as i64, i64),
            ("f32-dec-sep1", (), 1000000_f32, f32),
            ("f32-dec-sep2", (), 1000_f32, f32),
            ("f32-dec-sep3", (), 1_003.141_6, f32),
            ("f32-dec-sep4", (), 99e+13_f32, f32),
            ("f32-dec-sep5", (), 1.220_001_2e28, f32),
            ("f32-hex-sep1", (), 0xa0f0099 as f32, f32),
            ("f32-hex-sep2", (), 0x1aa0f as f32, f32),
            ("f64-dec-sep1", (), 1000000_f64, f64),
            ("f64-dec-sep2", (), 1000_f64, f64),
            ("f64-dec-sep3", (), 1003.141592, f64),
            ("f64-dec-sep4", (), 99e-123_f64, f64),
            ("f64-dec-sep5", (), 122000.11354e23, f64),
            ("f64-hex-sep1", (), 0xaf00f00009999_u64 as f64, f64),
            ("f64-hex-sep2", (), 0x1aa0f as f64, f64),
        ]
    );
    Ok(())
}

#[test]
fn test_wasm_float_memory() -> Result<()> {
    let code = include_bytes!("../../../dora-compiler/src/wasm/tests/suites/float_memory.wat");
    build_wasm_code!(code, artifact);
    generate_test_cases!(
        &artifact,
        [
            ("i64.load", (), 0x7ff4000000000000, i64),
            ("reset", (), (), ()),
            ("i64.load", (), 0x00, i64),
            ("f64.load", (), 0x00 as f64, f64),
        ]
    );
    Ok(())
}

#[test]
fn test_wasm_forward() -> Result<()> {
    let code = include_bytes!("../../../dora-compiler/src/wasm/tests/suites/forward.wat");
    build_wasm_code!(code, artifact);
    generate_test_cases!(
        &artifact,
        [
            ("even", 13, 0, i32),
            ("even", 20, 1, i32),
            ("odd", 13, 1, i32),
            ("odd", 20, 0, i32),
        ]
    );
    Ok(())
}

#[test]
fn test_wasm_func() -> Result<()> {
    let code = include_bytes!("../../../dora-compiler/src/wasm/tests/suites/func.wat");
    build_wasm_code!(code, artifact);
    generate_test_cases!(
        &artifact,
        [
            ("type-use-1", (), (), ()),
            ("type-use-2", (), 0, i32),
            ("type-use-3", 1, (), ()),
            ("type-use-4", (1, 1, 1), 0, i32),
            ("type-use-5", (), 0, i32),
            ("type-use-6", 1, (), ()),
            ("type-use-7", (1, 1, 1), 0, i32),
            ("local-first-i32", (), 0, i32),
            ("local-first-i64", (), 0, i64),
            ("local-first-f32", (), 0.0, f32),
            ("local-first-f64", (), 0.0, f64),
            ("local-second-i32", (), 0, i32),
            ("local-second-i64", (), 0, i64),
            ("local-second-f32", (), 0.0, f32),
            ("local-second-f64", (), 0.0, f64),
            ("local-mixed", (), 0.0, f64),
            ("param-first-i32", (2, 3), 2, i32),
            ("param-first-i64", (2, 3), 2, i64),
            ("param-first-f32", (2.0_f32, 3.0_f32), 2.0, f32),
            ("param-first-f64", (2.0, 3.0), 2.0, f64),
            ("param-second-i32", (2, 3), 3, i32),
            ("param-second-i64", (2, 3), 3, i64),
            ("param-second-f32", (2.0_f32, 3.0_f32), 3.0, f32),
            ("param-second-f64", (2.0, 3.0), 3.0, f64),
            // ("param-mixed", (1.0_f32, 2_i32, 3_i64, 4_i32, 5.5_f64, 6_i32), 5.5_f64, f64),
            ("empty", (), (), ()),
            ("value-void", (), (), ()),
            ("value-i32", (), 77, i32),
            ("value-i64", (), 7777, i64),
            ("value-f32", (), 77.7, f32),
            ("value-f64", (), 77.77, f64),
            ("value-i32-f64", (), (77_i32, 7.0_f64), (i32, f64)),
            // ("value-i32-i32-i32", (), (1_i32, 2_i32, 3_i32), (i32, i32, i32)),
            ("value-block-void", (), (), ()),
            ("value-block-i32", (), 77, i32),
            ("value-block-i32-i64", (), (1, 2), (i32, i64)),
            ("return-empty", (), (), ()),
            ("return-i32", (), 78, i32),
            ("return-i64", (), 7878, i64),
            ("return-f32", (), 78.7, f32),
            ("return-f64", (), 78.78, f64),
            ("return-i32-f64", (), (78, 78.78), (i32, f64)),
            // ("return-i32-i32-i32", (), (1, 2, 3), (i32, i32, i32)),
            ("return-block-i32", (), 77, i32),
            ("return-block-i32-i64", (), (1, 2), (i32, i64)),
            ("break-empty", (), (), ()),
            ("break-i32", (), 79, i32),
            ("break-i64", (), 7979, i64),
            ("break-f32", (), 79.9, f32),
            ("break-f64", (), 79.79, f64),
            ("break-i32-f64", (), (79, 79.79), (i32, f64)),
            // ("break-i32-i32-i32", (), (1, 2, 3), (i32, i32, i32)),
            ("break-block-i32", (), 77, i32),
            ("break-block-i32-i64", (), (1, 2), (i32, i64)),
            ("break-br_if-empty", (0,), (), ()),
            ("break-br_if-empty", (2,), (), ()),
            ("break-br_if-num", (0,), 51, i32),
            ("break-br_if-num", (1,), 50, i32),
            ("break-br_if-num-num", (0,), (51, 52), (i32, i64)),
            ("break-br_if-num-num", (1,), (50, 51), (i32, i64)),
            ("break-br_table-empty", (0,), (), ()),
            ("break-br_table-empty", (1,), (), ()),
            ("break-br_table-empty", (5,), (), ()),
            ("break-br_table-empty", (-1,), (), ()),
            ("break-br_table-num", (0,), 50, i32),
            ("break-br_table-num", (1,), 50, i32),
            ("break-br_table-num", (10,), 50, i32),
            ("break-br_table-num", (-100,), 50, i32),
            ("break-br_table-num-num", (0,), (50, 51), (i32, i64)),
            ("break-br_table-num-num", (1,), (50, 51), (i32, i64)),
            ("break-br_table-num-num", (10,), (50, 51), (i32, i64)),
            ("break-br_table-num-num", (-100,), (50, 51), (i32, i64)),
            ("break-br_table-nested-empty", (0,), (), ()),
            ("break-br_table-nested-empty", (1,), (), ()),
            ("break-br_table-nested-empty", (3,), (), ()),
            ("break-br_table-nested-empty", (-2,), (), ()),
            ("break-br_table-nested-num", (0,), 52, i32),
            ("break-br_table-nested-num", (1,), 50, i32),
            ("break-br_table-nested-num", (2,), 52, i32),
            ("break-br_table-nested-num", (-3,), 52, i32),
            ("break-br_table-nested-num-num", (0,), (101, 52), (i32, i32)),
            ("break-br_table-nested-num-num", (1,), (50, 51), (i32, i32)),
            ("break-br_table-nested-num-num", (2,), (101, 52), (i32, i32)),
            (
                "break-br_table-nested-num-num",
                (-3,),
                (101, 52),
                (i32, i32)
            ),
            ("init-local-i32", (), 0, i32),
            ("init-local-i64", (), 0, i64),
            ("init-local-f32", (), 0.0, f32),
            ("init-local-f64", (), 0.0, f64),
        ]
    );
    Ok(())
}

#[test]
fn test_wasm_func_ptr() -> Result<()> {
    let code = include_bytes!("../../../dora-compiler/src/wasm/tests/suites/func_ptr.wat");
    build_wasm_code!(code, artifact);
    generate_test_cases!(
        &artifact,
        [
            ("one", (), 13, i32),
            ("two", 13, 14, i32),
            ("three", 13, 11, i32),
        ]
    );
    Ok(())
}

#[test]
fn test_wasm_global() -> Result<()> {
    let code = include_bytes!("../../../dora-compiler/src/wasm/tests/suites/global.wat");
    build_wasm_code!(code, artifact);
    generate_test_cases!(
        &artifact,
        [
            ("get-a", (), (-2,), (i32,)),
            ("get-b", (), (-5,), (i64,)),
            ("get-x", (), (-12,), (i32,)),
            ("get-y", (), (-15,), (i64,)),
            ("get-z1", (), (0,), (i32,)),
            ("get-z2", (), (1,), (i64,)),
            ("get-3", (), (-3.0,), (f32,)),
            ("get-4", (), (-4.0,), (f64,)),
            ("get-7", (), (-13.0,), (f32,)),
            ("get-8", (), (-14.0,), (f64,)),
            ("set-x", (6,), (), ()),
            ("set-y", (7,), (), ()),
            ("set-7", (8_f32,), (), ()),
            ("set-8", (9_f64,), (), ()),
            ("get-x", (), (6,), (i32,)),
            ("get-y", (), (7,), (i64,)),
            ("get-7", (), (8.0,), (f32,)),
            ("get-8", (), (9.0,), (f64,)),
            ("set-7", (8_f32,), (), ()),
            ("set-8", (9_f64,), (), ()),
            ("set-mr", (10,), (), ()),
            ("get-x", (), (6,), (i32,)),
            ("get-y", (), (7,), (i64,)),
            ("get-7", (), (8.0,), (f32,)),
            ("get-8", (), (9.0,), (f64,)),
            ("as-select-first", (), (6,), (i32,)),
            ("as-select-mid", (), (2,), (i32,)),
            ("as-select-last", (), (2,), (i32,)),
            ("as-loop-first", (), (6,), (i32,)),
            ("as-loop-mid", (), (6,), (i32,)),
            ("as-loop-last", (), (6,), (i32,)),
            ("as-if-condition", (), (2,), (i32,)),
            ("as-if-then", (), (6,), (i32,)),
            ("as-if-else", (), (6,), (i32,)),
            ("as-br_if-first", (), (6,), (i32,)),
            ("as-br_if-last", (), (2,), (i32,)),
            ("as-br_table-first", (), (6,), (i32,)),
            ("as-br_table-last", (), (2,), (i32,)),
            // TODO: test errors on macos.
            // ("as-call_indirect-first", (), (6,), (i32,)),
            // ("as-call_indirect-mid", (), (2,), (i32,)),
            ("as-store-first", (), (), ()),
            ("as-store-last", (), (), ()),
            ("as-load-operand", (1,), (), ()),
            ("as-memory.grow-value", (1,), (), ()),
            ("as-call-value", (), (6,), (i32,)),
            ("as-return-value", (), (6,), (i32,)),
            ("as-drop-operand", (), (), ()),
            ("as-br-value", (), (6,), (i32,)),
            ("as-local.set-value", (1,), (6,), (i32,)),
            ("as-local.tee-value", (1,), (6,), (i32,)),
            ("as-global.set-value", (6,), (), ()),
            ("as-unary-operand", (), (0,), (i32,)),
            ("as-binary-operand", (), (36,), (i32,)),
        ]
    );
    Ok(())
}

#[test]
fn test_wasm_i32_arith() -> Result<()> {
    let code = include_bytes!("../../../dora-compiler/src/wasm/tests/suites/i32_arith.wat");
    build_wasm_code!(code, artifact);
    generate_test_cases!(
        &artifact,
        [
            ("add", (1, 1), 2, i32),
            ("add", (1, 0), 1, i32),
            ("add", (-1, -1), -2, i32),
            ("add", (-1, 1), 0, i32),
            ("add", (0x7fffffff, 1), 0x80000000_u32 as i32, i32),
            ("add", (0x80000000_u32 as i32, -1), 0x7fffffff, i32),
            (
                "add",
                (0x80000000_u32 as i32, 0x80000000_u32 as i32),
                0,
                i32
            ),
            ("add", (0x3fffffff, 1), 0x40000000, i32),
            ("sub", (1, 1), 0, i32),
            ("sub", (1, 0), 1, i32),
            ("sub", (-1, -1), 0, i32),
            ("sub", (0x7fffffff, -1), 0x80000000_u32 as i32, i32),
            ("sub", (0x80000000_u32 as i32, 1), 0x7fffffff, i32),
            (
                "sub",
                (0x80000000_u32 as i32, 0x80000000_u32 as i32),
                0,
                i32
            ),
            ("sub", (0x3fffffff, -1), 0x40000000, i32),
            ("mul", (1, 1), 1, i32),
            ("mul", (1, 0), 0, i32),
            ("mul", (-1, -1), 1, i32),
            ("mul", (0x10000000, 4096), 0, i32),
            ("mul", (0x80000000_u32 as i32, 0), 0, i32),
            (
                "mul",
                (0x80000000_u32 as i32, -1),
                0x80000000_u32 as i32,
                i32
            ),
            ("mul", (0x7fffffff, -1), 0x80000001_u32 as i32, i32),
            ("mul", (0x01234567, 0x76543210), 0x358e7470, i32),
            ("mul", (0x7fffffff, 0x7fffffff), 1, i32),
            ("div_s", (1, 1), 1, i32),
            ("div_s", (0, 1), 0, i32),
            ("div_s", (0, -1), 0, i32),
            ("div_s", (-1, -1), 1, i32),
            (
                "div_s",
                (0x80000000_u32 as i32, 2),
                0xc0000000_u32 as i32,
                i32
            ),
            (
                "div_s",
                (0x80000001_u32 as i32, 1000),
                0xffdf3b65_u32 as i32,
                i32
            ),
            ("div_s", (5, 2), 2, i32),
            ("div_s", (-5, 2), -2, i32),
            ("div_s", (5, -2), -2, i32),
            ("div_s", (-5, -2), 2, i32),
            ("div_s", (7, 3), 2, i32),
            ("div_s", (-7, 3), -2, i32),
            ("div_s", (7, -3), -2, i32),
            ("div_s", (-7, -3), 2, i32),
            ("div_s", (11, 5), 2, i32),
            ("div_s", (17, 7), 2, i32),
            ("div_u", (1, 1), 1, i32),
            ("div_u", (0, 1), 0, i32),
            ("div_u", (-1, -1), 1, i32),
            ("div_u", (0x80000000_u32 as i32, -1), 0, i32),
            ("div_u", (0x80000000_u32 as i32, 2), 0x40000000, i32),
            ("div_u", (0x8ff00ff0_u32 as i32, 0x10001), 0x8fef, i32),
            ("div_u", (0x80000001_u32 as i32, 1000), 0x20c49b, i32),
            ("div_u", (5, 2), 2, i32),
            ("div_u", (-5, 2), 0x7ffffffd, i32),
            ("div_u", (5, -2), 0, i32),
            ("div_u", (-5, -2), 0, i32),
            ("div_u", (7, 3), 2, i32),
            ("div_u", (11, 5), 2, i32),
            ("div_u", (17, 7), 2, i32),
            ("rem_s", (0x7fffffff, -1), 0, i32),
            ("rem_s", (1, 1), 0, i32),
            ("rem_s", (0, 1), 0, i32),
            ("rem_s", (0, -1), 0, i32),
            ("rem_s", (-1, -1), 0, i32),
            // ("rem_s", (0x80000000_u32 as i32, -1), 0, i32),
            ("rem_s", (0x80000000_u32 as i32, 2), 0, i32),
            ("rem_s", (0x80000001_u32 as i32, 1000), -647, i32),
            ("rem_s", (5, 2), 1, i32),
            ("rem_s", (-5, 2), -1, i32),
            ("rem_s", (5, -2), 1, i32),
            ("rem_s", (-5, -2), -1, i32),
            ("rem_s", (7, 3), 1, i32),
            ("rem_s", (-7, 3), -1, i32),
            ("rem_s", (7, -3), 1, i32),
            ("rem_s", (-7, -3), -1, i32),
            ("rem_s", (11, 5), 1, i32),
            ("rem_s", (17, 7), 3, i32),
            ("rem_u", (1, 1), 0, i32),
            ("rem_u", (0, 1), 0, i32),
            ("rem_u", (-1, -1), 0, i32),
            (
                "rem_u",
                (0x80000000_u32 as i32, -1),
                0x80000000_u32 as i32,
                i32
            ),
            ("rem_u", (0x80000000_u32 as i32, 2), 0, i32),
            ("rem_u", (0x8ff00ff0_u32 as i32, 0x10001), 0x8001, i32),
            ("rem_u", (0x80000001_u32 as i32, 1000), 649, i32),
            ("rem_u", (5, 2), 1, i32),
            ("rem_u", (-5, 2), 1, i32),
            ("rem_u", (5, -2), 5, i32),
            ("rem_u", (-5, -2), -5, i32),
            ("rem_u", (7, 3), 1, i32),
            ("rem_u", (11, 5), 1, i32),
            ("rem_u", (17, 7), 3, i32),
            ("and", (1, 0), 0, i32),
            ("and", (0, 1), 0, i32),
            ("and", (1, 1), 1, i32),
            ("and", (0, 0), 0, i32),
            ("and", (0x7fffffff, 0x80000000_u32 as i32), 0, i32),
            ("and", (0x7fffffff, -1), 0x7fffffff, i32),
            (
                "and",
                (0xf0f0ffff_u32 as i32, 0xfffff0f0_u32 as i32),
                0xf0f0f0f0_u32 as i32,
                i32
            ),
            (
                "and",
                (0xffffffff_u32 as i32, 0xffffffff_u32 as i32),
                0xffffffff_u32 as i32,
                i32
            ),
            ("or", (1, 0), 1, i32),
            ("or", (0, 1), 1, i32),
            ("or", (1, 1), 1, i32),
            ("or", (0, 0), 0, i32),
            ("or", (0x7fffffff, 0x80000000_u32 as i32), -1, i32),
            ("or", (0x80000000_u32 as i32, 0), 0x80000000_u32 as i32, i32),
            (
                "or",
                (0xf0f0ffff_u32 as i32, 0xfffff0f0_u32 as i32),
                0xffffffff_u32 as i32,
                i32
            ),
            (
                "or",
                (0xffffffff_u32 as i32, 0xffffffff_u32 as i32),
                0xffffffff_u32 as i32,
                i32
            ),
            ("xor", (1, 0), 1, i32),
            ("xor", (0, 1), 1, i32),
            ("xor", (1, 1), 0, i32),
            ("xor", (0, 0), 0, i32),
            (
                "xor",
                (0x7fffffff_u32 as i32, 0x80000000_u32 as i32),
                -1,
                i32
            ),
            (
                "xor",
                (0x80000000_u32 as i32, 0),
                0x80000000_u32 as i32,
                i32
            ),
            ("xor", (-1, 0x80000000_u32 as i32), 0x7fffffff, i32),
            ("xor", (-1, 0x7fffffff), 0x80000000_u32 as i32, i32),
            (
                "xor",
                (0xf0f0ffff_u32 as i32, 0xfffff0f0_u32 as i32),
                0x0f0f0f0f_u32 as i32,
                i32
            ),
            (
                "xor",
                (0xffffffff_u32 as i32, 0xffffffff_u32 as i32),
                0,
                i32
            ),
            ("shl", (1, 1), 2, i32),
            ("shl", (1, 0), 1, i32),
            (
                "shl",
                (0x7fffffff_u32 as i32, 1),
                0xfffffffe_u32 as i32,
                i32
            ),
            (
                "shl",
                (0xffffffff_u32 as i32, 1),
                0xfffffffe_u32 as i32,
                i32
            ),
            ("shl", (0x80000000_u32 as i32, 1), 0, i32),
            ("shl", (0x40000000, 1), 0x80000000_u32 as i32, i32),
            ("shl", (1, 31), 0x80000000_u32 as i32, i32),
            // Note: The result of shift overflow is 0
            ("shl", (1, 32), 0, i32),
            ("shl", (1, 33), 0, i32),
            ("shl", (1, -1), 0, i32),
            ("shl", (1, 0x7fffffff), 0, i32),
            ("shr_s", (1, 1), 0, i32),
            ("shr_s", (1, 0), 1, i32),
            ("shr_s", (-1, 1), -1, i32),
            ("shr_s", (0x7fffffff, 1), 0x3fffffff, i32),
            (
                "shr_s",
                (0x80000000_u32 as i32, 1),
                0xc0000000_u32 as i32,
                i32
            ),
            ("shr_s", (0x40000000, 1), 0x20000000, i32),
            // Note: The result of shift overflow is 0
            ("shr_s", (1, 32), 0, i32),
            ("shr_s", (1, 33), 0, i32),
            ("shr_s", (1, -1), 0, i32),
            ("shr_s", (1, 0x7fffffff), 0, i32),
            ("shr_s", (1, 0x80000000_u32 as i32), 0, i32),
            ("shr_s", (0x80000000_u32 as i32, 31), -1, i32),
            ("shr_s", (-1, 32), -1, i32),
            ("shr_s", (-1, 33), -1, i32),
            ("shr_s", (-1, -1), -1, i32),
            ("shr_s", (-1, 0x7fffffff_u32 as i32), -1, i32),
            ("shr_s", (-1, 0x80000000_u32 as i32), -1, i32),
            ("shr_u", (1, 1), 0, i32),
            ("shr_u", (1, 0), 1, i32),
            ("shr_u", (-1, 1), 0x7fffffff_u32 as i32, i32),
            ("shr_u", (0x7fffffff, 1), 0x3fffffff, i32),
            ("shr_u", (0x80000000_u32 as i32, 1), 0x40000000, i32),
            ("shr_u", (0x40000000, 1), 0x20000000, i32),
            // Note: The result of shift overflow is 0
            ("shr_u", (1, 32), 0, i32),
            ("shr_u", (1, 33), 0, i32),
            ("shr_u", (1, -1), 0, i32),
            ("shr_u", (1, 0x7fffffff), 0, i32),
            ("shr_u", (1, 0x80000000_u32 as i32), 0, i32),
            ("shr_u", (0x80000000_u32 as i32, 31), 1, i32),
            // Note: The result of shift overflow is 0
            ("shr_u", (-1, 32), 0, i32),
            ("shr_u", (-1, 33), 0, i32),
            ("shr_u", (-1, -1), 0, i32),
            ("shr_u", (-1, 0x7fffffff), 0, i32),
            ("shr_u", (-1, 0x80000000_u32 as i32), 0, i32),
            ("rotl", (1, 1), 2, i32),
            ("rotl", (1, 0), 1, i32),
            ("rotl", (-1, 1), -1, i32),
            ("rotl", (1, 32), 1, i32),
            (
                "rotl",
                (0xabcd9876_u32 as i32, 1),
                0x579b30ed_u32 as i32,
                i32
            ),
            (
                "rotl",
                (0xfe00dc00_u32 as i32, 4),
                0xe00dc00f_u32 as i32,
                i32
            ),
            (
                "rotl",
                (0xb0c1d2e3_u32 as i32, 5),
                0x183a5c76_u32 as i32,
                i32
            ),
            (
                "rotl",
                (0x00008000_u32 as i32, 37),
                0x00100000_u32 as i32,
                i32
            ),
            ("rotl", (0xb0c1d2e3_u32 as i32, 0xff05), 0x183a5c76, i32),
            ("rotl", (0x769abcdf, 0xffffffed_u32 as i32), 0x579beed3, i32),
            ("rotl", (0x769abcdf, 0x8000000d_u32 as i32), 0x579beed3, i32),
            ("rotl", (1, 31), 0x80000000_u32 as i32, i32),
            ("rotl", (0x80000000_u32 as i32, 1), 1, i32),
            ("rotr", (1, 1), 0x80000000_u32 as i32, i32),
            ("rotr", (1, 0), 1, i32),
            ("rotr", (-1, 1), -1, i32),
            ("rotr", (1, 32), 1, i32),
            (
                "rotr",
                (0xff00cc00_u32 as i32, 1),
                0x7f806600_u32 as i32,
                i32
            ),
            (
                "rotr",
                (0x00080000_u32 as i32, 4),
                0x00008000_u32 as i32,
                i32
            ),
            (
                "rotr",
                (0xb0c1d2e3_u32 as i32, 5),
                0x1d860e97_u32 as i32,
                i32
            ),
            (
                "rotr",
                (0x00008000_u32 as i32, 37),
                0x00000400_u32 as i32,
                i32
            ),
            (
                "rotr",
                (0xb0c1d2e3_u32 as i32, 0xff05),
                0x1d860e97_u32 as i32,
                i32
            ),
            (
                "rotr",
                (0x769abcdf_u32 as i32, 0xffffffed_u32 as i32),
                0xe6fbb4d5_u32 as i32,
                i32
            ),
            (
                "rotr",
                (0x769abcdf_u32 as i32, 0x8000000d_u32 as i32),
                0xe6fbb4d5_u32 as i32,
                i32
            ),
            ("rotr", (1, 31), 2, i32),
            ("rotr", (0x80000000_u32 as i32, 31), 1, i32),
            ("clz", (0xffffffff_u32 as i32,), 0, i32),
            ("clz", (0,), 32, i32),
            ("clz", (0x00008000_u32 as i32,), 16, i32),
            ("clz", (0xff,), 24, i32),
            ("clz", (0x80000000_u32 as i32,), 0, i32),
            ("clz", (1,), 31, i32),
            ("clz", (2,), 30, i32),
            ("clz", (0x7fffffff_u32 as i32,), 1, i32),
            ("ctz", (-1,), 0, i32),
            ("ctz", (0,), 32, i32),
            ("ctz", (0x00008000_u32 as i32,), 15, i32),
            ("ctz", (0x00010000_u32 as i32,), 16, i32),
            ("ctz", (0x80000000_u32 as i32,), 31, i32),
            ("ctz", (0x7fffffff_u32 as i32,), 0, i32),
            ("popcnt", (-1,), 32, i32),
            ("popcnt", (0,), 0, i32),
            ("popcnt", (0x00008000_u32 as i32,), 1, i32),
            ("popcnt", (0x80008000_u32 as i32,), 2, i32),
            ("popcnt", (0x7fffffff_u32 as i32,), 31, i32),
            ("popcnt", (0xaaaaaaaa_u32 as i32,), 16, i32),
            ("popcnt", (0x55555555_u32 as i32,), 16, i32),
            ("popcnt", (0xdeadbeef_u32 as i32,), 24, i32),
            ("extend8_s", (0,), 0, i32),
            ("extend8_s", (0x7f,), 127, i32),
            ("extend8_s", (0x80,), -128, i32),
            ("extend8_s", (0xff,), -1, i32),
            ("extend8_s", (0x01234500_u32 as i32,), 0, i32),
            ("extend8_s", (0xfedcba80_u32 as i32,), -128, i32),
            ("extend8_s", (-1,), -1, i32),
            ("extend16_s", (0,), 0, i32),
            ("extend16_s", (0x7fff,), 32767, i32),
            ("extend16_s", (0x8000,), -32768, i32),
            ("extend16_s", (0xffff,), -1, i32),
            ("extend16_s", (0x01230000_u32 as i32,), 0, i32),
            ("extend16_s", (0xfedc8000_u32 as i32,), -32768, i32),
            ("extend16_s", (-1,), -1, i32),
            ("eqz", (0,), 1, i32),
            ("eqz", (1,), 0, i32),
            ("eqz", (0x80000000_u32 as i32,), 0, i32),
            ("eqz", (0x7fffffff_u32 as i32,), 0, i32),
            ("eqz", (0xffffffff_u32 as i32,), 0, i32),
            ("eq", (0, 0), 1, i32),
            ("eq", (1, 1), 1, i32),
            ("eq", (-1, 1), 0, i32),
            ("eq", (0x80000000_u32 as i32, 0x80000000_u32 as i32), 1, i32),
            ("eq", (0x7fffffff_u32 as i32, 0x7fffffff_u32 as i32), 1, i32),
            ("eq", (-1, -1), 1, i32),
            ("eq", (1, 0), 0, i32),
            ("eq", (0, 1), 0, i32),
            ("eq", (0x80000000_u32 as i32, 0), 0, i32),
            ("eq", (0, 0x80000000_u32 as i32), 0, i32),
            ("eq", (0x80000000_u32 as i32, -1), 0, i32),
            ("eq", (-1, 0x80000000_u32 as i32), 0, i32),
            ("eq", (0x80000000_u32 as i32, 0x7fffffff_u32 as i32), 0, i32),
            ("eq", (0x7fffffff_u32 as i32, 0x80000000_u32 as i32), 0, i32),
            ("ne", (0, 0), 0, i32),
            ("ne", (1, 1), 0, i32),
            ("ne", (-1, 1), 1, i32),
            ("ne", (0x80000000_u32 as i32, 0x80000000_u32 as i32), 0, i32),
            ("ne", (0x7fffffff_u32 as i32, 0x7fffffff_u32 as i32), 0, i32),
            ("ne", (-1, -1), 0, i32),
            ("ne", (1, 0), 1, i32),
            ("ne", (0, 1), 1, i32),
            ("ne", (0x80000000_u32 as i32, 0), 1, i32),
            ("ne", (0, 0x80000000_u32 as i32), 1, i32),
            ("ne", (0x80000000_u32 as i32, -1), 1, i32),
            ("ne", (-1, 0x80000000_u32 as i32), 1, i32),
            ("ne", (0x80000000_u32 as i32, 0x7fffffff_u32 as i32), 1, i32),
            ("ne", (0x7fffffff_u32 as i32, 0x80000000_u32 as i32), 1, i32),
            ("lt_s", (0, 0), 0, i32),
            ("lt_s", (1, 1), 0, i32),
            ("lt_s", (-1, 1), 1, i32),
            (
                "lt_s",
                (0x80000000_u32 as i32, 0x80000000_u32 as i32),
                0,
                i32
            ),
            (
                "lt_s",
                (0x7fffffff_u32 as i32, 0x7fffffff_u32 as i32),
                0,
                i32
            ),
            ("lt_s", (-1, -1), 0, i32),
            ("lt_s", (1, 0), 0, i32),
            ("lt_s", (0, 1), 1, i32),
            ("lt_s", (0x80000000_u32 as i32, 0), 1, i32),
            ("lt_s", (0, 0x80000000_u32 as i32), 0, i32),
            ("lt_s", (0x80000000_u32 as i32, -1), 1, i32),
            ("lt_s", (-1, 0x80000000_u32 as i32), 0, i32),
            (
                "lt_s",
                (0x80000000_u32 as i32, 0x7fffffff_u32 as i32),
                1,
                i32
            ),
            (
                "lt_s",
                (0x7fffffff_u32 as i32, 0x80000000_u32 as i32),
                0,
                i32
            ),
            ("lt_u", (0, 0), 0, i32),
            ("lt_u", (1, 1), 0, i32),
            ("lt_u", (-1, 1), 0, i32),
            (
                "lt_u",
                (0x80000000_u32 as i32, 0x80000000_u32 as i32),
                0,
                i32
            ),
            (
                "lt_u",
                (0x7fffffff_u32 as i32, 0x7fffffff_u32 as i32),
                0,
                i32
            ),
            ("lt_u", (-1, -1), 0, i32),
            ("lt_u", (1, 0), 0, i32),
            ("lt_u", (0, 1), 1, i32),
            ("lt_u", (0x80000000_u32 as i32, 0), 0, i32),
            ("lt_u", (0, 0x80000000_u32 as i32), 1, i32),
            ("lt_u", (0x80000000_u32 as i32, -1), 1, i32),
            ("lt_u", (-1, 0x80000000_u32 as i32), 0, i32),
            (
                "lt_u",
                (0x80000000_u32 as i32, 0x7fffffff_u32 as i32),
                0,
                i32
            ),
            (
                "lt_u",
                (0x7fffffff_u32 as i32, 0x80000000_u32 as i32),
                1,
                i32
            ),
            ("le_s", (0, 0), 1, i32),
            ("le_s", (1, 1), 1, i32),
            ("le_s", (-1, 1), 1, i32),
            (
                "le_s",
                (0x80000000_u32 as i32, 0x80000000_u32 as i32),
                1,
                i32
            ),
            (
                "le_s",
                (0x7fffffff_u32 as i32, 0x7fffffff_u32 as i32),
                1,
                i32
            ),
            ("le_s", (-1, -1), 1, i32),
            ("le_s", (1, 0), 0, i32),
            ("le_s", (0, 1), 1, i32),
            ("le_s", (0x80000000_u32 as i32, 0), 1, i32),
            ("le_s", (0, 0x80000000_u32 as i32), 0, i32),
            ("le_s", (0x80000000_u32 as i32, -1), 1, i32),
            ("le_s", (-1, 0x80000000_u32 as i32), 0, i32),
            (
                "le_s",
                (0x80000000_u32 as i32, 0x7fffffff_u32 as i32),
                1,
                i32
            ),
            (
                "le_s",
                (0x7fffffff_u32 as i32, 0x80000000_u32 as i32),
                0,
                i32
            ),
            ("le_u", (0, 0), 1, i32),
            ("le_u", (1, 1), 1, i32),
            ("le_u", (-1, 1), 0, i32),
            (
                "le_u",
                (0x80000000_u32 as i32, 0x80000000_u32 as i32),
                1,
                i32
            ),
            (
                "le_u",
                (0x7fffffff_u32 as i32, 0x7fffffff_u32 as i32),
                1,
                i32
            ),
            ("le_u", (-1, -1), 1, i32),
            ("le_u", (1, 0), 0, i32),
            ("le_u", (0, 1), 1, i32),
            ("le_u", (0x80000000_u32 as i32, 0), 0, i32),
            ("le_u", (0, 0x80000000_u32 as i32), 1, i32),
            ("le_u", (0x80000000_u32 as i32, -1), 1, i32),
            ("le_u", (-1, 0x80000000_u32 as i32), 0, i32),
            (
                "le_u",
                (0x80000000_u32 as i32, 0x7fffffff_u32 as i32),
                0,
                i32
            ),
            (
                "le_u",
                (0x7fffffff_u32 as i32, 0x80000000_u32 as i32),
                1,
                i32
            ),
            ("gt_s", (0, 0), 0, i32),
            ("gt_s", (1, 1), 0, i32),
            ("gt_s", (-1, 1), 0, i32),
            (
                "gt_s",
                (0x80000000_u32 as i32, 0x80000000_u32 as i32),
                0,
                i32
            ),
            (
                "gt_s",
                (0x7fffffff_u32 as i32, 0x7fffffff_u32 as i32),
                0,
                i32
            ),
            ("gt_s", (-1, -1), 0, i32),
            ("gt_s", (1, 0), 1, i32),
            ("gt_s", (0, 1), 0, i32),
            ("gt_s", (0x80000000_u32 as i32, 0), 0, i32),
            ("gt_s", (0, 0x80000000_u32 as i32), 1, i32),
            ("gt_s", (0x80000000_u32 as i32, -1), 0, i32),
            ("gt_s", (-1, 0x80000000_u32 as i32), 1, i32),
            (
                "gt_s",
                (0x80000000_u32 as i32, 0x7fffffff_u32 as i32),
                0,
                i32
            ),
            (
                "gt_s",
                (0x7fffffff_u32 as i32, 0x80000000_u32 as i32),
                1,
                i32
            ),
            ("gt_u", (0, 0), 0, i32),
            ("gt_u", (1, 1), 0, i32),
            ("gt_u", (-1, 1), 1, i32),
            (
                "gt_u",
                (0x80000000_u32 as i32, 0x80000000_u32 as i32),
                0,
                i32
            ),
            (
                "gt_u",
                (0x7fffffff_u32 as i32, 0x7fffffff_u32 as i32),
                0,
                i32
            ),
            ("gt_u", (-1, -1), 0, i32),
            ("gt_u", (1, 0), 1, i32),
            ("gt_u", (0, 1), 0, i32),
            ("gt_u", (0x80000000_u32 as i32, 0), 1, i32),
            ("gt_u", (0, 0x80000000_u32 as i32), 0, i32),
            ("gt_u", (0x80000000_u32 as i32, -1), 0, i32),
            ("gt_u", (-1, 0x80000000_u32 as i32), 1, i32),
            (
                "gt_u",
                (0x80000000_u32 as i32, 0x7fffffff_u32 as i32),
                1,
                i32
            ),
            (
                "gt_u",
                (0x7fffffff_u32 as i32, 0x80000000_u32 as i32),
                0,
                i32
            ),
        ]
    );
    Ok(())
}

#[test]
fn test_wasm_i64_arith() -> Result<()> {
    let code = include_bytes!("../../../dora-compiler/src/wasm/tests/suites/i64_arith.wat");
    build_wasm_code!(code, artifact);
    generate_test_cases!(
        &artifact,
        [
            ("add", (1, 1), 2, i64),
            ("add", (1, 0), 1, i64),
            ("add", (-1_i64, -1_i64), -2, i64),
            ("add", (-1_i64, 1_i64), 0, i64),
            (
                "add",
                (0x7fffffffffffffff_u64 as i64, 1_i64),
                0x8000000000000000_u64 as i64,
                i64
            ),
            (
                "add",
                (0x8000000000000000_u64 as i64, -1_i64),
                0x7fffffffffffffff_u64 as i64,
                i64
            ),
            (
                "add",
                (0x8000000000000000_u64 as i64, 0x8000000000000000_u64 as i64),
                0,
                i64
            ),
            ("add", (0x3fffffff_i64, 1_i64), 0x40000000, i64),
            ("sub", (1_i64, 1_i64), 0, i64),
            ("sub", (1_i64, 0_i64), 1, i64),
            ("sub", (-1_i64, -1_i64), 0, i64),
            (
                "sub",
                (0x7fffffffffffffff_u64 as i64, -1_i64),
                0x8000000000000000_u64 as i64,
                i64
            ),
            (
                "sub",
                (0x8000000000000000_u64 as i64, 1_i64),
                0x7fffffffffffffff_u64 as i64,
                i64
            ),
            (
                "sub",
                (0x8000000000000000_u64 as i64, 0x8000000000000000_u64 as i64),
                0,
                i64
            ),
            ("sub", (0x3fffffff_i64, -1_i64), 0x40000000, i64),
            ("mul", (1_i64, 1_i64), 1, i64),
            ("mul", (1_i64, 0_i64), 0, i64),
            ("mul", (-1_i64, -1_i64), 1, i64),
            ("mul", (0x1000000000000000_u64 as i64, 4096_i64), 0, i64),
            ("mul", (0x8000000000000000_u64 as i64, 0_i64), 0, i64),
            (
                "mul",
                (0x8000000000000000_u64 as i64, -1_i64),
                0x8000000000000000_u64 as i64,
                i64
            ),
            (
                "mul",
                (0x7fffffffffffffff_u64 as i64, -1_i64),
                0x8000000000000001_u64 as i64,
                i64
            ),
            (
                "mul",
                (0x0123456789abcdef_u64 as i64, 0xfedcba9876543210_u64 as i64),
                0x2236d88fe5618cf0,
                i64
            ),
            (
                "mul",
                (0x7fffffffffffffff_u64 as i64, 0x7fffffffffffffff_u64 as i64),
                1,
                i64
            ),
            ("div_s", (1_i64, 1_i64), 1, i64),
            ("div_s", (0_i64, 1_i64), 0, i64),
            ("div_s", (0_i64, -1_i64), 0, i64),
            ("div_s", (-1_i64, -1_i64), 1, i64),
            (
                "div_s",
                (0x8000000000000000_u64 as i64, 2),
                0xc000000000000000_u64 as i64,
                i64
            ),
            (
                "div_s",
                (0x8000000000000001_u64 as i64, 1000),
                0xffdf3b645a1cac09_u64 as i64,
                i64
            ),
            ("div_s", (5_i64, 2_i64), 2_i64, i64),
            ("div_s", (-5_i64, 2_i64), -2_i64, i64),
            ("div_s", (5_i64, -2_i64), -2_i64, i64),
            ("div_s", (-5_i64, -2_i64), 2_i64, i64),
            ("div_s", (7_i64, 3_i64), 2_i64, i64),
            ("div_s", (-7_i64, 3_i64), -2_i64, i64),
            ("div_s", (7_i64, -3_i64), -2_i64, i64),
            ("div_s", (-7_i64, -3_i64), 2_i64, i64),
            ("div_s", (11_i64, 5_i64), 2_i64, i64),
            ("div_s", (17_i64, 7_i64), 2_i64, i64),
            ("div_u", (1_i64, 1_i64), 1_i64, i64),
            ("div_u", (0_i64, 1_i64), 0_i64, i64),
            ("div_u", (-1_i64, -1_i64), 1_i64, i64),
            ("div_u", (0x8000000000000000_u64 as i64, -1_i64), 0_i64, i64),
            (
                "div_u",
                (0x8000000000000000_u64 as i64, 2_i64),
                0x4000000000000000_u64 as i64,
                i64
            ),
            (
                "div_u",
                (0x8ff00ff00ff00ff0_u64 as i64, 0x100000001_u64 as i64),
                0x8ff00fef_u64 as i64,
                i64
            ),
            (
                "div_u",
                (0x8000000000000001_u64 as i64, 1000_i64),
                0x20c49ba5e353f7_u64 as i64,
                i64
            ),
            ("div_u", (5_i64, 2_i64), 2_i64, i64),
            ("div_u", (-5_i64, 2_i64), 0x7ffffffffffffffd_u64 as i64, i64),
            ("div_u", (5_i64, -2_i64), 0_i64, i64),
            ("div_u", (-5_i64, -2_i64), 0_i64, i64),
            ("div_u", (7_i64, 3_i64), 2_i64, i64),
            ("div_u", (11_i64, 5_i64), 2_i64, i64),
            ("div_u", (17_i64, 7_i64), 2_i64, i64),
            ("rem_s", (0x7fffffffffffffff_u64 as i64, -1_i64), 0_i64, i64),
            ("rem_s", (1_i64, 1_i64), 0_i64, i64),
            ("rem_s", (0_i64, 1_i64), 0_i64, i64),
            ("rem_s", (0_i64, -1_i64), 0_i64, i64),
            ("rem_s", (-1_i64, -1_i64), 0_i64, i64),
            // ("rem_s", (0x8000000000000000_u64 as i64, -1_i64), 0_i64, i64),
            ("rem_s", (0x8000000000000000_u64 as i64, 2_i64), 0_i64, i64),
            (
                "rem_s",
                (0x8000000000000001_u64 as i64, 1000_i64),
                -807_i64,
                i64
            ),
            ("rem_s", (5_i64, 2_i64), 1_i64, i64),
            ("rem_s", (-5_i64, 2_i64), -1_i64, i64),
            ("rem_s", (5_i64, -2_i64), 1_i64, i64),
            ("rem_s", (-5_i64, -2_i64), -1_i64, i64),
            ("rem_s", (7_i64, 3_i64), 1_i64, i64),
            ("rem_s", (-7_i64, 3_i64), -1_i64, i64),
            ("rem_s", (7_i64, -3_i64), 1_i64, i64),
            ("rem_s", (-7_i64, -3_i64), -1_i64, i64),
            ("rem_s", (11_i64, 5_i64), 1_i64, i64),
            ("rem_s", (17_i64, 7_i64), 3_i64, i64),
            ("rem_u", (1_i64, 1_i64), 0_i64, i64),
            ("rem_u", (0_i64, 1_i64), 0_i64, i64),
            ("rem_u", (-1_i64, -1_i64), 0_i64, i64),
            (
                "rem_u",
                (0x8000000000000000_u64 as i64, -1_i64),
                0x8000000000000000_u64 as i64,
                i64
            ),
            ("rem_u", (0x8000000000000000_u64 as i64, 2_i64), 0, i64),
            (
                "rem_u",
                (0x8ff00ff00ff00ff0_u64 as i64, 0x100000001_u64 as i64),
                0x80000001_u64 as i64,
                i64
            ),
            ("rem_u", (0x8000000000000001_u64 as i64, 1000), 809, i64),
            ("rem_u", (5_i64, 2_i64), 1_i64, i64),
            ("rem_u", (-5_i64, 2_i64), 1_i64, i64),
            ("rem_u", (5_i64, -2_i64), 5_i64, i64),
            ("rem_u", (-5_i64, -2_i64), -5_i64, i64),
            ("rem_u", (7_i64, 3_i64), 1_i64, i64),
            ("rem_u", (11_i64, 5_i64), 1_i64, i64),
            ("rem_u", (17_i64, 7_i64), 3_i64, i64),
            ("and", (1_i64, 0_i64), 0_i64, i64),
            ("and", (0_i64, 1_i64), 0_i64, i64),
            ("and", (1_i64, 1_i64), 1_i64, i64),
            ("and", (0_i64, 0_i64), 0_i64, i64),
            (
                "and",
                (0x7fffffffffffffff_u64 as i64, 0x8000000000000000_u64 as i64),
                0,
                i64
            ),
            (
                "and",
                (0x7fffffffffffffff_u64 as i64, -1_i64),
                0x7fffffffffffffff_u64 as i64,
                i64
            ),
            (
                "and",
                (0xf0f0ffff_u64 as i64, 0xfffff0f0_u64 as i64),
                0xf0f0f0f0_u64 as i64,
                i64
            ),
            (
                "and",
                (0xffffffffffffffff_u64 as i64, 0xffffffffffffffff_u64 as i64),
                0xffffffffffffffff_u64 as i64,
                i64
            ),
            ("or", (1_i64, 0_i64), 1_i64, i64),
            ("or", (0_i64, 1_i64), 1_i64, i64),
            ("or", (1_i64, 1_i64), 1_i64, i64),
            ("or", (0_i64, 0_i64), 0_i64, i64),
            (
                "or",
                (0x7fffffffffffffff_u64 as i64, 0x8000000000000000_u64 as i64),
                -1,
                i64
            ),
            (
                "or",
                (0x8000000000000000_u64 as i64, 0),
                0x8000000000000000_u64 as i64,
                i64
            ),
            (
                "or",
                (0xf0f0ffff_u64 as i64, 0xfffff0f0_u64 as i64),
                0xffffffff_u64 as i64,
                i64
            ),
            (
                "or",
                (0xffffffffffffffff_u64 as i64, 0xffffffffffffffff_u64 as i64),
                0xffffffffffffffff_u64 as i64,
                i64
            ),
            ("xor", (1_i64, 0_i64), 1_i64, i64),
            ("xor", (0_i64, 1_i64), 1_i64, i64),
            ("xor", (1_i64, 1_i64), 0_i64, i64),
            ("xor", (0_i64, 0_i64), 0_i64, i64),
            (
                "xor",
                (0x7fffffffffffffff_u64 as i64, 0x8000000000000000_u64 as i64),
                -1_i64,
                i64
            ),
            (
                "xor",
                (0x8000000000000000_u64 as i64, 0),
                0x8000000000000000_u64 as i64,
                i64
            ),
            (
                "xor",
                (-1_i64, 0x8000000000000000_u64 as i64),
                0x7fffffffffffffff_u64 as i64,
                i64
            ),
            (
                "xor",
                (-1_i64, 0x7fffffffffffffff_u64 as i64),
                0x8000000000000000_u64 as i64,
                i64
            ),
            (
                "xor",
                (0xf0f0ffff_u64 as i64, 0xfffff0f0_u64 as i64),
                0x0f0f0f0f_u64 as i64,
                i64
            ),
            (
                "xor",
                (0xffffffffffffffff_u64 as i64, 0xffffffffffffffff_u64 as i64),
                0,
                i64
            ),
            ("shl", (1_i64, 1_i64), 2_i64, i64),
            ("shl", (1_i64, 0_i64), 1_i64, i64),
            (
                "shl",
                (0x7fffffffffffffff_u64 as i64, 1),
                0xfffffffffffffffe_u64 as i64,
                i64
            ),
            (
                "shl",
                (0xffffffffffffffff_u64 as i64, 1),
                0xfffffffffffffffe_u64 as i64,
                i64
            ),
            ("shl", (0x8000000000000000_u64 as i64, 1), 0_i64, i64),
            (
                "shl",
                (0x4000000000000000_u64 as i64, 1),
                0x8000000000000000_u64 as i64,
                i64
            ),
            ("shl", (1_i64, 63_i64), 0x8000000000000000_u64 as i64, i64),
            // Note: The result of shift overflow is 0
            ("shl", (1_i64, 64_i64), 0, i64),
            ("shl", (1_i64, 65_i64), 0, i64),
            ("shl", (1_i64, -1_i64), 0, i64),
            ("shl", (1_i64, 0x7fffffffffffffff_u64 as i64), 0, i64),
            ("shr_s", (1_i64, 1_i64), 0, i64),
            ("shr_s", (1_i64, 0_i64), 1, i64),
            ("shr_s", (-1_i64, 1_i64), -1, i64),
            (
                "shr_s",
                (0x7fffffffffffffff_u64 as i64, 1),
                0x3fffffffffffffff_u64 as i64,
                i64
            ),
            (
                "shr_s",
                (0x8000000000000000_u64 as i64, 1),
                0xc000000000000000_u64 as i64,
                i64
            ),
            (
                "shr_s",
                (0x4000000000000000_u64 as i64, 1),
                0x2000000000000000_u64 as i64,
                i64
            ),
            // Note: The result of shift overflow is 0
            ("shr_s", (1_i64, 64_i64), 0, i64),
            ("shr_s", (1_i64, 65_i64), 0, i64),
            ("shr_s", (1_i64, -1_i64), 0, i64),
            ("shr_s", (1_i64, 0x7fffffffffffffff_u64 as i64), 0, i64),
            ("shr_s", (1_i64, 0x8000000000000000_u64 as i64), 0, i64),
            ("shr_s", (0x8000000000000000_u64 as i64, 63), -1_i64, i64),
            ("shr_s", (-1_i64, 64_i64), -1, i64),
            ("shr_s", (-1_i64, 65_i64), -1, i64),
            ("shr_s", (-1_i64, -1_i64), -1, i64),
            ("shr_s", (-1_i64, 0x7fffffffffffffff_u64 as i64), -1, i64),
            ("shr_s", (-1_i64, 0x8000000000000000_u64 as i64), -1, i64),
            ("shr_u", (1_i64, 1_i64), 0_i64, i64),
            ("shr_u", (1_i64, 0_i64), 1_i64, i64),
            ("shr_u", (-1_i64, 1_i64), 0x7fffffffffffffff_u64 as i64, i64),
            (
                "shr_u",
                (0x7fffffffffffffff_u64 as i64, 1),
                0x3fffffffffffffff_u64 as i64,
                i64
            ),
            (
                "shr_u",
                (0x8000000000000000_u64 as i64, 1),
                0x4000000000000000_u64 as i64,
                i64
            ),
            (
                "shr_u",
                (0x4000000000000000_u64 as i64, 1),
                0x2000000000000000_u64 as i64,
                i64
            ),
            // Note: The result of shift overflow is 0
            ("shr_u", (1_i64, 64_i64), 0_i64, i64),
            ("shr_u", (1_i64, 65_i64), 0_i64, i64),
            ("shr_u", (1_i64, -1_i64), 0_i64, i64),
            ("shr_u", (1_i64, 0x7fffffffffffffff_u64 as i64), 0, i64),
            ("shr_u", (1_i64, 0x8000000000000000_u64 as i64), 0, i64),
            ("shr_u", (0x8000000000000000_u64 as i64, 63), 1, i64),
            // Note: The result of shift overflow is 0
            ("shr_u", (-1_i64, 64_i64), 0, i64),
            ("shr_u", (-1_i64, 65_i64), 0, i64),
            ("shr_u", (-1_i64, -1_i64), 0, i64),
            ("shr_u", (-1_i64, 0x7fffffffffffffff_u64 as i64), 0, i64),
            ("shr_u", (-1_i64, 0x8000000000000000_u64 as i64), 0, i64),
            ("rotl", (1_i64, 1_i64), 2_i64, i64),
            ("rotl", (1_i64, 0_i64), 1_i64, i64),
            ("rotl", (-1_i64, 1_i64), -1, i64),
            ("rotl", (1_i64, 64_i64), 1_i64, i64),
            (
                "rotl",
                (0xabcd987602468ace_u64 as i64, 1),
                0x579b30ec048d159d_u64 as i64,
                i64
            ),
            (
                "rotl",
                (0xfe000000dc000000_u64 as i64, 4),
                0xe000000dc000000f_u64 as i64,
                i64
            ),
            (
                "rotl",
                (0xabcd1234ef567809_u64 as i64, 53),
                0x013579a2469deacf_u64 as i64,
                i64
            ),
            (
                "rotl",
                (0xabd1234ef567809c_u64 as i64, 63),
                0x55e891a77ab3c04e_u64 as i64,
                i64
            ),
            (
                "rotl",
                (0xabcd1234ef567809_u64 as i64, 0xf5),
                0x013579a2469deacf_u64 as i64,
                i64
            ),
            (
                "rotl",
                (0xabcd7294ef567809_u64 as i64, 0xffffffffffffffed_u64 as i64),
                0xcf013579ae529dea_u64 as i64,
                i64
            ),
            (
                "rotl",
                (0xabd1234ef567809c_u64 as i64, 0x800000000000003f_u64 as i64),
                0x55e891a77ab3c04e_u64 as i64,
                i64
            ),
            ("rotl", (1_i64, 63_i64), 0x8000000000000000_u64 as i64, i64),
            ("rotl", (0x8000000000000000_u64 as i64, 1), 1, i64),
            ("rotr", (1_i64, 1_i64), 0x8000000000000000_u64 as i64, i64),
            ("rotr", (1_i64, 0_i64), 1_i64, i64),
            ("rotr", (-1_i64, 1_i64), -1_i64, i64),
            ("rotr", (1_i64, 64_i64), 1_i64, i64),
            (
                "rotr",
                (0xabcd987602468ace_u64 as i64, 1),
                0x55e6cc3b01234567_u64 as i64,
                i64
            ),
            (
                "rotr",
                (0xfe000000dc000000_u64 as i64, 4),
                0x0fe000000dc00000_u64 as i64,
                i64
            ),
            (
                "rotr",
                (0xabcd1234ef567809_u64 as i64, 53),
                0x6891a77ab3c04d5e_u64 as i64,
                i64
            ),
            (
                "rotr",
                (0xabd1234ef567809c_u64 as i64, 63),
                0x57a2469deacf0139_u64 as i64,
                i64
            ),
            (
                "rotr",
                (0xabcd1234ef567809_u64 as i64, 0xf5),
                0x6891a77ab3c04d5e_u64 as i64,
                i64
            ),
            (
                "rotr",
                (0xabcd7294ef567809_u64 as i64, 0xffffffffffffffed_u64 as i64),
                0x94a77ab3c04d5e6b_u64 as i64,
                i64
            ),
            (
                "rotr",
                (0xabd1234ef567809c_u64 as i64, 0x800000000000003f_u64 as i64),
                0x57a2469deacf0139_u64 as i64,
                i64
            ),
            ("rotr", (1_i64, 63_i64), 2, i64),
            ("rotr", (0x8000000000000000_u64 as i64, 63), 1, i64),
            ("clz", (0xffffffffffffffff_u64 as i64,), 0, i64),
            ("clz", (0,), 64, i64),
            ("clz", (0x00008000_u64 as i64,), 48, i64),
            ("clz", (0xff,), 56, i64),
            ("clz", (0x8000000000000000_u64 as i64,), 0, i64),
            ("clz", (1,), 63, i64),
            ("clz", (2,), 62, i64),
            ("clz", (0x7fffffffffffffff_u64 as i64,), 1, i64),
            ("ctz", (-1_i64,), 0, i64),
            ("ctz", (0,), 64, i64),
            ("ctz", (0x00008000_u64 as i64,), 15, i64),
            ("ctz", (0x00010000_u64 as i64,), 16, i64),
            ("ctz", (0x8000000000000000_u64 as i64,), 63, i64),
            ("ctz", (0x7fffffffffffffff_u64 as i64,), 0, i64),
            ("popcnt", (-1_i64,), 64, i64),
            ("popcnt", (0,), 0, i64),
            ("popcnt", (0x00008000_u64 as i64,), 1, i64),
            ("popcnt", (0x8000800080008000_u64 as i64,), 4, i64),
            ("popcnt", (0x7fffffffffffffff_u64 as i64,), 63, i64),
            ("popcnt", (0xAAAAAAAA55555555_u64 as i64,), 32, i64),
            ("popcnt", (0x99999999AAAAAAAA_u64 as i64,), 32, i64),
            ("popcnt", (0xDEADBEEFDEADBEEF_u64 as i64,), 48, i64),
            ("extend8_s", (0,), 0, i64),
            ("extend8_s", (0x7f,), 127_i64, i64),
            ("extend8_s", (0x80,), -128_i64, i64),
            ("extend8_s", (0xff,), -1_i64, i64),
            ("extend8_s", (0x0123456789abcd00_u64 as i64,), 0, i64),
            ("extend8_s", (0xfedcba9876543280_u64 as i64,), -128_i64, i64),
            ("extend8_s", (-1_i64,), -1_i64, i64),
            ("extend16_s", (0,), 0, i64),
            ("extend16_s", (0x7fff,), 32767_i64, i64),
            ("extend16_s", (0x8000,), -32768_i64, i64),
            ("extend16_s", (0xffff,), -1_i64, i64),
            ("extend16_s", (0x123456789abc0000_u64 as i64,), 0, i64),
            ("extend16_s", (0xfedcba9876548000_u64 as i64,), -32768, i64),
            ("extend16_s", (-1_i64,), -1_i64, i64),
            ("extend32_s", (0,), 0, i64),
            ("extend32_s", (0x7fff,), 32767, i64),
            ("extend32_s", (0x8000,), 32768, i64),
            ("extend32_s", (0xffff,), 65535, i64),
            ("extend32_s", (0x7fffffff_u64 as i64,), 0x7fffffff, i64),
            ("extend32_s", (0x80000000_u64 as i64,), -0x80000000, i64),
            ("extend32_s", (0xffffffff_u64 as i64,), -1, i64),
            ("extend32_s", (0x0123456700000000_u64 as i64,), 0, i64),
            (
                "extend32_s",
                (0xfedcba9880000000_u64 as i64,),
                -0x80000000,
                i64
            ),
            ("extend32_s", (-1_i64,), -1, i64),
            ("eqz", (0,), 1, i32),
            ("eqz", (1,), 0, i32),
            ("eqz", (0x8000000000000000_u64 as i64,), 0, i32),
            ("eqz", (0x7fffffffffffffff_u64 as i64,), 0, i32),
            ("eqz", (0xffffffffffffffff_u64 as i64,), 0, i32),
            ("eq", (0_i64, 0_i64), 1_i32, i32),
            ("eq", (1_i64, 1_i64), 1_i32, i32),
            ("eq", (-1_i64, 1_i64), 0_i32, i32),
            (
                "eq",
                (0x8000000000000000_u64 as i64, 0x8000000000000000_u64 as i64),
                1,
                i32
            ),
            (
                "eq",
                (0x7fffffffffffffff_u64 as i64, 0x7fffffffffffffff_u64 as i64),
                1,
                i32
            ),
            ("eq", (-1_i64, -1_i64), 1_i32, i32),
            ("eq", (1_i64, 0_i64), 0_i32, i32),
            ("eq", (0_i64, 1_i64), 0_i32, i32),
            ("eq", (0x8000000000000000_u64 as i64, 0), 0, i32),
            ("eq", (0_i64, 0x8000000000000000_u64 as i64), 0, i32),
            ("eq", (0x8000000000000000_u64 as i64, -1_i64), 0, i32),
            ("eq", (-1_i64, 0x8000000000000000_u64 as i64), 0, i32),
            (
                "eq",
                (0x8000000000000000_u64 as i64, 0x7fffffffffffffff_u64 as i64),
                0,
                i32
            ),
            (
                "eq",
                (0x7fffffffffffffff_u64 as i64, 0x8000000000000000_u64 as i64),
                0,
                i32
            ),
            ("ne", (0_i64, 0_i64), 0_i32, i32),
            ("ne", (1_i64, 1_i64), 0_i32, i32),
            ("ne", (-1_i64, 1_i64), 1_i32, i32),
            (
                "ne",
                (0x8000000000000000_u64 as i64, 0x8000000000000000_u64 as i64),
                0,
                i32
            ),
            (
                "ne",
                (0x7fffffffffffffff_u64 as i64, 0x7fffffffffffffff_u64 as i64),
                0,
                i32
            ),
            ("ne", (-1_i64, -1_i64), 0, i32),
            ("ne", (1_i64, 0_i64), 1, i32),
            ("ne", (0_i64, 1_i64), 1, i32),
            ("ne", (0x8000000000000000_u64 as i64, 0), 1, i32),
            ("ne", (0_i64, 0x8000000000000000_u64 as i64), 1, i32),
            ("ne", (0x8000000000000000_u64 as i64, -1), 1, i32),
            ("ne", (-1_i64, 0x8000000000000000_u64 as i64), 1, i32),
            (
                "ne",
                (0x8000000000000000_u64 as i64, 0x7fffffffffffffff_u64 as i64),
                1,
                i32
            ),
            (
                "ne",
                (0x7fffffffffffffff_u64 as i64, 0x8000000000000000_u64 as i64),
                1,
                i32
            ),
            ("lt_s", (0_i64, 0_i64), 0, i32),
            ("lt_s", (1_i64, 1_i64), 0, i32),
            ("lt_s", (-1_i64, 1_i64), 1, i32),
            (
                "lt_s",
                (0x8000000000000000_u64 as i64, 0x8000000000000000_u64 as i64),
                0,
                i32
            ),
            (
                "lt_s",
                (0x7fffffffffffffff_u64 as i64, 0x7fffffffffffffff_u64 as i64),
                0,
                i32
            ),
            ("lt_s", (-1_i64, -1_i64), 0, i32),
            ("lt_s", (1_i64, 0_i64), 0, i32),
            ("lt_s", (0_i64, 1_i64), 1, i32),
            ("lt_s", (0x8000000000000000_u64 as i64, 0_i64), 1_i32, i32),
            ("lt_s", (0_i64, 0x8000000000000000_u64 as i64), 0, i32),
            ("lt_s", (0x8000000000000000_u64 as i64, -1_i64), 1_i32, i32),
            ("lt_s", (-1_i64, 0x8000000000000000_u64 as i64), 0, i32),
            (
                "lt_s",
                (0x8000000000000000_u64 as i64, 0x7fffffffffffffff_u64 as i64),
                1,
                i32
            ),
            (
                "lt_s",
                (0x7fffffffffffffff_u64 as i64, 0x8000000000000000_u64 as i64),
                0,
                i32
            ),
            ("lt_u", (0_i64, 0_i64), 0, i32),
            ("lt_u", (1_i64, 1_i64), 0, i32),
            ("lt_u", (-1_i64, 1_i64), 0, i32),
            (
                "lt_u",
                (0x8000000000000000_u64 as i64, 0x8000000000000000_u64 as i64),
                0,
                i32
            ),
            (
                "lt_u",
                (0x7fffffffffffffff_u64 as i64, 0x7fffffffffffffff_u64 as i64),
                0,
                i32
            ),
            ("lt_u", (-1_i64, -1_i64), 0, i32),
            ("lt_u", (1_i64, 0_i64), 0_i32, i32),
            ("lt_u", (0_i64, 1_i64), 1_i32, i32),
            ("lt_u", (0x8000000000000000_u64 as i64, 0_i64), 0, i32),
            ("lt_u", (0_i64, 0x8000000000000000_u64 as i64), 1, i32),
            ("lt_u", (0x8000000000000000_u64 as i64, -1_i64), 1, i32),
            ("lt_u", (-1_i64, 0x8000000000000000_u64 as i64), 0, i32),
            (
                "lt_u",
                (0x8000000000000000_u64 as i64, 0x7fffffffffffffff_u64 as i64),
                0,
                i32
            ),
            (
                "lt_u",
                (0x7fffffffffffffff_u64 as i64, 0x8000000000000000_u64 as i64),
                1,
                i32
            ),
            ("le_s", (0_i64, 0_i64), 1, i32),
            ("le_s", (1_i64, 1_i64), 1, i32),
            ("le_s", (-1_i64, 1_i64), 1, i32),
            (
                "le_s",
                (0x8000000000000000_u64 as i64, 0x8000000000000000_u64 as i64),
                1,
                i32
            ),
            (
                "le_s",
                (0x7fffffffffffffff_u64 as i64, 0x7fffffffffffffff_u64 as i64),
                1,
                i32
            ),
            ("le_s", (-1_i64, -1_i64), 1, i32),
            ("le_s", (1_i64, 0_i64), 0_i32, i32),
            ("le_s", (0_i64, 1_i64), 1_i32, i32),
            ("le_s", (0x8000000000000000_u64 as i64, 0), 1_i32, i32),
            ("le_s", (0, 0x8000000000000000_u64 as i64), 0_i32, i32),
            ("le_s", (0x8000000000000000_u64 as i64, -1_i64), 1_i32, i32),
            ("le_s", (-1_i64, 0x8000000000000000_u64 as i64), 0_i32, i32),
            (
                "le_s",
                (0x8000000000000000_u64 as i64, 0x7fffffffffffffff_u64 as i64),
                1,
                i32
            ),
            (
                "le_s",
                (0x7fffffffffffffff_u64 as i64, 0x8000000000000000_u64 as i64),
                0,
                i32
            ),
            ("le_u", (0_i64, 0_i64), 1_i32, i32),
            ("le_u", (1_i64, 1_i64), 1_i32, i32),
            ("le_u", (-1_i64, 1_i64), 0_i32, i32),
            (
                "le_u",
                (0x8000000000000000_u64 as i64, 0x8000000000000000_u64 as i64),
                1_i32,
                i32
            ),
            (
                "le_u",
                (0x7fffffffffffffff_u64 as i64, 0x7fffffffffffffff_u64 as i64),
                1_i32,
                i32
            ),
            ("le_u", (-1_i64, -1_i64), 1_i32, i32),
            ("le_u", (1_i64, 0_i64), 0_i32, i32),
            ("le_u", (0_i64, 1_i64), 1_i32, i32),
            ("le_u", (0x8000000000000000_u64 as i64, 0), 0_i32, i32),
            ("le_u", (0, 0x8000000000000000_u64 as i64), 1_i32, i32),
            ("le_u", (0x8000000000000000_u64 as i64, -1_i64), 1_i32, i32),
            ("le_u", (-1_i64, 0x8000000000000000_u64 as i64), 0_i32, i32),
            (
                "le_u",
                (0x8000000000000000_u64 as i64, 0x7fffffffffffffff_u64 as i64),
                0,
                i32
            ),
            (
                "le_u",
                (0x7fffffffffffffff_u64 as i64, 0x8000000000000000_u64 as i64),
                1,
                i32
            ),
            ("gt_s", (0_i64, 0_i64), 0_i32, i32),
            ("gt_s", (1_i64, 1_i64), 0_i32, i32),
            ("gt_s", (-1_i64, 1_i64), 0_i32, i32),
            (
                "gt_s",
                (0x8000000000000000_u64 as i64, 0x8000000000000000_u64 as i64),
                0,
                i32
            ),
            (
                "gt_s",
                (0x7fffffffffffffff_u64 as i64, 0x7fffffffffffffff_u64 as i64),
                0,
                i32
            ),
            ("gt_s", (-1_i64, -1_i64), 0, i32),
            ("gt_s", (1_i64, 0_i64), 1, i32),
            ("gt_s", (0_i64, 1_i64), 0, i32),
            ("gt_s", (0x8000000000000000_u64 as i64, 0), 0, i32),
            ("gt_s", (0_i64, 0x8000000000000000_u64 as i64), 1, i32),
            ("gt_s", (0x8000000000000000_u64 as i64, -1), 0, i32),
            ("gt_s", (-1_i64, 0x8000000000000000_u64 as i64), 1, i32),
            (
                "gt_s",
                (0x8000000000000000_u64 as i64, 0x7fffffffffffffff_u64 as i64),
                0,
                i32
            ),
            (
                "gt_s",
                (0x7fffffffffffffff_u64 as i64, 0x8000000000000000_u64 as i64),
                1,
                i32
            ),
            ("gt_u", (0, 0), 0, i32),
            ("gt_u", (1_i64, 1_i64), 0_i32, i32),
            ("gt_u", (-1_i64, 1_i64), 1_i32, i32),
            (
                "gt_u",
                (0x8000000000000000_u64 as i64, 0x8000000000000000_u64 as i64),
                0,
                i32
            ),
            (
                "gt_u",
                (0x7fffffffffffffff_u64 as i64, 0x7fffffffffffffff_u64 as i64),
                0,
                i32
            ),
            ("gt_u", (-1_i64, -1_i64), 0, i32),
            ("gt_u", (1, 0), 1, i32),
            ("gt_u", (0, 1), 0, i32),
            ("gt_u", (0x8000000000000000_u64 as i64, 0), 1, i32),
            ("gt_u", (0_i64, 0x8000000000000000_u64 as i64), 0, i32),
            ("gt_u", (0x8000000000000000_u64 as i64, -1_i64), 0, i32),
            ("gt_u", (-1_i64, 0x8000000000000000_u64 as i64), 1, i32),
            (
                "gt_u",
                (0x8000000000000000_u64 as i64, 0x7fffffffffffffff_u64 as i64),
                1,
                i32
            ),
            (
                "gt_u",
                (0x7fffffffffffffff_u64 as i64, 0x8000000000000000_u64 as i64),
                0,
                i32
            ),
            ("ge_s", (0, 0), 1, i32),
            ("ge_s", (1, 1), 1, i32),
            ("ge_s", (-1_i64, 1_i64), 0, i32),
            (
                "ge_s",
                (0x8000000000000000_u64 as i64, 0x8000000000000000_u64 as i64),
                1,
                i32
            ),
            (
                "ge_s",
                (0x7fffffffffffffff_u64 as i64, 0x7fffffffffffffff_u64 as i64),
                1,
                i32
            ),
            ("ge_s", (-1_i64, -1_i64), 1, i32),
            ("ge_s", (1, 0), 1, i32),
            ("ge_s", (0, 1), 0, i32),
            ("ge_s", (0x8000000000000000_u64 as i64, 0), 0, i32),
            ("ge_s", (0, 0x8000000000000000_u64 as i64), 1, i32),
            ("ge_s", (0x8000000000000000_u64 as i64, -1), 0, i32),
            ("ge_s", (-1_i64, 0x8000000000000000_u64 as i64), 1, i32),
            (
                "ge_s",
                (0x8000000000000000_u64 as i64, 0x7fffffffffffffff_u64 as i64),
                0,
                i32
            ),
            (
                "ge_s",
                (0x7fffffffffffffff_u64 as i64, 0x8000000000000000_u64 as i64),
                1,
                i32
            ),
            ("ge_u", (0, 0), 1, i32),
            ("ge_u", (1, 1), 1, i32),
            ("ge_u", (-1_i64, 1_i64), 1, i32),
            (
                "ge_u",
                (0x8000000000000000_u64 as i64, 0x8000000000000000_u64 as i64),
                1,
                i32
            ),
            (
                "ge_u",
                (0x7fffffffffffffff_u64 as i64, 0x7fffffffffffffff_u64 as i64),
                1,
                i32
            ),
            ("ge_u", (-1_i64, -1_i64), 1, i32),
            ("ge_u", (1, 0), 1, i32),
            ("ge_u", (0, 1), 0, i32),
            ("ge_u", (0x8000000000000000_u64 as i64, 0), 1, i32),
            ("ge_u", (0, 0x8000000000000000_u64 as i64), 0, i32),
            ("ge_u", (0x8000000000000000_u64 as i64, -1_i64), 0, i32),
            ("ge_u", (-1_i64, 0x8000000000000000_u64 as i64), 1, i32),
            (
                "ge_u",
                (0x8000000000000000_u64 as i64, 0x7fffffffffffffff_u64 as i64),
                1,
                i32
            ),
            (
                "ge_u",
                (0x7fffffffffffffff_u64 as i64, 0x8000000000000000_u64 as i64),
                0,
                i32
            ),
        ]
    );
    Ok(())
}

#[test]
fn test_wasm_if() -> Result<()> {
    let code = include_bytes!("../../../dora-compiler/src/wasm/tests/suites/if.wat");
    build_wasm_code!(code, artifact);
    generate_test_cases!(
        &artifact,
        [
            ("empty", (0,), (), ()),
            ("empty", (1,), (), ()),
            ("empty", (100,), (), ()),
            ("empty", (-2,), (), ()),
            ("singular", (0,), 8, i32),
            ("singular", (1,), 7, i32),
            ("singular", (10,), 7, i32),
            ("singular", (-10,), 7, i32),
            ("multi", (0,), (9, -1), (i32, i32)),
            ("multi", (1,), (8, 1), (i32, i32)),
            ("multi", (13,), (8, 1), (i32, i32)),
            ("multi", (-5,), (8, 1), (i32, i32)),
            ("nested", (0, 0), 11, i32),
            ("nested", (1, 0), 10, i32),
            ("nested", (0, 1), 10, i32),
            ("nested", (3, 2), 9, i32),
            ("nested", (0, -100), 10, i32),
            ("nested", (10, 10), 9, i32),
            ("nested", (0, -1), 10, i32),
            ("nested", (-111, -2), 9, i32),
            ("as-select-first", (0,), 0, i32),
            ("as-select-first", (1,), 1, i32),
            ("as-select-mid", (0,), 2, i32),
            ("as-select-mid", (1,), 2, i32),
            ("as-select-last", (0,), 3, i32),
            ("as-select-last", (1,), 2, i32),
            ("as-loop-first", (0,), 0, i32),
            ("as-loop-first", (1,), 1, i32),
            ("as-loop-mid", (0,), 0, i32),
            ("as-loop-mid", (1,), 1, i32),
            ("as-loop-last", (0,), 0, i32),
            ("as-loop-last", (1,), 1, i32),
            ("as-if-condition", (0,), 3, i32),
            ("as-if-condition", (1,), 2, i32),
            ("as-br_if-first", (0,), 0, i32),
            ("as-br_if-first", (1,), 1, i32),
            ("as-br_if-last", (0,), 3, i32),
            ("as-br_if-last", (1,), 2, i32),
            ("as-br_table-first", (0,), 0, i32),
            ("as-br_table-first", (1,), 1, i32),
            ("as-br_table-last", (0,), 2, i32),
            ("as-br_table-last", (1,), 2, i32),
            // ("as-call_indirect-first", (0,), 0, i32),
            // ("as-call_indirect-first", (1,), 1, i32),
            // ("as-call_indirect-mid", (0,), 2, i32),
            // ("as-call_indirect-mid", (1,), 2, i32),
            // ("as-call_indirect-last", (0,), 2, i32),
            ("as-store-first", (0,), (), ()),
            ("as-store-first", (1,), (), ()),
            ("as-store-last", (0,), (), ()),
            ("as-store-last", (1,), (), ()),
            ("as-memory.grow-value", (0,), 1, i32),
            ("as-memory.grow-value", (1,), 1, i32),
            ("as-call-value", (0,), 0, i32),
            ("as-call-value", (1,), 1, i32),
            ("as-return-value", (0,), 0, i32),
            ("as-return-value", (1,), 1, i32),
            ("as-drop-operand", (0,), (), ()),
            ("as-drop-operand", (1,), (), ()),
            ("as-br-value", (0,), 0, i32),
            ("as-br-value", (1,), 1, i32),
            ("as-local.set-value", (0,), 0, i32),
            ("as-local.set-value", (1,), 1, i32),
            ("as-local.tee-value", (0,), 0, i32),
            ("as-local.tee-value", (1,), 1, i32),
            ("as-global.set-value", (0,), 0, i32),
            ("as-global.set-value", (1,), 1, i32),
            ("as-load-operand", (0,), 0, i32),
            ("as-load-operand", (1,), 0, i32),
            ("as-unary-operand", (0,), 0, i32),
            ("as-unary-operand", (1,), 0, i32),
            ("as-unary-operand", (-1,), 0, i32),
            ("as-binary-operand", (0, 0), 15, i32),
            ("as-binary-operand", (0, 1), -12, i32),
            ("as-binary-operand", (1, 0), -15, i32),
            ("as-binary-operand", (1, 1), 12, i32),
            ("as-test-operand", (0,), 1, i32),
            ("as-test-operand", (1,), 0, i32),
            ("as-compare-operand", (0, 0), 1, i32),
            ("as-compare-operand", (0, 1), 0, i32),
            ("as-compare-operand", (1, 0), 1, i32),
            ("as-compare-operand", (1, 1), 0, i32),
            ("as-binary-operands", (0,), -12, i32),
            ("as-binary-operands", (1,), 12, i32),
            ("as-compare-operands", (0,), 1, i32),
            ("as-compare-operands", (1,), 0, i32),
            ("as-mixed-operands", (0,), -3, i32),
            ("as-mixed-operands", (1,), 27, i32),
            ("break-bare", (), 19, i32),
            ("break-value", (1,), 18, i32),
            ("break-value", (0,), 21, i32),
            // ("break-multi-value", (0,), (-18_i32, 18_i32, -18_i64), (i32, i32, i64)),
            // ("break-multi-value", (1,), (18_i32, -18_i32, 18_i64), (i32, i32, i64)),
            ("param", (0,), -1, i32),
            ("param", (1,), 3, i32),
            ("params", (0,), -1, i32),
            ("params", (1,), 3, i32),
            ("params-id", (0,), 3, i32),
            ("params-id", (1,), 3, i32),
            ("param-break", (0,), -1, i32),
            ("param-break", (1,), 3, i32),
            ("params-break", (0,), -1, i32),
            ("params-break", (1,), 3, i32),
            ("params-id-break", (0,), 3, i32),
            ("params-id-break", (1,), 3, i32),
            ("effects", (1,), -14, i32),
            ("effects", (0,), -6, i32),
        ]
    );
    Ok(())
}

#[test]
fn test_wasm_int_exprs() -> Result<()> {
    let code = include_bytes!("../../../dora-compiler/src/wasm/tests/suites/int_exprs.wat");
    build_wasm_code!(code, artifact);
    generate_test_cases!(
        &artifact,
        [
            (
                "i32.no_fold_cmp_s_offset",
                (0x7fffffff_u32 as i32, 0),
                1,
                i32
            ),
            (
                "i32.no_fold_cmp_u_offset",
                (0xffffffff_u32 as i32, 0),
                1,
                i32
            ),
            (
                "i64.no_fold_cmp_s_offset",
                (0x7fffffffffffffff_u64 as i64, 0),
                1,
                i32
            ),
            (
                "i64.no_fold_cmp_u_offset",
                (0xffffffffffffffff_u64 as i64, 0),
                1,
                i32
            ),
            (
                "i64.no_fold_wrap_extend_s",
                (0x0010203040506070_u64 as i64,),
                0x0000000040506070_u64 as i64,
                i64
            ),
            (
                "i64.no_fold_wrap_extend_s",
                (0x00a0b0c0d0e0f0a0_u64 as i64,),
                0xffffffffd0e0f0a0_u64 as i64,
                i64
            ),
            ("i32.no_fold_shl_shr_s", (0x80000000_u64 as i64,), 0, i32),
            ("i32.no_fold_shl_shr_u", (0x80000000_u64 as i64,), 0, i32),
            (
                "i64.no_fold_shl_shr_s",
                (0x8000000000000000_u64 as i64,),
                0,
                i64
            ),
            (
                "i64.no_fold_shl_shr_u",
                (0x8000000000000000_u64 as i64,),
                0,
                i64
            ),
            ("i32.no_fold_div_s_mul", (1,), 0, i32),
            ("i32.no_fold_div_u_mul", (1,), 0, i32),
            ("i64.no_fold_div_s_mul", (1,), 0, i64),
            ("i64.no_fold_div_u_mul", (1,), 0, i64),
            ("i32.no_fold_div_s_self", (0,), (), ()),
            ("i32.no_fold_div_u_self", (0,), (), ()),
            ("i64.no_fold_div_s_self", (0,), (), ()),
            ("i64.no_fold_div_u_self", (0,), (), ()),
            ("i32.no_fold_rem_s_self", (0,), (), ()),
            ("i32.no_fold_rem_u_self", (0,), (), ()),
            ("i64.no_fold_rem_s_self", (0,), (), ()),
            ("i64.no_fold_rem_u_self", (0,), (), ()),
            ("i32.no_fold_mul_div_s", (0x80000000_u32 as i32,), 0, i32),
            ("i32.no_fold_mul_div_u", (0x80000000_u32 as i32,), 0, i32),
            (
                "i64.no_fold_mul_div_s",
                (0x8000000000000000_u64 as i64,),
                0,
                i64
            ),
            (
                "i64.no_fold_mul_div_u",
                (0x8000000000000000_u64 as i64,),
                0,
                i64
            ),
            ("i32.no_fold_div_s_2", (-11_i32,), -5_i32, i32),
            ("i64.no_fold_div_s_2", (-11_i64,), -5_i64, i64),
            ("i32.no_fold_rem_s_2", (-11_i32,), -1_i32, i32),
            ("i64.no_fold_rem_s_2", (-11_i64,), -1_i64, i64),
            ("i32.div_s_0", (71,), (), ()),
            ("i32.div_u_0", (71,), (), ()),
            ("i64.div_s_0", (71,), (), ()),
            ("i64.div_u_0", (71,), (), ()),
            ("i32.div_s_3", (71,), 23, i32),
            (
                "i32.div_s_3",
                (0x60000000_u32 as i32,),
                0x20000000_u32 as i32,
                i32
            ),
            ("i32.div_u_3", (71,), 23, i32),
            (
                "i32.div_u_3",
                (0xc0000000_u32 as i32,),
                0x40000000_u32 as i32,
                i32
            ),
            ("i64.div_s_3", (71,), 23, i64),
            (
                "i64.div_s_3",
                (0x3000000000000000_u64 as i64,),
                0x1000000000000000_u64 as i64,
                i64
            ),
            ("i64.div_u_3", (71,), 23, i64),
            (
                "i64.div_u_3",
                (0xc000000000000000_u64 as i64,),
                0x4000000000000000_u64 as i64,
                i64
            ),
            ("i32.div_s_5", (71,), 14, i32),
            (
                "i32.div_s_5",
                (0x50000000_u32 as i32,),
                0x10000000_u32 as i32,
                i32
            ),
            ("i32.div_u_5", (71,), 14, i32),
            (
                "i32.div_u_5",
                (0xa0000000_u32 as i32,),
                0x20000000_u32 as i32,
                i32
            ),
            ("i64.div_s_5", (71,), 14, i64),
            (
                "i64.div_s_5",
                (0x5000000000000000_u64 as i64,),
                0x1000000000000000_u64 as i64,
                i64
            ),
            ("i64.div_u_5", (71,), 14, i64),
            (
                "i64.div_u_5",
                (0xa000000000000000_u64 as i64,),
                0x2000000000000000_u64 as i64,
                i64
            ),
            ("i32.div_s_7", (71,), 10, i32),
            (
                "i32.div_s_7",
                (0x70000000_u32 as i32,),
                0x10000000_u32 as i32,
                i32
            ),
            ("i32.div_u_7", (71,), 10, i32),
            (
                "i32.div_u_7",
                (0xe0000000_u32 as i32,),
                0x20000000_u32 as i32,
                i32
            ),
            ("i64.div_s_7", (71,), 10, i64),
            (
                "i64.div_s_7",
                (0x7000000000000000_u64 as i64,),
                0x1000000000000000_u64 as i64,
                i64
            ),
            ("i64.div_u_7", (71,), 10, i64),
            (
                "i64.div_u_7",
                (0xe000000000000000_u64 as i64,),
                0x2000000000000000_u64 as i64,
                i64
            ),
            ("i32.rem_s_3", (71,), 2, i32),
            ("i32.rem_s_3", (0x60000000_u32 as i32,), 0, i32),
            ("i32.rem_u_3", (71,), 2, i32),
            ("i32.rem_u_3", (0xc0000000_u32 as i32,), 0, i32),
            ("i64.rem_s_3", (71,), 2, i64),
            ("i64.rem_s_3", (0x3000000000000000_u64 as i64,), 0, i64),
            ("i64.rem_u_3", (71,), 2, i64),
            ("i64.rem_u_3", (0xc000000000000000_u64 as i64,), 0, i64),
            ("i32.rem_s_5", (71,), 1, i32),
            ("i32.rem_s_5", (0x50000000_u32 as i32,), 0, i32),
            ("i32.rem_u_5", (71,), 1, i32),
            ("i32.rem_u_5", (0xa0000000_u32 as i32,), 0, i32),
            ("i64.rem_s_5", (71,), 1, i64),
            ("i64.rem_s_5", (0x5000000000000000_u64 as i64,), 0, i64),
            ("i64.rem_u_5", (71,), 1, i64),
            ("i64.rem_u_5", (0xa000000000000000_u64 as i64,), 0, i64),
            ("i32.rem_s_7", (71,), 1, i32),
            ("i32.rem_s_7", (0x70000000_u32 as i32,), 0, i32),
            ("i32.rem_u_7", (71,), 1, i32),
            ("i32.rem_u_7", (0xe0000000_u32 as i32,), 0, i32),
            ("i64.rem_s_7", (71,), 1, i64),
            ("i64.rem_s_7", (0x7000000000000000_u64 as i64,), 0, i64),
            ("i64.rem_u_7", (71,), 1, i64),
            ("i64.rem_u_7", (0xe000000000000000_u64 as i64,), 0, i64),
        ]
    );
    Ok(())
}

#[test]
fn test_wasm_int_literals() -> Result<()> {
    let code = include_bytes!("../../../dora-compiler/src/wasm/tests/suites/int_literals.wat");
    build_wasm_code!(code, artifact);
    generate_test_cases!(
        &artifact,
        [
            ("i32.test", (), 195940365, i32),
            ("i32.umax", (), -1, i32),
            ("i32.smax", (), 2147483647, i32),
            ("i32.neg_smax", (), -2147483647, i32),
            ("i32.smin", (), -2147483648, i32),
            ("i32.alt_smin", (), -2147483648, i32),
            ("i32.inc_smin", (), -2147483647, i32),
            ("i32.neg_zero", (), 0, i32),
            ("i32.not_octal", (), 10, i32),
            ("i32.unsigned_decimal", (), -1, i32),
            ("i32.plus_sign", (), 42, i32),
            ("i64.test", (), 913028331277281902, i64),
            ("i64.umax", (), -1, i64),
            ("i64.smax", (), 9223372036854775807, i64),
            ("i64.neg_smax", (), -9223372036854775807, i64),
            ("i64.smin", (), -9223372036854775808, i64),
            ("i64.alt_smin", (), -9223372036854775808, i64),
            ("i64.inc_smin", (), -9223372036854775807, i64),
            ("i64.neg_zero", (), 0, i64),
            ("i64.not_octal", (), 10, i64),
            ("i64.unsigned_decimal", (), -1, i64),
            ("i64.plus_sign", (), 42, i64),
            ("i32-dec-sep1", (), 1000000, i32),
            ("i32-dec-sep2", (), 1000, i32),
            ("i32-hex-sep1", (), 0xa0f0099, i32),
            ("i32-hex-sep2", (), 0x1aa0f, i32),
            ("i64-dec-sep1", (), 1000000, i64),
            ("i64-dec-sep2", (), 1000, i64),
            ("i64-hex-sep1", (), 0xaf00f00009999, i64),
            ("i64-hex-sep2", (), 0x1aa0f, i64),
        ]
    );
    Ok(())
}

#[test]
fn test_wasm_label() -> Result<()> {
    let code = include_bytes!("../../../dora-compiler/src/wasm/tests/suites/label.wat");
    build_wasm_code!(code, artifact);
    generate_test_cases!(
        &artifact,
        [
            ("block", (), 1, i32),
            ("loop1", (), 5, i32),
            ("loop2", (), 8, i32),
            ("loop3", (), 1, i32),
            ("loop4", (8,), 16, i32),
            ("loop5", (), 2, i32),
            ("loop6", (), 3, i32),
            ("if", (), 5, i32),
            ("if2", (), 5, i32),
            ("switch", (0,), 50, i32),
            ("switch", (1,), 20, i32),
            ("switch", (2,), 20, i32),
            ("switch", (3,), 3, i32),
            ("switch", (4,), 50, i32),
            ("switch", (5,), 50, i32),
            ("return", (0,), 0, i32),
            ("return", (1,), 2, i32),
            ("return", (2,), 2, i32),
            ("br_if0", (), 0x1d, i32),
            ("br_if1", (), 1, i32),
            ("br_if2", (), 1, i32),
            ("br_if3", (), 2, i32),
            ("br", (), 1, i32),
            ("shadowing", (), 1, i32),
            ("redefinition", (), 5, i32),
        ]
    );
    Ok(())
}

#[test]
fn test_wasm_left_to_right() -> Result<()> {
    let code = include_bytes!("../../../dora-compiler/src/wasm/tests/suites/left_to_right.wat");
    build_wasm_code!(code, artifact);
    generate_test_cases!(
        &artifact,
        [
            ("i32_add", (), 0x0102, i32),
            ("i64_add", (), 0x0102, i32),
            ("i32_sub", (), 0x0102, i32),
            ("i64_sub", (), 0x0102, i32),
            ("i32_mul", (), 0x0102, i32),
            ("i64_mul", (), 0x0102, i32),
            ("i32_div_s", (), 0x0102, i32),
            ("i64_div_s", (), 0x0102, i32),
            ("i32_div_u", (), 0x0102, i32),
            ("i64_div_u", (), 0x0102, i32),
            ("i32_rem_s", (), 0x0102, i32),
            ("i64_rem_s", (), 0x0102, i32),
            ("i32_rem_u", (), 0x0102, i32),
            ("i64_rem_u", (), 0x0102, i32),
            ("i32_and", (), 0x0102, i32),
            ("i64_and", (), 0x0102, i32),
            ("i32_or", (), 0x0102, i32),
            ("i64_or", (), 0x0102, i32),
            ("i32_xor", (), 0x0102, i32),
            ("i64_xor", (), 0x0102, i32),
            ("i32_shl", (), 0x0102, i32),
            ("i64_shl", (), 0x0102, i32),
            ("i32_shr_u", (), 0x0102, i32),
            ("i64_shr_u", (), 0x0102, i32),
            ("i32_shr_s", (), 0x0102, i32),
            ("i64_shr_s", (), 0x0102, i32),
            ("i32_eq", (), 0x0102, i32),
            ("i64_eq", (), 0x0102, i32),
            ("i32_ne", (), 0x0102, i32),
            ("i64_ne", (), 0x0102, i32),
            ("i32_lt_s", (), 0x0102, i32),
            ("i64_lt_s", (), 0x0102, i32),
            ("i32_le_s", (), 0x0102, i32),
            ("i64_le_s", (), 0x0102, i32),
            ("i32_lt_u", (), 0x0102, i32),
            ("i64_lt_u", (), 0x0102, i32),
            ("i32_le_u", (), 0x0102, i32),
            ("i64_le_u", (), 0x0102, i32),
            ("i32_gt_s", (), 0x0102, i32),
            ("i64_gt_s", (), 0x0102, i32),
            ("i32_ge_s", (), 0x0102, i32),
            ("i64_ge_s", (), 0x0102, i32),
            ("i32_gt_u", (), 0x0102, i32),
            ("i64_gt_u", (), 0x0102, i32),
            ("i32_ge_u", (), 0x0102, i32),
            ("i64_ge_u", (), 0x0102, i32),
            ("i32_store", (), 0x0102, i32),
            ("i32_store8", (), 0x0102, i32),
            ("i32_store16", (), 0x0102, i32),
            ("i64_store32", (), 0x0102, i32),
            ("i32_call", (), 0x0102, i32),
            ("i64_call", (), 0x0102, i32),
            // ("i32_call_indirect", (), 0x010204, i32),
            // ("i64_call_indirect", (), 0x010204, i32),
            ("i32_select", (), 0x010205, i32),
            ("i64_select", (), 0x010205, i32),
            ("f32_add", (), 0x0102, i32),
            ("f64_add", (), 0x0102, i32),
            ("f32_sub", (), 0x0102, i32),
            ("f64_sub", (), 0x0102, i32),
            ("f32_mul", (), 0x0102, i32),
            ("f64_mul", (), 0x0102, i32),
            ("f32_div", (), 0x0102, i32),
            ("f64_div", (), 0x0102, i32),
            ("f32_copysign", (), 0x0102, i32),
            ("f64_copysign", (), 0x0102, i32),
            ("f32_eq", (), 0x0102, i32),
            ("f64_eq", (), 0x0102, i32),
            ("f32_ne", (), 0x0102, i32),
            ("f64_ne", (), 0x0102, i32),
            ("f32_lt", (), 0x0102, i32),
            ("f64_lt", (), 0x0102, i32),
            ("f32_le", (), 0x0102, i32),
            ("f64_le", (), 0x0102, i32),
            ("f32_gt", (), 0x0102, i32),
            ("f64_gt", (), 0x0102, i32),
            ("f32_ge", (), 0x0102, i32),
            ("f64_ge", (), 0x0102, i32),
            ("f32_min", (), 0x0102, i32),
            ("f64_min", (), 0x0102, i32),
            ("f32_max", (), 0x0102, i32),
            ("f64_max", (), 0x0102, i32),
            ("f32_store", (), 0x0102, i32),
            ("f64_store", (), 0x0102, i32),
            ("f32_call", (), 0x0102, i32),
            ("f64_call", (), 0x0102, i32),
            // ("f32_call_indirect", (), 0x010204, i32),
            // ("f64_call_indirect", (), 0x010204, i32),
            ("f32_select", (), 0x010205, i32),
            ("f64_select", (), 0x010205, i32),
            ("br_if", (), 0x0102, i32),
            ("br_table", (), 0x0102, i32),
        ]
    );
    Ok(())
}

#[test]
fn test_wasm_load() -> Result<()> {
    let code = include_bytes!("../../../dora-compiler/src/wasm/tests/suites/load.wat");
    build_wasm_code!(code, artifact);
    generate_test_cases!(
        &artifact,
        [
            ("as-br-value", (), 0, i32),
            ("as-br_if-cond", (), (), ()),
            ("as-br_if-value", (), 0, i32),
            ("as-br_if-value-cond", (), 7, i32),
            ("as-br_table-index", (), (), ()),
            ("as-br_table-value", (), 0, i32),
            ("as-br_table-value-index", (), 6, i32),
            ("as-return-value", (), 0, i32),
            ("as-if-cond", (), 1, i32),
            ("as-if-then", (), 0, i32),
            ("as-if-else", (), 0, i32),
            ("as-select-first", (0, 1), 0, i32),
            ("as-select-second", (0, 0), 0, i32),
            ("as-select-cond", (), 1, i32),
            ("as-call-first", (), -1, i32),
            ("as-call-mid", (), -1, i32),
            ("as-call-last", (), -1, i32),
            // ("as-call_indirect-first", (), -1, i32),
            // ("as-call_indirect-mid", (), -1, i32),
            // ("as-call_indirect-last", (), -1, i32),
            // ("as-call_indirect-index", (), -1, i32),
            ("as-local.set-value", (), (), ()),
            ("as-local.tee-value", (), 0, i32),
            ("as-global.set-value", (), (), ()),
            ("as-load-address", (), 0, i32),
            ("as-loadN-address", (), 0, i32),
            ("as-store-address", (), (), ()),
            ("as-store-value", (), (), ()),
            ("as-storeN-address", (), (), ()),
            ("as-storeN-value", (), (), ()),
            ("as-unary-operand", (), 32, i32),
            ("as-binary-left", (), 10, i32),
            ("as-binary-right", (), 10, i32),
            ("as-test-operand", (), 1, i32),
            ("as-compare-left", (), 1, i32),
            ("as-compare-right", (), 1, i32),
            ("as-memory.grow-size", (), 1, i32),
        ]
    );
    Ok(())
}
