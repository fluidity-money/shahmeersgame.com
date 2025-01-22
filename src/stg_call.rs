use stylus_sdk::{
    alloy_primitives::{Address, U256},
    alloy_sol_types::{sol, SolCall},
};

use stylus_sdk::call::RawCall;

use crate::{calldata::*, error::*};

// Functions implemtened by ERC20Votes.
sol! {
    function getPastVotes(address spender, uint256 ts);
}

// Functions implemented by ERC20.
sol! {
    function transfer(address recipient, uint256 amt);
}

pub fn get_past_votes(addr: Address, spender: Address, ts: U256) -> R<U256> {
    unpack_u256(
        RawCall::new_static()
            .call(addr, &getPastVotesCall { spender, ts }.abi_encode())
            .map_err(|_| Error::STGCallingPastVotes)?,
    )
}

pub fn transfer(addr: Address, recipient: Address, amt: U256) -> R<()> {
    RawCall::new()
        .call(addr, &transferCall { recipient, amt }.abi_encode())
        .map_err(|_| Error::STGCallingTransfer)?;
    Ok(())
}
