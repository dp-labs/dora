use crate::Compiler;
use crate::context::Context;
use crate::evm::program::{Operation, Program};
use crate::evm::{EVMCompileOptions, EVMCompiler};
use dora_primitives::SpecId;
use num_bigint::BigInt;
use num_bigint::BigUint;

macro_rules! assert_snapshot {
    ($operations:expr) => {
        assert_snapshot!($operations, false)
    };
    ($operations:expr, $is_eof:expr) => {
        let program = Program::from_operations($operations, $is_eof);
        let spec_id = if $is_eof {
            SpecId::OSAKA
        } else {
            SpecId::CANCUN
        };
        let context = Context::new();
        let compiler = EVMCompiler::new(&context, EVMCompileOptions::default().spec_id(spec_id));
        let mut module = compiler
            .compile(&program)
            .expect("failed to compile program");
        crate::evm::pass::run(&context.mlir_context, &mut module.mlir_module).unwrap();
        crate::dora::pass::run(
            &context.mlir_context,
            &mut module.mlir_module,
            &crate::dora::pass::PassOptions {
                code_size: program.code_size(),
                ..Default::default()
            },
        )
        .unwrap();
        let op = module.module().as_operation();
        assert!(op.verify());
        insta::assert_snapshot!(op);
    };
}

#[test]
fn push_push_add() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, 10_u8.into())),
        Operation::Push((1_u8, 10_u8.into())),
        Operation::Add,
    ]);
}

#[test]
fn push_push_add_overflow() {
    assert_snapshot!(vec![
        Operation::Push((32_u8, u128::MAX.into())),
        Operation::Push((32_u8, 1_u8.into())),
        Operation::Add,
    ]);
}

#[test]
fn push_push_add_overflow_1() {
    assert_snapshot!(vec![
        Operation::Push((32_u8, u64::MAX.into())),
        Operation::Push((32_u8, 1_u64.into())),
        Operation::Add,
    ]);
}

#[test]
fn push_push_mul() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, 10_u8.into())),
        Operation::Push((1_u8, 10_u8.into())),
        Operation::Mul,
    ]);
}

#[test]
fn push_push_mul_large() {
    assert_snapshot!(vec![
        Operation::Push((32_u8, 123456789_u128.into())),
        Operation::Push((32_u8, 987654321_u128.into())),
        Operation::Mul,
    ]);
}

#[test]
fn push_push_mul_overflow() {
    assert_snapshot!(vec![
        Operation::Push((32_u8, u64::MAX.into())),
        Operation::Push((32_u8, 2_u64.into())),
        Operation::Mul,
    ]);
}

#[test]
fn push_push_sub() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, 10_u8.into())),
        Operation::Push((1_u8, 10_u8.into())),
        Operation::Sub,
    ]);
}

#[test]
fn push_push_sub_underflow() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, 10_u8.into())),
        Operation::Push0,
        Operation::Sub,
    ]);
}

#[test]
fn push_push_sub_underflow_1() {
    assert_snapshot!(vec![
        Operation::Push((32_u8, 1_u64.into())),
        Operation::Push((32_u8, 0_u64.into())),
        Operation::Sub,
    ]);
}

#[test]
fn push_push_div() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, 10_u8.into())),
        Operation::Push((1_u8, 10_u8.into())),
        Operation::Div,
    ]);
}

#[test]
fn push_push_div_zero() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, 10_u8.into())),
        Operation::Push0,
        Operation::Div,
    ]);
}

#[test]
fn push_push_mod_zero_1() {
    assert_snapshot!(vec![
        Operation::Push((32_u8, 10_u64.into())),
        Operation::Push((32_u8, 0_u64.into())),
        Operation::Mod,
    ]);
}

#[test]
fn push_push_div_zero_0() {
    assert_snapshot!(vec![
        Operation::Push((32_u8, 10_u64.into())),
        Operation::Push((32_u8, 0_u64.into())),
        Operation::Div,
    ]);
}

#[test]
fn push_push_sdiv_0() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, 10_u8.into())),
        Operation::Push((1_u8, 10_u8.into())),
        Operation::SDiv,
    ]);
}

#[test]
fn push_push_sdiv_1() {
    assert_snapshot!(vec![
        Operation::Push((
            32_u8,
            BigUint::from_bytes_be(&[
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF,
            ]),
        )),
        Operation::Push((
            32_u8,
            BigUint::from_bytes_be(&[
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF,
            ]),
        )),
        Operation::SDiv,
    ]);
}

#[test]
fn push_push_mod_0() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, 3_u8.into())),
        Operation::Push((1_u8, 10_u8.into())),
        Operation::Mod,
    ]);
}

#[test]
fn push_push_mod_1() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, 5_u8.into())),
        Operation::Push((1_u8, 17_u8.into())),
        Operation::Mod,
    ]);
}

#[test]
fn push_push_smod() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, 3_u8.into())),
        Operation::Push((1_u8, 10_u8.into())),
        Operation::SMod,
    ]);
}

#[test]
fn push_push_smod_with_negative() {
    let (a, b) = (
        biguint_256_from_bigint(BigInt::from(-8_i64)),
        biguint_256_from_bigint(BigInt::from(-3_i64)),
    );
    assert_snapshot!(vec![
        Operation::Push((32, b.clone())),
        Operation::Push((32, a.clone())),
        Operation::SMod,
    ]);
}

#[test]
fn push_push_push_addmod() {
    let (a, b) = (BigUint::from(1_u8), 2_u32);
    assert_snapshot!(vec![
        Operation::Push((1_u8, b.into())),
        Operation::Push((1_u8, a.clone())),
        Operation::Push((1_u8, a.clone())),
        Operation::AddMod,
    ]);
}

#[test]
fn push_push_push_addmod_large_mod() {
    assert_snapshot!(vec![
        Operation::Push((32_u8, u128::MAX.into())),
        Operation::Push((32_u8, u128::MAX.into())),
        Operation::Push((32_u8, 100_u8.into())),
        Operation::AddMod,
    ]);
}

#[test]
fn push_push_push_mulmod() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, 8_u32.into())),
        Operation::Push((1_u8, 10_u32.into())),
        Operation::Push((1_u8, 10_u32.into())),
        Operation::MulMod,
    ]);
}

#[test]
fn push_push_push_mulmod_zero_mod() {
    assert_snapshot!(vec![
        Operation::Push((32_u8, 2_u8.into())),
        Operation::Push((32_u8, 2_u8.into())),
        Operation::Push((32_u8, 0_u8.into())), // modulus
        Operation::MulMod,
    ]);
}

#[test]
fn push_push_exp() {
    let (a, b) = (BigUint::from(3_u8), 3_u32);
    assert_snapshot!(vec![
        Operation::Push((1_u8, b.into())),
        Operation::Push((1_u8, a.clone())),
        Operation::Exp,
    ]);
}

#[test]
fn push_push_exp_edge_case() {
    let (a, b) = (BigUint::from(u128::MAX), 1_u32);
    assert_snapshot!(vec![
        Operation::Push((1_u8, b.into())),
        Operation::Push((a.bits() as u8 / 8 + 1, a.clone())),
        Operation::Exp,
    ]);
}

#[test]
fn push_push_exp_large_base() {
    let (a, b) = (BigUint::from(123456789_u64), 5_u32);
    assert_snapshot!(vec![
        Operation::Push((4_u8, b.into())),
        Operation::Push((a.bits() as u8 / 8 + 1, a.clone())),
        Operation::Exp,
    ]);
}

