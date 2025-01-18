use anyhow::Error;

use crate::data::effect::{EnemyEffect, PlayerEffect};
use crate::data::encounter::Encounter;
use crate::enemy::{EnemyPartyGenerator, EnemyState, EnemyStatus};
use crate::player::{CombatAction, CombatController, PlayerController};
use crate::rng::{Seed, StsRandom};

pub struct EncounterSimulator<'a> {
    encounter: Encounter,
    seed_for_floor: Seed,
    ai_rng: StsRandom,
    misc_rng: &'a mut StsRandom,
    player: CombatController<'a>,
    enemy_party: [Option<EnemyState>; 5],
}

impl<'a> EncounterSimulator<'a> {
    pub fn new(
        seed_for_floor: Seed,
        encounter: Encounter,
        misc_rng: &'a mut StsRandom,
        player: &'a mut PlayerController,
    ) -> Self {
        let ai_rng = StsRandom::from(seed_for_floor);
        let combat_controller = player.start_combat(StsRandom::from(seed_for_floor));
        Self {
            encounter,
            seed_for_floor,
            ai_rng,
            misc_rng,
            player: combat_controller,
            enemy_party: [None, None, None, None, None],
        }
    }

    pub fn run(mut self) -> Result<bool, Error> {
        println!(
            "[EncounterSimulator] Running encounter: {:?}",
            self.encounter
        );
        EnemyPartyGenerator::new(
            self.seed_for_floor,
            self.encounter,
            &mut self.ai_rng,
            self.misc_rng,
        )
        .generate(&mut self.enemy_party);

        loop {
            if self.conduct_player_turn()? {
                return Ok(self.player.hp() > 0);
            }
            if self.conduct_enemy_turn()? {
                return Ok(self.player.hp() > 0);
            }
        }
    }

    // Returns true iff the battle is over
    fn conduct_player_turn(&mut self) -> Result<bool, Error> {
        self.player.start_turn()?;
        loop {
            let enemy_statuses = self
                .enemy_party
                .iter()
                .map(|enemy| enemy.as_ref().map(|enemy| enemy.status()))
                .collect::<Vec<Option<EnemyStatus>>>();
            match self.player.choose_next_action(&enemy_statuses)? {
                // 1. Iterate over the effect chain and apply effects to all enemies
                //    a. If an effect provokes a reaction, apply the reaction to the appropriate
                //       entity
                CombatAction::PlayCard(_, _) => todo!(),
                // 1. Iterate over the effect chain and apply effects to one enemy
                //    a. If an effect provokes a reaction, apply the reaction to the appropriate
                //       entity
                CombatAction::PlayCardAgainstEnemy(_, _, _) => todo!(),
                CombatAction::Potion(_) => todo!(),
                CombatAction::EndTurn => break,
            }
            if self.enemy_party.iter().all(|enemy| enemy.is_none()) {
                // Battle is over!
                return Ok(true);
            }
            self.player.discard_card_just_played()?;
        }
        self.player.end_turn()?;
        Ok(false)
    }

    // Returns true iff the battle is over
    fn conduct_enemy_turn(&mut self) -> Result<bool, Error> {
        for enemy in self
            .enemy_party
            .iter_mut()
            .filter_map(|enemy| enemy.as_mut())
        {
            // TODO: check for death and remove
            let _ = enemy.start_turn();
            //enemy_party.retain(|enemy| enemy.health().0 > 0);
        }
        for maybe_enemy in self.enemy_party.iter_mut() {
            if let Some(enemy) = maybe_enemy.as_mut() {
                for effect in enemy.next_action(&mut self.ai_rng).effect_chain.iter() {
                    // TODO: reactions
                    //self.player.receive(&effect);
                    println!("[EncounterSimulator] Applying effect: {:?}", effect);
                    if enemy.hp() == 0 {
                        *maybe_enemy = None;
                        break;
                    }
                    if self.player.hp() == 0 {
                        break;
                    }
                }
            }
        }
        Ok(self.player.hp() == 0 || self.enemy_party.iter().all(|enemy| enemy.is_none()))
    }
}
