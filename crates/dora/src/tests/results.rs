use bytes::Bytes;
use dora_compiler::evm::program::Operation;
use dora_primitives::{spec::SpecId, Address, Bytes32, B256, U256};
use dora_runtime::{
    constants::MAX_STACK_SIZE,
    context::{Log, LogData},
};

use crate::tests::utils::{run_result, run_result_with_spec};

#[test]
fn empty() {
    let operations = vec![];
    let result = run_result(operations);
    assert!(result.is_success());
    assert_eq!(result.output(), None);
    assert_eq!(result.gas_used(), 0)
}

#[test]
fn stop() {
    let operations = vec![Operation::Stop];
    let result = run_result(operations);
    assert!(result.is_success());
    assert_eq!(result.output(), None);
    assert_eq!(result.gas_used(), 0);
}

#[test]
fn push0_no_stop() {
    let operations = vec![Operation::Push0];
    let result = run_result(operations);
    assert!(result.is_success());
    assert_eq!(result.output(), None);
    assert_eq!(result.gas_used(), 2)
}

#[test]
fn invalid_no_stop() {
    let operations = vec![Operation::Invalid];
    let result = run_result(operations);
    assert!(result.is_halt());
    assert_eq!(result.output(), None);
    assert_eq!(result.gas_used(), 0)
}

#[test]
fn push0_stack_not_overflow() {
    let not_overflow_stack_size = MAX_STACK_SIZE;
    let operations = vec![Operation::Push0; not_overflow_stack_size];
    let result = run_result(operations);
    assert!(result.is_success());
    assert_eq!(result.output(), None);
    assert_eq!(result.gas_used(), 2 * not_overflow_stack_size as u64);
}

#[test]
fn push0_stack_overflow() {
    let overflow_stack_size = MAX_STACK_SIZE + 1;
    let operations = vec![Operation::Push0; overflow_stack_size];
    let result = run_result(operations);
    assert!(result.halt_reason().unwrap().is_stack_overflow());
    assert_eq!(result.output(), None);
    assert_eq!(result.gas_used(), 2 * overflow_stack_size as u64);
}

#[test]
fn push1_stack_overflow() {
    let overflow_stack_size = MAX_STACK_SIZE + 1;
    let operations = vec![Operation::Push((1, 10_u8.into())); overflow_stack_size];
    let result = run_result(operations);
    assert!(result.halt_reason().unwrap().is_stack_overflow());
    assert_eq!(result.output(), None);
    assert_eq!(result.gas_used(), 3 * overflow_stack_size as u64);
}

#[test]
fn add_stack_underflow() {
    let operations = vec![Operation::Add];
    let result = run_result(operations);
    assert!(result.halt_reason().unwrap().is_stack_underflow());
    assert_eq!(result.output(), None);
    assert_eq!(result.gas_used(), 3)
}

#[test]
fn push0_add_stack_underflow() {
    let operations = vec![Operation::Push0, Operation::Add];
    let result = run_result(operations);
    assert!(result.halt_reason().unwrap().is_stack_underflow());
    assert_eq!(result.output(), None);
    assert_eq!(result.gas_used(), 5)
}

#[test]
fn push0_pop_add_stack_underflow() {
    let operations = vec![Operation::Push0, Operation::Pop, Operation::Add];
    let result = run_result(operations);
    assert!(result.halt_reason().unwrap().is_stack_underflow());
    assert_eq!(result.output(), None);
    assert_eq!(result.gas_used(), 7)
}

#[test]
fn push0_add_pop_stack_underflow() {
    let operations = vec![Operation::Push0, Operation::Add, Operation::Pop];
    let result = run_result(operations);
    assert!(result.halt_reason().unwrap().is_stack_underflow());
    assert_eq!(result.output(), None);
    assert_eq!(result.gas_used(), 5)
}

#[test]
fn sub_stack_underflow() {
    let operations = vec![Operation::Sub];
    let result = run_result(operations);
    assert!(result.halt_reason().unwrap().is_stack_underflow());
    assert_eq!(result.output(), None);
    assert_eq!(result.gas_used(), 3)
}

#[test]
fn mul_stack_underflow() {
    let operations = vec![Operation::Mul];
    let result = run_result(operations);
    assert!(result.halt_reason().unwrap().is_stack_underflow());
    assert_eq!(result.output(), None);
    assert_eq!(result.gas_used(), 5)
}

#[test]
fn div_stack_underflow() {
    let operations = vec![Operation::Div];
    let result = run_result(operations);
    assert!(result.halt_reason().unwrap().is_stack_underflow());
    assert_eq!(result.output(), None);
    assert_eq!(result.gas_used(), 5)
}

#[test]
fn sdiv_stack_underflow() {
    let operations = vec![Operation::SDiv];
    let result = run_result(operations);
    assert!(result.halt_reason().unwrap().is_stack_underflow());
    assert_eq!(result.output(), None);
    assert_eq!(result.gas_used(), 5)
}

#[test]
fn mod_stack_underflow() {
    let operations = vec![Operation::Mod];
    let result = run_result(operations);
    assert!(result.halt_reason().unwrap().is_stack_underflow());
    assert_eq!(result.output(), None);
    assert_eq!(result.gas_used(), 5)
}

#[test]
fn smod_stack_underflow() {
    let operations = vec![Operation::SMod];
    let result = run_result(operations);
    assert!(result.halt_reason().unwrap().is_stack_underflow());
    assert_eq!(result.output(), None);
    assert_eq!(result.gas_used(), 5)
}

