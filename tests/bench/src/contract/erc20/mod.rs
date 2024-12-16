use alloy_sol_types::{sol, SolCall};
use dora_primitives::{keccak256, Address, U256};
use dora_runtime::{account::AccountStatus, db::DbAccount};
use rustc_hash::FxHashMap;

use super::{address_to_u256, indices_to_u256, str_to_u256};

pub const ERC20_BYTECODE_HEX: &str = include_str!("./contract.hex");

/// ERC20 contract
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
    pub fn to_db_account(&self) -> DbAccount {
        let bytecode = hex::decode(ERC20_BYTECODE_HEX).unwrap();
        let hash = keccak256(&bytecode);

        let mut storage = FxHashMap::default();
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

        DbAccount {
            nonce: 1,
            balance: U256::ZERO,
            storage,
            bytecode_hash: hash,
            status: AccountStatus::Created,
            ..Default::default()
        }
    }

    #[inline]
    pub fn transfer_abi_encode(recipient: Address, amount: U256) -> Vec<u8> {
        transferCall { recipient, amount }.abi_encode()
    }
}
