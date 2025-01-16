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

#[test]
fn test_build_wasm_artifact_and_run() {
    // Build WASM code
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
    let code = wat2wasm(code).unwrap();
    let artifact = build_wasm_artifact::<MemoryDB>(&code.to_vec().into()).unwrap();
    // Run WASM code with env.
    let env = Env::default();
    let mut host = DummyHost::new(env);
    let mut runtime_context = RuntimeContext::new(
        Contract::new_with_env(&host.env, Bytecode::new(code.to_vec().into()), None),
        1,
        false,
        false,
        &mut host,
        SpecId::CANCUN,
    );
    let mut gas = INIT_GAS;
    artifact.execute(
        &mut runtime_context,
        &mut gas,
        &mut Stack::new(),
        &mut (MAX_STACK_SIZE as u64),
    );
}
