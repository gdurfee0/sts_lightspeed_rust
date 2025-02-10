use anyhow::Error;

use crate::components::{Interaction, Notification};
use crate::systems::base::RelicSystem;
use crate::types::{Block, Health, Hp};

use super::damage_calculator::CalculatedDamage;

pub struct BlockSystem<'a, I: Interaction> {
    comms: &'a I,
    relic_system: &'a RelicSystem<'a, I>,
}

impl<'a, I: Interaction> BlockSystem<'a, I> {
    pub fn new(comms: &'a I) -> Self {
        Self { comms }
    }

    pub fn notify_player(&self, player_block: &Block) -> Result<(), Error> {
        self.comms
            .send_notification(Notification::Block(*player_block))
    }

    pub fn reset_at_start_of_player_turn(&self, player_block: &mut Block) -> Result<(), Error> {
        if *player_block > 0 {
            *player_block = 0;
            self.notify_player(player_block)?;
        }
        Ok(())
    }

    pub fn gain_block(&self, player_block: &mut Block, amount: Block) -> Result<(), Error> {
        *player_block = player_block.saturating_add(amount);
        self.notify_player(player_block)
    }

    pub fn damage_player(
        &self,
        player_block: &mut Block,
        player_health: &mut Health,
        damage: CalculatedDamage,
        effect_queue: &mut EffectQueue,
    ) -> Result<(), Error> {
        let damage_taken = Self::take_damage(player_block, player_health, damage);
        if damage_taken.blocked > 0 {
            self.comms
                .send_notification(Notification::DamageBlocked(damage_taken.blocked))?;
            self.comms
                .send_notification(Notification::Block(*player_block))?;
        }
        if damage_taken.hp_lost == 0 {
            return Ok(());
        }

        self.comms
            .send_notification(Notification::DamageTaken(damage_taken.hp_lost))?;

        self.comms
            .send_notification(Notification::Hp(player_health.0))
    }

    pub fn damage_enemy(
        &self,
        enemy_block: &mut Block,
        enemy_health: &mut Health,
        damage: CalculatedDamage,
        effect_queue: &mut EffectQueue,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn take_damage(
        block: &mut Block,
        health: &mut Health,
        damage: CalculatedDamage,
    ) -> DamageTaken {
        match damage {
            CalculatedDamage::Blockable(amount) | CalculatedDamage::BlockableNonAttack(amount) => {
                let blocked = (*block).min(amount);
                *block = block.saturating_sub(blocked);
                let hp_lost = amount - blocked;
                (*health).0 = health.0.saturating_sub(hp_lost);
                DamageTaken {
                    blocked,
                    hp_lost,
                    provokes_thorns: !matches!(damage, CalculatedDamage::BlockableNonAttack(_)),
                }
            }
            CalculatedDamage::HpLoss(amount) => {
                (*health).0 = health.0.saturating_sub(amount);
                DamageTaken {
                    blocked: 0,
                    hp_lost: amount,
                    provokes_thorns: false,
                }
            }
        }
    }
}
