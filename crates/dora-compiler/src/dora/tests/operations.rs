use crate::context::Context;
use crate::evm::program::{Operation, Program};
use crate::evm::{CompileOptions, EVMCompiler};
use crate::Compiler;
use num_bigint::BigInt;
use num_bigint::BigUint;

macro_rules! assert_snapshot {
    ($operations:expr) => {
        let program = Program {
            operations: $operations,
            code_size: 0,
        };
        let context = Context::new();
        let compiler = EVMCompiler::new(&context);
        let mut module = compiler
            .compile(&program, &(), &CompileOptions::default())
            .expect("failed to compile program");
        crate::evm::pass::run(&context.mlir_context, &mut module.mlir_module).unwrap();
        crate::dora::pass::run(
            &context.mlir_context,
            &mut module.mlir_module,
            &crate::dora::pass::PassOptions {
                program_code_size: program.code_size,
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
        Operation::Push((1_u8, BigUint::from(10_u8))),
        Operation::Push((1_u8, BigUint::from(10_u8))),
        Operation::Add,
    ]);
}

#[test]
fn push_push_add_overflow() {
    assert_snapshot!(vec![
        Operation::Push((32_u8, BigUint::from(u128::MAX))),
        Operation::Push((32_u8, BigUint::from(1_u8))),
        Operation::Add,
    ]);
}

#[test]
fn push_push_add_overflow_1() {
    assert_snapshot!(vec![
        Operation::Push((32_u8, BigUint::from(u64::MAX))),
        Operation::Push((32_u8, BigUint::from(1_u64))),
        Operation::Add,
    ]);
}

#[test]
fn push_push_mul() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, BigUint::from(10_u8))),
        Operation::Push((1_u8, BigUint::from(10_u8))),
        Operation::Mul,
    ]);
}

#[test]
fn push_push_mul_large() {
    assert_snapshot!(vec![
        Operation::Push((32_u8, BigUint::from(123456789_u128))),
        Operation::Push((32_u8, BigUint::from(987654321_u128))),
        Operation::Mul,
    ]);
}

#[test]
fn push_push_mul_overflow() {
    assert_snapshot!(vec![
        Operation::Push((32_u8, BigUint::from(u64::MAX))),
        Operation::Push((32_u8, BigUint::from(2_u64))),
        Operation::Mul,
    ]);
}

#[test]
fn push_push_sub() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, BigUint::from(10_u8))),
        Operation::Push((1_u8, BigUint::from(10_u8))),
        Operation::Sub,
    ]);
}

#[test]
fn push_push_sub_underflow() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, BigUint::from(10_u8))),
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Sub,
    ]);
}

#[test]
fn push_push_sub_underflow_1() {
    assert_snapshot!(vec![
        Operation::Push((32_u8, BigUint::from(1_u64))),
        Operation::Push((32_u8, BigUint::from(0_u64))),
        Operation::Sub,
    ]);
}

#[test]
fn push_push_div() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, BigUint::from(10_u8))),
        Operation::Push((1_u8, BigUint::from(10_u8))),
        Operation::Div,
    ]);
}

#[test]
fn push_push_div_zero() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, BigUint::from(10_u8))),
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Div,
    ]);
}

#[test]
fn push_push_mod_zero_1() {
    assert_snapshot!(vec![
        Operation::Push((32_u8, BigUint::from(10_u64))),
        Operation::Push((32_u8, BigUint::from(0_u64))),
        Operation::Mod,
    ]);
}

#[test]
fn push_push_div_zero_0() {
    assert_snapshot!(vec![
        Operation::Push((32_u8, BigUint::from(10_u64))),
        Operation::Push((32_u8, BigUint::from(0_u64))),
        Operation::Div,
    ]);
}

#[test]
fn push_push_sdiv_0() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, BigUint::from(10_u8))),
        Operation::Push((1_u8, BigUint::from(10_u8))),
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
        Operation::Push((1_u8, BigUint::from(3_u8))),
        Operation::Push((1_u8, BigUint::from(10_u8))),
        Operation::Mod,
    ]);
}

#[test]
fn push_push_mod_1() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, BigUint::from(5_u8))),
        Operation::Push((1_u8, BigUint::from(17_u8))),
        Operation::Mod,
    ]);
}

#[test]
fn push_push_smod() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, BigUint::from(3_u8))),
        Operation::Push((1_u8, BigUint::from(10_u8))),
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
        Operation::Push((1_u8, b.clone())),
        Operation::Push((1_u8, a.clone())),
        Operation::SMod,
    ]);
}

