use anyhow::Error;

use crate::components::{
    DamageTaken, Effect, EffectQueue, Interaction, Notification, PlayerPersistentState,
};
use crate::data::{Damage, PlayerEffect, Relic, TargetEffect};

use super::HealthSystem;

pub struct RelicSystem;

impl RelicSystem {
    /// Notifies the player of their current relics.
    pub fn notify_player<I: Interaction>(
        comms: &I,
        pps: &PlayerPersistentState,
    ) -> Result<(), Error> {
        comms.send_notification(Notification::Relics(pps.relics.to_vec()))
    }

    /// Adds the supplied relic to the player's relics and notifies them of the change.
    pub fn obtain_relic<I: Interaction>(
        comms: &I,
        pps: &mut PlayerPersistentState,
        relic: Relic,
    ) -> Result<(), Error> {
        pps.relics.push(relic);
        Self::notify_player(comms, pps)
    }

    /// Replaces the indicated relic with the incoming relic and notifies the player of the change.
    pub fn replace_relic<I: Interaction>(
        comms: &I,
        pps: &mut PlayerPersistentState,
        relic_to_replace: Relic,
        incoming_relic: Relic,
    ) -> Result<(), Error> {
        let index = pps
            .relics
            .iter()
            .position(|r| *r == relic_to_replace)
            .expect("Relic to replace not found");
        pps.relics[index] = incoming_relic;
        Self::notify_player(comms, pps)
    }

    /// Returns the number of extra cards to draw at the start of the player's turn.
    pub fn extra_cards_to_draw_at_start_of_player_turn(pps: &PlayerPersistentState) -> usize {
        if pps.relics.contains(&Relic::SneckoEye) {
            2
        } else {
            0
        }
    }

    /// Processes damage to be taken by the player based on the presence of certain relics.

    pub fn modify_damage_taken_by_player(
        pps: &PlayerPersistentState,
        damage_taken: &mut DamageTaken,
    ) {
        let mut has_torii = false;
        let mut has_tungsten_rod = false;
        for relic in pps.relics.iter() {
            if *relic == Relic::Torii {
                has_torii = true;
            }
            if *relic == Relic::TungstenRod {
                has_tungsten_rod = true;
            }
        }
        if has_torii && (1..=5).contains(&damage_taken.hp_lost) {
            damage_taken.hp_lost = 1;
        }
        if has_tungsten_rod {
            damage_taken.hp_lost = damage_taken.hp_lost.saturating_sub(1);
        }
    }

    /// Queues any relic effects triggered by a card being exhausted.
    pub fn on_card_exhausted(pps: &PlayerPersistentState, effect_queue: &mut EffectQueue) {
        for relic in pps.relics.iter() {
            match relic {
                Relic::CharonsAshes => {
                    effect_queue.push_back(Effect::FromPlayerState(PlayerEffect::ToAllEnemies(
                        TargetEffect::Deal(Damage::BlockableNonAttack(3)),
                    )));
                }
                Relic::DeadBranch => todo!(),
                _ => {}
            }
        }
    }

    /// Queues any relic effects triggered by the player obtaining gold.
    pub fn on_gold_obtained<I: Interaction>(
        comms: &I,
        pps: &mut PlayerPersistentState,
    ) -> Result<(), Error> {
        let mut has_bloody_idol = false;
        for relic in pps.relics.iter() {
            match relic {
                Relic::BloodyIdol => {
                    has_bloody_idol = true;
                }
                _ => {}
            }
        }
        if has_bloody_idol {
            HealthSystem::heal(comms, pps, 5)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::data::IRONCLAD;

    use super::*;

    #[test]
    fn test_process_damage_taken_by_player() {
        let mut pps = PlayerPersistentState::new(IRONCLAD);
        pps.relics = vec![];
        let mut damage_taken = DamageTaken {
            blocked: 5,
            hp_lost: 5,
            provokes_thorns: false,
        };
        RelicSystem::modify_damage_taken_by_player(&pps, &mut damage_taken);
        assert_eq!(damage_taken.hp_lost, 5);

        pps.relics = vec![Relic::TungstenRod];
        let mut damage_taken = DamageTaken {
            blocked: 5,
            hp_lost: 5,
            provokes_thorns: false,
        };
        RelicSystem::modify_damage_taken_by_player(&pps, &mut damage_taken);
        assert_eq!(damage_taken.hp_lost, 4);

        pps.relics = vec![Relic::Torii];
        let mut damage_taken = DamageTaken {
            blocked: 5,
            hp_lost: 5,
            provokes_thorns: false,
        };
        RelicSystem::modify_damage_taken_by_player(&pps, &mut damage_taken);
        assert_eq!(damage_taken.hp_lost, 1);

        pps.relics = vec![Relic::Torii, Relic::TungstenRod];
        let mut damage_taken = DamageTaken {
            blocked: 5,
            hp_lost: 5,
            provokes_thorns: false,
        };
        RelicSystem::modify_damage_taken_by_player(&pps, &mut damage_taken);
        assert_eq!(damage_taken.hp_lost, 0);
    }
}
