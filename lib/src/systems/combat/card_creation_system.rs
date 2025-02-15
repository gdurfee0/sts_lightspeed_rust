use anyhow::Error;

use crate::components::{
    CardCombatState, EffectQueue, Interaction, PlayerCombatState, PlayerPersistentState,
};
use crate::data::{Card, CardDestination, CardPool, CardSelection, CostModifier};

pub struct CardCreationSystem;

impl CardCreationSystem {
    pub fn create_card<I: Interaction>(
        comms: &I,
        pps: &mut PlayerPersistentState,
        pcs: &mut PlayerCombatState,
        card_args: (&CardPool, &CardSelection, &CardDestination, &CostModifier),
        effect_queue: &mut EffectQueue,
    ) -> Result<(), Error> {
        let (card_pool, card_selection, card_destination, cost_modifier) = card_args;
        let card_pool_vec: Vec<Card> = Self::get_card_pool_vec(card_pool);
        let card_selection_vec: Vec<Card> =
            Self::get_card_selection_vec(card_selection, card_pool_vec);
        let modified_cards = Self::modify_costs(cost_modifier, card_selection_vec);
        Self::add_cards_to_destination(
            comms,
            pps,
            pcs,
            modified_cards,
            card_destination,
            effect_queue,
        )
    }

    fn get_card_pool_vec(card_pool: &CardPool) -> Vec<Card> {
        match card_pool {
            CardPool::AttacksAndPowersInHand => todo!(),
            CardPool::CardInPlay => todo!(),
            CardPool::CharacterAttackPool => todo!(),
            CardPool::CharacterCardPool => todo!(),
            CardPool::CharacterPowerPool => todo!(),
            CardPool::CharacterSkillPool => todo!(),
            CardPool::ColorlessCardPool => todo!(),
            CardPool::Fixed(cards) => cards.to_vec(),
            CardPool::UpgradedColorlessCardPool => todo!(),
        }
    }

    fn get_card_selection_vec(
        card_selection: &CardSelection,
        card_pool_vec: Vec<Card>,
    ) -> Vec<Card> {
        match card_selection {
            CardSelection::All => card_pool_vec,
            CardSelection::PlayerChoice(_) => todo!(),
            CardSelection::PlayerChoiceUnlimited => todo!(),
            CardSelection::PlayerChoiceUpTo(_) => todo!(),
            CardSelection::Random(_) => todo!(),
            CardSelection::RandomThenPlayerChoice(_, _) => todo!(),
            CardSelection::RandomX => todo!(),
        }
    }

    fn modify_costs(
        cost_modifier: &CostModifier,
        card_selection_vec: Vec<Card>,
    ) -> Vec<CardCombatState> {
        card_selection_vec
            .iter()
            .map(|card| match cost_modifier {
                CostModifier::None => CardCombatState::new(*card, None),
                CostModifier::ZeroThisCombat => todo!(),
                CostModifier::ZeroThisTurn => todo!(),
                CostModifier::ZeroUntilPlayed => todo!(),
            })
            .collect()
    }

    fn add_cards_to_destination<I: Interaction>(
        comms: &I,
        pps: &mut PlayerPersistentState,
        pcs: &mut PlayerCombatState,
        modified_cards: Vec<CardCombatState>,
        card_destination: &CardDestination,
        effect_queue: &mut EffectQueue,
    ) -> Result<(), Error> {
        for card in modified_cards {
            match card_destination {
                CardDestination::BottomOfDrawPile => todo!(),
                CardDestination::DiscardPile => {
                    pcs.cards.discard_pile.push(card);
                }
                CardDestination::ExhaustPile => todo!(),
                CardDestination::Hand => todo!(),
                CardDestination::ShuffledIntoDrawPile => todo!(),
                CardDestination::TopOfDrawPile => todo!(),
                CardDestination::TwoCopiesInHand => todo!(),
            }
        }
        Ok(())
    }
}