#[test]
fn push_push_push_addmod() {
    let (a, b) = (BigUint::from(1_u8), 2_u32);
    assert_snapshot!(vec![
        Operation::Push((1_u8, BigUint::from(b))),
        Operation::Push((1_u8, a.clone())),
        Operation::Push((1_u8, a.clone())),
        Operation::AddMod,
    ]);
}

#[test]
fn push_push_push_addmod_large_mod() {
    assert_snapshot!(vec![
        Operation::Push((32_u8, BigUint::from(u128::MAX))),
        Operation::Push((32_u8, BigUint::from(u128::MAX))),
        Operation::Push((32_u8, BigUint::from(100_u8))),
        Operation::AddMod,
    ]);
}

#[test]
fn push_push_push_mulmod() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, BigUint::from(8_u32))),
        Operation::Push((1_u8, BigUint::from(10_u32))),
        Operation::Push((1_u8, BigUint::from(10_u32))),
        Operation::MulMod,
    ]);
}

#[test]
fn push_push_push_mulmod_zero_mod() {
    assert_snapshot!(vec![
        Operation::Push((32_u8, BigUint::from(2_u8))),
        Operation::Push((32_u8, BigUint::from(2_u8))),
        Operation::Push((32_u8, BigUint::from(0_u8))), // modulus
        Operation::MulMod,
    ]);
}

#[test]
fn push_push_exp() {
    let (a, b) = (BigUint::from(3_u8), 3_u32);
    assert_snapshot!(vec![
        Operation::Push((1_u8, BigUint::from(b))),
        Operation::Push((1_u8, a.clone())),
        Operation::Exp,
    ]);
}

#[test]
fn push_push_exp_edge_case() {
    let (a, b) = (BigUint::from(u128::MAX), 1_u32);
    assert_snapshot!(vec![
        Operation::Push((1_u8, BigUint::from(b))),
        Operation::Push((a.bits() as u8 / 8 + 1, a.clone())),
        Operation::Exp,
    ]);
}

#[test]
fn push_push_exp_large_base() {
    let (a, b) = (BigUint::from(123456789_u64), 5_u32);
    assert_snapshot!(vec![
        Operation::Push((4_u8, BigUint::from(b))),
        Operation::Push((a.bits() as u8 / 8 + 1, a.clone())),
        Operation::Exp,
    ]);
}

#[test]
fn push_push_signextend() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, BigUint::from_bytes_be(&[0xFF]))),
        Operation::Push((1_u8, BigUint::from(0_u32))),
        Operation::SignExtend,
    ]);
}

#[test]
fn push_push_lt() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, BigUint::from(10_u8))),
        Operation::Push((1_u8, BigUint::from(9_u8))),
        Operation::Lt,
    ]);
}

#[test]
fn push_push_gt() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, BigUint::from(10_u8))),
        Operation::Push((1_u8, BigUint::from(9_u8))),
        Operation::Gt,
    ]);
}

#[test]
fn push_push_eq_0() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, BigUint::from(10_u8))),
        Operation::Push((1_u8, BigUint::from(10_u8))),
        Operation::Eq,
    ]);
}

#[test]
fn push_push_eq_1() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, BigUint::from(5_u8))),
        Operation::Push((1_u8, BigUint::from(10_u8))),
        Operation::Eq,
    ]);
}

#[test]
fn push_push_iszero_0() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, BigUint::from(10_u8))),
        Operation::IsZero,
    ]);
}

#[test]
fn push_push_iszero_1() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::IsZero,
    ]);
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
        Operation::Push((1_u8, BigUint::from(0_u8))),
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
        Operation::Push((32_u8, BigUint::from(0_u32))),
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
        Operation::Push((1_u8, BigUint::from(0_u8))),
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
        Operation::Push((32_u8, BigUint::from(0_u32))),
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
        Operation::Push((1_u8, BigUint::from(0_u8))),
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
        Operation::Push((32_u8, BigUint::from(0_u32))),
        Operation::Xor,
    ]);
}

#[test]
fn push_not() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Not,
    ]);
}

#[test]
fn push_push_byte() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, BigUint::from_bytes_be(&[0xFF]))),
        Operation::Push((1_u8, BigUint::from(31_u8))),
        Operation::Byte,
    ]);
}

#[test]
fn push_push_shl() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, BigUint::from(1_u8))),
        Operation::Push((1_u8, BigUint::from(1_u8))),
        Operation::Shl,
    ]);
}

#[test]
fn push_push_shr() {
    let (a, b) = (BigUint::from(3_u8), 3_u32);
    assert_snapshot!(vec![
        Operation::Push((1_u8, BigUint::from(b))),
        Operation::Push((1_u8, a.clone())),
        Operation::Shr,
    ]);
}

