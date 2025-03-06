use crate::components::CardCombatState;
use crate::types::EnemyIndex;

pub enum PlayerCombatAction {
    EndTurn,
    PlayCard(CardCombatState, Option<EnemyIndex>),
}
