use std::collections::VecDeque;

use anyhow::Error;

use crate::components::{
    CardCombatState, Effect, EffectQueue, Interaction, Notification, PlayerCombatState,
    PlayerPersistentState,
};
use crate::types::HandIndex;

use super::exhaust_system::ExhaustSystem;

pub struct DiscardSystem;

impl DiscardSystem {
    /// Discards the player's hand at the end of their turn.
    pub fn discard_hand_at_end_of_player_turn<I: Interaction>(
        comms: &I,
        pps: &PlayerPersistentState,
        pcs: &mut PlayerCombatState,
        effect_queue: &mut EffectQueue,
    ) -> Result<(), Error> {
        // Emulating the game's behavior
        let mut retained_cards = VecDeque::with_capacity(pcs.cards.hand.len());
        while let Some(combat_card) = pcs.cards.hand.pop() {
            let hand_index = pcs.cards.hand.len();
            if let Some(effect) = combat_card.details.if_in_hand_at_end_of_turn.as_ref() {
                effect_queue.push_back(Effect::FromCard(effect));
            }
            if combat_card.details.ethereal {
                ExhaustSystem::exhaust_card(
                    comms,
                    pps,
                    pcs,
                    hand_index,
                    combat_card,
                    effect_queue,
                )?;
            } else if combat_card.details.retain {
                // TODO: Cost reduction on retain
                retained_cards.push_front(combat_card);
            } else {
                Self::discard_card(comms, pcs, hand_index, combat_card)?;
            }
        }
        pcs.cards.hand.extend(retained_cards);
        Ok(())
    }

    /// Discards the indicated card and notifies the player of the change.
    pub fn discard_card<I: Interaction>(
        comms: &I,
        pcs: &mut PlayerCombatState,
        hand_index: HandIndex,
        combat_card: CardCombatState,
    ) -> Result<(), Error> {
        pcs.cards.discard_pile.push(combat_card);
        comms.send_notification(Notification::CardDiscarded(hand_index, combat_card.card))
    }
}
