use std::fmt;

use crate::{Debuff, Hp, HpMax, StackCount};

use super::id::EnemyType;
use super::intent::Intent;

/// `EnemyStatus` is a small bundle of information about the enemy that is made available to
/// the player. The player is not allowed to know anything else about the enemy, such as its
/// internal state or future moves.
#[derive(Clone, Debug)]
pub struct EnemyStatus {
    pub enemy_type: EnemyType,
    pub intent: Intent,
    pub hp: Hp,
    pub hp_max: HpMax,
    pub debuffs: Vec<(Debuff, StackCount)>,
}

impl fmt::Display for EnemyStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?}, HP: {}/{}, intends: {:?}",
            self.enemy_type, self.hp, self.hp_max, self.intent
        )?;
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
