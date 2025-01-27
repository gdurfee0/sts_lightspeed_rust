use anyhow::Error;

use crate::components::{CardInCombat, EnemyStatus};
use crate::data::{Card, Encounter, EnemyCondition, EnemyEffect, PlayerCondition, PlayerEffect};
use crate::systems::enemy::{EnemyInCombat, EnemyPartyGenerator};
use crate::systems::player::{CombatAction, Player, PlayerInCombat};
use crate::systems::rng::{Seed, StsRandom};
use crate::types::{AttackDamage, Block, EnemyIndex};
use crate::Notification;

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
        let player_in_combat = PlayerInCombat::new(player, seed_for_floor);
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
        EnemyPartyGenerator::new(
            self.seed_for_floor,
            self.encounter,
            &mut self.ai_rng,
            self.misc_rng,
        )
        .generate(&mut self.enemy_party);
        self.player_in_combat.start_combat(
            &self
                .enemy_party
                .iter()
                .map(|maybe_enemy| maybe_enemy.as_ref().map(EnemyStatus::from))
                .collect::<Vec<_>>(),
        )?;
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
        let victorious = self.player_in_combat.player.state.hp > 0;
        self.player_in_combat.end_combat()?;
        Ok(victorious)
    }

    fn combat_should_end(&self) -> bool {
        self.player_in_combat.player.state.hp == 0
            || self.enemy_party.iter().all(|enemy| enemy.is_none())
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
                    self.play_card(&card_details)?;
                    if self.combat_should_end() {
                        return Ok(());
                    }
                    self.player_in_combat.dispose_of_card_just_played()?;
                }
                CombatAction::PlayCardAgainstEnemy(card_details, enemy_index) => {
                    self.play_card_against_enemy(card_details, enemy_index)?;
                    if self.combat_should_end() {
                        return Ok(());
                    }
                    self.player_in_combat.dispose_of_card_just_played()?;
                }
                CombatAction::EndTurn => break,
            }
        }
        self.player_in_combat.end_turn()?;
        Ok(())
    }

    fn conduct_enemies_turn(&mut self) -> Result<(), Error> {
        for enemy in self.enemy_party.iter_mut().filter_map(|e| e.as_mut()) {
            enemy.start_turn();
        }
        for (enemy_index, maybe_enemy) in self.enemy_party.iter_mut().enumerate() {
            if let Some(enemy) = maybe_enemy.as_mut() {
                for effect in enemy.next_action(&mut self.ai_rng).effect_chain().iter() {
                    // TODO: reactions
                    match effect {
                        EnemyEffect::AddToDiscardPile(cards) => {
                            self.player_in_combat.add_cards_to_discard_pile(cards)?;
                        }
                        EnemyEffect::Apply(condition) => {
                            self.player_in_combat.apply_condition(condition)?;
                        }
                        EnemyEffect::ApplyToSelf(enemy_condition) => {
                            enemy.apply_condition(enemy_condition);
                        }
                        EnemyEffect::DealDamage(amount) => {
                            self.player_in_combat
                                .take_blockable_damage(Self::incoming_damage(
                                    &self.player_in_combat,
                                    enemy,
                                    *amount,
                                ))?;
                        }
                        EnemyEffect::GainBlock(amount) => enemy.state.block += *amount,
                        EnemyEffect::GainStrength(amount) => enemy.state.strength += *amount,
                    }
                    let enemy_status = EnemyStatus::from(&*enemy);
                    self.player_in_combat
                        .player
                        .comms
                        .send_notification(Notification::EnemyStatus(enemy_index, enemy_status))?;
                    if enemy.state.is_dead() {
                        *maybe_enemy = None;
                        break;
                    }
                    if self.player_in_combat.player.state.hp == 0 {
                        break;
                    }
                }
            }
        }
        for enemy in self.enemy_party.iter_mut().filter_map(|e| e.as_mut()) {
            enemy.end_turn();
        }
        Ok(())
    }

    // TODO: reactions
    fn play_card(&mut self, card: &CardInCombat) -> Result<(), Error> {
        for effect in card.details.effect_chain.iter() {
            match effect {
                PlayerEffect::AddRandomCardThatCostsZeroThisTurnToHand(_) => todo!(),
                PlayerEffect::AddSelfCopyToDiscardPile() => {
                    self.player_in_combat.add_card_to_discard_pile(card)?;
                    todo!();
                }
                PlayerEffect::Apply(_) => unreachable!(
                    "Debuff should be handled by play_card_against_enemy, {:?}",
                    card
                ),
                PlayerEffect::ApplyToAll(enemy_condition) => {
                    self.apply_to_all_enemies(enemy_condition)?;
                }
                PlayerEffect::ApplyToSelf(player_condition) => {
                    self.player_in_combat.apply_condition(player_condition)?;
                }
                PlayerEffect::DealDamage(_) | PlayerEffect::DealDamageCustom() => unreachable!(
                    "DealDamage should be handled by play_card_against_enemy, {:?}",
                    card
                ),
                PlayerEffect::DealDamageToAll(amount) => {
                    self.attack_all_enemies(*amount)?;
                }
                PlayerEffect::GainBlock(amount) => {
                    self.player_in_combat
                        .gain_block(Self::incoming_block(&self.player_in_combat, *amount))?;
                }
                PlayerEffect::UpgradeOneCardInHandThisCombat() => todo!(),
                PlayerEffect::UpgradeAllCardsInHandThisCombat() => todo!(),
            }
            if self.combat_should_end() {
                break;
            }
        }
        Ok(())
    }

    fn play_card_against_enemy(
        &mut self,
        card: CardInCombat,
        enemy_index: EnemyIndex,
    ) -> Result<(), Error> {
        for effect in card.details.effect_chain.iter() {
            match effect {
                PlayerEffect::Apply(condition) => {
                    self.apply_to_enemy(enemy_index, condition)?;
                }
                PlayerEffect::DealDamage(amount) => {
                    self.attack_enemy(enemy_index, *amount)?;
                }
                PlayerEffect::DealDamageCustom() => match card.card {
                    Card::BodySlam(_) => {
                        self.attack_enemy(enemy_index, self.player_in_combat.state.block)?;
                    }
                    _ => unreachable!("{:?}", card),
                },
                PlayerEffect::AddRandomCardThatCostsZeroThisTurnToHand(_)
                | PlayerEffect::AddSelfCopyToDiscardPile()
                | PlayerEffect::ApplyToAll(_)
                | PlayerEffect::ApplyToSelf(_)
                | PlayerEffect::DealDamageToAll(_)
                | PlayerEffect::GainBlock(_)
                | PlayerEffect::UpgradeOneCardInHandThisCombat()
                | PlayerEffect::UpgradeAllCardsInHandThisCombat() => unreachable!(
                    "Inappropriate card handled by play_card_against_enemy, {:?}",
                    card
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
                .player
                .comms
                .send_notification(Notification::EnemyStatus(index, enemy_status))?;
            if enemy.state.is_dead() {
                self.player_in_combat
                    .player
                    .comms
                    .send_notification(Notification::EnemyDied(index, enemy_type))?;
                // Invoke death effects
                for condition in enemy.state.conditions.iter() {
                    if let EnemyCondition::SporeCloud(stacks) = condition {
                        self.player_in_combat
                            .apply_condition(&PlayerCondition::Vulnerable(*stacks))?;
                    }
                }

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
            let enemy_status = EnemyStatus::from(&*enemy);
            self.player_in_combat
                .player
                .comms
                .send_notification(Notification::EnemyStatus(index, enemy_status))?;
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
        let amount = amount.saturating_add_signed(player.state.dexterity);
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
