use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
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

impl fmt::Display for Potion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
