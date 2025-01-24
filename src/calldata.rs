use crate::error::*;

use stylus_sdk::alloy_primitives::U256;

#[cfg(target_arch = "wasm32")]
use alloc::vec::Vec;

pub fn unpack_u256(x: Vec<u8>) -> R<U256> {
    x.try_into()
        .map_err(|_| Error::STGUnpacking)
        .map(U256::from_be_bytes::<32>)
}

#[test]
fn test_unpack_u256() {
    assert_eq!(
        U256::from(123),
        unpack_u256(const_hex::decode("000000000000000000000000000000000000000000000000000000000000007b").unwrap()).unwrap(),
    );
}
