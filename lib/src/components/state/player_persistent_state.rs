use crate::data::{Card, Character, Potion, Relic};
use crate::types::{Gold, Hp, HpMax};

/// Encapsulates the state of the player in the game, e.g. HP, gold, deck, etc., which persists
/// between combat encounters.
#[derive(Debug)]
pub struct PlayerPersistentState {
    pub hp: Hp,
    pub hp_max: HpMax,
    pub gold: Gold,
    pub relics: Vec<Relic>,
    pub deck: Vec<Card>,
    pub potions: Vec<Option<Potion>>,
}

impl PlayerPersistentState {
    pub fn new(character: &'static Character) -> Self {
        let relics = vec![character.starting_relic];
        let deck = character.starting_deck.to_vec();
        Self {
            hp: character.starting_hp,
            hp_max: character.starting_hp,
            gold: 99,
            relics,
            deck,
            potions: vec![None; 3],
        }
    }
}
