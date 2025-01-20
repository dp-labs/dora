use crate::build_wasm_artifact;
use crate::tests::INIT_GAS;
use crate::MemoryDB;
use dora_primitives::Bytecode;
use dora_primitives::SpecId;
use dora_runtime::artifact::Artifact;
use dora_runtime::constants::MAX_STACK_SIZE;
use dora_runtime::context::Contract;
use dora_runtime::context::RuntimeContext;
use dora_runtime::context::Stack;
use dora_runtime::env::Env;
use dora_runtime::host::DummyHost;
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
        let mut $runtime_context = RuntimeContext::new(
            Contract::new_with_env(&host.env, Bytecode::new(wasm_code.to_vec().into()), None),
            1,
            false,
            false,
            &mut host,
            SpecId::CANCUN,
        );
        let mut $gas = INIT_GAS;
    };
}

macro_rules! build_runtime_context {
    ($runtime_context:ident, $gas:ident) => {
        // Run WASM code with env.
        let env = Env::default();
        let mut host = DummyHost::new(env);
        let $runtime_context = RuntimeContext::new(
            Contract::new_with_env(&host.env, Bytecode::new(vec![].into()), None),
            1,
            false,
            false,
            &mut host,
            SpecId::CANCUN,
        );
        let $gas = INIT_GAS;
    };
}

#[test]
fn test_wasm_main() {
    let code = br#"
(module
  (func $sum_f (param $x i32) (param $y i32) (result i32)
    local.get $x
    local.get $y
    i32.add)

  (func (export "main") 
    (call $sum_f (i32.const 2) (i32.const 3))
    drop
  )
)
"#;
    build_wasm_code!(code, artifact, runtime_context, gas);
    artifact.execute(
        &mut runtime_context,
        &mut gas,
        &mut Stack::new(),
        &mut (MAX_STACK_SIZE as u64),
    );
}

#[test]
fn test_wasm_fib() {
    let code = br#"
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
"#;
    build_wasm_code!(code, artifact);
    let tests: &[(i64, i64)] = &[
        (0, 1),
        (1, 1),
        (2, 2),
        (3, 3),
        (4, 5),
        (5, 8),
        (6, 13),
        (7, 21),
    ];
    for (input, output) in tests {
        build_runtime_context!(runtime_context, gas);
        let result: i64 = artifact
            .execute_wasm_func("fib", *input, runtime_context, gas)
            .unwrap();
        assert_eq!(result, *output);
    }
}
