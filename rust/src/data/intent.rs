#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Intent {
    Aggressive(u32, u32),
    AggressiveBuff(u32, u32),
    AggressiveDebuff(u32, u32),
    AggressiveDefensive(u32, u32),
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