#[test]
fn push_push_signextend() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, BigUint::from_bytes_be(&[0xFF]))),
        Operation::Push((1_u8, 0_u32.into())),
        Operation::SignExtend,
    ]);
}

#[test]
fn push_push_lt() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, 10_u8.into())),
        Operation::Push((1_u8, 9_u8.into())),
        Operation::Lt,
    ]);
}

#[test]
fn push_push_gt() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, 10_u8.into())),
        Operation::Push((1_u8, 9_u8.into())),
        Operation::Gt,
    ]);
}

#[test]
fn push_push_eq_0() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, 10_u8.into())),
        Operation::Push((1_u8, 10_u8.into())),
        Operation::Eq,
    ]);
}

#[test]
fn push_push_eq_1() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, 5_u8.into())),
        Operation::Push((1_u8, 10_u8.into())),
        Operation::Eq,
    ]);
}

#[test]
fn push_push_iszero_0() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, 10_u8.into())),
        Operation::IsZero,
    ]);
}

#[test]
fn push_push_iszero_1() {
    assert_snapshot!(vec![Operation::Push0, Operation::IsZero,]);
}

#[test]
fn push_push_and_0() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, BigUint::from_bytes_be(&[0xFF]))),
        Operation::Push((1_u8, BigUint::from_bytes_be(&[0xFF]))),
        Operation::And,
    ]);
}

#[test]
fn push_push_and_1() {
    assert_snapshot!(vec![
        Operation::Push0,
        Operation::Push((1_u8, BigUint::from_bytes_be(&[0xFF]))),
        Operation::And,
    ]);
}

#[test]
fn push_push_and_edge_case() {
    assert_snapshot!(vec![
        Operation::Push((
            32_u8,
            BigUint::from_bytes_be(&[
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF,
            ])
        )),
        Operation::Push((32_u8, 0_u32.into())),
        Operation::And,
    ]);
}

#[test]
fn push_push_or_0() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, BigUint::from_bytes_be(&[0xF]))),
        Operation::Push((1_u8, BigUint::from_bytes_be(&[0xF0]))),
        Operation::Or,
    ]);
}

#[test]
fn push_push_or_1() {
    assert_snapshot!(vec![
        Operation::Push0,
        Operation::Push((1_u8, BigUint::from_bytes_be(&[0xFF]))),
        Operation::Or,
    ]);
}

#[test]
fn push_push_or_edge_case() {
    assert_snapshot!(vec![
        Operation::Push((
            32_u8,
            BigUint::from_bytes_be(&[
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF,
            ])
        )),
        Operation::Push((32_u8, 0_u32.into())),
        Operation::Or,
    ]);
}

#[test]
fn push_push_xor_0() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, BigUint::from_bytes_be(&[0xF]))),
        Operation::Push((1_u8, BigUint::from_bytes_be(&[0xF0]))),
        Operation::Xor,
    ]);
}

#[test]
fn push_push_xor_1() {
    assert_snapshot!(vec![
        Operation::Push0,
        Operation::Push((1_u8, BigUint::from_bytes_be(&[0xFF]))),
        Operation::Xor,
    ]);
}

#[test]
fn push_push_xor_edge_case() {
    assert_snapshot!(vec![
        Operation::Push((
            32_u8,
            BigUint::from_bytes_be(&[
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF,
            ])
        )),
        Operation::Push((32_u8, 0_u32.into())),
        Operation::Xor,
    ]);
}

#[test]
fn push_not() {
    assert_snapshot!(vec![Operation::Push0, Operation::Not,]);
}

#[test]
fn push_push_byte() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, BigUint::from_bytes_be(&[0xFF]))),
        Operation::Push((1_u8, 31_u8.into())),
        Operation::Byte,
    ]);
}

#[test]
fn push_push_shl() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, 1_u8.into())),
        Operation::Push((1_u8, 1_u8.into())),
        Operation::Shl,
    ]);
}

#[test]
fn push_push_shr() {
    let (a, b) = (BigUint::from(3_u8), 3_u32);
    assert_snapshot!(vec![
        Operation::Push((1_u8, b.into())),
        Operation::Push((1_u8, a.clone())),
        Operation::Shr,
    ]);
}

#[test]
fn push_push_sar() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, 2_u8.into())),
        Operation::Push((1_u8, 1_u8.into())),
        Operation::Sar,
    ]);
}

#[test]
fn push_mstore_keccak256() {
    assert_snapshot!(vec![
        Operation::Push((
            32_u8,
            BigUint::from_bytes_be(&[
                0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00,
            ]),
        )),
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 4_u8.into())),
        Operation::Push0,
        Operation::Keccak256,
    ]);
}

#[test]
fn address() {
    assert_snapshot!(vec![Operation::Address]);
}

#[test]
fn address_balance() {
    assert_snapshot!(vec![Operation::Address, Operation::Balance]);
}

#[test]
fn origin() {
    assert_snapshot!(vec![Operation::Origin]);
}

#[test]
fn caller() {
    assert_snapshot!(vec![Operation::Caller]);
}

#[test]
fn callvalue() {
    assert_snapshot!(vec![Operation::CallValue]);
}

#[test]
fn push_calldataload() {
    assert_snapshot!(vec![Operation::Push0, Operation::CalldataLoad,]);
}

#[test]
fn push_calldataload_edge_case() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, 100_u8.into())),
        Operation::CalldataLoad,
    ]);
}

#[test]
fn calldatasize() {
    assert_snapshot!(vec![Operation::CalldataSize]);
}

#[test]
fn push_calldatacopy() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Push0,
        Operation::CalldataCopy,
    ]);
}

#[test]
fn push_calldatacopy_partial() {
    assert_snapshot!(vec![
        Operation::Push0,
        Operation::Push((1_u8, 10_u8.into())),
        Operation::Push((1_u8, 20_u8.into())),
        Operation::CalldataCopy,
    ]);
}

#[test]
fn push_calldatacopy_out_of_bounds() {
    assert_snapshot!(vec![
        Operation::Push0,
        Operation::Push((1_u8, 100_u8.into())),
        Operation::Push((1_u8, 10_u8.into())),
        Operation::CalldataCopy,
    ]);
}

#[test]
fn push_pop_codesize() {
    assert_snapshot!(vec![Operation::Push0, Operation::Pop, Operation::CodeSize,]);
}

#[test]
fn codesize_edge_case_empty() {
    assert_snapshot!(vec![Operation::CodeSize,]);
}

#[test]
fn push_codecopy() {
    assert_snapshot!(vec![
        Operation::Push((
            30_u8,
            BigUint::from_bytes_be(&[
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF,
            ]),
        )),
        Operation::Push((32_u8, 0_u8.into())),
        Operation::Pop,
        Operation::Pop,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Push0,
        Operation::CodeCopy,
    ]);
}

#[test]
fn push_codecopy_1() {
    assert_snapshot!(vec![
        Operation::Push0,
        Operation::Push0,
        Operation::Push((1_u8, 10_u8.into())),
        Operation::CodeCopy,
    ]);
}

#[test]
fn push_codecopy_partial() {
    assert_snapshot!(vec![
        Operation::Push0,
        Operation::Push((1_u8, 5_u8.into())),
        Operation::Push((1_u8, 5_u8.into())),
        Operation::CodeCopy,
    ]);
}

