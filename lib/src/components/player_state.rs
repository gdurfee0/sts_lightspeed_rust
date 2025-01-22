use crate::data::{Card, Character, Potion, Relic};
use crate::types::{Gold, Health, Hp, HpMax};

/// Encapsulates the state of the player in the game, e.g. HP, gold, deck, etc.
/// Mostly a dumb container, just does some clamping on attributes.
#[derive(Debug)]
pub struct PlayerState {
    // State that persists outside of combat
    pub hp: Hp,
    pub hp_max: HpMax,
    pub gold: Gold,
    pub relics: Vec<Relic>,
    pub deck: Vec<Card>,
    pub potions: Vec<Option<Potion>>,
}

impl PlayerState {
    pub fn new(character: &'static Character) -> Self {
        let relics = vec![character.starting_relic];
        let deck = character.starting_deck.to_vec();
        let potions = [None; 3].to_vec();
        Self {
            hp: character.starting_hp,
            hp_max: character.starting_hp,
            gold: 99,
            relics,
            deck,
            potions,
        }
    }

    pub fn has_potion_slot_available(&self) -> bool {
        self.potions.iter().any(|potion| potion.is_none())
    }

    pub fn has_relic(&self, relic: Relic) -> bool {
        self.relics.contains(&relic)
    }

    pub fn health(&self) -> Health {
        (self.hp, self.hp_max)
    }
}
