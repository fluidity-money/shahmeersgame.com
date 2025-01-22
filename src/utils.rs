use stylus_sdk::alloy_primitives::U256;

pub fn min(x: U256, y: U256) -> U256 {
    if x > y {
        x
    } else {
        y
    }
}
