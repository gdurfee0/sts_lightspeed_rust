mod card_combat_state;
mod combat_cards;
mod player_combat_state;
mod player_persistent_state;

pub use card_combat_state::CardCombatState;
pub use combat_cards::CombatCards;
pub use player_combat_state::PlayerCombatState;
pub use player_persistent_state::PlayerPersistentState;

use super::status::PlayerStatus;

impl From<(&PlayerPersistentState, &PlayerCombatState)> for PlayerStatus {
    fn from(value: (&PlayerPersistentState, &PlayerCombatState)) -> Self {
        let (persistent_state, combat_state) = value;
        Self {
            hp: persistent_state.hp,
            hp_max: persistent_state.hp_max,
            gold: persistent_state.gold,
            relics: persistent_state.relics.clone(),
            deck: persistent_state.deck.clone(),
            potions: persistent_state.potions.clone(),
            energy: combat_state.energy,
            block: combat_state.block,
            conditions: combat_state.conditions.clone(),
            hand: combat_state.cards.hand.clone(),
            draw_pile: combat_state.cards.sanitized_draw_pile(),
            discard_pile: combat_state.cards.sanitized_discard_pile(),
            exhaust_pile: combat_state.cards.sanitized_exhaust_pile(),
            hp_loss_count: combat_state.hp_loss_count,
            strength: combat_state.strength,
            dexterity: combat_state.dexterity,
        }
    }
}
