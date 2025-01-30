use crate::data::{Card, CardDetails, EnergyCost};
use crate::types::DeckIndex;

#[derive(Clone, Copy, Debug)]
#[cfg_attr(test, derive(Eq, Hash, PartialEq))]
pub struct CardInCombat {
    pub deck_index: Option<DeckIndex>,
    pub card: Card,
    pub details: &'static CardDetails,
    pub cost_this_combat: EnergyCost,
    pub cost_this_turn: EnergyCost,
}

impl CardInCombat {
    pub fn new(deck_index: Option<DeckIndex>, card: Card) -> Self {
        let details = CardDetails::for_card(card);
        Self {
            deck_index,
            card,
            details,
            cost_this_combat: details.cost,
            cost_this_turn: details.cost,
        }
    }
}