#[test]
fn push_push_sar() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, BigUint::from(2_u8))),
        Operation::Push((1_u8, BigUint::from(1_u8))),
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
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::MStore,
        Operation::Push((1_u8, BigUint::from(4_u8))),
        Operation::Push((1_u8, BigUint::from(0_u8))),
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
    assert_snapshot!(vec![
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::CalldataLoad,
    ]);
}

#[test]
fn push_calldataload_edge_case() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, BigUint::from(100_u8))),
        Operation::CalldataLoad,
    ]);
}

#[test]
fn push_calldatasize() {
    assert_snapshot!(vec![Operation::CalldataSize,]);
}

#[test]
fn push_calldatacopy() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, BigUint::from(32_u8))),
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::CalldataCopy,
    ]);
}

#[test]
fn push_calldatacopy_partial() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Push((1_u8, BigUint::from(10_u8))),
        Operation::Push((1_u8, BigUint::from(20_u8))),
        Operation::CalldataCopy,
    ]);
}

#[test]
fn push_calldatacopy_out_of_bounds() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Push((1_u8, BigUint::from(100_u8))),
        Operation::Push((1_u8, BigUint::from(10_u8))),
        Operation::CalldataCopy,
    ]);
}

#[test]
fn push_pop_codesize() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Pop,
        Operation::CodeSize,
    ]);
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
        Operation::Push((32_u8, BigUint::from(0_u8))),
        Operation::Pop,
        Operation::Pop,
        Operation::Push((1_u8, BigUint::from(32_u8))),
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::CodeCopy,
    ]);
}

#[test]
fn push_codecopy_1() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Push((1_u8, BigUint::from(10_u8))),
        Operation::CodeCopy,
    ]);
}

#[test]
fn push_codecopy_partial() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Push((1_u8, BigUint::from(5_u8))),
        Operation::Push((1_u8, BigUint::from(5_u8))),
        Operation::CodeCopy,
    ]);
}

#[test]
fn push_codecopy_out_of_bounds() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Push((1_u8, BigUint::from(50_u8))),
        Operation::Push((1_u8, BigUint::from(10_u8))),
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
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::MStore,
        Operation::Push((
            32_u8,
            BigUint::from_bytes_be(&[
                0x7F, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF,
            ]),
        )),
        Operation::Push((1_u8, BigUint::from(32_u8))),
        Operation::MStore,
        Operation::Push((1_u8, BigUint::from(41_u8))),
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Push((1_u8, BigUint::from(0_u8))),
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
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::MStore,
        Operation::Push((
            32_u8,
            BigUint::from_bytes_be(&[
                0x7F, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF,
            ]),
        )),
        Operation::Push((1_u8, BigUint::from(32_u8))),
        Operation::MStore,
        Operation::Push((1_u8, BigUint::from(41_u8))),
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Create,
        Operation::ExtCodeSize,
        // Return result
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1, 32_u8.into())),
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
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::MStore,
        Operation::Push((
            32_u8,
            BigUint::from_bytes_be(&[
                0x7F, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF,
            ]),
        )),
        Operation::Push((1_u8, BigUint::from(32_u8))),
        Operation::MStore,
        Operation::Push((1_u8, BigUint::from(41_u8))),
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Create,
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::MStore,
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Push((1_u8, BigUint::from(32_u8))),
        Operation::MStore,
        Operation::Push((1_u8, BigUint::from(32_u8))),
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Dup(4),
        Operation::ExtCodeCopy,
    ]);
}

#[test]
fn push_extcodecopy_basic() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Push((20_u8, BigUint::from_bytes_be(&[0xde, 0xad, 0xbe, 0xef]))),
        Operation::Push((1_u8, BigUint::from(10_u8))),
        Operation::ExtCodeCopy,
    ]);
}

#[test]
fn push_extcodecopy_partial() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Push((1_u8, BigUint::from(5_u8))),
        Operation::Push((20_u8, BigUint::from_bytes_be(&[0xde, 0xad, 0xbe, 0xef]))),
        Operation::Push((1_u8, BigUint::from(5_u8))),
        Operation::ExtCodeCopy,
    ]);
}

