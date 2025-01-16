use std::iter::repeat;

use crate::data::Card;
use crate::{
    AttackCount, AttackDamage, BlockAmount, Debuff, Effect, HandIndex, PotionIndex, StackCount,
};

#[derive(Clone, Debug)]
pub enum Action {
    DiscardPotion(PotionIndex),
    DrinkPotion(PotionIndex),
    EndTurn,
    PlayCard(&'static CardAction),
    PlayCardAgainstEnemy(&'static CardAction, HandIndex),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Target {
    AllEnemies,
    OneEnemy,
    Player,
}

#[derive(Debug)]
pub struct CardAction {
    pub effects: Vec<Effect>,
    pub target: Target,
}

impl CardAction {
    fn deal_damage(amount: AttackDamage, times: AttackCount) -> CardActionBuilder {
        CardActionBuilder {
            effects: repeat(Effect::DealDamage(amount))
                .take(times as usize)
                .collect(),
        }
    }

    fn gain_block(amount: BlockAmount) -> CardAction {
        CardAction {
            effects: vec![Effect::GainBlock(amount)],
            target: Target::Player,
        }
    }
}

struct CardActionBuilder {
    effects: Vec<Effect>,
}

impl CardActionBuilder {
    fn then_inflict(mut self, debuff: Debuff, stacks: StackCount) -> Self {
        self.effects.push(Effect::Inflict(debuff, stacks));
        self
    }

    fn to_all_enemies(self) -> CardAction {
        CardAction {
            effects: self.effects,
            target: Target::AllEnemies,
        }
    }

    fn to_one_enemy(self) -> CardAction {
        CardAction {
            effects: self.effects,
            target: Target::OneEnemy,
        }
    }
}

// Convenience macros
macro_rules! define_card {
    ($name:ident, $player_move:expr) => {
        static $name: once_cell::sync::Lazy<CardAction> =
            once_cell::sync::Lazy::new(|| $player_move);
    };
}

define_card!(
    BASH,
    CardAction::deal_damage(8, 1)
        .then_inflict(Debuff::Vulnerable, 2)
        .to_one_enemy()
);
define_card!(DEFEND, CardAction::gain_block(5));
define_card!(STRIKE, CardAction::deal_damage(6, 1).to_one_enemy());

impl CardAction {
    pub fn for_card(card: Card) -> &'static CardAction {
        match card {
            Card::Bash => &BASH,
            Card::Defend => &DEFEND,
            Card::Strike => &STRIKE,
            _ => todo!(),
        }
    }
}
