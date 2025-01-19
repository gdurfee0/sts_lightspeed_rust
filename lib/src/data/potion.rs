// Source: Slay the Spire wiki (https://slay-the-spire.fandom.com/wiki/Potions)

#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(test, derive(Eq, Hash))]
#[allow(clippy::enum_variant_names)] // For consistency with the game
pub enum Potion {
    Ambrosia,
    AncientPotion,
    AttackPotion,
    BlessingOfTheForge,
    BlockPotion,
    BloodPotion,
    BottledMiracle,
    ColorlessPotion,
    CultistPotion,
    CunningPotion,
    DexterityPotion,
    DistilledChaos,
    DuplicationPotion,
    Elixir,
    EnergyPotion,
    EntropicBrew,
    EssenceOfDarkness,
    EssenceOfSteel,
    ExplosivePotion,
    FairyInABottle,
    FearPotion,
    FirePotion,
    FlexPotion,
    FocusPotion,
    FruitJuice,
    GamblersBrew,
    GhostInAJar,
    HeartOfIron,
    LiquidBronze,
    LiquidMemories,
    PoisonPotion,
    PotionOfCapacity,
    PowerPotion,
    RegenPotion,
    SkillPotion,
    SmokeBomb,
    SneckoOil,
    SpeedPotion,
    StancePotion,
    StrengthPotion,
    SwiftPotion,
    WeakPotion,
}

impl Potion {
    pub fn can_drink_anywhere(&self) -> bool {
        matches!(
            *self,
            Potion::BloodPotion | Potion::EntropicBrew | Potion::FruitJuice
        )
    }
}
