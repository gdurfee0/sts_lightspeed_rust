use crate::data::Card;
use crate::types::{DeckIndex, Energy};

#[derive(Clone, Copy, Debug)]
pub struct CardInCombat {
    pub deck_index: Option<DeckIndex>,
    pub card: Card,
    pub cost_this_combat: Energy,
}

impl CardInCombat {
    pub fn new(deck_index: Option<DeckIndex>, card: Card) -> Self {
        Self {
            deck_index,
            card,
            cost_this_combat: card.cost(),
        }
    }
}
