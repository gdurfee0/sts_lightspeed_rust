pub type AttackDamage = Hp; // Amount of damage dealt by an attack.
pub type AttackCount = u32; // Number of attacks in a multi-attack.
pub type Block = u32; // Amount of block applied by a skill.
pub type Dexterity = i32; // Player or enemy strength. Can be negative.
pub type DiscardCount = u32; // Number of cards discarded by a skill.
pub type DrawCount = u32; // Number of cards drawn by a skill.
pub type Gold = u32;
pub type Energy = u32;
pub type Focus = i32;
pub type Floor = u64;
pub type JustApplied = bool; // Whether a buff or debuff was just applied.
pub type Health = (Hp, HpMax); // Current and maximum hit points.
pub type Hp = u32; // Player or enemy current hit points.
pub type HpMax = u32; // Maximum player or enemy hit points.
pub type OrbCount = u32; // Number of orbs of a particular type.
pub type OrbSlots = u32; // Number of orb slots available.
pub type PotionSlots = u32; // Number of potion slots available.
pub type ScryCount = u32; // Number of cards to scry.
pub type StackCount = u32; // Number of stacks of a buff or debuff.
pub type Strength = i32; // Player or enemy strength. Can be negative.

pub type DeckIndex = usize;
pub type DiscardIndex = usize;
pub type DrawIndex = usize;
pub type EnemyIndex = usize;
pub type HandIndex = usize;
pub type PotionIndex = usize;
pub type RewardIndex = usize;

// Map coordinates.
pub type ColumnIndex = usize;
pub type RowIndex = usize;