#[test]
fn addmod_stack_underflow() {
    let operations = vec![Operation::AddMod];
    let result = run_result(operations);
    assert!(result.halt_reason().unwrap().is_stack_underflow());
    assert_eq!(result.output(), None);
    assert_eq!(result.gas_used(), 8)
}

#[test]
fn mulmod_stack_underflow() {
    let operations = vec![Operation::MulMod];
    let result = run_result(operations);
    assert!(result.halt_reason().unwrap().is_stack_underflow());
    assert_eq!(result.output(), None);
    assert_eq!(result.gas_used(), 8)
}

#[test]
fn exp_stack_underflow() {
    let operations = vec![Operation::Exp];
    let result = run_result(operations);
    assert!(result.halt_reason().unwrap().is_stack_underflow());
    assert_eq!(result.output(), None);
    assert_eq!(result.gas_used(), 10)
}

#[test]
fn signextend_stack_underflow() {
    let operations = vec![Operation::SignExtend];
    let result = run_result(operations);
    assert!(result.halt_reason().unwrap().is_stack_underflow());
    assert_eq!(result.output(), None);
    assert_eq!(result.gas_used(), 5)
}

#[test]
fn jump_stack_underflow() {
    let operations = vec![Operation::Jump];
    let result = run_result(operations);
    assert!(result.halt_reason().unwrap().is_stack_underflow());
    assert_eq!(result.output(), None);
    assert_eq!(result.gas_used(), 8)
}

#[test]
fn jumpi_stack_underflow() {
    let operations = vec![Operation::Jumpi];
    let result = run_result(operations);
    assert!(result.halt_reason().unwrap().is_stack_underflow());
    assert_eq!(result.output(), None);
    assert_eq!(result.gas_used(), 10)
}

#[test]
fn push0_jumpi_stack_underflow() {
    let operations = vec![Operation::Push0, Operation::Jumpi];
    let result = run_result(operations);
    assert!(result.halt_reason().unwrap().is_stack_underflow());
    assert_eq!(result.output(), None);
    assert_eq!(result.gas_used(), 2 + 10)
}

#[test]
fn exp_1() {
    let operations = vec![
        Operation::Push((1, 0_u32.into())),
        Operation::Push((1, 0_u32.into())),
        Operation::Exp,
    ];
    let result = run_result(operations);
    assert!(result.is_success());
    assert_eq!(result.output(), None);
    assert_eq!(result.gas_used(), 3 + 3 + 10)
}

#[test]
fn exp_2() {
    let operations = vec![
        Operation::Push((1, 0_u32.into())),
        Operation::Push((1, 2_u32.into())),
        Operation::Exp,
    ];
    let result = run_result(operations);
    assert!(result.is_success());
    assert_eq!(result.output(), None);
    assert_eq!(result.gas_used(), 3 + 3 + 10)
}

#[test]
fn exp_3() {
    let operations = vec![
        Operation::Push((1, 1_u32.into())),
        Operation::Push((1, 2_u32.into())),
        Operation::Exp,
    ];
    let result = run_result(operations);
    assert!(result.is_success());
    assert_eq!(result.output(), None);
    // 50 is the dynamic gas
    assert_eq!(result.gas_used(), 3 + 3 + 10 + 50)
}

#[test]
fn exp_overflow() {
    let operations = vec![
        Operation::Push((2, 256_u32.into())),
        Operation::Push((1, 2_u32.into())),
        Operation::Exp,
    ];
    let result = run_result(operations);
    assert!(result.is_success());
    assert_eq!(result.output(), None);
    // 100 is the dynamic gas
    assert_eq!(result.gas_used(), 3 + 3 + 10 + 100)
}

#[test]
fn push0_not_found_in_merge_spec() {
    let operations = vec![Operation::Push0];
    let result = run_result_with_spec(operations, SpecId::MERGE);
    assert!(result.halt_reason().unwrap().is_opcode_not_found());
    assert_eq!(result.output(), None);
    assert_eq!(result.gas_used(), 0)
}

#[test]
fn push0_found_in_shanghai_spec() {
    let operations = vec![Operation::Push0];
    let result = run_result_with_spec(operations, SpecId::SHANGHAI);
    assert!(result.is_success());
    assert_eq!(result.output(), None);
    assert_eq!(result.gas_used(), 2)
}

#[test]
fn stack_push1_pop() {
    let operations = vec![Operation::Push((1, 1_u8.into())), Operation::Pop];
    let result = run_result(operations);
    assert!(result.is_success());
    assert_eq!(result.output(), None);
    assert_eq!(result.gas_used(), 3 + 2)
}

#[test]
fn stack_push1_dup1() {
    let operations = vec![Operation::Push((1, 1_u8.into())), Operation::Dup(1)];
    let result = run_result(operations);
    assert!(result.is_success());
    assert_eq!(result.output(), None);
    assert_eq!(result.gas_used(), 3 + 3)
}

#[test]
fn stack_push1_dupn() {
    let operations = vec![
        Operation::Push((1, 1_u8.into())),
        Operation::Dup(3),
        Operation::Stop,
    ];
    let result = run_result(operations);
    assert!(result.is_success());
    assert_eq!(result.output(), None);
    assert_eq!(result.gas_used(), 3 + 3)
}

#[test]
fn stack_push1_push1_swap1() {
    let operations = vec![
        Operation::Push((1, 1_u8.into())),
        Operation::Push((1, 1_u8.into())),
        Operation::Swap(1),
    ];
    let result = run_result(operations);
    assert!(result.is_success());
    assert_eq!(result.output(), None);
    assert_eq!(result.gas_used(), 3 + 3 + 3)
}

