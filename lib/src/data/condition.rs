use crate::types::{JustApplied, StackCount};

#[derive(Clone, Debug)]
#[cfg_attr(test, derive(Eq, Hash, PartialEq))]
pub enum EnemyCondition {
    /// At the end of its turn, gains X strength.
    Ritual(StackCount, JustApplied),
    /// Target takes 50% more damage from attacks.
    Vulnerable(StackCount),
    /// Target deals 25% less attack damage.
    Weak(StackCount),
}

#[derive(Clone, Debug)]
#[cfg_attr(test, derive(Eq, Hash, PartialEq))]
pub enum PlayerCondition {
    /// Block gained from cards is reduced by 25%.
    Frail(StackCount),
    /// You take 50% more damage from attacks.
    Vulnerable(StackCount),
    /// You deal 25% less attack damage.
    Weak(StackCount),
}
