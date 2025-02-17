use crate::components::{AttackerStatus, DefenderStatus, PlayerStatus};
use crate::data::{Card, PlayerCondition};
use crate::types::{Block, Dexterity, Energy, Strength};

use super::combat_cards::CombatCards;
use super::player_persistent_state::PlayerPersistentState;

/// Captures the state of a combat encounter, including the player's hand, draw pile, etc.
/// Lives only as long as the combat encounter itself.
#[derive(Debug)]
pub struct PlayerCombatState<'a> {
    pub pps: &'a mut PlayerPersistentState,
    pub energy: Energy,
    pub block: Block,
    pub conditions: Vec<PlayerCondition>,
    pub cards: CombatCards,
    pub hp_loss_count: usize,
    pub strength: Strength,
    pub dexterity: Dexterity,
}

impl<'a> PlayerCombatState<'a> {
    pub fn new(pps: &'a mut PlayerPersistentState) -> Self {
        let cards = CombatCards::new(&pps.deck);
        Self {
            pps,
            energy: 3,
            block: 0,
            conditions: Vec::new(),
            cards,
            hp_loss_count: 0,
            strength: 0,
            dexterity: 0,
        }
    }
}

impl AttackerStatus for PlayerCombatState<'_> {
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

impl DefenderStatus for PlayerCombatState<'_> {
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

impl<'a> From<&PlayerCombatState<'a>> for PlayerStatus {
    fn from(pcs: &PlayerCombatState<'a>) -> Self {
        Self {
            hp: pcs.pps.hp,
            hp_max: pcs.pps.hp_max,
            gold: pcs.pps.gold,
            relics: pcs.pps.relics.clone(),
            deck: pcs.pps.deck.clone(),
            potions: pcs.pps.potions.clone(),
            energy: pcs.energy,
            block: pcs.block,
            conditions: pcs.conditions.clone(),
            hand: pcs.cards.hand.clone(),
            draw_pile: pcs.cards.sanitized_draw_pile(),
            discard_pile: pcs.cards.sanitized_discard_pile(),
            exhaust_pile: pcs.cards.sanitized_exhaust_pile(),
            hp_loss_count: pcs.hp_loss_count,
            strength: pcs.strength,
            dexterity: pcs.dexterity,
        }
    }
}
