use std::fmt;

use crate::data::{NeowBlessing, NeowBonus, NeowPenalty};

impl fmt::Display for NeowBlessing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NeowBlessing::Composite(benefit, drawback) => write!(f, "{}. {}", benefit, drawback),
            _ => write!(
                f,
                "{}",
                match self {
                    NeowBlessing::ChooseCard => "Choose one of 3 cards to obtain",
                    NeowBlessing::ChooseColorlessCard =>
                        "Choose an uncommon colorless card to obtain",
                    NeowBlessing::GainOneHundredGold => "Receive 100 gold",
                    NeowBlessing::IncreaseMaxHpByTenPercent => "Increase max HP by 10%",
                    NeowBlessing::NeowsLament => "Enemies in the next three combat rooms have 1 HP",
                    NeowBlessing::ObtainRandomCommonRelic => "Obtain a random common relic",
                    NeowBlessing::ObtainRandomRareCard => "Obtain a random rare card",
                    NeowBlessing::ObtainThreeRandomPotions => "Obtain 3 random potions",
                    NeowBlessing::RemoveCard => "Remove a card",
                    NeowBlessing::ReplaceStarterRelic =>
                        "Replace your starter relic with a random boss relic",
                    NeowBlessing::TransformCard => "Transform a card",
                    NeowBlessing::UpgradeCard => "Upgrade a card",
                    NeowBlessing::Composite(_, _) => unreachable!(),
                }
            ),
        }
    }
}

impl fmt::Display for NeowBonus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                NeowBonus::ChooseRareCard => "Choose a rare card to obtain",
                NeowBonus::ChooseRareColorlessCard => "Choose a rare colorless card to obtain",
                NeowBonus::GainTwoHundredFiftyGold => "Receive 250 gold",
                NeowBonus::IncreaseMaxHpByTwentyPercent => "Increase max HP by 20%",
                NeowBonus::ObtainRandomRareRelic => "Obtain a random rare relic",
                NeowBonus::RemoveTwoCards => "Remove two cards",
                NeowBonus::TransformTwoCards => "Transform two cards",
            }
        )
    }
}

impl fmt::Display for NeowPenalty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                NeowPenalty::DecreaseMaxHpByTenPercent => "Decrease max HP by 10%",
                NeowPenalty::LoseAllGold => "Lose all gold",
                NeowPenalty::ObtainCurse => "Obtain a curse",
                NeowPenalty::TakeDamage => "Take 30% of your max HP as damage",
            }
        )
    }
}
