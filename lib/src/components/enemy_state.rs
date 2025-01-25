use crate::data::{Enemy, EnemyAction, EnemyCondition};
use crate::types::{Block, Hp, HpMax, Strength};

use super::enemy_status::EnemyStatus;

/// The `EnemyState` is the basic unit representing enemy combatants in the game.
/// As per the ECS model, this is just a collection of components. We do provide some
/// convenience methods for querying state in the impl though.
#[derive(Debug)]
pub struct EnemyState {
    pub enemy: Enemy,
    pub hp: Hp,
    pub hp_max: HpMax,
    pub block: Block,
    pub strength: Strength,
    pub conditions: Vec<EnemyCondition>,
    pub next_action: EnemyAction,
    pub run_length: usize,
}

impl EnemyState {
    pub fn new(
        enemy: Enemy,
        hp_max: HpMax,
        first_action: EnemyAction,
        conditions: Vec<EnemyCondition>,
    ) -> Self {
        Self {
            enemy,
            hp: hp_max,
            hp_max,
            block: 0,
            strength: 0,
            conditions,
            run_length: 1,
            next_action: first_action,
        }
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
}

impl From<&EnemyState> for EnemyStatus {
    fn from(enemy: &EnemyState) -> Self {
        Self {
            enemy_type: enemy.enemy,
            hp: enemy.hp,
            hp_max: enemy.hp_max,
            strength: enemy.strength,
            block: enemy.block,
            conditions: enemy.conditions.clone(),
            intent: enemy.next_action.intent(),
        }
    }
}
