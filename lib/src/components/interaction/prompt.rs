#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Prompt {
    ChooseCardToPutOnTopOfDrawPile,
    ChooseCombatReward,
    ChooseForEvent,
    ChooseNeow,
    ChooseNext, // Expectation is that the player may accept more than one of the Choices offered.
    ChooseOne,  // Expectation is that the player can pick at most one of the Choices offered.
    ChooseRestSiteAction,
    CombatAction,
    ClimbFloor,
    ClimbFloorHasPotion,
    RemoveCard,
    TargetEnemy,
    UpgradeCard,
}
