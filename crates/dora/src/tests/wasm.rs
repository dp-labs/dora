use crate::build_wasm_artifact;
use crate::tests::INIT_GAS;
use crate::MemoryDB;
use dora_primitives::Bytecode;
use dora_primitives::SpecId;
use dora_runtime::context::Contract;
use dora_runtime::context::RuntimeContext;
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
    let _ret: () = artifact
        .execute_wasm_func("main", (), runtime_context, gas)
        .unwrap();
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

#[test]
fn test_wasm_global() {
    let code = br#"
(module
  (global $a i32 (i32.const 255))
  (global $b i32 (i32.const 255))

  (func $main (export "user_entrypoint") (param $c i32) (result i32)
    global.get $a
    global.get $b
    i32.add
    local.get $c
    i32.add
  )
)
"#;
    build_wasm_code!(code, artifact, runtime_context, gas);
    let result: i32 = artifact
        .execute_wasm_func("user_entrypoint", 10, runtime_context, gas)
        .unwrap();
    assert_eq!(result, 10 + 255 + 255);
}

// #[test]
// TODO: fix panic on macos arm-64.
fn _test_wasm_brainfuck_with_host_functions() {
    let code = br#"
(module
  (import "vm_hooks" "read_args"    (func $read_args   (param i32    )))
  (import "vm_hooks" "write_result" (func $return_data (param i32 i32)))
  (memory (export "memory") 1 1)

  ;; args advances byte by byte
  (global $args_ptr (mut i32) (i32.const 0x00))
  (global $args_end (mut i32) (i32.const 0x00))

  ;; outs just extends the length
  (global $outs_ptr i32 (i32.const 0x400))
  (global $outs_len (mut i32) (i32.const 0))
  (global $outs_cap i32 (i32.const 0x400))

  (global $cell (mut i32) (i32.const 0x800))

  ;; sets up the entry point
  (func $main (export "user_entrypoint") (param $args_len i32) (result i32)

      ;; load the args
      local.get $args_len
      global.set $args_end
      global.get $args_ptr
      call $read_args

      ;; set up the pointer
      global.get $outs_ptr
      global.get $outs_cap
      i32.add
      global.set $cell

      ;; call the generated program
      call $user

      ;; write the outs
      ;; global.get $outs_ptr
      ;; global.get $outs_len
      ;; call $return_data

      ;; return status success
      i32.const 0)

  ;; reads one byte from the args
  (func $comma
      ;; read the value
      global.get $args_ptr
      global.get $args_end
      i32.eq
      (if (then
        ;; at the end, write a 0
        global.get $cell
        i32.const 0
        i32.store8
        return))

      ;; write the value
      global.get $cell
      global.get $args_ptr
      i32.load8_u
      i32.store8

      ;; advance 1 byte
      global.get $args_ptr
      i32.const 1
      i32.add
      global.set $args_ptr)

  ;; writes one byte to the outs
  (func $dot
      ;; noop when out of space
      global.get $outs_len
      global.get $outs_cap
      i32.eq
      (if (then (return)))

      ;; store the value
      global.get $outs_ptr
      global.get $outs_len
      i32.add
      global.get $cell
      i32.load8_u
      i32.store8

      ;; advance by 1
      global.get $outs_len
      i32.const 1
      i32.add
      global.set $outs_len)

  (func $right
      global.get $cell
      i32.const 1
      i32.add
      global.set $cell)

  (func $left
      global.get $cell
      i32.const 1
      i32.sub
      global.set $cell)

  (func $plus
      global.get $cell
      global.get $cell
      i32.load8_u
      i32.const 1
      i32.add
      i32.store8)

  (func $minus
      global.get $cell
      global.get $cell
      i32.load8_u
      i32.const 1
      i32.sub
      i32.store8)

  (func $repeat (result i32)
      global.get $cell
      i32.load8_u
      i32.const 0
      i32.ne)
  (func $user)
)
"#;
    build_wasm_code!(code, artifact, runtime_context, gas);
    let result: i32 = artifact
        .execute_wasm_func("user_entrypoint", 0, runtime_context, gas)
        .unwrap();
    assert_eq!(result, 0);
}
