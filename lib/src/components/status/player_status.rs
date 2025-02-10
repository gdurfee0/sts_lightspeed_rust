use crate::components::{AttackerStatus, CardCombatState, DefenderStatus};
use crate::data::{Card, PlayerCondition, Potion, Relic};
use crate::types::{Block, Dexterity, Energy, Gold, Hp, HpMax, Strength};

/// `PlayerStatus` is the information about the player that is made available to the client.
/// Some information is sanitized, e.g. the specific order of the cards in the draw pile, etc.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct PlayerStatus {
    pub hp: Hp,
    pub hp_max: HpMax,
    pub gold: Gold,
    pub relics: Vec<Relic>,
    pub deck: Vec<Card>,
    pub potions: Vec<Option<Potion>>,
    pub energy: Energy,
    pub block: Block,
    pub conditions: Vec<PlayerCondition>,
    pub hand: Vec<CardCombatState>,
    pub draw_pile: Vec<CardCombatState>,
    pub discard_pile: Vec<CardCombatState>,
    pub exhaust_pile: Vec<CardCombatState>,
    pub hp_loss_count: usize,
    pub strength: Strength,
    pub dexterity: Dexterity,
}

impl PlayerStatus {
    pub fn cards_iter(&self) -> impl Iterator<Item = &CardCombatState> {
        self.hand
            .iter()
            .chain(self.draw_pile.iter())
            .chain(self.discard_pile.iter())
            .chain(self.exhaust_pile.iter())
    }
}

impl AttackerStatus for PlayerStatus {
    fn block(&self) -> Block {
        self.block
    }

    fn draw_pile_size(&self) -> usize {
        self.draw_pile.len()
    }

    fn hand_size(&self) -> usize {
        self.hand.len()
    }

    fn is_weak(&self) -> bool {
        self.conditions
            .iter()
            .any(|c| matches!(c, PlayerCondition::Weak(_)))
    }

    fn number_of_strike_cards_owned(&self) -> usize {
        self.cards_iter()
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

impl DefenderStatus for PlayerStatus {
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