#[test]
fn push_extcodecopy_out_of_bounds() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Push((1_u8, BigUint::from(50_u8))),
        Operation::Push((20_u8, BigUint::from_bytes_be(&[0xde, 0xad, 0xbe, 0xef]))),
        Operation::Push((1_u8, BigUint::from(10_u8))),
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
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::MStore,
        Operation::Push((
            32_u8,
            BigUint::from_bytes_be(&[
                0xFF, 0x60, 0x00, 0x52, 0x7F, 0xFF, 0x60, 0x00, 0x52, 0x60, 0x20, 0x60, 0x00, 0xF3,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00
            ]),
        )),
        Operation::Push((1_u8, BigUint::from(32_u8))),
        Operation::MStore,
        Operation::Push((
            32_u8,
            BigUint::from_bytes_be(&[
                0x00, 0x00, 0x00, 0x00, 0x60, 0x20, 0x52, 0x60, 0x29, 0x60, 0x00, 0xF3, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00
            ]),
        )),
        Operation::Push((1_u8, BigUint::from(64_u8))),
        Operation::MStore,
        Operation::Push((1_u8, BigUint::from(77_u8))),
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Create,
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Dup(5),
        Operation::Push((4_u8, BigUint::from_bytes_be(&[0xFF, 0xFF, 0xFF, 0xFF,]),)),
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
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::MStore,
        Operation::Push((
            32_u8,
            BigUint::from_bytes_be(&[
                0xFF, 0x60, 0x00, 0x52, 0x7F, 0xFF, 0x60, 0x00, 0x52, 0x60, 0x20, 0x60, 0x00, 0xF3,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00
            ]),
        )),
        Operation::Push((1_u8, BigUint::from(32_u8))),
        Operation::MStore,
        Operation::Push((
            32_u8,
            BigUint::from_bytes_be(&[
                0x00, 0x00, 0x00, 0x00, 0x60, 0x20, 0x52, 0x60, 0x29, 0x60, 0x00, 0xF3, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00
            ]),
        )),
        Operation::Push((1_u8, BigUint::from(64_u8))),
        Operation::MStore,
        Operation::Push((1_u8, BigUint::from(77_u8))),
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Create,
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Dup(5),
        Operation::Push((4_u8, BigUint::from_bytes_be(&[0xFF, 0xFF, 0xFF, 0xFF,]),)),
        Operation::Staticcall,
        Operation::Pop,
        Operation::Pop,
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::MStore,
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Push((1_u8, BigUint::from(32_u8))),
        Operation::MStore,
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Push((1_u8, BigUint::from(64_u8))),
        Operation::MStore,
        Operation::Push((1_u8, BigUint::from(32_u8))),
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::ReturndataCopy,
    ]);
}

#[test]
fn returndatacopy_basic() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, BigUint::from(0_u8))), // Destination offset in memory
        Operation::Push((1_u8, BigUint::from(0_u8))), // Source offset in returndata
        Operation::Push((1_u8, BigUint::from(32_u8))), // Number of bytes to copy
        Operation::ReturndataCopy,
    ]);
}

#[test]
fn returndatacopy_partial() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, BigUint::from(0_u8))), // Destination offset in memory
        Operation::Push((1_u8, BigUint::from(10_u8))), // Source offset in returndata
        Operation::Push((1_u8, BigUint::from(20_u8))), // Number of bytes to copy
        Operation::ReturndataCopy,
    ]);
}

#[test]
fn returndatacopy_out_of_bounds() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, BigUint::from(0_u8))), // Destination offset in memory
        Operation::Push((1_u8, BigUint::from(50_u8))), // Offset exceeding returndata size
        Operation::Push((1_u8, BigUint::from(10_u8))), // Number of bytes to copy
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
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::MStore,
        Operation::Push((1_u8, BigUint::from(13_u8))),
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Push((1_u8, BigUint::from(0_u8))),
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
        Operation::Push((1_u8, BigUint::from(599423545_u32))),
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
        Operation::Push((3_u8, BigUint::from(21000u32))),
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
        Operation::Push((3_u8, BigUint::from(21000u32))),
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
        Operation::Push((3_u8, 125985_u32.into(),)),
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
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::MStore,
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::MLoad,
    ]);
}

#[test]
fn mstore_basic() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Push((32_u8, BigUint::from(42_u64))),
        Operation::MStore,
    ]);
}

#[test]
fn mstore_overwrite() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Push((32_u8, BigUint::from(42_u64))),
        Operation::MStore,
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Push((32_u8, BigUint::from(99_u64))),
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
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::MStore,
    ]);
}

#[test]
fn mstore_high_address() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, BigUint::from(1024_u64))),
        Operation::Push((32_u8, BigUint::from(123_u64))),
        Operation::MStore,
    ]);
}

#[test]
fn mload_basic() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Push((32_u8, BigUint::from(42_u64))),
        Operation::MStore,
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::MLoad,
    ]);
}

#[test]
fn mload_uninitialized() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::MLoad,
    ]);
}

