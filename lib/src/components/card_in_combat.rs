use crate::data::Card;
use crate::types::{DeckIndex, Energy};

#[derive(Clone, Copy, Debug)]
pub struct CardInCombat {
    pub deck_index: Option<DeckIndex>,
    pub card: Card,
    pub cost_this_combat: Energy,
    pub cost_this_turn: Energy,
}
