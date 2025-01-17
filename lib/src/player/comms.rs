use std::iter::once;
use std::sync::mpsc::{Receiver, Sender};

use anyhow::{anyhow, Error};

use crate::data::{Card, NeowBlessing, Potion};
use crate::enemy::{Enemy, EnemyStatus, EnemyType};
use crate::{
    Block, ColumnIndex, Debuff, DeckIndex, EnemyIndex, Energy, Gold, HandIndex, Health, Hp, Relic,
    StackCount,
};

use super::action::EnemyEffectChain;
use super::message::{CardPlay, Choice, MainScreenOption, PotionAction, Prompt, StsMessage};

/// Handles all interactions with the player via the from_client and to_client channels, sending
/// messages to the player to prompt for decisions and returning the choices made by the player.
#[derive(Debug)]
pub struct Comms {
    from_client: Receiver<usize>,
    to_client: Sender<StsMessage>,
}

impl Comms {
    pub fn new(from_client: Receiver<usize>, to_client: Sender<StsMessage>) -> Self {
        Self {
            from_client,
            to_client,
        }
    }

    pub fn choose_neow_blessing(&self, blessings: &[NeowBlessing]) -> Result<NeowBlessing, Error> {
        let choices = blessings
            .iter()
            .copied()
            .map(Choice::NeowBlessing)
            .collect::<Vec<_>>();
        match self.prompt_for_choice(Prompt::ChooseNeow, choices)? {
            Choice::NeowBlessing(blessing) => Ok(blessing),
            _ => unreachable!(),
        }
    }

