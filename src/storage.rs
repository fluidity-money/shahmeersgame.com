use stylus_sdk::{alloy_primitives::*, prelude::*, storage::*};

pub type Concept = FixedBytes<32>;

#[cfg(not(target_arch = "wasm32"))]
use proptest::prelude::*;

#[storage]
pub struct GameEpoch {
    pub time: StorageU64,

    // Quadratic amounts invested in a concept.
    pub concept_quad_amts: StorageMap<Concept, StorageU256>,

    // STG amounts invested in a concept.
    pub concept_stg_amts: StorageMap<Concept, StorageU256>,

    // Token amounts invested by a user.
    pub user_stg_amts: StorageMap<Address, StorageU256>,

    // Quadratic amounts invested by a user into a specific concept.
    pub user_concept_quad_amts: StorageMap<Address, StorageMap<Concept, StorageU256>>,

    // Quadratic amounts invested in every concept by users.
    pub global_quad_amts: StorageU256,

    // Has the operator picked winners for this epoch?
    pub winners_picked: StorageBool,

    // Future STG claimable by users who invested in this winning concept.
    pub concept_stg_claimable: StorageMap<Concept, StorageU256>,

    // Whether the future STG claimable by users was collected.
    pub user_concept_claimed: StorageMap<Address, StorageMap<Concept, StorageBool>>,
}

#[entrypoint]
#[storage]
pub struct ShahmeersGame {
    pub enabled: StorageBool,
    pub version: StorageU256,

    pub token_addr: StorageAddress,
    pub operator_addr: StorageAddress,

    // The amount of STG to release each epoch to the chosen winners,
    // diluting the users who bet incorrectly on their proposals.
    pub dilution_stg_correct_concepts: StorageU256,

    // The amount of STG to release each epoch to the submitters
    // of ideas that are included.
    pub dilution_stg_submitters: StorageU256,

    // Submitters of ideas, for sending the token compensation for having their ideas
    // marked for inclusion.
    pub submitters: StorageMap<Concept, StorageAddress>,

    // Submitters who are entitled to a fixed amount for having their idea
    // be included for submission.
    pub submitters_claimable: StorageMap<Address, StorageU256>,

    // Was this concept correct? We do this to remember in perpetuity so we don't have a
    // mysterious repeat of this concept's submission.
    pub concept_is_correct: StorageMap<Concept, StorageBool>,

    pub epochs: StorageVec<GameEpoch>,
}

#[cfg(not(target_arch = "wasm32"))]
impl core::fmt::Debug for ShahmeersGame {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "ShahmeersGame {{ enabled: {:?}, version: {:?}, token addr: {:?}, operator addr: {:?}, dilution stg correct concepts: {:?}, dilution stg submitters: {:?}, .. }}",
            self.enabled,
            self.version,
            self.token_addr,
            self.operator_addr,
            self.dilution_stg_correct_concepts,
            self.dilution_stg_submitters,
        )
    }
}

#[macro_export]
macro_rules! define_storage_op {
    ($name:ident, $f:expr) => {
        #[macro_export]
        macro_rules! $name {
            ($field:expr, $val:expr) => {{
                let a = $field.get();
                $field.set($f(a, $val)?)
            }};
            ($field:expr, $h:expr, $val:expr) => {{
                let a = $field.get($h);
                $field.setter($h).set($f(a, $val)?)
            }};
            ($field:expr, $h:expr, $k:expr, $val:expr) => {{
                let a = $field.get($h).get($k);
                $field.setter($h).setter($k).set($f(a, $val)?)
            }};
        }
    };
}

define_storage_op!(storage_add, checked_add);
define_storage_op!(storage_sub, checked_sub);

#[macro_export]
macro_rules! storage_adds {
    ( $( $args:tt );* $(;)? ) => {$(storage_add! $args;)*};
}

#[macro_export]
macro_rules! storage_subs {
    ( $( $args:tt );* $(;)? ) => {$(storage_sub! $args;)*};
}

#[macro_export]
macro_rules! storage_set_fields {
    ($ty: ty, $i:ident, { $($field:ident),+ $(,)? }) => {
        {
            let mut c = unsafe { <$ty>::new($i, 0) };
            $(
                c.$field.set($field);
            )+
            c
        }
    };
}

/// Simple strategy that generates values up to a million.
#[cfg(not(target_arch = "wasm32"))]
pub fn strat_tiny_u256() -> impl Strategy<Value = U256> {
    (0..1_000_000).prop_map(|x| U256::from(x))
}

#[cfg(not(target_arch = "wasm32"))]
fn strat_fixed_bytes_sizeable<const N: usize>() -> impl Strategy<Value = FixedBytes<N>> {
    // Create a slice of fixed bytes, with a preference for the lower side, a
    // la how I recall seeing Parity's Ethereum client do it. This has a 33%
    // chance of filling out a third of the lower bits, which, in our
    // interpretation, is decoded as big endian in the next function, so
    // the right side, a 33% chance of two thirds, and a 33% chance of
    // everything is potentially filled out.
    (0..3).prop_perturb(move |s, mut rng| {
        let mut x: [u8; N] = [0u8; N];
        let q = N / 3;
        if s == 2 {
            for i in q * 2..N {
                x[N - i - 1] = rng.gen();
            }
        }
        if s >= 1 {
            for i in q..q * 2 {
                x[N - i - 1] = rng.gen();
            }
        }
        for i in 0..q {
            x[N - i - 1] = rng.gen();
        }
        FixedBytes::<N>::from(x)
    })
}

#[cfg(not(target_arch = "wasm32"))]
pub fn strat_large_u256() -> impl Strategy<Value = U256> {
    strat_fixed_bytes_sizeable::<32>().prop_map(|x| U256::from_be_bytes(x.into()))
}

#[cfg(not(target_arch = "wasm32"))]
pub fn strat_fixed_bytes<const N: usize>() -> impl Strategy<Value = FixedBytes<N>> {
    strat_fixed_bytes_sizeable::<N>()
}

#[cfg(not(target_arch = "wasm32"))]
pub fn strat_address() -> impl Strategy<Value = Address> {
    proptest::arbitrary::any::<[u8; 20]>().prop_map(Address::new)
}

#[cfg(not(target_arch = "wasm32"))]
pub fn strat_shahmeers_game() -> impl Strategy<Value = ShahmeersGame> {
    (
        strat_large_u256().no_shrink(), // Storage offset
        any::<bool>(),
        strat_large_u256(),
        strat_address(),
        strat_address(),
        strat_tiny_u256(),
        strat_tiny_u256(),
    )
        .prop_map(
            |(
                i,
                enabled,
                version,
                token_addr,
                operator_addr,
                dilution_stg_correct_concepts,
                dilution_stg_submitters,
            )| {
                storage_set_fields!(ShahmeersGame, i, {
                    enabled,
                    version,
                    token_addr,
                    operator_addr,
                    dilution_stg_correct_concepts,
                    dilution_stg_submitters,
                })
            },
        )
}
