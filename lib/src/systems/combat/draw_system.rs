use anyhow::Error;

use crate::components::{CardCombatState, Effect, Interaction, Notification};
use crate::data::{
    Card, CardType, Damage, EnergyCost, PlayerCondition, PlayerEffect, TargetEffect,
};
use crate::systems::base::{CombatContext, RelicSystem};
use crate::types::DrawCount;

pub struct DrawSystem;

impl DrawSystem {
    /// Sets up the draw pile for combat.
    pub fn on_combat_started<I: Interaction>(ctx: &mut CombatContext<I>) {
        ctx.shuffle_rng
            .java_compat_shuffle(&mut ctx.pcs.cards.draw_pile);
        ctx.pcs
            .cards
            .draw_pile
            .sort_by_key(|card| card.details.innate);
        // Count the innate cards
        let innate_count = ctx
            .pcs
            .cards
            .draw_pile
            .iter()
            .filter(|card| card.details.innate)
            .count() as DrawCount;
        let cards_to_draw =
            5 + RelicSystem::extra_cards_to_draw_at_start_of_player_turn(ctx.pcs.pps);
        if innate_count > cards_to_draw {
            ctx.effect_queue
                .push_back(Effect::PlayerState(PlayerEffect::Draw(
                    innate_count - cards_to_draw,
                )));
        }
    }

    /// Draws the appropriate number of cards at the start of the player's turn.
    pub fn on_player_turn_started<I: Interaction>(ctx: &mut CombatContext<I>) {
        ctx.effect_queue
            .push_back(Effect::PlayerState(PlayerEffect::Draw(
                5 + RelicSystem::extra_cards_to_draw_at_start_of_player_turn(ctx.pcs.pps),
            )));
    }

    /// Draws one card.
    pub fn draw_one_card<I: Interaction>(ctx: &mut CombatContext<I>) -> Result<(), Error> {
        if ctx.pcs.conditions.contains(&PlayerCondition::NoDraw) || ctx.pcs.cards.hand.len() >= 10 {
            Ok(())
        } else if let Some(card) = ctx.pcs.cards.draw_pile.pop() {
            Self::put_drawn_card_into_hand(ctx, card)
        } else {
            // Shuffle discard pile into draw pile
            ctx.comms
                .send_notification(Notification::ShufflingDiscardPileIntoDrawPile)?;
            ctx.shuffle_rng
                .java_compat_shuffle(&mut ctx.pcs.cards.discard_pile);
            ctx.pcs
                .cards
                .draw_pile
                .append(&mut ctx.pcs.cards.discard_pile);
            if let Some(card) = ctx.pcs.cards.draw_pile.pop() {
                Self::put_drawn_card_into_hand(ctx, card)
            } else {
                Ok(())
            }
        }
    }

    /// Puts a drawn card into the player's hand.
    fn put_drawn_card_into_hand<I: Interaction>(
        ctx: &mut CombatContext<I>,
        mut combat_card: CardCombatState,
    ) -> Result<(), Error> {
        // TODO: Move these checks into ConditionSystem?
        for condition in ctx.pcs.conditions.iter() {
            match condition {
                PlayerCondition::Confused => {
                    combat_card.cost_this_combat = *ctx.card_randomizer_rng.choose(&[
                        EnergyCost::Zero,
                        EnergyCost::One,
                        EnergyCost::Two,
                        EnergyCost::Three,
                    ]);
                    combat_card.cost_this_turn = combat_card.cost_this_combat;
                }
                PlayerCondition::Corruption => {
                    if matches!(combat_card.details.type_, CardType::Skill) {
                        combat_card.cost_this_turn = EnergyCost::Zero;
                    }
                }
                PlayerCondition::Evolve(draw_count) => {
                    if matches!(combat_card.details.type_, CardType::Status) {
                        ctx.effect_queue
                            .push_back(Effect::PlayerState(PlayerEffect::Draw(*draw_count)));
                    }
                }
                PlayerCondition::FireBreathing(hp) => {
                    if matches!(
                        combat_card.details.type_,
                        CardType::Status | CardType::Curse
                    ) {
                        ctx.effect_queue.push_back(Effect::PlayerState(
                            PlayerEffect::ToAllEnemies(TargetEffect::Deal(
                                Damage::BlockableNonAttack(*hp),
                            )),
                        ));
                    }
                }
                PlayerCondition::NoDraw => unreachable!(),
                _ => {}
            }
        }
        if let Some(effect) = combat_card.details.on_draw.as_ref() {
            ctx.effect_queue.push_back(Effect::Card(effect));
        }
        if combat_card.card == Card::Normality {
            // TODO: Implement normality counter
        }
        ctx.pcs.cards.hand.push(combat_card);
        ctx.comms.send_notification(Notification::CardDrawn(
            ctx.pcs.cards.hand.len() - 1,
            combat_card,
        ))
    }
}
