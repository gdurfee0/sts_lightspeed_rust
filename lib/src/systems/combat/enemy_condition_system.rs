use crate::components::{Effect, EffectQueue};
use crate::data::{EnemyCondition, EnemyEffect, PlayerCondition};
use crate::systems::enemy::{EnemyParty, EnemyState};

pub struct EnemyConditionSystem;

impl EnemyConditionSystem {
    /// Ticks down the conditions at the start of the enemies' turn.
    pub fn start_turn(enemy_party: &mut EnemyParty) {
        for maybe_enemy in enemy_party.0.iter_mut() {
            let enemy_died = if let Some(enemy_state) = maybe_enemy {
                enemy_state
                    .conditions
                    .retain_mut(|c| c.start_turn(&mut enemy_state.strength));
                enemy_state.is_dead()
            } else {
                false
            };
            if enemy_died {
                *maybe_enemy = None;
            }
        }
    }

    /// Ticks down the conditions at the end of the enemies' turn.
    pub fn end_turn(enemy_party: &mut EnemyParty) {
        for maybe_enemy in enemy_party.0.iter_mut() {
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

    /// Triggers effects from the enemy's conditions when it dies.
    pub fn on_death(enemy: &mut EnemyState, effect_queue: &mut EffectQueue) {
        for condition in enemy.conditions.iter() {
            if let EnemyCondition::SporeCloud(stacks) = condition {
                effect_queue.push_front(Effect::EnemyState(EnemyEffect::Inflict(
                    PlayerCondition::Vulnerable(*stacks),
                )));
            }
        }
    }
}