#[test]
fn push_codecopy_out_of_bounds() {
    assert_snapshot!(vec![
        Operation::Push0,
        Operation::Push((1_u8, 50_u8.into())),
        Operation::Push((1_u8, 10_u8.into())),
        Operation::CodeCopy,
    ]);
}

#[test]
fn gasprice() {
    assert_snapshot!(vec![Operation::GasPrice,]);
}

#[test]
fn push_mstore_create_extcodesize() {
    assert_snapshot!(vec![
        Operation::Push((
            32_u8,
            BigUint::from_bytes_be(&[
                0x7F, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF,
            ]),
        )),
        Operation::Push0,
        Operation::MStore,
        Operation::Push((
            32_u8,
            BigUint::from_bytes_be(&[
                0x7F, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF,
            ]),
        )),
        Operation::Push((1_u8, 32_u8.into())),
        Operation::MStore,
        Operation::Push((1_u8, 41_u8.into())),
        Operation::Push0,
        Operation::Push0,
        Operation::Create,
        Operation::ExtCodeSize,
    ]);
}

#[test]
fn push_extcodesize() {
    assert_snapshot!(vec![
        Operation::Push((20_u8, BigUint::from_bytes_be(&[0xde, 0xad, 0xbe, 0xef]))),
        Operation::ExtCodeSize,
    ]);
}

#[test]
fn push_extcodesize_nonexistent() {
    assert_snapshot!(vec![
        Operation::Push((20_u8, BigUint::from_bytes_be(&[0x00, 0x00, 0x00, 0x00]))),
        Operation::ExtCodeSize,
    ]);
}

#[test]
fn test_create_extcodesize() {
    assert_snapshot!(vec![
        Operation::Push((
            32_u8,
            BigUint::from_bytes_be(&[
                0x7F, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF,
            ]),
        )),
        Operation::Push0,
        Operation::MStore,
        Operation::Push((
            32_u8,
            BigUint::from_bytes_be(&[
                0x7F, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF,
            ]),
        )),
        Operation::Push((1_u8, 32_u8.into())),
        Operation::MStore,
        Operation::Push((1_u8, 41_u8.into())),
        Operation::Push0,
        Operation::Push0,
        Operation::Create,
        Operation::ExtCodeSize,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ]);
}

#[test]
fn push_mstore_create_extcodecopy() {
    assert_snapshot!(vec![
        Operation::Push((
            32_u8,
            BigUint::from_bytes_be(&[
                0x7F, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF,
            ]),
        )),
        Operation::Push0,
        Operation::MStore,
        Operation::Push((
            32_u8,
            BigUint::from_bytes_be(&[
                0x7F, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF,
            ]),
        )),
        Operation::Push((1_u8, 32_u8.into())),
        Operation::MStore,
        Operation::Push((1_u8, 41_u8.into())),
        Operation::Push0,
        Operation::Push0,
        Operation::Create,
        Operation::Push0,
        Operation::Push0,
        Operation::MStore,
        Operation::Push0,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Push0,
        Operation::Dup(4),
        Operation::ExtCodeCopy,
    ]);
}

#[test]
fn push_extcodecopy_basic() {
    assert_snapshot!(vec![
        Operation::Push0,
        Operation::Push0,
        Operation::Push((20_u8, BigUint::from_bytes_be(&[0xde, 0xad, 0xbe, 0xef]))),
        Operation::Push((1_u8, 10_u8.into())),
        Operation::ExtCodeCopy,
    ]);
}

#[test]
fn push_extcodecopy_partial() {
    assert_snapshot!(vec![
        Operation::Push0,
        Operation::Push((1_u8, 5_u8.into())),
        Operation::Push((20_u8, BigUint::from_bytes_be(&[0xde, 0xad, 0xbe, 0xef]))),
        Operation::Push((1_u8, 5_u8.into())),
        Operation::ExtCodeCopy,
    ]);
}

#[test]
fn push_extcodecopy_out_of_bounds() {
    assert_snapshot!(vec![
        Operation::Push0,
        Operation::Push((1_u8, 50_u8.into())),
        Operation::Push((20_u8, BigUint::from_bytes_be(&[0xde, 0xad, 0xbe, 0xef]))),
        Operation::Push((1_u8, 10_u8.into())),
        Operation::ExtCodeCopy,
    ]);
}

#[test]
fn push_mstore_create_returndatasize() {
    assert_snapshot!(vec![
        Operation::Push((
            32_u8,
            BigUint::from_bytes_be(&[
                0x7F, 0x7F, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF, 0xFF, 0xFF
            ]),
        )),
        Operation::Push0,
        Operation::MStore,
        Operation::Push((
            32_u8,
            BigUint::from_bytes_be(&[
                0xFF, 0x60, 0x00, 0x52, 0x7F, 0xFF, 0x60, 0x00, 0x52, 0x60, 0x20, 0x60, 0x00, 0xF3,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00
            ]),
        )),
        Operation::Push((1_u8, 32_u8.into())),
        Operation::MStore,
        Operation::Push((
            32_u8,
            BigUint::from_bytes_be(&[
                0x00, 0x00, 0x00, 0x00, 0x60, 0x20, 0x52, 0x60, 0x29, 0x60, 0x00, 0xF3, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00
            ]),
        )),
        Operation::Push((1_u8, 64_u8.into())),
        Operation::MStore,
        Operation::Push((1_u8, 77_u8.into())),
        Operation::Push0,
        Operation::Push0,
        Operation::Create,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Dup(5),
        Operation::Push((4_u8, BigUint::from_bytes_be(&[0xFF, 0xFF, 0xFF, 0xFF,]))),
        Operation::Staticcall,
        Operation::ReturndataSize,
    ]);
}

#[test]
fn push_mstore_create_returndatacopy() {
    assert_snapshot!(vec![
        Operation::Push((
            32_u8,
            BigUint::from_bytes_be(&[
                0x7F, 0x7F, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF, 0xFF, 0xFF
            ]),
        )),
        Operation::Push0,
        Operation::MStore,
        Operation::Push((
            32_u8,
            BigUint::from_bytes_be(&[
                0xFF, 0x60, 0x00, 0x52, 0x7F, 0xFF, 0x60, 0x00, 0x52, 0x60, 0x20, 0x60, 0x00, 0xF3,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00
            ]),
        )),
        Operation::Push((1_u8, 32_u8.into())),
        Operation::MStore,
        Operation::Push((
            32_u8,
            BigUint::from_bytes_be(&[
                0x00, 0x00, 0x00, 0x00, 0x60, 0x20, 0x52, 0x60, 0x29, 0x60, 0x00, 0xF3, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00
            ]),
        )),
        Operation::Push((1_u8, 64_u8.into())),
        Operation::MStore,
        Operation::Push((1_u8, 77_u8.into())),
        Operation::Push0,
        Operation::Push0,
        Operation::Create,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Dup(5),
        Operation::Push((4_u8, BigUint::from_bytes_be(&[0xFF, 0xFF, 0xFF, 0xFF,]))),
        Operation::Staticcall,
        Operation::Pop,
        Operation::Pop,
        Operation::Push0,
        Operation::Push0,
        Operation::MStore,
        Operation::Push0,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::MStore,
        Operation::Push0,
        Operation::Push((1_u8, 64_u8.into())),
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Push0,
        Operation::ReturndataCopy,
    ]);
}

