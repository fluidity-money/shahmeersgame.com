#[cfg(target_arch = "wasm32")]
pub use crate::wasm_stg_call::*;

#[cfg(not(target_arch = "wasm32"))]
pub use crate::host_stg_call::*;
