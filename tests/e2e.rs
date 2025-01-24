#![cfg(not(target_arch = "wasm32"))]

use proptest::prelude::*;

use stylus_sdk::{alloy_primitives::*};

use libshahmeersgame::*;

proptest! {
    #[test]
    fn test_best_user_story(
        mut c in strat_shahmeers_game(),
        votes_1 in strat_tiny_u256(),
        votes_2 in strat_tiny_u256(),
        concept in strat_fixed_bytes::<32>(),
    ) {
        c.enabled.set(false);
        c.version.set(U256::from(0));
        c.ctor(
            c.token_addr.get(),
            c.operator_addr.get(),
            c.dilution_stg_correct_concepts.get(),
            c.dilution_stg_submitters.get()
        ).unwrap();
        c.register(concept, msg_sender()).unwrap();
        let votes = votes_1 + votes_2;
        let stg_amt_1 = use_votes!{
            {msg_sender() => votes},
            c.add_votes(concept, votes_1)
        };
        let stg_amt_2 = use_votes!{
            {msg_sender() => votes},
            c.add_votes(concept, votes_2)
        };
        // Let's see what happens if we take votes from our user, then add it back.
        c.take_votes(concept, votes_1).unwrap();
        c.take_votes(concept, votes_2).unwrap();
        // Give back our votes.
        use_votes!{
            {msg_sender() => votes},
            c.add_votes(concept, votes_1)
        };
        use_votes!{
            {msg_sender() => votes},
            c.add_votes(concept, votes_2)
        };
        let epoch = c.epochs.get(c.epochs.len() -1).unwrap();
        assert_eq!(stg_amt_1.root(2) + stg_amt_2.root(2), epoch.concept_quad_amts.get(concept));
        c.operator_addr.set(msg_sender());
        let (winning_concept, stg_to_gain) = c.choose_winners(3, vec![concept]).unwrap()[0];
        assert_eq!(concept, winning_concept);
        // There's only one outcome, so we should be able to collect the entire bag of STG.
        assert_eq!(c.dilution_stg_correct_concepts.get(), stg_to_gain);
        // Now the sender can claim the amount.
        c.pick_winners_that_accomplished(0, vec![concept]).unwrap();
        // Now the caller can claim the money!
        assert_eq!(
            stg_to_gain,
            c.draw_down_winner(0, concept, msg_sender()).unwrap()
        );
        // See if we can repeat the cycle.
        assert_eq!(1, c.bump_epoch().unwrap());
    }
}
