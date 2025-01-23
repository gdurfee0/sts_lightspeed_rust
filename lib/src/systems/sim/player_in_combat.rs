use anyhow::Error;

use crate::components::{CardInCombat, EnemyStatus, Notification, PlayerCombatState, PotionAction};
use crate::data::{Card, Enemy, PlayerCondition, Relic};
use crate::systems::rng::StsRandom;
use crate::types::{AttackDamage, Block, EnemyIndex, HandIndex};

use super::player::Player;

/// Captures the state of a combat encounter, including the player's hand, draw pile, etc.
/// Lives only as long as the combat encounter itself.  TODO: lock down field visibility
#[derive(Debug)]
pub struct PlayerInCombat<'a> {
    shuffle_rng: StsRandom,
    player: &'a mut Player,
    pub state: PlayerCombatState,
    card_just_played: Option<HandIndex>,
}

/// Returned by various methods to indicate the player's choice of action in combat.
#[derive(Clone, Debug)]
pub enum CombatAction {
    EndTurn,
    PlayCard(Card),
    PlayCardAgainstEnemy(Card, EnemyIndex),
    Potion(PotionAction),
}

impl<'a> PlayerInCombat<'a> {
    pub fn new(mut shuffle_rng: StsRandom, player: &'a mut Player) -> Self {
        let mut combat_state = PlayerCombatState::new(&player.state.deck);
        shuffle_rng.java_compat_shuffle(&mut combat_state.draw_pile);
        // Move innate cards to the top of the draw pile
        combat_state
            .draw_pile
            .sort_by_key(|card| card.card.is_innate());
        // TODO: Draw more than 5 cards if there are more than 5 innate cards
        Self {
            shuffle_rng,
            player,
            state: combat_state,
            card_just_played: None,
        }
    }

    pub fn apply_condition(&mut self, condition: &PlayerCondition) -> Result<(), Error> {
        for preexisting_condition in self.state.conditions.iter_mut() {
            if Self::maybe_merge_conditions(preexisting_condition, condition) {
                return self
                    .player
                    .comms
                    .send_notification(Notification::Conditions(self.state.conditions.clone()));
            }
        }
        // If we make it here, we didn't have this condition already.
        self.state.conditions.push(condition.clone());
        self.player
            .comms
            .send_notification(Notification::Conditions(self.state.conditions.clone()))
    }

    fn maybe_merge_conditions(
        existing_condition: &mut PlayerCondition,
        incoming_condition: &PlayerCondition,
    ) -> bool {
        match existing_condition {
            PlayerCondition::Frail(turns) => {
                if let PlayerCondition::Frail(additional_turns) = incoming_condition {
                    *turns = turns.saturating_add(*additional_turns);
                    return true;
                }
            }
            PlayerCondition::Vulnerable(turns) => {
                if let PlayerCondition::Vulnerable(additional_turns) = incoming_condition {
                    *turns = turns.saturating_add(*additional_turns);
                    return true;
                }
            }
            PlayerCondition::Weak(turns) => {
                if let PlayerCondition::Weak(additional_turns) = incoming_condition {
                    *turns = turns.saturating_add(*additional_turns);
                    return true;
                }
            }
        }
        false
    }

    pub fn tick_down_conditions(&mut self) {
        for condition in self.state.conditions.iter_mut() {
            match condition {
                PlayerCondition::Frail(turns) => *turns = turns.saturating_sub(1),
                PlayerCondition::Vulnerable(turns) => *turns = turns.saturating_sub(1),
                PlayerCondition::Weak(turns) => *turns = turns.saturating_sub(1),
            }
        }
        self.state.conditions.retain(|c| {
            !matches!(
                c,
                PlayerCondition::Frail(0)
                    | PlayerCondition::Vulnerable(0)
                    | PlayerCondition::Weak(0)
            )
        });
    }

    pub fn start_combat(&self) -> Result<(), Error> {
        self.player
            .comms
            .send_notification(Notification::StartingCombat)
    }

