use anyhow::Error;

use crate::components::{
    CardCombatState, Effect, EffectQueue, Interaction, Notification, PlayerCombatState,
};
use crate::data::{
    Card, CardType, Damage, EnergyCost, PlayerCondition, PlayerEffect, TargetEffect,
};
use crate::systems::base::RelicSystem;
use crate::systems::rng::{Seed, StsRandom};

pub struct DrawSystem {
    shuffle_rng: StsRandom,
    card_randomizer_rng: StsRandom,
}

impl DrawSystem {
    /// Creates a new draw system with the given seed.
    pub fn new(seed_for_floor: Seed) -> Self {
        let shuffle_rng = StsRandom::from(seed_for_floor);
        let card_randomizer_rng = StsRandom::from(seed_for_floor);
        Self {
            shuffle_rng,
            card_randomizer_rng,
        }
    }

    /// Sets up the draw pile for combat.
    pub fn start_combat(&mut self, pcs: &mut PlayerCombatState) {
        self.shuffle_rng
            .java_compat_shuffle(&mut pcs.cards.draw_pile);
        pcs.cards.draw_pile.sort_by_key(|card| card.details.innate);
    }

    /// Draws the appropriate number of cards at the start of the player's turn.
    pub fn start_turn<I: Interaction>(
        &mut self,
        comms: &I,
        pcs: &mut PlayerCombatState,
        effect_queue: &mut EffectQueue,
    ) -> Result<(), Error> {
        let cards_to_draw = 5 + RelicSystem::extra_cards_to_draw_at_start_of_player_turn(pcs.pps);
        // TODO: Extra draws when innate card count > cards_to_draw
        for _ in 0..cards_to_draw {
            self.draw_one_card(comms, pcs, effect_queue)?;
        }
        Ok(())
    }

    /// Draws one card.
    pub fn draw_one_card<I: Interaction>(
        &mut self,
        comms: &I,
        pcs: &mut PlayerCombatState,
        effect_queue: &mut EffectQueue,
    ) -> Result<(), Error> {
        if pcs.conditions.contains(&PlayerCondition::NoDraw) || pcs.cards.hand.len() >= 10 {
            Ok(())
        } else if let Some(card) = pcs.cards.draw_pile.pop() {
            self.put_drawn_card_into_hand(comms, pcs, card, effect_queue)
        } else {
            // Shuffle discard pile into draw pile
            comms.send_notification(Notification::ShufflingDiscardPileIntoDrawPile)?;
            self.shuffle_rng
                .java_compat_shuffle(&mut pcs.cards.discard_pile);
            pcs.cards.draw_pile.append(&mut pcs.cards.discard_pile);
            if let Some(card) = pcs.cards.draw_pile.pop() {
                self.put_drawn_card_into_hand(comms, pcs, card, effect_queue)
            } else {
                Ok(())
            }
        }
    }

    /// Puts a drawn card into the player's hand.
    fn put_drawn_card_into_hand<I: Interaction>(
        &mut self,
        comms: &I,
        pcs: &mut PlayerCombatState,
        mut combat_card: CardCombatState,
        effect_queue: &mut EffectQueue,
    ) -> Result<(), Error> {
        // TODO: Move these checks into ConditionSystem?
        for condition in pcs.conditions.iter() {
            match condition {
                PlayerCondition::Confused => {
                    combat_card.cost_this_combat = *self.card_randomizer_rng.choose(&[
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
                        effect_queue
                            .push_back(Effect::PlayerState(PlayerEffect::Draw(*draw_count)));
                    }
                }
                PlayerCondition::FireBreathing(hp) => {
                    if matches!(
                        combat_card.details.type_,
                        CardType::Status | CardType::Curse
                    ) {
                        effect_queue.push_back(Effect::PlayerState(PlayerEffect::ToAllEnemies(
                            TargetEffect::Deal(Damage::BlockableNonAttack(*hp)),
                        )));
                    }
                }
                PlayerCondition::NoDraw => unreachable!(),
                _ => {}
            }
        }
        if let Some(effect) = combat_card.details.on_draw.as_ref() {
            effect_queue.push_back(Effect::Card(effect));
        }
        if combat_card.card == Card::Normality {
            // TODO: Implement normality counter
        }
        pcs.cards.hand.push(combat_card);
        comms.send_notification(Notification::CardDrawn(
            pcs.cards.hand.len() - 1,
            combat_card,
        ))
    }
}
