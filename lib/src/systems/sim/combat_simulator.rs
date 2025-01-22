use anyhow::Error;

use crate::components::EnemyStatus;
use crate::data::{CardDetails, Encounter, EnemyCondition, EnemyEffect, PlayerEffect};
use crate::systems::rng::{Seed, StsRandom};
use crate::types::{AttackDamage, Block, EnemyIndex};

use super::enemy_in_combat::EnemyInCombat;
use super::party_generator::EnemyPartyGenerator;
use super::player::Player;
use super::player_in_combat::{CombatAction, PlayerInCombat};

pub struct CombatSimulator<'a> {
    encounter: Encounter,
    seed_for_floor: Seed,
    ai_rng: StsRandom,
    misc_rng: &'a mut StsRandom,
    player_in_combat: PlayerInCombat<'a>,
    enemy_party: [Option<EnemyInCombat>; 5],
}

impl<'a> CombatSimulator<'a> {
    pub fn new(
        seed_for_floor: Seed,
        encounter: Encounter,
        misc_rng: &'a mut StsRandom,
        player: &'a mut Player,
    ) -> Self {
        let ai_rng = StsRandom::from(seed_for_floor);
        let player_in_combat = PlayerInCombat::new(StsRandom::from(seed_for_floor), player);
        Self {
            encounter,
            seed_for_floor,
            ai_rng,
            misc_rng,
            player_in_combat,
            enemy_party: [None, None, None, None, None],
        }
    }

    pub fn run(mut self) -> Result<bool, Error> {
        println!(
            "[EncounterSimulator] Running encounter: {:?}",
            self.encounter
        );
        self.player_in_combat.start_combat()?;
        EnemyPartyGenerator::new(
            self.seed_for_floor,
            self.encounter,
            &mut self.ai_rng,
            self.misc_rng,
        )
        .generate(&mut self.enemy_party);
        loop {
            self.conduct_player_turn()?;
            if self.combat_should_end() {
                break;
            }
            self.conduct_enemies_turn()?;
            if self.combat_should_end() {
                break;
            }
        }
        let victorious = !self.player_in_combat.is_dead();
        self.player_in_combat.end_combat()?;
        Ok(victorious)
    }

    fn combat_should_end(&self) -> bool {
        self.player_in_combat.is_dead() || self.enemy_party.iter().all(|enemy| enemy.is_none())
    }

    fn conduct_player_turn(&mut self) -> Result<(), Error> {
        self.player_in_combat.start_turn()?;
        loop {
            let enemy_statuses = self
                .enemy_party
                .iter()
                .map(|maybe_enemy| maybe_enemy.as_ref().map(EnemyStatus::from))
                .collect::<Vec<_>>();
            match self.player_in_combat.choose_next_action(&enemy_statuses)? {
                CombatAction::PlayCard(card_details) => {
                    self.play_card(card_details)?;
                    if self.combat_should_end() {
                        return Ok(());
                    }
                    self.player_in_combat.dispose_card_just_played()?;
                }
                CombatAction::PlayCardAgainstEnemy(card_details, enemy_index) => {
                    self.play_card_against_enemy(card_details, enemy_index)?;
                    if self.combat_should_end() {
                        return Ok(());
                    }
                    self.player_in_combat.dispose_card_just_played()?;
                }
                CombatAction::Potion(_) => todo!(),
                CombatAction::EndTurn => break,
            }
        }
        self.player_in_combat.end_turn()?;
        Ok(())
    }

    fn conduct_enemies_turn(&mut self) -> Result<(), Error> {
        for maybe_enemy in self.enemy_party.iter_mut() {
            if let Some(enemy) = maybe_enemy.as_mut() {
                for effect in enemy.next_action(&mut self.ai_rng).effect_chain().iter() {
                    // TODO: reactions
                    match effect {
                        EnemyEffect::AddToDiscardPile(cards) => {
                            self.player_in_combat.add_to_discard_pile(cards)?;
                        }
                        EnemyEffect::Apply(condition) => {
                            self.player_in_combat.apply_condition(condition)?;
                        }
                        EnemyEffect::ApplyToSelf(enemy_condition) => {
                            enemy.apply_condition(enemy_condition);
                        }
                        EnemyEffect::DealDamage(amount) => {
                            self.player_in_combat.take_damage(Self::incoming_damage(
                                &self.player_in_combat,
                                enemy,
                                *amount,
                            ))?;
                        }
                    }
                    if enemy.state.is_dead() {
                        *maybe_enemy = None;
                        break;
                    }
                    if self.player_in_combat.is_dead() {
                        break;
                    }
                }
            }
        }
        for enemy in self
            .enemy_party
            .iter_mut()
            .filter_map(|enemy| enemy.as_mut())
        {
            enemy.end_turn();
        }

        Ok(())
    }

