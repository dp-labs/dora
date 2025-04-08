use std::collections::HashMap;

use alloy_sol_types::{SolCall, sol};
use dora_primitives::{AccountStatus, Address, U256, keccak256};
use dora_runtime::db::DbAccount;
use ruint::UintTryFrom;
use rustc_hash::FxHashMap;

use super::{address_to_u256, indices_to_u256, str_to_u256};

pub const ERC20_BYTECODE_HEX: &str = include_str!("./contract.hex");

/// ERC20 contract
#[derive(Debug, Default)]
pub struct ERC20Contract {
    /// Storage Slot 0
    pub balances: FxHashMap<Address, U256>,
    /// Storage Slot 1
    pub allowances: FxHashMap<(Address, Address), U256>,
    /// Storage Slot 2
    pub total_supply: U256,
    /// Token name, Storage Slot 3
    pub name: String,
    /// Storage Slot 4
    pub symbol: String,
    /// Storage 5
    pub decimals: U256,
}

sol! {
    function transfer(address recipient, uint256 amount) public payable;
}

impl ERC20Contract {
    pub fn new<U, V>(name: &str, symbol: &str, decimals: U, initial_supply: V) -> Self
    where
        U256: UintTryFrom<U> + UintTryFrom<V>,
    {
        Self {
            name: String::from(name),
            symbol: String::from(symbol),
            decimals: U256::from(decimals),
            total_supply: U256::from(initial_supply),
            balances: FxHashMap::default(),
            allowances: FxHashMap::default(),
        }
    }

    pub fn add_balances(&mut self, addresses: &[Address], amount: U256) -> &mut Self {
        for address in addresses {
            self.balances.insert(*address, amount);
        }
        self
    }

    pub fn add_allowances(
        &mut self,
        addresses: &[Address],
        spender: Address,
        amount: U256,
    ) -> &mut Self {
        for address in addresses {
            self.allowances.insert((*address, spender), amount);
        }
        self
    }

    pub fn to_db_account(&self) -> (Vec<u8>, DbAccount) {
        let bytecode = hex::decode(ERC20_BYTECODE_HEX).unwrap();
        let hash = keccak256(&bytecode);

        let mut storage = HashMap::default();
        storage.insert(U256::from(0), U256::from(0));
        storage.insert(U256::from(1), U256::from(0));
        storage.insert(U256::from(2), self.total_supply);
        storage.insert(U256::from(3), str_to_u256(&self.name));
        storage.insert(U256::from(4), str_to_u256(&self.symbol));
        storage.insert(U256::from(5), self.decimals);

        for (address, amount) in &self.balances {
            storage.insert(
                indices_to_u256(U256::from(0), &[address_to_u256(*address)]),
                *amount,
            );
        }

        for ((address, spender), amount) in &self.allowances {
            storage.insert(
                indices_to_u256(
                    U256::from(1),
                    &[address_to_u256(*address), address_to_u256(*spender)],
                ),
                *amount,
            );
        }

        (
            bytecode,
            DbAccount {
                nonce: 1,
                balance: U256::ZERO,
                storage,
                bytecode_hash: hash,
                status: AccountStatus::Created,
                ..Default::default()
            },
        )
    }

    #[inline]
    pub fn transfer_abi_encode(recipient: Address, amount: U256) -> Vec<u8> {
        transferCall { recipient, amount }.abi_encode()
    }
}
