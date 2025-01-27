use crate::data::{Enemy, EnemyCondition, Intent};
use crate::types::{Block, Hp, HpMax, Strength};

/// `EnemyStatus` is a small bundle of information about the enemy that is made available to
/// the player. The player is not allowed to know anything else about the enemy, such as its
/// internal state or future moves.
#[derive(Clone, Debug)]
#[cfg_attr(test, derive(Eq, Hash, PartialEq))]
pub struct EnemyStatus {
    pub enemy_type: Enemy,
    pub hp: Hp,
    pub hp_max: HpMax,
    pub block: Block,
    pub strength: Strength,
    pub conditions: Vec<EnemyCondition>,
    pub intent: Intent,
}

impl EnemyStatus {
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

#[cfg(test)]
mod tests {
    use super::*;

    use crate::types::Health;

    // Helper methods for tests elsewhere in the codebase
    impl EnemyStatus {
        pub fn new(enemy_type: Enemy, health: Health, intent: Intent) -> Self {
            Self {
                enemy_type,
                hp: health.0,
                hp_max: health.1,
                strength: 0,
                block: 0,
                conditions: Vec::new(),
                intent,
            }
        }

        pub fn with_block(mut self, block: Block) -> Self {
            self.block = block;
            self
        }

        pub fn with_condition(mut self, condition: EnemyCondition) -> Self {
            self.conditions.push(condition);
            self
        }

        pub fn with_strength(mut self, strength: Strength) -> Self {
            self.strength = strength;
            self
        }
    }
}