    /// Prompts the user to choose a column to enter on the row above their current row.
    pub fn choose_main_screen_option(
        &self,
        columns: &[ColumnIndex],
        potion_actions: &[PotionAction],
    ) -> Result<MainScreenOption, Error> {
        let choices = columns
            .iter()
            .map(|column_index| Choice::ClimbFloor(*column_index))
            .chain(potion_actions.iter().copied().map(Choice::PotionAction))
            .collect::<Vec<_>>();
        let prompt = if potion_actions.is_empty() {
            Prompt::ClimbFloor
        } else {
            Prompt::ClimbFloorHasPotion
        };
        match self.prompt_for_choice(prompt, choices)? {
            Choice::ClimbFloor(column_index) => Ok(MainScreenOption::ClimbFloor(column_index)),
            Choice::PotionAction(potion_action) => Ok(MainScreenOption::Potion(potion_action)),
            _ => unreachable!(),
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
            choices,
        )? {
            Choice::ObtainCard(card) => Ok(Some(card)),
            Choice::Skip => Ok(None),
            _ => unreachable!(),
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
            choices,
        )? {
            Choice::ObtainPotion(potion) => Ok(Some(potion)),
            Choice::Skip => Ok(None),
            _ => unreachable!(),
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
        match self.prompt_for_choice(Prompt::RemoveCard, choices)? {
            Choice::RemoveCard(card_index, _) => Ok(card_index),
            _ => unreachable!(),
        }
    }

    /// Prompts the user to choose a card from their hand to play, returning the index of the card
    /// or None if the user chooses to end their turn.
    pub fn choose_card_to_play(&self, card_plays: &[CardPlay]) -> Result<Option<CardPlay>, Error> {
        let choices = card_plays
            .iter()
            .cloned()
            .map(Choice::PlayCardFromHand)
            .chain(once(Choice::EndTurn))
            .collect::<Vec<_>>();
        match self.prompt_for_choice(Prompt::CombatAction, choices)? {
            Choice::PlayCardFromHand(card_play) => Ok(Some(card_play)),
            Choice::EndTurn => Ok(None),
            _ => unreachable!(),
        }
    }

    /// Prompts the user to choose an enemy to target for their card or potion effect.
    pub fn choose_enemy_to_target(
        &self,
        enemies: &[Option<Enemy>],
        effect_chains: &[Option<EnemyEffectChain>],
    ) -> Result<EnemyIndex, Error> {
        let choices = enemies
            .iter()
            .zip(effect_chains)
            .enumerate()
            .filter_map(|(index, (maybe_enemy, maybe_effect_chain))| {
                maybe_enemy.as_ref().map(|enemy| {
                    Choice::TargetEnemy(
                        index,
                        enemy.enemy_type(),
                        maybe_effect_chain
                            .as_ref()
                            .cloned()
                            .expect("Missing effect chain"),
                    )
                })
            })
            .collect::<Vec<_>>();
        match self.prompt_for_choice(Prompt::TargetEnemy, choices)? {
            Choice::TargetEnemy(enemy_index, _, _) => Ok(enemy_index),
            _ => unreachable!(),
        }
    }

    pub fn send_add_to_discard_pile(&self, cards: &[Card]) -> Result<(), Error> {
        self.to_client
            .send(StsMessage::AddToDiscardPile(cards.to_vec()))?;
        Ok(())
    }

    pub fn send_block(&self, amount: Block) -> Result<(), Error> {
        self.to_client.send(StsMessage::Block(amount))?;
        Ok(())
    }

    pub fn send_block_gained(&self, amount: Block) -> Result<(), Error> {
        self.to_client.send(StsMessage::BlockGained(amount))?;
        Ok(())
    }

    pub fn send_block_lost(&self, amount: Block) -> Result<(), Error> {
        self.to_client.send(StsMessage::BlockLost(amount))?;
        Ok(())
    }

    pub fn send_card_discarded(&self, hand_index: HandIndex, card: Card) -> Result<(), Error> {
        self.to_client
            .send(StsMessage::CardDiscarded(hand_index, card))?;
        Ok(())
    }

    pub fn send_card_drawn(&self, hand_index: HandIndex, card: Card) -> Result<(), Error> {
        self.to_client
            .send(StsMessage::CardDrawn(hand_index, card))?;
        Ok(())
    }

    pub fn send_card_obtained(&self, card: Card) -> Result<(), Error> {
        self.to_client.send(StsMessage::CardObtained(card))?;
        Ok(())
    }

    pub fn send_card_removed(&self, card: Card) -> Result<(), Error> {
        self.to_client.send(StsMessage::CardRemoved(card))?;
        Ok(())
    }

    pub fn send_damage_blocked(&self, amount: Hp) -> Result<(), Error> {
        self.to_client.send(StsMessage::DamageBlocked(amount))?;
        Ok(())
    }

    pub fn send_damage_taken(&self, amount: Hp) -> Result<(), Error> {
        self.to_client.send(StsMessage::DamageTaken(amount))?;
        Ok(())
    }

    pub fn send_debuffs(&self, debuffs: &[(Debuff, StackCount)]) -> Result<(), Error> {
        self.to_client.send(StsMessage::Debuffs(debuffs.to_vec()))?;
        Ok(())
    }

    pub fn send_deck(&self, deck: &[Card]) -> Result<(), Error> {
        self.to_client.send(StsMessage::Deck(deck.to_vec()))?;
        Ok(())
    }

    pub fn send_enemy_died(&self, index: EnemyIndex, enemy_type: EnemyType) -> Result<(), Error> {
        self.to_client
            .send(StsMessage::EnemyDied(index, enemy_type))?;
        Ok(())
    }

    pub fn send_enemy_party(&self, enemies: Vec<Option<EnemyStatus>>) -> Result<(), Error> {
        self.to_client.send(StsMessage::EnemyParty(enemies))?;
        Ok(())
    }

    pub fn send_enemy_status(&self, index: EnemyIndex, status: EnemyStatus) -> Result<(), Error> {
        self.to_client
            .send(StsMessage::EnemyStatus(index, status))?;
        Ok(())
    }

    pub fn send_energy(&self, energy: Energy) -> Result<(), Error> {
        self.to_client.send(StsMessage::Energy(energy))?;
        Ok(())
    }

    pub fn send_gold_changed(&self, gold: Gold) -> Result<(), Error> {
        self.to_client.send(StsMessage::Gold(gold))?;
        Ok(())
    }

    pub fn send_hand_discarded(&self) -> Result<(), Error> {
        self.to_client.send(StsMessage::HandDiscarded)?;
        Ok(())
    }

    pub fn send_health_changed(&self, health: Health) -> Result<(), Error> {
        self.to_client.send(StsMessage::Health(health))?;
        Ok(())
    }

    pub fn send_map_string(&self, map_string: String) -> Result<(), Error> {
        self.to_client.send(StsMessage::Map(map_string))?;
        Ok(())
    }

    pub fn send_potions(&self, potions: &[Option<Potion>]) -> Result<(), Error> {
        self.to_client.send(StsMessage::Potions(potions.to_vec()))?;
        Ok(())
    }

    pub fn send_relic_obtained(&self, relic: Relic) -> Result<(), Error> {
        self.to_client.send(StsMessage::RelicObtained(relic))?;
        Ok(())
    }

    pub fn send_relics(&self, relics: &[Relic]) -> Result<(), Error> {
        self.to_client.send(StsMessage::Relics(relics.to_vec()))?;
        Ok(())
    }

    pub fn send_shuffling_discard_to_draw(&self) -> Result<(), Error> {
        self.to_client.send(StsMessage::ShufflingDiscardToDraw)?;
        Ok(())
    }

    fn prompt_for_choice(&self, prompt: Prompt, choices: Vec<Choice>) -> Result<Choice, Error> {
        self.to_client
            .send(StsMessage::Choices(prompt, choices.clone()))?;
        let choice_index = self.from_client.recv()?;
        match choices.get(choice_index) {
            Some(choice) => Ok(choice.clone()),
            _ => Err(anyhow!("Invalid choice")),
        }
    }
}
