use anyhow::Error;

use crate::components::{Effect, Interaction, Notification};
use crate::systems::base::CombatContext;

use super::enemy_condition_system::EnemyConditionSystem;
use super::enemy_effect_system::EnemyEffectSystem;
use super::player_effect_system::PlayerEffectSystem;

pub struct EffectSystem;

impl EffectSystem {
    /// Processes all effects in the queue.
    pub fn process_effect_queue<I: Interaction>(ctx: &mut CombatContext<I>) -> Result<(), Error> {
        while let Some(effect) = ctx.effect_queue.pop_front() {
            Self::process_effect(ctx, effect)?;
            if ctx.combat_should_end() {
                break;
            }
        }
        Ok(())
    }

    /// Handles the incoming effect (PlayerEffect or EnemyEffect).
    pub fn process_effect<I: Interaction>(
        ctx: &mut CombatContext<I>,
        effect: Effect,
    ) -> Result<(), Error> {
        match effect {
            Effect::Card(player_effect) => {
                PlayerEffectSystem::process_player_effect(ctx, player_effect)?;
            }
            Effect::EnemyPlaybook(enemy_effect) => {
                EnemyEffectSystem::process_enemy_effect(ctx, enemy_effect)?;
            }
            Effect::EnemyState(enemy_effect) => {
                EnemyEffectSystem::process_enemy_effect(ctx, &enemy_effect)?;
            }
            Effect::PlayerState(player_effect) => {
                PlayerEffectSystem::process_player_effect(ctx, &player_effect)?;
            }
        }
        if let Some(enemy_index) = ctx.maybe_enemy_index {
            let should_remove_enemy =
                if let Some(enemy_state) = ctx.enemy_party.0[enemy_index].as_mut() {
                    if enemy_state.is_dead() {
                        EnemyConditionSystem::on_enemy_death(enemy_state, &mut ctx.effect_queue);
                        ctx.comms.send_notification(Notification::EnemyDied(
                            enemy_index,
                            enemy_state.enemy,
                        ))?;
                        true
                    } else {
                        false
                    }
                } else {
                    false
                };
            if should_remove_enemy {
                ctx.enemy_party.0[enemy_index] = None;
            }
        }
        Ok(())
    }
}
