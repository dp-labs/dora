use dora_primitives::{keccak256, Address, B256, U256};
use ruint::aliases::U160;

pub mod erc20;
pub mod uniswap;

pub(crate) fn str_to_u256(text: &str) -> U256 {
    assert!(text.len() < 32);
    let encoded_as_b256 = B256::bit_or(
        B256::right_padding_from(text.as_bytes()),
        B256::left_padding_from(&[(text.len() * 2) as u8]),
    );
    encoded_as_b256.into()
}

pub(crate) fn indices_to_u256(slot: U256, indices: &[U256]) -> U256 {
    let mut result = B256::from(U256::from(slot));
    for index in indices {
        let to_prepend = B256::from(U256::from(*index));
        result = keccak256([to_prepend.as_slice(), result.as_slice()].concat())
    }
    result.into()
}

pub(crate) fn address_to_u256(address: Address) -> U256 {
    let encoded_as_u160: U160 = address.into();
    U256::from(encoded_as_u160)
}
