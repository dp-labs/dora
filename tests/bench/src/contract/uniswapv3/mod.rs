use alloy_sol_types::{SolCall, sol};
use dora_primitives::{
    AccountStatus, Address, B256, FixedBytes, HashMap, ToHexExt, U256, Uint, fixed_bytes,
    keccak256, uint,
};
use dora_runtime::db::DbAccount;

use super::{address_to_u256, indices_to_u256, keccak256_slice_list, str_to_u256, tick_to_u256};

pub const WETH_BYTECODE_HEX: &str = include_str!("./weth.hex");
pub const FACTORY_BYTECODE_HEX: &str = include_str!("./factory.hex");
pub const POOL_BYTECODE_HEX: &str = include_str!("./pool.hex");
pub const ROUTER_BYTECODE_HEX: &str = include_str!("./router.hex");
pub const SWAP_BYTECODE_HEX: &str = include_str!("./swap.hex");

pub const POOL_FEE: u32 = 3000;
pub const TICK_SPACING: i32 = 60;

#[derive(Debug)]
pub struct WethContract {
    /// Token name, Storage Slot 0
    pub name: String,
    /// Storage Slot 1
    pub symbol: String,
    /// Storage 2
    pub decimals: U256,
    /// Storage Slot 3
    pub deposit: HashMap<Address, U256>,
    /// Storage Slot 4
    pub allowances: HashMap<(Address, Address), U256>,
}

impl Default for WethContract {
    fn default() -> Self {
        Self {
            name: "WETH".to_string(),
            symbol: "WETH".to_string(),
            decimals: U256::from(18),
            deposit: Default::default(),
            allowances: Default::default(),
        }
    }
}

impl WethContract {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn to_db_account(&self) -> (Vec<u8>, DbAccount) {
        let bytecode = hex::decode(WETH_BYTECODE_HEX).unwrap();
        let hash = keccak256(&bytecode);

        let mut storage = HashMap::default();
        storage.insert(U256::from(0), str_to_u256(&self.name));
        storage.insert(U256::from(1), str_to_u256(&self.symbol));
        storage.insert(U256::from(2), self.decimals);
        storage.insert(U256::from(3), U256::from(0));
        storage.insert(U256::from(4), U256::from(0));

        for (address, amount) in &self.deposit {
            storage.insert(
                indices_to_u256(U256::from(3), &[address_to_u256(*address)]),
                *amount,
            );
        }

