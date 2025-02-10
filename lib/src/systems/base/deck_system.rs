use std::iter::once;

use anyhow::Error;

use crate::components::{Choice, Interaction, Notification, PlayerPersistentState, Prompt};
use crate::data::{Card, CardDetails};

pub struct DeckSystem;

impl DeckSystem {
    /// Notifies the player of the current deck.
    pub fn notify_player<I: Interaction>(
        comms: &I,
        pps: &mut PlayerPersistentState,
    ) -> Result<(), Error> {
        comms.send_notification(Notification::Deck(pps.deck.to_vec()))
    }

    /// Prompts the player to obtain a card and notifies them of the change.
    pub fn choose_card_to_obtain<I: Interaction>(
        comms: &I,
        pps: &mut PlayerPersistentState,
        cards: &[Card],
    ) -> Result<(), Error> {
        let choices = cards
            .iter()
            .copied()
            .enumerate()
            .map(|(reward_index, card)| Choice::ObtainCard(reward_index, card))
            .chain(once(Choice::Skip))
            .collect::<Vec<_>>();
        match comms.prompt_for_choice(Prompt::ChooseOne, &choices)? {
            Choice::ObtainCard(_, card) => Self::obtain_card(comms, pps, *card),
            Choice::Skip => Ok(()),
            invalid => unreachable!("{:?}", invalid),
        }
    }

    /// Prompts the player to remove a card from the deck and notifies them of the change.
    pub fn choose_card_to_remove<I: Interaction>(
        comms: &I,
        pps: &mut PlayerPersistentState,
    ) -> Result<(), Error> {
        let choices = pps
            .deck
            .iter()
            .copied()
            .enumerate()
            .map(|(deck_index, card)| Choice::RemoveCard(deck_index, card))
            .collect::<Vec<_>>();
        match comms.prompt_for_choice(Prompt::RemoveCard, &choices)? {
            Choice::RemoveCard(deck_index, _) => {
                let card = pps.deck.remove(*deck_index);
                comms.send_notification(Notification::CardRemoved(card))?;
            }
            invalid => unreachable!("{:?}", invalid),
        }
        Self::notify_player(comms, pps)
    }

    /// Prompts the player to upgrade a card and notifies them of the change.
    pub fn choose_card_to_upgrade<I: Interaction>(
        comms: &I,
        pps: &mut PlayerPersistentState,
    ) -> Result<(), Error> {
        let choices = pps
            .deck
            .iter()
            .copied()
            .enumerate()
            .filter_map(|(deck_index, card)| {
                let details = CardDetails::for_card(card);
                details
                    .upgrade
                    .map(|upgraded| Choice::UpgradeCard(deck_index, card, upgraded))
            })
            .collect::<Vec<_>>();
        match comms.prompt_for_choice(Prompt::UpgradeCard, &choices)? {
            Choice::UpgradeCard(deck_index, _, upgraded) => {
                // TODO: Check if the upgraded card goes in place of the current card or is
                // added to the end of the deck
                let card = pps.deck.remove(*deck_index);
                comms.send_notification(Notification::CardRemoved(card))?;
                Self::obtain_card(comms, pps, *upgraded)?;
            }
            invalid => unreachable!("{:?}", invalid),
        }
        Self::notify_player(comms, pps)
    }

    /// Adds the indicated card to the deck and notifies the player.
    fn obtain_card<I: Interaction>(
        comms: &I,
        pps: &mut PlayerPersistentState,
        card: Card,
    ) -> Result<(), Error> {
        pps.deck.push(card);
        comms.send_notification(Notification::CardObtained(card))?;
        Self::notify_player(comms, pps)
    }
}
