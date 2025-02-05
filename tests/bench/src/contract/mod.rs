use dora_primitives::{keccak256, Address, B256, I256, U256};
use ruint::aliases::U160;

pub mod erc20;
pub mod uniswapv3;

pub(crate) fn str_to_u256(text: &str) -> U256 {
    assert!(text.len() < 32);
    let str_b256 = B256::bit_or(
        B256::right_padding_from(text.as_bytes()),
        B256::left_padding_from(&[(text.len() * 2) as u8]),
    );
    str_b256.into()
}

pub(crate) fn indices_to_u256(slot: U256, indices: &[U256]) -> U256 {
    let mut result = B256::from(U256::from(slot));
    for index in indices {
        let to_prepend = B256::from(U256::from(*index));
        result = keccak256([to_prepend.as_slice(), result.as_slice()].concat())
    }
    result.into()
}

#[inline]
pub(crate) fn address_to_u256(address: Address) -> U256 {
    let address_u160: U160 = address.into();
    U256::from(address_u160)
}

#[inline]
pub(crate) fn tick_to_u256(tick: i32) -> U256 {
    let tick_i256 = I256::try_from(tick).unwrap();
    tick_i256.into_raw()
}

#[inline]
pub(crate) fn keccak256_slice_list(slice_list: &[&[u8]]) -> B256 {
    keccak256(slice_list.concat())
}