#[test]
fn stack_push1_push1_swap2() {
    let operations = vec![
        Operation::Push((1, 1_u8.into())),
        Operation::Push((1, 1_u8.into())),
        Operation::Push((1, 1_u8.into())),
        Operation::Swap(1),
    ];
    let result = run_result(operations);
    assert!(result.is_success());
    assert_eq!(result.output(), None);
    assert_eq!(result.gas_used(), 3 + 3 + 3 + 3)
}

#[test]
fn basic_jump() {
    let operations = vec![
        Operation::Push((1, 3_u8.into())),
        Operation::Jump,
        Operation::Jumpdest { pc: 3 },
        Operation::Push((1, 69_u8.into())),
        Operation::Stop,
    ];
    let result = run_result(operations);
    assert!(result.is_success());
    assert_eq!(result.output(), None);
    assert_eq!(result.gas_used(), 3 + 8 + 1 + 3)
}

#[test]
fn basic_jumpi() {
    let operations = vec![
        Operation::Jumpdest { pc: 1 },
        Operation::Push0,
        Operation::Push0,
        Operation::Jumpi,
        Operation::Push((1, 1_u8.into())),
    ];
    let result = run_result(operations);
    assert!(result.is_success());
    assert_eq!(result.output(), None);
    assert_eq!(result.gas_used(), 1 + 2 + 2 + 10 + 3)
}

#[test]
fn basic_jumpi_2() {
    let operations = vec![
        Operation::Push((1, 1_u8.into())),
        Operation::Push((1, 5_u8.into())),
        Operation::Jumpi,
        Operation::Jumpdest { pc: 4 },
        Operation::Push((1, 6_u8.into())),
    ];
    let result = run_result(operations);
    assert!(result.is_success());
    assert_eq!(result.output(), None);
    assert_eq!(result.gas_used(), 3 + 3 + 10 + 1 + 3)
}

#[test]
fn jumpi_invalid_target() {
    let operations = vec![
        Operation::Push0,
        Operation::Push0,
        Operation::Jumpi,
        Operation::Push((1, 1_u8.into())),
    ];
    let result = run_result(operations);
    assert!(result.is_success());
    assert_eq!(result.output(), None);
    assert_eq!(result.gas_used(), 2 + 2 + 10 + 3)
}

#[test]
fn jumpi_invalid_target_2() {
    let operations = vec![
        Operation::Push((1, 1_u8.into())),
        Operation::Push0,
        Operation::Jumpi,
        Operation::Jumpdest { pc: 4 },
        Operation::Push((1, 6_u8.into())),
    ];
    let result = run_result(operations);
    assert!(result.halt_reason().unwrap().is_invalid_jump());
    assert_eq!(result.output(), None);
    assert_eq!(result.gas_used(), 3 + 2 + 10)
}

#[test]
fn stack_underflow_after_push_jump() {
    let operations = vec![
        Operation::Push((1, 3_u8.into())),
        Operation::Jump,
        Operation::Jumpdest { pc: 3 },
        Operation::Push0,
        Operation::Add,
    ];
    let result = run_result(operations);
    assert!(result.halt_reason().unwrap().is_stack_underflow());
    assert_eq!(result.output(), None);
    assert_eq!(result.gas_used(), 3 + 8 + 1 + 2 + 3)
}

#[test]
fn basic_loop() {
    let operations = vec![
        Operation::Push((1, 3_u8.into())), // i = 3; loop counter
        Operation::Jumpdest { pc: 2 },
        Operation::Push((1, 1_u8.into())),
        Operation::Swap(1),
        Operation::Sub,
        Operation::Dup(1),                 // i = i - 1
        Operation::Push((1, 2_u8.into())), // 2; jump dest
        Operation::Jumpi,
        Operation::Pop,
        Operation::Push((1, 6_u8.into())),
    ];
    let result = run_result(operations);
    assert!(result.is_success());
    assert_eq!(result.output(), None);
    assert_eq!(
        result.gas_used(),
        3 + (1 + 3 + 3 + 3 + 3 + 3 + 10) * 3 + 2 + 3
    )
}

#[test]
fn basic_pc() {
    let operations = vec![
        Operation::PC { pc: 0 },
        Operation::PC { pc: 1 },
        Operation::Push((1, 6_u8.into())),
        Operation::PC { pc: 4 },
        Operation::Push0,
        Operation::PC { pc: 6 },
    ];
    let result = run_result(operations);
    assert!(result.is_success());
    assert_eq!(result.output(), None);
    assert_eq!(result.gas_used(), 2 + 2 + 3 + 2 + 2 + 2)
}

#[test]
fn gas() {
    let operations = vec![
        Operation::Gas,
        Operation::Gas,
        Operation::Jumpdest { pc: 3 },
        Operation::Gas,
    ];
    let result = run_result(operations);
    assert!(result.is_success());
    assert_eq!(result.output(), None);
    assert_eq!(result.gas_used(), 2 + 2 + 1 + 2)
}

#[test]
fn address() {
    let operations = vec![Operation::Address, Operation::Address];
    let result = run_result(operations);
    assert!(result.is_success());
    assert_eq!(result.output(), None);
    assert_eq!(result.gas_used(), 2 + 2)
}

#[test]
fn origin() {
    let operations = vec![Operation::Origin, Operation::Origin];
    let result = run_result(operations);
    assert!(result.is_success());
    assert_eq!(result.output(), None);
    assert_eq!(result.gas_used(), 2 + 2)
}

