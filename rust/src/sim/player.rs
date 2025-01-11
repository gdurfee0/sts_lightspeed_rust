use crate::data::{Card, Relic};

/// Encapsulates the state of the player in the game, e.g. HP, gold, deck, etc.
pub struct Player {
    pub hp: u32,
    pub hp_max: u32,
    pub gold: u32,
    pub relics: Vec<Relic>,
    pub deck: Vec<Card>,
}
