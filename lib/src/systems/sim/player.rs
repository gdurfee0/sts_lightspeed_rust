use std::sync::mpsc::{Receiver, Sender};

use anyhow::Error;

use crate::components::{Notification, PlayerInteraction, PlayerState, PotionAction, StsMessage};
use crate::data::{Card, Character, Potion, Relic};
use crate::types::{ColumnIndex, DeckIndex, Gold, Hp, HpMax};
use crate::{Choice, Prompt};

/// Encapsulates the state of the player in the game, e.g. HP, gold, deck, etc.
/// Also handles interactions with the player via the input_rx and output_tx channels, sending
/// messages to the player to prompt for decisions, following up with more questions when necessary.
#[derive(Debug)]
pub struct Player {
    pub state: PlayerState,
    pub comms: PlayerInteraction,
}

impl Player {
    pub fn new(
        character: &'static Character,
        from_client: Receiver<usize>,
        to_client: Sender<StsMessage>,
    ) -> Self {
        let state = PlayerState::new(character);
        let comms = PlayerInteraction::new(from_client, to_client);
        Self { state, comms }
    }

    pub fn increase_hp(&mut self, amount: Hp) -> Result<(), Error> {
        self.state.hp = self.state.hp.saturating_add(amount).min(self.state.hp_max);
        self.comms
            .send_notification(Notification::Health(self.state.health()))
    }

    pub fn decrease_hp(&mut self, amount: Hp) -> Result<(), Error> {
        self.state.hp = self.state.hp.saturating_sub(amount);
        self.comms
            .send_notification(Notification::Health(self.state.health()))
    }

    pub fn increase_hp_max(&mut self, amount: HpMax) -> Result<(), Error> {
        self.state.hp_max = self.state.hp_max.saturating_add(amount);
        self.state.hp = self.state.hp.saturating_add(amount);
        self.comms
            .send_notification(Notification::Health(self.state.health()))
    }

    pub fn decrease_hp_max(&mut self, amount: HpMax) -> Result<(), Error> {
        self.state.hp_max = self.state.hp_max.saturating_sub(amount);
        self.state.hp = self.state.hp.min(self.state.hp_max);
        self.comms
            .send_notification(Notification::Health(self.state.health()))
    }

    pub fn increase_gold(&mut self, amount: u32) -> Result<(), Error> {
        self.state.gold = self.state.gold.saturating_add(amount);
        self.comms
            .send_notification(Notification::Gold(self.state.gold))
    }

    pub fn decrease_gold(&mut self, amount: u32) -> Result<(), Error> {
        self.state.gold = self.state.gold.saturating_sub(amount);
        self.comms
            .send_notification(Notification::Gold(self.state.gold))
    }

    pub fn send_full_player_state(&self) -> Result<(), Error> {
        self.comms
            .send_notification(Notification::Health(self.state.health()))?;
        self.comms
            .send_notification(Notification::Gold(self.state.gold))?;
        self.comms
            .send_notification(Notification::Deck(self.state.deck.clone()))?;
        self.comms
            .send_notification(Notification::Potions(self.state.potions.clone()))?;
        self.comms
            .send_notification(Notification::Relics(self.state.relics.clone()))
    }

    pub fn obtain_card(&mut self, card: Card) -> Result<(), Error> {
        self.state.deck.push(card);
        self.comms
            .send_notification(Notification::CardObtained(card))
    }

    pub fn obtain_relic(&mut self, relic: Relic) -> Result<(), Error> {
        self.state.relics.push(relic);
        self.comms
            .send_notification(Notification::RelicObtained(relic))
    }

    pub fn extend_with_potion_choices(&self, choices: &mut Vec<Choice>, in_combat: bool) -> bool {
        let mut has_potion = false;
        for (index, maybe_potion) in self.state.potions.iter().enumerate() {
            if let Some(potion) = maybe_potion {
                choices.push(Choice::ExpendPotion(PotionAction::Discard(index, *potion)));
                if in_combat || potion.can_drink_anywhere() {
                    choices.push(Choice::ExpendPotion(PotionAction::Drink(index, *potion)));
                }
                has_potion = true;
            }
        }
        has_potion
    }

    pub fn climb_floor(&mut self, climb_options: &[ColumnIndex]) -> Result<ColumnIndex, Error> {
        loop {
            let mut choices = climb_options
                .iter()
                .copied()
                .map(Choice::ClimbFloor)
                .collect();
            let prompt = if self.extend_with_potion_choices(&mut choices, false) {
                Prompt::ClimbFloorHasPotion
            } else {
                Prompt::ClimbFloor
            };
            match self.comms.prompt_for_choice(prompt, &choices)? {
                Choice::ClimbFloor(column_index) => return Ok(*column_index),
                Choice::ExpendPotion(potion_action) => self.expend_potion(potion_action)?,
                invalid => unreachable!("{:?}", invalid),
            }
        }
    }

