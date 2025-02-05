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
                assert_eq!(result, $expect);
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
