// Copyright 2024 The Dora Authors.
// Licensed under the Apache License, Version 2.0.

use dora::primitives::{Address, Bytes, Bytes32, B256, U256};
use dora::runtime::as_u64_saturated;
use dora::runtime::call::{CallMessage, CallResult};
use dora::runtime::context::Log;
use dora::runtime::env::{BlobExcessGasAndPrice, BlockEnv, CfgEnv, Env, TxEnv};
use dora::runtime::host::{
    AccountLoad, CodeLoad, Host, SStoreResult, SStoreStatus, SelfdestructResult, StateLoad,
};
use dora::runtime::result::EVMError;
use evmc_sys::{evmc_access_status, evmc_address, evmc_bytes32, evmc_storage_status};
use evmc_vm::{ExecutionContext, ExecutionMessage};
use std::mem::transmute;

use crate::{call_kind_to_evmc_msg_kind, evmc_address_to_address, evmc_status_to_status};

pub(crate) struct EvmcDelegateHost<'a> {
    context: &'a mut ExecutionContext<'a>,
    env: Env,
}

impl<'a> EvmcDelegateHost<'a> {
    #[inline]
    pub(crate) fn new(context: &'a mut ExecutionContext<'a>) -> Self {
        let tx_context = context.get_tx_context();
        let chain_id = as_u64_saturated!(U256::from_be_bytes(tx_context.chain_id.bytes));
        let blob_hashes = unsafe {
            Vec::<evmc_bytes32>::from_raw_parts(
                tx_context.blob_hashes as _,
                tx_context.blob_hashes_count,
                tx_context.blob_hashes_count,
            )
            .iter()
            .map(|b| B256::from_slice(&b.bytes))
            .collect()
        };
        Self {
            env: Env {
                cfg: CfgEnv {
                    chain_id,
                    ..Default::default()
                },
                block: BlockEnv {
                    number: U256::from(tx_context.block_number),
                    coinbase: evmc_address_to_address(&tx_context.block_coinbase),
                    timestamp: U256::from(tx_context.block_timestamp),
                    gas_limit: U256::from(tx_context.block_gas_limit),
                    basefee: U256::from_be_bytes(tx_context.block_base_fee.bytes),
                    difficulty: U256::from_be_bytes(tx_context.block_prev_randao.bytes),
                    prevrandao: Some(B256::from_slice(&tx_context.block_prev_randao.bytes)),
                    blob_excess_gas_and_price: Some(BlobExcessGasAndPrice::new(as_u64_saturated!(
                        U256::from_be_bytes(tx_context.blob_base_fee.bytes)
                    ))),
                },
                tx: TxEnv {
                    caller: evmc_address_to_address(&tx_context.tx_origin),
                    gas_price: U256::from_be_bytes(tx_context.tx_gas_price.bytes),
                    blob_hashes,
                    max_fee_per_blob_gas: Some(U256::from_be_bytes(tx_context.blob_base_fee.bytes)),
                    chain_id: Some(chain_id),
                    ..Default::default()
                },
            },
            context,
        }
    }
}

