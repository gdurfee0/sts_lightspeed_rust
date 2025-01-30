use crate::data::{Card, Enemy, EnergyCost, PlayerCondition, Potion, Relic};
use crate::types::{
    Block, Dexterity, EnemyIndex, Energy, Gold, HandIndex, Health, Hp, PotionIndex, Strength,
};

use super::enemy_status::EnemyStatus;

/// Message type for communication from the Simualtor to a client (human operator or AI agent).
/// The Simulator will send any number of these messages to the client, concluding with a
/// one of the question type messages (`Choices` and `NestedChoices`), at which point control
/// the Simulator waits for a response on the input channel.
#[derive(Clone, Debug)]
#[cfg_attr(test, derive(Eq, Hash, PartialEq))]
pub enum Notification {
    // State updates for the main game loop, outside of an encounter or event.
    CardObtained(Card),
    CardRemoved(Card),
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
    BlockLost(Block),
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
    HandDiscarded,
    Health(Health),
    ShufflingDiscardToDraw,
    Strength(Strength),
}