#[test]
fn caller() {
    let operations = vec![Operation::Caller, Operation::Caller];
    let result = run_result(operations);
    assert!(result.is_success());
    assert_eq!(result.output(), None);
    assert_eq!(result.gas_used(), 2 + 2)
}

#[test]
fn callvalue() {
    let operations = vec![Operation::CallValue, Operation::CallValue];
    let result = run_result(operations);
    assert!(result.is_success());
    assert_eq!(result.output(), None);
    assert_eq!(result.gas_used(), 2 + 2)
}

#[test]
fn calldataload() {
    let operations = vec![Operation::Push0, Operation::CalldataLoad];
    let result = run_result(operations);
    assert!(result.is_success());
    assert_eq!(result.output(), None);
    assert_eq!(result.gas_used(), 2 + 3)
}

#[test]
fn calldatasize() {
    let operations = vec![Operation::CalldataSize, Operation::CalldataSize];
    let result = run_result(operations);
    assert!(result.is_success());
    assert_eq!(result.output(), None);
    assert_eq!(result.gas_used(), 2 + 2)
}

#[test]
fn calldatacopy() {
    let operations = vec![
        Operation::Push((1, 32_u8.into())),
        Operation::Push0,
        Operation::Push0,
        Operation::CalldataCopy,
    ];
    let result = run_result(operations);
    assert!(result.is_success());
    assert_eq!(result.output(), None);
    assert_eq!(result.gas_used(), 3 + 2 + 2 + 3 + 3);
    assert_eq!(result.memory, vec![0xCC; 32]);
}

#[test]
fn codesize() {
    let operations = vec![Operation::CodeSize, Operation::CodeSize];
    let result = run_result(operations);
    assert!(result.is_success());
    assert_eq!(result.output(), None);
    assert_eq!(result.gas_used(), 2 + 2)
}

#[test]
fn codecopy() {
    let operations = vec![
        Operation::Push((1, 5_u8.into())),
        Operation::Push0,
        Operation::Push0,
        Operation::CodeCopy,
    ];
    let result = run_result(operations);
    assert!(result.is_success());
    assert_eq!(result.output(), None);
    assert_eq!(result.gas_used(), 3 + 2 + 2 + 3 + 3 + 3);
    assert_eq!(
        result.memory,
        hex_literal::hex!("60055f5f39000000000000000000000000000000000000000000000000000000")
    );
}

#[test]
fn keccak256_stack_underflow() {
    let operations = vec![Operation::Push0, Operation::Keccak256];
    let result = run_result(operations);
    assert!(result.halt_reason().unwrap().is_stack_underflow());
    assert_eq!(result.output(), None);
    assert_eq!(result.gas_used(), 2 + 30)
}

#[test]
fn keccak256_empty_1() {
    let operations = vec![Operation::Push0, Operation::Push0, Operation::Keccak256];
    let result = run_result(operations);
    assert!(result.is_success());
    assert_eq!(result.output(), None);
    assert_eq!(result.gas_used(), 2 + 2 + 30)
}

#[test]
fn keccak256_empty_2() {
    let operations = vec![
        Operation::Push0,
        Operation::Push((1, 32_u8.into())),
        Operation::Keccak256,
    ];
    let result = run_result(operations);
    assert!(result.is_success());
    assert_eq!(result.output(), None);
    assert_eq!(result.gas_used(), 2 + 3 + 30)
}

#[test]
fn keccak256_1() {
    let operations = vec![
        Operation::Push((1, 32_u8.into())),
        Operation::Push0,
        Operation::Keccak256,
    ];
    let result = run_result(operations);
    assert!(result.is_success());
    assert_eq!(result.output(), None);
    let expect_gas = 3 + 2 // Push1 + Push0
        + 30 // 30 is the static gas of Keccak256
        + 6  // 6 = 6 * (32 + 31) / 32 is the dynamic gas cost of Keccak256
        + 3  // 3 = 3 * (32 + 31) / 32 is the memory extention gas cost
        ;
    assert_eq!(result.gas_used(), expect_gas);
    // Check the extended memory
    assert_eq!(result.memory, vec![0; 32]);
}

#[test]
fn keccak256_2() {
    let operations = vec![
        Operation::Push((2, 55555_u16.into())),
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1, 32_u8.into())),
        Operation::Push0,
        Operation::Keccak256,
    ];
    let result = run_result(operations);
    assert!(result.is_success());
    assert_eq!(result.output(), None);
    let expect_gas = 3 + 2 // Push2 + Push0
        + 3 + 3 // MStore
        + 3 + 2 // Push2 + Push0
        + 30 // 30 is the static gas of Keccak256
        + 6  // 6 = 6 * (32 + 31) / 32 is the dynamic gas cost of Keccak256
        ;
    assert_eq!(result.gas_used(), expect_gas);
    // Check the extended memory
    assert_eq!(result.memory, Bytes32::from(55555_u32).to_be_bytes());
}

#[test]
fn returndatasize() {
    let operations = vec![
        Operation::ReturndataSize,
        Operation::ReturndataSize,
        Operation::Push0,
        Operation::MStore,
    ];
    let result = run_result(operations);
    assert!(result.is_success());
    assert_eq!(result.output(), None);
    assert_eq!(result.gas_used(), 2 + 2 + 2 + 3 + 3);
    assert_eq!(result.memory, Bytes32::from(64_u8).to_be_bytes());
}

#[test]
fn returndatacopy() {
    let operations = vec![
        Operation::Push((1, 32_u8.into())),
        Operation::Push0,
        Operation::Push0,
        Operation::ReturndataCopy,
    ];
    let result = run_result(operations);
    assert!(result.is_success());
    assert_eq!(result.output(), None);
    assert_eq!(result.gas_used(), 3 + 2 + 2 + 3 + 3 + 3);
    assert_eq!(result.memory, vec![0xDD; 32]);
}

