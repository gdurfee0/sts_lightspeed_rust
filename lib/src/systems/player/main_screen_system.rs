use anyhow::Error;

use crate::components::{Choice, Interaction, PlayerPersistentState, Prompt};
use crate::data::{Card, Potion};
use crate::types::{ColumnIndex, Gold};

use super::deck_system::DeckSystem;
use super::gold_system::GoldSystem;
use super::health_system::HealthSystem;
use super::potion_system::PotionSystem;
use super::relic_system::RelicSystem;

pub struct MainScreenSystem<'a, I: Interaction> {
    comms: &'a I,
    deck_system: &'a DeckSystem<'a, I>,
    gold_system: &'a GoldSystem<'a, I>,
    health_system: &'a HealthSystem<'a, I>,
    potion_system: &'a PotionSystem<'a, I>,
    relic_system: &'a RelicSystem<'a, I>,
}

impl<'a, I: Interaction> MainScreenSystem<'a, I> {
    pub fn new(
        comms: &'a I,
        deck_system: &'a DeckSystem<'a, I>,
        gold_system: &'a GoldSystem<'a, I>,
        health_system: &'a HealthSystem<'a, I>,
        potion_system: &'a PotionSystem<'a, I>,
        relic_system: &'a RelicSystem<'a, I>,
    ) -> Self {
        Self {
            comms,
            deck_system,
            gold_system,
            health_system,
            potion_system,
            relic_system,
        }
    }

    pub fn notify_player(
        &self,
        player_persistent_state: &PlayerPersistentState,
    ) -> Result<(), Error> {
        self.deck_system
            .notify_player(&player_persistent_state.deck)?;
        self.gold_system
            .notify_player(&player_persistent_state.gold)?;
        self.health_system
            .notify_player(&player_persistent_state.health)?;
        self.potion_system
            .notify_player(&player_persistent_state.potions)?;
        self.relic_system
            .notify_player(&player_persistent_state.relics)
    }

    pub fn choose_combat_rewards(
        &self,
        player_persistent_state: &mut PlayerPersistentState,
        available_gold: Gold,
        mut maybe_potion: Option<Potion>,
        available_cards: &[Card],
    ) -> Result<(), Error> {
        let mut maybe_gold: Option<Gold> = Some(available_gold);
        let mut available_card_vec = available_cards.to_vec();
        let mut cards_left_to_choose = 1;
        while maybe_gold.is_some()
            || (maybe_potion.is_some()
                && self
                    .potion_system
                    .has_potion_slot_available(&player_persistent_state.potions))
            || (!available_card_vec.is_empty() && cards_left_to_choose > 0)
        {
            let mut choices = Vec::with_capacity(available_card_vec.len() + 2);
            if let Some(gold_to_obtain) = maybe_gold {
                choices.push(Choice::ObtainGold(gold_to_obtain));
            }
            if let Some(potion_to_obtain) = maybe_potion {
                choices.push(Choice::ObtainPotion(potion_to_obtain));
            }
            if cards_left_to_choose > 0 {
                choices.extend(
                    available_card_vec.iter().copied().enumerate().map(
                        |(card_reward_index, card)| Choice::ObtainCard(card_reward_index, card),
                    ),
                );
            }
            self.potion_system.extend_with_potion_actions(
                &player_persistent_state.potions,
                false,
                &mut choices,
            );

            choices.push(Choice::Skip);
            match self.comms.prompt_for_choice(Prompt::ChooseNext, &choices)? {
                Choice::ExpendPotion(potion_action) => {
                    self.potion_system.expend_potion_out_of_combat(
                        &mut player_persistent_state.potions,
                        &potion_action,
                        &mut player_persistent_state.health,
                    )?
                }
                Choice::ObtainCard(card_reward_index, _) => {
                    let card_to_obtain = available_card_vec.remove(*card_reward_index);
                    self.deck_system
                        .obtain_card(&mut player_persistent_state.deck, card_to_obtain)?;
                    cards_left_to_choose -= 1;
                }
                Choice::ObtainGold(gold_to_obtain) => {
                    self.gold_system.increase_gold(
                        &mut player_persistent_state.gold,
                        *gold_to_obtain,
                        &player_persistent_state.relics,
                        &mut player_persistent_state.health,
                    )?;
                    maybe_gold = None;
                }
                Choice::ObtainPotion(potion_to_obtain) => {
                    self.potion_system
                        .obtain_potion(&mut player_persistent_state.potions, *potion_to_obtain)?;
                    maybe_potion = None;
                }
                Choice::Skip => break,
                invalid => unreachable!("{:?}", invalid),
            }
        }
        Ok(())
    }
}
