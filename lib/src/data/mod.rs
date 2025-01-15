mod act;
mod card;
mod character;
mod encounter;
mod enemy;
mod event;
mod intent;
mod neow;
mod potion;
mod relic;

pub use act::Act;
pub use card::{Card, CURSE_CARD_POOL, UNCOMMON_COLORLESS_CARDS};
pub use character::Character;
pub use encounter::Encounter;
pub use enemy::EnemyType;
pub use event::Event;
pub use intent::Intent;
pub use neow::{
    NeowBlessing, NeowBonus, NeowPenalty, FIRST_NEOW_POOL, SECOND_NEOW_POOL, THIRD_NEOW_POOL,
};
pub use potion::Potion;
pub use relic::Relic;

pub type AttackAmount = u32;
pub type AttackCount = u32;

#[cfg(test)]
pub use character::{DEFECT, IRONCLAD, SILENT, WATCHER};
