use std::any::Any;
use std::panic::panic_any;
use wasmer_vm::TrapCode;

/// Implementation for raising a WASM trap
pub fn wasm_raise_trap(trap_code: TrapCode) -> ! {
    panic_any(trap_code)
}

/// Trap error to string
pub fn err_to_str(err: Box<dyn Any + Send>) -> String {
    if let Some(trap) = err.downcast_ref::<TrapCode>() {
        trap.message().to_string()
    } else if let Some(s) = err.downcast_ref::<&str>() {
        s.to_string()
    } else if let Some(s) = err.downcast_ref::<&String>() {
        (*s).clone()
    } else if let Some(s) = err.downcast_ref::<String>() {
        (*s).clone()
    } else {
        "".to_string()
    }
}
