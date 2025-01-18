use std::fmt;

use crate::data::debuff::Debuff;
use crate::data::enemy::EnemyType;
use crate::{Block, Hp, HpMax, StackCount};

/// `EnemyStatus` is a small bundle of information about the enemy that is made available to
/// the player. The player is not allowed to know anything else about the enemy, such as its
/// internal state or future moves.
#[derive(Clone, Debug)]
pub struct EnemyStatus {
    pub enemy_type: EnemyType,
    pub hp: Hp,
    pub hp_max: HpMax,
    pub block: Block,
    pub debuffs: Vec<(Debuff, StackCount)>,
}

impl fmt::Display for EnemyStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}, HP: {}/{}", self.enemy_type, self.hp, self.hp_max)?;
        if self.block > 0 {
            write!(f, ", block: {}", self.block)?;
        }
        if !self.debuffs.is_empty() {
            write!(
                f,
                ", debuffs: [{}]",
                self.debuffs
                    .iter()
                    .map(|(debuff, stack_count)| format!("{:?}({}), ", debuff, stack_count))
                    .collect::<Vec<_>>()
                    .join(", ")
            )?;
        }
        Ok(())
    }
}
