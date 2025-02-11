use crate::components::CardCombatState;
use crate::types::EnemyIndex;

pub enum CombatAction {
    EndTurn,
    PlayCard(CardCombatState, Option<EnemyIndex>),
}
