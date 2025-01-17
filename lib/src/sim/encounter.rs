use anyhow::Error;

use crate::data::Encounter;
use crate::enemy::{Enemy, EnemyAction, EnemyPartyGenerator};
use crate::player::{CombatController, PlayerAction, PlayerController};
use crate::rng::{Seed, StsRandom};
use crate::Effect;

pub struct EncounterSimulator<'a> {
    encounter: Encounter,
    seed_for_floor: Seed,
    ai_rng: StsRandom,
    misc_rng: &'a mut StsRandom,
    player: CombatController<'a>,
    enemy_party: [Option<Enemy>; 5],
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
            match self.player.choose_next_action(&self.enemy_party)? {
                PlayerAction::ApplyEffectChainToPlayer(effect_chain) => {
                    for effect in effect_chain.iter() {
                        match effect {
                            Effect::AddToDiscardPile(_) => todo!(),
                            Effect::AttackDamage(_) => {
                                unreachable!("Players do not attack themselves")
                            }
                            Effect::GainBlock(amount) => {
                                self.player.gain_block(*amount)?;
                            }
                            Effect::Inflict(_, _) => {
                                unreachable!("Players do not inflict debuffs on themselves")
                            }
                        }
                    }
                }
                PlayerAction::ApplyEffectChainToEnemy(effect_chain, enemy_index) => {
                    match self.enemy_party[enemy_index].as_mut() {
                        Some(enemy) => {
                            for effect in effect_chain.iter() {
                                enemy.apply_effect(effect);
                                if enemy.hp() == 0 {
                                    // Remove this enemy from the party
                                    self.player.enemy_died(enemy_index, enemy.enemy_type())?;
                                    self.enemy_party[enemy_index] = None;
                                    break;
                                }
                                self.player
                                    .update_enemy_status(enemy_index, enemy.status())?;
                            }
                        }
                        _ => unreachable!(),
                    }
                }
                PlayerAction::EndTurn => break,
                _ => todo!(),
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
        }
        //enemy_party.retain(|enemy| enemy.health().0 > 0);
        for maybe_enemy in self.enemy_party.iter_mut() {
            if let Some(enemy) = maybe_enemy.as_mut() {
                let action: &EnemyAction = enemy.next_action(&mut self.ai_rng);
                for effect in action.effects.iter() {
                    match effect {
                        Effect::AddToDiscardPile(cards) => {
                            self.player.add_to_discard_pile(cards)?;
                        }
                        Effect::AttackDamage(amount) => {
                            self.player.take_damage(*amount)?;
                        }
                        Effect::GainBlock(_) => todo!(),
                        Effect::Inflict(debuff, stacks) => {
                            self.player.apply_debuff(*debuff, *stacks)?
                        }
                    }
                    if enemy.hp() == 0 {
                        *maybe_enemy = None;
                        break;
                    }
                    if self.player.hp() == 0 {
                        return Ok(true);
                    }
                }
            }
        }
        Ok(self.enemy_party.iter().all(|enemy| enemy.is_none()))
    }
}