#[test]
fn extcodesize() {
    let operations = vec![
        Operation::Push((1, 32_u8.into())),
        Operation::Push0,
        Operation::Push0,
        Operation::ExtCodeSize,
    ];
    let result = run_result(operations);
    assert!(result.is_success());
    assert_eq!(result.output(), None);
    assert_eq!(result.gas_used(), 3 + 2 + 2 + 2600);
    assert_eq!(result.memory, vec![0x00; 0]);
}

#[test]
fn extcodecopy() {
    let operations = vec![
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::ExtCodeCopy,
    ];
    let result = run_result(operations);
    assert!(result.is_success());
    assert_eq!(result.output(), None);
    assert_eq!(result.gas_used(), 2 + 2 + 2 + 2 + 2600);
    assert_eq!(result.memory, vec![0x00; 0]);
}

#[test]
fn extcodehash() {
    let operations = vec![
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::ExtCodeHash,
    ];
    let result = run_result(operations);
    assert!(result.is_success());
    assert_eq!(result.output(), None);
    assert_eq!(result.gas_used(), 2 + 2 + 2 + 2 + 2600);
    assert_eq!(result.memory, vec![0x00; 0]);
}

#[test]
fn gasprice() {
    let operations = vec![Operation::GasPrice, Operation::GasPrice];
    let result = run_result(operations);
    assert!(result.is_success());
    assert_eq!(result.output(), None);
    assert_eq!(result.gas_used(), 2 + 2);
    assert_eq!(result.memory, vec![0x00; 0]);
}

#[test]
fn gaslimit() {
    let operations = vec![Operation::GasLimit, Operation::GasLimit];
    let result = run_result(operations);
    assert!(result.is_success());
    assert_eq!(result.output(), None);
    assert_eq!(result.gas_used(), 2 + 2);
    assert_eq!(result.memory, vec![0x00; 0]);
}

#[test]
fn coinbase() {
    let operations = vec![Operation::Coinbase, Operation::Coinbase];
    let result = run_result(operations);
    assert!(result.is_success());
    assert_eq!(result.output(), None);
    assert_eq!(result.gas_used(), 2 + 2);
    assert_eq!(result.memory, vec![0x00; 0]);
}

#[test]
fn timestamp() {
    let operations = vec![Operation::Timestamp, Operation::Timestamp];
    let result = run_result(operations);
    assert!(result.is_success());
    assert_eq!(result.output(), None);
    assert_eq!(result.gas_used(), 2 + 2);
    assert_eq!(result.memory, vec![0x00; 0]);
}

#[test]
fn number() {
    let operations = vec![Operation::Number, Operation::Number];
    let result = run_result(operations);
    assert!(result.is_success());
    assert_eq!(result.output(), None);
    assert_eq!(result.gas_used(), 2 + 2);
    assert_eq!(result.memory, vec![0x00; 0]);
}

#[test]
fn prevrandao() {
    let operations = vec![Operation::Prevrandao, Operation::Prevrandao];
    let result = run_result(operations);
    assert!(result.is_success());
    assert_eq!(result.output(), None);
    assert_eq!(result.gas_used(), 2 + 2);
    assert_eq!(result.memory, vec![0x00; 0]);
}

#[test]
fn chainid() {
    let operations = vec![Operation::Chainid, Operation::Chainid];
    let result = run_result(operations);
    assert!(result.is_success());
    assert_eq!(result.output(), None);
    assert_eq!(result.gas_used(), 2 + 2);
    assert_eq!(result.memory, vec![0x00; 0]);
}

#[test]
fn selfbalance() {
    let operations = vec![Operation::SelfBalance, Operation::SelfBalance];
    let result = run_result(operations);
    assert!(result.is_success());
    assert_eq!(result.output(), None);
    assert_eq!(result.gas_used(), 5 + 5);
    assert_eq!(result.memory, vec![0x00; 0]);
}

#[test]
fn basefee() {
    let operations = vec![Operation::BaseFee, Operation::BaseFee];
    let result = run_result(operations);
    assert!(result.is_success());
    assert_eq!(result.output(), None);
    assert_eq!(result.gas_used(), 2 + 2);
    assert_eq!(result.memory, vec![0x00; 0]);
}

#[test]
fn blobhash() {
    let operations = vec![Operation::Push0, Operation::BlobHash];
    let result = run_result(operations);
    assert!(result.is_success());
    assert_eq!(result.output(), None);
    assert_eq!(result.gas_used(), 2 + 3);
    assert_eq!(result.memory, vec![0x00; 0]);
}

#[test]
fn blobbasefee() {
    let operations = vec![Operation::BlobBaseFee, Operation::BlobBaseFee];
    let result = run_result(operations);
    assert!(result.is_success());
    assert_eq!(result.output(), None);
    assert_eq!(result.gas_used(), 2 + 2);
    assert_eq!(result.memory, vec![0x00; 0]);
}

#[test]
fn mload_1() {
    let operations = vec![Operation::Push0, Operation::MLoad];
    let result = run_result(operations);
    assert!(result.is_success());
    assert_eq!(result.output(), None);
    assert_eq!(result.gas_used(), 2 + 3 + 3);
    // Check the memory extened by the MLOAD opcode.
    assert_eq!(result.memory, vec![0x00; 32]);
}

