use std::iter::once;
use std::sync::mpsc::{Receiver, Sender};

use anyhow::{anyhow, Error};

use crate::data::{Card, Character, NeowBlessing, Potion, Relic};

use super::message::{Choice, Prompt, StsMessage};

/// Encapsulates the state of the player in the game, e.g. HP, gold, deck, etc.
/// Also handles interactions with the player via the input_rx and output_tx channels, sending
/// messages to the player to prompt for decisions, following up with more questions when necessary.
#[derive(Debug)]
pub struct Player {
    pub(crate) hp: u32,
    pub(crate) hp_max: u32,
    pub(crate) gold: u32,
    pub(crate) relics: Vec<Relic>,
    pub(crate) deck: Vec<Card>,
    pub(crate) potions: Vec<Option<Potion>>,

    // Communication channels
    pub(crate) input_rx: Receiver<usize>,
    pub(crate) output_tx: Sender<StsMessage>,
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
        let potions = [None; 3].to_vec();
        Self {
            hp: character.starting_hp,
            hp_max: character.starting_hp,
            gold: 99,
            relics,
            deck,
            potions,
            input_rx,
            output_tx,
        }
    }

    pub fn hp_max(&self) -> u32 {
        self.hp_max
    }

    pub fn hp(&self) -> u32 {
        self.hp
    }

    pub fn gold(&self) -> u32 {
        self.gold
    }

    pub fn take_damage(&mut self, amount: u32) -> Result<(), Error> {
        self.hp = self.hp.saturating_sub(amount);
        self.output_tx
            .send(StsMessage::HealthChanged(self.hp, self.hp_max))?;
        if self.hp == 0 {
            self.output_tx.send(StsMessage::GameOver(false))?;
            Err(anyhow!("Player died"))
        } else {
            Ok(())
        }
    }

    pub fn increase_hp_max(&mut self, amount: u32) -> Result<(), Error> {
        self.hp_max = self.hp_max.saturating_add(amount);
        self.hp = self.hp.saturating_add(amount);
        self.output_tx
            .send(StsMessage::HealthChanged(self.hp, self.hp_max))?;
        Ok(())
    }

    pub fn decrease_hp_max(&mut self, amount: u32) -> Result<(), Error> {
        self.hp_max = self.hp_max.saturating_sub(amount);
        self.hp = self.hp.min(self.hp_max);
        self.output_tx
            .send(StsMessage::HealthChanged(self.hp, self.hp_max))?;
        Ok(())
    }

    pub fn decrease_gold(&mut self, amount: u32) -> Result<(), Error> {
        self.gold = self.gold.saturating_sub(amount);
        self.output_tx.send(StsMessage::GoldChanged(self.gold))?;
        Ok(())
    }

    pub fn increase_gold(&mut self, amount: u32) -> Result<(), Error> {
        self.gold = self.gold.saturating_add(amount);
        self.output_tx.send(StsMessage::GoldChanged(self.gold))?;
        Ok(())
    }

    pub fn send_initial_state(&self) -> Result<(), Error> {
        self.output_tx.send(StsMessage::Deck(self.deck.clone()))?;
        self.output_tx
            .send(StsMessage::Potions(self.potions.clone()))?;
        self.output_tx
            .send(StsMessage::Relics(self.relics.clone()))?;
        Ok(())
    }

    pub fn send_map_string(&self, map_string: String) -> Result<(), anyhow::Error> {
        self.output_tx.send(StsMessage::Map(map_string))?;
        Ok(())
    }

    pub fn obtain_card(&mut self, card: Card) -> Result<(), Error> {
        self.deck.push(card);
        self.output_tx.send(StsMessage::CardObtained(card))?;
        Ok(())
    }

    pub fn obtain_relic(&mut self, relic: Relic) -> Result<(), Error> {
        self.relics.push(relic);
        self.output_tx.send(StsMessage::RelicObtained(relic))?;
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
            .send(StsMessage::Choices(Prompt::ChooseNeow, choices.clone()))?;
        let choice_index = self.input_rx.recv()?;
        match choices.get(choice_index) {
            Some(Choice::NeowBlessing(blessing)) => Ok(*blessing),
            _ => Err(anyhow!("Invalid choice")),
        }
    }

    pub fn choose_movement_option(&mut self, options: Vec<u8>) -> Result<u8, Error> {
        let choices = options
            .iter()
            .map(|col| Choice::ClimbFloor(*col))
            .collect::<Vec<_>>();
        self.output_tx
            .send(StsMessage::Choices(Prompt::ClimbFloor, choices.clone()))?;
        let choice_index = self.input_rx.recv()?;
        match choices.get(choice_index) {
            Some(Choice::ClimbFloor(col)) => Ok(*col),
            _ => Err(anyhow!("Invalid choice")),
        }
    }

    pub fn choose_card_to_obtain(&mut self, card_vec: Vec<Card>) -> Result<(), Error> {
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
                self.obtain_card(*card)?;
                Ok(())
            }
            Some(Choice::Skip) => Ok(()),
            _ => Err(anyhow!("Invalid choice")),
        }
    }

    pub fn choose_potions_to_obtain(&mut self, mut choices_vec: Vec<Potion>) -> Result<(), Error> {
        loop {
            let next_available_slot = self.potions.iter().position(Option::is_none);
            if let Some(slot) = next_available_slot {
                let choices = choices_vec
                    .clone()
                    .into_iter()
                    .map(Choice::ObtainPotion)
                    .chain(once(Choice::Skip))
                    .collect::<Vec<_>>();
                self.output_tx
                    .send(StsMessage::Choices(Prompt::ChooseNext, choices.clone()))?;
                let choice_index = self.input_rx.recv()?;
                match choices.get(choice_index) {
                    Some(Choice::ObtainPotion(potion)) => {
                        self.potions[slot] = Some(*potion);
                        self.output_tx
                            .send(StsMessage::PotionObtained(*potion, slot as u8))?;
                    }
                    Some(Choice::Skip) => break,
                    _ => return Err(anyhow!("Invalid choice")),
                }
                choices_vec.remove(choice_index);
            } else {
                // No available slots.
                break;
            }
        }
        self.output_tx
            .send(StsMessage::Potions(self.potions.clone()))?;
        Ok(())
    }

    pub fn choose_card_to_remove(&mut self) -> Result<(), Error> {
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
                let index = self
                    .deck
                    .iter()
                    .position(|&c| c == *card)
                    .expect("Card not found");
                self.deck.remove(index);
                self.output_tx.send(StsMessage::CardRemoved(*card))?;
                Ok(())
            }
            _ => Err(anyhow!("Invalid choice")),
        }
    }
}