#[test]
fn returndatacopy_basic() {
    assert_snapshot!(vec![
        Operation::Push0,                      // Destination offset in memory
        Operation::Push0,                      // Source offset in returndata
        Operation::Push((1_u8, 32_u8.into())), // Number of bytes to copy
        Operation::ReturndataCopy,
    ]);
}

#[test]
fn returndatacopy_partial() {
    assert_snapshot!(vec![
        Operation::Push0,                      // Destination offset in memory
        Operation::Push((1_u8, 10_u8.into())), // Source offset in returndata
        Operation::Push((1_u8, 20_u8.into())), // Number of bytes to copy
        Operation::ReturndataCopy,
    ]);
}

#[test]
fn returndatacopy_out_of_bounds() {
    assert_snapshot!(vec![
        Operation::Push0,                      // Destination offset in memory
        Operation::Push((1_u8, 50_u8.into())), // Offset exceeding returndata size
        Operation::Push((1_u8, 10_u8.into())), // Number of bytes to copy
        Operation::ReturndataCopy,
    ]);
}

#[test]
fn push_mstore_create_extcodehash() {
    assert_snapshot!(vec![
        Operation::Push((
            13_u8,
            BigUint::from_bytes_be(&[
                0x63, 0xFF, 0xFF, 0xFF, 0xFF, 0x60, 0x00, 0x52, 0x60, 0x04, 0x60, 0x00, 0xF3
            ]),
        )),
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 13_u8.into())),
        Operation::Push0,
        Operation::Push0,
        Operation::Create,
        Operation::ExtCodeHash,
    ]);
}

#[test]
fn extcodehash_basic() {
    assert_snapshot!(vec![
        Operation::Push((20_u8, BigUint::from_bytes_be(&[0xde, 0xad, 0xbe, 0xef]))), // External address
        Operation::ExtCodeHash,
    ]);
}

#[test]
fn extcodehash_nonexistent() {
    assert_snapshot!(vec![
        Operation::Push((20_u8, BigUint::from_bytes_be(&[0x00, 0x00, 0x00, 0x00]))), // Nonexistent address
        Operation::ExtCodeHash,
    ]);
}

#[test]
fn blockhash() {
    assert_snapshot!(vec![
        Operation::Push((4, 599423545_u32.into())),
        Operation::BlockHash,
    ]);
}

#[test]
fn coinbase() {
    assert_snapshot!(vec![Operation::Coinbase,]);
}

#[test]
fn timestamp() {
    assert_snapshot!(vec![Operation::Timestamp,]);
}

#[test]
fn number() {
    assert_snapshot!(vec![Operation::Number,]);
}

#[test]
fn prevrandao() {
    assert_snapshot!(vec![Operation::Prevrandao,]);
}

#[test]
fn push_gas_gaslimit() {
    assert_snapshot!(vec![
        Operation::Gas,
        Operation::Push((3_u8, 21000u32.into())),
        Operation::GasLimit,
        Operation::Sub,
        Operation::Sub,
    ]);
}

#[test]
fn chainid() {
    assert_snapshot!(vec![Operation::Chainid,]);
}

#[test]
fn selfbalance() {
    assert_snapshot!(vec![Operation::SelfBalance,]);
}

#[test]
fn basefee() {
    assert_snapshot!(vec![Operation::BaseFee,]);
}

#[test]
fn blobhash() {
    assert_snapshot!(vec![
        Operation::Push((3_u8, 21000u32.into())),
        Operation::BlobHash,
    ]);
}

#[test]
fn blobbasefee() {
    assert_snapshot!(vec![Operation::BlobBaseFee]);
}

#[test]
fn push_pop() {
    assert_snapshot!(vec![
        Operation::Push((3_u8, 125985_u32.into())),
        Operation::Pop,
    ]);
}

#[test]
fn push_mstore_mload() {
    assert_snapshot!(vec![
        Operation::Push((
            32_u8,
            BigUint::from_bytes_be(&[
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0xFF,
            ]),
        )),
        Operation::Push0,
        Operation::MStore,
        Operation::Push0,
        Operation::MLoad,
    ]);
}

#[test]
fn mstore_basic() {
    assert_snapshot!(vec![
        Operation::Push0,
        Operation::Push((32_u8, 42_u64.into())),
        Operation::MStore,
    ]);
}

#[test]
fn mstore_overwrite() {
    assert_snapshot!(vec![
        Operation::Push0,
        Operation::Push((32_u8, 42_u64.into())),
        Operation::MStore,
        Operation::Push0,
        Operation::Push((32_u8, 99_u64.into())),
        Operation::MStore,
    ]);
}

#[test]
fn push_mstore() {
    assert_snapshot!(vec![
        Operation::Push((
            32_u8,
            BigUint::from_bytes_be(&[
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0xFF,
            ]),
        )),
        Operation::Push0,
        Operation::MStore,
    ]);
}

#[test]
fn mstore_high_address() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, 1024_u64.into())),
        Operation::Push((32_u8, 123_u64.into())),
        Operation::MStore,
    ]);
}

#[test]
fn mload_basic() {
    assert_snapshot!(vec![
        Operation::Push0,
        Operation::Push((32_u8, 42_u64.into())),
        Operation::MStore,
        Operation::Push0,
        Operation::MLoad,
    ]);
}

#[test]
fn mload_uninitialized() {
    assert_snapshot!(vec![Operation::Push0, Operation::MLoad,]);
}

#[test]
fn mload_high_address() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, 1024_u64.into())),
        Operation::Push((32_u8, 987654321_u64.into())),
        Operation::MStore,
        Operation::Push((1_u8, 1024_u64.into())),
        Operation::MLoad,
    ]);
}

#[test]
fn push_mstore8() {
    assert_snapshot!(vec![
        Operation::Push((32_u8, BigUint::from_bytes_be(&[0xFF, 0xFF,]))),
        Operation::Push0,
        Operation::MStore8,
    ]);
}

#[test]
fn push_push_sstore_sload() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, 46_u8.into())),
        Operation::Push0,
        Operation::SStore,
        Operation::Push0,
        Operation::SLoad,
    ]);
}

#[test]
fn sstore_basic() {
    assert_snapshot!(vec![
        Operation::Push((32_u8, 0_u64.into())),
        Operation::Push((32_u8, 100_u64.into())),
        Operation::SStore,
    ]);
}

#[test]
fn sstore_overwrite() {
    assert_snapshot!(vec![
        Operation::Push((32_u8, 0_u64.into())),
        Operation::Push((32_u8, 200_u64.into())),
        Operation::SStore,
        Operation::Push((32_u8, 0_u64.into())),
        Operation::Push((32_u8, 300_u64.into())),
        Operation::SStore,
    ]);
}

#[test]
fn sstore_multiple_slots() {
    assert_snapshot!(vec![
        Operation::Push((32_u8, 0_u64.into())),   // Storage slot 0
        Operation::Push((32_u8, 500_u64.into())), // Value to store in slot 0
        Operation::SStore,
        Operation::Push((32_u8, 1_u64.into())), // Storage slot 1
        Operation::Push((32_u8, 600_u64.into())), // Value to store in slot 1
        Operation::SStore,
    ]);
}

#[test]
fn sstore_high_slot() {
    let key = BigUint::from(2_u64).pow(256) - 1_u64;
    assert_snapshot!(vec![
        Operation::Push((32_u8, key)), // High storage slot (max slot)
        Operation::Push((32_u8, 777_u64.into())), // Value to store
        Operation::SStore,
    ]);
}

