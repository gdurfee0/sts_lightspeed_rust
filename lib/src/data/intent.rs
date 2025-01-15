use crate::{AttackAmount, AttackCount};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Intent {
    Aggressive(AttackAmount, AttackCount),
    AggressiveBuff(AttackAmount, AttackCount),
    AggressiveDebuff(AttackAmount, AttackCount),
    AggressiveDefensive(AttackAmount, AttackCount),
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
