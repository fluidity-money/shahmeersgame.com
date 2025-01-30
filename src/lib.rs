#![cfg_attr(target_arch = "wasm32", no_main, no_std)]

extern crate alloc;

#[cfg(target_arch = "wasm32")]
use alloc::vec::Vec;

mod host;
mod storage;
pub use storage::*;
pub mod error;
use error::*;
mod calldata;
pub mod events;
mod utils;
pub use utils::*;
mod maths;

#[macro_use]
pub mod host_stg_call;
mod stg_call;
pub mod wasm_stg_call;

use maths::SCALING_AMT;

use stylus_sdk::{alloy_primitives::*, block, evm, prelude::*};

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
        assert_or!(
            !dilution_stg_concepts.is_zero() && !dilution_stg_submitters.is_zero(),
            Error::ZeroSTGAmt
        );
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
        assert_or!(!beneficiary.is_zero(), Error::BadBeneficiary);
        self.submitters.setter(concept).set(beneficiary);
        evm::log(events::Registered {
            concept,
            beneficiary,
        });
        Ok(())
    }

    /// Vote, using the delegated voting power of the OZ token at the point in
    /// time of the epoch creation. Returns the STG amount that was taken.
    pub fn add_votes(&mut self, concept: Concept, stg_amt: U256) -> R<U256> {
        assert_or!(self.enabled.get(), Error::NotEnabled);
        assert_or!(
            !self.submitters.get(concept).is_zero(),
            Error::ConceptNotRegistered
        );
        assert_or!(
            !self.concept_is_correct.get(concept),
            Error::ConceptDoneAlready
        );
        let mut e = self.epochs.setter(self.epochs.len() - 1).unwrap();
        // We can't allow this to happen if the operator has already picked winners.
        assert_or!(!e.winners_picked.get(), Error::WinnersPicked);
        let stg_already_spent = e.user_stg_amts.get(msg_sender());
        let stg_amt = {
            // Get the past votes at the point in time of the epoch's creation.
            let votes = stg_call::get_past_votes(
                self.token_addr.get(),
                msg_sender(),
                U256::from(e.time.get()),
            )?;
            assert_or!(!votes.is_zero(), Error::ZeroVotes);
            // Get the past amount spent by the user in this epoch.
            min(stg_amt, votes - stg_already_spent)
        };
        assert_or!(!stg_amt.is_zero(), Error::NotEnoughToken);
        let quad_amt = maths::stg_to_quad(stg_amt);
        storage_adds! {
            (e.concept_quad_amts, concept, quad_amt);
            (e.concept_stg_amts, concept, stg_amt);
            (e.user_stg_amts, msg_sender(), stg_amt);
            (e.user_concept_quad_amts, msg_sender(), concept, quad_amt);
            (e.global_quad_amts, quad_amt);
        };
        Ok(stg_amt)
    }

    /// Take some STG from a concept. We can't allow this to take place if the
    /// winners were already called, though it would be waste without harm!
    pub fn take_votes(&mut self, concept: Concept, stg_amt: U256) -> R<()> {
        assert_or!(self.enabled.get(), Error::NotEnabled);
        let mut e = self.epochs.setter(self.epochs.len() - 1).unwrap();
        assert_or!(!e.winners_picked.get(), Error::WinnersPicked);
        assert_or!(
            e.user_stg_amts.get(msg_sender()) >= stg_amt,
            Error::NotEnoughToken
        );
        // Go to take some quadratic voting power from the concept given,
        // reverting if the user didn't supply enough for the STG they're taking.
        let quad_amt = maths::stg_to_quad(stg_amt);
        assert_or!(
            e.user_concept_quad_amts.getter(msg_sender()).get(concept) >= quad_amt,
            Error::ConceptNoUserQuad
        );
        // Now that we've confirmed they actually had that much, we can take from
        // the global allocations. Of course, with checked storage, it's not needed to
        // actually check above like we did, but, thanks to Stylus' caching, we can
        // do this check for a better user experience without too great a detriment to
        // the code's gas profile.
        storage_subs! {
            (e.concept_quad_amts, concept, quad_amt);
            (e.concept_stg_amts, concept, stg_amt);
            (e.user_stg_amts, msg_sender(), stg_amt);
            (e.user_concept_quad_amts, msg_sender(), concept, quad_amt);
            (e.global_quad_amts, quad_amt);
        };
        Ok(())
    }

    /// Choose winners by going through the amount of quadratic votes in each
    /// concept, until the minimum amount in a outcome is greater than the
    /// amount tracked as being allocated in the entire epoch. Bump the epoch
    /// for the entire game, and return winning proposals amount of STG that
    /// could be received if they come true.
    pub fn choose_winners(
        &mut self,
        concept_count: u64,
        concepts: Vec<Concept>,
    ) -> R<Vec<(Concept, U256)>> {
        assert_or!(self.enabled.get(), Error::NotEnabled);
        assert_or!(self.operator_addr.get() == msg_sender(), Error::NotOperator);
        let mut e = self.epochs.setter(self.epochs.len() - 1).unwrap();
        // We can't allow someone to make a mistake by somehow calling this twice.
        assert_or!(!e.winners_picked.get(), Error::WinnersPicked);
        // It might be better to prescreen the concepts to see if they
        // match, but this is fine.
        assert_or!(!concepts.is_empty(), Error::ConceptsEmpty);
        let mut concepts = concepts
            .into_iter()
            .map(|c| (c, e.concept_quad_amts.get(c)))
            .collect::<Vec<(Concept, U256)>>();
        // Sort this inplace, replacing what's there.
        concepts.sort_unstable_by(|(_, x), (_, y)| y.cmp(x));
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
            assert_or!(!min_concept.is_zero(), Error::ConceptZeroSupplied);
            assert_or!(
                *min_concept > e.global_quad_amts.get() - concepts_allocated,
                Error::BadConcepts
            );
        }
        // Now that we know the actual winning concepts, we need to pick the ones
        // that are under the number of concepts we want to declare as winners.
        // We need to apportion the size of the STG token we want to release each
        // epoch. Here, we take as much as we can (up to the amount asked).
        let concepts = &concepts
            .into_iter()
            .take(concept_count as usize)
            .collect::<Vec<_>>();
        // We take the summed amount to know how to dilute the share of the STG
        // token to distribute.
        let winning_concept_sum: U256 = concepts.iter().map(|(_, x)| x).sum();
        e.winners_picked.set(true);
        concepts
            .iter()
            .map(|(c, quad_amt)| {
                let stg_pct_for_winner = (quad_amt * SCALING_AMT) / winning_concept_sum;
                let stg_amt_for_winner =
                    (self.dilution_stg_correct_concepts.get() * stg_pct_for_winner) / SCALING_AMT;
                // This amount will be claimable by the predictors of this outcome if it comes true.
                e.concept_stg_claimable.setter(*c).set(stg_amt_for_winner);
                evm::log(events::WinnerChosen {
                    concept: *c,
                    stgToGain: stg_amt_for_winner,
                });
                // Now that we've had a winner chosen, we need to send the
                // submitters a fixed dividend for their idea.
                stg_call::transfer(
                    self.token_addr.get(),
                    self.submitters.get(*c),
                    self.dilution_stg_submitters.get(),
                )?;
                Ok((*c, stg_amt_for_winner))
            })
            .collect::<R<Vec<_>>>()
    }

    /// Declare that these winners were correct, and claimable.
    pub fn pick_winners_that_accomplished(&mut self, epoch: u64, concepts: Vec<Concept>) -> R<()> {
        assert_or!(self.enabled.get(), Error::NotEnabled);
        assert_or!(self.operator_addr.get() == msg_sender(), Error::NotOperator);
        let e = self.epochs.setter(epoch).ok_or(Error::BadEpoch)?;
        for c in concepts {
            assert_or!(
                !e.concept_stg_claimable.get(c).is_zero(),
                Error::BadConcepts
            );
            self.concept_is_correct.setter(c).set(true);
            evm::log(events::ConceptCameTrue { concept: c });
        }
        Ok(())
    }

    /// Draw down winning amount on behalf of a user, sending it to their
    /// address. It should be fine to allow someone to call this without
    /// checks, since it will send based on delegations.
    pub fn draw_down_winner(&mut self, epoch: u64, concept: Concept, winner: Address) -> R<U256> {
        assert_or!(self.enabled.get(), Error::NotEnabled);
        assert_or!(
            self.concept_is_correct.get(concept),
            Error::NotCorrectConcept
        );
        let mut e = self.epochs.setter(epoch).ok_or(Error::BadEpoch)?;
        assert_or!(
            !e.user_concept_claimed.getter(winner).get(concept),
            Error::AlreadyClaimed
        );
        // Get the user's share of the pool of quadratic tokens invested in this outcome.
        let pct_of_quad = (e.user_concept_quad_amts.get(msg_sender()).get(concept) * SCALING_AMT)
            / e.concept_quad_amts.get(concept);
        let stg_amt = (pct_of_quad * e.concept_stg_claimable.get(concept)) / SCALING_AMT;
        stg_call::transfer(self.token_addr.get(), winner, stg_amt)?;
        e.user_concept_claimed
            .setter(msg_sender())
            .setter(concept)
            .set(true);
        evm::log(events::STGClaimed {
            concept,
            winner,
            amt: stg_amt,
        });
        Ok(stg_amt)
    }

    /// The operator bumps the epoch for whatever reason. Maybe voting has
    /// been concluded, and we're ready for the next batch.
    pub fn bump_epoch(&mut self) -> R<u64> {
        assert_or!(self.enabled.get(), Error::NotEnabled);
        assert_or!(self.operator_addr.get() == msg_sender(), Error::NotOperator);
        let prev_epoch = self.epochs.len() - 1;
        self.epochs.grow().time.set(U64::from(block::timestamp()));
        evm::log(events::EpochBumped {
            prevEpoch: U256::from(prev_epoch),
        });
        Ok(prev_epoch as u64 + 1)
    }

    pub fn get_votes(&self, c: Concept) -> R<U256> {
        let e = self.epochs.getter(self.epochs.len() - 1).unwrap();
        Ok(e.concept_quad_amts.get(c))
    }

    pub fn get_s_t_g(&self, c: Concept) -> R<U256> {
        let e = self.epochs.getter(self.epochs.len() - 1).unwrap();
        Ok(e.concept_stg_amts.get(c))
    }

    pub fn are_winners_picked(&self) -> R<bool> {
        let e = self.epochs.getter(self.epochs.len() - 1).unwrap();
        Ok(e.winners_picked.get())
    }

    pub fn get_user_votes(&self, c: Concept, user: Address) -> R<U256> {
        let e = self.epochs.getter(self.epochs.len() - 1).unwrap();
        Ok(e.user_concept_quad_amts.getter(user).get(c))
    }

    pub fn get_user_s_t_g_spent(&self, user: Address) -> R<U256> {
        let e = self.epochs.getter(self.epochs.len() - 1).unwrap();
        Ok(e.user_stg_amts.get(user))
    }

    pub fn is_concept_correct(&self, c: Concept) -> R<bool> {
        Ok(self.concept_is_correct.get(c))
    }

    pub fn is_concept_claimable(&self, c: Concept, user: Address) -> R<bool> {
        let e = self.epochs.getter(self.epochs.len() - 1).unwrap();
        Ok(e.user_concept_claimed.getter(user).get(c))
    }
}
