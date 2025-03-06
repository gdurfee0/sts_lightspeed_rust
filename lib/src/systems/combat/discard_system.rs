use std::collections::VecDeque;

use anyhow::Error;

use crate::components::{CardCombatState, Effect, Interaction, Notification};
use crate::systems::base::CombatContext;
use crate::types::HandIndex;

use super::exhaust_system::ExhaustSystem;

pub struct DiscardSystem;

impl DiscardSystem {
    /// Discards the player's hand at the end of their turn.
    pub fn on_player_turn_finished<I: Interaction>(
        ctx: &mut CombatContext<I>,
    ) -> Result<(), Error> {
        // Emulating the game's behavior
        let mut retained_cards = VecDeque::with_capacity(ctx.pcs.cards.hand.len());
        while let Some(combat_card) = ctx.pcs.cards.hand.pop() {
            let hand_index = ctx.pcs.cards.hand.len();
            if let Some(effect) = combat_card.details.on_linger.as_ref() {
                ctx.effect_queue.push_back(Effect::Card(effect));
            }
            if combat_card.details.ethereal {
                ExhaustSystem::push(ctx, hand_index, combat_card)?;
            } else if combat_card.details.retain {
                // TODO: Cost reduction on retain
                retained_cards.push_front(combat_card);
            } else {
                Self::push(ctx, hand_index, combat_card)?;
            }
        }
        ctx.pcs.cards.hand.extend(retained_cards.iter());
        drop(retained_cards);
        for combat_card in ctx.pcs.cards.iter_mut() {
            combat_card.cost_this_turn = combat_card.cost_this_combat;
        }
        Ok(())
    }

    /// Discards the indicated card and notifies the player of the change.
    pub fn push<I: Interaction>(
        ctx: &mut CombatContext<I>,
        hand_index: HandIndex,
        combat_card: CardCombatState,
    ) -> Result<(), Error> {
        ctx.pcs.cards.discard_pile.push(combat_card);
        // TODO: On discard?
        ctx.comms
            .send_notification(Notification::CardDiscarded(hand_index, combat_card))
    }
}