#[test]
fn sload_basic() {
    assert_snapshot!(vec![
        Operation::Push((32_u8, 0_u64.into())),   // Storage slot
        Operation::Push((32_u8, 400_u64.into())), // Value to store
        Operation::SStore,
        Operation::Push((32_u8, 0_u64.into())), // Same slot
        Operation::SLoad,
    ]);
}

#[test]
fn sload_uninitialized() {
    assert_snapshot!(vec![
        Operation::Push((32_u8, 0_u64.into())), // Attempt to load from uninitialized slot
        Operation::SLoad,
    ]);
}

#[test]
fn sload_high_slot() {
    let key = BigUint::from(2_u64).pow(256) - 1_u64;
    assert_snapshot!(vec![
        Operation::Push((32_u8, key.clone())), // High storage slot (max slot)
        Operation::Push((32_u8, 123_u64.into())), // Value to store
        Operation::SStore,
        Operation::Push((32_u8, key)), // Same high slot
        Operation::SLoad,
    ]);
}

#[test]
fn sload_multiple_slots() {
    assert_snapshot!(vec![
        Operation::Push((32_u8, 0_u64.into())),
        Operation::Push((32_u8, 100_u64.into())),
        Operation::SStore,
        Operation::Push((32_u8, 1_u64.into())),
        Operation::Push((32_u8, 200_u64.into())),
        Operation::SStore,
        Operation::Push((32_u8, 0_u64.into())),
        Operation::SLoad,
        Operation::Push((32_u8, 1_u64.into())),
        Operation::SLoad,
    ]);
}

#[test]
fn push_push_sstore() {
    assert_snapshot!(vec![
        Operation::Push((2_u8, BigUint::from_bytes_be(&[0xFF, 0xFF]))),
        Operation::Push0,
        Operation::SStore,
    ]);
}

#[test]
fn jump_basic() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, 6_u8.into())),
        Operation::Jump,
        Operation::Push((1_u8, 99_u8.into())),
        Operation::Jumpdest { pc: 6 },
        Operation::Push((1_u8, 42_u8.into())),
    ]);
}

#[test]
fn jump_unconditional() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, 4_u8.into())),
        Operation::Push((1_u8, 10_u8.into())),
        Operation::Jump,
        Operation::Push((1_u8, 20_u8.into())),
        Operation::Jumpdest { pc: 4 },
    ]);
}

#[test]
fn jump_with_jumpdest() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, 5_u8.into())),
        Operation::Jump,
        Operation::Push0,
        Operation::Push((1_u8, 1_u8.into())),
        Operation::Jumpdest { pc: 5 },
        Operation::Push((1_u8, 2_u8.into())),
    ]);
}

#[test]
fn jump_invalid() {
    assert_snapshot!(vec![Operation::Push((1_u8, 10_u8.into())), Operation::Jump,]);
}

#[test]
fn jump_with_push_pop() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, 6_u8.into())),
        Operation::Jump,
        Operation::Push0,
        Operation::Push((1_u8, 42_u8.into())),
        Operation::Pop,
        Operation::Jumpdest { pc: 6 },
    ]);
}

#[test]
fn jump_to_jumpdest_twice() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, 4_u8.into())),
        Operation::Jump,
        Operation::Push((1_u8, 99_u8.into())),
        Operation::Jumpdest { pc: 4 },
        Operation::Push((1_u8, 42_u8.into())),
        Operation::Jump,
        Operation::Push((1_u8, 4_u8.into())),
        Operation::Jumpdest { pc: 5 },
    ]);
}

#[test]
fn jump_multiple_destinations() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, 8_u8.into())),
        Operation::Jump,
        Operation::Push((1_u8, 10_u8.into())),
        Operation::Push((1_u8, 20_u8.into())),
        Operation::Jumpdest { pc: 8 },
        Operation::Push((1_u8, 50_u8.into())),
    ]);
}

#[test]
fn jump_invalid_jumpdest() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, 10_u8.into())),
        Operation::Push((1_u8, 99_u8.into())),
        Operation::Jump,
    ]);
}

#[test]
fn push_pop_with_jump() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, 6_u8.into())),
        Operation::Jump,
        Operation::Push((1_u8, 10_u8.into())),
        Operation::Jumpdest { pc: 6 },
        Operation::Push((1_u8, 20_u8.into())),
        Operation::Pop,
    ]);
}

#[test]
fn push_jump_invalid_jumpdest() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, 4_u8.into())),
        Operation::Jump,
        Operation::Invalid,
        Operation::Jumpdest { pc: (1) },
        Operation::Push((1_u8, 1_u8.into())),
    ]);
}

#[test]
fn push_jumpi_valid_jumpdest() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, 5_u8.into())),
        Operation::Push((1_u8, 1_u8.into())),
        Operation::JumpI,
        Operation::Invalid,
        Operation::Jumpdest { pc: (5) },
        Operation::Push((1_u8, 2_u8.into())),
    ]);
}

#[test]
fn pc() {
    assert_snapshot!(vec![
        Operation::PC { pc: 0 },
        Operation::PC { pc: 1 },
        Operation::Jumpdest { pc: 2 },
        Operation::PC { pc: 3 },
        Operation::Push((1_u8, 1_u8.into())),
        Operation::PC { pc: 6 },
    ]);
}

#[test]
fn push_mload_misze() {
    assert_snapshot!(vec![
        Operation::MSize,
        Operation::Push0,
        Operation::MLoad,
        Operation::Pop,
        Operation::MSize,
        Operation::Push((1_u8, 39_u8.into())),
        Operation::MLoad,
        Operation::Pop,
        Operation::MSize,
    ]);
}

#[test]
fn push_gas() {
    assert_snapshot!(vec![
        Operation::Gas,
        Operation::Push((3_u8, 21000_u32.into())),
        Operation::GasLimit,
        Operation::Sub,
        Operation::Sub,
    ]);
}

#[test]
fn push_tstore_tload() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, 46_u8.into())),
        Operation::Push0,
        Operation::TStore,
        Operation::Push0,
        Operation::TLoad,
    ]);
}

#[test]
fn push_tstore_0() {
    assert_snapshot!(vec![
        Operation::Push((2_u8, BigUint::from_bytes_be(&[0xFF, 0xFF]))),
        Operation::Push0,
        Operation::TStore,
    ]);
}

#[test]
fn push_tstore_1() {
    assert_snapshot!(vec![
        Operation::Push((2_u8, BigUint::from_bytes_be(&[0xFF, 0xFF]))),
        Operation::Push((2_u8, 8965u32.into())),
        Operation::TStore,
    ]);
}

#[test]
fn push_mstore_mcopy() {
    assert_snapshot!(vec![
        Operation::Push((
            32_u8,
            BigUint::from_bytes_be(&[
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0xFF,
            ]),
        )),
        Operation::Push((1_u8, 32_u8.into())),
        Operation::MStore,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::MCopy,
    ]);
}

#[test]
fn push_pop_basic() {
    assert_snapshot!(vec![Operation::Push((1_u8, 42_u8.into())), Operation::Pop,]);
}

#[test]
fn push_multiple_pop() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, 1_u8.into())),
        Operation::Push((1_u8, 2_u8.into())),
        Operation::Push((1_u8, 3_u8.into())),
        Operation::Pop,
        Operation::Pop,
        Operation::Pop,
    ]);
}

