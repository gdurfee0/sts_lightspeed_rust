use crate::data::{Card, Enemy, EnergyCost, NeowBlessing, Potion};
use crate::types::{
    CardRewardIndex, ColumnIndex, DeckIndex, DiscardIndex, EnemyIndex, Gold, HandIndex, PotionIndex,
};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Choice {
    EndTurn,
    EventChoice(usize, String), // Free-form text for events
    ExpendPotion(PotionAction),
    ClimbFloor(ColumnIndex),
    NeowBlessing(NeowBlessing),
    ObtainCard(CardRewardIndex, Card),
    ObtainGold(Gold),
    ObtainPotion(Potion),
    PlayCardFromHand(HandIndex, Card, EnergyCost),
    PutOnTopOfDrawPile(DiscardIndex, Card),
    RemoveCard(DeckIndex, Card),
    Rest,
    Skip,
    TargetEnemy(EnemyIndex, Enemy),
    UpgradeCard(DeckIndex, Card, Card),
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum PotionAction {
    Discard(PotionIndex, Potion),
    Drink(PotionIndex, Potion),
}
