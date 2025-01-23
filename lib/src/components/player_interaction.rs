use std::iter::once;
use std::sync::mpsc::{Receiver, Sender};

use anyhow::Error;

use crate::data::{Card, NeowBlessing, Potion};
use crate::types::{ColumnIndex, DeckIndex, EnemyIndex, Gold, HandIndex};

use super::enemy_status::EnemyStatus;
use super::message::{Choice, PotionAction, Prompt, StsMessage};
use super::notification::Notification;
use super::CardInCombat;

#[derive(Clone, Debug)]
pub enum MainScreenAction {
    ClimbFloor(ColumnIndex),
    Potion(PotionAction),
}

#[derive(Clone, Debug)]
pub enum CombatRewardsAction {
    ObtainCard(Card),
    ObtainGold(Gold),
    ObtainPotion(Potion),
    SkipCombatRewards,
}

/// Handles all interactions with the player via the from_client and to_client channels, sending
/// messages to the player to prompt for decisions and returning the choices made by the player.
#[derive(Debug)]
pub struct PlayerInteraction {
    from_client: Receiver<usize>,
    to_client: Sender<StsMessage>,
}

impl PlayerInteraction {
    pub fn new(from_client: Receiver<usize>, to_client: Sender<StsMessage>) -> Self {
        Self {
            from_client,
            to_client,
        }
    }

    /// Prompts the user to choose a Neow blessing from the supplied list of blessings.
    /// The user must pick exactly one.
    pub fn choose_neow_blessing(&self, blessings: &[NeowBlessing]) -> Result<NeowBlessing, Error> {
        let choices = blessings
            .iter()
            .copied()
            .map(Choice::NeowBlessing)
            .collect::<Vec<_>>();
        match self.prompt_for_choice(Prompt::ChooseNeow, &choices)? {
            Choice::NeowBlessing(blessing) => Ok(*blessing),
            invalid => unreachable!("{:?}", invalid),
        }
    }

    /// Prompts the user to choose a column to enter on the row (floor) above their current room,
    /// or to use/discard a potion if available.
    pub fn choose_main_screen_action(
        &self,
        columns: &[ColumnIndex],
        potion_actions: &[PotionAction],
    ) -> Result<MainScreenAction, Error> {
        let choices = columns
            .iter()
            .copied()
            .map(Choice::ClimbFloor)
            .chain(potion_actions.iter().copied().map(Choice::PotionAction))
            .collect::<Vec<_>>();
        let prompt = if potion_actions.is_empty() {
            Prompt::ClimbFloor
        } else {
            Prompt::ClimbFloorHasPotion
        };
        match self.prompt_for_choice(prompt, &choices)? {
            Choice::ClimbFloor(column_index) => Ok(MainScreenAction::ClimbFloor(*column_index)),
            Choice::PotionAction(potion_action) => Ok(MainScreenAction::Potion(*potion_action)),
            invalid => unreachable!("{:?}", invalid),
        }
    }

    /// Prompts the user to choose a card from a list of cards or to skip the choice.
    /// If `one_only` is true, the expectation is that the user will be able to pick at most
    /// one card.
    pub fn choose_card_to_obtain(
        &self,
        cards: &[Card],
        one_only: bool,
    ) -> Result<Option<Card>, Error> {
        let choices = cards
            .iter()
            .copied()
            .map(Choice::ObtainCard)
            .chain(once(Choice::Skip))
            .collect::<Vec<_>>();
        match self.prompt_for_choice(
            if one_only {
                Prompt::ChooseOne
            } else {
                Prompt::ChooseNext
            },
            &choices,
        )? {
            Choice::ObtainCard(card) => Ok(Some(*card)),
            Choice::Skip => Ok(None),
            invalid => unreachable!("{:?}", invalid),
        }
    }

    /// Prompts the user to choose a card from a list of cards to remove from their deck.
    pub fn choose_card_to_remove(&self, deck: &[Card]) -> Result<DeckIndex, Error> {
        let choices = deck
            .iter()
            .copied()
            .enumerate()
            .map(|(card_index, card)| Choice::RemoveCard(card_index, card))
            .collect::<Vec<_>>();
        match self.prompt_for_choice(Prompt::RemoveCard, &choices)? {
            Choice::RemoveCard(card_index, _) => Ok(*card_index),
            invalid => unreachable!("{:?}", invalid),
        }
    }