#[test]
fn mload_2() {
    let operations = vec![Operation::Push((1, 1_u8.into())), Operation::MLoad];
    let result = run_result(operations);
    assert!(result.is_success());
    assert_eq!(result.output(), None);
    assert_eq!(result.gas_used(), 3 + 3 + 3 * 2);
    // Check the memory extened by the MLOAD opcode.
    assert_eq!(result.memory, vec![0x00; 64]);
}

#[test]
fn mload_3() {
    let operations = vec![Operation::Push((1, 32_u8.into())), Operation::MLoad];
    let result = run_result(operations);
    assert!(result.is_success());
    assert_eq!(result.output(), None);
    assert_eq!(result.gas_used(), 3 + 3 + 3 * 2);
    // Check the memory extened by the MLOAD opcode.
    assert_eq!(result.memory, vec![0x00; 64]);
}

#[test]
fn mload_4() {
    let operations = vec![Operation::Push((1, 33_u8.into())), Operation::MLoad];
    let result = run_result(operations);
    assert!(result.is_success());
    assert_eq!(result.output(), None);
    assert_eq!(result.gas_used(), 3 + 3 + 3 * 3);
    // Check the memory extened by the MLOAD opcode.
    assert_eq!(result.memory, vec![0x00; 96]);
}

#[test]
fn mload_overflow_1() {
    let operations = vec![Operation::Push((8, u64::MAX.into())), Operation::MLoad];
    let result = run_result(operations);
    assert!(result.halt_reason().unwrap().is_out_of_gas());
    assert_eq!(result.output(), None);
    assert_eq!(result.gas_used(), 3 + 3);
    // Check the memory extened by the MLOAD opcode.
    assert_eq!(result.memory, vec![0x00; 0]);
}

#[test]
fn mload_overflow_2() {
    let operations = vec![Operation::Push((8, u64::MAX.into())), Operation::MLoad];
    let result = run_result(operations);
    assert!(result.halt_reason().unwrap().is_out_of_gas());
    assert_eq!(result.output(), None);
    assert_eq!(result.gas_used(), 3 + 3);
    // Check the memory extened by the MLOAD opcode.
    assert_eq!(result.memory, vec![0x00; 0]);
}

#[test]
fn mload_overflow_3() {
    let operations = vec![Operation::Push((16, u128::MAX.into())), Operation::MLoad];
    let result = run_result(operations);
    assert!(result.halt_reason().unwrap().is_out_of_gas());
    assert_eq!(result.output(), None);
    assert_eq!(result.gas_used(), 3 + 3);
    // Check the memory extened by the MLOAD opcode.
    assert_eq!(result.memory, vec![0x00; 0]);
}

#[test]
fn mstore_1() {
    let operations = vec![Operation::Push0, Operation::Push0, Operation::MStore];
    let result = run_result(operations);
    assert!(result.is_success());
    assert_eq!(result.output(), None);
    assert_eq!(result.gas_used(), 2 + 2 + 3 + 3);
    // Check the memory extened by the MSTORE opcode.
    assert_eq!(result.memory, vec![0x00; 32]);
}

#[test]
fn mstore_2() {
    let operations = vec![
        Operation::Push((1, 0xAA_u8.into())),
        Operation::Push0,
        Operation::MStore,
    ];
    let result = run_result(operations);
    assert!(result.is_success());
    assert_eq!(result.output(), None);
    assert_eq!(result.gas_used(), 3 + 2 + 3 + 3);
    // Check the memory extened by the MSTORE opcode.
    assert_eq!(result.memory, {
        let mut mem = [0; 32];
        mem[31] = 0xAA;
        mem
    });
}

#[test]
fn mstore8_1() {
    let operations = vec![Operation::Push0, Operation::Push0, Operation::MStore8];
    let result = run_result(operations);
    assert!(result.is_success());
    assert_eq!(result.output(), None);
    assert_eq!(result.gas_used(), 2 + 2 + 3 + 3);
    // Check the memory extened by the MSTORE8 opcode.
    assert_eq!(result.memory, vec![0x00; 32]);
}

#[test]
fn mstore8_2() {
    let operations = vec![
        Operation::Push((1, 0xAA_u8.into())),
        Operation::Push0,
        Operation::MStore8,
    ];
    let result = run_result(operations);
    assert!(result.is_success());
    assert_eq!(result.output(), None);
    assert_eq!(result.gas_used(), 3 + 2 + 3 + 3);
    // Check the memory extened by the MSTORE8 opcode.
    assert_eq!(result.memory, {
        let mut mem = [0; 32];
        mem[0] = 0xAA;
        mem
    });
}

#[test]
fn msize_1() {
    let operations = vec![Operation::MSize, Operation::MSize];
    let result = run_result(operations);
    assert!(result.is_success());
    assert_eq!(result.output(), None);
    assert_eq!(result.gas_used(), 2 + 2);
    assert_eq!(result.memory, vec![0x00; 0]);
}

#[test]
fn msize_2() {
    let operations = vec![
        Operation::MSize,
        Operation::Push0,
        Operation::MLoad,
        Operation::Pop,
        Operation::MSize,
        Operation::Push((1, 1_u8.into())),
        Operation::MLoad,
        Operation::Pop,
        Operation::MSize,
    ];
    let result = run_result(operations);
    assert!(result.is_success());
    assert_eq!(result.output(), None);
    assert_eq!(
        result.gas_used(),
        2 + 2 + 3 + 3 + 2 + 2 + 3 + (3 + 3) + 2 + 2
    );
    assert_eq!(result.memory, vec![0x00; 64]);
}

