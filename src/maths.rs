use stylus_sdk::alloy_primitives::U256;

pub const SCALING_AMT: U256 = U256::from_limbs([1000, 0, 0, 0]);

/// Compute the quadratic voting power of a STG amount using the Babylonian method.
pub fn stg_to_quad(x: U256) -> U256 {
    // Since the Alloy implementation uses some floating point conversions
    // the Stylus runtime doesn't have. Rounds down.
    if x.is_zero() {
        return U256::ZERO;
    }
    let mut z = (x >> 1) + U256::from(1);
    let mut y = x;
    while z < y {
        y = z;
        z = (x / z + z) >> 1;
    }
    if y * y > x {
        y -= U256::from(1);
    }
    y
}

#[cfg(not(target_arch = "wasm32"))]
mod test {
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_stg_to_quad(x in crate::storage::strat_tiny_u256()) {
            assert_eq!(x.root(2), super::stg_to_quad(x));
        }
    }
}
