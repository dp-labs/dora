//！Reference: https://github.com/WebAssembly/spec/tree/main/test/core

use crate::build_wasm_artifact;
use crate::MemoryDB;
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

    let code = include_bytes!("suites/brainfuck.wat");
    build_wasm_code!(code, artifact, runtime_context, gas);
    let result: i32 = artifact
        .execute_wasm_func_with_context("user_entrypoint", 0, runtime_context, gas)
        .unwrap();
    assert_eq!(result, 0);
}

#[test]
fn test_wasm_sum() -> anyhow::Result<()> {
    let code = include_bytes!("suites/sum.wat");
    build_wasm_code!(code, artifact);
    generate_test_cases!(&artifact, [("main", (), (), ()),]);
    Ok(())
}

#[test]
fn test_wasm_fib() -> anyhow::Result<()> {
    let code = include_bytes!("suites/fib.wat");
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
fn test_wasm_global() -> anyhow::Result<()> {
    let code = include_bytes!("suites/global.wat");
    build_wasm_code!(code, artifact);
    generate_test_cases!(&artifact, [("user_entrypoint", 10, 10 + 255 + 255, i32),]);
    Ok(())
}

#[test]
fn test_wasm_address() -> anyhow::Result<()> {
    let code = include_bytes!("suites/address.wat");
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
fn test_wasm_align_read_write() -> anyhow::Result<()> {
    let code = include_bytes!("suites/align.wat");
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
fn test_wasm_block() -> anyhow::Result<()> {
    let code = include_bytes!("suites/block.wat");
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
