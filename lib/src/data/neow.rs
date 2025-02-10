// Source: Slay the Spire wiki (https://slay-the-spire.fandom.com/wiki/Neow)

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum NeowBlessing {
    ChooseCard,
    ChooseColorlessCard,
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

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum NeowBonus {
    ChooseRareCard,
    ChooseRareColorlessCard,

    GainTwoHundredFiftyGold,
    IncreaseMaxHpByTwentyPercent,
    ObtainRandomRareRelic,
    RemoveTwoCards,
    TransformTwoCards,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum NeowPenalty {
    DecreaseMaxHpByTenPercent,
    LoseAllGold,
    ObtainCurse,
    TakeDamage,
}

// Special thanks to gamerpuppy for the pool orderings below, which match the game's rng.

pub const FIRST_NEOW_POOL: &[NeowBlessing] = &[
    NeowBlessing::ChooseCard,
    NeowBlessing::ObtainRandomRareCard,
    NeowBlessing::RemoveCard,
    NeowBlessing::UpgradeCard,
    NeowBlessing::TransformCard,
    NeowBlessing::ChooseColorlessCard,
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
