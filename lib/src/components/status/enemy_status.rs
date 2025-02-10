use crate::components::{AttackerStatus, DefenderStatus};
use crate::data::{Enemy, EnemyCondition, Intent};
use crate::types::{Block, Dexterity, Hp, HpMax, Strength};

/// `EnemyStatus` is a small bundle of information about the enemy that is made available to
/// the player. The player is not allowed to know anything else about the enemy, such as its
/// internal state or future moves.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct EnemyStatus {
    pub enemy_type: Enemy,
    pub hp: Hp,
    pub hp_max: HpMax,
    pub block: Block,
    pub strength: Strength,
    pub conditions: Vec<EnemyCondition>,
    pub intent: Intent,
}

impl AttackerStatus for EnemyStatus {
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

impl DefenderStatus for EnemyStatus {
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
