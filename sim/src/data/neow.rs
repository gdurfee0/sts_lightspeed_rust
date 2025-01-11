use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NeowBlessing {
    ChooseOneOfThreeCards,
    ChooseUncommonColorlessCard,
    Composite(NeowBenefit, NeowDrawback),
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
pub enum NeowBenefit {
    ChooseRareCard,
    ChooseRareColorlessCard,
    GainTwoHundredFiftyGold,
    IncreaseMaxHpByTwentyPercent,
    ObtainRandomRareRelic,
    RemoveTwoCards,
    TransformTwoCards,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NeowDrawback {
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

pub const THIRD_NEOW_POOL: &[(NeowDrawback, &[NeowBenefit])] = &[
    (
        NeowDrawback::DecreaseMaxHpByTenPercent,
        &[
            NeowBenefit::ChooseRareColorlessCard,
            NeowBenefit::RemoveTwoCards,
            NeowBenefit::ObtainRandomRareRelic,
            NeowBenefit::ChooseRareCard,
            NeowBenefit::GainTwoHundredFiftyGold,
            NeowBenefit::TransformTwoCards,
        ],
    ),
    (
        NeowDrawback::LoseAllGold,
        &[
            NeowBenefit::ChooseRareColorlessCard,
            NeowBenefit::RemoveTwoCards,
            NeowBenefit::ObtainRandomRareRelic,
            NeowBenefit::ChooseRareCard,
            NeowBenefit::TransformTwoCards,
            NeowBenefit::IncreaseMaxHpByTwentyPercent,
        ],
    ),
    (
        NeowDrawback::ObtainCurse,
        &[
            NeowBenefit::ChooseRareColorlessCard,
            NeowBenefit::ObtainRandomRareRelic,
            NeowBenefit::ChooseRareCard,
            NeowBenefit::GainTwoHundredFiftyGold,
            NeowBenefit::TransformTwoCards,
            NeowBenefit::IncreaseMaxHpByTwentyPercent,
        ],
    ),
    (
        NeowDrawback::TakeDamage,
        &[
            NeowBenefit::ChooseRareColorlessCard,
            NeowBenefit::RemoveTwoCards,
            NeowBenefit::ObtainRandomRareRelic,
            NeowBenefit::ChooseRareCard,
            NeowBenefit::GainTwoHundredFiftyGold,
            NeowBenefit::TransformTwoCards,
            NeowBenefit::IncreaseMaxHpByTwentyPercent,
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
                    NeowBlessing::ChooseOneOfThreeCards => "Choose a card to obtain",
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

impl fmt::Display for NeowBenefit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                NeowBenefit::ChooseRareCard => "Choose a rare card to obtain",
                NeowBenefit::ChooseRareColorlessCard => "Choose a rare colorless card to obtain",
                NeowBenefit::GainTwoHundredFiftyGold => "Receive 250 gold",
                NeowBenefit::IncreaseMaxHpByTwentyPercent => "Increase max HP by 20%",
                NeowBenefit::ObtainRandomRareRelic => "Obtain a random rare relic",
                NeowBenefit::RemoveTwoCards => "Remove two cards",
                NeowBenefit::TransformTwoCards => "Transform two cards",
            }
        )
    }
}

impl fmt::Display for NeowDrawback {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                NeowDrawback::DecreaseMaxHpByTenPercent => "Decrease max HP by 10%",
                NeowDrawback::LoseAllGold => "Lose all gold",
                NeowDrawback::ObtainCurse => "Obtain a curse",
                NeowDrawback::TakeDamage => "Take 30% of your max HP as damage",
            }
        )
    }
}
