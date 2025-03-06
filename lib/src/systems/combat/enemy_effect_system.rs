use anyhow::Error;

use crate::components::Interaction;
use crate::data::{EnemyEffect, Resource};
use crate::systems::base::CombatContext;

use super::block_system::BlockSystem;
use super::card_creation_system::CardCreationSystem;
use super::damage_calculator::DamageCalculator;
use super::enemy_condition_system::EnemyConditionSystem;
use super::player_condition_system::PlayerConditionSystem;

pub struct EnemyEffectSystem;

impl EnemyEffectSystem {
    pub fn process_enemy_effect<I: Interaction>(
        ctx: &mut CombatContext<I>,
        effect: &EnemyEffect,
    ) -> Result<(), Error> {
        if let Some(enemy_state) = ctx
            .maybe_enemy_index
            .and_then(|i| ctx.enemy_party.0.get_mut(i))
            .and_then(|maybe_enemy| maybe_enemy.as_mut())
        {
            match effect {
                EnemyEffect::Apply(enemy_condition) => {
                    EnemyConditionSystem::apply_to_enemy(enemy_state, enemy_condition);
                }
                EnemyEffect::CreateCards(
                    card_pool,
                    card_selection,
                    card_destination,
                    cost_modifier,
                ) => {
                    CardCreationSystem::create_card(
                        ctx,
                        card_pool,
                        card_selection,
                        card_destination,
                        cost_modifier,
                    )?;
                }
                EnemyEffect::Deal(damage) => {
                    let damage = DamageCalculator::calculate_damage_inflicted(
                        enemy_state,
                        Some(&ctx.pcs),
                        damage,
                    );
                    BlockSystem::damage_player(ctx, damage)?;
                }
                EnemyEffect::Gain(Resource::Block(block)) => {
                    let calculated_block =
                        DamageCalculator::calculate_block_gained(enemy_state, *block);
                    enemy_state.block += calculated_block.amount;
                }
                EnemyEffect::Gain(Resource::Strength(strength)) => {
                    enemy_state.strength += strength;
                }
                EnemyEffect::Gain(invalid) => unreachable!("{:?}", invalid),
                EnemyEffect::Inflict(player_condition) => {
                    PlayerConditionSystem::apply_to_player(ctx, player_condition)?;
                }
            }
        }
        Ok(())
    }
}
