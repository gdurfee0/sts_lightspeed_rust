// Source: Slay the Spire wiki (https://slay-the-spire.fandom.com/wiki/Potions)

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[allow(clippy::enum_variant_names)] // For consistency with the game
pub enum Potion {
    /// Enter Divinity Stance.
    Ambrosia,

    /// Gain 1 (2) Artifact.
    AncientPotion,

    /// Add (2 copies of) 1 of 3 random Attack cards to your hand, costing 0 Energy this turn.
    AttackPotion,

    /// Upgrade all cards in your hand for the rest of combat.
    BlessingOfTheForge,

    /// Gain 12 (24) Block.
    BlockPotion,

    /// Heal for 20% (40%) of your max HP.
    BloodPotion,

    /// Add 2 (4) Miracles to your hand.
    BottledMiracle,

    /// Add (2 copies of) 1 of 3 random Colorless cards to your hand, costing 0 Energy this turn.
    ColorlessPotion,

    /// Gain 1 (2) Ritual.
    CultistPotion,

    /// Add 3 (6) Upgraded Shivs to your hand.
    CunningPotion,

    /// Gain 2 (4) Dexterity.
    DexterityPotion,

    /// Play the top 3 (6) cards of your draw pile.
    DistilledChaos,

    /// This turn your next (2) card(s) are played twice.
    DuplicationPotion,

    /// Exhaust any number of cards in your hand.
    Elixir,

    /// Gain 2 (4) Energy.
    EnergyPotion,

    /// Fill all your empty potion slots with random potions.
    EntropicBrew,

    /// Channel 1 (2) Dark Orbs for each orb slot.
    EssenceOfDarkness,

    /// Gain 4 (8) Plated Armor.
    EssenceOfSteel,

    /// Deal 10 (20) Damage to all enemies.
    ExplosivePotion,

    /// When you would die, heal 30% (60%) of your max HP instead and discard this potion.
    FairyInABottle,

    /// Apply 3 (6) Vulnerable to target enemy.
    FearPotion,

    /// Deal 20 (40) Damage to target enemy.
    FirePotion,

    /// Gain 5 (10) Strength. At the end of your turn, lose 5 (10) Strength.
    FlexPotion,

    /// Gain 2 (4) Focus.
    FocusPotion,

    /// Gain 5 (10) max HP.
    FruitJuice,

    /// Discard any nu mber of cards, then draw that many.
    GamblersBrew,

    /// Gain 1 (2) Intangible.
    GhostInAJar,

    /// Gain 6 (12) Metallicize.
    HeartOfIron,

    /// Gain 3 (6) Thorns.
    LiquidBronze,

    /// Choose 1 (2) card(s) in your discard pile and return it (them) to your hand. It (they) cost
    /// 0 this turn.
    LiquidMemories,

    /// Apply 6 (12) Poison to target enemy.
    PoisonPotion,

    /// Gain 2 (4) Orb slots.
    PotionOfCapacity,

    /// Add (2 copies of) 1 of 3 random Power cards to your hand, costing 0 Energy this turn.
    PowerPotion,

    /// Gain 5 (10) Regenereation.
    RegenPotion,

    /// Add (2 copies of) 1 of 3 random Skill cards to your hand, costing 0 Energy this turn.
    SkillPotion,

    /// Escape from a non-boss combat, receiving no rewards.
    SmokeBomb,

    /// Draw 5 (10) cards. Randomize the cost of all cards in your hand for the rest of combat.
    SneckoOil,

    /// Gain 5 (10) Dexterity. At the end of your turn, lose 5 (10) Dexterity.
    SpeedPotion,

    /// Enter Calm or Wrath.
    StancePotion,

    /// Gain 2 (4) Strength.
    StrengthPotion,

    /// Draw 3 (6) cards.
    SwiftPotion,

    /// Apply 3 (6) Weak to target enemy.
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
