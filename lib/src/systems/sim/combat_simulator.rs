use anyhow::Error;

use crate::components::{
    Effect, EffectQueue, Interaction, Notification, PlayerCombatState, PlayerPersistentState,
};
use crate::data::{Encounter, EnemyEffect, PlayerEffect, Resource, TargetEffect};
use crate::systems::combat::{
    BlockSystem, CardCreationSystem, DamageCalculator, EnemyConditionSystem, PlayerConditionSystem,
};
use crate::systems::enemy::{EnemyParty, EnemySystem};
use crate::systems::player::{CombatAction, PlayerCombatSystem};
use crate::systems::rng::{Seed, StsRandom};
use crate::types::EnemyIndex;

pub struct CombatSimulator<'a> {
    misc_rng: &'a mut StsRandom,
    player_combat_system: PlayerCombatSystem,
    enemy_system: EnemySystem,
}

#[derive(Debug)]
struct CombatContext<'a, I: Interaction> {
    pub comms: &'a I,
    pub pcs: &'a mut PlayerCombatState<'a>,
    pub enemy_party: &'a mut EnemyParty,
    pub maybe_enemy_index: Option<EnemyIndex>,
}

impl<'a> CombatSimulator<'a> {
    /// Creates a new combat simulator.
    pub fn new(seed_for_floor: Seed, misc_rng: &'a mut StsRandom) -> Self {
        Self {
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
        let mut pcs = PlayerCombatState::new(pps);
        let mut ctx = CombatContext {
            comms,
            pcs: &mut pcs,
            enemy_party: &mut self
                .enemy_system
                .create_enemy_party(encounter, self.misc_rng),
            maybe_enemy_index: None,
        };
        self.player_combat_system
            .start_combat(comms, ctx.pcs, ctx.enemy_party)?;
        loop {
            ctx.maybe_enemy_index = None;
            self.conduct_player_turn(&mut ctx)?;
            if Self::combat_should_end(&ctx) {
                break;
            }
            self.conduct_enemies_turn(&mut ctx)?;
            if Self::combat_should_end(&ctx) {
                break;
            }
        }
        self.player_combat_system.end_combat(comms, pps)?;
        let victorious = pps.hp > 0;
        Ok(victorious)
    }

    /// Conducts the player's turn.
    fn conduct_player_turn<I: Interaction>(
        &mut self,
        ctx: &mut CombatContext<I>,
    ) -> Result<(), Error> {
        let mut effect_queue = EffectQueue::new();
        self.player_combat_system
            .start_turn(ctx.comms, ctx.pcs, &mut effect_queue)?;
        self.process_effect_queue(ctx, &mut effect_queue)?;
        while !Self::combat_should_end(ctx) {
            match self.player_combat_system.choose_next_action(
                ctx.comms,
                ctx.pcs,
                ctx.enemy_party,
            )? {
                CombatAction::PlayCard(combat_card, maybe_enemy_index) => {
                    ctx.maybe_enemy_index = maybe_enemy_index;
                    println!("Disposing of card just played: {:?}", combat_card);
                    self.player_combat_system.dispose_of_card_just_played(
                        ctx.comms,
                        ctx.pcs,
                        &mut effect_queue,
                    )?;
                    self.process_effect_queue(ctx, &mut effect_queue)?;
                    for effect in combat_card.details.on_play.iter() {
                        effect_queue.push_back(Effect::Card(effect));
                    }
                    println!("Hand is now {:?}", ctx.pcs.cards.hand);
                    self.process_effect_queue(ctx, &mut effect_queue)?;
                }
                CombatAction::EndTurn => break,
            };
        }
        if !Self::combat_should_end(ctx) {
            self.player_combat_system
                .end_turn(ctx.comms, ctx.pcs, &mut effect_queue)?;
            self.process_effect_queue(ctx, &mut effect_queue)?;
        }
        Ok(())
    }

    /// Conducts the enemies' turn.
    fn conduct_enemies_turn<I: Interaction>(
        &mut self,
        ctx: &mut CombatContext<I>,
    ) -> Result<(), Error> {
        let mut effect_queue = EffectQueue::new();
        self.enemy_system.start_turn(ctx.enemy_party);
        for enemy_index in 0..ctx.enemy_party.0.len() {
            ctx.maybe_enemy_index = Some(enemy_index);
            if let Some(enemy_action) = ctx.enemy_party.0[enemy_index]
                .as_mut()
                .map(|e| e.next_action)
            {
                println!(
                    "[CombatSimulator] Enemy {} action: {:?}",
                    enemy_index, enemy_action
                );
                for effect in enemy_action.effect_chain().iter() {
                    effect_queue.push_back(Effect::EnemyPlaybook(effect));
                }
                while let Some(effect) = effect_queue.pop_front() {
                    if self.process_effect(ctx, effect, &mut effect_queue)?
                        || ctx.enemy_party.0[enemy_index].is_none()
                    {
                        break;
                    }
                }
            }
            if Self::combat_should_end(ctx) {
                break;
            }
        }
        if !Self::combat_should_end(ctx) {
            self.enemy_system.end_turn(ctx.enemy_party);
        }
        Ok(())
    }

    /// Processes all effects in the queue.
    fn process_effect_queue<I: Interaction>(
        &mut self,
        ctx: &mut CombatContext<I>,
        effect_queue: &mut EffectQueue,
    ) -> Result<(), Error> {
        while let Some(effect) = effect_queue.pop_front() {
            if self.process_effect(ctx, effect, effect_queue)? {
                break;
            }
        }
        Ok(())
    }

    /// Handles the incoming effect (PlayerEffect or EnemyEffect).
    fn process_effect<I: Interaction>(
        &mut self,
        ctx: &mut CombatContext<I>,
        effect: Effect,
        effect_queue: &mut EffectQueue,
    ) -> Result<bool, Error> {
        match effect {
            Effect::Card(player_effect) => {
                self.process_player_effect(ctx, player_effect, effect_queue)?;
            }
            Effect::EnemyPlaybook(enemy_effect) => {
                self.process_enemy_effect(ctx, enemy_effect, effect_queue)?;
            }
            Effect::EnemyState(enemy_effect) => {
                self.process_enemy_effect(ctx, &enemy_effect, effect_queue)?;
            }
            Effect::PlayerState(player_effect) => {
                self.process_player_effect(ctx, &player_effect, effect_queue)?;
            }
        }
        if let Some(enemy_index) = ctx.maybe_enemy_index {
            let should_remove_enemy =
                if let Some(enemy_state) = ctx.enemy_party.0[enemy_index].as_mut() {
                    if enemy_state.hp == 0 {
                        EnemyConditionSystem::on_enemy_death(enemy_state, effect_queue);
                        ctx.comms.send_notification(Notification::EnemyDied(
                            enemy_index,
                            enemy_state.enemy,
                        ))?;
                        true
                    } else {
                        false
                    }
                } else {
                    false
                };
            if should_remove_enemy {
                ctx.enemy_party.0[enemy_index] = None;
            }
        }
        Ok(Self::combat_should_end(ctx))
    }

    fn process_player_effect<I: Interaction>(
        &mut self,
        ctx: &mut CombatContext<I>,
        effect: &PlayerEffect,
        effect_queue: &mut EffectQueue,
    ) -> Result<(), Error> {
        match effect {
            PlayerEffect::Apply(player_condition) => {
                PlayerConditionSystem::apply_to_player(ctx.comms, ctx.pcs, player_condition)
            }
            PlayerEffect::Conditional(_player_effect_condition, _player_effects) => todo!(),
            PlayerEffect::CreateCards(
                _card_pool,
                _card_selection,
                _card_destination,
                _cost_modifier,
            ) => todo!(),
            PlayerEffect::Draw(draw_count) => {
                for _ in 0..*draw_count {
                    self.player_combat_system.draw_system.draw_one_card(
                        ctx.comms,
                        ctx.pcs,
                        effect_queue,
                    )?;
                }
                Ok(())
            }
            PlayerEffect::ForEachExhausted(_player_effects) => todo!(),
            PlayerEffect::Gain(resource) => self.gain_resource(ctx, resource, effect_queue),
            PlayerEffect::Lose(_resource) => todo!(),
            PlayerEffect::ManipulateCards(
                _card_source,
                _card_selection,
                _card_destination,
                _cost_modifier,
            ) => todo!(),
            PlayerEffect::PlayThenExhaustTopCardOfDrawPile => todo!(),
            PlayerEffect::RampUpCardDamage(_) => todo!(),
            PlayerEffect::TakeDamage(_damage) => todo!(),
            PlayerEffect::ToAllEnemies(target_effect) => {
                for enemy_index in 0..ctx.enemy_party.0.len() {
                    if ctx.enemy_party.0[enemy_index].is_some() {
                        ctx.maybe_enemy_index = Some(enemy_index);
                        self.to_target_effect(ctx, target_effect, effect_queue)?;
                    }
                    self.process_effect_queue(ctx, effect_queue)?;
                }
                Ok(())
            }
            PlayerEffect::ToRandomEnemy(target_effect) => {
                ctx.maybe_enemy_index = self.pick_random_enemy(ctx);
                self.to_target_effect(ctx, target_effect, effect_queue)
            }
            PlayerEffect::ToSingleTarget(target_effect) => {
                assert!(ctx.maybe_enemy_index.is_some());
                self.to_target_effect(ctx, target_effect, effect_queue)
            }
            PlayerEffect::Upgrade(_card_source, _card_selection) => todo!(),
        }
    }

    fn gain_resource<I: Interaction>(
        &mut self,
        ctx: &mut CombatContext<I>,
        resource: &Resource,
        _effect_queue: &mut EffectQueue,
    ) -> Result<(), Error> {
        match resource {
            Resource::Block(block) => {
                let calculated_block = DamageCalculator::calculate_block_gained(ctx.pcs, *block);
                BlockSystem::gain_block(ctx.comms, ctx.pcs, calculated_block)
            }
            Resource::CurrentBlockIsDoubled => todo!(),
            Resource::CurrentStrengthIsDoubled => todo!(),
            Resource::Energy(_) => todo!(),
            Resource::Gold(_) => todo!(),
            Resource::Hp(_) => todo!(),
            Resource::HpEqualToUnblockedDamage => todo!(),
            Resource::HpMax(_) => todo!(),
            Resource::Strength(_) => todo!(),
        }
    }

    fn pick_random_enemy<I: Interaction>(&mut self, ctx: &CombatContext<I>) -> Option<EnemyIndex> {
        let living_enemies = ctx
            .enemy_party
            .0
            .iter()
            .enumerate()
            .filter(|(_, e)| e.is_some())
            .collect::<Vec<_>>();
        let living_index = self.misc_rng.gen_range(0..living_enemies.len());
        Some(living_enemies[living_index].0)
    }

    fn to_target_effect<I: Interaction>(
        &self,
        ctx: &mut CombatContext<I>,
        effect: &TargetEffect,
        effect_queue: &mut EffectQueue,
    ) -> Result<(), Error> {
        if let Some(enemy_state) = ctx
            .maybe_enemy_index
            .and_then(|i| ctx.enemy_party.0.get_mut(i))
            .and_then(|maybe_enemy| maybe_enemy.as_mut())
        {
            match effect {
                TargetEffect::Conditional(_target_condition, _player_effects) => todo!(),
                TargetEffect::Deal(damage) => {
                    let calculated_damage = DamageCalculator::calculate_damage_inflicted(
                        ctx.pcs,
                        Some(enemy_state),
                        damage,
                    );
                    BlockSystem::damage_enemy(enemy_state, calculated_damage, effect_queue);
                    Ok(())
                }
                TargetEffect::DealXTimes(_damage) => todo!(),
                TargetEffect::Inflict(enemy_condition) => {
                    EnemyConditionSystem::apply_to_enemy(enemy_state, enemy_condition);
                    Ok(())
                }
                TargetEffect::SapStrength(_) => todo!(),
            }
        } else {
            Ok(())
        }
    }

    fn process_enemy_effect<I: Interaction>(
        &mut self,
        ctx: &mut CombatContext<I>,
        effect: &EnemyEffect,
        effect_queue: &mut EffectQueue,
    ) -> Result<(), Error> {
        if let Some(enemy_state) = ctx
            .maybe_enemy_index
            .and_then(|i| ctx.enemy_party.0.get_mut(i))
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
                ) => {
                    CardCreationSystem::create_card(
                        ctx.comms,
                        ctx.pcs,
                        (card_pool, card_selection, card_destination, cost_modifier),
                        effect_queue,
                    )?;
                }
                EnemyEffect::Deal(damage) => {
                    let damage = DamageCalculator::calculate_damage_inflicted(
                        enemy_state,
                        Some(ctx.pcs),
                        damage,
                    );
                    BlockSystem::damage_player(ctx.comms, ctx.pcs, damage, effect_queue)?;
                }
                EnemyEffect::Gain(Resource::Block(block)) => {
                    let calculated_block =
                        DamageCalculator::calculate_block_gained(enemy_state, *block);
                    enemy_state.block += calculated_block.amount;
                }
                EnemyEffect::Gain(Resource::Strength(strength)) => {
                    enemy_state.strength += strength;
                }
                EnemyEffect::Gain(invalid) => unreachable!("{:?}", invalid),
                EnemyEffect::Inflict(player_condition) => {
                    PlayerConditionSystem::apply_to_player(ctx.comms, ctx.pcs, player_condition)?;
                }
            }
        }
        Ok(())
    }

    /// Returns true if the combat should end.
    fn combat_should_end<I: Interaction>(ctx: &CombatContext<I>) -> bool {
        ctx.pcs.pps.hp == 0 || ctx.enemy_party.0.iter().all(|enemy| enemy.is_none())
    }
}
