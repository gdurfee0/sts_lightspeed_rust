use anyhow::Error;

use crate::data::{CardDetails, Encounter, EnemyCondition, EnemyEffect, PlayerEffect};
use crate::enemy::{EnemyPartyGenerator, EnemyState, EnemyStatus};
use crate::player::{CombatAction, CombatController, PlayerController};
use crate::rng::{Seed, StsRandom};
use crate::types::{AttackDamage, Block, EnemyIndex};

const ENEMY_PARTY_SIZE_MAX: usize = 5;

pub struct EncounterSimulator<'a> {
    encounter: Encounter,
    seed_for_floor: Seed,
    ai_rng: StsRandom,
    misc_rng: &'a mut StsRandom,
    player: CombatController<'a>,
    enemy_party: [Option<EnemyState>; ENEMY_PARTY_SIZE_MAX],
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
        self.player.start_combat()?;
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
        let victorious = !self.player.is_dead();
        self.player.end_combat()?;
        Ok(victorious)
    }

    fn combat_should_end(&self) -> bool {
        self.player.is_dead() || self.enemy_party.iter().all(|enemy| enemy.is_none())
    }

    fn conduct_player_turn(&mut self) -> Result<(), Error> {
        self.player.start_turn()?;
        loop {
            let enemy_statuses = self
                .enemy_party
                .iter()
                .map(|maybe_enemy| maybe_enemy.as_ref().map(EnemyStatus::from))
                .collect::<Vec<_>>();
            match self.player.choose_next_action(&enemy_statuses)? {
                CombatAction::PlayCard(_, card_details) => {
                    self.play_card(card_details)?;
                    if self.combat_should_end() {
                        return Ok(());
                    }
                    self.player.dispose_card_just_played()?;
                }
                CombatAction::PlayCardAgainstEnemy(_, card_details, enemy_index) => {
                    self.play_card_against_enemy(card_details, enemy_index)?;
                    if self.combat_should_end() {
                        return Ok(());
                    }
                    self.player.dispose_card_just_played()?;
                }
                CombatAction::Potion(_) => todo!(),
                CombatAction::EndTurn => break,
            }
        }
        self.player.end_turn()?;
        Ok(())
    }

    fn conduct_enemies_turn(&mut self) -> Result<(), Error> {
        for maybe_enemy in self.enemy_party.iter_mut() {
            if let Some(enemy) = maybe_enemy.as_mut() {
                let effect_chain = &enemy.next_action(&mut self.ai_rng).effect_chain;
                for effect in effect_chain {
                    // TODO: reactions
                    match effect {
                        EnemyEffect::AddToDiscardPile(cards) => {
                            self.player.add_to_discard_pile(cards)?;
                        }
                        EnemyEffect::Apply(condition) => {
                            self.player.apply(condition)?;
                        }
                        EnemyEffect::ApplyToSelf(enemy_condition) => {
                            enemy.apply(enemy_condition);
                        }
                        EnemyEffect::DealDamage(amount) => {
                            self.player.take_damage(Self::incoming_damage(
                                &self.player,
                                enemy,
                                *amount,
                            ))?;
                        }
                    }
                    if enemy.is_dead() {
                        *maybe_enemy = None;
                        break;
                    }
                    if self.player.is_dead() {
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
            println!("[EncounterSimulator] Applying effect: {:?}", effect);
            match effect {
                PlayerEffect::Apply(_) => unreachable!(
                    "Debuff should be handled by play_card_against_enemy, {:?}",
                    card_details
                ),
                PlayerEffect::ApplyToAll(condition) => {
                    self.apply_to_all_enemies(condition)?;
                }
                PlayerEffect::ApplyToSelf(player_condition) => {
                    self.player.apply(player_condition)?;
                }
                PlayerEffect::DealDamage(_) => unreachable!(
                    "DealDamage should be handled by play_card_against_enemy, {:?}",
                    card_details
                ),
                PlayerEffect::DealDamageToAll(amount) => {
                    self.attack_all_enemies(*amount)?;
                }
                PlayerEffect::GainBlock(amount) => {
                    self.player
                        .gain_block(Self::incoming_block(&self.player, *amount))?;
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
            enemy.take_damage(Self::outgoing_damage(&self.player, enemy, amount));
            let enemy_status = EnemyStatus::from(&*enemy);
            let enemy_type = enemy_status.enemy_type;
            self.player.send_enemy_status(index, enemy_status)?;
            if enemy.is_dead() {
                self.player.send_enemy_died(index, enemy_type)?;
                self.enemy_party[index] = None;
            }
        }
        Ok(())
    }

    fn attack_all_enemies(&mut self, amount: AttackDamage) -> Result<(), Error> {
        for index in 0..ENEMY_PARTY_SIZE_MAX {
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
            enemy.apply(condition);
            self.player
                .send_enemy_status(index, EnemyStatus::from(&*enemy))?;
        }
        Ok(())
    }

    fn apply_to_all_enemies(&mut self, condition: &EnemyCondition) -> Result<(), Error> {
        for index in 0..ENEMY_PARTY_SIZE_MAX {
            self.apply_to_enemy(index, condition)?;
        }
        Ok(())
    }

    fn incoming_block(player: &CombatController, amount: Block) -> Block {
        if player.is_frail() {
            (amount as f32 * 0.75).floor() as u32
        } else {
            amount
        }
    }

    fn incoming_damage(
        player: &CombatController,
        enemy: &EnemyState,
        amount: AttackDamage,
    ) -> AttackDamage {
        let enemy_modified_amount = if enemy.is_weak() {
            (amount as f32 * 0.75).floor() as u32
        } else {
            amount
        };
        if player.is_vulnerable() {
            (enemy_modified_amount as f32 * 1.5).floor() as u32
        } else {
            enemy_modified_amount
        }
    }

    fn outgoing_damage(
        player: &CombatController,
        enemy: &EnemyState,
        amount: AttackDamage,
    ) -> AttackDamage {
        let caster_modified_mount = if player.is_weak() {
            (amount as f32 * 0.75).floor() as u32
        } else {
            amount
        };
        if enemy.is_vulnerable() {
            (caster_modified_mount as f32 * 1.5).floor() as u32
        } else {
            caster_modified_mount
        }
    }
}
