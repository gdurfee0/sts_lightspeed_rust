use std::fmt;

use crate::data::{Card, EnemyType, Intent, NeowBlessing, Potion, Relic};

use super::action::Debuff;

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
    CardRemoved(Card, u32),
    PotionObtained(Potion, u8),
    RelicObtained(Relic),
    GoldChanged(u32),

    // Encounter / combat messages
    CardDrawn(Card, u8),
    DebuffsChanged(Vec<(Debuff, u32)>),
    DiscardPile(Vec<Card>),
    EnemyParty(Vec<(EnemyType, Intent, (u32, u32))>),
    HandDiscarded,
    HealthChanged(u32, u32), // "Health" always refers to the pair (current HP, max HP)
    ShufflingDiscardToDraw,

    /// A list of `Choice`s, each representing a possible action; the client must select one
    /// using zero-indexing and return its response as `usize` via its input_tx channel.
    Choices(Prompt, Vec<Choice>),
    GameOver(bool),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Prompt {
    ChooseNeow,
    ChooseNext, // Expectation is that the player may accept any and all of the Choices offered.
    ChooseOne,  // Expectation is that the player can pick at most one of the Choices offered.
    MoveTo,
    CombatAction,
    RemoveCard,
}

#[derive(Clone, Debug)]
pub enum Choice {
    EndTurn,
    MoveTo(u8),
    NeowBlessing(NeowBlessing),
    ObtainCard(Card),
    ObtainPotion(Potion),
    RemoveCard(Card),
    PlayCard(Card),
    PlayCardAgainstEnemy(Card, u8),
    Skip,
}

impl fmt::Display for Prompt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Prompt::ChooseNeow => write!(f, "Choose Neow's Blessing"),
            Prompt::ChooseNext => write!(f, "Choose the next item to obtain"),
            Prompt::ChooseOne => write!(f, "Choose an item to obtain"),
            Prompt::CombatAction => write!(f, "It is your turn to act"),
            Prompt::MoveTo => write!(f, "Move up into one of the following columns"),
            Prompt::RemoveCard => write!(f, "Choose a card to remove"),
        }
    }
}

impl fmt::Display for Choice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Choice::EndTurn => write!(f, "(End Turn)"),
            Choice::MoveTo(col) => write!(f, "Column {}", (b'a' + *col) as char),
            Choice::NeowBlessing(blessing) => write!(f, "{}", blessing),
            Choice::ObtainCard(card) => write!(f, "{}", card),
            Choice::ObtainPotion(potion) => write!(f, "{}", potion),
            Choice::PlayCard(card) => write!(f, "{}", card),
            Choice::PlayCardAgainstEnemy(card, idx) => write!(f, "{} against enemy {}", card, idx),
            Choice::RemoveCard(card) => write!(f, "{}", card),
            Choice::Skip => write!(f, "(Skip)"),
        }
    }
}
