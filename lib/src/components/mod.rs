mod damage_taken;
mod effect_queue;
mod interaction;
pub mod map;
mod state; // "state" is all internal state the simulator uses
mod status; // "status" is the subset of state that is exposed to the player

pub use damage_taken::DamageTaken;
pub use effect_queue::{Effect, EffectQueue};
pub use interaction::{Choice, Interaction, Notification, PotionAction, Prompt, StsMessage};
pub use map::Room;
pub use state::{CardCombatState, CombatCards, PlayerCombatState, PlayerPersistentState};
pub use status::{AttackerStatus, DefenderStatus, EnemyStatus, PlayerStatus};
