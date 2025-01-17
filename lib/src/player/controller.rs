use std::sync::mpsc::{Receiver, Sender};

use anyhow::Error;

use crate::data::{Card, Character, NeowBlessing, Potion, Relic};
use crate::rng::StsRandom;
use crate::{ColumnIndex, Effect, Gold, Hp, HpMax};

use super::combat::CombatController;
use super::comms::Comms;
use super::message::{MainScreenOption, PotionAction, StsMessage};
use super::state::PlayerState;

/// Encapsulates the state of the player in the game, e.g. HP, gold, deck, etc.
/// Also handles interactions with the player via the input_rx and output_tx channels, sending
/// messages to the player to prompt for decisions, following up with more questions when necessary.
#[derive(Debug)]
pub struct PlayerController {
    state: PlayerState,
    comms: Comms,
}

/// Some convenience methods for Player interaction.
impl PlayerController {
    pub fn new(
        character: &'static Character,
        from_client: Receiver<usize>,
        to_client: Sender<StsMessage>,
    ) -> Self {
        let state = PlayerState::new(character);
        let comms = Comms::new(from_client, to_client);
        Self { state, comms }
    }

    pub fn hp(&self) -> Hp {
        self.state.hp()
    }

    pub fn hp_max(&self) -> HpMax {
        self.state.hp_max()
    }

    pub fn gold(&self) -> Gold {
        self.state.gold()
    }

    pub fn increase_hp(&mut self, amount: Hp) -> Result<(), Error> {
        self.state.increase_hp(amount);
        self.comms.send_health_changed(self.state.health())
    }

    pub fn decrease_hp(&mut self, amount: Hp) -> Result<(), Error> {
        self.state.decrease_hp(amount);
        self.comms.send_health_changed(self.state.health())
    }

    pub fn increase_hp_max(&mut self, amount: HpMax) -> Result<(), Error> {
        self.state.increase_hp_max(amount);
        self.comms.send_health_changed(self.state.health())
    }

    pub fn decrease_hp_max(&mut self, amount: HpMax) -> Result<(), Error> {
        self.state.decrease_hp_max(amount);
        self.comms.send_health_changed(self.state.health())
    }

    pub fn increase_gold(&mut self, amount: u32) -> Result<(), Error> {
        self.state.increase_gold(amount);
        self.comms.send_gold_changed(self.state.gold())
    }

    pub fn decrease_gold(&mut self, amount: u32) -> Result<(), Error> {
        self.state.decrease_gold(amount);
        self.comms.send_gold_changed(self.state.gold())
    }

    pub fn send_full_player_state(&self) -> Result<(), Error> {
        self.comms.send_health_changed(self.state.health())?;
        self.comms.send_gold_changed(self.state.gold())?;
        self.comms.send_deck(self.state.deck())?;
        self.comms.send_potions(self.state.potions())?;
        self.comms.send_relics(self.state.relics())
    }

    pub fn send_map_string(&self, map_string: String) -> Result<(), anyhow::Error> {
        self.comms.send_map_string(map_string)
    }

    pub fn obtain_card(&mut self, card: Card) -> Result<(), Error> {
        self.state.obtain_card(card);
        self.comms.send_card_obtained(card)
    }

    pub fn choose_card_to_obtain(&mut self, cards: &[Card]) -> Result<(), Error> {
        if let Some(card) = self.comms.choose_card_to_obtain(cards, true)? {
            self.obtain_card(card)
        } else {
            Ok(())
        }
    }

    pub fn obtain_relic(&mut self, relic: Relic) -> Result<(), Error> {
        self.state.obtain_relic(relic);
        self.comms.send_relic_obtained(relic)
    }

    pub fn choose_neow_blessing(&self, blessings: &[NeowBlessing]) -> Result<NeowBlessing, Error> {
        self.comms.choose_neow_blessing(blessings)
    }

    pub fn climb_floor(&mut self, climb_options: &[ColumnIndex]) -> Result<ColumnIndex, Error> {
        loop {
            let mut potion_options = Vec::new();
            for (index, maybe_potion) in self.state.potions().iter().enumerate() {
                if let Some(potion) = maybe_potion {
                    potion_options.push(PotionAction::Discard(index, *potion));
                    if potion.can_drink_anywhere() {
                        potion_options.push(PotionAction::Drink(index, *potion));
                    }
                }
            }
            match self
                .comms
                .choose_main_screen_option(climb_options, &potion_options)?
            {
                MainScreenOption::ClimbFloor(index) => return Ok(index),
                MainScreenOption::Potion(PotionAction::Discard(index, _)) => {
                    self.state.discard_potion(index);
                    self.comms.send_potions(self.state.potions())?;
                }
                MainScreenOption::Potion(PotionAction::Drink(index, potion)) => {
                    self.state.discard_potion(index);
                    self.comms.send_potions(self.state.potions())?;
                    self.consume_potion(potion)?;
                }
            }
        }
    }

    fn consume_potion(&mut self, potion: Potion) -> Result<(), Error> {
        match potion {
            Potion::BloodPotion => self.increase_hp(self.hp_max() / 5),
            Potion::EntropicBrew => todo!(),
            Potion::FruitJuice => self.increase_hp_max(5),
            _ => unreachable!(),
        }
    }

    pub fn choose_potions_to_obtain(
        &mut self,
        potions: &[Potion],
        mut choice_count: usize,
    ) -> Result<(), Error> {
        let mut potion_vec = potions.to_vec();
        while self.state.has_potion_slot_available() && !potion_vec.is_empty() && choice_count > 0 {
            if let Some(potion) = self
                .comms
                .choose_potion_to_obtain(&potion_vec, choice_count == 1)?
            {
                self.state.obtain_potion(potion);
                self.comms.send_potions(self.state.potions())?;
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

    pub fn choose_card_to_remove(&mut self) -> Result<(), Error> {
        let deck_index = self.comms.choose_card_to_remove(self.state.deck())?;
        let card = self.state.remove_card(deck_index);
        self.comms.send_card_removed(card)?;
        self.comms.send_deck(self.state.deck())
    }

    pub fn start_combat(&mut self, shuffle_rng: StsRandom) -> CombatController {
        CombatController::new(shuffle_rng, &mut self.state, &mut self.comms)
    }

    pub fn interpret_effect(&self, effect: Effect) -> Effect {
        effect
    }
}
