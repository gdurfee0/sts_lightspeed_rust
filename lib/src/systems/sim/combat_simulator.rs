use anyhow::Error;

use crate::components::{CardCombatState, EnemyState, EnemyStatus, Interaction, PlayerCombatState};
use crate::data::{
    Card, CardDestination, CardPool, CardSelection, CostModifier, Damage, Encounter,
    EnemyCondition, EnemyEffect, PlayerCondition, PlayerEffect, Resource,
};
use crate::systems::enemy::{EnemyInCombat, EnemyPartyGenerator};
use crate::systems::player::{CombatAction, DamageSystem, PlayerController, PlayerInCombat};
use crate::systems::rng::{Seed, StsRandom};
use crate::types::{Block, EnemyIndex, Hp};
use crate::Notification;

pub struct CombatSimulator<'a> {
    seed_for_floor: Seed,
    encounter: Encounter,
    misc_rng: &'a mut StsRandom,
}

impl<'a> CombatSimulator<'a> {
    pub fn new(seed_for_floor: Seed, encounter: Encounter, misc_rng: &'a mut StsRandom) -> Self {
        let ai_rng = StsRandom::from(seed_for_floor);
        Self {
            seed_for_floor,
            encounter,
            ai_rng,
            misc_rng,
        }
    }

    pub fn run(mut self, player: &mut PlayerController) -> Result<bool, Error> {
        let mut enemy_party = EnemyPartyGenerator::new(
            self.seed_for_floor,
            self.encounter,
            &mut self.ai_rng,
            self.misc_rng,
        )
        .generate();
        let mut player_in_combat = PlayerInCombat::new(player, self.seed_for_floor);
        println!(
            "[EncounterSimulator] Running encounter: {:?}",
            self.encounter
        );
        player_in_combat.start_combat(
            &enemy_party
                .iter()
                .map(|maybe_enemy| maybe_enemy.as_ref().map(EnemyStatus::from))
                .collect::<Vec<_>>(),
        )?;
        loop {
            self.conduct_player_turn(&mut player_in_combat, &mut enemy_party)?;
            if Self::combat_should_end(&player_in_combat, &enemy_party) {
                break;
            }
            self.conduct_enemies_turn(&mut player_in_combat, &mut enemy_party)?;
            if Self::combat_should_end(&player_in_combat, &enemy_party) {
                break;
            }
        }
        let victorious = player_in_combat.player.state.hp > 0;
        player_in_combat.end_combat()?;
        Ok(victorious)
    }

    fn combat_should_end(
        player_in_combat: &PlayerInCombat,
        enemy_party: &[Option<EnemyInCombat>],
    ) -> bool {
        player_in_combat.player.state.hp == 0 || enemy_party.iter().all(|enemy| enemy.is_none())
    }

    fn conduct_player_turn(
        &mut self,
        player_in_combat: &mut PlayerInCombat,
        enemy_party: &mut [Option<EnemyInCombat>],
    ) -> Result<(), Error> {
        player_in_combat.start_turn()?;
        loop {
            let enemy_statuses = enemy_party
                .iter()
                .map(|maybe_enemy| maybe_enemy.as_ref().map(EnemyStatus::from))
                .collect::<Vec<_>>();
            match player_in_combat.choose_next_action(&enemy_statuses)? {
                CombatAction::PlayCard(card_details) => {
                    self.play_card(player_in_combat, enemy_party, card_details)?;
                    if Self::combat_should_end(player_in_combat, enemy_party) {
                        return Ok(());
                    }
                    player_in_combat.dispose_of_card_just_played()?;
                }
                CombatAction::PlayCardAgainstEnemy(card_details, enemy_index) => {
                    self.play_card_against_enemy(
                        player_in_combat,
                        enemy_party,
                        card_details,
                        enemy_index,
                    )?;
                    if Self::combat_should_end(player_in_combat, enemy_party) {
                        return Ok(());
                    }
                    player_in_combat.dispose_of_card_just_played()?;
                }
                CombatAction::EndTurn => break,
            }
        }
        player_in_combat.end_turn()
    }

