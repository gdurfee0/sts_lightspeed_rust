use anyhow::Error;

use crate::components::{
    DamageTaken, Effect, EffectQueue, Interaction, Notification, PlayerCombatState,
    PlayerPersistentState,
};
use crate::data::{
    Damage, EnemyCondition, EnemyEffect, PlayerCondition, PlayerEffect, TargetEffect,
};
use crate::systems::base::{HealthSystem, RelicSystem};
use crate::systems::enemy::{EnemyParty, EnemyState};
use crate::types::Block;

use super::damage_calculator::CalculatedDamage;

pub struct BlockSystem;

impl BlockSystem {
    /// Notifies the player of their current block amount.
    pub fn notify_player<I: Interaction>(comms: &I, pcs: &PlayerCombatState) -> Result<(), Error> {
        comms.send_notification(Notification::Block(pcs.block))
    }

    /// Resets the player's block to 0 at the start of their turn.
    pub fn start_player_turn<I: Interaction>(
        comms: &I,
        pcs: &mut PlayerCombatState,
    ) -> Result<(), Error> {
        if pcs.block > 0 {
            pcs.block = 0;
            Self::notify_player(comms, pcs)?;
        }
        Ok(())
    }

    /// Resets the enemyies' block to 0 at the start of their turn.
    pub fn start_enemy_turn(enemy_party: &mut EnemyParty) {
        for enemy in enemy_party.0.iter_mut().filter_map(|e| e.as_mut()) {
            enemy.block = 0;
        }
    }

    /// Gains block for the player.
    pub fn gain_block<I: Interaction>(
        comms: &I,
        pcs: &mut PlayerCombatState,
        amount: Block,
    ) -> Result<(), Error> {
        pcs.block = pcs.block.saturating_add(amount);
        Self::notify_player(comms, pcs)
    }

    /// Damages the player and notifies them of the change.
    pub fn damage_player<I: Interaction>(
        comms: &I,
        pps: &mut PlayerPersistentState,
        pcs: &mut PlayerCombatState,
        damage: CalculatedDamage,
        effect_queue: &mut EffectQueue,
    ) -> Result<(), Error> {
        let mut damage_taken = Self::damage_taken(pcs.block, damage);
        if damage_taken.blocked > 0 {
            pcs.block = pcs.block.saturating_sub(damage_taken.blocked);
            comms.send_notification(Notification::DamageBlocked(damage_taken.blocked))?;
            Self::notify_player(comms, pcs)?;
        }
        if damage_taken.provokes_thorns {
            for condition in pcs.conditions.iter_mut() {
                match condition {
                    PlayerCondition::FlameBarrier(hp) | PlayerCondition::Thorns(hp) => {
                        effect_queue.push_front(Effect::FromPlayerState(
                            PlayerEffect::ToSingleTarget(TargetEffect::Deal(
                                Damage::BlockableNonAttack(*hp),
                            )),
                        ));
                    }
                    _ => {}
                }
            }
        }
        if damage_taken.hp_lost == 0 {
            Ok(())
        } else {
            RelicSystem::modify_damage_taken_by_player(pps, &mut damage_taken);
            comms.send_notification(Notification::DamageTaken(damage_taken.hp_lost))?;
            HealthSystem::decrease_hp(comms, pps, damage_taken.hp_lost)?;
            Self::notify_player(comms, pcs)
        }
    }

    /// Inflicts the specified amount of damage on an enemy, provoking thorns if applicable.
    pub fn damage_enemy(
        enemy_state: &mut EnemyState,
        damage: CalculatedDamage,
        effect_queue: &mut EffectQueue,
    ) {
        let damage_taken = Self::damage_taken(enemy_state.block, damage);
        if damage_taken.provokes_thorns {
            for condition in enemy_state.conditions.iter_mut() {
                match condition {
                    EnemyCondition::Thorns(hp) => {
                        effect_queue.push_front(Effect::FromEnemyState(EnemyEffect::Deal(
                            Damage::BlockableNonAttack(*hp),
                        )));
                    }
                    _ => {}
                }
            }
        }
        if damage_taken.blocked > 0 {
            enemy_state.block = enemy_state.block.saturating_sub(damage_taken.blocked);
        }
        if damage_taken.hp_lost > 0 {
            enemy_state.hp = enemy_state.hp.saturating_sub(damage_taken.hp_lost);
        }
    }

    /// Helper method that calculates block and hp lost for a given damage amount.
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
