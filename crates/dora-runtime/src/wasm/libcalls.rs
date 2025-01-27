use wasmer_vm::libcalls::wasmer_vm_memory32_fill;
use wasmer_vm::catch_traps;
use wasmer::VMConfig;

const VM_CONFIG: VMConfig = VMConfig {
    wasm_stack_size: None
};
