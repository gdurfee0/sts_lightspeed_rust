use crate::data::Card;

use super::card_combat_state::CardCombatState;

#[derive(Debug)]
pub struct CombatCards {
    pub card_in_play: Option<CardCombatState>,
    pub hand: Vec<CardCombatState>,
    pub draw_pile: Vec<CardCombatState>,
    pub discard_pile: Vec<CardCombatState>,
    pub exhaust_pile: Vec<CardCombatState>,
}

impl CombatCards {
    pub fn new(deck: &[Card]) -> Self {
        Self {
            card_in_play: None,
            hand: Vec::with_capacity(10),
            draw_pile: deck
                .iter()
                .copied()
                .enumerate()
                .map(|(i, c)| CardCombatState::new(c, Some(i)))
                .collect(),
            discard_pile: Vec::with_capacity(deck.len()),
            exhaust_pile: Vec::new(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &CardCombatState> {
        self.hand
            .iter()
            .chain(&self.draw_pile)
            .chain(&self.discard_pile)
            .chain(&self.exhaust_pile)
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut CardCombatState> {
        self.hand
            .iter_mut()
            .chain(&mut self.draw_pile)
            .chain(&mut self.discard_pile)
            .chain(&mut self.exhaust_pile)
    }

    fn sanitized(cards: &[CardCombatState]) -> Vec<CardCombatState> {
        let mut result = cards.to_vec();
        result.sort_by_key(|c| c.deck_index);
        result
    }

    pub fn sanitized_draw_pile(&self) -> Vec<CardCombatState> {
        Self::sanitized(&self.draw_pile)
    }

    pub fn sanitized_discard_pile(&self) -> Vec<CardCombatState> {
        Self::sanitized(&self.discard_pile)
    }

    pub fn sanitized_exhaust_pile(&self) -> Vec<CardCombatState> {
        Self::sanitized(&self.exhaust_pile)
    }
}
