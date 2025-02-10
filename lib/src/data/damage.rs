use crate::types::{Hp, Strength};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Damage {
    Blockable(Hp),
    BlockableCountingStrikeCards(Hp, Hp),
    BlockableEqualToDrawPileSize,
    BlockableEqualToPlayerBlock,
    BlockableNonAttack(Hp),
    BlockableWithStrengthMultiplier(Hp, Strength),
    HpLoss(Hp),
    HpLossEqualToHandSize,
}
