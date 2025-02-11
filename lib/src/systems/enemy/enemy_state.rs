use crate::components::{AttackerStatus, DefenderStatus, EnemyStatus};
use crate::data::{Enemy, EnemyAction, EnemyCondition};
use crate::systems::rng::StsRandom;
use crate::types::{Block, Dexterity, Hp, HpMax, Strength};

use super::enemy_characteristics::EnemyCharacteristics;

/// The `EnemyState` is the basic unit representing enemy combatants in the game.
/// As per the ECS model, this is just a collection of components. We do provide some
/// convenience methods for querying state in the impl though.
pub struct EnemyState {
    pub enemy: Enemy,
    pub hp: Hp,
    pub hp_max: HpMax,
    pub block: Block,
    pub strength: Strength,
    pub conditions: Vec<EnemyCondition>,
    pub next_action: EnemyAction,
    pub run_length: usize,
    pub characteristics: Box<dyn EnemyCharacteristics>,
}

impl EnemyState {
    /// Creates a new enemy state from the specified enemy type and characteristics.
    pub fn new(
        enemy: Enemy,
        characteristics: Box<dyn EnemyCharacteristics>,
        ai_rng: &mut StsRandom,
    ) -> Self {
        let (hp_max, conditions, first_action) = characteristics.on_spawn(ai_rng);
        Self {
            enemy,
            hp: hp_max,
            hp_max,
            block: 0,
            strength: 0,
            conditions,
            run_length: 1,
            next_action: first_action,
            characteristics,
        }
    }

    /// Returns true iff the enemy is dead.
    pub fn is_dead(&self) -> bool {
        self.hp == 0
    }
}

impl AttackerStatus for EnemyState {
    fn block(&self) -> Block {
        self.block
    }

    fn draw_pile_size(&self) -> usize {
        0
    }

    fn hand_size(&self) -> usize {
        0
    }

    fn is_weak(&self) -> bool {
        self.conditions
            .iter()
            .any(|c| matches!(c, EnemyCondition::Weak(_)))
    }

    fn number_of_strike_cards_owned(&self) -> usize {
        0
    }

    fn strength(&self) -> Strength {
        self.strength
    }
}

impl DefenderStatus for EnemyState {
    fn dexterity(&self) -> Dexterity {
        0
    }

    fn is_frail(&self) -> bool {
        false
    }

    fn is_vulnerable(&self) -> bool {
        self.conditions
            .iter()
            .any(|c| matches!(c, EnemyCondition::Vulnerable(_)))
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
