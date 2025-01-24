#![cfg(not(target_arch = "wasm32"))]

use stylus_sdk::alloy_primitives::*;

use std::{cell::RefCell, collections::HashMap};

use crate::error::*;

thread_local! {
    static PAST_VOTES: RefCell<HashMap<Address, U256>> = RefCell::new(HashMap::new());
}

fn give_votes(spender: Address, amt: U256) {
    PAST_VOTES.with(|v| {
        let mut v = v.borrow_mut();
        v.insert(spender, amt);
    })
}

fn reset_all_votes() {
    PAST_VOTES.with(|v| {
        let mut v = v.borrow_mut();
        v.clear();
    })
}

pub fn use_points_f<T>(
    spenders: HashMap<Address, U256>,
    f: impl FnOnce() -> R<T>,
) -> Result<T, Error> {
    for (r, amt) in spenders.clone() {
        give_votes(r, amt)
    }
    let x = f();
    reset_all_votes();
    let v = x?;
    Ok(v)
}

#[macro_export]
macro_rules! use_votes {
    (
        { $( $key:expr => $value:expr ),* $(,)? },
        $func:expr
    ) => {
        $crate::host_stg_call::use_points_f(
            map_macro::hash_map! { $( $key => $value ),* },
            || { $func }
        ).unwrap()
    };
}

pub fn get_past_votes(_addr: Address, spender: Address, _ts: U256) -> R<U256> {
    Ok(PAST_VOTES.with(|v| *v.borrow().get(&spender).unwrap_or(&U256::ZERO)))
}

pub fn transfer(_addr: Address, _recipient: Address, _amt: U256) -> R<()> {
    Ok(())
}