    fn conduct_enemies_turn(
        &mut self,
        player_in_combat: &mut PlayerInCombat,
        enemy_party: &mut [Option<EnemyInCombat>],
    ) -> Result<(), Error> {
        for enemy in enemy_party.iter_mut().filter_map(|e| e.as_mut()) {
            enemy.start_turn();
        }
        for (enemy_index, maybe_enemy) in enemy_party.iter_mut().enumerate() {
            if let Some(enemy_in_combat) = maybe_enemy.as_mut() {
                if self.conduct_enemy_turn(player_in_combat, enemy_in_combat)? {
                    let enemy_status = EnemyStatus::from(&*enemy_in_combat);
                    player_in_combat
                        .player
                        .comms
                        .send_notification(Notification::EnemyStatus(enemy_index, enemy_status))?;
                } else {
                    player_in_combat
                        .player
                        .comms
                        .send_notification(Notification::EnemyDied(
                            enemy_index,
                            enemy_in_combat.state.enemy,
                        ))?;
                    *maybe_enemy = None;
                }
                if player_in_combat.player.state.hp == 0 {
                    break;
                }
            }
        }
        for enemy in enemy_party.iter_mut().filter_map(|e| e.as_mut()) {
            enemy.end_turn();
        }
        Ok(())
    }

    fn conduct_enemy_turn(
        &mut self,
        player_in_combat: &mut PlayerInCombat,
        enemy_in_combat: &mut EnemyInCombat,
    ) -> Result<bool, Error> {
        for effect in enemy_in_combat
            .next_action(&mut self.ai_rng)
            .effect_chain()
            .iter()
        {
            // TODO: reactions
            match effect {
                EnemyEffect::Apply(enemy_condition) => {
                    enemy_in_combat.apply_condition(enemy_condition);
                }
                EnemyEffect::CreateCards(
                    card_pool,
                    card_selection,
                    card_destination,
                    cost_modifier,
                ) => {
                    self.create_cards(card_pool, card_selection, card_destination, cost_modifier)?;
                }
                EnemyEffect::Deal(damage) => {
                    let incoming_damage = Self::incoming_damage(
                        &player_in_combat.state,
                        &enemy_in_combat.state,
                        damage,
                    );
                    DamageSystem::take_damage(
                        &mut player_in_combat.player.state,
                        &mut player_in_combat.state,
                        &player_in_combat.player.comms,
                        incoming_damage,
                    )?;
                }
                EnemyEffect::Gain(Resource::Block(amount)) => {
                    enemy_in_combat.state.block += *amount;
                }
                EnemyEffect::Gain(invalid) => unreachable!("{:?}", invalid),
                EnemyEffect::Inflict(condition) => {
                    player_in_combat.apply_condition(condition)?;
                }
            }
            if enemy_in_combat.state.is_dead() {
                return Ok(false);
            }
            if player_in_combat.player.state.hp == 0 {
                break;
            }
        }
        Ok(true)
    }

    fn create_cards(
        &mut self,
        card_pool: &CardPool,
        card_selection: &CardSelection,
        card_destination: &CardDestination,
        cost_modifier: &CostModifier,
    ) -> Result<(), Error> {
        Ok(())
    }