#[test]
fn push_stack_depth() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, 10_u8.into())),
        Operation::Push((1_u8, 20_u8.into())),
        Operation::Push((1_u8, 30_u8.into())),
        Operation::Push((1_u8, 40_u8.into())),
        Operation::Pop,
        Operation::Pop,
    ]);
}

#[test]
fn push_overflow() {
    let val = BigUint::from(2_u128).pow(256) - 1_u128;
    assert_snapshot!(vec![Operation::Push((32_u8, val)), Operation::Pop,]);
}

#[test]
fn push_dup1() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, 1_u8.into())),
        Operation::Dup(1),
    ]);
}

#[test]
fn push_dup2() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, 1_u8.into())),
        Operation::Push0,
        Operation::Dup(2),
    ]);
}

#[test]
fn push_dup3() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, 1_u8.into())),
        Operation::Push0,
        Operation::Push0,
        Operation::Dup(3),
    ]);
}

#[test]
fn push_dup4() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, 1_u8.into())),
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Dup(4),
    ]);
}

#[test]
fn push_dup5() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, 1_u8.into())),
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Dup(5),
    ]);
}

#[test]
fn push_dup6() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, 1_u8.into())),
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Dup(6),
    ]);
}

#[test]
fn push_dup7() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, 1_u8.into())),
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Dup(7),
    ]);
}

#[test]
fn push_dup8() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, 1_u8.into())),
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Dup(8),
    ]);
}

#[test]
fn push_dup9() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, 1_u8.into())),
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Dup(9),
    ]);
}

#[test]
fn push_dup10() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, 1_u8.into())),
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Dup(10),
    ]);
}

#[test]
fn push_dup11() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, 1_u8.into())),
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Dup(11),
    ]);
}

#[test]
fn push_dup12() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, 1_u8.into())),
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Dup(12),
    ]);
}

#[test]
fn push_dup13() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, 1_u8.into())),
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Dup(13),
    ]);
}

#[test]
fn push_dup14() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, 1_u8.into())),
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Dup(14),
    ]);
}

#[test]
fn push_dup15() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, 1_u8.into())),
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Dup(15),
    ]);
}

#[test]
fn push_dup16() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, 1_u8.into())),
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Dup(16),
    ]);
}

#[test]
fn push_swap1() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, 2_u8.into())),
        Operation::Push((1_u8, 1_u8.into())),
        Operation::Swap(1),
    ]);
}

#[test]
fn push_swap2() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, 2_u8.into())),
        Operation::Push0,
        Operation::Push((1_u8, 1_u8.into())),
        Operation::Swap(2),
    ]);
}

#[test]
fn push_swap3() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, 2_u8.into())),
        Operation::Push0,
        Operation::Push0,
        Operation::Push((1_u8, 1_u8.into())),
        Operation::Swap(3),
    ]);
}

#[test]
fn push_swap4() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, 2_u8.into())),
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push((1_u8, 1_u8.into())),
        Operation::Swap(4),
    ]);
}

#[test]
fn push_swap5() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, 2_u8.into())),
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push((1_u8, 1_u8.into())),
        Operation::Swap(5),
    ]);
}

#[test]
fn push_swap6() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, 2_u8.into())),
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push((1_u8, 1_u8.into())),
        Operation::Swap(6),
    ]);
}

#[test]
fn push_swap7() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, 2_u8.into())),
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push((1_u8, 1_u8.into())),
        Operation::Swap(7),
    ]);
}

#[test]
fn push_swap8() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, 2_u8.into())),
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push((1_u8, 1_u8.into())),
        Operation::Swap(8),
    ]);
}

#[test]
fn push_swap9() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, 2_u8.into())),
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push((1_u8, 1_u8.into())),
        Operation::Swap(9),
    ]);
}

#[test]
fn push_swap10() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, 2_u8.into())),
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push((1_u8, 1_u8.into())),
        Operation::Swap(10),
    ]);
}

#[test]
fn push_swap11() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, 2_u8.into())),
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push((1_u8, 1_u8.into())),
        Operation::Swap(11),
    ]);
}

#[test]
fn push_swap12() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, 2_u8.into())),
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push((1_u8, 1_u8.into())),
        Operation::Swap(12),
    ]);
}

#[test]
fn push_swap13() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, 2_u8.into())),
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push((1_u8, 1_u8.into())),
        Operation::Swap(13),
    ]);
}

#[test]
fn push_swap14() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, 2_u8.into())),
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push((1_u8, 1_u8.into())),
        Operation::Swap(14),
    ]);
}

#[test]
fn push_swap15() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, 2_u8.into())),
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push((1_u8, 1_u8.into())),
        Operation::Swap(15),
    ]);
}

#[test]
fn push_swap16() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, 2_u8.into())),
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push((1_u8, 1_u8.into())),
        Operation::Swap(16),
    ]);
}

#[test]
fn push_create_0() {
    assert_snapshot!(vec![
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Create,
    ]);
}

#[test]
fn push_create_1() {
    assert_snapshot!(vec![
        Operation::Push0,
        Operation::Push0,
        Operation::Push((1_u8, 9_u8.into())),
        Operation::Create,
    ]);
}

#[test]
fn push_create() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, 10_u8.into())),
        Operation::Push0,
        Operation::Push((1_u8, 20_u8.into())),
        Operation::Create,
    ]);
}

#[test]
fn push_create_with_value() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, 100_u8.into())),
        Operation::Push0,
        Operation::Push((1_u8, 40_u8.into())),
        Operation::Create,
    ]);
}

#[test]
fn push_create2_with_salt() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, 0x1234_u16.into())),
        Operation::Push0,
        Operation::Push((1_u8, 20_u8.into())),
        Operation::Push((1_u8, 10_u8.into())),
        Operation::Create2,
    ]);
}

#[test]
fn push_create2_with_large_salt() {
    assert_snapshot!(vec![
        Operation::Push((
            32_u8,
            BigUint::from_bytes_be(&[
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF, 0xFF, 0xFF
            ])
        )),
        Operation::Push0,
        Operation::Push((1_u8, 64_u8.into())),
        Operation::Push((1_u8, 50_u8.into())),
        Operation::Create2,
    ]);
}

#[test]
fn push_create_2() {
    assert_snapshot!(vec![
        Operation::Push((
            13_u8,
            BigUint::from_bytes_be(&[
                0x63, 0xFF, 0xFF, 0xFF, 0x60, 0x00, 0x52, 0x60, 0x04, 0x60, 0x1C, 0xF3,
            ])
        )),
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 13_u8.into())),
        Operation::Push((1_u8, 19_u8.into())),
        Operation::Push0,
        Operation::Create,
    ]);
}

#[test]
fn push_log0() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, 0x40_u8.into())),
        Operation::Push((1_u8, 0x20_u8.into())),
        Operation::Log(0_u8),
    ]);
}

#[test]
fn push_log1() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push((1_u8, 10_u8.into())),
        Operation::Push((1_u8, 0x01_u8.into())),
        Operation::Log(1_u8),
    ]);
}

#[test]
fn push_log2() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push((1_u8, 10_u8.into())),
        Operation::Push((1_u8, 0x01_u8.into())),
        Operation::Push((1_u8, 0x02_u8.into())),
        Operation::Log(2_u8),
    ]);
}

#[test]
fn push_log3() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push((1_u8, 10_u8.into())),
        Operation::Push((1_u8, 0x01_u8.into())),
        Operation::Push((1_u8, 0x02_u8.into())),
        Operation::Push((1_u8, 0x03_u8.into())),
        Operation::Log(3_u8),
    ]);
}

