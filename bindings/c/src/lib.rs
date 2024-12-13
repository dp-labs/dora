// Copyright 2024 The Dora Authors.
// Licensed under the Apache License, Version 2.0.

use dora::primitives::SpecId;
use evmc_declare::evmc_declare_vm;
use evmc_vm::*;

#[cfg(test)]
mod tests;

#[evmc_declare_vm("dora", "evm, ewasm, precompiles", "12.0.0")]
pub struct DoraVM {}

impl EvmcVm for DoraVM {
    fn init() -> Self {
        Self {}
    }

    fn execute<'a>(
        &self,
        revision: Revision,
        code: &'a [u8],
        message: &'a ExecutionMessage,
        context: Option<&'a mut ExecutionContext<'a>>,
    ) -> ExecutionResult {
        let Some(context) = context else {
            return ExecutionResult::failure();
        };
        let _spec_id = evmc_revision_to_spec_id(revision);
        let tx_context = *context.get_tx_context();

        let save_return_block_number: Vec<u8> = vec![
            0x43, 0x60, 0x00, 0x55, 0x43, 0x60, 0x00, 0x52, 0x59, 0x60, 0x00, 0xf3,
        ];

        if save_return_block_number != code {
            return ExecutionResult::failure();
        }

        assert!(tx_context.block_number <= 255);
        let block_number = tx_context.block_number as u8;

        let storage_key = Bytes32::default();
        let mut storage_value = Bytes32::default();
        storage_value.bytes[31] = block_number;
        context.set_storage(message.recipient(), &storage_key, &storage_value);

        let ret = format!("{}", block_number).into_bytes();
        ExecutionResult::success(message.gas() / 2, 0, Some(&ret))
    }
}

#[inline]
fn evmc_revision_to_spec_id(revision: Revision) -> SpecId {
    use evmc_sys::evmc_revision::*;
    match revision {
        EVMC_FRONTIER => SpecId::FRONTIER,
        EVMC_HOMESTEAD => SpecId::HOMESTEAD,
        EVMC_TANGERINE_WHISTLE => SpecId::TANGERINE,
        EVMC_SPURIOUS_DRAGON => SpecId::SPURIOUS_DRAGON,
        EVMC_BYZANTIUM => SpecId::BYZANTIUM,
        EVMC_CONSTANTINOPLE => SpecId::CONSTANTINOPLE,
        EVMC_PETERSBURG => SpecId::PETERSBURG,
        EVMC_ISTANBUL => SpecId::ISTANBUL,
        EVMC_BERLIN => SpecId::BERLIN,
        EVMC_LONDON => SpecId::LONDON,
        EVMC_PARIS => SpecId::MERGE,
        EVMC_SHANGHAI => SpecId::SHANGHAI,
        EVMC_CANCUN => SpecId::CANCUN,
        EVMC_PRAGUE => SpecId::PRAGUE,
        EVMC_OSAKA => SpecId::OSAKA,
    }
}
