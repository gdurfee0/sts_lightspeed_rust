use crate::data::{Card, Enemy, EnergyCost, NeowBlessing, Potion};
use crate::types::{
    ColumnIndex, DeckIndex, DiscardIndex, EnemyIndex, Energy, Gold, HandIndex, PotionIndex,
};

use super::notification::Notification;

/// Message type for communication from the Simualtor to a client (human operator or AI agent).
/// The Simulator will send any `Notification`s to the client, concluding with a `Choices`
/// message, at which point control the Simulator waits for a response on the input channel.
#[derive(Debug)]
#[cfg_attr(test, derive(Eq, Hash, PartialEq))]
pub enum StsMessage {
    Notification(Notification),

    /// A list of `Choice`s, each representing a possible action; the client must select one
    /// using zero-indexing and return its response as `usize` via its input_tx channel.
    Choices(Prompt, Vec<Choice>),
    GameOver(bool),
}

#[derive(Clone, Debug)]
#[cfg_attr(test, derive(Eq, Hash, PartialEq))]
pub enum Prompt {
    ChooseCardToPutOnTopOfDrawPile,
    ChooseCombatReward,
    ChooseForEvent,
    ChooseNeow,
    ChooseNext, // Expectation is that the player may accept more than one of the Choices offered.
    ChooseOne,  // Expectation is that the player can pick at most one of the Choices offered.
    ChooseRestSiteAction,
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
    EventChoice(usize, String), // Free-form text for events
    ExpendPotion(PotionAction),
    ClimbFloor(ColumnIndex),
    NeowBlessing(NeowBlessing),
    ObtainCard(Card),
    ObtainGold(Gold),
    ObtainPotion(Potion),
    PutOnTopOfDrawPile(DiscardIndex, Card),
    RemoveCard(DeckIndex, Card),
    Rest,
    Upgrade,
    PlayCardFromHand(HandIndex, Card, EnergyCost),
    Skip,
    TargetEnemy(EnemyIndex, Enemy),
}

#[derive(Clone, Copy, Debug)]
#[cfg_attr(test, derive(Eq, Hash, PartialEq))]
pub enum PotionAction {
    Discard(PotionIndex, Potion),
    Drink(PotionIndex, Potion),
}
