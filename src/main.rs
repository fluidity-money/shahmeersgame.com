#![cfg_attr(target_arch = "wasm32", no_main, no_std)]

// This is needed to know how to enter your Stylus program. This is
// created using a macro.
pub use libshahmeersgame::user_entrypoint;
