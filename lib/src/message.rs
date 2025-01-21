use crate::data::{Card, Enemy, NeowBlessing, PlayerCondition, Potion, Relic};
use crate::enemy::EnemyStatus;
use crate::types::{
    Block, ColumnIndex, DeckIndex, EnemyIndex, Energy, Gold, HandIndex, Health, Hp, PotionIndex,
};

/// Message type for communication from the Simualtor to a client (human operator or AI agent).
/// The Simulator will send any number of these messages to the client, concluding with a
/// one of the question type messages (`Choices` and `NestedChoices`), at which point control
/// the Simulator waits for a response on the input channel.
#[derive(Debug)]
#[cfg_attr(test, derive(Eq, Hash, PartialEq))]
pub enum StsMessage {
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
    CardDrawn(HandIndex, Card),
    CardExhausted(HandIndex, Card),
    Conditions(Vec<PlayerCondition>),
    DamageBlocked(Hp),
    DamageTaken(Hp),
    DiscardPile(Vec<Card>),
    EnemyStatus(EnemyIndex, EnemyStatus),
    EnemyDied(EnemyIndex, Enemy),
    EnemyParty(Vec<Option<EnemyStatus>>),
    Energy(Energy),
    HandDiscarded,
    Health(Health),
    ShufflingDiscardToDraw,

    /// A list of `Choice`s, each representing a possible action; the client must select one
    /// using zero-indexing and return its response as `usize` via its input_tx channel.
    Choices(Prompt, Vec<Choice>),
    GameOver(bool),
}

#[derive(Clone, Debug)]
#[cfg_attr(test, derive(Eq, Hash, PartialEq))]
pub enum Prompt {
    ChooseNeow,
    ChooseNext, // Expectation is that the player may accept any and all of the Choices offered.
    ChooseOne,  // Expectation is that the player can pick at most one of the Choices offered.
    CombatAction,
    ClimbFloor,
    ClimbFloorHasPotion,
    RemoveCard,
    TargetEnemy,
}

#[derive(Clone, Debug)]
#[cfg_attr(test, derive(Eq, Hash, PartialEq))]
pub enum Choice {
    EndTurn,
    PotionAction(PotionAction),
    ClimbFloor(ColumnIndex),
    NeowBlessing(NeowBlessing),
    ObtainCard(Card),
    ObtainGold(Gold),
    ObtainPotion(Potion),
    RemoveCard(DeckIndex, Card),
    PlayCardFromHand(HandIndex, Card),
    Skip,
    TargetEnemy(EnemyIndex, Enemy),
}

#[derive(Clone, Copy, Debug)]
#[cfg_attr(test, derive(Eq, Hash, PartialEq))]
pub enum PotionAction {
    Discard(PotionIndex, Potion),
    Drink(PotionIndex, Potion),
}