    // TODO: reactions
    fn play_card(&mut self, card_details: &'static CardDetails) -> Result<(), Error> {
        for effect in card_details.effect_chain.iter() {
            match effect {
                PlayerEffect::Apply(_) => unreachable!(
                    "Debuff should be handled by play_card_against_enemy, {:?}",
                    card_details
                ),
                PlayerEffect::ApplyToAll(condition) => {
                    self.apply_to_all_enemies(condition)?;
                }
                PlayerEffect::ApplyToSelf(player_condition) => {
                    self.player_in_combat.apply_condition(player_condition)?;
                }
                PlayerEffect::DealDamage(_) => unreachable!(
                    "DealDamage should be handled by play_card_against_enemy, {:?}",
                    card_details
                ),
                PlayerEffect::DealDamageToAll(amount) => {
                    self.attack_all_enemies(*amount)?;
                }
                PlayerEffect::GainBlock(amount) => {
                    self.player_in_combat
                        .gain_block(Self::incoming_block(&self.player_in_combat, *amount))?;
                }
                PlayerEffect::UpgradeOneCardInCombat() => todo!(),
            }
            if self.combat_should_end() {
                break;
            }
        }
        Ok(())
    }

    fn play_card_against_enemy(
        &mut self,
        card_details: &'static CardDetails,
        enemy_index: EnemyIndex,
    ) -> Result<(), Error> {
        for effect in card_details.effect_chain.iter() {
            match effect {
                PlayerEffect::Apply(condition) => {
                    self.apply_to_enemy(enemy_index, condition)?;
                }
                PlayerEffect::DealDamage(amount) => {
                    self.attack_enemy(enemy_index, *amount)?;
                }
                _ => unreachable!(
                    "Inappropriate card handled by play_card_against_enemy, {:?}",
                    card_details
                ),
            }
            if self.combat_should_end() {
                break;
            }
        }
        Ok(())
    }

    fn attack_enemy(&mut self, index: EnemyIndex, amount: AttackDamage) -> Result<(), Error> {
        // TODO: reaction
        if let Some(enemy) = self.enemy_party[index].as_mut() {
            enemy.take_damage(Self::outgoing_damage(&self.player_in_combat, enemy, amount));
            let enemy_status = EnemyStatus::from(&*enemy);
            let enemy_type = enemy_status.enemy_type;
            self.player_in_combat
                .update_enemy_status(index, enemy_status)?;
            if enemy.state.is_dead() {
                self.player_in_combat.send_enemy_died(index, enemy_type)?;
                self.enemy_party[index] = None;
            }
        }
        Ok(())
    }

    fn attack_all_enemies(&mut self, amount: AttackDamage) -> Result<(), Error> {
        for index in 0..5 {
            self.attack_enemy(index, amount)?;
        }
        Ok(())
    }

    fn apply_to_enemy(
        &mut self,
        index: EnemyIndex,
        condition: &EnemyCondition,
    ) -> Result<(), Error> {
        if let Some(enemy) = self.enemy_party[index].as_mut() {
            enemy.apply_condition(condition);
            self.player_in_combat
                .update_enemy_status(index, EnemyStatus::from(&*enemy))?;
        }
        Ok(())
    }

    fn apply_to_all_enemies(&mut self, condition: &EnemyCondition) -> Result<(), Error> {
        for index in 0..5 {
            self.apply_to_enemy(index, condition)?;
        }
        Ok(())
    }

    fn incoming_block(player: &PlayerInCombat, amount: Block) -> Block {
        if player.state.is_frail() {
            (amount as f32 * 0.75).floor() as u32
        } else {
            amount
        }
    }

    fn incoming_damage(
        player: &PlayerInCombat,
        enemy: &EnemyInCombat,
        amount: AttackDamage,
    ) -> AttackDamage {
        let amount = amount + enemy.state.strength as AttackDamage;
        let enemy_modified_amount = if enemy.state.is_weak() {
            (amount as f32 * 0.75).floor() as u32
        } else {
            amount
        };
        if player.state.is_vulnerable() {
            (enemy_modified_amount as f32 * 1.5).floor() as u32
        } else {
            enemy_modified_amount
        }
    }

    fn outgoing_damage(
        player: &PlayerInCombat,
        enemy: &EnemyInCombat,
        amount: AttackDamage,
    ) -> AttackDamage {
        let caster_modified_mount = if player.state.is_weak() {
            (amount as f32 * 0.75).floor() as u32
        } else {
            amount
        };
        if enemy.state.is_vulnerable() {
            (caster_modified_mount as f32 * 1.5).floor() as u32
        } else {
            caster_modified_mount
        }
    }
}