    // TODO: reactions
    fn play_card(
        &mut self,
        player_in_combat: &mut PlayerInCombat,
        enemy_party: &mut [Option<EnemyInCombat>],
        card: &CardCombatState,
    ) -> Result<(), Error> {
        for effect in card.details.on_play.iter() {
            match effect {
                PlayerEffect::Apply(player_condition) => todo!(),
                PlayerEffect::Conditional(player_effect_condition, player_effects) => todo!(),
                PlayerEffect::CreateCards(
                    card_pool,
                    card_selection,
                    card_destination,
                    cost_modifier,
                ) => todo!(),
                PlayerEffect::Draw(_) => todo!(),
                PlayerEffect::ForEachExhausted(player_effects) => todo!(),
                PlayerEffect::Gain(resource) => todo!(),
                PlayerEffect::ManipulateCards(
                    card_source,
                    card_selection,
                    card_destination,
                    cost_modifier,
                ) => todo!(),
                PlayerEffect::Offensive(target, target_effects) => todo!(),
                PlayerEffect::PlayThenExhaustTopCardOfDrawPile() => todo!(),
                PlayerEffect::RampUpCardDamage(_) => todo!(),
                PlayerEffect::TakeDamage(damage) => todo!(),
                PlayerEffect::Upgrade(card_source, card_selection) => todo!(),
                /*
                PlayerEffect::AddRandomCardToHandCostingZeroThisTurn(_) => todo!(),
                PlayerEffect::AddToDiscardPile(cards) => {
                    self.player.add_cards_to_discard_pile(cards)?;
                }
                PlayerEffect::AtEndOfTurn(_effect_chain) => todo!(),
                PlayerEffect::Apply(_) => unreachable!(
                    "Debuff should be handled by play_card_against_enemy, {:?}",
                    card
                ),
                PlayerEffect::ApplyToAll(enemy_condition) => {
                    self.apply_to_all_enemies(enemy_condition)?;
                }
                PlayerEffect::Apply(player_condition) => {
                    self.player.apply_condition(player_condition)?;
                }
                PlayerEffect::CloneAttackOrPowerCardIntoHand(_) => {
                    todo!();
                }
                PlayerEffect::CloneSelfIntoDiscardPile() => {
                    self.player.add_card_to_discard_pile(card)?;
                }
                PlayerEffect::DealDamageToAll(amount) => {
                    self.attack_all_enemies(*amount)?;
                }
                PlayerEffect::DealDamageToAllXTimes(amount) => {
                    let times = self.player.state.energy;
                    for _ in 0..times {
                        self.attack_all_enemies(*amount)?;
                    }
                }
                PlayerEffect::DealDamageToRandomEnemy(amount) => {
                    let enemy_index = self.pick_random_enemy();
                    if let Some(enemy_index) = enemy_index {
                        self.attack_enemy(enemy_index, *amount, 1)?;
                    }
                }
                PlayerEffect::Draw(count) => {
                    for _ in 0..*count {
                        self.player.draw_card()?;
                    }
                }
                PlayerEffect::ExhaustCardInHand() => todo!(),
                PlayerEffect::ExhaustCustom() => todo!(),
                PlayerEffect::ExhaustRandomCardInHand() => todo!(),
                PlayerEffect::GainBlock(amount) => {
                    self.player
                        .gain_block(Self::incoming_block(&self.player, *amount))?;
                }
                PlayerEffect::GainBlockCustom() => todo!(),
                PlayerEffect::GainEnergy(_) => todo!(),
                PlayerEffect::GainStrength(_) => todo!(),
                PlayerEffect::IfEnemyVulnerable(_) => todo!(),
                PlayerEffect::LoseHp(_) => todo!(),
                PlayerEffect::PlayThenExhaustTopCardOfDrawPile() => todo!(),
                PlayerEffect::PutCardFromDiscardPileOnTopOfDrawPile() => todo!(),
                PlayerEffect::PutCardFromHandOnTopOfDrawPile() => todo!(),
                PlayerEffect::UpgradeOneCardInHandThisCombat() => todo!(),
                PlayerEffect::UpgradeAllCardsInHandThisCombat() => todo!(),
                PlayerEffect::DealDamage(_)
                | PlayerEffect::DealDamageCustom()
                | PlayerEffect::DealDamageWithStrengthMultiplier(_, _)
                | PlayerEffect::RampUpCardDamage(_)
                | PlayerEffect::SapStrength(_) => unreachable!(
                    "DealDamage should be handled by play_card_against_enemy, {:?}",
                    card
                ),
                */
            }
            if Self::combat_should_end(player_in_combat, enemy_party) {
                break;
            }
        }
        Ok(())
    }

