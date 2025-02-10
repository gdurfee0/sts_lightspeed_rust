// Source: Slay the Spire wiki (https://slay-the-spire.fandom.com/wiki/Intent)

use crate::types::{AttackCount, Hp};

/// An `Intent` provides the user-visible view of the enemy's next action.
#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum Intent {
    Aggressive(Hp, AttackCount),
    AggressiveBuff(Hp, AttackCount),
    AggressiveDebuff(Hp, AttackCount),
    AggressiveDefensive(Hp, AttackCount),
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
