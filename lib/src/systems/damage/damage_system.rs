use crate::types::{Block, Hp};

use super::damage_calculator::CalculatedDamage;

pub struct DamageTaken {
    pub blocked: Block,
    pub hp_lost: Hp,
    pub provokes_thorns: bool,
}

pub struct DamageSystem;

impl DamageSystem {
    pub fn take_damage(hp: &mut Hp, block: &mut Block, damage: CalculatedDamage) -> DamageTaken {}
}

/*

impl DamageSystem {
    pub fn take_damage(
        hp: &mut Hp,
        block: &mut Block,
        hp_loss_count: Option<&mut usize>,
        comms: Option<&PlayerInteraction>,
        damage: FinalCalculatedDamage,
    ) -> Result<(), Error> {
        match damage {
            FinalCalculatedDamage::Blockable(amount) => {
                Self::take_blockable_damage(hp, block, hp_loss_count, comms, amount)
            }
            FinalCalculatedDamage::HpLoss(amount) => {
                Self::take_unblockable_damage(hp, hp_loss_count, comms, amount)
            }
        }
    }

    fn take_blockable_damage(
        hp: &mut Hp,
        block: &mut Block,
        hp_loss_count: Option<&mut usize>,
        comms: Option<&PlayerInteraction>,
        amount: Hp,
    ) -> Result<(), Error> {
        if amount <= *block {
            *block -= amount;
            if let Some(comms) = comms {
                comms.send_notification(Notification::DamageBlocked(amount))?;
                comms.send_notification(Notification::BlockLost(amount))?;
                comms.send_notification(Notification::Block(*block))?;
            }
            Ok(())
        } else if *block > 0 {
            let unblocked_damage = amount - *block;
            *block = 0;
            if let Some(comms) = comms {
                comms.send_notification(Notification::DamageBlocked(*block))?;
                comms.send_notification(Notification::BlockLost(*block))?;
                comms.send_notification(Notification::Block(0))?;
            }
            Self::take_unblockable_damage(hp, hp_loss_count, comms, unblocked_damage)
        } else {
            Self::take_unblockable_damage(hp, hp_loss_count, comms, amount)
        }
    }

    fn take_unblockable_damage(
        hp: &mut Hp,
        hp_loss_count: Option<&mut usize>,
        comms: Option<&PlayerInteraction>,
        amount: Hp,
    ) -> Result<(), Error> {
        *hp = hp.saturating_sub(amount);
        if amount > 0 {
            if let Some(hp_loss_count) = hp_loss_count {
                *hp_loss_count += 1;
            }
        }
        if let Some(comms) = comms {
            comms.send_notification(Notification::DamageTaken(amount))?;
            comms.send_notification(Notification::Hp(*hp))?;
        }
        Ok(())
    }
}


*/
