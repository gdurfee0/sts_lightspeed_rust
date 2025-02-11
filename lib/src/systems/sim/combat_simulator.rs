use anyhow::Error;

use crate::components::{
    Effect, EffectQueue, Interaction, PlayerCombatState, PlayerPersistentState,
};
use crate::data::{Encounter, EnemyEffect, PlayerEffect, Resource};
use crate::systems::combat::{
    BlockSystem, DamageCalculator, EnemyConditionSystem, PlayerConditionSystem,
};
use crate::systems::enemy::{EnemyParty, EnemySystem};
use crate::systems::player::{CombatAction, PlayerCombatSystem};
use crate::systems::rng::{Seed, StsRandom};
use crate::types::EnemyIndex;

pub struct CombatSimulator<'a> {
    seed_for_floor: Seed,
    misc_rng: &'a mut StsRandom,
    player_combat_system: PlayerCombatSystem,
    enemy_system: EnemySystem,
}

impl<'a> CombatSimulator<'a> {
    /// Creates a new combat simulator.
    pub fn new(seed_for_floor: Seed, misc_rng: &'a mut StsRandom) -> Self {
        Self {
            seed_for_floor,
            misc_rng,
            player_combat_system: PlayerCombatSystem::new(seed_for_floor),
            enemy_system: EnemySystem::new(seed_for_floor),
        }
    }

    /// Runs a combat encounter, returning true if the player wins.
    pub fn run_encounter<I: Interaction>(
        mut self,
        comms: &I,
        encounter: Encounter,
        pps: &mut PlayerPersistentState,
    ) -> Result<bool, Error> {
        println!("[CombatSimulator] Running encounter: {:?}", encounter);
        let mut enemy_party = self
            .enemy_system
            .create_enemy_party(encounter, self.misc_rng);
        let mut pcs = PlayerCombatState::new(pps);
        self.player_combat_system
            .start_combat(comms, pps, &mut pcs, &mut enemy_party)?;
        loop {
            self.conduct_player_turn(comms, pps, &mut pcs, &mut enemy_party)?;
            if Self::combat_should_end(pps, &enemy_party) {
                break;
            }
            self.conduct_enemies_turn(comms, pps, &mut pcs, &mut enemy_party)?;
            if Self::combat_should_end(pps, &enemy_party) {
                break;
            }
        }
        self.player_combat_system.end_combat(comms)?;
        let victorious = pps.hp > 0;
        Ok(victorious)
    }

    /// Conducts the player's turn.
    fn conduct_player_turn<I: Interaction>(
        &mut self,
        comms: &I,
        pps: &mut PlayerPersistentState,
        pcs: &mut PlayerCombatState,
        enemy_party: &mut EnemyParty,
    ) -> Result<(), Error> {
        let mut effect_queue = EffectQueue::new();
        self.player_combat_system
            .start_turn(comms, pps, pcs, &mut effect_queue)?;
        while let Some(effect) = effect_queue.pop_front() {
            self.process_effect(
                comms,
                pps,
                pcs,
                enemy_party,
                effect,
                None,
                &mut effect_queue,
            )?;
            if Self::combat_should_end(pps, enemy_party) {
                break;
            }
        }
        while !Self::combat_should_end(pps, enemy_party) {
            match self
                .player_combat_system
                .choose_next_action(comms, pps, pcs, enemy_party)?
            {
                CombatAction::PlayCard(combat_card, maybe_enemy_index) => {
                    for effect in combat_card.details.on_play.iter() {
                        effect_queue.push_back(Effect::FromCard(effect));
                    }
                    while let Some(effect) = effect_queue.pop_front() {
                        self.process_effect(
                            comms,
                            pps,
                            pcs,
                            enemy_party,
                            effect,
                            maybe_enemy_index,
                            &mut effect_queue,
                        )?;
                        if Self::combat_should_end(pps, enemy_party) {
                            break;
                        }
                    }
                }
                CombatAction::EndTurn => break,
            };
        }
        if !Self::combat_should_end(pps, enemy_party) {
            self.player_combat_system
                .end_turn(comms, pps, pcs, &mut effect_queue)?;
            while let Some(effect) = effect_queue.pop_front() {
                self.process_effect(
                    comms,
                    pps,
                    pcs,
                    enemy_party,
                    effect,
                    None,
                    &mut effect_queue,
                )?;
                if Self::combat_should_end(pps, enemy_party) {
                    break;
                }
            }
        }
        Ok(())
    }

    /// Conducts the enemies' turn.
    fn conduct_enemies_turn<I: Interaction>(
        &mut self,
        comms: &I,
        pps: &mut PlayerPersistentState,
        pcs: &mut PlayerCombatState,
        enemy_party: &mut EnemyParty,
    ) -> Result<(), Error> {
        let mut effect_queue = EffectQueue::new();
        self.enemy_system.start_turn(enemy_party);
        for enemy_index in 0..enemy_party.0.len() {
            if let Some(enemy_action) = enemy_party.0[enemy_index].as_mut().map(|e| e.next_action) {
                for effect in enemy_action.effect_chain().iter() {
                    effect_queue.push_back(Effect::FromEnemyPlaybook(effect));
                }
                while let Some(effect) = effect_queue.pop_front() {
                    self.process_effect(
                        comms,
                        pps,
                        pcs,
                        enemy_party,
                        effect,
                        Some(enemy_index),
                        &mut effect_queue,
                    )?;
                    if Self::combat_should_end(pps, enemy_party)
                        || enemy_party.0[enemy_index].is_none()
                    {
                        break;
                    }
                }
            }
            if Self::combat_should_end(pps, enemy_party) {
                break;
            }
        }
        if !Self::combat_should_end(pps, enemy_party) {
            self.enemy_system.end_turn(enemy_party);
        }
        Ok(())
    }

