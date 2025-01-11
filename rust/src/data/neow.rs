use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NeowBlessing {
    ChooseOneOfThreeCards,
    ChooseUncommonColorlessCard,
    Composite(NeowBonus, NeowPenalty),
    GainOneHundredGold,
    IncreaseMaxHpByTenPercent,
    NeowsLament,
    ObtainRandomCommonRelic,
    ObtainRandomRareCard,
    ObtainThreeRandomPotions,
    RemoveCard,
    ReplaceStarterRelic,
    TransformCard,
    UpgradeCard,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NeowBonus {
    ChooseRareCard,
    ChooseRareColorlessCard,
    GainTwoHundredFiftyGold,
    IncreaseMaxHpByTwentyPercent,
    ObtainRandomRareRelic,
    RemoveTwoCards,
    TransformTwoCards,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NeowPenalty {
    DecreaseMaxHpByTenPercent,
    LoseAllGold,
    ObtainCurse,
    TakeDamage,
}

pub const FIRST_NEOW_POOL: &[NeowBlessing] = &[
    NeowBlessing::ChooseOneOfThreeCards,
    NeowBlessing::ObtainRandomRareCard,
    NeowBlessing::RemoveCard,
    NeowBlessing::UpgradeCard,
    NeowBlessing::TransformCard,
    NeowBlessing::ChooseUncommonColorlessCard,
];

pub const SECOND_NEOW_POOL: &[NeowBlessing] = &[
    NeowBlessing::ObtainThreeRandomPotions,
    NeowBlessing::ObtainRandomCommonRelic,
    NeowBlessing::IncreaseMaxHpByTenPercent,
    NeowBlessing::NeowsLament,
    NeowBlessing::GainOneHundredGold,
];

pub const THIRD_NEOW_POOL: &[(NeowPenalty, &[NeowBonus])] = &[
    (
        NeowPenalty::DecreaseMaxHpByTenPercent,
        &[
            NeowBonus::ChooseRareColorlessCard,
            NeowBonus::RemoveTwoCards,
            NeowBonus::ObtainRandomRareRelic,
            NeowBonus::ChooseRareCard,
            NeowBonus::GainTwoHundredFiftyGold,
            NeowBonus::TransformTwoCards,
        ],
    ),
    (
        NeowPenalty::LoseAllGold,
        &[
            NeowBonus::ChooseRareColorlessCard,
            NeowBonus::RemoveTwoCards,
            NeowBonus::ObtainRandomRareRelic,
            NeowBonus::ChooseRareCard,
            NeowBonus::TransformTwoCards,
            NeowBonus::IncreaseMaxHpByTwentyPercent,
        ],
    ),
    (
        NeowPenalty::ObtainCurse,
        &[
            NeowBonus::ChooseRareColorlessCard,
            NeowBonus::ObtainRandomRareRelic,
            NeowBonus::ChooseRareCard,
            NeowBonus::GainTwoHundredFiftyGold,
            NeowBonus::TransformTwoCards,
            NeowBonus::IncreaseMaxHpByTwentyPercent,
        ],
    ),
    (
        NeowPenalty::TakeDamage,
        &[
            NeowBonus::ChooseRareColorlessCard,
            NeowBonus::RemoveTwoCards,
            NeowBonus::ObtainRandomRareRelic,
            NeowBonus::ChooseRareCard,
            NeowBonus::GainTwoHundredFiftyGold,
            NeowBonus::TransformTwoCards,
            NeowBonus::IncreaseMaxHpByTwentyPercent,
        ],
    ),
];

impl fmt::Display for NeowBlessing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NeowBlessing::Composite(benefit, drawback) => write!(f, "{}. {}", benefit, drawback),
            _ => write!(
                f,
                "{}",
                match self {
                    NeowBlessing::ChooseOneOfThreeCards => "Choose one of 3 cards to obtain",
                    NeowBlessing::ChooseUncommonColorlessCard =>
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
