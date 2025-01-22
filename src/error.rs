#[cfg(target_arch = "wasm32")]
use alloc::{vec::Vec, vec};

#[repr(C)]
pub enum Error {
    AlreadyCreated,
    NotEnabled,
    ConceptRegistered,
    ConceptNotRegistered,
    STGCallingTransfer,
    STGCallingPastVotes,
    STGUnpacking,
    NotEnoughToken,
    CheckedAdd,
    ConceptsEmpty,
    BadConcepts,
    WinnersPicked,
    NotOperator,
}

impl From<Error> for u8 {
    fn from(v: Error) -> Self {
        unsafe { *<*const _>::from(&v).cast::<u8>() }
    }
}

pub type R<A> = Result<A, Error>;

impl From<Error> for Vec<u8> {
    fn from(v: Error) -> Self {
        vec![0x77, 0x70, v.into()]
    }
}

#[macro_export]
macro_rules! assert_or {
    ($cond:expr, $err:expr) => {
        if !($cond) {
            Err($err)?;
        }
    };
}
