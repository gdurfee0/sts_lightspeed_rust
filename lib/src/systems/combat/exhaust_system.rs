use anyhow::Error;

use crate::components::{
    CardCombatState, Effect, EffectQueue, Interaction, Notification, PlayerCombatState,
};
use crate::data::{PlayerCondition, PlayerEffect, Resource};
use crate::systems::base::RelicSystem;
use crate::types::HandIndex;

pub struct ExhaustSystem;

impl ExhaustSystem {
    /// Exhausts the indicated card and notifies the player of the change. Queues any effects
    /// that should be applied as a result of the player's relics and conditions.
    pub fn push<I: Interaction>(
        comms: &I,
        pcs: &mut PlayerCombatState,
        hand_index: HandIndex,
        combat_card: CardCombatState,
        effect_queue: &mut EffectQueue,
    ) -> Result<(), Error> {
        RelicSystem::on_card_exhausted(pcs.pps, effect_queue);
        // TODO: Move these checks to PlayerConditionSystem
        for condition in pcs.conditions.iter() {
            match condition {
                PlayerCondition::DarkEmbrace(card_count) => {
                    effect_queue.push_back(Effect::PlayerState(PlayerEffect::Draw(*card_count)));
                }
                PlayerCondition::FeelNoPain(block) => {
                    effect_queue.push_back(Effect::PlayerState(PlayerEffect::Gain(
                        Resource::Block(*block),
                    )));
                }
                _ => {}
            }
        }
        if let Some(effect) = combat_card.details.on_exhaust.as_ref() {
            effect_queue.push_back(Effect::Card(effect));
        }
        pcs.cards.exhaust_pile.push(combat_card);
        comms.send_notification(Notification::CardExhausted(hand_index, combat_card))
    }
}
