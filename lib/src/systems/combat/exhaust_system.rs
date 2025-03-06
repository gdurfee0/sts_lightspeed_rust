use anyhow::Error;

use crate::components::{CardCombatState, Effect, Interaction, Notification};
use crate::systems::base::{CombatContext, RelicSystem};
use crate::types::HandIndex;

use super::player_condition_system::PlayerConditionSystem;

pub struct ExhaustSystem;

impl ExhaustSystem {
    /// Exhausts the indicated card and notifies the player of the change. Queues any effects
    /// that should be applied as a result of the player's relics and conditions.
    pub fn push<I: Interaction>(
        ctx: &mut CombatContext<I>,
        hand_index: HandIndex,
        combat_card: CardCombatState,
    ) -> Result<(), Error> {
        RelicSystem::on_card_exhausted(ctx);
        PlayerConditionSystem::on_card_exhausted(ctx)?;
        if let Some(effect) = combat_card.details.on_exhaust.as_ref() {
            ctx.effect_queue.push_back(Effect::Card(effect));
        }
        ctx.pcs.cards.exhaust_pile.push(combat_card);
        ctx.comms
            .send_notification(Notification::CardExhausted(hand_index, combat_card))
    }
}
