use crate::native::fibonacci_rust;
use alloy_sol_types::{sol, SolCall};
use dora_primitives::address;
use ruint::aliases::U256;
use std::hint::black_box;

pub const FIBONACCI_BYTECODE_HEX: &str =
    "5f355f60015b8215601a578181019150909160019003916005565b9150505f5260205ff3";
pub const FACTORIAL_BYTECODE_HEX: &str =
    "5f355f60015b8215601b57906001018091029160019003916005565b9150505f5260205ff3";
pub const COUNTER_BYTECODE_HEX: &str = "6080806040526004361015610012575f80fd5b5f3560e01c9081633fb5c1cb146101035781638381f58a146100cc575063d09de08a1461003d575f80fd5b346100c8575f7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126100c8575f547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff811461009b576001015f55005b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f80fd5b346100c8575f7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126100c8576020905f548152f35b346100c85760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126100c8576004355f5500fea264697066735822122019105920c33dab502c8d24693bd24f1b4b0fbb562ad3a475173a15b7aa3574f064736f6c63430008190033";
pub const ERC20_BYTECODE_HEX: &str = include_str!("../../../examples/erc20/contract.hex");
pub const UNISWAP_V2_BYTECODE_HEX: &str = include_str!("../../../examples/uniswap_v2/contract.hex");

#[derive(Clone, Debug, Default)]
pub struct Bench {
    pub name: &'static str,
    pub bytecode: Vec<u8>,
    pub calldata: Vec<u8>,
    pub native: Option<fn()>,
}

pub fn get_benches() -> Vec<Bench> {
    sol! {
        function mint(address to) public payable;
        function balanceOf(address account) public payable;
        function transfer(address recipient, uint256 amount) public payable;
        function increment() public;
    }
    vec![
        Bench {
            name: "fibonacci",
            bytecode: hex::decode(FIBONACCI_BYTECODE_HEX).unwrap(),
            calldata: num_to_calldata(1000),
            native: Some(|| {
                black_box(fibonacci_rust(black_box(U256::from(1000))));
            }),
        },
        Bench {
            name: "factorial",
            bytecode: hex::decode(FACTORIAL_BYTECODE_HEX).unwrap(),
            calldata: U256::from(1000).to_be_bytes_vec(),
            ..Default::default()
        },
        // Note: There may be differences in performance results when using different Runtime Contexts or different Memory/Stoage DBs
        Bench {
            name: "counter",
            bytecode: hex::decode(COUNTER_BYTECODE_HEX).unwrap(),
            calldata: incrementCall {}.abi_encode(),
            ..Default::default()
        },
        // Note: There may be differences in performance results when using different Runtime Contexts or different Memory/Stoage DBs
        Bench {
            name: "erc20_transfer",
            bytecode: hex::decode(ERC20_BYTECODE_HEX).unwrap(),
            calldata: transferCall {
                recipient: address!("1234000000000000000000000000000000000000"),
                amount: U256::from(0),
            }
            .abi_encode(),
            ..Default::default()
        },
        // Note: There may be differences in performance results when using different Runtime Contexts or different Memory/Stoage DBs
        Bench {
            name: "uniswap_v2_transfer",
            bytecode: hex::decode(UNISWAP_V2_BYTECODE_HEX).unwrap(),
            calldata: transferCall {
                recipient: address!("1234000000000000000000000000000000000000"),
                amount: U256::from(0),
            }
            .abi_encode(),
            ..Default::default()
        },
    ]
}

pub fn num_to_calldata(num: u32) -> Vec<u8> {
    let mut calldata = vec![0x00; 32];
    calldata[28..32].copy_from_slice(&num.to_be_bytes());
    calldata
}
