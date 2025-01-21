// Source: Slay the Spire wiki (https://slay-the-spire.fandom.com/wiki/Intent)

use crate::types::{AttackCount, AttackDamage};

/// An `Intent` provides the user-visible view of the enemy's next action.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(test, derive(Eq, Hash, PartialEq))]
pub enum Intent {
    Aggressive(AttackDamage, AttackCount),
    AggressiveBuff(AttackDamage, AttackCount),
    AggressiveDebuff(AttackDamage, AttackCount),
    AggressiveDefensive(AttackDamage, AttackCount),
    Cowardly,
    Defensive,
    DefensiveBuff,
    DefensiveDebuff,
    Sleeping,
    StrategicBuff,
    StrategicDebuff,
    Stunned,
    Unknown,
}
