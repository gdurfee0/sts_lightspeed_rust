use anyhow::Error;

use crate::components::{CardCombatState, DamageTaken, Interaction, Notification};
use crate::data::PlayerCondition;
use crate::systems::base::CombatContext;

pub struct PlayerConditionSystem;

impl PlayerConditionSystem {
    /// Notifies the player of their current conditions (buffs and debuffs).
    pub fn notify_player<I: Interaction>(ctx: &mut CombatContext<I>) -> Result<(), Error> {
        ctx.comms
            .send_notification(Notification::Conditions(ctx.pcs.conditions.to_vec()))
    }

    /// Ticks down the conditions at the start of the player's turn.
    pub fn on_player_turn_started<I: Interaction>(ctx: &mut CombatContext<I>) -> Result<(), Error> {
        ctx.pcs.conditions.retain_mut(|c| c.on_turn_started());
        Self::notify_player(ctx)
    }

    /// Ticks down the conditions at the end of the player's turn.
    pub fn on_player_turn_finished<I: Interaction>(
        ctx: &mut CombatContext<I>,
    ) -> Result<(), Error> {
        ctx.pcs.conditions.retain_mut(|c| c.on_turn_finished());
        Self::notify_player(ctx)
    }

    /// Applies a condition to the player and notifies them of the change.
    pub fn apply_to_player<I: Interaction>(
        ctx: &mut CombatContext<I>,
        condition: &PlayerCondition,
    ) -> Result<(), Error> {
        for preexisting_condition in ctx.pcs.conditions.iter_mut() {
            if preexisting_condition.merge(condition) {
                return Self::notify_player(ctx);
            }
        }
        ctx.pcs.conditions.push(condition.clone());
        Self::notify_player(ctx)
    }

    pub fn on_card_exhausted<I: Interaction>(ctx: &mut CombatContext<I>) -> Result<(), Error> {
        ctx.pcs
            .conditions
            .retain_mut(|c| c.on_card_exhausted(&mut ctx.effect_queue));
        Self::notify_player(ctx)
    }

    /// Queues any effects triggered by the player taking damage.
    pub fn on_damage_taken<I: Interaction>(
        ctx: &mut CombatContext<I>,
        damage_taken: &DamageTaken,
    ) -> Result<(), Error> {
        ctx.pcs
            .conditions
            .retain_mut(|c| c.on_damage_taken(damage_taken, &mut ctx.effect_queue));
        Self::notify_player(ctx)
    }

    /// Queues any effects triggered by the player playing a card.
    pub fn on_some_card_played<I: Interaction>(
        ctx: &mut CombatContext<I>,
        combat_card: &CardCombatState,
    ) -> Result<(), Error> {
        ctx.pcs
            .conditions
            .retain_mut(|c| c.on_some_card_played(combat_card, &mut ctx.effect_queue));
        Self::notify_player(ctx)
    }
}
