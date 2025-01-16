use crate::{AttackDamage, BlockAmount, StackCount};

use super::{Card, Debuff};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Effect {
    AddToDiscardPile(&'static [Card]),
    DealDamage(AttackDamage),
    GainBlock(BlockAmount),
    Inflict(Debuff, StackCount),
}