    pub fn expend_potion(&mut self, potion_action: &PotionAction) -> Result<(), Error> {
        match potion_action {
            PotionAction::Discard(potion_index, _) => {
                self.state.potions[*potion_index] = None;
            }
            PotionAction::Drink(potion_index, Potion::BloodPotion) => {
                self.state.potions[*potion_index] = None;
                self.increase_hp(self.state.hp_max / 5);
            }
            PotionAction::Drink(_, Potion::EntropicBrew) => todo!(),
            PotionAction::Drink(potion_index, Potion::FruitJuice) => {
                self.state.potions[*potion_index] = None;
                self.increase_hp_max(5);
            }
            invalid => unreachable!("{:?} is not a consumable potion outside of combat", invalid),
        }
        self.comms
            .send_notification(Notification::Potions(self.state.potions.to_vec()))
    }

    pub fn choose_potions_to_obtain(
        &mut self,
        potions: &[Potion],
        mut choice_count: usize,
    ) -> Result<(), Error> {
        let mut incoming_potion_vec = potions.to_vec();
        while !incoming_potion_vec.is_empty() && choice_count > 0 {
            let mut choices = vec![];
            if self.state.has_potion_slot_available() {
                choices.extend(
                    incoming_potion_vec
                        .iter()
                        .copied()
                        .map(Choice::ObtainPotion),
                );
            }
            let _ = self.extend_with_potion_choices(&mut choices, false);
            choices.push(Choice::Skip);
            match self.comms.prompt_for_choice(
                if choice_count > 1 {
                    Prompt::ChooseNext
                } else {
                    Prompt::ChooseOne
                },
                &choices,
            )? {
                Choice::ExpendPotion(potion_action) => self.expend_potion(potion_action)?,
                Choice::ObtainPotion(potion) => {
                    *self
                        .state
                        .potions
                        .iter_mut()
                        .find(|p| p.is_none())
                        .expect("Just checked that potion slots are available") = Some(*potion);
                    let potion_index = incoming_potion_vec
                        .iter()
                        .position(|p| *p == *potion)
                        .expect("Potion not found");
                    incoming_potion_vec.remove(potion_index);
                    choice_count -= 1;
                }
                Choice::Skip => break,
                invalid => unreachable!("{:?}", invalid),
            }
        }
        Ok(())
    }

    pub fn choose_combat_rewards(
        &mut self,
        gold: Gold,
        mut maybe_potion: Option<Potion>,
        cards: &[Card],
    ) -> Result<(), Error> {
        let mut gold_option: Option<Gold> = Some(gold);
        let mut card_vec = cards.to_vec();
        let mut cards_left_to_choose = 1;
        while gold_option.is_some()
            || (maybe_potion.is_some() && self.state.has_potion_slot_available())
            || (!card_vec.is_empty() && cards_left_to_choose > 0)
        {
            let mut choices = vec![];
            if let Some(gold) = gold_option {
                choices.push(Choice::ObtainGold(gold));
            }
            if let Some(potion) = maybe_potion {
                choices.push(Choice::ObtainPotion(potion));
            }
            if cards_left_to_choose > 0 {
                choices.extend(card_vec.iter().copied().map(Choice::ObtainCard));
            }
            self.extend_with_potion_choices(&mut choices, false);
            choices.push(Choice::Skip);
            match self.comms.prompt_for_choice(Prompt::ChooseNext, &choices)? {
                Choice::ObtainGold(gold) => {
                    self.increase_gold(*gold)?;
                    gold_option = None;
                }
                Choice::ObtainPotion(potion) => {
                    *self
                        .state
                        .potions
                        .iter_mut()
                        .find(|p| p.is_none())
                        .expect("Just checked that potion slots are available") = Some(*potion);
                    self.comms
                        .send_notification(Notification::Potions(self.state.potions.to_vec()))?;
                    maybe_potion = None;
                }
                Choice::ObtainCard(card) => {
                    self.obtain_card(*card)?;
                    let reward_index = card_vec
                        .iter()
                        .position(|c| *c == *card)
                        .expect("Card not found");
                    card_vec.remove(reward_index);
                    cards_left_to_choose -= 1;
                }
                Choice::ExpendPotion(potion_action) => self.expend_potion(potion_action)?,
                Choice::Skip => break,
                invalid => unreachable!("{:?}", invalid),
            }
        }
        Ok(())
    }

    pub fn remove_card(&mut self, deck_index: DeckIndex) -> Result<(), Error> {
        let card = self.state.deck.remove(deck_index);
        self.comms
            .send_notification(Notification::CardRemoved(card))?;
        self.comms
            .send_notification(Notification::Deck(self.state.deck.to_vec()))
    }
}
