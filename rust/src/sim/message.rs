use std::fmt;

use crate::data::{Card, NeowBlessing, Potion, Relic};

/// Message type for communication from the Simualtor to a client (human operator or AI agent).
/// The Simulator will send any number of these messages to the client, concluding with a
/// one of the question type messages (`Choices` and `NestedChoices`), at which point control
/// the Simulator waits for a response on the input channel.
#[derive(Debug)]
pub enum StsMessage {
    Map(String),
    Deck(Vec<Card>),
    Potions(Vec<Option<Potion>>),
    Relics(Vec<Relic>),
    CardObtained(Card),
    CardRemoved(Card, usize),
    PotionObtained(Potion, usize),
    RelicObtained(Relic),
    GoldChanged(u32),
    HpChanged(u32, u32),
    GameOver(bool),

    /// A list of `Choice`s, each representing a possible action; the client must select one
    /// using zero-indexing and return its response as `usize` via its input_tx channel.
    Choices(Prompt, Vec<Choice>),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Prompt {
    ChooseNext, // Expectation is that the player may accept and and all of the Choices offered.
    ChooseOne,  // Expectation is that the player can pick at most one of the Choices offered.
    MoveTo,
    NeowBlessing,
    RemoveCard,
}

#[derive(Clone, Debug)]
pub enum Choice {
    MoveTo(usize),
    NeowBlessing(NeowBlessing),
    ObtainCard(Card),
    ObtainPotion(Potion),
    RemoveCard(Card),
    Skip,
}

impl fmt::Display for Prompt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Prompt::ChooseNext => write!(f, "Choose the next item to obtain"),
            Prompt::ChooseOne => write!(f, "Choose an item to obtain"),
            Prompt::MoveTo => write!(f, "Move up into one of the following columns"),
            Prompt::NeowBlessing => write!(f, "Choose Neow's Blessing"),
            Prompt::RemoveCard => write!(f, "Choose a card to remove"),
        }
    }
}

impl fmt::Display for Choice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Choice::MoveTo(col) => write!(f, "Column {}", (b'a' + *col as u8) as char),
            Choice::NeowBlessing(blessing) => write!(f, "{}", blessing),
            Choice::ObtainCard(card) => write!(f, "{}", card),
            Choice::ObtainPotion(potion) => write!(f, "{}", potion),
            Choice::RemoveCard(card) => write!(f, "{}", card),
            Choice::Skip => write!(f, "(Skip)"),
        }
    }
}
