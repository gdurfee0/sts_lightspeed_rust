use anyhow::Error;

use crate::components::Interaction;
use crate::data::{PlayerEffect, Resource, TargetEffect};
use crate::systems::base::CombatContext;
use crate::types::EnemyIndex;

use super::block_system::BlockSystem;
use super::card_creation_system::CardCreationSystem;
use super::damage_calculator::DamageCalculator;
use super::draw_system::DrawSystem;
use super::effect_system::EffectSystem;
use super::enemy_condition_system::EnemyConditionSystem;
use super::player_condition_system::PlayerConditionSystem;

pub struct PlayerEffectSystem;

impl PlayerEffectSystem {
    pub fn process_player_effect<I: Interaction>(
        ctx: &mut CombatContext<I>,
        effect: &PlayerEffect,
    ) -> Result<(), Error> {
        match effect {
            PlayerEffect::Apply(player_condition) => {
                PlayerConditionSystem::apply_to_player(ctx, player_condition)
            }
            PlayerEffect::Conditional(_player_effect_condition, _player_effects) => todo!(),
            PlayerEffect::CreateCards(
                card_pool,
                card_selection,
                card_destination,
                cost_modifier,
            ) => CardCreationSystem::create_card(
                ctx,
                card_pool,
                card_selection,
                card_destination,
                cost_modifier,
            ),
            PlayerEffect::Draw(draw_count) => {
                for _ in 0..*draw_count {
                    DrawSystem::draw_one_card(ctx)?;
                }
                Ok(())
            }
            PlayerEffect::ForEachExhausted(_player_effects) => todo!(),
            PlayerEffect::Gain(resource) => Self::gain_resource(ctx, resource),
            PlayerEffect::Lose(_resource) => todo!(),
            PlayerEffect::ManipulateCards(
                _card_source,
                _card_selection,
                _card_destination,
                _cost_modifier,
            ) => todo!(),
            PlayerEffect::PlayThenExhaustTopCardOfDrawPile => todo!(),
            PlayerEffect::RampUpCardDamage(_) => todo!(),
            PlayerEffect::TakeDamage(_damage) => todo!(),
            PlayerEffect::ToAllEnemies(target_effect) => {
                for enemy_index in 0..ctx.enemy_party.0.len() {
                    if ctx.enemy_party.0[enemy_index].is_some() {
                        ctx.maybe_enemy_index = Some(enemy_index);
                        Self::to_target_effect(ctx, target_effect)?;
                    }
                    EffectSystem::process_effect_queue(ctx)?;
                }
                Ok(())
            }
            PlayerEffect::ToRandomEnemy(target_effect) => {
                ctx.maybe_enemy_index = Self::pick_random_enemy(ctx);
                Self::to_target_effect(ctx, target_effect)
            }
            PlayerEffect::ToSingleTarget(target_effect) => {
                assert!(ctx.maybe_enemy_index.is_some());
                Self::to_target_effect(ctx, target_effect)
            }
            PlayerEffect::Upgrade(_card_source, _card_selection) => todo!(),
        }
    }

    fn gain_resource<I: Interaction>(
        ctx: &mut CombatContext<I>,
        resource: &Resource,
    ) -> Result<(), Error> {
        match resource {
            Resource::Block(block) => {
                let calculated_block = DamageCalculator::calculate_block_gained(&ctx.pcs, *block);
                BlockSystem::gain_block(ctx, calculated_block)
            }
            Resource::CurrentBlockIsDoubled => todo!(),
            Resource::CurrentStrengthIsDoubled => todo!(),
            Resource::Energy(_) => todo!(),
            Resource::Gold(_) => todo!(),
            Resource::Hp(_) => todo!(),
            Resource::HpEqualToUnblockedDamage => todo!(),
            Resource::HpMax(_) => todo!(),
            Resource::Strength(_) => todo!(),
        }
    }

    fn pick_random_enemy<I: Interaction>(ctx: &mut CombatContext<I>) -> Option<EnemyIndex> {
        let living_enemies = ctx
            .enemy_party
            .0
            .iter()
            .enumerate()
            .filter(|(_, e)| e.is_some())
            .collect::<Vec<_>>();
        let living_index = ctx.misc_rng.gen_range(0..living_enemies.len());
        Some(living_enemies[living_index].0)
    }

    fn to_target_effect<I: Interaction>(
        ctx: &mut CombatContext<I>,
        effect: &TargetEffect,
    ) -> Result<(), Error> {
        if let Some(enemy_state) = ctx
            .maybe_enemy_index
            .and_then(|i| ctx.enemy_party.0.get_mut(i))
            .and_then(|maybe_enemy| maybe_enemy.as_mut())
        {
            match effect {
                TargetEffect::Conditional(_target_condition, _player_effects) => todo!(),
                TargetEffect::Deal(damage) => {
                    let calculated_damage = DamageCalculator::calculate_damage_inflicted(
                        &ctx.pcs,
                        Some(enemy_state),
                        damage,
                    );
                    BlockSystem::damage_enemy(
                        enemy_state,
                        calculated_damage,
                        &mut ctx.effect_queue,
                    );
                    Ok(())
                }
                TargetEffect::DealXTimes(_damage) => todo!(),
                TargetEffect::Inflict(enemy_condition) => {
                    EnemyConditionSystem::apply_to_enemy(enemy_state, enemy_condition);
                    Ok(())
                }
                TargetEffect::SapStrength(_) => todo!(),
            }
        } else {
            Ok(())
        }
    }
}
