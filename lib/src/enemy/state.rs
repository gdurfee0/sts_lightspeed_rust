use crate::data::{EnemyCondition, EnemyType};
use crate::rng::StsRandom;
use crate::types::{AttackDamage, Block, Hp, HpMax, Strength};

use super::action::{enemy_params, Action, NextActionFn};
use super::status::EnemyStatus;

/// The `EnemyState` is the basic unit representing enemy combatants in the game.
#[derive(Debug)]
pub struct EnemyState {
    enemy_type: EnemyType,
    hp: Hp,
    hp_max: HpMax,
    block: Block,
    strength: Strength,
    conditions: Vec<EnemyCondition>,
    next_action_fn: NextActionFn,
    next_action: &'static Action,
    run_length: u8,
}

impl EnemyState {
    pub fn new(enemy_type: EnemyType, hp_rng: &mut StsRandom, ai_rng: &mut StsRandom) -> Self {
        let (health_range, next_action_fn) = enemy_params(enemy_type);
        let hp = hp_rng.gen_range(health_range);
        let hp_max = hp;
        let next_action = next_action_fn(ai_rng, None, 0);
        Self {
            enemy_type,
            hp,
            hp_max,
            block: 0,
            strength: 0,
            conditions: Vec::new(),
            next_action_fn,
            next_action,
            run_length: 1,
        }
    }

    pub fn hp(&self) -> Hp {
        self.hp
    }

    pub fn next_action(&mut self, ai_rng: &mut StsRandom) -> &'static Action {
        let action = self.next_action;
        self.next_action = (self.next_action_fn)(ai_rng, Some(action), self.run_length);
        if self.next_action == action {
            self.run_length = self.run_length.saturating_add(1);
        } else {
            self.run_length = 1;
        }
        action
    }

    /*
    pub fn start_turn(&mut self) -> bool {
        // TODO: Should this go at the end of the enemy's turn?
        for (_, stacks) in self.debuffs.iter_mut() {
            *stacks = stacks.saturating_sub(1);
        }
        self.debuffs.retain(|(_, stacks)| *stacks > 0);
        true
    }
    */

    pub fn end_turn(&mut self) {
        for condition in self.conditions.iter_mut() {
            match condition {
                EnemyCondition::Ritual(intensity, just_applied) => {
                    if !*just_applied {
                        self.strength += *intensity as i32;
                    }
                    *just_applied = false;
                }
                EnemyCondition::Vulnerable(turns) => *turns = turns.saturating_sub(1),
                EnemyCondition::Weak(turns) => *turns = turns.saturating_sub(1),
            }
        }
        /*
        self.conditions.retain(|c| {
            !matches!(
                c,
                EnemyCondition::Ritual(0, _)
                    | EnemyCondition::Vulnerable(0, _)
                    | EnemyCondition::Weak(0, _)
            )
        });
        */
    }

    pub fn is_dead(&self) -> bool {
        self.hp == 0
    }

    pub fn is_vulnerable(&self) -> bool {
        self.conditions
            .iter()
            .any(|c| matches!(c, EnemyCondition::Vulnerable(_)))
    }

    pub fn is_weak(&self) -> bool {
        self.conditions
            .iter()
            .any(|c| matches!(c, EnemyCondition::Weak(_)))
    }

    pub fn apply(&mut self, condition: &EnemyCondition) {
        for preexisting_condition in self.conditions.iter_mut() {
            if Self::maybe_merge_conditions(preexisting_condition, condition) {
                return;
            }
        }
        // If we make it here, we didn't have this condition already.
        self.conditions.push(condition.clone());
    }

    fn maybe_merge_conditions(
        existing_condition: &mut EnemyCondition,
        incoming_condition: &EnemyCondition,
    ) -> bool {
        match existing_condition {
            EnemyCondition::Ritual(intensity, just_applied) => {
                if let EnemyCondition::Ritual(additional_intensity, _) = incoming_condition {
                    *intensity = intensity.saturating_add(*additional_intensity);
                    *just_applied = true;
                    return true;
                }
            }
            EnemyCondition::Vulnerable(turns) => {
                if let EnemyCondition::Vulnerable(additional_turns) = incoming_condition {
                    *turns = turns.saturating_add(*additional_turns);
                    return true;
                }
            }
            EnemyCondition::Weak(turns) => {
                if let EnemyCondition::Weak(additional_turns) = incoming_condition {
                    *turns = turns.saturating_add(*additional_turns);
                    return true;
                }
            }
        }
        false
    }

    /// Damage amount must already have player and enemy conditions applied.
    pub fn take_damage(&mut self, amount: AttackDamage) -> (Block, AttackDamage) {
        let block = self.block;
        let remaining_damage = amount.saturating_sub(block);
        self.block = self.block.saturating_sub(amount);
        self.hp = self.hp.saturating_sub(remaining_damage);
        (block, remaining_damage)
    }
}

impl From<&EnemyState> for EnemyStatus {
    fn from(enemy: &EnemyState) -> Self {
        Self {
            enemy_type: enemy.enemy_type,
            hp: enemy.hp,
            hp_max: enemy.hp_max,
            strength: enemy.strength,
            block: enemy.block,
            conditions: enemy.conditions.clone(),
            intent: enemy.next_action.intent,
        }
    }
}
