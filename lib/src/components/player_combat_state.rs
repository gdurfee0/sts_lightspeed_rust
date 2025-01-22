use crate::data::{Card, PlayerCondition};
use crate::types::{Block, Energy};

/// Captures the state of a combat encounter, including the player's hand, draw pile, etc.
/// Lives only as long as the combat encounter itself.  TODO: lock down field visibility
#[derive(Debug)]
pub struct PlayerCombatState {
    pub energy: Energy,
    pub block: Block,
    pub conditions: Vec<PlayerCondition>,
    pub hand: Vec<Card>,
    pub draw_pile: Vec<Card>,
    pub discard_pile: Vec<Card>,
    pub exhaust_pile: Vec<Card>,
}

impl PlayerCombatState {
    pub fn new(deck: &[Card]) -> Self {
        Self {
            energy: 3,
            block: 0,
            conditions: Vec::new(),
            hand: Vec::new(),
            draw_pile: deck.to_vec(),
            discard_pile: Vec::new(),
            exhaust_pile: Vec::new(),
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
}