impl Host for EvmcDelegateHost<'_> {
    #[inline]
    fn env(&self) -> &Env {
        &self.env
    }

    #[inline]
    fn env_mut(&mut self) -> &mut Env {
        &mut self.env
    }

    fn sload(&mut self, addr: Address, key: Bytes32) -> Option<StateLoad<Bytes32>> {
        unsafe {
            let addr = transmute::<Address, evmc_address>(addr);
            let key = transmute::<Bytes32, evmc_bytes32>(key.to_be());
            let is_cold = matches!(
                self.context.access_storage(&addr, &key),
                evmc_access_status::EVMC_ACCESS_COLD
            );
            let result = self.context.get_storage(&addr, &key);
            Some(StateLoad::new(
                Bytes32::from_be_bytes(result.bytes),
                is_cold,
            ))
        }
    }

    fn sstore(
        &mut self,
        addr: Address,
        key: Bytes32,
        value: Bytes32,
    ) -> Option<StateLoad<SStoreResult>> {
        unsafe {
            let addr = transmute::<Address, evmc_address>(addr);
            let key = transmute::<Bytes32, evmc_bytes32>(key.to_be());
            let value = transmute::<Bytes32, evmc_bytes32>(value.to_be());
            let is_cold = matches!(
                self.context.access_storage(&addr, &key),
                evmc_access_status::EVMC_ACCESS_COLD
            );
            let status = self.context.set_storage(&addr, &key, &value);
            Some(StateLoad::new(
                SStoreResult::Status(transmute::<evmc_storage_status, SStoreStatus>(status)),
                is_cold,
            ))
        }
    }

    fn tload(&mut self, addr: Address, key: Bytes32) -> Bytes32 {
        unsafe {
            let addr = transmute::<Address, evmc_address>(addr);
            let key = transmute::<Bytes32, evmc_bytes32>(key.to_be());
            let result = self.context.get_transient_storage(&addr, &key);
            Bytes32::from_be_bytes(result.bytes)
        }
    }

    fn tstore(&mut self, addr: Address, key: Bytes32, value: Bytes32) {
        unsafe {
            let addr = transmute::<Address, evmc_address>(addr);
            let key = transmute::<Bytes32, evmc_bytes32>(key.to_be());
            let value = transmute::<Bytes32, evmc_bytes32>(value.to_be());
            self.context.set_transient_storage(&addr, &key, &value);
        }
    }

    fn load_account_delegated(&mut self, _addr: Address) -> Option<AccountLoad> {
        // Nothing to do
        None
    }

    fn balance(&mut self, addr: Address) -> Option<StateLoad<Bytes32>> {
        unsafe {
            let addr = transmute::<Address, evmc_address>(addr);
            let is_cold = matches!(
                self.context.access_account(&addr),
                evmc_access_status::EVMC_ACCESS_COLD
            );
            let value = self.context.get_balance(&addr);
            Some(StateLoad::new(Bytes32::from_be_bytes(value.bytes), is_cold))
        }
    }

    fn code(&mut self, addr: Address) -> Option<CodeLoad<Bytes>> {
        unsafe {
            let addr = transmute::<Address, evmc_address>(addr);
            let is_cold = matches!(
                self.context.access_account(&addr),
                evmc_access_status::EVMC_ACCESS_COLD
            );
            let size = self.context.get_code_size(&addr);
            let mut code = Vec::with_capacity(size);
            self.context.copy_code(&addr, 0, &mut code);
            Some(CodeLoad::new(StateLoad::new(code.into(), is_cold), false))
        }
    }

    fn code_hash(&mut self, addr: Address) -> Option<CodeLoad<Bytes32>> {
        unsafe {
            let addr = transmute::<Address, evmc_address>(addr);
            let hash = self.context.get_code_hash(&addr);
            let is_cold = matches!(
                self.context.access_account(&addr),
                evmc_access_status::EVMC_ACCESS_COLD
            );
            Some(CodeLoad::new(
                StateLoad::new(Bytes32::from_be_bytes(hash.bytes), is_cold),
                false,
            ))
        }
    }

    fn selfdestruct(
        &mut self,
        addr: Address,
        target: Address,
    ) -> Option<StateLoad<SelfdestructResult>> {
        unsafe {
            let addr = transmute::<Address, evmc_address>(addr);
            let target = transmute::<Address, evmc_address>(target);
            let is_cold = matches!(
                self.context.access_account(&target),
                evmc_access_status::EVMC_ACCESS_COLD
            );
            let balance = self.context.get_balance(&addr);
            let had_value = !Bytes32::from_be_bytes(balance.bytes).as_u256().is_zero();
            let target_exists = self.context.account_exists(&target);
            let first_registerd = self.context.selfdestruct(&addr, &target);
            Some(StateLoad::new(
                SelfdestructResult {
                    had_value,
                    target_exists,
                    previously_destroyed: !first_registerd,
                },
                is_cold,
            ))
        }
    }

    #[inline]
    fn block_hash(&mut self, number: u64) -> Option<Bytes32> {
        Some(Bytes32::from_be_bytes(
            self.context.get_block_hash(number as i64).bytes,
        ))
    }

    fn log(&mut self, log: Log) {
        unsafe {
            let addr = transmute::<Address, evmc_address>(log.address);
            self.context.emit_log(
                &addr,
                &log.data.data,
                &log.data
                    .topics
                    .iter()
                    .map(|t| transmute(t.0))
                    .collect::<Vec<evmc_bytes32>>(),
            );
        }
    }

    fn call(&mut self, msg: CallMessage) -> Result<CallResult, EVMError> {
        unsafe {
            let result = self.context.call(&ExecutionMessage::new(
                call_kind_to_evmc_msg_kind(msg.kind),
                0,
                msg.depth as i32,
                msg.gas_limit as i64,
                transmute::<Address, evmc_address>(msg.recipient),
                transmute::<Address, evmc_address>(msg.caller),
                Some(&msg.input.0),
                transmute::<[u8; 32], evmc_bytes32>(Bytes32::from(msg.value).to_be_bytes()),
                transmute::<B256, evmc_bytes32>(msg.salt.unwrap_or_default()),
                transmute::<Address, evmc_address>(msg.code_address),
                if msg.input.len() > 0 {
                    Some(&msg.input.0)
                } else {
                    None
                },
            ));
            Ok(CallResult {
                status: evmc_status_to_status(result.status_code()),
                gas_limit: msg.gas_limit,
                gas_remaining: result.gas_left() as u64,
                gas_refunded: result.gas_refund(),
                output: result.output().cloned().unwrap_or_default().into(),
                create_address: result.create_address().map(evmc_address_to_address),
            })
        }
    }
}