#[test]
fn mload_high_address() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, BigUint::from(1024_u64))),
        Operation::Push((32_u8, BigUint::from(987654321_u64))),
        Operation::MStore,
        Operation::Push((1_u8, BigUint::from(1024_u64))),
        Operation::MLoad,
    ]);
}

#[test]
fn push_mstore8() {
    assert_snapshot!(vec![
        Operation::Push((32_u8, BigUint::from_bytes_be(&[0xFF, 0xFF,]),)),
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::MStore8,
    ]);
}

#[test]
fn push_push_sstore_sload() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, BigUint::from(46_u8))),
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::SStore,
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::SLoad,
    ]);
}

#[test]
fn sstore_basic() {
    assert_snapshot!(vec![
        Operation::Push((32_u8, BigUint::from(0_u64))),
        Operation::Push((32_u8, BigUint::from(100_u64))),
        Operation::SStore,
    ]);
}

#[test]
fn sstore_overwrite() {
    assert_snapshot!(vec![
        Operation::Push((32_u8, BigUint::from(0_u64))),
        Operation::Push((32_u8, BigUint::from(200_u64))),
        Operation::SStore,
        Operation::Push((32_u8, BigUint::from(0_u64))),
        Operation::Push((32_u8, BigUint::from(300_u64))),
        Operation::SStore,
    ]);
}

#[test]
fn sstore_multiple_slots() {
    assert_snapshot!(vec![
        Operation::Push((32_u8, BigUint::from(0_u64))), // Storage slot 0
        Operation::Push((32_u8, BigUint::from(500_u64))), // Value to store in slot 0
        Operation::SStore,
        Operation::Push((32_u8, BigUint::from(1_u64))), // Storage slot 1
        Operation::Push((32_u8, BigUint::from(600_u64))), // Value to store in slot 1
        Operation::SStore,
    ]);
}

#[test]
fn sstore_high_slot() {
    let key = BigUint::from(2_u64).pow(256) - 1_u64;
    assert_snapshot!(vec![
        Operation::Push((32_u8, key)), // High storage slot (max slot)
        Operation::Push((32_u8, BigUint::from(777_u64))), // Value to store
        Operation::SStore,
    ]);
}

#[test]
fn sload_basic() {
    assert_snapshot!(vec![
        Operation::Push((32_u8, BigUint::from(0_u64))), // Storage slot
        Operation::Push((32_u8, BigUint::from(400_u64))), // Value to store
        Operation::SStore,
        Operation::Push((32_u8, BigUint::from(0_u64))), // Same slot
        Operation::SLoad,
    ]);
}

#[test]
fn sload_uninitialized() {
    assert_snapshot!(vec![
        Operation::Push((32_u8, BigUint::from(0_u64))), // Attempt to load from uninitialized slot
        Operation::SLoad,
    ]);
}

#[test]
fn sload_high_slot() {
    let key = BigUint::from(2_u64).pow(256) - 1_u64;
    assert_snapshot!(vec![
        Operation::Push((32_u8, key.clone())), // High storage slot (max slot)
        Operation::Push((32_u8, BigUint::from(123_u64))), // Value to store
        Operation::SStore,
        Operation::Push((32_u8, key)), // Same high slot
        Operation::SLoad,
    ]);
}

#[test]
fn sload_multiple_slots() {
    assert_snapshot!(vec![
        Operation::Push((32_u8, BigUint::from(0_u64))),
        Operation::Push((32_u8, BigUint::from(100_u64))),
        Operation::SStore,
        Operation::Push((32_u8, BigUint::from(1_u64))),
        Operation::Push((32_u8, BigUint::from(200_u64))),
        Operation::SStore,
        Operation::Push((32_u8, BigUint::from(0_u64))),
        Operation::SLoad,
        Operation::Push((32_u8, BigUint::from(1_u64))),
        Operation::SLoad,
    ]);
}

#[test]
fn push_push_sstore() {
    assert_snapshot!(vec![
        Operation::Push((2_u8, BigUint::from_bytes_be(&[0xFF, 0xFF]))),
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::SStore,
    ]);
}

#[test]
fn jump_basic() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, BigUint::from(6_u8))),
        Operation::Jump,
        Operation::Push((1_u8, BigUint::from(99_u8))),
        Operation::Jumpdest { pc: 6 },
        Operation::Push((1_u8, BigUint::from(42_u8))),
    ]);
}

#[test]
fn jump_unconditional() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, BigUint::from(4_u8))),
        Operation::Push((1_u8, BigUint::from(10_u8))),
        Operation::Jump,
        Operation::Push((1_u8, BigUint::from(20_u8))),
        Operation::Jumpdest { pc: 4 },
    ]);
}

