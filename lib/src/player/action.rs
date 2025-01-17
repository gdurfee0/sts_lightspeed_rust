use std::iter::repeat;

use crate::data::Card;
use crate::{
    AttackCount, AttackDamage, Block, Debuff, Effect, EnemyIndex, Energy, PotionIndex, StackCount,
};

#[derive(Clone, Debug)]
pub struct EffectChain(Vec<Effect>); // Effects as proscibed by a Card's text.

#[derive(Clone, Debug)]
pub struct PlayerEffectChain(Vec<Effect>); // Effects after player buffs/debuffs are applied.

#[derive(Clone, Debug)]
pub struct EnemyEffectChain(Vec<Effect>); // After enemy buffs/debuffs are applied to previous.

#[derive(Clone, Debug)]
pub enum Action {
    ApplyEffectChainToAllEnemies(Vec<Option<EnemyEffectChain>>),
    ApplyEffectChainToEnemy(EnemyEffectChain, EnemyIndex),
    ApplyEffectChainToPlayer(PlayerEffectChain),
    DiscardPotion(PotionIndex),
    DrinkPotion(PotionIndex),
    EndTurn,
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
    pub effect_chain: EffectChain,
    pub target: Target,
    pub exhaust: bool,
}

impl CardDetails {
    fn with_cost(cost: Energy) -> CardDetailsBuilder {
        CardDetailsBuilder {
            cost,
            effect_chain: EffectChain::default(),
        }
    }

    fn exhaust(mut self) -> CardDetails {
        self.exhaust = true;
        self
    }
}

impl Default for EffectChain {
    fn default() -> Self {
        EffectChain(Vec::with_capacity(1))
    }
}

impl EffectChain {
    pub fn iter(&self) -> impl Iterator<Item = &Effect> {
        self.0.iter()
    }
}

impl PlayerEffectChain {
    pub fn new(effects: Vec<Effect>) -> Self {
        PlayerEffectChain(effects)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Effect> {
        self.0.iter()
    }
}

impl EnemyEffectChain {
    pub fn new(effects: Vec<Effect>) -> Self {
        EnemyEffectChain(effects)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Effect> {
        self.0.iter()
    }
}

struct CardDetailsBuilder {
    cost: Energy,
    effect_chain: EffectChain,
}

impl CardDetailsBuilder {
    fn deal_damage(mut self, amount: AttackDamage, times: AttackCount) -> CardDetailsBuilder {
        self.effect_chain
            .0
            .extend(repeat(Effect::AttackDamage(amount)).take(times as usize));
        self
    }

    fn do_nothing(self) -> CardDetails {
        CardDetails {
            cost: self.cost,
            effect_chain: EffectChain::default(),
            target: Target::Player,
            exhaust: false,
        }
    }

    fn gain_block(mut self, amount: Block) -> CardDetails {
        self.effect_chain.0.push(Effect::GainBlock(amount));
        CardDetails {
            cost: self.cost,
            effect_chain: self.effect_chain,
            target: Target::Player,
            exhaust: false,
        }
    }

    fn inflict(mut self, debuff: Debuff, stacks: StackCount) -> Self {
        self.effect_chain.0.push(Effect::Inflict(debuff, stacks));
        self
    }

    fn to_all_enemies(self) -> CardDetails {
        CardDetails {
            cost: self.cost,
            effect_chain: self.effect_chain,
            target: Target::AllEnemies,
            exhaust: false,
        }
    }

    fn to_one_enemy(self) -> CardDetails {
        CardDetails {
            cost: self.cost,
            effect_chain: self.effect_chain,
            target: Target::OneEnemy,
            exhaust: false,
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
define_card!(SLIMED, CardDetails::with_cost(1).do_nothing().exhaust());
define_card!(
    STRIKE,
    CardDetails::with_cost(1).deal_damage(6, 1).to_one_enemy()
);

impl CardDetails {
    pub fn for_card(card: Card) -> &'static CardDetails {
        match card {
            Card::Bash => &BASH,
            Card::Defend => &DEFEND,
            Card::Slimed => &SLIMED,
            Card::Strike => &STRIKE,
            _ => todo!("{:?}", card),
        }
    }
}
