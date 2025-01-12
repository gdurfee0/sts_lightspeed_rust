use std::fmt;

use crate::data::{Card, NeowBlessing, Potion, Relic};

/// Message type for communication from the Simualtor to a client (human operator or AI agent).
/// The Simulator will send any number of these messages to the client, concluding with a
/// one of the question type messages (`Choices` and `NestedChoices`), at which point control
/// the Simulator waits for a response on the input channel.
#[derive(Debug)]
pub enum StsMessage {
    /// ASCII representation of the current map.
    Map(String),

    /// All of the player's relics in order of acquisition.
    Relics(Vec<Relic>),

    /// The player's card deck in order of acquisition.
    Deck(Vec<Card>),

    /// All information that might change on a move-by-move basis, such as the player's HP and gold.
    View(PlayerView),

    /// Indicates that the game is over. The boolean indicates whether the player won or lost.
    GameOver(bool),

    /// A list of `Choice`s, each representing a possible action; the client must select one
    /// using zero-indexing and return its response as `usize` via its input_tx channel.
    /// It is not expected that Choices not chosen will remain after the decision has been made.
    Choices(Prompt, Vec<Choice>),

    /// A list of `Choice`s, each representing a possible action; the client must select one
    /// at a time by communicating the appropriate `usize` on the input channel. The expectation
    /// is that the client will be able to make multiple choices from the same list consecutively,
    /// e.g. when being handed 3 potions by Neow.
    Rewards(Prompt, Vec<Choice>),
}

#[derive(Clone, Debug)]
pub struct PlayerView {
    pub hp: u32,
    pub hp_max: u32,
    pub gold: u32,
    // TODO: potions
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Prompt {
    AcceptRewards,
    ChooseCard,
    MoveTo,
    NeowBlessing,
}

#[derive(Clone, Debug)]
pub enum Reward {
    Card(Card),
    Potion(Potion),
    Relic(Relic),
}

#[derive(Clone, Debug)]
pub enum Choice {
    MoveTo(usize),
    NeowBlessing(NeowBlessing),
    ObtainCard(Card),
    ObtainPotion(Potion),
    Skip,
}

impl fmt::Display for Prompt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Prompt::AcceptRewards => write!(f, "Available rewards"),
            Prompt::ChooseCard => write!(f, "Choose a card to obtain"),
            Prompt::MoveTo => write!(f, "Move up into one of the following columns"),
            Prompt::NeowBlessing => write!(f, "Choose Neow's Blessing"),
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
            Choice::Skip => write!(f, "(Skip)"),
        }
    }
}