    fn pick_random_enemy(&mut self, enemy_party: &[Option<EnemyInCombat>]) -> Option<EnemyIndex> {
        let living_foes = enemy_party
            .iter()
            .enumerate()
            .filter(|(_, e)| e.is_some())
            .collect::<Vec<_>>();
        let alive_index = self.misc_rng.gen_range(0..living_foes.len());
        Some(living_foes[alive_index].0)
    }

    fn play_card_against_enemy(
        &mut self,
        card: &mut CardCombatState,
        enemy_index: EnemyIndex,
        misc_rng: &mut StsRandom,
    ) -> Result<(), Error> {
        for effect in card.details.effect_chain.iter() {
            match effect {
                PlayerEffect::Apply(condition) => {
                    self.apply_to_enemy(enemy_index, condition)?;
                }
                PlayerEffect::DealDamage(amount) => {
                    self.attack_enemy(enemy_index, *amount, 1)?;
                }
                PlayerEffect::DealDamageCustom() => match card.card {
                    Card::BodySlam(_) => {
                        self.attack_enemy(enemy_index, self.player.state.block, 1)?;
                    }
                    Card::PerfectedStrike(upgraded) => {
                        let per_strike_bonus = if upgraded { 3 } else { 2 };
                        let strike_count = self
                            .player
                            .state
                            .cards_iter()
                            .filter(|c| {
                                matches!(
                                    c.card,
                                    Card::Strike(_)
                                        | Card::MeteorStrike(_)
                                        | Card::PerfectedStrike(_)
                                        | Card::PommelStrike(_)
                                        | Card::SneakyStrike(_)
                                        | Card::ThunderStrike(_)
                                        | Card::TwinStrike(_)
                                        | Card::WildStrike(_)
                                        | Card::WindmillStrike(_)
                                )
                            })
                            .count();
                        let damage = 6 + (per_strike_bonus * strike_count) as AttackDamage;
                        self.attack_enemy(enemy_index, damage, 1)?;
                    }
                    _ => unreachable!("{:?}", card),
                },
                PlayerEffect::DealDamageWithStrengthMultiplier(amount, multiplier) => {
                    self.attack_enemy(enemy_index, *amount, *multiplier)?;
                }
                PlayerEffect::PutCardFromDiscardPileOnTopOfDrawPile() => {
                    self.player
                        .put_card_from_discard_pile_on_top_of_draw_pile()?;
                }
                PlayerEffect::RampUpCardDamage(amount) => {
                    card.additional_damage += *amount;
                }
                PlayerEffect::SapStrength(amount) => {
                    if let Some(enemy) = self.enemy_party[enemy_index].as_mut() {
                        enemy.state.strength -= *amount;

                        let enemy_status = EnemyStatus::from(&*enemy);
                        self.player
                            .player
                            .comms
                            .send_notification(Notification::EnemyStatus(
                                enemy_index,
                                enemy_status,
                            ))?;
                    }
                }
                effect => unreachable!(
                    "Inappropriate card handled by play_card_against_enemy, {:?} {:?}",
                    card, effect
                ),
            }
            if self.combat_should_end() {
                break;
            }
        }
        Ok(())
    }

    fn attack_enemy(&mut self, index: EnemyIndex, amount: Damage) -> Result<(), Error> {
        // TODO: reaction
        if let Some(enemy) = self.enemy_party[index].as_mut() {
            enemy.take_damage(Self::outgoing_damage(
                &self.player,
                enemy,
                amount,
                strength_multiplier,
            ));
            let enemy_status = EnemyStatus::from(&*enemy);
            let enemy_type = enemy_status.enemy_type;
            self.player
                .player
                .comms
                .send_notification(Notification::EnemyStatus(index, enemy_status))?;
            if enemy.state.is_dead() {
                self.player
                    .player
                    .comms
                    .send_notification(Notification::EnemyDied(index, enemy_type))?;
                // Invoke death effects
                for condition in enemy.state.conditions.iter() {
                    if let EnemyCondition::SporeCloud(stacks) = condition {
                        self.player
                            .apply_condition(&PlayerCondition::Vulnerable(*stacks))?;
                    }
                }

                self.enemy_party[index] = None;
            }
        }
        Ok(())
    }

