use std::fmt;

use crate::data::NeowBlessing;

/// Message type for communication from the Simualtor to a client (human operator or AI agent).
/// The Simulator will send any number of these messages to the client, concluding with a
/// `Choose` message, at which point control passes to the client and the Simulator waits
/// for a response.
#[derive(Debug)]
pub enum StsMessage {
    /// ASCII representation of the current map.
    Map(String),

    // TODO: less frequently changing information such as deck composition, relics, etc.
    //
    // All information that might change on a move-by-move basis, such as the player's HP and gold.
    View(PlayerView),

    /// A list of `Choice`s, each representing a possible action; the client must select one
    /// using zero-indexing and return its response as `usize` via its input_tx channel.
    Choose(Prompt, Vec<Choice>),

    /// Indicates that the game is over. The boolean indicates whether the player won or lost.
    GameOver(bool),
}

#[derive(Clone, Debug)]
pub struct PlayerView {
    // TODO: keys
    // TODO: character? or expect client to remember?
    pub hp: u32,
    pub hp_max: u32,
    pub gold: u32,
    // TODO: potions
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Prompt {
    HaltAndCatchFire,
    NeowBlessing,
}

#[derive(Clone, Debug)]
pub enum Choice {
    CatchFire,
    NeowBlessing(NeowBlessing),
}

impl fmt::Display for Prompt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Prompt::HaltAndCatchFire => write!(f, "You halt. Now decide your fate"),
            Prompt::NeowBlessing => write!(f, "Choose Neow's Blessing"),
        }
    }
}

impl fmt::Display for Choice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Choice::CatchFire => write!(f, "Catch Fire"),
            Choice::NeowBlessing(blessing) => write!(f, "{}", blessing),
        }
    }
}
