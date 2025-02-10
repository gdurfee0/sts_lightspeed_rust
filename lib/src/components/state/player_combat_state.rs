use crate::components::{AttackerStatus, DefenderStatus};
use crate::data::{Card, PlayerCondition};
use crate::types::{Block, Dexterity, Energy, Strength};

use super::CombatCards;

/// Captures the state of a combat encounter, including the player's hand, draw pile, etc.
/// Lives only as long as the combat encounter itself.
#[derive(Debug)]
pub struct PlayerCombatState {
    pub energy: Energy,
    pub block: Block,
    pub conditions: Vec<PlayerCondition>,
    pub cards: CombatCards,
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
            cards: CombatCards::new(deck),
            hp_loss_count: 0,
            strength: 0,
            dexterity: 0,
        }
    }
}

impl AttackerStatus for PlayerCombatState {
    fn block(&self) -> Block {
        self.block
    }

    fn draw_pile_size(&self) -> usize {
        self.cards.draw_pile.len()
    }

    fn hand_size(&self) -> usize {
        self.cards.hand.len()
    }

    fn is_weak(&self) -> bool {
        self.conditions
            .iter()
            .any(|c| matches!(c, PlayerCondition::Weak(_)))
    }

    fn number_of_strike_cards_owned(&self) -> usize {
        self.cards
            .iter()
            .filter(|c| {
                matches!(
                    c.card,
                    Card::Strike(_)
                        | Card::MeteorStrike(_)
                        | Card::PerfectedStrike(_)
                        | Card::PommelStrike(_)
                        | Card::SneakyStrike(_)
                        | Card::ThunderStrike(_)
                        | Card::TwinStrike(_)
                        | Card::WildStrike(_)
                        | Card::WindmillStrike(_)
                )
            })
            .count()
    }

    fn strength(&self) -> Strength {
        self.strength
    }
}

impl DefenderStatus for PlayerCombatState {
    fn dexterity(&self) -> Dexterity {
        self.dexterity
    }

    fn is_frail(&self) -> bool {
        self.conditions
            .iter()
            .any(|c| matches!(c, PlayerCondition::Frail(_)))
    }

    fn is_vulnerable(&self) -> bool {
        self.conditions
            .iter()
            .any(|c| matches!(c, PlayerCondition::Vulnerable(_)))
    }
}
