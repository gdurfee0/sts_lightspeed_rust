use anyhow::Error;

use crate::components::{Interaction, Notification, PlayerPersistentState};
use crate::types::{Hp, HpMax};

pub struct HealthSystem;

impl HealthSystem {
    /// Notifies the player of their current health.
    pub fn notify_player<I: Interaction>(
        comms: &I,
        pps: &PlayerPersistentState,
    ) -> Result<(), Error> {
        comms.send_notification(Notification::Health((pps.hp, pps.hp_max)))
    }

    /// Heals the player for the given amount and notifies them of the change.
    pub fn heal<I: Interaction>(
        comms: &I,
        pps: &mut PlayerPersistentState,
        amount: Hp,
    ) -> Result<(), Error> {
        pps.hp = pps.hp.saturating_add(amount).min(pps.hp_max);
        Self::notify_player(comms, pps)
    }

    /// Increases the player's health by the given amount and notifies them of the change.
    pub fn increase_hp<I: Interaction>(
        comms: &I,
        pps: &mut PlayerPersistentState,
        amount: Hp,
    ) -> Result<(), Error> {
        Self::heal(comms, pps, amount)
    }

    /// Decreases the player's health by the given amount and notifies them of the change.
    pub fn decrease_hp<I: Interaction>(
        comms: &I,
        pps: &mut PlayerPersistentState,
        amount: Hp,
    ) -> Result<(), Error> {
        pps.hp = pps.hp.saturating_sub(amount);
        Self::notify_player(comms, pps)
    }

    /// Increases the player's maximum health by the given amount and notifies them of the change.
    /// The player's HP is always increased by the same amount as their maximum HP.
    pub fn increase_hp_max<I: Interaction>(
        comms: &I,
        pps: &mut PlayerPersistentState,
        amount: HpMax,
    ) -> Result<(), Error> {
        pps.hp_max = pps.hp_max.saturating_add(amount);
        pps.hp = pps.hp.saturating_add(amount);
        Self::notify_player(comms, pps)
    }

    /// Decreases the player's maximum health by the given amount and notifies them of the change.
    /// The player's HP is never decreased except to stay at or below their maximum HP.
    pub fn decrease_hp_max<I: Interaction>(
        comms: &I,
        pps: &mut PlayerPersistentState,
        amount: HpMax,
    ) -> Result<(), Error> {
        pps.hp_max = pps.hp_max.saturating_sub(amount);
        pps.hp = pps.hp.min(pps.hp_max);
        Self::notify_player(comms, pps)
    }
}