    /// Returns true if the combat should end.
    fn combat_should_end(pps: &PlayerPersistentState, enemy_party: &EnemyParty) -> bool {
        pps.hp == 0 || enemy_party.0.iter().all(|enemy| enemy.is_none())
    }

    /// Handles the incoming effect (PlayerEffect or EnemyEffect).
    fn process_effect<I: Interaction>(
        &mut self,
        comms: &I,
        pps: &mut PlayerPersistentState,
        pcs: &mut PlayerCombatState,
        enemy_party: &mut EnemyParty,
        effect: Effect,
        maybe_enemy_index: Option<EnemyIndex>,
        effect_queue: &mut EffectQueue,
    ) -> Result<(), Error> {
        match effect {
            Effect::FromCard(player_effect) => self.process_player_effect(
                comms,
                pps,
                pcs,
                enemy_party,
                player_effect,
                maybe_enemy_index,
                effect_queue,
            ),
            Effect::FromEnemyPlaybook(enemy_effect) => self.process_enemy_effect(
                comms,
                pps,
                pcs,
                enemy_party,
                enemy_effect,
                maybe_enemy_index,
                effect_queue,
            ),
            Effect::FromEnemyState(enemy_effect) => self.process_enemy_effect(
                comms,
                pps,
                pcs,
                enemy_party,
                &enemy_effect,
                maybe_enemy_index,
                effect_queue,
            ),
            Effect::FromPlayerState(player_effect) => self.process_player_effect(
                comms,
                pps,
                pcs,
                enemy_party,
                &player_effect,
                maybe_enemy_index,
                effect_queue,
            ),
        }
    }

    fn process_player_effect<I: Interaction>(
        &mut self,
        comms: &I,
        pps: &mut PlayerPersistentState,
        pcs: &mut PlayerCombatState,
        enemy_party: &mut EnemyParty,
        effect: &PlayerEffect,
        maybe_enemy_index: Option<EnemyIndex>,
        effect_queue: &mut EffectQueue,
    ) -> Result<(), Error> {
        match effect {
            PlayerEffect::Apply(player_condition) => {
                PlayerConditionSystem::apply_to_player(comms, pcs, player_condition)
            }
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
            PlayerEffect::Lose(resource) => todo!(),
            PlayerEffect::ManipulateCards(
                card_source,
                card_selection,
                card_destination,
                cost_modifier,
            ) => todo!(),
            PlayerEffect::PlayThenExhaustTopCardOfDrawPile => todo!(),
            PlayerEffect::RampUpCardDamage(_) => todo!(),
            PlayerEffect::TakeDamage(damage) => todo!(),
            PlayerEffect::ToAllEnemies(target_effect) => todo!(),
            PlayerEffect::ToRandomEnemy(target_effect) => todo!(),
            PlayerEffect::ToSingleTarget(target_effect) => todo!(),
            PlayerEffect::Upgrade(card_source, card_selection) => todo!(),
        }
    }

    fn process_enemy_effect<I: Interaction>(
        &mut self,
        comms: &I,
        pps: &mut PlayerPersistentState,
        pcs: &mut PlayerCombatState,
        enemy_party: &mut EnemyParty,
        effect: &EnemyEffect,
        maybe_enemy_index: Option<EnemyIndex>,
        effect_queue: &mut EffectQueue,
    ) -> Result<(), Error> {
        if let Some(enemy_state) = maybe_enemy_index
            .and_then(|i| enemy_party.0.get_mut(i))
            .and_then(|maybe_enemy| maybe_enemy.as_mut())
        {
            match effect {
                EnemyEffect::Apply(enemy_condition) => {
                    EnemyConditionSystem::apply_to_enemy(enemy_state, enemy_condition);
                }
                EnemyEffect::CreateCards(
                    card_pool,
                    card_selection,
                    card_destination,
                    cost_modifier,
                ) => todo!(),
                EnemyEffect::Deal(damage) => {
                    let damage = DamageCalculator::calculate_damage_inflicted(
                        enemy_state,
                        Some(pcs),
                        damage,
                    );
                    BlockSystem::damage_player(comms, pps, pcs, damage, effect_queue)?;
                }
                EnemyEffect::Gain(Resource::Block(block)) => {
                    enemy_state.block += block;
                }
                EnemyEffect::Gain(Resource::Strength(strength)) => {
                    enemy_state.strength += strength;
                }
                EnemyEffect::Gain(invalid) => unreachable!("{:?}", invalid),
                EnemyEffect::Inflict(player_condition) => {
                    PlayerConditionSystem::apply_to_player(comms, pcs, player_condition)?;
                }
            }
        }
        Ok(())
    }

    /*
    // TODO: reactions

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
    */
}
