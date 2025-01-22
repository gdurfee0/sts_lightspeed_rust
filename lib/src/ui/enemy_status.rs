use std::fmt;

use crate::components::EnemyStatus;

impl fmt::Display for EnemyStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}, HP: {}/{}", self.enemy_type, self.hp, self.hp_max)?;
        if self.block > 0 {
            write!(f, ", block: {}", self.block)?;
        }
        if !self.conditions.is_empty() {
            write!(
                f,
                ", debuffs: [{}]",
                self.conditions
                    .iter()
                    .map(|condition| format!("{:?}", condition))
                    .collect::<Vec<_>>()
                    .join(", ")
            )?;
        }
        write!(f, ", intent: {:?}", self.intent)
    }
}