#[test]
fn push_log4() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push((1_u8, 10_u8.into())),
        Operation::Push((1_u8, 0x01_u8.into())),
        Operation::Push((1_u8, 0x02_u8.into())),
        Operation::Push((1_u8, 0x03_u8.into())),
        Operation::Push((1_u8, 0x04_u8.into())),
        Operation::Log(4_u8),
    ]);
}

#[test]
fn push0_dataload() {
    assert_snapshot!(vec![Operation::Push0, Operation::DataLoad], true);
}

#[test]
fn dataloadn() {
    assert_snapshot!(vec![Operation::DataLoadN(0_u16)], true);
}

#[test]
fn datasize() {
    assert_snapshot!(vec![Operation::DataSize], true);
}

#[test]
fn push_datacopy() {
    assert_snapshot!(
        vec![
            Operation::Push((1_u8, 32_u8.into())),
            Operation::Push0,
            Operation::Push0,
            Operation::DataCopy,
        ],
        true
    );
}

#[test]
fn push_datacopy_partial() {
    assert_snapshot!(
        vec![
            Operation::Push0,
            Operation::Push((1_u8, 10_u8.into())),
            Operation::Push((1_u8, 20_u8.into())),
            Operation::DataCopy,
        ],
        true
    );
}

#[test]
fn push_datacopy_out_of_bounds() {
    assert_snapshot!(
        vec![
            Operation::Push0,
            Operation::Push((1_u8, 100_u8.into())),
            Operation::Push((1_u8, 10_u8.into())),
            Operation::DataCopy,
        ],
        true
    );
}

// TODO : `rjump`, `rjumpi`, `rjumpv`, `callf`, `retf` and `jumpf` snapshots

#[test]
fn push_dupn_0() {
    assert_snapshot!(
        vec![Operation::Push((1_u8, 1_u8.into())), Operation::DupN(0_u8),],
        true
    );
}

#[test]
fn push_dupn_16() {
    assert_snapshot!(
        vec![
            Operation::Push((1_u8, 1_u8.into())),
            Operation::Push0,
            Operation::Push0,
            Operation::Push0,
            Operation::Push0,
            Operation::Push0,
            Operation::Push0,
            Operation::Push0,
            Operation::Push0,
            Operation::Push0,
            Operation::Push0,
            Operation::Push0,
            Operation::Push0,
            Operation::Push0,
            Operation::Push0,
            Operation::Push0,
            Operation::Push0,
            Operation::DupN(16_u8),
        ],
        true
    );
}

#[test]
fn push_swapn_0() {
    assert_snapshot!(
        vec![
            Operation::Push((1_u8, 2_u8.into())),
            Operation::Push((1_u8, 1_u8.into())),
            Operation::SwapN(0_u8),
        ],
        true
    );
}

#[test]
fn push_swapn_16() {
    assert_snapshot!(
        vec![
            Operation::Push((1_u8, 2_u8.into())),
            Operation::Push0,
            Operation::Push0,
            Operation::Push0,
            Operation::Push0,
            Operation::Push0,
            Operation::Push0,
            Operation::Push0,
            Operation::Push0,
            Operation::Push0,
            Operation::Push0,
            Operation::Push0,
            Operation::Push0,
            Operation::Push0,
            Operation::Push0,
            Operation::Push0,
            Operation::Push0,
            Operation::Push((1_u8, 1_u8.into())),
            Operation::SwapN(16_u8),
        ],
        true
    );
}

#[test]
fn push_exchange_0() {
    assert_snapshot!(
        vec![
            Operation::Push((1_u8, 1_u8.into())),
            Operation::Push((1_u8, 2_u8.into())),
            Operation::Push0,
            Operation::Exchange(0_u8),
        ],
        true
    );
}

#[test]
fn push_exchange_255() {
    assert_snapshot!(
        vec![
            Operation::Push((1_u8, 1_u8.into())),
            Operation::Push0,
            Operation::Push0,
            Operation::Push0,
            Operation::Push0,
            Operation::Push0,
            Operation::Push0,
            Operation::Push0,
            Operation::Push0,
            Operation::Push0,
            Operation::Push0,
            Operation::Push0,
            Operation::Push0,
            Operation::Push0,
            Operation::Push0,
            Operation::Push0,
            Operation::Push((1_u8, 2_u8.into())),
            Operation::Push0,
            Operation::Push0,
            Operation::Push0,
            Operation::Push0,
            Operation::Push0,
            Operation::Push0,
            Operation::Push0,
            Operation::Push0,
            Operation::Push0,
            Operation::Push0,
            Operation::Push0,
            Operation::Push0,
            Operation::Push0,
            Operation::Push0,
            Operation::Push0,
            Operation::Push0,
            Operation::Exchange(255_u8),
        ],
        true
    );
}

#[test]
fn push_eofcreate_with_salt() {
    assert_snapshot!(
        vec![
            Operation::Push0,
            Operation::Push((1_u8, 20_u8.into())),
            Operation::Push((1_u8, 0x1234_u16.into())),
            Operation::Push((1_u8, 10_u8.into())),
            Operation::EofCreate(0_u8),
        ],
        true
    );
}

#[test]
fn push_eofcreate_with_large_salt() {
    assert_snapshot!(
        vec![
            Operation::Push0,
            Operation::Push((1_u8, 64_u8.into())),
            Operation::Push((
                32_u8,
                BigUint::from_bytes_be(&[
                    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF
                ])
            )),
            Operation::Push((1_u8, 50_u8.into())),
            Operation::EofCreate(0_u8),
        ],
        true
    );
}

#[test]
fn push_returncontract() {
    assert_snapshot!(
        vec![
            Operation::Push0,
            Operation::Push((1_u8, 20_u8.into())),
            Operation::ReturnContract(0_u8),
        ],
        true
    );
}

#[test]
fn push_call() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, 10000_u32.into())),
        Operation::Push((1_u8, 0x1000_u32.into())),
        Operation::Push((1_u8, 1_u32.into())),
        Operation::Push((1_u8, 32_u32.into())),
        Operation::Push((1_u8, 32_u32.into())),
        Operation::Push((1_u8, 64_u32.into())),
        Operation::Push((1_u8, 64_u32.into())),
        Operation::Call,
    ]);
}

#[test]
fn push_mstore_create_call_0() {
    assert_snapshot!(vec![
        Operation::Push((
            17_u8,
            BigUint::from_bytes_be(&[
                0x67, 0x60, 0x00, 0x35, 0x60, 0x07, 0x57, 0xFE, 0x5B, 0x60, 0x00, 0x52, 0x60, 0x08,
                0x60, 0x18, 0xF3,
            ]),
        )),
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 17_u8.into())),
        Operation::Push((1_u8, 15_u8.into())),
        Operation::Push0,
        Operation::Create,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Dup(6),
        Operation::Push((2_u8, BigUint::from_bytes_be(&[0xFF, 0xFF]))),
        Operation::Call,
    ]);
}

