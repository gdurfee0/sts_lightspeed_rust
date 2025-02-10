use anyhow::Error;

use crate::components::{
    Choice, Interaction, Notification, PlayerPersistentState, PotionAction, Prompt,
};
use crate::data::Potion;

use super::health_system::HealthSystem;

pub struct PotionSystem;

impl PotionSystem {
    /// Notifies the player of their current potions.
    pub fn notify_player<I: Interaction>(
        comms: &I,
        pps: &PlayerPersistentState,
    ) -> Result<(), Error> {
        comms.send_notification(Notification::Potions(pps.potions.to_vec()))
    }

    /// Prompts the player to obtain a potion and notifies them of the change.
    pub fn choose_potions_to_obtain<I: Interaction>(
        comms: &I,
        pps: &mut PlayerPersistentState,
        potion_choices: &[Potion],
        mut choice_count: usize,
    ) -> Result<(), Error> {
        let mut potion_choice_vec = potion_choices.to_vec();
        while !potion_choice_vec.is_empty() && choice_count > 0 {
            let mut choices = vec![];
            if Self::has_potion_slot_available(pps) {
                choices.extend(potion_choice_vec.iter().copied().map(Choice::ObtainPotion));
            }
            let _ = Self::extend_with_potion_actions(pps, false, &mut choices);
            choices.push(Choice::Skip);
            match comms.prompt_for_choice(
                if choice_count > 1 {
                    Prompt::ChooseNext
                } else {
                    Prompt::ChooseOne
                },
                &choices,
            )? {
                Choice::ExpendPotion(potion_action) => {
                    Self::expend_potion_out_of_combat(comms, pps, &potion_action)?
                }
                Choice::ObtainPotion(potion) => {
                    *pps.potions
                        .iter_mut()
                        .find(|p| p.is_none())
                        .expect("Just checked that potion slots are available") = Some(*potion);
                    let potion_index = potion_choice_vec
                        .iter()
                        .position(|p| *p == *potion)
                        .expect("Potion not found");
                    potion_choice_vec.remove(potion_index);
                    choice_count -= 1;
                }
                Choice::Skip => break,
                invalid => unreachable!("{:?}", invalid),
            }
            Self::notify_player(comms, pps)?;
        }
        Ok(())
    }

    /// Add choices to discard any potion, or to drink a potion if it's allowed out of combat.
    /// Returns true iff there was at least one choice related to the player's potions.
    pub fn extend_with_potion_actions(
        pps: &PlayerPersistentState,
        in_combat: bool,
        choices: &mut Vec<Choice>,
    ) -> bool {
        let mut has_potion = false;
        for (index, maybe_potion) in pps.potions.iter().enumerate() {
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

    /// Discard any potion, or drink a potion if it's allowed to do so out of combat.
    pub fn expend_potion_out_of_combat<I: Interaction>(
        comms: &I,
        pps: &mut PlayerPersistentState,
        potion_action: &PotionAction,
    ) -> Result<(), Error> {
        match potion_action {
            PotionAction::Discard(potion_index, _) => {
                pps.potions[*potion_index] = None;
            }

            PotionAction::Drink(potion_index, potion) => {
                pps.potions[*potion_index] = None;
                match potion {
                    Potion::BloodPotion => HealthSystem::increase_hp(comms, pps, pps.hp_max / 5)?,
                    Potion::EntropicBrew => todo!(),
                    Potion::FruitJuice => HealthSystem::increase_hp_max(comms, pps, 5)?,
                    invalid => {
                        unreachable!("Should not be able to drink {:?} out of combat", invalid)
                    }
                }
            }
        }
        Self::notify_player(comms, pps)
    }

    /// Checks if there is a potion slot available.
    fn has_potion_slot_available(pps: &PlayerPersistentState) -> bool {
        pps.potions.iter().any(|p| p.is_none())
    }

    /// Adds the supplied potion to an available potion slot and notifies the player of the change.
    fn obtain_potion<I: Interaction>(
        comms: &I,
        pps: &mut PlayerPersistentState,
        potion: Potion,
    ) -> Result<(), Error> {
        *pps.potions
            .iter_mut()
            .find(|p| p.is_none())
            .expect("Must be called only when there is a potion slot available") = Some(potion);
        Self::notify_player(comms, pps)
    }
}
