use std::sync::mpsc::{Receiver, Sender};

use anyhow::Error;

use crate::components::{
    CombatRewardsAction, MainScreenAction, Notification, PlayerInteraction, PlayerState,
    PotionAction, StsMessage,
};
use crate::data::{Card, Character, NeowBlessing, Potion, Relic};
use crate::types::{ColumnIndex, Gold, Hp, HpMax};

/// Encapsulates the state of the player in the game, e.g. HP, gold, deck, etc.
/// Also handles interactions with the player via the input_rx and output_tx channels, sending
/// messages to the player to prompt for decisions, following up with more questions when necessary.
#[derive(Debug)]
pub struct Player {
    pub state: PlayerState,
    pub comms: PlayerInteraction,
}

/// Some convenience methods for Player interaction.
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

    pub fn send_game_over(&self) -> Result<(), Error> {
        self.comms.send_game_over(self.state.hp > 0)
    }

    pub fn send_map_string(&self, map_string: String) -> Result<(), anyhow::Error> {
        self.comms.send_notification(Notification::Map(map_string))
    }

    pub fn obtain_card(&mut self, card: Card) -> Result<(), Error> {
        self.state.deck.push(card);
        self.comms
            .send_notification(Notification::CardObtained(card))
    }

    pub fn choose_card_to_obtain(&mut self, cards: &[Card]) -> Result<(), Error> {
        if let Some(card) = self.comms.choose_card_to_obtain(cards, true)? {
            self.obtain_card(card)
        } else {
            Ok(())
        }
    }

    pub fn obtain_relic(&mut self, relic: Relic) -> Result<(), Error> {
        self.state.relics.push(relic);
        self.comms
            .send_notification(Notification::RelicObtained(relic))
    }

    pub fn choose_neow_blessing(&self, blessings: &[NeowBlessing]) -> Result<NeowBlessing, Error> {
        self.comms.choose_neow_blessing(blessings)
    }

    pub fn climb_floor(&mut self, climb_options: &[ColumnIndex]) -> Result<ColumnIndex, Error> {
        loop {
            let mut potion_options = Vec::new();
            for (index, maybe_potion) in self.state.potions.iter().enumerate() {
                if let Some(potion) = maybe_potion {
                    potion_options.push(PotionAction::Discard(index, *potion));
                    if potion.can_drink_anywhere() {
                        potion_options.push(PotionAction::Drink(index, *potion));
                    }
                }
            }
            match self
                .comms
                .choose_main_screen_action(climb_options, &potion_options)?
            {
                MainScreenAction::ClimbFloor(index) => return Ok(index),
                MainScreenAction::Potion(PotionAction::Discard(index, _)) => {
                    self.state.potions[index] = None;
                    self.comms
                        .send_notification(Notification::Potions(self.state.potions.to_vec()))?;
                }
                MainScreenAction::Potion(PotionAction::Drink(index, potion)) => {
                    self.state.potions[index] = None;
                    self.comms
                        .send_notification(Notification::Potions(self.state.potions.to_vec()))?;
                    self.consume_potion(potion)?;
                }
            }
        }
    }

    fn consume_potion(&mut self, potion: Potion) -> Result<(), Error> {
        match potion {
            Potion::BloodPotion => self.increase_hp(self.state.hp_max / 5),
            Potion::EntropicBrew => todo!(),
            Potion::FruitJuice => self.increase_hp_max(5),
            invalid => unreachable!("{:?} is not a consumable potion outside of combat", invalid),
        }
    }

    pub fn choose_potions_to_obtain(
        &mut self,
        potions: &[Potion],
        mut choice_count: usize,
    ) -> Result<(), Error> {
        let mut potion_vec = potions.to_vec();
        // TODO: Allow discarding potions & drinking the main-screen drinkable ones here
        while self.state.has_potion_slot_available() && !potion_vec.is_empty() && choice_count > 0 {
            if let Some(potion) = self
                .comms
                .choose_potion_to_obtain(&potion_vec, choice_count == 1)?
            {
                *self
                    .state
                    .potions
                    .iter_mut()
                    .find(|p| p.is_none())
                    .expect("Just checked that potion slots are available") = Some(potion);
                self.comms
                    .send_notification(Notification::Potions(self.state.potions.to_vec()))?;
                let potion_index = potion_vec
                    .iter()
                    .position(|p| *p == potion)
                    .expect("Potion not found");
                potion_vec.remove(potion_index);
                choice_count -= 1;
            } else {
                break;
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
        // TODO: Allow discarding potions & drinking the main-screen drinkable ones here
        while gold_option.is_some()
            || (maybe_potion.is_some() && self.state.has_potion_slot_available())
            || (!card_vec.is_empty() && cards_left_to_choose > 0)
        {
            if cards_left_to_choose == 0 {
                card_vec.clear();
            }
            if !self.state.has_potion_slot_available() {
                maybe_potion = None;
            }
            match self
                .comms
                .choose_combat_reward(gold_option, maybe_potion, &card_vec)?
            {
                CombatRewardsAction::ObtainGold(g) => {
                    self.increase_gold(g)?;
                    gold_option = None;
                }
                CombatRewardsAction::ObtainPotion(potion) => {
                    *self
                        .state
                        .potions
                        .iter_mut()
                        .find(|p| p.is_none())
                        .expect("Just checked that potion slots are available") = Some(potion);
                    self.comms
                        .send_notification(Notification::Potions(self.state.potions.to_vec()))?;
                    maybe_potion = None;
                }
                CombatRewardsAction::ObtainCard(card) => {
                    self.obtain_card(card)?;
                    let reward_index = card_vec
                        .iter()
                        .position(|c| *c == card)
                        .expect("Card not found");
                    card_vec.remove(reward_index);
                    cards_left_to_choose -= 1;
                }
                CombatRewardsAction::SkipCombatRewards => break,
                action => unreachable!("Unexpected action {:?}", action),
            }
        }
        Ok(())
    }

    pub fn choose_card_to_remove(&mut self) -> Result<(), Error> {
        let deck_index = self.comms.choose_card_to_remove(&self.state.deck)?;
        let card = self.state.deck.remove(deck_index);
        self.comms
            .send_notification(Notification::CardRemoved(card))?;
        self.comms
            .send_notification(Notification::Deck(self.state.deck.to_vec()))
    }
}