#[test]
fn push_mstore_create_call_1() {
    assert_snapshot!(vec![
        Operation::Push((
            17_u8,
            BigUint::from_bytes_be(&[
                0x67, 0x60, 0x00, 0x35, 0x60, 0x07, 0x57, 0xFE, 0x5B, 0x60, 0x00, 0x52, 0x60, 0x08,
                0x60, 0x18, 0xF3,
            ]),
        )),
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 17_u8.into())),
        Operation::Push((1_u8, 15_u8.into())),
        Operation::Push0,
        Operation::Create,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Dup(6),
        Operation::Push((2_u8, BigUint::from_bytes_be(&[0xFF, 0xFF]))),
        Operation::Call,
        Operation::Push0,
        Operation::Push0,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Push0,
        Operation::Dup(7),
        Operation::Push((2_u8, BigUint::from_bytes_be(&[0xFF, 0xFF]))),
        Operation::Call,
    ]);
}

#[test]
fn push_callcode() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, 5000_u32.into())),
        Operation::Push((1_u8, 0x2000_u32.into())),
        Operation::Push((1_u8, 0_u32.into())),
        Operation::Push((1_u8, 32_u32.into())),
        Operation::Push((1_u8, 32_u32.into())),
        Operation::Push((1_u8, 64_u32.into())),
        Operation::Push((1_u8, 64_u32.into())),
        Operation::Callcode,
    ]);
}

#[test]
fn push_mstore_create_callcode() {
    assert_snapshot!(vec![
        Operation::Push((
            17_u8,
            BigUint::from_bytes_be(&[
                0x67, 0x60, 0x00, 0x35, 0x60, 0x07, 0x57, 0xFE, 0x5B, 0x60, 0x00, 0x52, 0x60, 0x08,
                0x60, 0x18, 0xF3,
            ]),
        )),
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 17_u8.into())),
        Operation::Push((1_u8, 15_u8.into())),
        Operation::Push0,
        Operation::Create,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Dup(6),
        Operation::Push((2_u8, BigUint::from_bytes_be(&[0xFF, 0xFF]))),
        Operation::Callcode,
        Operation::Push((1_u8, 1_u8.into())),
        Operation::Push0,
        Operation::SStore,
        Operation::Push0,
        Operation::Push0,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Push0,
        Operation::Dup(7),
        Operation::Push((2_u8, BigUint::from_bytes_be(&[0xFF, 0xFF]))),
        Operation::Callcode,
    ]);
}

#[test]
fn push_return() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Return,
    ]);
}

#[test]
fn push_return_large_data() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, 64_u8.into())),
        Operation::Push0,
        Operation::Return,
    ]);
}

#[test]
fn push_store_return() {
    assert_snapshot!(vec![
        Operation::Push((
            32_u8,
            BigUint::from_bytes_be(&[
                0xFF, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            ]),
        )),
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 2_u8.into())),
        Operation::Push0,
        Operation::Return,
    ]);
}

#[test]
fn push_delegatecall() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, 7000_u32.into())),
        Operation::Push((1_u8, 0x3000_u32.into())),
        Operation::Push((1_u8, 32_u32.into())),
        Operation::Push((1_u8, 32_u32.into())),
        Operation::Push((1_u8, 64_u32.into())),
        Operation::Push((1_u8, 64_u32.into())),
        Operation::Delegatecall,
    ]);
}

#[test]
fn push_mstore_create_delegatecall() {
    assert_snapshot!(vec![
        Operation::Push((
            17_u8,
            BigUint::from_bytes_be(&[
                0x67, 0x60, 0x00, 0x35, 0x60, 0x07, 0x57, 0xFE, 0x5B, 0x60, 0x00, 0x52, 0x60, 0x08,
                0x60, 0x18, 0xF3,
            ]),
        )),
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1_u8, 17_u8.into())),
        Operation::Push((1_u8, 15_u8.into())),
        Operation::Push0,
        Operation::Create,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Dup(5),
        Operation::Push((2_u8, BigUint::from_bytes_be(&[0xFF, 0xFF]))),
        Operation::Delegatecall,
        Operation::Push((1_u8, 1_u8.into())),
        Operation::Push0,
        Operation::SStore,
        Operation::Push0,
        Operation::Push0,
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Dup(6),
        Operation::Push((2_u8, BigUint::from_bytes_be(&[0xFF, 0xFF]))),
        Operation::Delegatecall,
    ]);
}

#[test]
fn push_staticcall() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, 8000_u32.into())),
        Operation::Push((1_u8, 0x4000_u32.into())),
        Operation::Push((1_u8, 32_u32.into())),
        Operation::Push((1_u8, 32_u32.into())),
        Operation::Push((1_u8, 64_u32.into())),
        Operation::Push((1_u8, 64_u32.into())),
        Operation::Staticcall,
    ]);
}

#[test]
fn push_extcall() {
    assert_snapshot!(
        vec![
            Operation::Push((1_u8, 32_u32.into())),
            Operation::Push((1_u8, 1_u32.into())),
            Operation::Push((1_u8, 32_u32.into())),
            Operation::Push((1_u8, 64_u32.into())),
            Operation::ExtCall,
        ],
        true
    );
}

#[test]
fn push_extdelegatecall() {
    assert_snapshot!(
        vec![
            Operation::Push((1_u8, 1_u32.into())),
            Operation::Push((1_u8, 32_u32.into())),
            Operation::Push((1_u8, 64_u32.into())),
            Operation::ExtDelegatecall,
        ],
        true
    );
}

#[test]
fn push_extstaticcall() {
    assert_snapshot!(
        vec![
            Operation::Push((1_u8, 1_u32.into())),
            Operation::Push((1_u8, 32_u32.into())),
            Operation::Push((1_u8, 64_u32.into())),
            Operation::ExtStaticcall,
        ],
        true
    );
}

#[test]
fn returndataload() {
    assert_snapshot!(
        vec![
            Operation::Push((2_u8, 0_u8.into())),
            Operation::ReturndataLoad,
        ],
        true
    );
}

#[test]
fn push_revert() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, 32_u8.into())),
        Operation::Push0,
        Operation::Revert,
    ]);
}

#[test]
fn push_revert_large_data() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, 64_u8.into())),
        Operation::Push0,
        Operation::Revert,
    ]);
}

#[test]
fn push_mstore_revert() {
    assert_snapshot!(vec![
        Operation::Push((
            32_u8,
            BigUint::from_bytes_be(&[
                0xFF, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            ]),
        )),
        Operation::Push((32_u8, 0_u8.into())),
        Operation::MStore,
        Operation::Push((1_u8, 2_u8.into())),
        Operation::Push0,
        Operation::Revert,
    ]);
}

#[test]
fn invalid() {
    assert_snapshot!(vec![Operation::Invalid]);
}

#[test]
fn stop() {
    assert_snapshot!(vec![Operation::Stop]);
}

#[test]
fn push_selfdestruct() {
    assert_snapshot!(vec![
        Operation::Push((
            20_u8,
            BigUint::from_bytes_be(&[
                0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0, 0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC,
                0xDE, 0xF0, 0x12, 0x34, 0x56, 0x78
            ])
        )),
        Operation::Selfdestruct,
    ]);
}

#[test]
fn push_selfdestruct_zero_address() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, 0_u32.into())),
        Operation::Selfdestruct,
    ]);
}

fn biguint_256_from_bigint(value: BigInt) -> BigUint {
    if value >= BigInt::ZERO {
        value.magnitude().clone()
    } else {
        let bytes = value.to_signed_bytes_be();
        let mut buffer: Vec<u8> = vec![255_u8; 32];
        let finish = 32;
        let start = finish - bytes.len();
        buffer[start..finish].copy_from_slice(&bytes);
        BigUint::from_bytes_be(&buffer)
    }
}
