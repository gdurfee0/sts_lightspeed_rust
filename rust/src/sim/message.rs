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
    PotionObtained(Potion, u32),
    RelicObtained(Relic),
    GoldChanged(u32),

    // Encounter / combat messages
    EnemyParty(Vec<(EnemyType, u32, u32, Intent)>),
    HealthChanged(u32, u32), // "Health" always refers to the pair (current HP, max HP)
    DebuffsChanged(Vec<(Debuff, u32)>),
    DiscardPile(Vec<Card>),

    GameOver(bool),

    /// A list of `Choice`s, each representing a possible action; the client must select one
    /// using zero-indexing and return its response as `usize` via its input_tx channel.
    Choices(Prompt, Vec<Choice>),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Prompt {
    ChooseNeow,
    ChooseNext, // Expectation is that the player may accept any and all of the Choices offered.
    ChooseOne,  // Expectation is that the player can pick at most one of the Choices offered.
    MoveTo,
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
            Prompt::ChooseNeow => write!(f, "Choose Neow's Blessing"),
            Prompt::ChooseNext => write!(f, "Choose the next item to obtain"),
            Prompt::ChooseOne => write!(f, "Choose an item to obtain"),
            Prompt::MoveTo => write!(f, "Move up into one of the following columns"),
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
