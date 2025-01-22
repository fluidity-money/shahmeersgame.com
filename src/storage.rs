use stylus_sdk::{alloy_primitives::*, prelude::*, storage::*};

pub type Concept = FixedBytes<32>;

#[storage]
pub struct GameEpoch {
    pub time: StorageU64,

    // Quadratic amounts invested in a concept.
    pub concept_quad_amts: StorageMap<Concept, StorageU256>,

    // STG amounts invested in a concept.
    pub concept_stg_amts: StorageMap<Concept, StorageU256>,

    // Quadratic amounts invested by a user.
    pub user_quad_amts: StorageMap<Address, StorageU256>,

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

    // Did this concecpt come true?
    pub concept_is_correct: StorageMap<Concept, StorageBool>,
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

    // Number of ongoing campaigns having taken place in the past.
    pub epoch_count: StorageU256,

    // Submitters of ideas, for sending the token compensation for having their ideas
    // marked for inclusion.
    pub submitters: StorageMap<Concept, StorageAddress>,

    // Submitters who are entitled to a fixed amount for having their idea
    // be included for submission.
    pub submitters_claimable: StorageMap<Address, StorageU256>,

    pub epochs: StorageVec<GameEpoch>,
}

#[macro_export]
macro_rules! checked_add {
    ($x:expr, $y:expr) => {
        $x.checked_add($y).ok_or(Error::CheckedAdd)?
    };
}

#[macro_export]
macro_rules! storage_add {
    ($field:expr, $val:expr) => {
        let a = $field.get();
        $field.set(checked_add!(a, $val));
    };
    ($field:expr, $h:expr, $val:expr) => {
        let a = $field.get($h);
        $field.setter($h).set(checked_add!(a, $val));
    };
    ($field:expr, $h:expr, $k:expr, $val:expr) => {
        let a = $field.get($h).get($k);
        $field.setter($h).setter($k).set(checked_add!(a, $val));
    };
}

#[macro_export]
macro_rules! storage_adds {
    ( $( $args:tt );* $(;)? ) => {
        $(
            storage_add! $args;
        )*
    };
}
