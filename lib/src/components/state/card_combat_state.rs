use crate::data::{Card, CardDetails, EnergyCost};
use crate::types::{DeckIndex, Hp};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CardCombatState {
    pub card: Card,
    pub deck_index: Option<DeckIndex>,
    pub details: &'static CardDetails,
    pub cost_this_combat: EnergyCost,
    pub cost_this_turn: EnergyCost,
    pub cost_until_played: EnergyCost,
    pub additional_damage: Hp,
}

impl CardCombatState {
    pub fn new(card: Card, deck_index: Option<DeckIndex>) -> Self {
        let details = CardDetails::for_card(card);
        Self {
            card,
            deck_index,
            details,
            cost_this_combat: details.cost,
            cost_this_turn: details.cost,
            cost_until_played: details.cost,
            additional_damage: 0,
        }
    }
}
