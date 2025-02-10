use anyhow::Error;

use crate::components::{Interaction, Notification, PlayerPersistentState};
use crate::data::Relic;
use crate::types::Gold;

use super::RelicSystem;

pub struct GoldSystem;

impl GoldSystem {
    /// Notifies the player of the current gold amount.
    pub fn notify_player<I: Interaction>(
        comms: &I,
        pps: &mut PlayerPersistentState,
    ) -> Result<(), Error> {
        comms.send_notification(Notification::Gold(pps.gold))
    }

    /// Checks if the player can afford the given gold cost.
    pub fn can_afford(pps: &PlayerPersistentState, gold_cost: Gold) -> bool {
        pps.gold >= gold_cost
    }

    /// Increases the player's gold amount by the given amount and notifies them of the change.
    pub fn increase_gold<I: Interaction>(
        comms: &I,
        pps: &mut PlayerPersistentState,
        amount: Gold,
    ) -> Result<(), Error> {
        if amount == 0 || pps.relics.contains(&Relic::Ectoplasm) {
            return Ok(());
        }
        pps.gold = pps.gold.saturating_add(amount);
        RelicSystem::on_gold_obtained(comms, pps)?;
        Self::notify_player(comms, pps)
    }

    /// Decreases the player's gold amount by the given amount and notifies them of the change.
    pub fn decrease_gold<I: Interaction>(
        comms: &I,
        pps: &mut PlayerPersistentState,
        amount: Gold,
    ) -> Result<(), Error> {
        pps.gold = pps.gold.saturating_sub(amount);
        Self::notify_player(comms, pps)
    }
}
