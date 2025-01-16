// Source: Slay the Spire wiki (https://slay-the-spire.fandom.com/wiki/Intent)

use crate::{AttackCount, AttackDamage};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
