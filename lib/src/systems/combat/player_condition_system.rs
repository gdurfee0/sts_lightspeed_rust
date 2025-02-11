use anyhow::Error;

use crate::components::{DamageTaken, EffectQueue, Interaction, Notification, PlayerCombatState};
use crate::data::PlayerCondition;

pub struct PlayerConditionSystem;

impl PlayerConditionSystem {
    /// Notifies the player of their current conditions (buffs and debuffs).
    pub fn notify_player<I: Interaction>(comms: &I, pcs: &PlayerCombatState) -> Result<(), Error> {
        comms.send_notification(Notification::Conditions(pcs.conditions.to_vec()))
    }

    /// Ticks down the conditions at the start of the player's turn.
    pub fn start_turn<I: Interaction>(comms: &I, pcs: &mut PlayerCombatState) -> Result<(), Error> {
        pcs.conditions.retain_mut(|c| c.start_turn());
        Self::notify_player(comms, pcs)
    }

    /// Ticks down the conditions at the end of the player's turn.
    pub fn end_turn<I: Interaction>(comms: &I, pcs: &mut PlayerCombatState) -> Result<(), Error> {
        pcs.conditions.retain_mut(|c| c.end_turn());
        Self::notify_player(comms, pcs)
    }

    /// Applies a condition to the player and notifies them of the change.
    pub fn apply_to_player<I: Interaction>(
        comms: &I,
        pcs: &mut PlayerCombatState,
        condition: &PlayerCondition,
    ) -> Result<(), Error> {
        for preexisting_condition in pcs.conditions.iter_mut() {
            if preexisting_condition.merge(condition) {
                return Self::notify_player(comms, pcs);
            }
        }
        pcs.conditions.push(condition.clone());
        Self::notify_player(comms, pcs)
    }

    /// Queues any effects triggered by the player taking damage.
    pub fn on_damage_taken<I: Interaction>(
        comms: &I,
        pcs: &mut PlayerCombatState,
        damage_taken: &DamageTaken,
        effect_queue: &mut EffectQueue,
    ) -> Result<(), Error> {
        pcs.conditions
            .retain_mut(|c| c.on_damage_taken(damage_taken, effect_queue));
        Self::notify_player(comms, pcs)
    }
}