    /// Prompts the user to choose a card from their hand to play from the supplied list of
    /// possibly non-consecutive hand indexes (holes may be caused by unplayable or too-expensive
    /// cards).
    pub fn choose_card_to_play(
        &self,
        playable_cards: &[(HandIndex, CardInCombat)],
    ) -> Result<Option<HandIndex>, Error> {
        let choices = playable_cards
            .iter()
            .copied()
            .map(|(index, card)| Choice::PlayCardFromHand(index, card.card))
            .chain(once(Choice::EndTurn))
            .collect::<Vec<_>>();
        match self.prompt_for_choice(Prompt::CombatAction, &choices)? {
            Choice::PlayCardFromHand(hand_index, _) => Ok(Some(*hand_index)),
            Choice::EndTurn => Ok(None),
            invalid => unreachable!("{:?}", invalid),
        }
    }

    /// Prompts the user to choose one of the available combat rewards.
    pub fn choose_combat_reward(
        &self,
        maybe_gold: Option<Gold>,
        maybe_potion: Option<Potion>,
        cards: &[Card],
    ) -> Result<CombatRewardsAction, Error> {
        let choices = maybe_gold
            .into_iter()
            .map(Choice::ObtainGold)
            .chain(maybe_potion.into_iter().map(Choice::ObtainPotion))
            .chain(cards.iter().copied().map(Choice::ObtainCard))
            .collect::<Vec<_>>();
        match self.prompt_for_choice(
            if maybe_gold.is_some() {
                Prompt::ChooseNext
            } else {
                Prompt::ChooseOne
            },
            &choices,
        )? {
            Choice::ObtainGold(gold) => Ok(CombatRewardsAction::ObtainGold(*gold)),
            Choice::ObtainPotion(potion) => Ok(CombatRewardsAction::ObtainPotion(*potion)),
            Choice::ObtainCard(card) => Ok(CombatRewardsAction::ObtainCard(*card)),
            invalid => unreachable!("{:?}", invalid),
        }
    }

    /// Prompts the user to choose an enemy to target for their card or potion effect.
    pub fn choose_enemy_to_target(
        &self,
        enemies: &[Option<EnemyStatus>],
    ) -> Result<EnemyIndex, Error> {
        let choices = enemies
            .iter()
            .enumerate()
            .filter_map(|(index, maybe_enemy)| {
                maybe_enemy
                    .as_ref()
                    .map(|enemy| Choice::TargetEnemy(index, enemy.enemy_type))
            })
            .collect::<Vec<_>>();
        match self.prompt_for_choice(Prompt::TargetEnemy, &choices)? {
            Choice::TargetEnemy(enemy_index, _) => Ok(*enemy_index),
            invalid => unreachable!("{:?}", invalid),
        }
    }

    /// Prompts the user to choose a potion from a list of potions or to skip the choice.
    /// If `one_only` is true, the expectation is that the user will be able to pick at most
    /// one potion. Returns the index of the potion chosen.
    pub fn choose_potion_to_obtain(
        &mut self,
        potions: &[Potion],
        one_only: bool,
    ) -> Result<Option<Potion>, Error> {
        // TODO: Allow discarding and drinking potions here too.
        let choices = potions
            .iter()
            .copied()
            .map(Choice::ObtainPotion)
            .chain(once(Choice::Skip))
            .collect::<Vec<_>>();
        match self.prompt_for_choice(
            if one_only {
                Prompt::ChooseOne
            } else {
                Prompt::ChooseNext
            },
            &choices,
        )? {
            Choice::ObtainPotion(potion) => Ok(Some(*potion)),
            Choice::Skip => Ok(None),
            invalid => unreachable!("{:?}", invalid),
        }
    }

    /// Sends the supplied notification to the user.
    pub fn send_notification(&self, notification: Notification) -> Result<(), Error> {
        self.to_client
            .send(StsMessage::Notification(notification))?;
        Ok(())
    }

    pub fn send_game_over(&self, result: bool) -> Result<(), Error> {
        self.to_client.send(StsMessage::GameOver(result))?;
        Ok(())
    }

    /// Internal helper function to prompt the user to choose one of the supplied choices.
    /// Annoyingly repeats the prompt until the user makes a valid choice.
    pub fn prompt_for_choice<'a>(
        &self,
        prompt: Prompt,
        choices: &'a [Choice],
    ) -> Result<&'a Choice, Error> {
        loop {
            self.to_client
                .send(StsMessage::Choices(prompt.clone(), choices.to_vec()))?;
            let choice_index = self.from_client.recv()?;
            if let Some(choice) = choices.get(choice_index) {
                return Ok(choice);
            }
        }
    }
}