#[test]
fn jump_with_jumpdest() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, BigUint::from(5_u8))),
        Operation::Jump,
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Push((1_u8, BigUint::from(1_u8))),
        Operation::Jumpdest { pc: 5 },
        Operation::Push((1_u8, BigUint::from(2_u8))),
    ]);
}

#[test]
fn jump_invalid() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, BigUint::from(10_u8))),
        Operation::Jump,
    ]);
}

#[test]
fn jump_with_push_pop() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, BigUint::from(6_u8))),
        Operation::Jump,
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Push((1_u8, BigUint::from(42_u8))),
        Operation::Pop,
        Operation::Jumpdest { pc: 6 },
    ]);
}

#[test]
fn jump_to_jumpdest_twice() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, BigUint::from(4_u8))),
        Operation::Jump,
        Operation::Push((1_u8, BigUint::from(99_u8))),
        Operation::Jumpdest { pc: 4 },
        Operation::Push((1_u8, BigUint::from(42_u8))),
        Operation::Jump,
        Operation::Push((1_u8, BigUint::from(4_u8))),
        Operation::Jumpdest { pc: 5 },
    ]);
}

#[test]
fn jump_multiple_destinations() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, BigUint::from(8_u8))),
        Operation::Jump,
        Operation::Push((1_u8, BigUint::from(10_u8))),
        Operation::Push((1_u8, BigUint::from(20_u8))),
        Operation::Jumpdest { pc: 8 },
        Operation::Push((1_u8, BigUint::from(50_u8))),
    ]);
}

#[test]
fn jump_invalid_jumpdest() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, BigUint::from(10_u8))),
        Operation::Push((1_u8, BigUint::from(99_u8))),
        Operation::Jump,
    ]);
}

#[test]
fn push_pop_with_jump() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, BigUint::from(6_u8))),
        Operation::Jump,
        Operation::Push((1_u8, BigUint::from(10_u8))),
        Operation::Jumpdest { pc: 6 },
        Operation::Push((1_u8, BigUint::from(20_u8))),
        Operation::Pop,
    ]);
}

#[test]
fn push_jump_invalid_jumpdest() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, BigUint::from(4_u8))),
        Operation::Jump,
        Operation::Invalid,
        Operation::Jumpdest { pc: (1) },
        Operation::Push((1_u8, BigUint::from(1_u8))),
    ]);
}

#[test]
fn push_jumpi_valid_jumpdest() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, BigUint::from(5_u8))),
        Operation::Push((1_u8, BigUint::from(1_u8))),
        Operation::Jumpi,
        Operation::Invalid,
        Operation::Jumpdest { pc: (5) },
        Operation::Push((1_u8, BigUint::from(2_u8))),
    ]);
}

#[test]
fn pc() {
    assert_snapshot!(vec![
        Operation::PC { pc: 0 },
        Operation::PC { pc: 1 },
        Operation::Jumpdest { pc: 2 },
        Operation::PC { pc: 3 },
        Operation::Push((1_u8, BigUint::from(1_u8))),
        Operation::PC { pc: 6 },
    ]);
}

#[test]
fn push_mload_misze() {
    assert_snapshot!(vec![
        Operation::MSize,
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::MLoad,
        Operation::Pop,
        Operation::MSize,
        Operation::Push((1_u8, BigUint::from(39_u8))),
        Operation::MLoad,
        Operation::Pop,
        Operation::MSize,
    ]);
}

#[test]
fn push_gas() {
    assert_snapshot!(vec![
        Operation::Gas,
        Operation::Push((3_u8, BigUint::from(21000_u32))),
        Operation::GasLimit,
        Operation::Sub,
        Operation::Sub,
    ]);
}

#[test]
fn push_tstore_tload() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, BigUint::from(46_u8))),
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::TStore,
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::TLoad,
    ]);
}

#[test]
fn push_tstore_0() {
    assert_snapshot!(vec![
        Operation::Push((2_u8, BigUint::from_bytes_be(&[0xFF, 0xFF]))),
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::TStore,
    ]);
}

#[test]
fn push_tstore_1() {
    assert_snapshot!(vec![
        Operation::Push((2_u8, BigUint::from_bytes_be(&[0xFF, 0xFF]))),
        Operation::Push((2_u8, BigUint::from(8965u32))),
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
        Operation::Push((1_u8, BigUint::from(32_u8))),
        Operation::MStore,
        Operation::Push((1_u8, BigUint::from(32_u8))),
        Operation::Push((1_u8, BigUint::from(32_u8))),
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::MCopy,
    ]);
}

