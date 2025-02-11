use anyhow::Error;

use crate::components::{Interaction, Notification, PlayerCombatState};
use crate::data::EnergyCost;
use crate::types::Energy;

pub struct EnergySystem;

impl EnergySystem {
    /// Notifies the player of their current energy.
    pub fn notify_player<I: Interaction>(comms: &I, pcs: &PlayerCombatState) -> Result<(), Error> {
        comms.send_notification(Notification::Energy(pcs.energy))
    }

    /// Resets the player's energy to 3 at the start of their turn.
    pub fn start_turn<I: Interaction>(comms: &I, pcs: &mut PlayerCombatState) -> Result<(), Error> {
        pcs.energy = 3;
        Self::notify_player(comms, pcs)
    }

    /// Checks if the player can afford the specified energy cost.
    pub fn can_afford(pcs: &PlayerCombatState, energy_cost: EnergyCost) -> bool {
        match energy_cost {
            EnergyCost::Zero | EnergyCost::X => true,
            EnergyCost::One => pcs.energy >= 1,
            EnergyCost::Two => pcs.energy >= 2,
            EnergyCost::Three => pcs.energy >= 3,
            EnergyCost::ThreeMinusHpLossCount => pcs.energy + pcs.hp_loss_count as Energy >= 3,
            EnergyCost::Four => pcs.energy >= 4,
            EnergyCost::FourMinusHpLossCount => pcs.energy + pcs.hp_loss_count as Energy >= 4,
            EnergyCost::Five => pcs.energy >= 5,
        }
    }

    /// Spends the specified amount of energy from the player.
    pub fn spend<I: Interaction>(
        comms: &I,
        pcs: &mut PlayerCombatState,
        energy_cost: EnergyCost,
    ) -> Result<(), Error> {
        pcs.energy = match energy_cost {
            EnergyCost::Zero => pcs.energy,
            EnergyCost::One => pcs.energy.saturating_sub(1),
            EnergyCost::Two => pcs.energy.saturating_sub(2),
            EnergyCost::Three => pcs.energy.saturating_sub(3),
            EnergyCost::ThreeMinusHpLossCount => pcs
                .energy
                .saturating_sub(3u32.saturating_sub(pcs.hp_loss_count as Energy)),
            EnergyCost::Four => pcs.energy.saturating_sub(4),
            EnergyCost::FourMinusHpLossCount => pcs
                .energy
                .saturating_sub(4u32.saturating_sub(pcs.hp_loss_count as Energy)),
            EnergyCost::Five => pcs.energy.saturating_sub(5),
            EnergyCost::X => 0,
        };
        Self::notify_player(comms, pcs)
    }
}
