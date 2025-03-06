use anyhow::Error;

use crate::components::{CardCombatState, Interaction, Notification};
use crate::data::{Card, CardDestination, CardPool, CardSelection, Character, CostModifier};
use crate::systems::base::CombatContext;

pub struct CardCreationSystem;

impl CardCreationSystem {
    /// Creates a card and adds it to the destination, applying modifiers to the cost if necessary.
    pub fn create_card<I: Interaction>(
        ctx: &mut CombatContext<I>,
        card_pool: &CardPool,
        card_selection: &CardSelection,
        card_destination: &CardDestination,
        cost_modifier: &CostModifier,
    ) -> Result<(), Error> {
        let card_pool = Self::get_card_pool(card_pool, ctx.pcs.pps.character);
        let card_selection = Self::get_card_selection(card_selection, card_pool);
        let modified_cards = Self::modify_costs(cost_modifier, card_selection);
        Self::add_cards_to_destination(ctx, modified_cards, card_destination)
    }

    /// Gets the card pool for the given character.
    fn get_card_pool(card_pool: &CardPool, character: &Character) -> &'static [Card] {
        match card_pool {
            CardPool::AttacksAndPowersInHand => todo!(),
            CardPool::CardInPlay => todo!(),
            CardPool::CharacterAttackPool => character.attack_card_pool,
            CardPool::CharacterCardPool => todo!(),
            CardPool::CharacterPowerPool => todo!(),
            CardPool::CharacterSkillPool => todo!(),
            CardPool::ColorlessCardPool => todo!(),
            CardPool::Fixed(cards) => cards,
            CardPool::UpgradedColorlessCardPool => todo!(),
        }
    }

    /// Selects the cards to be created from the card pool.
    fn get_card_selection(card_selection: &CardSelection, card_pool: &'static [Card]) -> Vec<Card> {
        match card_selection {
            CardSelection::All => card_pool.to_vec(),
            CardSelection::PlayerChoice(_) => todo!(),
            CardSelection::PlayerChoiceUnlimited => todo!(),
            CardSelection::PlayerChoiceUpTo(_) => todo!(),
            CardSelection::Random(num_cards) => Self::get_random_cards(card_pool, *num_cards),
            CardSelection::RandomThenPlayerChoice(_, _) => todo!(),
            CardSelection::RandomX => todo!(),
        }
    }

    /// Gets a random number of cards from the card pool.
    fn get_random_cards(_card_pool_vec: &'static [Card], _num_cards: usize) -> Vec<Card> {
        //card_pool_vec.to_vec()
        todo!()
    }

    /// Modifies the costs of the cards to be created.
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

    /// Adds the cards to the destination.
    fn add_cards_to_destination<I: Interaction>(
        ctx: &mut CombatContext<I>,
        created_cards: Vec<CardCombatState>,
        card_destination: &CardDestination,
    ) -> Result<(), Error> {
        for combat_card in created_cards {
            match card_destination {
                CardDestination::BottomOfDrawPile => todo!(),
                CardDestination::DiscardPile => {
                    ctx.pcs.cards.discard_pile.push(combat_card);
                }
                CardDestination::ExhaustPile => todo!(),
                CardDestination::Hand => todo!(),
                CardDestination::ShuffledIntoDrawPile => todo!(),
                CardDestination::TopOfDrawPile => todo!(),
                CardDestination::TwoCopiesInHand => todo!(),
            }
            ctx.comms
                .send_notification(Notification::CardCreated(combat_card, *card_destination))?;
        }
        Ok(())
    }
}
