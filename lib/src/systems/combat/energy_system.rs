use anyhow::Error;

use crate::components::{Interaction, Notification};
use crate::data::EnergyCost;
use crate::types::Energy;

pub struct EnergySystem;

impl EnergySystem {
    pub fn notify_player(&self, energy: &Energy) -> Result<(), Error> {
        self.comms.send_notification(Notification::Energy(*energy))
    }

    pub fn reset_at_start_of_player_turn(&self, energy: &mut Energy) {
        *energy = 3;
    }

    pub fn can_afford(
        &self,
        energy: &Energy,
        hp_loss_count: &usize,
        energy_cost: EnergyCost,
    ) -> bool {
        match energy_cost {
            EnergyCost::Zero | EnergyCost::X => true,
            EnergyCost::One => *energy >= 1,
            EnergyCost::Two => *energy >= 2,
            EnergyCost::Three => *energy >= 3,
            EnergyCost::ThreeMinusHpLossCount => *energy + *hp_loss_count as Energy >= 3,
            EnergyCost::Four => *energy >= 4,
            EnergyCost::FourMinusHpLossCount => *energy + *hp_loss_count as Energy >= 4,
            EnergyCost::Five => *energy >= 5,
        }
    }

    pub fn spend(
        &self,
        energy: &mut Energy,
        hp_loss_count: &mut usize,
        energy_cost: EnergyCost,
    ) -> Result<(), Error> {
        *energy = match energy_cost {
            EnergyCost::Zero => *energy,
            EnergyCost::One => (*energy).saturating_sub(1),
            EnergyCost::Two => (*energy).saturating_sub(2),
            EnergyCost::Three => (*energy).saturating_sub(3),
            EnergyCost::ThreeMinusHpLossCount => {
                (*energy).saturating_sub(3u32.saturating_sub(*hp_loss_count as Energy))
            }
            EnergyCost::Four => (*energy).saturating_sub(4),
            EnergyCost::FourMinusHpLossCount => {
                (*energy).saturating_sub(4u32.saturating_sub(*hp_loss_count as Energy))
            }
            EnergyCost::Five => (*energy).saturating_sub(5),
            EnergyCost::X => 0,
        };
        self.notify_player(energy)
    }
}
