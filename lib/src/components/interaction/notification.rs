use crate::components::{CardCombatState, EnemyStatus, PlayerStatus};
use crate::data::{Card, Enemy, PlayerCondition, Potion, Relic};
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

    AddToDiscardPile(Vec<CardCombatState>),
    Block(Block),
    BlockGained(Block),
    CardDiscarded(HandIndex, CardCombatState),
    CardDrawn(HandIndex, CardCombatState),
    CardExhausted(HandIndex, CardCombatState),
    Conditions(Vec<PlayerCondition>),
    DamageBlocked(Hp),
    DamageTaken(Hp),
    Dexterity(Dexterity),
    DiscardPile(Vec<CardCombatState>),
    EnemyDied(EnemyIndex, Enemy),
    EnemyParty(Vec<Option<EnemyStatus>>),
    Energy(Energy),
    Health(Health),
    ShufflingDiscardPileIntoDrawPile,
    Status(PlayerStatus),
    Strength(Strength),
}
