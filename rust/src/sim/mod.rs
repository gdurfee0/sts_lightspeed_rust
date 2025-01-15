mod combat;
mod encounter;
mod enemy;
mod map;
mod message;
mod neow;
mod player;
mod simulator;

pub use message::{Choice, Prompt, StsMessage};
pub use simulator::StsSimulator;

pub type HandIndex = usize;
pub type EnemyIndex = usize;
pub type ColumnIndex = u8;
pub type PotionIndex = u8;
pub type StackCount = u32;
pub type Hp = u32;
pub type HpMax = u32;
pub type Gold = u32;
pub type Energy = u32;
pub type BlockAmount = u32;

use crate::data::Card;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Debuff {
    Frail,      // Block gained from cards is reduced by 25%.
    Vulnerable, // Target takes 50% more damage from attacks.
    Weak,       // Target deals 25% less attack damage.
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Effect {
    AddToDiscardPile(&'static [Card]),
    DealDamage(u32),
    GainBlock(u32),
    Inflict(Debuff, u32),
}