    pub fn start_turn(&mut self) -> Result<(), Error> {
        // Reset energy
        // TODO: energy conservation
        self.state.energy = 3;

        // Draw cards
        self.draw_cards()?;

        // Set block to 0
        if self.state.block > 0 {
            self.state.block = 0;
            self.player
                .comms
                .send_notification(Notification::Block(self.state.block))?;
        }

        // TODO: Apply other start-of-turn effects
        Ok(())
    }

    pub fn end_turn(&mut self) -> Result<(), Error> {
        self.discard_hand()?;

        // Tick down debuffs
        self.tick_down_conditions();
        self.player
            .comms
            .send_notification(Notification::Conditions(self.state.conditions.clone()))?;

        // TODO: Apply other end-of-turn effects
        Ok(())
    }

    pub fn end_combat(self) -> Result<(), Error> {
        if self.player.state.has_relic(Relic::BurningBlood) {
            self.player.increase_hp(6)?;
        }
        self.player
            .comms
            .send_notification(Notification::EndingCombat)
    }

    pub fn take_blockable_damage(&mut self, amount: AttackDamage) -> Result<(), Error> {
        if amount <= self.state.block {
            self.state.block -= amount;
            self.player
                .comms
                .send_notification(Notification::DamageBlocked(amount))?;
            self.player
                .comms
                .send_notification(Notification::BlockLost(amount))?;
            self.player
                .comms
                .send_notification(Notification::Block(self.state.block))
        } else if self.state.block > 0 {
            let remaining_damage = amount - self.state.block;
            self.player
                .comms
                .send_notification(Notification::DamageBlocked(self.state.block))?;
            self.player
                .comms
                .send_notification(Notification::BlockLost(self.state.block))?;
            self.state.block = 0;
            self.player
                .comms
                .send_notification(Notification::Block(0))?;
            self.take_unblockable_damage(remaining_damage)
        } else {
            self.take_unblockable_damage(amount)
        }
    }

    pub fn take_unblockable_damage(&mut self, amount: AttackDamage) -> Result<(), Error> {
        self.player
            .comms
            .send_notification(Notification::DamageTaken(amount))?;
        if amount > 0 {
            self.state.hp_loss_count += 1;
            // TODO: Lookup instead of linear pass?
            for card in self.state.cards_iter_mut() {
                if let Card::BloodForBlood = card.card {
                    card.cost_this_combat = card.cost_this_combat.saturating_sub(1);
                }
            }
        }
        self.player.decrease_hp(amount)
    }

    pub fn draw_cards(&mut self) -> Result<(), Error> {
        // Draw new cards
        let draw_count = 5;
        for i in 0..draw_count {
            if let Some(card) = self.state.draw_pile.pop() {
                self.player
                    .comms
                    .send_notification(Notification::CardDrawn(i, card.card))?;
                self.state.hand.push(card);
            } else {
                // Shuffle discard pile into draw pile
                self.player
                    .comms
                    .send_notification(Notification::ShufflingDiscardToDraw)?;
                self.shuffle_rng
                    .java_compat_shuffle(&mut self.state.discard_pile);
                self.state.draw_pile.append(&mut self.state.discard_pile);
                if let Some(card) = self.state.draw_pile.pop() {
                    self.player
                        .comms
                        .send_notification(Notification::CardDrawn(i, card.card))?;
                    self.state.hand.push(card);
                }
            }
        }
        Ok(())
    }

    pub fn add_to_discard_pile(&mut self, cards: &[Card]) -> Result<(), Error> {
        cards
            .iter()
            .map(|&card| CardInCombat::new(None, card))
            .for_each(|card| {
                self.state.discard_pile.push(card);
            });
        self.player
            .comms
            .send_notification(Notification::AddToDiscardPile(cards.to_vec()))
    }

    fn discard_hand(&mut self) -> Result<(), Error> {
        // Emulating the game's behavior
        while let Some(card) = self.state.hand.pop() {
            self.state.discard_pile.push(card);
        }
        self.player
            .comms
            .send_notification(Notification::HandDiscarded)
    }

    pub fn send_enemy_died(&self, index: EnemyIndex, enemy: Enemy) -> Result<(), Error> {
        self.player
            .comms
            .send_notification(Notification::EnemyDied(index, enemy))
    }

