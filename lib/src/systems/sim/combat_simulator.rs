use anyhow::Error;

use crate::components::{Effect, Interaction, PlayerPersistentState};
use crate::data::Encounter;
use crate::systems::base::CombatContext;
use crate::systems::combat::{
    EffectSystem, EnemyCombatSystem, PlayerCombatAction, PlayerCombatSystem,
};
use crate::systems::rng::{Seed, StsRandom};

pub struct CombatSimulator<'a> {
    seed_for_floor: Seed,
    misc_rng: &'a mut StsRandom,
}

impl<'a> CombatSimulator<'a> {
    /// Creates a new combat simulator.
    pub fn new(seed_for_floor: Seed, misc_rng: &'a mut StsRandom) -> Self {
        Self {
            seed_for_floor,
            misc_rng,
        }
    }

    /// Runs a combat encounter, returning true if the player wins.
    pub fn run_encounter<I: Interaction>(
        self,
        comms: &I,
        encounter: Encounter,
        pps: &mut PlayerPersistentState,
    ) -> Result<bool, Error> {
        println!("[CombatSimulator] Running encounter: {:?}", encounter);
        let mut ctx = CombatContext::new(comms, self.seed_for_floor, encounter, pps, self.misc_rng);
        PlayerCombatSystem::on_combat_started(&mut ctx)?;
        loop {
            ctx.maybe_enemy_index = None;
            Self::conduct_player_turn(&mut ctx)?;
            if ctx.combat_should_end() {
                break;
            }
            Self::conduct_enemies_turn(&mut ctx)?;
            if ctx.combat_should_end() {
                break;
            }
        }
        PlayerCombatSystem::on_combat_finished(comms, pps)?;
        let victorious = pps.hp > 0;
        Ok(victorious)
    }

    /// Conducts the player's turn.
    fn conduct_player_turn<I: Interaction>(ctx: &mut CombatContext<I>) -> Result<(), Error> {
        PlayerCombatSystem::on_player_turn_started(ctx)?;
        EffectSystem::process_effect_queue(ctx)?;
        while !ctx.combat_should_end() {
            match PlayerCombatSystem::choose_next_action(ctx)? {
                PlayerCombatAction::PlayCard(combat_card, maybe_enemy_index) => {
                    ctx.maybe_enemy_index = maybe_enemy_index;
                    println!("Disposing of card just played: {:?}", combat_card);
                    PlayerCombatSystem::dispose_of_card_just_played(ctx)?;
                    EffectSystem::process_effect_queue(ctx)?;
                    for effect in combat_card.details.on_play.iter() {
                        ctx.effect_queue.push_back(Effect::Card(effect));
                    }
                    println!("Hand is now {:?}", ctx.pcs.cards.hand);
                    EffectSystem::process_effect_queue(ctx)?;
                }
                PlayerCombatAction::EndTurn => break,
            };
        }
        if !ctx.combat_should_end() {
            PlayerCombatSystem::on_player_turn_finished(ctx)?;
            EffectSystem::process_effect_queue(ctx)?;
        }
        Ok(())
    }

    /// Conducts the enemies' turn.
    fn conduct_enemies_turn<I: Interaction>(ctx: &mut CombatContext<I>) -> Result<(), Error> {
        EnemyCombatSystem::on_enemies_turn_started(ctx);
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
                    ctx.effect_queue.push_back(Effect::EnemyPlaybook(effect));
                }
                while let Some(effect) = ctx.effect_queue.pop_front() {
                    EffectSystem::process_effect(ctx, effect)?;
                    if ctx.combat_should_end() || ctx.enemy_party.0[enemy_index].is_none() {
                        break;
                    }
                }
            }
            if ctx.combat_should_end() {
                break;
            }
        }
        if !ctx.combat_should_end() {
            EnemyCombatSystem::on_enemies_turn_finished(ctx);
        }
        Ok(())
    }
}