        for ((address, spender), amount) in &self.allowances {
            storage.insert(
                indices_to_u256(
                    U256::from(4),
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
}

#[derive(Debug, Default)]
pub struct FactoryContract {
    pub owner: Address,
    pub pools: HashMap<(Address, Address, U256), Address>,
}

impl FactoryContract {
    pub fn new(owner: Address) -> Self {
        Self {
            owner,
            pools: HashMap::default(),
        }
    }

    pub fn add_pool(
        &mut self,
        token_0: Address,
        token_1: Address,
        pool_address: Address,
    ) -> &mut Self {
        self.pools
            .insert((token_0, token_1, U256::from(POOL_FEE)), pool_address);
        self
    }

    pub fn to_db_account(&self, address: Address) -> (Vec<u8>, DbAccount) {
        let hex = FACTORY_BYTECODE_HEX.trim().replace(
            "0b748751e6f8b1a38c9386a19d9f8966b3593a9e",
            &address.encode_hex(),
        );
        let bytecode = hex::decode(hex).unwrap();
        let hash = keccak256(&bytecode);

        let mut storage = HashMap::default();
        storage.insert(U256::from(0), U256::from(0));
        storage.insert(U256::from(1), U256::from(1));
        storage.insert(U256::from(2), U256::from(0));
        storage.insert(U256::from(3), address_to_u256(self.owner));
        storage.insert(U256::from(4), U256::from(0));
        storage.insert(U256::from(5), U256::from(0));

        // mapping (uint24 => int24) feeAmountTickSpacing
        let fee_amount_tick_spacing = [(500, 10), (3000, 60), (10000, 200)];
        for (k, v) in fee_amount_tick_spacing {
            storage.insert(
                indices_to_u256(U256::from(4), &[U256::from(k)]),
                U256::from(v),
            );
        }

        // pool fees mapping(address => mapping(address => mapping(uint24 => address)))
        for ((token_0, token_1, fee), addr) in &self.pools {
            storage.insert(
                indices_to_u256(
                    U256::from(5),
                    &[address_to_u256(*token_0), address_to_u256(*token_1), *fee],
                ),
                address_to_u256(*addr),
            );
            storage.insert(
                indices_to_u256(
                    U256::from(5),
                    &[address_to_u256(*token_1), address_to_u256(*token_0), *fee],
                ),
                address_to_u256(*addr),
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

    pub fn create_pool_abi_encode(token_a: Address, token_b: Address, fee: Uint<24, 1>) -> Vec<u8> {
        sol! {
            function createPool(
                address tokenA,
                address tokenB,
                uint24 fee
            ) external override noDelegateCall returns (address pool);
        }
        createPoolCall {
            tokenA: token_a,
            tokenB: token_b,
            fee,
        }
        .abi_encode()
    }
}

#[derive(Debug, Default)]
pub struct PoolContract {
    pub token_0: Address,
    pub token_1: Address,
    pub factory: Address,
    pub positions: HashMap<U256, [U256; 4]>,
    pub ticks: HashMap<U256, [U256; 4]>,
    pub tick_bitmap: HashMap<U256, U256>,
}

impl PoolContract {
    pub fn new(token_0: Address, token_1: Address, factory: Address) -> Self {
        Self {
            token_0,
            token_1,
            factory,
            positions: HashMap::default(),
            ticks: HashMap::default(),
            tick_bitmap: HashMap::default(),
        }
    }

    pub fn add_position(
        &mut self,
        owner: Address,
        tick_lower: i32,
        tick_upper: i32,
        value: [U256; 4],
    ) -> &mut Self {
        let t0 = &FixedBytes::<20>::from(owner)[..]; // 20 bytes
        let t1 = &FixedBytes::<32>::from(tick_to_u256(tick_lower))[29..32]; // 3 bytes
        let t2 = &FixedBytes::<32>::from(tick_to_u256(tick_upper))[29..32]; // 3 bytes
        let key: U256 = keccak256([t0, t1, t2].concat()).into();
        self.positions.insert(key, value);
        self
    }

    pub fn add_tick(&mut self, tick: i32, value: [U256; 4]) -> &mut Self {
        self.ticks.insert(tick_to_u256(tick), value);

        let index: i32 = tick / TICK_SPACING;
        self.tick_bitmap
            .entry(tick_to_u256(index >> 8))
            .or_default()
            .set_bit((index & 0xff).try_into().unwrap(), true);
        self
    }

    pub fn to_db_account(&self, address: Address) -> (Vec<u8>, DbAccount) {
        let hex = POOL_BYTECODE_HEX
            .trim()
            .replace(
                "261d8c5e9742e6f7f1076fa1f560894524e19cad",
                &self.token_0.encode_hex(),
            )
            .replace(
                "ce3478a9e0167a6bc5716dc39dbbbfac38f27623",
                &self.token_1.encode_hex(),
            )
            .replace(
                "cba6b9a951749b8735c603e7ffc5151849248772",
                &self.factory.encode_hex(),
            )
            .replace(
                "d495d5e5cab2567777fff988c4fcd71328b17c9d",
                &address.encode_hex(),
            );
        let bytecode = hex::decode(hex).unwrap();
        let hash = keccak256(&bytecode);

        let mut storage = HashMap::default();
        storage.insert(
            U256::from(0),
            uint!(0x0001000001000100000000000000000000000001000000000000000000000000_U256),
        );
        storage.insert(U256::from(1), U256::from(0));
        storage.insert(U256::from(2), U256::from(0));
        storage.insert(U256::from(3), U256::from(0));
        storage.insert(
            U256::from(4),
            U256::from(111_111_000_000_010_412_955_141u128),
        );
        storage.insert(U256::from(5), U256::from(0));
        storage.insert(U256::from(6), U256::from(0));
        storage.insert(U256::from(7), U256::from(0));
        storage.insert(
            U256::from(8),
            uint!(0x0100000000000000000000000000000000000000000000000000000000000001_U256),
        );

        for (k, v) in &self.ticks {
            let slot = indices_to_u256(U256::from(5), &[U256::from(*k)]);
            for (index, item) in v.iter().enumerate() {
                storage.insert(slot.wrapping_add(U256::from(index)), *item);
            }
        }

        for (k, v) in &self.tick_bitmap {
            storage.insert(indices_to_u256(U256::from(6), &[*k]), *v);
        }

        for (k, v) in &self.positions {
            let slot = indices_to_u256(U256::from(7), &[U256::from(*k)]);
            for (index, item) in v.iter().enumerate() {
                storage.insert(slot.wrapping_add(U256::from(index)), *item);
            }
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

    pub fn pool_address(&self, factory_address: Address, pool_init_code_hash: B256) -> Address {
        let hash = keccak256_slice_list(&[
            fixed_bytes!("ff").as_slice(),
            factory_address.as_slice(),
            keccak256_slice_list(&[
                B256::left_padding_from(self.token_0.as_slice()).as_slice(),
                B256::left_padding_from(self.token_1.as_slice()).as_slice(),
                B256::from(U256::from(POOL_FEE)).as_slice(),
            ])
            .as_slice(),
            pool_init_code_hash.as_slice(),
        ]);
        Address::from_slice(&hash[12..32])
    }
}

#[derive(Debug, Default)]
pub struct RouterContract {
    pub weth: Address,
    pub factory: Address,
    pub pool_init_code_hash: B256,
}

impl RouterContract {
    pub fn new(weth: Address, factory: Address, pool_init_code_hash: B256) -> Self {
        Self {
            weth,
            factory,
            pool_init_code_hash,
        }
    }

    pub fn to_db_account(&self) -> (Vec<u8>, DbAccount) {
        let hex = ROUTER_BYTECODE_HEX
            .trim()
            .replace(
                "6509f2a854ba7441039fce3b959d5badd2ffcfcd",
                &self.weth.encode_hex(),
            )
            .replace(
                "d787a42ee3ac477c46dd6c912e7af795d44453d5",
                &self.factory.encode_hex(),
            )
            .replace(
                "636fcf59d7fdf3a03833dba1c6d936c9d7c6730057c8c4c8e5059feaeab60e04",
                &self.pool_init_code_hash.encode_hex(),
            );
        let bytecode = hex::decode(hex).unwrap();
        let hash = keccak256(&bytecode);

        let mut storage = HashMap::default();
        storage.insert(
            U256::from(0),
            uint!(0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff_U256),
        );

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
}

#[derive(Debug, Default)]
pub struct SwapContract {
    pub router: Address,
    pub token_0: Address,
    pub token_1: Address,
}

impl SwapContract {
    pub const fn new(router: Address, token_0: Address, token_1: Address) -> Self {
        Self {
            router,
            token_0,
            token_1,
        }
    }

    pub fn to_db_account(&self) -> (Vec<u8>, DbAccount) {
        let hex = SWAP_BYTECODE_HEX.trim().replace(
            "e7cfcccb38ce07ba9d8d13431afe8cf6172de031",
            &self.router.encode_hex(),
        );
        let bytecode = hex::decode(hex).unwrap();
        let hash = keccak256(&bytecode);

        let mut storage = HashMap::default();
        storage.insert(U256::from(0), address_to_u256(self.token_0));
        storage.insert(U256::from(1), address_to_u256(self.token_1));
        // fee: slot 1 offset 20 bytes 3
        {
            let offset = 20;
            let length = 3;
            let entry = storage.entry(U256::from(1)).or_default();
            let mut buffer = B256::from(*entry);
            let value_buffer = B256::from(U256::from(POOL_FEE));
            buffer[(32 - offset - length)..(32 - offset)]
                .copy_from_slice(&value_buffer[(32 - length)..32]);
            *entry = buffer.into();
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
}
