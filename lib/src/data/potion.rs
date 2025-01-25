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

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PotionRarity {
    Common,
    Uncommon,
    Rare,
}

impl Potion {
    pub fn can_drink_anywhere(&self) -> bool {
        matches!(
            *self,
            Potion::BloodPotion | Potion::EntropicBrew | Potion::FruitJuice
        )
    }

    pub fn rarity(&self) -> PotionRarity {
        match *self {
            Potion::Ambrosia => PotionRarity::Rare,
            Potion::AncientPotion => PotionRarity::Uncommon,
            Potion::AttackPotion => PotionRarity::Common,
            Potion::BlessingOfTheForge => PotionRarity::Common,
            Potion::BlockPotion => PotionRarity::Common,
            Potion::BloodPotion => PotionRarity::Common,
            Potion::BottledMiracle => PotionRarity::Common,
            Potion::ColorlessPotion => PotionRarity::Common,
            Potion::CultistPotion => PotionRarity::Rare,
            Potion::CunningPotion => PotionRarity::Uncommon,
            Potion::DexterityPotion => PotionRarity::Common,
            Potion::DistilledChaos => PotionRarity::Uncommon,
            Potion::DuplicationPotion => PotionRarity::Uncommon,
            Potion::Elixir => PotionRarity::Uncommon,
            Potion::EnergyPotion => PotionRarity::Common,
            Potion::EntropicBrew => PotionRarity::Rare,
            Potion::EssenceOfDarkness => PotionRarity::Rare,
            Potion::EssenceOfSteel => PotionRarity::Rare,
            Potion::ExplosivePotion => PotionRarity::Common,
            Potion::FairyInABottle => PotionRarity::Rare,
            Potion::FearPotion => PotionRarity::Common,
            Potion::FirePotion => PotionRarity::Common,
            Potion::FlexPotion => PotionRarity::Common,
            Potion::FocusPotion => PotionRarity::Common,
            Potion::FruitJuice => PotionRarity::Rare,
            Potion::GamblersBrew => PotionRarity::Uncommon,
            Potion::GhostInAJar => PotionRarity::Rare,
            Potion::HeartOfIron => PotionRarity::Rare,
            Potion::LiquidBronze => PotionRarity::Uncommon,
            Potion::LiquidMemories => PotionRarity::Uncommon,
            Potion::PoisonPotion => PotionRarity::Common,
            Potion::PotionOfCapacity => PotionRarity::Uncommon,
            Potion::PowerPotion => PotionRarity::Common,
            Potion::RegenPotion => PotionRarity::Uncommon,
            Potion::SkillPotion => PotionRarity::Common,
            Potion::SmokeBomb => PotionRarity::Rare,
            Potion::SneckoOil => PotionRarity::Rare,
            Potion::SpeedPotion => PotionRarity::Common,
            Potion::StancePotion => PotionRarity::Uncommon,
            Potion::StrengthPotion => PotionRarity::Common,
            Potion::SwiftPotion => PotionRarity::Common,
            Potion::WeakPotion => PotionRarity::Common,
        }
    }
}
