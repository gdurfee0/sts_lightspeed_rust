use crate::components::{EnemyStatus, PlayerStatus};
use crate::data::{Card, Enemy, EnergyCost, PlayerCondition, Potion, Relic};
use crate::types::{
    Block, Dexterity, EnemyIndex, Energy, Gold, HandIndex, Health, Hp, PotionIndex, Strength,
};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Notification {
    // State updates for the main game loop, outside of an encounter or event.
    CardObtained(Card),
    CardRemoved(Card),
    CardUpgraded(Card, Card),
    Deck(Vec<Card>),
    Gold(Gold),
    Map(String),
    RelicObtained(Relic),
    Relics(Vec<Relic>),

    PotionObtained(PotionIndex, Potion),
    Potions(Vec<Option<Potion>>),

    // Encounter / combat messages
    StartingCombat,
    EndingCombat,

    AddToDiscardPile(Vec<Card>),
    Block(Block),
    BlockGained(Block),
    CardDiscarded(HandIndex, Card),
    CardDrawn(HandIndex, Card, EnergyCost),
    CardExhausted(HandIndex, Card),
    Conditions(Vec<PlayerCondition>),
    DamageBlocked(Hp),
    DamageTaken(Hp),
    Dexterity(Dexterity),
    DiscardPile(Vec<Card>),
    EnemyStatus(EnemyIndex, EnemyStatus),
    EnemyDied(EnemyIndex, Enemy),
    EnemyParty(Vec<Option<EnemyStatus>>),
    Energy(Energy),
    Health(Health),
    Hp(Hp),
    ShufflingDiscardPileIntoDrawPile,
    Status(PlayerStatus),
    Strength(Strength),
}
