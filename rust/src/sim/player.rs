use std::iter::once;
use std::sync::mpsc::{Receiver, Sender};

use anyhow::{anyhow, Error};

use crate::data::{Card, Character, NeowBlessing, Potion, Relic};

use super::message::{Choice, PlayerView, Prompt, StsMessage};

/// Encapsulates the state of the player in the game, e.g. HP, gold, deck, etc.
/// Also handles interactions with the player via the input_rx and output_tx channels.
pub struct Player {
    // Keeping the first few of these public for now to avoid having to write getters and setters,
    // but this might change if the need to manually call send_player_view() every time becomes
    // too onerous.
    pub hp: u32,
    pub hp_max: u32,
    pub gold: u32,
    pub relics: Vec<Relic>,
    pub deck: Vec<Card>,
    pub potions: Vec<Potion>,
    pub potion_slots: usize,

    // Communication channels
    input_rx: Receiver<usize>,
    output_tx: Sender<StsMessage>,
}

/// Some convenience methods for Player interaction.
impl Player {
    pub fn new(
        character: &'static Character,
        input_rx: Receiver<usize>,
        output_tx: Sender<StsMessage>,
    ) -> Self {
        let relics = vec![character.starting_relic];
        let deck = character.starting_deck.to_vec();
        let potion_slots = 3; // TODO: ascension
        let potions = Vec::with_capacity(potion_slots);
        Self {
            hp: character.starting_hp,
            hp_max: character.starting_hp,
            gold: 99,
            relics,
            deck,
            potions,
            potion_slots,
            input_rx,
            output_tx,
        }
    }

    pub fn send_map_string(&self, map_string: String) -> Result<(), anyhow::Error> {
        self.output_tx.send(StsMessage::Map(map_string))?;
        Ok(())
    }

    pub fn send_relics(&self) -> Result<(), anyhow::Error> {
        self.output_tx
            .send(StsMessage::Relics(self.relics.clone()))?;
        Ok(())
    }

    pub fn send_deck(&self) -> Result<(), anyhow::Error> {
        self.output_tx.send(StsMessage::Deck(self.deck.clone()))?;
        Ok(())
    }

    pub fn send_player_view(&self) -> Result<(), Error> {
        self.output_tx.send(StsMessage::View(PlayerView {
            hp: self.hp,
            hp_max: self.hp_max,
            gold: self.gold,
            potions: self.potions.clone(),
        }))?;
        Ok(())
    }

    pub fn choose_neow_blessing(
        &mut self,
        blessings: &[NeowBlessing; 4],
    ) -> Result<NeowBlessing, Error> {
        let choices = blessings
            .iter()
            .copied()
            .map(Choice::NeowBlessing)
            .collect::<Vec<_>>();
        self.output_tx
            .send(StsMessage::Choices(Prompt::NeowBlessing, choices.clone()))?;
        let choice_index = self.input_rx.recv()?;
        match choices.get(choice_index) {
            Some(Choice::NeowBlessing(blessing)) => Ok(*blessing),
            _ => Err(anyhow!("Invalid choice")),
        }
    }

    pub fn choose_movement_option(&mut self, options: Vec<usize>) -> Result<usize, Error> {
        let choices = options
            .iter()
            .map(|col| Choice::MoveTo(*col))
            .collect::<Vec<_>>();
        self.output_tx
            .send(StsMessage::Choices(Prompt::MoveTo, choices.clone()))?;
        let choice_index = self.input_rx.recv()?;
        match choices.get(choice_index) {
            Some(Choice::MoveTo(col)) => Ok(*col),
            _ => Err(anyhow!("Invalid choice")),
        }
    }

    pub fn choose_one_card(&mut self, card_vec: Vec<Card>) -> Result<(), Error> {
        let choices = card_vec
            .into_iter()
            .map(Choice::ObtainCard)
            .chain(once(Choice::Skip))
            .collect::<Vec<_>>();
        self.output_tx
            .send(StsMessage::Choices(Prompt::ChooseOne, choices.clone()))?;
        let choice_index = self.input_rx.recv()?;
        match choices.get(choice_index) {
            Some(Choice::ObtainCard(card)) => {
                self.deck.push(*card);
                Ok(())
            }
            Some(Choice::Skip) => Ok(()),
            _ => Err(anyhow!("Invalid choice")),
        }
    }

    pub fn choose_many_potions(&mut self, mut potions_vec: Vec<Potion>) -> Result<(), Error> {
        loop {
            let choices = potions_vec
                .clone()
                .into_iter()
                .map(Choice::ObtainPotion)
                .chain(once(Choice::Skip))
                .collect::<Vec<_>>();
            self.output_tx
                .send(StsMessage::Choices(Prompt::ChooseNext, choices.clone()))?;
            match choices.get(self.input_rx.recv()?) {
                Some(Choice::ObtainPotion(potion)) => {
                    self.potions.push(*potion);
                    potions_vec.retain(|&p| p != *potion);
                    if potions_vec.is_empty() || self.potions.len() >= self.potion_slots {
                        break;
                    }
                }
                Some(Choice::Skip) => break,
                _ => return Err(anyhow!("Invalid choice")),
            }
        }
        Ok(())
    }

    pub fn remove_one_card(&mut self) -> Result<(), Error> {
        let choices = self
            .deck
            .iter()
            .copied()
            .map(Choice::RemoveCard)
            .collect::<Vec<_>>();
        self.output_tx
            .send(StsMessage::Choices(Prompt::RemoveCard, choices.clone()))?;
        let choice_index = self.input_rx.recv()?;
        match choices.get(choice_index) {
            Some(Choice::RemoveCard(card)) => {
                // Remove the first occurrence of the card from the deck.
                self.deck.remove(
                    self.deck
                        .iter()
                        .position(|&c| c == *card)
                        .expect("Card not found"),
                );
                Ok(())
            }
            _ => Err(anyhow!("Invalid choice")),
        }
    }
}
