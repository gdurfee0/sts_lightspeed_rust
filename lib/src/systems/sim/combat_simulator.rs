use anyhow::Error;

use crate::components::{CardInCombat, EnemyStatus};
use crate::data::{Card, Encounter, EnemyCondition, EnemyEffect, PlayerCondition, PlayerEffect};
use crate::systems::enemy::{EnemyInCombat, EnemyPartyGenerator};
use crate::systems::player::{CombatAction, Player, PlayerInCombat};
use crate::systems::rng::{Seed, StsRandom};
use crate::types::{AttackDamage, Block, EnemyIndex, Strength};
use crate::Notification;

pub struct CombatSimulator<'a> {
    encounter: Encounter,
    seed_for_floor: Seed,
    ai_rng: StsRandom,
    misc_rng: &'a mut StsRandom,
    player: PlayerInCombat<'a>,
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
            player: player_in_combat,
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
        self.player.start_combat(
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
        let victorious = self.player.player.state.hp > 0;
        self.player.end_combat()?;
        Ok(victorious)
    }

    fn combat_should_end(&self) -> bool {
        self.player.player.state.hp == 0 || self.enemy_party.iter().all(|enemy| enemy.is_none())
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
                CombatAction::PlayCard(card_details) => {
                    self.play_card(&card_details)?;
                    if self.combat_should_end() {
                        return Ok(());
                    }
                    self.player.dispose_of_card_just_played()?;
                }
                CombatAction::PlayCardAgainstEnemy(card_details, enemy_index) => {
                    self.play_card_against_enemy(card_details, enemy_index)?;
                    if self.combat_should_end() {
                        return Ok(());
                    }
                    self.player.dispose_of_card_just_played()?;
                }
                CombatAction::EndTurn => break,
            }
        }
        self.player.end_turn()?;
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
                            self.player.add_cards_to_discard_pile(cards)?;
                        }
                        EnemyEffect::Apply(condition) => {
                            self.player.apply_condition(condition)?;
                        }
                        EnemyEffect::ApplyToSelf(enemy_condition) => {
                            enemy.apply_condition(enemy_condition);
                        }
                        EnemyEffect::DealDamage(amount) => {
                            self.player.take_blockable_damage(Self::incoming_damage(
                                &self.player,
                                enemy,
                                *amount,
                            ))?;
                        }
                        EnemyEffect::GainBlock(amount) => enemy.state.block += *amount,
                        EnemyEffect::GainStrength(amount) => enemy.state.strength += *amount,
                    }
                    let enemy_status = EnemyStatus::from(&*enemy);
                    self.player
                        .player
                        .comms
                        .send_notification(Notification::EnemyStatus(enemy_index, enemy_status))?;
                    if enemy.state.is_dead() {
                        *maybe_enemy = None;
                        break;
                    }
                    if self.player.player.state.hp == 0 {
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
                PlayerEffect::AddToDiscardPile(cards) => {
                    self.player.add_cards_to_discard_pile(cards)?;
                }
                PlayerEffect::ApplyToSelfAtEndOfTurn(_) => todo!(),
                PlayerEffect::Apply(_) => unreachable!(
                    "Debuff should be handled by play_card_against_enemy, {:?}",
                    card
                ),
                PlayerEffect::ApplyToAll(enemy_condition) => {
                    self.apply_to_all_enemies(enemy_condition)?;
                }
                PlayerEffect::ApplyToSelf(player_condition) => {
                    self.player.apply_condition(player_condition)?;
                }
                PlayerEffect::CloneAttackOrPowerCardIntoHand(_) => {
                    todo!();
                }
                PlayerEffect::CloneSelfIntoDiscardPile() => {
                    self.player.add_card_to_discard_pile(card)?;
                    todo!();
                }
                PlayerEffect::DealDamageToAll(amount) => {
                    self.attack_all_enemies(*amount)?;
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
                | PlayerEffect::DealDamageWithStrengthMultiplier(_, _) => unreachable!(
                    "DealDamage should be handled by play_card_against_enemy, {:?}",
                    card
                ),
            }
            if self.combat_should_end() {
                break;
            }
        }
        Ok(())
    }

    fn pick_random_enemy(&mut self) -> Option<EnemyIndex> {
        let living_foes = self
            .enemy_party
            .iter()
            .enumerate()
            .filter(|(_, e)| e.is_some())
            .collect::<Vec<_>>();
        let alive_index = self.misc_rng.gen_range(0..living_foes.len());
        Some(living_foes[alive_index].0)
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

    fn attack_enemy(
        &mut self,
        index: EnemyIndex,
        amount: AttackDamage,
        strength_multiplier: Strength,
    ) -> Result<(), Error> {
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

    fn attack_all_enemies(&mut self, amount: AttackDamage) -> Result<(), Error> {
        for index in 0..5 {
            self.attack_enemy(index, amount, 1)?;
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
            self.player
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
        strength_multiplier: Strength,
    ) -> AttackDamage {
        let amount = amount + (player.state.strength * strength_multiplier) as AttackDamage;
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
        /*
        let x = [
            PlayCardFromHand(0, Bash(false), 1),
            TargetEnemy(0, GremlinNob),
            PlayCardFromHand(1, Thunderclap(false), 1),
            PlayCardFromHand(0, Defend(false), 1),
            EndTurn,
            PlayCardFromHand(4, Defend(false), 1),
            PlayCardFromHand(4, Defend(false), 0),
            PlayCardFromHand(0, Strike(false), 0),
            TargetEnemy(0, GremlinNob),
            PlayCardFromHand(2, InfernalBlade(false), 0),
        ];
        */
    }
}
