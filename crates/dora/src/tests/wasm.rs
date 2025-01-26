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
