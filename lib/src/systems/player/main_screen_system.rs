use anyhow::Error;

use crate::components::{Choice, Interaction, PlayerPersistentState, Prompt};
use crate::data::{Card, Potion};
use crate::systems::base::{DeckSystem, GoldSystem, HealthSystem, PotionSystem, RelicSystem};
use crate::types::Gold;

pub struct MainScreenSystem;

impl MainScreenSystem {
    /// Notifies the player of the current state of their persistent stats.
    pub fn notify_player<I: Interaction>(
        comms: &I,
        pps: &PlayerPersistentState,
    ) -> Result<(), Error> {
        DeckSystem::notify_player(comms, pps)?;
        GoldSystem::notify_player(comms, pps)?;
        HealthSystem::notify_player(comms, pps)?;
        PotionSystem::notify_player(comms, pps)?;
        RelicSystem::notify_player(comms, pps)
    }

    /// Prompts the player to choose their combat rewards.
    pub fn choose_combat_rewards<I: Interaction>(
        comms: &I,
        pps: &mut PlayerPersistentState,
        available_gold: Gold,
        mut maybe_potion: Option<Potion>,
        available_cards: &[Card],
    ) -> Result<(), Error> {
        let mut maybe_gold: Option<Gold> = Some(available_gold);
        let mut available_card_vec = available_cards.to_vec();
        let mut cards_left_to_choose = 1;
        while maybe_gold.is_some()
            || (maybe_potion.is_some() && PotionSystem::has_potion_slot_available(pps))
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
            let _ = PotionSystem::extend_with_potion_actions(pps, false, &mut choices);
            choices.push(Choice::Skip);
            match comms.prompt_for_choice(Prompt::ChooseNext, &choices)? {
                Choice::ExpendPotion(potion_action) => {
                    PotionSystem::expend_potion_out_of_combat(comms, pps, &potion_action)?
                }
                Choice::ObtainCard(card_reward_index, _) => {
                    let card_to_obtain = available_card_vec.remove(*card_reward_index);
                    DeckSystem::obtain_card(comms, pps, card_to_obtain)?;
                    cards_left_to_choose -= 1;
                }
                Choice::ObtainGold(gold_to_obtain) => {
                    GoldSystem::increase_gold(comms, pps, *gold_to_obtain)?;
                    maybe_gold = None;
                }
                Choice::ObtainPotion(potion_to_obtain) => {
                    PotionSystem::obtain_potion(comms, pps, *potion_to_obtain)?;
                    maybe_potion = None;
                }
                Choice::Skip => break,
                invalid => unreachable!("{:?}", invalid),
            }
        }
        Ok(())
    }
}
