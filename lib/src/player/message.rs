use std::fmt;

use crate::data::{Card, NeowBlessing, Potion, Relic};
use crate::enemy::{EnemyStatus, EnemyType};
use crate::{
    ColumnIndex, Debuff, DeckIndex, EnemyIndex, Energy, Gold, HandIndex, Health, PotionIndex,
    StackCount,
};

/// Message type for communication from the Simualtor to a client (human operator or AI agent).
/// The Simulator will send any number of these messages to the client, concluding with a
/// one of the question type messages (`Choices` and `NestedChoices`), at which point control
/// the Simulator waits for a response on the input channel.
#[derive(Debug)]
pub enum StsMessage {
    // State updates for the main game loop, outside of an encounter or event.
    Map(String),
    Deck(Vec<Card>),
    Potions(Vec<Option<Potion>>),
    Relics(Vec<Relic>),
    CardObtained(Card),
    CardRemoved(Card),
    PotionObtained(PotionIndex, Potion),
    RelicObtained(Relic),
    Gold(Gold),

    // Encounter / combat messages
    CardDrawn(HandIndex, Card),
    Debuffs(Vec<(Debuff, StackCount)>),
    DiscardPile(Vec<Card>),
    EnemyStatus(EnemyIndex, EnemyStatus),
    EnemyDied(EnemyIndex, EnemyType),
    HandDiscarded,
    Health(Health),
    ShufflingDiscardToDraw,
    CardDiscarded(HandIndex, Card),
    Energy(Energy),

    /// A list of `Choice`s, each representing a possible action; the client must select one
    /// using zero-indexing and return its response as `usize` via its input_tx channel.
    Choices(Prompt, Vec<Choice>),
    GameOver(bool),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Prompt {
    ChooseNeow,
    ChooseNext, // Expectation is that the player may accept any and all of the Choices offered.
    ChooseOne,  // Expectation is that the player can pick at most one of the Choices offered.
    CombatAction,
    ClimbFloor,
    RemoveCard,
    TargetEnemy,
}

#[derive(Clone, Copy, Debug)]
pub enum Choice {
    EndTurn,
    ClimbFloor(ColumnIndex),
    NeowBlessing(NeowBlessing),
    ObtainCard(Card),
    ObtainPotion(Potion),
    RemoveCard(DeckIndex, Card),
    PlayCardFromHand(HandIndex, Card),
    Skip,
    TargetEnemy(EnemyIndex, EnemyType),
}

// TODO: Move these to a presentation module
impl fmt::Display for Prompt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Prompt::ChooseNeow => write!(f, "Choose Neow's Blessing"),
            Prompt::ChooseNext => write!(f, "Choose the next item to obtain"),
            Prompt::ChooseOne => write!(f, "Choose an item to obtain"),
            Prompt::ClimbFloor => write!(f, "Move up into one of the following columns"),
            Prompt::CombatAction => write!(f, "It is your turn to act"),
            Prompt::RemoveCard => write!(f, "Choose a card to remove"),
            Prompt::TargetEnemy => write!(f, "Choose an enemy to target"),
        }
    }
}

impl fmt::Display for Choice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Choice::ClimbFloor(column_index) => {
                write!(f, "Column {}", (b'a' + *column_index as u8) as char)
            }
            Choice::EndTurn => write!(f, "(End Turn)"),
            Choice::NeowBlessing(blessing) => write!(f, "{}", blessing),
            Choice::ObtainCard(card) => write!(f, "{:?}", card),
            Choice::ObtainPotion(potion) => write!(f, "{:?}", potion),
            Choice::PlayCardFromHand(_, card) => write!(f, "Play card \"{:?}\"", card),
            Choice::RemoveCard(_, card) => write!(f, "{:?}", card),
            Choice::Skip => write!(f, "(Skip)"),
            Choice::TargetEnemy(_, enemy) => write!(f, "Target \"{:?}\"", enemy), // TODO: Index?
        }
    }
}