#[test]
fn mcopy_1() {
    let operations = vec![
        Operation::Push((1, 32_u8.into())),
        Operation::Push0,
        Operation::Push((1, 32_u8.into())),
        Operation::MCopy,
    ];
    let result = run_result(operations);
    assert!(result.is_success());
    assert_eq!(result.output(), None);
    assert_eq!(result.gas_used(), 3 + 2 + 3 + 3 + 3 + 3 * 2);
    assert_eq!(result.memory, vec![0x00; 64]);
}

#[test]
fn mcopy_2() {
    let operations = vec![
        Operation::Push((2, 0xABCD_u16.into())),
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1, 2_u8.into())),
        Operation::Push((1, 30_u8.into())),
        Operation::Push((1, 1_u8.into())),
        Operation::MCopy,
    ];
    let result = run_result(operations);
    assert!(result.is_success());
    assert_eq!(result.output(), None);
    assert_eq!(result.gas_used(), 3 + 2 + 3 + 3 + 3 + 3 + 3 + 3 * 2);
    assert_eq!(result.memory, {
        let mut mem = [0; 32];
        mem[30] = 0xAB;
        mem[31] = 0xCD;
        mem[1] = 0xAB;
        mem[2] = 0xCD;
        mem
    });
}

#[test]
fn balance() {
    let operations = vec![Operation::Push0, Operation::Balance, Operation::Balance];
    let result = run_result(operations);
    assert!(result.is_success());
    assert_eq!(result.output(), None);
    // 2600 is the cold account access cost
    assert_eq!(result.gas_used(), 2 + 2600 + 2600);
    assert_eq!(result.memory, vec![0x00; 0]);
}

#[test]
fn sload_1() {
    let operations = vec![Operation::Push0, Operation::SLoad];
    let result = run_result(operations);
    assert!(result.is_success());
    assert_eq!(result.output(), None);
    // 2100 is the cold storage cost
    assert_eq!(result.gas_used(), 2 + 2100);
    assert_eq!(result.memory, vec![0x00; 0]);
}

#[test]
fn sstore_1() {
    let operations = vec![
        Operation::Push((1, 200_u8.into())),
        Operation::SLoad,
        Operation::Push((1, 100_u8.into())),
        Operation::Push((1, 200_u8.into())),
        Operation::SStore,
        Operation::Push((1, 200_u8.into())),
        Operation::SLoad,
    ];
    let result = run_result(operations);
    assert!(result.is_success());
    assert_eq!(result.output(), None);
    // 2100 is the cold storage cost
    // 20000 is the new value is set from zero cost
    assert_eq!(result.gas_used(), 3 + 2100 + 3 + 3 + 20000 + 3 + 100);
    assert_eq!(result.memory, vec![0x00; 0]);
    assert_eq!(result.sload(U256::from(200)), U256::from(100));
}

#[test]
fn tload_1() {
    let operations = vec![Operation::Push0, Operation::TLoad];
    let result = run_result(operations);
    assert!(result.is_success());
    assert_eq!(result.output(), None);
    assert_eq!(result.gas_used(), 2 + 100);
    assert_eq!(result.memory, vec![0x00; 0]);
}

#[test]
fn tstore_1() {
    let operations = vec![
        Operation::Push((1, 200_u8.into())),
        Operation::TLoad,
        Operation::Push((1, 100_u8.into())),
        Operation::Push((1, 200_u8.into())),
        Operation::TStore,
        Operation::Push((1, 200_u8.into())),
        Operation::TLoad,
    ];
    let result = run_result(operations);
    assert!(result.is_success());
    assert_eq!(result.output(), None);
    assert_eq!(result.gas_used(), 3 + 100 + 3 + 3 + 100 + 3 + 100);
    assert_eq!(result.memory, vec![0x00; 0]);
    assert_eq!(result.tload(U256::from(200)), U256::from(100));
}

#[test]
fn log0() {
    let operations = vec![Operation::Push0, Operation::Push0, Operation::Log(0)];
    let result = run_result(operations);
    assert!(result.is_success());
    assert_eq!(result.output(), None);
    // 375 is the log0 cost
    assert_eq!(result.gas_used(), 2 + 2 + 375);
    assert_eq!(result.memory, vec![0x00; 0]);
    assert_eq!(
        result.logs(),
        &[Log {
            address: Address::default(),
            data: LogData::default()
        }]
    );
}

#[test]
fn log0_data() {
    let operations = vec![
        Operation::Push((2, 0xAABB_u16.into())),
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1, 32_u8.into())),
        Operation::Push0,
        Operation::Log(0),
    ];
    let result = run_result(operations);
    assert!(result.is_success());
    assert_eq!(result.output(), None);
    // 375 + 8 * 32 is the log0 with 32 length cost
    assert_eq!(result.gas_used(), 3 + 2 + (3 + 3) + 3 + 2 + 375 + 8 * 32);
    assert_eq!(
        result.memory,
        Bytes32::from(0xAABB_u16).to_be_bytes().to_vec()
    );
    assert_eq!(
        result.logs(),
        &[Log {
            address: Address::default(),
            data: LogData::new_unchecked(vec![], Bytes32::from(0xAABB_u16).to_be_bytes().to_vec()),
        }]
    );
}

#[test]
fn log1_1() {
    let operations = vec![
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Log(1),
    ];
    let result = run_result(operations);
    assert!(result.is_success());
    assert_eq!(result.output(), None);
    assert_eq!(result.gas_used(), 2 + 2 + 2 + 375 * 2);
    assert_eq!(result.memory, vec![0x00; 0]);
    assert_eq!(
        result.logs(),
        &[Log {
            address: Address::default(),
            data: LogData::new_unchecked(vec![B256::zero()], vec![]),
        }]
    );
}

