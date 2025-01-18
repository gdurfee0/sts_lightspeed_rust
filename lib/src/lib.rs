mod data;
mod enemy;
mod map;
mod player;
mod rng;
mod sim;
mod ui;

pub use data::character::Character;
pub use data::encounter::Encounter;
pub use player::{Choice, PlayerController, PotionAction, Prompt, StsMessage};
pub use rng::{Seed, StsRandom};
pub use sim::{EncounterSimulator, StsSimulator};

pub type AttackDamage = Hp; // Amount of damage dealt by an attack.
pub type AttackCount = u32; // Number of attacks in a multi-attack.
pub type Block = u32; // Amount of block applied by a skill.
pub type DiscardCount = u32; // Number of cards discarded by a skill.
pub type DrawCount = u32; // Number of cards drawn by a skill.
pub type Gold = u32;
pub type Energy = u32;
pub type Focus = i32;
pub type Health = (Hp, HpMax); // Current and maximum hit points.
pub type Hp = u32; // Player or enemy current hit points.
pub type HpMax = u32; // Maximum player or enemy hit points.
pub type OrbCount = u32; // Number of orbs of a particular type.
pub type StackCount = i32; // Number of stacks of a buff or debuff.

pub type DeckIndex = usize;
pub type DiscardIndex = usize;
pub type DrawIndex = usize;
pub type EnemyIndex = usize;
pub type HandIndex = usize;
pub type PotionIndex = usize;

// Map coordinates.
pub type ColumnIndex = usize;
pub type RowIndex = usize;