#[test]
fn push_pop_basic() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, BigUint::from(42_u8))),
        Operation::Pop,
    ]);
}

#[test]
fn push_multiple_pop() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, BigUint::from(1_u8))),
        Operation::Push((1_u8, BigUint::from(2_u8))),
        Operation::Push((1_u8, BigUint::from(3_u8))),
        Operation::Pop,
        Operation::Pop,
        Operation::Pop,
    ]);
}

#[test]
fn push_stack_depth() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, BigUint::from(10_u8))),
        Operation::Push((1_u8, BigUint::from(20_u8))),
        Operation::Push((1_u8, BigUint::from(30_u8))),
        Operation::Push((1_u8, BigUint::from(40_u8))),
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
        Operation::Push((1_u8, BigUint::from(1_u8))),
        Operation::Dup(1),
    ]);
}

#[test]
fn push_dup2() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, BigUint::from(1_u8))),
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Dup(2),
    ]);
}

#[test]
fn push_swap1() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, BigUint::from(2_u8))),
        Operation::Push((1_u8, BigUint::from(1_u8))),
        Operation::Swap(1),
    ]);
}

#[test]
fn push_swap2() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, BigUint::from(2_u8))),
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Push((1_u8, BigUint::from(1_u8))),
        Operation::Swap(2),
    ]);
}

#[test]
fn push_create_0() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Create,
    ]);
}

#[test]
fn push_create_1() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Push((1_u8, BigUint::from(9_u8))),
        Operation::Create,
    ]);
}

#[test]
fn push_create() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, BigUint::from(10_u8))),
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Push((1_u8, BigUint::from(20_u8))),
        Operation::Create,
    ]);
}

#[test]
fn push_create_with_value() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, BigUint::from(100_u8))),
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Push((1_u8, BigUint::from(40_u8))),
        Operation::Create,
    ]);
}

#[test]
fn push_create2_with_salt() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, BigUint::from(0x1234_u16))),
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Push((1_u8, BigUint::from(20_u8))),
        Operation::Push((1_u8, BigUint::from(10_u8))),
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
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Push((1_u8, BigUint::from(64_u8))),
        Operation::Push((1_u8, BigUint::from(50_u8))),
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
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::MStore,
        Operation::Push((1_u8, BigUint::from(13_u8))),
        Operation::Push((1_u8, BigUint::from(19_u8))),
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Create,
    ]);
}

#[test]
fn log0() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, BigUint::from(0x40_u8))),
        Operation::Push((1_u8, BigUint::from(0x20_u8))),
        Operation::Log(0),
    ]);
}

#[test]
fn log1() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, BigUint::from(32_u8))),
        Operation::Push((1_u8, BigUint::from(10_u8))),
        Operation::Push((1_u8, BigUint::from(0x01_u8))),
        Operation::Log(1),
    ]);
}

#[test]
fn log2() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, BigUint::from(32_u8))),
        Operation::Push((1_u8, BigUint::from(10_u8))),
        Operation::Push((1_u8, BigUint::from(0x01_u8))),
        Operation::Push((1_u8, BigUint::from(0x02_u8))),
        Operation::Log(2),
    ]);
}

#[test]
fn log3() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, BigUint::from(32_u8))),
        Operation::Push((1_u8, BigUint::from(10_u8))),
        Operation::Push((1_u8, BigUint::from(0x01_u8))),
        Operation::Push((1_u8, BigUint::from(0x02_u8))),
        Operation::Push((1_u8, BigUint::from(0x03_u8))),
        Operation::Log(3),
    ]);
}

#[test]
fn log4() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, BigUint::from(32_u8))),
        Operation::Push((1_u8, BigUint::from(10_u8))),
        Operation::Push((1_u8, BigUint::from(0x01_u8))),
        Operation::Push((1_u8, BigUint::from(0x02_u8))),
        Operation::Push((1_u8, BigUint::from(0x03_u8))),
        Operation::Push((1_u8, BigUint::from(0x04_u8))),
        Operation::Log(4),
    ]);
}

