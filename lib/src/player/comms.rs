use std::iter::once;
use std::sync::mpsc::{Receiver, Sender};

use anyhow::{anyhow, Error};

use crate::data::{Card, Debuff, EnemyType, NeowBlessing, Potion, Relic};
use crate::enemy::EnemyStatus;
use crate::message::{Choice, PotionAction, Prompt, StsMessage};
use crate::types::{
    Block, ColumnIndex, DeckIndex, EnemyIndex, Energy, Gold, HandIndex, Health, Hp, StackCount,
};

#[derive(Clone, Debug)]
pub enum MainScreenAction {
    ClimbFloor(ColumnIndex),
    Potion(PotionAction),
}

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

    /// Prompts the user to choose a column to enter on the row above their current row,
    /// or to use/discard a potion if available.
    pub fn choose_main_screen_action(
        &self,
        columns: &[ColumnIndex],
        potion_actions: &[PotionAction],
    ) -> Result<MainScreenAction, Error> {
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
            Choice::ClimbFloor(column_index) => Ok(MainScreenAction::ClimbFloor(column_index)),
            Choice::PotionAction(potion_action) => Ok(MainScreenAction::Potion(potion_action)),
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

    /// Prompts the user to choose a card from their hand to play from the supplied list of
    /// possibly non-consecutive hand indexes (holes may be caused by unplayable or too-expensive
    /// cards).
    pub fn choose_card_to_play(
        &self,
        playable_cards: &[(HandIndex, Card)],
    ) -> Result<Option<HandIndex>, Error> {
        let choices = playable_cards
            .iter()
            .copied()
            .map(|(index, card)| Choice::PlayCardFromHand(index, card))
            .chain(once(Choice::EndTurn))
            .collect::<Vec<_>>();
        match self.prompt_for_choice(Prompt::CombatAction, choices)? {
            Choice::PlayCardFromHand(index, _) => Ok(Some(index)),
            Choice::EndTurn => Ok(None),
            _ => unreachable!(),
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
        match self.prompt_for_choice(Prompt::TargetEnemy, choices)? {
            Choice::TargetEnemy(enemy_index, _) => Ok(enemy_index),
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

    pub fn send_card_exhausted(&self, hand_index: HandIndex, card: Card) -> Result<(), Error> {
        self.to_client
            .send(StsMessage::CardExhausted(hand_index, card))?;
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

    pub fn send_ending_combat(&self) -> Result<(), Error> {
        self.to_client.send(StsMessage::EndingCombat)?;
        Ok(())
    }

    pub fn send_enemy_died(&self, index: EnemyIndex, enemy_type: EnemyType) -> Result<(), Error> {
        self.to_client
            .send(StsMessage::EnemyDied(index, enemy_type))?;
        Ok(())
    }

    pub fn send_enemy_statuses(&self, enemies: &[Option<EnemyStatus>]) -> Result<(), Error> {
        self.to_client
            .send(StsMessage::EnemyParty(enemies.to_vec()))?;
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

    pub fn send_game_over(&self, victory: bool) -> Result<(), Error> {
        self.to_client.send(StsMessage::GameOver(victory))?;
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

    pub fn send_starting_combat(&self) -> Result<(), Error> {
        self.to_client.send(StsMessage::StartingCombat)?;
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
