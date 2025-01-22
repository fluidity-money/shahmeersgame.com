#![cfg_attr(target_arch = "wasm32", no_main, no_std)]

extern crate alloc;

#[cfg(target_arch = "wasm32")]
use alloc::vec::Vec;

mod storage;
pub use storage::*;
pub mod error;
use error::*;
mod calldata;
pub mod events;
mod stg_call;
mod utils;
use utils::*;
mod maths;

use maths::SCALING_AMT;

use stylus_sdk::{alloy_primitives::*, block, evm, msg, prelude::*};

#[public]
impl ShahmeersGame {
    pub fn ctor(
        &mut self,
        token: Address,
        operator: Address,
        dilution_stg_concepts: U256,
        dilution_stg_submitters: U256,
    ) -> R<()> {
        assert_or!(self.version.is_zero(), Error::AlreadyCreated);
        self.enabled.set(true);
        self.version.set(U256::from(1));
        self.token_addr.set(token);
        self.operator_addr.set(operator);
        self.dilution_stg_correct_concepts
            .set(dilution_stg_concepts);
        self.dilution_stg_submitters.set(dilution_stg_submitters);
        let mut e = self.epochs.grow();
        e.time.set(U64::from(block::timestamp()));
        Ok(())
    }

    /// Register a new concept, claiming any fees for whoever
    /// uses it to this address.
    pub fn register(&mut self, concept: Concept, beneficiary: Address) -> R<()> {
        assert_or!(self.enabled.get(), Error::NotEnabled);
        assert_or!(
            self.submitters.get(concept).is_zero(),
            Error::ConceptRegistered
        );
        self.submitters.setter(concept).set(beneficiary);
        evm::log(events::Registered {
            concept,
            beneficiary,
        });
        Ok(())
    }

    // Vote, using the delegated voting power of the OZ token at the point in
    // time of the epoch creation.
    pub fn add_votes(&mut self, concept: Concept, stg_amt: U256) -> R<U256> {
        assert_or!(self.enabled.get(), Error::NotEnabled);
        assert_or!(
            !self.submitters.get(concept).is_zero(),
            Error::ConceptNotRegistered
        );
        let mut e = self.epochs.setter(self.epoch_count.get()).unwrap();
        // We can't allow this to happen if the operator has already picked winners.
        assert_or!(!e.winners_picked.get(), Error::WinnersPicked);
        let stg_already_spent = e.user_stg_amts.get(msg::sender());
        let stg_amt = {
            // Get the past votes at the point in time of the epoch's creation.
            let votes = stg_call::get_past_votes(
                self.token_addr.get(),
                msg::sender(),
                U256::from(e.time.get()),
            )?;
            // Get the past amount spent by the user in this epoch.
            min(stg_amt, votes - stg_already_spent)
        };
        assert_or!(!stg_amt.is_zero(), Error::NotEnoughToken);
        let quad_amt = maths::stg_to_quad(stg_amt);
        // Now that we have the amount that the user wanted to spend, we
        // can compute the amount to actually take, by doing a roundtrip.
        let stg_amt = maths::quad_to_stg(quad_amt);
        storage_adds! {
            (e.concept_quad_amts, concept, quad_amt);
            (e.concept_stg_amts, concept, stg_amt);
            (e.user_quad_amts, msg::sender(), quad_amt);
            (e.user_stg_amts, msg::sender(), stg_amt);
            (e.user_concept_quad_amts, msg::sender(), concept, quad_amt);
            (e.global_quad_amts, quad_amt);
        }
        Ok(stg_amt)
    }

    /// Choose winners by going through the amount of quadratic votes in each
    /// concept, until the minimum amount in a outcome is greater than the
    /// amount tracked as being allocated in the entire epoch. Bump the epoch
    /// for the entire game, and mark teh winning proposals amount of STG that
    /// could be received if they come true.
    pub fn choose_winners(
        &mut self,
        concept_count: u64,
        concepts: Vec<Concept>,
    ) -> R<Vec<(Concept, U256)>> {
        assert_or!(self.enabled.get(), Error::NotEnabled);
        assert_or!(
            self.operator_addr.get() == msg::sender(),
            Error::NotOperator
        );
        let mut e = self.epochs.setter(self.epoch_count.get()).unwrap();
        // We can't allow someone to make a mistake by somehow calling this twice.
        assert_or!(!e.winners_picked.get(), Error::WinnersPicked);
        // It might be better to prescreen the concepts to see if they
        // match, but this is fine.
        assert_or!(!concepts.is_empty(), Error::ConceptsEmpty);
        let mut concepts = concepts
            .into_iter()
            .map(|c| (c, e.concept_quad_amts.get(c)))
            .collect::<Vec<(Concept, U256)>>();
        concepts.sort_unstable_by(|(_, x), (_, y)| y.cmp(&x));
        // The problem with this dedup is that it will be wasteful if
        // someone makes a mistake with their calldata here. But it
        // exists to prevent abuse, not to be efficient.
        concepts.dedup();
        // Now that we've sorted the concepts supplied, we can sum the
        // amounts, then compare the difference between the tracked
        // amount as being allocated globally, and the minimum here.
        {
            let concepts_allocated: U256 = concepts.iter().map(|(_, x)| x).sum();
            let (_, min_concept) = concepts.last().unwrap();
            assert_or!(
                *min_concept > e.global_quad_amts.get() - concepts_allocated,
                Error::BadConcepts
            );
        }
        // Now that we know the actual winning concepts, we need to pick the ones
        // that are under the number of concepts we want to declare as winners.
        // We need to apportion the size of the STG token we want to release each
        // epoch.
        let concepts = &concepts[..concept_count as usize];
        // We take the summed amount to know how to dilute the share of the STG
        // token to distribute.
        let winning_concept_sum: U256 = concepts.iter().map(|(_, x)| x).sum();
        for (c, quad_amt) in concepts {
            let stg_amt_for_winners = (quad_amt * SCALING_AMT / winning_concept_sum) / SCALING_AMT;
            // This amount will be claimable by the predictors of this outcome if it comes true.
            e.concept_stg_claimable.setter(*c).set(stg_amt_for_winners);
            evm::log(events::WinnerChosen {
                concept: *c,
                stgToGain: stg_amt_for_winners,
            });
            // Now that we've had a winner chosen, we need to send the
            // submitters a fixed dividend for their idea.
            stg_call::transfer(
                self.token_addr.get(),
                self.submitters.get(*c),
                self.dilution_stg_submitters.get(),
            )?;
        }
        e.winners_picked.set(true);
        Ok(concepts.to_vec())
    }

    // The operator bumps the epoch for whatever reason. Maybe voting has
    // been concluded, and we're ready for the next batch.
    pub fn bump_epoch(&mut self) -> R<()> {
        assert_or!(self.enabled.get(), Error::NotEnabled);
        assert_or!(
            self.operator_addr.get() == msg::sender(),
            Error::NotOperator
        );
        let prev_epoch = self.epoch_count.get();
        self.epoch_count.set(prev_epoch + U256::from(1));
        evm::log(events::EpochBumped {
            prevEpoch: prev_epoch,
        });
        Ok(())
    }

}
