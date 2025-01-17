use crate::{AttackDamage, Block, StackCount};

use super::{Card, Debuff};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Effect {
    AddToDiscardPile(&'static [Card]),
    AttackDamage(AttackDamage),
    GainBlock(Block),
    Inflict(Debuff, StackCount),
}
