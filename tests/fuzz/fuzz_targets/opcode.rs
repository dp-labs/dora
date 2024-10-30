#![no_main]

use dora::run_evm_program;
use dora_compiler::evm::Program;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let program = Program::from_opcode(data);
    let _ = run_evm_program(&program, &[], 999_999);
});
