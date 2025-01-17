use std::iter::{once, repeat};

use crate::data::Card;
use crate::{
    AttackCount, AttackDamage, Block, Debuff, Effect, Energy, HandIndex, PotionIndex, StackCount,
};

#[derive(Clone, Debug)]
pub enum Action {
    DiscardPotion(PotionIndex),
    DrinkPotion(PotionIndex),
    EndTurn,
    PlayCard(&'static CardDetails),
    PlayCardAgainstEnemy(&'static CardDetails, HandIndex),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Target {
    AllEnemies,
    OneEnemy,
    Player,
}

#[derive(Debug)]
pub struct CardDetails {
    pub cost: Energy,
    pub effects: Vec<Effect>,
    pub target: Target,
}

impl CardDetails {
    pub fn with_cost(cost: Energy) -> CardDetailsBuilder {
        CardDetailsBuilder {
            cost,
            effects: vec![],
        }
    }
}

struct CardDetailsBuilder {
    cost: Energy,
    effects: Vec<Effect>,
}

impl CardDetailsBuilder {
    fn deal_damage(mut self, amount: AttackDamage, times: AttackCount) -> CardDetailsBuilder {
        self.effects
            .extend(repeat(Effect::AttackDamage(amount)).take(times as usize));
        self
    }

    fn gain_block(mut self, amount: Block) -> CardDetails {
        self.effects.push(Effect::GainBlock(amount));
        CardDetails {
            cost: self.cost,
            effects: self.effects,
            target: Target::Player,
        }
    }

    fn inflict(mut self, debuff: Debuff, stacks: StackCount) -> Self {
        self.effects.push(Effect::Inflict(debuff, stacks));
        self
    }

    fn to_all_enemies(self) -> CardDetails {
        CardDetails {
            cost: self.cost,
            effects: self.effects,
            target: Target::AllEnemies,
        }
    }

    fn to_one_enemy(self) -> CardDetails {
        CardDetails {
            cost: self.cost,
            effects: self.effects,
            target: Target::OneEnemy,
        }
    }
}

// Convenience macros
macro_rules! define_card {
    ($name:ident, $details:expr) => {
        static $name: once_cell::sync::Lazy<CardDetails> = once_cell::sync::Lazy::new(|| $details);
    };
}

define_card!(
    BASH,
    CardDetails::with_cost(2)
        .deal_damage(8, 1)
        .inflict(Debuff::Vulnerable, 2)
        .to_one_enemy()
);
define_card!(DEFEND, CardDetails::with_cost(1).gain_block(5));
define_card!(
    STRIKE,
    CardDetails::with_cost(1).deal_damage(6, 1).to_one_enemy()
);

impl CardDetails {
    pub fn for_card(card: Card) -> &'static CardDetails {
        match card {
            Card::Bash => &BASH,
            Card::Defend => &DEFEND,
            Card::Strike => &STRIKE,
            _ => todo!(),
        }
    }
}
