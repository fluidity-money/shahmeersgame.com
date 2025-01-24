
use stylus_sdk::alloy_primitives::U256;

pub const SCALING_AMT: U256 = U256::from_limbs([1000, 0, 0, 0]);

/// Compute the quadratic voting power of a STG amount.
pub fn stg_to_quad(x: U256) -> U256 {
    x.root(2)
}
