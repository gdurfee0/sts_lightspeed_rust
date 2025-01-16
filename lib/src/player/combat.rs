use anyhow::Error;

use crate::data::Card;
use crate::enemy::{EnemyStatus, EnemyType};
use crate::rng::StsRandom;
use crate::{Buff, Debuff, EnemyIndex, Energy, HandIndex, StackCount};

use super::action::{Action, CardAction, Target};
use super::comms::Comms;
use super::state::PlayerState;

/// Encapsulates the state of the player in the game, e.g. HP, gold, deck, etc.
/// Also handles interactions with the player via the input_rx and output_tx channels, sending
/// messages to the player to prompt for decisions, following up with more questions when necessary.
#[derive(Debug)]
pub struct CombatController<'a> {
    shuffle_rng: StsRandom,
    energy: Energy,
    buffs: Vec<(Buff, StackCount)>,
    debuffs: Vec<(Debuff, StackCount)>,
    hand: Vec<Card>,
    card_just_played: Option<HandIndex>,
    draw_pile: Vec<Card>,
    discard_pile: Vec<Card>,
    exhaust_pile: Vec<Card>,

    state: &'a mut PlayerState,
    comms: &'a mut Comms,
}

impl<'a> CombatController<'a> {
    pub fn new(
        mut shuffle_rng: StsRandom,
        state: &'a mut PlayerState,
        comms: &'a mut Comms,
    ) -> Self {
        let hand = Vec::new();
        let mut draw_pile = state.deck().to_vec();
        shuffle_rng.java_compat_shuffle(&mut draw_pile);
        let discard_pile = Vec::new();
        let exhaust_pile = Vec::new();
        let debuffs = Vec::new();
        Self {
            shuffle_rng,
            energy: 3,
            buffs: Vec::new(),
            debuffs,
            hand,
            card_just_played: None,
            draw_pile,
            discard_pile,
            exhaust_pile,
            state,
            comms,
        }
    }

    pub fn start_turn(&mut self) -> Result<(), Error> {
        // Reset energy
        self.energy = 3;

        // Draw cards
        self.draw_cards()?;

        // Tick down debuffs
        for (_, stacks) in self.debuffs.iter_mut() {
            *stacks = (*stacks - 1).max(0);
        }
        self.debuffs.retain(|(_, stacks)| *stacks > 0);
        self.comms.send_debuffs(&self.debuffs)?;

        // Apply any other start-of-turn effects
        Ok(())
    }

    pub fn draw_cards(&mut self) -> Result<(), Error> {
        // Draw new cards
        let draw_count = 5;
        for i in 0..draw_count {
            if let Some(card) = self.draw_pile.pop() {
                self.hand.push(card);
                self.comms.send_card_drawn(i, card)?;
            } else {
                // Shuffle discard pile into draw pile
                self.comms.send_shuffling_discard_to_draw()?;
                self.shuffle_rng.java_compat_shuffle(&mut self.discard_pile);
                self.draw_pile.append(&mut self.discard_pile);
                if let Some(card) = self.draw_pile.pop() {
                    self.hand.push(card);
                    self.comms.send_card_drawn(i, card)?;
                }
            }
        }
        Ok(())
    }

    pub fn discard_hand(&mut self) -> Result<(), Error> {
        // Emulating the game's behavior
        while let Some(card) = self.hand.pop() {
            self.discard_pile.push(card);
        }
        self.comms.send_hand_discarded()
    }

    pub fn enemy_died(&self, index: EnemyIndex, enemy_type: EnemyType) -> Result<(), Error> {
        self.comms.send_enemy_died(index, enemy_type)
    }

    pub fn choose_next_action(&mut self, enemies: &[EnemyType]) -> Result<Action, Error> {
        // TODO: drink a potion, discard a potion
        // TODO: check for unwinnable situations
        match self.comms.choose_card_to_play(&self.hand)? {
            Some(hand_index) => {
                self.card_just_played = Some(hand_index);
                let card_action = CardAction::for_card(self.hand[hand_index]);
                match card_action.target {
                    Target::OneEnemy => {
                        let enemy_index = self.comms.choose_enemy_to_target(enemies)?;
                        Ok(Action::PlayCardAgainstEnemy(card_action, enemy_index))
                    }
                    Target::AllEnemies => Ok(Action::PlayCard(card_action)),
                    Target::Player => Ok(Action::PlayCard(card_action)),
                }
            }
            None => Ok(Action::EndTurn),
        }
    }

    pub fn discard_card_just_played(&mut self) -> Result<(), Error> {
        if let Some(hand_index) = self.card_just_played {
            let card = self.hand.remove(hand_index);
            self.discard_pile.push(card);
            self.comms.send_card_discarded(hand_index, card)?;
        }
        Ok(())
    }

    pub fn apply_debuff(&mut self, debuff: Debuff, stacks: StackCount) -> Result<(), Error> {
        if let Some((_, c)) = self.debuffs.iter_mut().find(|(d, _)| *d == debuff) {
            *c += stacks;
        } else {
            self.debuffs.push((debuff, stacks));
        }
        self.comms.send_debuffs(&self.debuffs)
    }

    pub fn update_enemy_status(&self, index: EnemyIndex, status: EnemyStatus) -> Result<(), Error> {
        self.comms.send_enemy_status(index, status)
    }
}
