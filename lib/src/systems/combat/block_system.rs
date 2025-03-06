use anyhow::Error;

use crate::components::{DamageTaken, EffectQueue, Interaction, Notification};
use crate::systems::base::{CombatContext, EnemyState, HealthSystem, RelicSystem};
use crate::types::Block;

use super::damage_calculator::{CalculatedBlock, CalculatedDamage};
use super::enemy_condition_system::EnemyConditionSystem;
use super::player_condition_system::PlayerConditionSystem;

pub struct BlockSystem;

impl BlockSystem {
    /// Notifies the player of their current block amount.
    pub fn notify_player<I: Interaction>(ctx: &mut CombatContext<I>) -> Result<(), Error> {
        ctx.comms
            .send_notification(Notification::Block(ctx.pcs.block))
    }

    /// Resets the player's block to 0 at the start of their turn.
    pub fn on_player_turn_started<I: Interaction>(ctx: &mut CombatContext<I>) -> Result<(), Error> {
        if ctx.pcs.block > 0 {
            ctx.pcs.block = 0;
            Self::notify_player(ctx)?;
        }
        Ok(())
    }

    /// Resets the enemies' block to 0 at the start of their turn.
    pub fn on_enemies_turn_started<I: Interaction>(ctx: &mut CombatContext<I>) {
        for enemy in ctx.enemy_party.0.iter_mut().filter_map(|e| e.as_mut()) {
            enemy.block = 0;
        }
    }

    /// Gains block for the player.
    pub fn gain_block<I: Interaction>(
        ctx: &mut CombatContext<I>,
        calculated_block: CalculatedBlock,
    ) -> Result<(), Error> {
        ctx.pcs.block = ctx.pcs.block.saturating_add(calculated_block.amount);
        ctx.comms
            .send_notification(Notification::BlockGained(calculated_block.amount))?;
        Self::notify_player(ctx)
    }

    /// Damages the player and notifies them of the change.
    pub fn damage_player<I: Interaction>(
        ctx: &mut CombatContext<I>,
        damage: CalculatedDamage,
    ) -> Result<(), Error> {
        let mut damage_taken = Self::damage_taken(ctx.pcs.block, damage);
        if damage_taken.blocked > 0 {
            ctx.pcs.block = ctx.pcs.block.saturating_sub(damage_taken.blocked);
            ctx.comms
                .send_notification(Notification::DamageBlocked(damage_taken.blocked))?;
            Self::notify_player(ctx)?;
        }
        if damage_taken.hp_lost == 0 {
            Ok(())
        } else {
            RelicSystem::modify_damage_taken_by_player(ctx.pcs.pps, &mut damage_taken);
            PlayerConditionSystem::on_damage_taken(ctx, &damage_taken)?;
            ctx.comms
                .send_notification(Notification::DamageTaken(damage_taken.hp_lost))?;
            HealthSystem::decrease_hp(ctx.comms, ctx.pcs.pps, damage_taken.hp_lost)?;
            Self::notify_player(ctx)
        }
    }

    /// Inflicts the specified amount of damage on an enemy, provoking thorns if applicable.
    pub fn damage_enemy(
        enemy_state: &mut EnemyState,
        damage: CalculatedDamage,
        effect_queue: &mut EffectQueue,
    ) {
        let damage_taken = Self::damage_taken(enemy_state.block, damage);
        EnemyConditionSystem::on_damage_taken(enemy_state, &damage_taken, effect_queue);
        if damage_taken.blocked > 0 {
            enemy_state.block = enemy_state.block.saturating_sub(damage_taken.blocked);
        }
        if damage_taken.hp_lost > 0 {
            enemy_state.hp = enemy_state.hp.saturating_sub(damage_taken.hp_lost);
        }
    }

    /// Helper method that calculates block and HP lost for a given damage amount.
    /// Used for both damage to the player and damage to an enemy.
    fn damage_taken(block: Block, damage: CalculatedDamage) -> DamageTaken {
        match damage {
            CalculatedDamage::Blockable(amount) | CalculatedDamage::BlockableNonAttack(amount) => {
                let blocked = block.min(amount);
                let hp_lost = amount - blocked;
                DamageTaken {
                    blocked,
                    hp_lost,
                    provokes_thorns: !matches!(damage, CalculatedDamage::BlockableNonAttack(_)),
                }
            }
            CalculatedDamage::HpLoss(amount) => DamageTaken {
                blocked: 0,
                hp_lost: amount,
                provokes_thorns: false,
            },
        }
    }
}
