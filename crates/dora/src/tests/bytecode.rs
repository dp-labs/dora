use dora_primitives::{Bytecode, IsWASMBytecode};
use wasmer::wat2wasm;

#[test]
fn test_evm_bytecode() {
    let bytecode = hex_literal::hex!(
        "6080806040526004361015610012575f80fd5b5f3560e01c9081633fb5c1cb146101035781638381f58a146100cc575063d09de08a1461003d575f80fd5b346100c8575f7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126100c8575f547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff811461009b576001015f55005b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f80fd5b346100c8575f7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126100c8576020905f548152f35b346100c85760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126100c8576004355f5500fea264697066735822122019105920c33dab502c8d24693bd24f1b4b0fbb562ad3a475173a15b7aa3574f064736f6c63430008190033"
    );
    assert!(!Bytecode::new_raw(bytecode.into()).is_wasm())
}

#[test]
fn test_eof_evm_bytecode() {
    let bytecode = hex_literal::hex!(
        "ef0001010004020001013c04006d00008000056080806040526004361015e100035f80fd5f3560e01c9081633fb5c1cb14e100e081638381f58a14e1009c5063d09de08a14e100045fe0ffd534e100875f7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc360112e1005c5f547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8114e100086001015f555f80f37f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5f80fd5f80fd34e100335f7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc360112e100086020905f548152f35f80fd5f80fd34e1003460207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc360112e100086004355f555f80f35f80fd5f80fda3646970667358221220cc6570f0f9fb641c08c7d9d1c36810bd19987855bcd8f9ccde7cfcd8670b41fa6c6578706572696d656e74616cf564736f6c63782c302e382e32372d646576656c6f702e323032342e372e32342b636f6d6d69742e64353139363430342e6d6f64006b"
    );
    assert!(Bytecode::new_raw(bytecode.into()).is_eof())
}

#[test]
fn test_wasm_bytecode() {
    let bytecode = include_bytes!("../../../dora-compiler/src/wasm/tests/suites/counter.wat");
    let bytecode = wat2wasm(bytecode).unwrap();
    assert_eq!(hex::encode(&bytecode[0..4]), "0061736d");
    assert!(Bytecode::new_raw(bytecode.to_vec().into()).is_wasm())
}
