use anyhow::Error;

use crate::components::{Interaction, Notification, PlayerCombatState};
use crate::data::{EnemyCondition, PlayerCondition};
use crate::systems::enemy::EnemyState;

use super::condition::Condition;

pub struct ConditionSystem;

impl ConditionSystem {
    /// Notifies the player of their current conditions (buffs and debuffs).
    pub fn notify_player<I: Interaction>(comms: &I, pcs: &PlayerCombatState) -> Result<(), Error> {
        comms.send_notification(Notification::Conditions(pcs.conditions.to_vec()))
    }

    /// Ticks down the conditions at the start of the player's turn.
    pub fn tick_down_at_start_of_player_turn<I: Interaction>(
        comms: &I,
        pcs: &mut PlayerCombatState,
    ) -> Result<(), Error> {
        pcs.conditions.retain_mut(|c| c.start_turn());
        Self::notify_player(comms, pcs)
    }

    /// Ticks down the conditions at the end of the player's turn.
    pub fn tick_down_at_end_of_player_turn<I: Interaction>(
        comms: &I,
        pcs: &mut PlayerCombatState,
    ) -> Result<(), Error> {
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

    /// Ticks down the conditions at the start of the enemies' turn.
    pub fn tick_down_at_start_of_enemies_turn(enemy_party: &mut [Option<EnemyState>]) {
        for maybe_enemy in enemy_party.iter_mut() {
            let enemy_died = if let Some(enemy) = maybe_enemy {
                enemy.conditions.retain_mut(|c| c.start_turn());
                enemy.is_dead()
            } else {
                false
            };
            if enemy_died {
                *maybe_enemy = None;
            }
        }
    }

    /// Ticks down the conditions at the end of the enemies' turn.
    pub fn tick_down_at_end_of_enemies_turn(enemy_party: &mut [Option<EnemyState>]) {
        for maybe_enemy in enemy_party.iter_mut() {
            let enemy_died = if let Some(enemy) = maybe_enemy {
                enemy.conditions.retain_mut(|c| c.end_turn());
                enemy.is_dead()
            } else {
                false
            };
            if enemy_died {
                *maybe_enemy = None;
            }
        }
    }

    /// Applies a condition to an enemy.
    pub fn apply_to_enemy(enemy: &mut EnemyState, condition: &EnemyCondition) {
        for preexisting_condition in enemy.conditions.iter_mut() {
            if preexisting_condition.merge(condition) {
                return;
            }
        }
        enemy.conditions.push(condition.clone());
    }

    /// Returns true if the player has the given condition.
    fn player_has_condition(pcs: &PlayerCombatState, condition: PlayerCondition) -> bool {
        pcs.conditions.iter().any(|c| *c == condition)
    }
}