    fn attack_all_enemies(&mut self, damage: Damage) -> Result<(), Error> {
        for index in 0..5 {
            self.attack_enemy(index, damage)?;
        }
        Ok(())
    }

    fn apply_to_enemy(
        &mut self,
        index: EnemyIndex,
        condition: &EnemyCondition,
        comms: &Interaction,
    ) -> Result<(), Error> {
        if let Some(enemy) = self.enemy_party[index].as_mut() {
            enemy.apply_condition(condition);
            let enemy_status = EnemyStatus::from(&*enemy);
            comms.send_notification(Notification::EnemyStatus(index, enemy_status))?;
        }
        Ok(())
    }

    fn apply_to_all_enemies(
        &mut self,
        condition: &EnemyCondition,
        comms: &Interaction,
    ) -> Result<(), Error> {
        for index in 0..5 {
            self.apply_to_enemy(index, condition, comms)?;
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
        player_state: &PlayerCombatState,
        enemy_state: &EnemyState,
        damage: &Damage,
    ) -> Damage {
        let amount = match damage {
            Damage::Blockable(amount) => amount.saturating_add_signed(enemy_state.strength),
            Damage::BlockableCountingStrikeCards(_, _) => unreachable!(),
            Damage::BlockableEqualToDrawPileSize => unreachable!(),
            Damage::BlockableEqualToPlayerBlock => unreachable!(),
            Damage::BlockableWithStrengthMultiplier(_, _) => unreachable!(),
            Damage::HpLoss(_) => unreachable!(),
            Damage::HpLossEqualToHandSize => unreachable!(),
        };
        let enemy_modified_amount = if enemy_state.is_weak() {
            (amount as f32 * 0.75).floor() as u32
        } else {
            amount
        };
        if player_state.is_vulnerable() {
            Damage::Blockable((enemy_modified_amount as f32 * 1.5).floor() as Hp)
        } else {
            Damage::Blockable(enemy_modified_amount)
        }
    }

    fn outgoing_damage(
        player_state: &PlayerCombatState,
        enemy_state: &EnemyState,
        damage: &Damage,
    ) -> Hp {
        let amount = match damage {
            Damage::Blockable(amount) => amount.saturating_add_signed(player_state.strength),
            Damage::BlockableCountingStrikeCards(base_amount, per_strike_bonus) => {
                let strike_count = player_state
                    .cards_iter()
                    .filter(|c| {
                        matches!(
                            c.card,
                            Card::Strike(_)
                                | Card::MeteorStrike(_)
                                | Card::PerfectedStrike(_)
                                | Card::PommelStrike(_)
                                | Card::SneakyStrike(_)
                                | Card::ThunderStrike(_)
                                | Card::TwinStrike(_)
                                | Card::WildStrike(_)
                                | Card::WindmillStrike(_)
                        )
                    })
                    .count() as Hp;
                (base_amount + strike_count * per_strike_bonus)
                    .saturating_add_signed(player_state.strength)
            }
            Damage::BlockableEqualToDrawPileSize => {
                (player_state.draw_pile.len() as Hp).saturating_add_signed(player_state.strength)
            }
            Damage::BlockableEqualToPlayerBlock => {
                (player_state.block as Hp).saturating_add_signed(player_state.strength)
            }
            Damage::BlockableWithStrengthMultiplier(base_amount, strength_multiplier) => {
                base_amount.saturating_add_signed(player_state.strength * strength_multiplier)
            }
            Damage::HpLoss(_) => unreachable!(),
            Damage::HpLossEqualToHandSize => unreachable!(),
        };
        let caster_modified_mount = if player_state.is_weak() {
            (amount as f32 * 0.75).floor() as u32
        } else {
            amount
        };
        if enemy_state.is_vulnerable() {
            (caster_modified_mount as f32 * 1.5).floor() as Hp
        } else {
            caster_modified_mount
        }
    }
}
