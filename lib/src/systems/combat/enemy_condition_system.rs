use crate::components::{DamageTaken, EffectQueue, Interaction};
use crate::data::EnemyCondition;
use crate::systems::base::{CombatContext, EnemyState};

pub struct EnemyConditionSystem;

impl EnemyConditionSystem {
    /// Ticks down the conditions at the start of the enemies' turn.
    pub fn on_enemies_turn_started<I: Interaction>(ctx: &mut CombatContext<I>) {
        for maybe_enemy in ctx.enemy_party.0.iter_mut() {
            let enemy_died = if let Some(enemy_state) = maybe_enemy {
                enemy_state.conditions.retain_mut(|c| c.on_turn_started());
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
    pub fn on_enemies_turn_finished<I: Interaction>(ctx: &mut CombatContext<I>) {
        for maybe_enemy in ctx.enemy_party.0.iter_mut() {
            let enemy_died = if let Some(enemy) = maybe_enemy {
                enemy
                    .conditions
                    .retain_mut(|c| c.on_turn_finished(&mut enemy.strength));
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

    /// Queues any effects triggered by the enemy taking damage.
    pub fn on_damage_taken(
        enemy: &mut EnemyState,
        damage_taken: &DamageTaken,
        effect_queue: &mut EffectQueue,
    ) {
        enemy
            .conditions
            .retain_mut(|c| c.on_damage_taken(&mut enemy.block, damage_taken, effect_queue));
    }

    /// Triggers effects from the enemy's conditions when it dies.
    pub fn on_enemy_death(enemy: &mut EnemyState, effect_queue: &mut EffectQueue) {
        enemy.conditions.retain_mut(|c| c.on_death(effect_queue));
    }
}