#[test]
fn push_call() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, BigUint::from(10000_u32))),
        Operation::Push((1_u8, BigUint::from(0x1000_u32))),
        Operation::Push((1_u8, BigUint::from(1_u32))),
        Operation::Push((1_u8, BigUint::from(32_u32))),
        Operation::Push((1_u8, BigUint::from(32_u32))),
        Operation::Push((1_u8, BigUint::from(64_u32))),
        Operation::Push((1_u8, BigUint::from(64_u32))),
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
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::MStore,
        Operation::Push((1_u8, BigUint::from(17_u8))),
        Operation::Push((1_u8, BigUint::from(15_u8))),
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Create,
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Push((1_u8, BigUint::from(0_u8))),
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
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::MStore,
        Operation::Push((1_u8, BigUint::from(17_u8))),
        Operation::Push((1_u8, BigUint::from(15_u8))),
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Create,
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Dup(6),
        Operation::Push((2_u8, BigUint::from_bytes_be(&[0xFF, 0xFF]))),
        Operation::Call,
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Push((1_u8, BigUint::from(32_u8))),
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Dup(7),
        Operation::Push((2_u8, BigUint::from_bytes_be(&[0xFF, 0xFF]))),
        Operation::Call,
    ]);
}

#[test]
fn push_callcode() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, BigUint::from(5000_u32))),
        Operation::Push((1_u8, BigUint::from(0x2000_u32))),
        Operation::Push((1_u8, BigUint::from(0_u32))),
        Operation::Push((1_u8, BigUint::from(32_u32))),
        Operation::Push((1_u8, BigUint::from(32_u32))),
        Operation::Push((1_u8, BigUint::from(64_u32))),
        Operation::Push((1_u8, BigUint::from(64_u32))),
        Operation::CallCode,
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
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::MStore,
        Operation::Push((1_u8, BigUint::from(17_u8))),
        Operation::Push((1_u8, BigUint::from(15_u8))),
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Create,
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Dup(6),
        Operation::Push((2_u8, BigUint::from_bytes_be(&[0xFF, 0xFF]))),
        Operation::CallCode,
        Operation::Push((1_u8, BigUint::from(1_u8))),
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::SStore,
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Push((1_u8, BigUint::from(32_u8))),
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Dup(7),
        Operation::Push((2_u8, BigUint::from_bytes_be(&[0xFF, 0xFF]))),
        Operation::CallCode,
    ]);
}

#[test]
fn push_return() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, BigUint::from(32_u8))),
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Return,
    ]);
}

#[test]
fn push_return_large_data() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, BigUint::from(64_u8))),
        Operation::Push((1_u8, BigUint::from(0_u8))),
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
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::MStore,
        Operation::Push((1_u8, BigUint::from(2_u8))),
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Return,
    ]);
}

#[test]
fn push_delegatecall() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, BigUint::from(7000_u32))),
        Operation::Push((1_u8, BigUint::from(0x3000_u32))),
        Operation::Push((1_u8, BigUint::from(32_u32))),
        Operation::Push((1_u8, BigUint::from(32_u32))),
        Operation::Push((1_u8, BigUint::from(64_u32))),
        Operation::Push((1_u8, BigUint::from(64_u32))),
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
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::MStore,
        Operation::Push((1_u8, BigUint::from(17_u8))),
        Operation::Push((1_u8, BigUint::from(15_u8))),
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Create,
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Dup(5),
        Operation::Push((2_u8, BigUint::from_bytes_be(&[0xFF, 0xFF]))),
        Operation::Delegatecall,
        Operation::Push((1_u8, BigUint::from(1_u8))),
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::SStore,
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Push((1_u8, BigUint::from(32_u8))),
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Dup(6),
        Operation::Push((2_u8, BigUint::from_bytes_be(&[0xFF, 0xFF]))),
        Operation::Delegatecall,
    ]);
}

#[test]
fn push_staticcall() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, BigUint::from(8000_u32))),
        Operation::Push((1_u8, BigUint::from(0x4000_u32))),
        Operation::Push((1_u8, BigUint::from(32_u32))),
        Operation::Push((1_u8, BigUint::from(32_u32))),
        Operation::Push((1_u8, BigUint::from(64_u32))),
        Operation::Push((1_u8, BigUint::from(64_u32))),
        Operation::Staticcall,
    ]);
}

#[test]
fn push_revert() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, BigUint::from(32_u8))),
        Operation::Push((1_u8, BigUint::from(0_u8))),
        Operation::Revert,
    ]);
}

#[test]
fn push_revert_large_data() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, BigUint::from(64_u8))),
        Operation::Push((1_u8, BigUint::from(0_u8))),
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
        Operation::Push((32_u8, BigUint::from(0_u8))),
        Operation::MStore,
        Operation::Push((1_u8, BigUint::from(2_u8))),
        Operation::Push((1_u8, BigUint::from(0_u8))),
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
        Operation::SelfDestruct,
    ]);
}

#[test]
fn push_selfdestruct_zero_address() {
    assert_snapshot!(vec![
        Operation::Push((1_u8, BigUint::from(0_u32))),
        Operation::SelfDestruct,
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
