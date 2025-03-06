use crate::components::Interaction;
use crate::systems::base::CombatContext;

use super::block_system::BlockSystem;
use super::enemy_condition_system::EnemyConditionSystem;

pub struct EnemyCombatSystem;

impl EnemyCombatSystem {
    pub fn on_enemies_turn_started<I: Interaction>(ctx: &mut CombatContext<I>) {
        EnemyConditionSystem::on_enemies_turn_started(ctx);
        BlockSystem::on_enemies_turn_started(ctx);
    }

    pub fn on_enemies_turn_finished<I: Interaction>(ctx: &mut CombatContext<I>) {
        EnemyConditionSystem::on_enemies_turn_finished(ctx);
        for enemy_state in ctx.enemy_party.0.iter_mut().filter_map(|e| e.as_mut()) {
            enemy_state.advance_action(&mut ctx.enemy_rng);
        }
    }
}
