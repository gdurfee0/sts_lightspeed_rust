use anyhow::Error;

use crate::components::{
    DamageTaken, Effect, EffectQueue, Interaction, Notification, PlayerCombatState,
    PlayerPersistentState,
};
use crate::data::{Damage, PlayerCondition, PlayerEffect, Relic, TargetEffect};
use crate::systems::combat::PlayerConditionSystem;

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

    /// Applies any relic effects triggered by the start of combat.
    pub fn on_start_combat<I: Interaction>(
        comms: &I,
        pps: &PlayerPersistentState,
        pcs: &mut PlayerCombatState,
    ) -> Result<(), Error> {
        if pps.relics.contains(&Relic::SneckoEye) {
            PlayerConditionSystem::apply_to_player(comms, pcs, &PlayerCondition::Confused)?;
        }
        Ok(())
    }

    /// Applies any relic effects triggered by the end of combat.
    pub fn on_end_combat<I: Interaction>(
        comms: &I,
        pps: &mut PlayerPersistentState,
    ) -> Result<(), Error> {
        let mut has_burning_blood = false;
        let mut has_black_blood = false;
        for relic in pps.relics.iter() {
            if *relic == Relic::BurningBlood {
                has_burning_blood = true;
            }
            if *relic == Relic::BlackBlood {
                has_black_blood = true;
            }
        }
        if has_burning_blood {
            HealthSystem::heal(comms, pps, 6)?;
        }
        if has_black_blood {
            HealthSystem::heal(comms, pps, 12)?;
        }
        Ok(())
    }

    /// Modifies damage to be taken by the player based on the presence of certain relics.
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
                    effect_queue.push_back(Effect::PlayerState(PlayerEffect::ToAllEnemies(
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
            if *relic == Relic::BloodyIdol {
                has_bloody_idol = true;
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
