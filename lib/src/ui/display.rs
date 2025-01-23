use std::fmt;

use crate::components::{Choice, PotionAction, Prompt};
use crate::data::CardDetails;

impl fmt::Display for Prompt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Prompt::ChooseCombatReward => write!(f, "Choose a combat reward"),
            Prompt::ChooseForEvent => write!(f, "Choose an option for the event"),
            Prompt::ChooseNeow => write!(f, "Choose Neow's Blessing"),
            Prompt::ChooseNext => write!(f, "Choose the next item to obtain"),
            Prompt::ChooseOne => write!(f, "Choose an item to obtain"),
            Prompt::ClimbFloor => write!(f, "Move up into one of the following columns"),
            Prompt::ClimbFloorHasPotion => write!(
                f,
                "Move up into one of the following columns, or drink/discard a potion"
            ),
            Prompt::CombatAction => write!(f, "It is your turn to act"),
            Prompt::RemoveCard => write!(f, "Choose a card to remove"),
            Prompt::TargetEnemy => write!(f, "Choose an enemy to target"),
        }
    }
}

impl fmt::Display for Choice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Choice::ClimbFloor(column_index) => {
                write!(
                    f,
                    "Climb Spire, column {}",
                    (b'a' + *column_index as u8) as char
                )
            }
            Choice::EventChoice(_, text) => write!(f, "{}", text),
            Choice::ExpendPotion(PotionAction::Discard(_, potion)) => {
                write!(f, "Discard potion \"{:?}\"", potion)
            }
            Choice::ExpendPotion(PotionAction::Drink(_, potion)) => {
                write!(f, "Drink potion \"{:?}\"", potion)
            }

            Choice::EndTurn => write!(f, "(End Turn)"),
            Choice::NeowBlessing(blessing) => write!(f, "{}", blessing),
            Choice::ObtainCard(card) => {
                write!(
                    f,
                    "{:?} {:?}",
                    card,
                    CardDetails::for_card(*card).effect_chain
                )
            }
            Choice::ObtainGold(gold) => write!(f, "Obtain {} gold", gold),
            Choice::ObtainPotion(potion) => write!(f, "{:?}", potion),
            Choice::PlayCardFromHand(_, card) => write!(f, "Play \"{:?}\"", card),
            Choice::RemoveCard(_, card) => write!(f, "{:?}", card),
            Choice::Skip => write!(f, "(Skip)"),
            Choice::TargetEnemy(_, enemy) => {
                write!(f, "Target \"{:?}\"", enemy)
            }
        }
    }
}