#[test]
fn log1_2() {
    let operations = vec![
        Operation::Push((32, 0xFFFFFFFF_FFFFFFFF_FFFFFFFF_u128.into())),
        Operation::Push((32, 50_u8.into())),
        Operation::Push0,
        Operation::Log(1),
    ];
    let result = run_result(operations);
    assert!(result.is_success());
    assert_eq!(result.output(), None);
    assert_eq!(result.gas_used(), 3 + 3 + 2 + 3 * 2 + 375 * 2 + 8 * 50);
    assert_eq!(result.memory, vec![0x00; 64]);
    assert_eq!(
        result.logs(),
        &[Log {
            address: Address::default(),
            data: LogData::new_unchecked(
                vec![Bytes32::from(0xFFFFFFFF_FFFFFFFF_FFFFFFFF_u128).to_b256()],
                vec![0; 50]
            ),
        }]
    );
}

#[test]
fn log2_1() {
    let operations = vec![
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Log(2),
    ];
    let result = run_result(operations);
    assert!(result.is_success());
    assert_eq!(result.output(), None);
    assert_eq!(result.gas_used(), 2 + 2 + 2 + 2 + 375 * 3);
    assert_eq!(result.memory, vec![0x00; 0]);
    assert_eq!(
        result.logs(),
        &[Log {
            address: Address::default(),
            data: LogData::new_unchecked(vec![B256::zero(); 2], vec![]),
        }]
    );
}

#[test]
fn log3_1() {
    let operations = vec![
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Log(3),
    ];
    let result = run_result(operations);
    assert!(result.is_success());
    assert_eq!(result.output(), None);
    assert_eq!(result.gas_used(), 2 + 2 + 2 + 2 + 2 + 375 * 4);
    assert_eq!(result.memory, vec![0x00; 0]);
    assert_eq!(
        result.logs(),
        &[Log {
            address: Address::default(),
            data: LogData::new_unchecked(vec![B256::zero(); 3], vec![]),
        }]
    );
}

#[test]
fn log4_1() {
    let operations = vec![
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Push0,
        Operation::Log(4),
    ];
    let result = run_result(operations);
    assert!(result.is_success());
    assert_eq!(result.output(), None);
    assert_eq!(result.gas_used(), 2 + 2 + 2 + 2 + 2 + 2 + 375 * 5);
    assert_eq!(result.memory, vec![0x00; 0]);
    assert_eq!(
        result.logs(),
        &[Log {
            address: Address::default(),
            data: LogData::new_unchecked(vec![B256::zero(); 4], vec![]),
        }]
    );
}

#[test]
fn call_1() {
    let operations = vec![
        Operation::Push((1, 1_u32.into())), // ret length
        Operation::Push((1, 2_u32.into())), // ret offset
        Operation::Push((1, 3_u32.into())), // args length
        Operation::Push((1, 4_u32.into())), // args offset
        Operation::Push((1, 5_u32.into())), // value
        Operation::Push((1, 6_u32.into())), // address
        Operation::Push((1, 7_u32.into())), // gas
        Operation::Call,
    ];
    let result = run_result(operations);
    assert!(result.is_success());
    assert_eq!(result.output(), None);
    assert_eq!(result.memory, vec![0x00; 32]);
}

#[test]
fn delegatecall_1() {
    let operations = vec![
        Operation::Push((1, 1_u32.into())), // ret length
        Operation::Push((1, 2_u32.into())), // ret offset
        Operation::Push((1, 3_u32.into())), // args length
        Operation::Push((1, 4_u32.into())), // args offset
        Operation::Push((1, 5_u32.into())), // address
        Operation::Push((1, 6_u32.into())), // gas
        Operation::DelegateCall,
    ];
    let result = run_result(operations);
    assert!(result.is_success());
    assert_eq!(result.output(), None);
    assert_eq!(result.memory, vec![0x00; 32]);
}

#[test]
fn staticcall_1() {
    let operations = vec![
        Operation::Push((1, 1_u32.into())), // ret length
        Operation::Push((1, 2_u32.into())), // ret offset
        Operation::Push((1, 3_u32.into())), // args length
        Operation::Push((1, 4_u32.into())), // args offset
        Operation::Push((1, 5_u32.into())), // address
        Operation::Push((1, 6_u32.into())), // gas
        Operation::StaticCall,
    ];
    let result = run_result(operations);
    assert!(result.is_success());
    assert_eq!(result.output(), None);
    assert_eq!(result.memory, vec![0x00; 32]);
}

#[test]
fn revert() {
    let operations = vec![
        Operation::Push((1, 0xAA_u8.into())),
        Operation::Push0,
        Operation::MStore,
        Operation::Push((1, 32_u8.into())),
        Operation::Push0,
        Operation::Revert,
    ];
    let result = run_result(operations);
    assert!(result.is_revert());
    assert_eq!(result.gas_used(), 3 + 2 + 3 + 3 + 3 + 2);
    assert_eq!(
        result.output(),
        Some(&Bytes::from(Bytes32::from(0xAA_u32).to_be_bytes().to_vec()))
    );
}

#[test]
fn selfdestruct() {
    let operations = vec![
        Operation::Push((1, 0xAA_u8.into())),
        Operation::SelfDestruct,
    ];
    let result = run_result(operations);
    assert!(result.is_success());
    assert_eq!(result.gas_used(), 3 + 5000);
    assert_eq!(result.output(), None);
    assert_eq!(result.memory, vec![0x00; 0]);
}
