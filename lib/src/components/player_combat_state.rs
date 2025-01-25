use crate::data::{Card, PlayerCondition};
use crate::types::{Block, Dexterity, Energy, Strength};

use super::card_in_combat::CardInCombat;

/// Captures the state of a combat encounter, including the player's hand, draw pile, etc.
/// Lives only as long as the combat encounter itself.  TODO: lock down field visibility
#[derive(Debug)]
pub struct PlayerCombatState {
    pub energy: Energy,
    pub block: Block,
    pub conditions: Vec<PlayerCondition>,
    pub hand: Vec<CardInCombat>,
    pub draw_pile: Vec<CardInCombat>,
    pub discard_pile: Vec<CardInCombat>,
    pub exhaust_pile: Vec<CardInCombat>,
    pub hp_loss_count: usize,
    pub strength: Strength,
    pub dexterity: Dexterity,
}

impl PlayerCombatState {
    pub fn new(deck: &[Card]) -> Self {
        Self {
            energy: 3,
            block: 0,
            conditions: Vec::new(),
            hand: Vec::new(),
            draw_pile: deck
                .iter()
                .copied()
                .enumerate()
                .map(|(i, card)| CardInCombat {
                    deck_index: Some(i),
                    card,
                    cost_this_combat: card.cost(),
                    cost_this_turn: card.cost(),
                })
                .collect(),
            discard_pile: Vec::new(),
            exhaust_pile: Vec::new(),
            hp_loss_count: 0,
            strength: 0,
            dexterity: 0,
        }
    }

    pub fn is_frail(&self) -> bool {
        self.conditions
            .iter()
            .any(|c| matches!(c, PlayerCondition::Frail(_)))
    }

    pub fn is_vulnerable(&self) -> bool {
        self.conditions
            .iter()
            .any(|c| matches!(c, PlayerCondition::Vulnerable(_)))
    }

    pub fn is_weak(&self) -> bool {
        self.conditions
            .iter()
            .any(|c| matches!(c, PlayerCondition::Weak(_)))
    }

    pub fn is_confused(&self) -> bool {
        self.conditions
            .iter()
            .any(|c| matches!(c, PlayerCondition::Confused()))
    }

    pub fn cards_iter_mut(&mut self) -> impl Iterator<Item = &mut CardInCombat> {
        self.hand
            .iter_mut()
            .chain(self.draw_pile.iter_mut())
            .chain(self.discard_pile.iter_mut())
            .chain(self.exhaust_pile.iter_mut())
    }
}