    pub fn choose_next_action(
        &mut self,
        enemies: &[Option<EnemyStatus>],
    ) -> Result<CombatAction, Error> {
        // TODO: drink a potion, discard a potion
        // TODO: check for unwinnable situations
        // TODO: Intent
        self.player
            .comms
            .send_notification(Notification::EnemyParty(enemies.to_vec()))?;
        self.player
            .comms
            .send_notification(Notification::Energy(self.state.energy))?;
        let playable_cards = self
            .state
            .hand
            .iter()
            .copied()
            .enumerate()
            .filter_map(|(hand_index, card)| {
                if card.cost_this_combat > self.state.energy {
                    None
                } else {
                    Some((hand_index, card))
                }
            })
            .collect::<Vec<_>>();

        match self.player.comms.choose_card_to_play(&playable_cards)? {
            Some(hand_index) => {
                self.card_just_played = Some(hand_index);
                let card = self.state.hand[hand_index].card;
                if card.requires_target() {
                    let enemy_index = self.player.comms.choose_enemy_to_target(enemies)?;
                    Ok(CombatAction::PlayCardAgainstEnemy(card, enemy_index))
                } else {
                    Ok(CombatAction::PlayCard(card))
                }
            }
            None => Ok(CombatAction::EndTurn),
        }
    }

    pub fn dispose_card_just_played(&mut self) -> Result<(), Error> {
        if let Some(hand_index) = self.card_just_played {
            let card_in_combat = self.state.hand.remove(hand_index);
            self.state.energy = self
                .state
                .energy
                .saturating_sub(card_in_combat.cost_this_combat);
            if card_in_combat.card.exhausts() {
                self.state.exhaust_pile.push(card_in_combat);
                self.player
                    .comms
                    .send_notification(Notification::CardExhausted(hand_index, card_in_combat.card))
            } else {
                self.state.discard_pile.push(card_in_combat);
                self.player
                    .comms
                    .send_notification(Notification::CardDiscarded(hand_index, card_in_combat.card))
            }
        } else {
            Ok(())
        }
    }

    pub fn gain_block(&mut self, amount: Block) -> Result<(), Error> {
        self.player
            .comms
            .send_notification(Notification::BlockGained(amount))?;
        self.state.block = self.state.block.saturating_add(amount);
        self.player
            .comms
            .send_notification(Notification::Block(self.state.block))
    }

    pub fn update_enemy_status(&self, index: EnemyIndex, status: EnemyStatus) -> Result<(), Error> {
        self.player
            .comms
            .send_notification(Notification::EnemyStatus(index, status))
    }

    pub fn is_dead(&self) -> bool {
        self.player.state.hp == 0
    }
}

#[cfg(test)]
mod tests {
    use std::sync::mpsc::channel;

    use crate::data::IRONCLAD;
    use crate::Seed;

    use super::*;

    #[test]
    fn test_blood_for_blood() {
        let (to_server, from_client) = channel();
        let (to_client, from_server) = channel();

        let mut player = Player::new(IRONCLAD, from_client, to_client);
        player.state.deck = vec![Card::BloodForBlood];
        let mut player_in_combat = PlayerInCombat::new(Seed::from(3).into(), &mut player);

        assert_eq!(
            4,
            player_in_combat
                .state
                .draw_pile
                .iter()
                .find(|card| card.card == Card::BloodForBlood)
                .unwrap()
                .cost_this_combat
        );
        player_in_combat.take_blockable_damage(5).unwrap();
        assert_eq!(
            3,
            player_in_combat
                .state
                .draw_pile
                .iter()
                .find(|card| card.card == Card::BloodForBlood)
                .unwrap()
                .cost_this_combat
        );
        player_in_combat.take_blockable_damage(5).unwrap();
        player_in_combat.take_blockable_damage(5).unwrap();
        player_in_combat.take_blockable_damage(5).unwrap();
        player_in_combat.take_blockable_damage(5).unwrap();
        player_in_combat.take_blockable_damage(5).unwrap();
        assert_eq!(
            0,
            player_in_combat
                .state
                .draw_pile
                .iter()
                .find(|card| card.card == Card::BloodForBlood)
                .unwrap()
                .cost_this_combat
        );

        drop(to_server);
        drop(from_server);
    }
}
