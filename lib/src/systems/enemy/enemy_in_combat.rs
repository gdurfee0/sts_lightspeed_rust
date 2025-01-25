use crate::components::{EnemyState, EnemyStatus};
use crate::data::{Enemy, EnemyAction, EnemyCondition};
use crate::systems::rng::StsRandom;
use crate::types::{AttackDamage, Block};

use super::enemy_characteristics::{characteristics_for_enemy, EnemyCharacteristics};

#[derive(Debug)]
pub struct EnemyInCombat {
    pub state: EnemyState,
    enemy_characteristics: Box<dyn EnemyCharacteristics>,
}

impl EnemyInCombat {
    pub fn new(enemy: Enemy, hp_rng: &mut StsRandom, ai_rng: &mut StsRandom) -> Self {
        let (health_range, mut enemy_characteristics) = characteristics_for_enemy(enemy);
        let hp_max = hp_rng.gen_range(health_range.clone());
        let state = EnemyState::new(
            enemy,
            hp_max,
            enemy_characteristics.first_action(ai_rng),
            enemy_characteristics.powers(),
        );
        Self {
            state,
            enemy_characteristics,
        }
    }

    pub fn next_action(&mut self, ai_rng: &mut StsRandom) -> EnemyAction {
        let action = self.state.next_action;
        self.state.next_action =
            self.enemy_characteristics
                .next_action(ai_rng, action, self.state.run_length);
        if self.state.next_action == action {
            self.state.run_length = self.state.run_length.saturating_add(1);
        } else {
            self.state.run_length = 1;
        }
        action
    }

    pub fn start_turn(&mut self) {
        self.state.block = 0;
    }

    pub fn end_turn(&mut self) {
        for condition in self.state.conditions.iter_mut() {
            match condition {
                EnemyCondition::Ritual(intensity, just_applied) => {
                    if !*just_applied {
                        self.state.strength += *intensity as i32;
                    }
                    *just_applied = false;
                }
                EnemyCondition::SporeCloud(_) => {}
                EnemyCondition::Vulnerable(turns) => *turns = turns.saturating_sub(1),
                EnemyCondition::Weak(turns) => *turns = turns.saturating_sub(1),
            }
        }
        self.state.conditions.retain(|c| match c {
            EnemyCondition::Vulnerable(turns) => *turns > 0,
            EnemyCondition::Weak(turns) => *turns > 0,
            _ => true,
        });
    }

    pub fn apply_condition(&mut self, condition: &EnemyCondition) {
        for preexisting_condition in self.state.conditions.iter_mut() {
            if Self::maybe_merge_conditions(preexisting_condition, condition) {
                return;
            }
        }
        // If we make it here, we didn't have this condition already.
        self.state.conditions.push(condition.clone());
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
            EnemyCondition::SporeCloud(_) => {
                if let EnemyCondition::SporeCloud(_) = incoming_condition {
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
        let block = self.state.block;
        let remaining_damage = amount.saturating_sub(block);
        self.state.block = self.state.block.saturating_sub(amount);
        self.state.hp = self.state.hp.saturating_sub(remaining_damage);
        (block, remaining_damage)
    }
}

impl From<&EnemyInCombat> for EnemyStatus {
    fn from(enemy: &EnemyInCombat) -> Self {
        (&enemy.state).into()
    }
}
