#![no_main]

use dora::run_evm_bytecode_with_calldata;
use dora_primitives::spec::SpecId;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let program = hex::encode(data);
    let _ = run_evm_bytecode_with_calldata(&program, "", 999_999, SpecId::CANCUN);
});
