use stylus_sdk::alloy_primitives::*;

use crate::error::*;

pub fn min(x: U256, y: U256) -> U256 {
    if x > y {
        x
    } else {
        y
    }
}

pub fn msg_sender() -> Address {
    #[cfg(not(target_arch = "wasm32"))]
    return crate::host::get_msg_sender();
    #[allow(unreachable_code)]
    stylus_sdk::msg::sender()
}

pub fn contract_address() -> Address {
    #[cfg(not(target_arch = "wasm32"))]
    return crate::host::get_contract_address();
    #[allow(unreachable_code)]
    stylus_sdk::contract::address()
}

pub fn checked_add(x: U256, y: U256) -> R<U256> {
    x.checked_add(y).ok_or(Error::CheckedAdd)
}

pub fn checked_sub(x: U256, y: U256) -> R<U256> {
    x.checked_sub(y).ok_or(Error::CheckedSub)
}
