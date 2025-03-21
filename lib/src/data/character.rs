use anyhow::anyhow;

use super::card::Card;
use super::potion::Potion;
use super::relic::Relic;

#[derive(Debug)]
#[cfg_attr(test, derive(Eq, Hash, PartialEq))]
pub struct Character {
    /// The character's starting max hit points.
    pub starting_hp: u32,

    /// The character's starting relic.
    pub starting_relic: Relic,

    /// The character's starting deck in the order displayed in-game.
    pub starting_deck: &'static [Card],

    /// The character's common cards, available from shops, encounters, etc. Ordered appropriately
    /// for fidelity with the game's rng.
    pub common_card_pool: &'static [Card],

    /// Ordered appropriately for fidelity with the game's rng.
    pub uncommon_card_pool: &'static [Card],

    /// Rewards if the rng is feeling really nice. Ordered appropriately for fidelity with the
    /// game's rng.
    pub rare_card_pool: &'static [Card],

    /// For when a random attack card is needed. Omits healing cards like Feed and Reaper.
    pub attack_card_pool: &'static [Card],

    /// For when a random skill card is needed.
    pub skill_card_pool: &'static [Card],

    /// For when a random power card is needed.
    pub power_card_pool: &'static [Card],

    /// Potions, again the same order as the game's rng.
    pub potion_pool: &'static [Potion],

    /// Relic pools ordered appropriately for fidelity with the game's rng.
    pub common_relic_pool: &'static [Relic],
    pub uncommon_relic_pool: &'static [Relic],
    pub rare_relic_pool: &'static [Relic],
    pub shop_relic_pool: &'static [Relic],
    pub boss_relic_pool: &'static [Relic],
}

impl TryFrom<&str> for &'static Character {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().chars().next() {
            Some('i') => Ok(IRONCLAD),
            Some('s') => Ok(SILENT),
            Some('d') => Ok(DEFECT),
            Some('w') => Ok(WATCHER),
            _ => Err(anyhow!(
                "Character options are (I)ronclad, (S)ilent, (D)efect, and (W)atcher"
            )),
        }
    }
}

// Credit to gamerpuppy for the pool orderings below, which match the game's rng.

pub const IRONCLAD: &Character = &Character {
    starting_hp: 80,
    starting_relic: Relic::BurningBlood,
    starting_deck: &[
        Card::Strike(false),
        Card::Strike(false),
        Card::Strike(false),
        Card::Strike(false),
        Card::Strike(false),
        Card::Defend(false),
        Card::Defend(false),
        Card::Defend(false),
        Card::Defend(false),
        Card::Bash(false),
    ],
    common_card_pool: &[
        Card::Anger(false),
        Card::Cleave(false),
        Card::Warcry(false),
        Card::Flex(false),
        Card::IronWave(false),
        Card::BodySlam(false),
        Card::TrueGrit(false),
        Card::ShrugItOff(false),
        Card::Clash(false),
        Card::Thunderclap(false),
        Card::PommelStrike(false),
        Card::TwinStrike(false),
        Card::Clothesline(false),
        Card::Armaments(false),
        Card::Havoc(false),
        Card::Headbutt(false),
        Card::WildStrike(false),
        Card::HeavyBlade(false),
        Card::PerfectedStrike(false),
        Card::SwordBoomerang(false),
    ],
    uncommon_card_pool: &[
        Card::SpotWeakness(false),
        Card::Inflame(false),
        Card::PowerThrough(false),
        Card::DualWield(false),
        Card::InfernalBlade(false),
        Card::RecklessCharge(false),
        Card::Hemokinesis(false),
        Card::Intimidate(false),
        Card::BloodForBlood(false),
        Card::FlameBarrier(false),
        Card::Pummel(false),
        Card::BurningPact(false),
        Card::Metallicize(false),
        Card::Shockwave(false),
        Card::Rampage(false),
        Card::SeverSoul(false),
        Card::Whirlwind(false),
        Card::Combust(false),
        Card::DarkEmbrace(false),
        Card::SeeingRed(false),
        Card::Disarm(false),
        Card::FeelNoPain(false),
        Card::Rage(false),
        Card::Entrench(false),
        Card::Sentinel(false),
        Card::BattleTrance(false),
        Card::SearingBlow(0),
        Card::SecondWind(false),
        Card::Rupture(false),
        Card::Bloodletting(false),
        Card::Carnage(false),
        Card::Dropkick(false),
        Card::FireBreathing(false),
        Card::GhostlyArmor(false),
        Card::Uppercut(false),
        Card::Evolve(false),
    ],
    rare_card_pool: &[
        Card::Immolate(false),
        Card::Offering(false),
        Card::Exhume(false),
        Card::Reaper(false),
        Card::Brutality(false),
        Card::Juggernaut(false),
        Card::Impervious(false),
        Card::Berserk(false),
        Card::FiendFire(false),
        Card::Barricade(false),
        Card::Corruption(false),
        Card::LimitBreak(false),
        Card::Feed(false),
        Card::Bludgeon(false),
        Card::DemonForm(false),
        Card::DoubleTap(false),
    ],
    attack_card_pool: &[
        Card::SwordBoomerang(false),
        Card::PerfectedStrike(false),
        Card::HeavyBlade(false),
        Card::WildStrike(false),
        Card::Headbutt(false),
        Card::Clothesline(false),
        Card::TwinStrike(false),
        Card::PommelStrike(false),
        Card::Thunderclap(false),
        Card::Clash(false),
        Card::BodySlam(false),
        Card::IronWave(false),
        Card::Cleave(false),
        Card::Anger(false),
        Card::Uppercut(false),
        Card::Dropkick(false),
        Card::Carnage(false),
        Card::SearingBlow(0),
        Card::Whirlwind(false),
        Card::SeverSoul(false),
        Card::Rampage(false),
        Card::Pummel(false),
        Card::BloodForBlood(false),
        Card::Hemokinesis(false),
        Card::RecklessCharge(false),
        Card::Bludgeon(false),
        Card::FiendFire(false),
        Card::Immolate(false),
    ],
    skill_card_pool: &[
        Card::Havoc(false),
        Card::Armaments(false),
        Card::ShrugItOff(false),
        Card::TrueGrit(false),
        Card::Flex(false),
        Card::Warcry(false),
        Card::GhostlyArmor(false),
        Card::Bloodletting(false),
        Card::SecondWind(false),
        Card::BattleTrance(false),
        Card::Sentinel(false),
        Card::Entrench(false),
        Card::Rage(false),
        Card::Disarm(false),
        Card::SeeingRed(false),
        Card::Shockwave(false),
        Card::BurningPact(false),
        Card::FlameBarrier(false),
        Card::Intimidate(false),
        Card::InfernalBlade(false),
        Card::DualWield(false),
        Card::PowerThrough(false),
        Card::SpotWeakness(false),
        Card::DoubleTap(false),
        Card::LimitBreak(false),
        Card::Impervious(false),
        Card::Exhume(false),
        Card::Offering(false),
    ],
    power_card_pool: &[
        Card::Evolve(false),
        Card::FireBreathing(false),
        Card::Rupture(false),
        Card::FeelNoPain(false),
        Card::DarkEmbrace(false),
        Card::Combust(false),
        Card::Metallicize(false),
        Card::Inflame(false),
        Card::DemonForm(false),
        Card::Corruption(false),
        Card::Barricade(false),
        Card::Berserk(false),
        Card::Juggernaut(false),
        Card::Brutality(false),
    ],
    potion_pool: &[
        Potion::BloodPotion,
        Potion::Elixir,
        Potion::HeartOfIron,
        Potion::BlockPotion,
        Potion::DexterityPotion,
        Potion::EnergyPotion,
        Potion::ExplosivePotion,
        Potion::FirePotion,
        Potion::StrengthPotion,
        Potion::SwiftPotion,
        Potion::WeakPotion,
        Potion::FearPotion,
        Potion::AttackPotion,
        Potion::SkillPotion,
        Potion::PowerPotion,
        Potion::ColorlessPotion,
        Potion::FlexPotion,
        Potion::SpeedPotion,
        Potion::BlessingOfTheForge,
        Potion::RegenPotion,
        Potion::AncientPotion,
        Potion::LiquidBronze,
        Potion::GamblersBrew,
        Potion::EssenceOfSteel,
        Potion::DuplicationPotion,
        Potion::DistilledChaos,
        Potion::LiquidMemories,
        Potion::CultistPotion,
        Potion::FruitJuice,
        Potion::SneckoOil,
        Potion::FairyInABottle,
        Potion::SmokeBomb,
        Potion::EntropicBrew,
    ],
    common_relic_pool: &[
        Relic::Whetstone,
        Relic::TheBoot,
        Relic::BloodVial,
        Relic::MealTicket,
        Relic::PenNib,
        Relic::Akabeko,
        Relic::Lantern,
        Relic::RegalPillow,
        Relic::BagOfPreparation,
        Relic::AncientTeaSet,
        Relic::SmilingMask,
        Relic::PotionBelt,
        Relic::PreservedInsect,
        Relic::Omamori,
        Relic::MawBank,
        Relic::ArtOfWar,
        Relic::ToyOrnithopter,
        Relic::CeramicFish,
        Relic::Vajra,
        Relic::CentennialPuzzle,
        Relic::Strawberry,
        Relic::HappyFlower,
        Relic::OddlySmoothStone,
        Relic::WarPaint,
        Relic::BronzeScales,
        Relic::JuzuBracelet,
        Relic::DreamCatcher,
        Relic::Nunchaku,
        Relic::TinyChest,
        Relic::Orichalcum,
        Relic::Anchor,
        Relic::BagOfMarbles,
        Relic::RedSkull,
    ],
    uncommon_relic_pool: &[
        Relic::BottledTornado,
        Relic::Sundial,
        Relic::Kunai,
        Relic::Pear,
        Relic::BlueCandle,
        Relic::EternalFeather,
        Relic::StrikeDummy,
        Relic::SingingBowl,
        Relic::Matryoshka,
        Relic::InkBottle,
        Relic::TheCourier,
        Relic::FrozenEgg,
        Relic::OrnamentalFan,
        Relic::BottledLightning,
        Relic::GremlinHorn,
        Relic::HornCleat,
        Relic::ToxicEgg,
        Relic::LetterOpener,
        Relic::QuestionCard,
        Relic::BottledFlame,
        Relic::Shuriken,
        Relic::MoltenEgg,
        Relic::MeatOnTheBone,
        Relic::DarkstonePeriapt,
        Relic::MummifiedHand,
        Relic::Pantograph,
        Relic::WhiteBeastStatue,
        Relic::MercuryHourglass,
        Relic::SelfFormingClay,
        Relic::PaperPhrog,
    ],
    rare_relic_pool: &[
        Relic::Ginger,
        Relic::OldCoin,
        Relic::BirdFacedUrn,
        Relic::UnceasingTop,
        Relic::Torii,
        Relic::StoneCalendar,
        Relic::Shovel,
        Relic::WingBoots,
        Relic::ThreadAndNeedle,
        Relic::Turnip,
        Relic::IceCream,
        Relic::Calipers,
        Relic::LizardTail,
        Relic::PrayerWheel,
        Relic::Girya,
        Relic::DeadBranch,
        Relic::DuVuDoll,
        Relic::Pocketwatch,
        Relic::Mango,
        Relic::IncenseBurner,
        Relic::GamblingChip,
        Relic::PeacePipe,
        Relic::CaptainsWheel,
        Relic::FossilizedHelix,
        Relic::TungstenRod,
        Relic::MagicFlower,
        Relic::CharonsAshes,
        Relic::ChampionBelt,
    ],
    shop_relic_pool: &[
        Relic::SlingOfCourage,
        Relic::HandDrill,
        Relic::Toolbox,
        Relic::ChemicalX,
        Relic::LeesWaffle,
        Relic::Orrery,
        Relic::DollysMirror,
        Relic::OrangePellets,
        Relic::PrismaticShard,
        Relic::ClockworkSouvenir,
        Relic::FrozenEye,
        Relic::TheAbacus,
        Relic::MedicalKit,
        Relic::Cauldron,
        Relic::StrangeSpoon,
        Relic::MembershipCard,
        Relic::Brimstone,
    ],
    boss_relic_pool: &[
        Relic::FusionHammer,
        Relic::VelvetChoker,
        Relic::RunicDome,
        Relic::SlaversCollar,
        Relic::SneckoEye,
        Relic::PandorasBox,
        Relic::CursedKey,
        Relic::BustedCrown,
        Relic::Ectoplasm,
        Relic::TinyHouse,
        Relic::Sozu,
        Relic::PhilosophersStone,
        Relic::Astrolabe,
        Relic::BlackStar,
        Relic::SacredBark,
        Relic::EmptyCage,
        Relic::RunicPyramid,
        Relic::CallingBell,
        Relic::CoffeeDripper,
        Relic::BlackBlood,
        Relic::MarkOfPain,
        Relic::RunicCube,
    ],
};

pub const SILENT: &Character = &Character {
    starting_hp: 70,
    starting_relic: Relic::RingOfTheSnake,
    starting_deck: &[
        Card::Strike(false),
        Card::Strike(false),
        Card::Strike(false),
        Card::Strike(false),
        Card::Strike(false),
        Card::Defend(false),
        Card::Defend(false),
        Card::Defend(false),
        Card::Defend(false),
        Card::Defend(false),
        Card::Survivor(false),
        Card::Neutralize(false),
    ],
    common_card_pool: &[
        Card::CloakAndDagger(false),
        Card::SneakyStrike(false),
        Card::DeadlyPoison(false),
        Card::DaggerSpray(false),
        Card::Bane(false),
        Card::BladeDance(false),
        Card::Deflect(false),
        Card::DaggerThrow(false),
        Card::PoisonedStab(false),
        Card::Acrobatics(false),
        Card::QuickSlash(false),
        Card::Slice(false),
        Card::Backflip(false),
        Card::Outmaneuver(false),
        Card::Prepared(false),
        Card::PiercingWail(false),
        Card::SuckerPunch(false),
        Card::DodgeAndRoll(false),
        Card::FlyingKnee(false),
    ],
    uncommon_card_pool: &[
        Card::CripplingCloud(false),
        Card::LegSweep(false),
        Card::Catalyst(false),
        Card::Tactician(false),
        Card::Expertise(false),
        Card::Choke(false),
        Card::Caltrops(false),
        Card::Blur(false),
        Card::Setup(false),
        Card::EndlessAgony(false),
        Card::RiddleWithHoles(false),
        Card::Skewer(false),
        Card::CalculatedGamble(false),
        Card::EscapePlan(false),
        Card::Finisher(false),
        Card::WellLaidPlans(false),
        Card::Terror(false),
        Card::HeelHook(false),
        Card::NoxiousFumes(false),
        Card::InfiniteBlades(false),
        Card::Reflex(false),
        Card::Eviscerate(false),
        Card::Dash(false),
        Card::Backstab(false),
        Card::BouncingFlask(false),
        Card::Concentrate(false),
        Card::Flechettes(false),
        Card::MasterfulStab(false),
        Card::Accuracy(false),
        Card::Footwork(false),
        Card::Distraction(false),
        Card::AllOutAttack(false),
        Card::Predator(false),
    ],
    rare_card_pool: &[
        Card::GrandFinale(false),
        Card::AThousandCuts(false),
        Card::GlassKnife(false),
        Card::StormOfSteel(false),
        Card::BulletTime(false),
        Card::AfterImage(false),
        Card::Unload(false),
        Card::Nightmare(false),
        Card::ToolsOfTheTrade(false),
        Card::WraithForm(false),
        Card::Burst(false),
        Card::Doppelganger(false),
        Card::Envenom(false),
        Card::Adrenaline(false),
        Card::DieDieDie(false),
        Card::PhantasmalKiller(false),
        Card::Malaise(false),
        Card::CorpseExplosion(false),
        Card::Alchemize(false),
    ],
    attack_card_pool: &[], // TODO: Add attack cards
    skill_card_pool: &[],  // TODO: Add skill cards
    power_card_pool: &[],  // TODO: Add power cards
    potion_pool: &[
        Potion::PoisonPotion,
        Potion::CunningPotion,
        Potion::GhostInAJar,
        Potion::BlockPotion,
        Potion::DexterityPotion,
        Potion::EnergyPotion,
        Potion::ExplosivePotion,
        Potion::FirePotion,
        Potion::StrengthPotion,
        Potion::SwiftPotion,
        Potion::WeakPotion,
        Potion::FearPotion,
        Potion::AttackPotion,
        Potion::SkillPotion,
        Potion::PowerPotion,
        Potion::ColorlessPotion,
        Potion::FlexPotion,
        Potion::SpeedPotion,
        Potion::BlessingOfTheForge,
        Potion::RegenPotion,
        Potion::AncientPotion,
        Potion::LiquidBronze,
        Potion::GamblersBrew,
        Potion::EssenceOfSteel,
        Potion::DuplicationPotion,
        Potion::DistilledChaos,
        Potion::LiquidMemories,
        Potion::CultistPotion,
        Potion::FruitJuice,
        Potion::SneckoOil,
        Potion::FairyInABottle,
        Potion::SmokeBomb,
        Potion::EntropicBrew,
    ],
    common_relic_pool: &[
        Relic::Whetstone,
        Relic::TheBoot,
        Relic::BloodVial,
        Relic::MealTicket,
        Relic::PenNib,
        Relic::Akabeko,
        Relic::Lantern,
        Relic::RegalPillow,
        Relic::BagOfPreparation,
        Relic::AncientTeaSet,
        Relic::SmilingMask,
        Relic::PotionBelt,
        Relic::PreservedInsect,
        Relic::Omamori,
        Relic::MawBank,
        Relic::ArtOfWar,
        Relic::ToyOrnithopter,
        Relic::CeramicFish,
        Relic::Vajra,
        Relic::CentennialPuzzle,
        Relic::Strawberry,
        Relic::HappyFlower,
        Relic::OddlySmoothStone,
        Relic::WarPaint,
        Relic::BronzeScales,
        Relic::JuzuBracelet,
        Relic::DreamCatcher,
        Relic::Nunchaku,
        Relic::TinyChest,
        Relic::Orichalcum,
        Relic::Anchor,
        Relic::BagOfMarbles,
        Relic::SneckoSkull,
    ],
    uncommon_relic_pool: &[
        Relic::BottledTornado,
        Relic::Sundial,
        Relic::Kunai,
        Relic::Pear,
        Relic::BlueCandle,
        Relic::EternalFeather,
        Relic::StrikeDummy,
        Relic::SingingBowl,
        Relic::Matryoshka,
        Relic::InkBottle,
        Relic::TheCourier,
        Relic::FrozenEgg,
        Relic::OrnamentalFan,
        Relic::BottledLightning,
        Relic::GremlinHorn,
        Relic::HornCleat,
        Relic::ToxicEgg,
        Relic::LetterOpener,
        Relic::QuestionCard,
        Relic::BottledFlame,
        Relic::Shuriken,
        Relic::MoltenEgg,
        Relic::MeatOnTheBone,
        Relic::DarkstonePeriapt,
        Relic::MummifiedHand,
        Relic::Pantograph,
        Relic::WhiteBeastStatue,
        Relic::MercuryHourglass,
        Relic::NinjaScroll,
        Relic::PaperKrane,
    ],
    rare_relic_pool: &[
        Relic::Ginger,
        Relic::OldCoin,
        Relic::BirdFacedUrn,
        Relic::UnceasingTop,
        Relic::Torii,
        Relic::StoneCalendar,
        Relic::Shovel,
        Relic::WingBoots,
        Relic::ThreadAndNeedle,
        Relic::Turnip,
        Relic::IceCream,
        Relic::Calipers,
        Relic::LizardTail,
        Relic::PrayerWheel,
        Relic::Girya,
        Relic::DeadBranch,
        Relic::DuVuDoll,
        Relic::Pocketwatch,
        Relic::Mango,
        Relic::IncenseBurner,
        Relic::GamblingChip,
        Relic::PeacePipe,
        Relic::CaptainsWheel,
        Relic::FossilizedHelix,
        Relic::TungstenRod,
        Relic::ToughBandages,
        Relic::TheSpecimen,
        Relic::Tingsha,
    ],
    shop_relic_pool: &[
        Relic::SlingOfCourage,
        Relic::HandDrill,
        Relic::Toolbox,
        Relic::ChemicalX,
        Relic::LeesWaffle,
        Relic::Orrery,
        Relic::DollysMirror,
        Relic::OrangePellets,
        Relic::PrismaticShard,
        Relic::ClockworkSouvenir,
        Relic::FrozenEye,
        Relic::TheAbacus,
        Relic::MedicalKit,
        Relic::Cauldron,
        Relic::StrangeSpoon,
        Relic::MembershipCard,
        Relic::TwistedFunnel,
    ],
    boss_relic_pool: &[
        Relic::FusionHammer,
        Relic::VelvetChoker,
        Relic::RunicDome,
        Relic::SlaversCollar,
        Relic::SneckoEye,
        Relic::PandorasBox,
        Relic::CursedKey,
        Relic::BustedCrown,
        Relic::Ectoplasm,
        Relic::TinyHouse,
        Relic::Sozu,
        Relic::PhilosophersStone,
        Relic::Astrolabe,
        Relic::BlackStar,
        Relic::SacredBark,
        Relic::EmptyCage,
        Relic::RunicPyramid,
        Relic::CallingBell,
        Relic::CoffeeDripper,
        Relic::WristBlade,
        Relic::HoveringKite,
        Relic::RingOfTheSerpent,
    ],
};

pub const DEFECT: &Character = &Character {
    starting_hp: 75,
    starting_relic: Relic::CrackedCore,
    starting_deck: &[
        Card::Strike(false),
        Card::Strike(false),
        Card::Strike(false),
        Card::Strike(false),
        Card::Defend(false),
        Card::Defend(false),
        Card::Defend(false),
        Card::Defend(false),
        Card::Zap(false),
        Card::Dualcast(false),
    ],
    common_card_pool: &[
        Card::GoForTheEyes(false),
        Card::BallLightning(false),
        Card::Streamline(false),
        Card::Recursion(false),
        Card::CompileDriver(false),
        Card::Barrage(false),
        Card::Stack(false),
        Card::Rebound(false),
        Card::Claw(false),
        Card::Coolheaded(false),
        Card::Turbo(false),
        Card::SweepingBeam(false),
        Card::ChargeBattery(false),
        Card::Hologram(false),
        Card::BeamCell(false),
        Card::Leap(false),
        Card::ColdSnap(false),
        Card::SteamBarrier(false),
    ],
    uncommon_card_pool: &[
        Card::Storm(false),
        Card::GeneticAlgorithm(false),
        Card::Overclock(false),
        Card::HelloWorld(false),
        Card::Sunder(false),
        Card::Glacier(false),
        Card::Consume(false),
        Card::Fusion(false),
        Card::Aggregate(false),
        Card::Blizzard(false),
        Card::Chaos(false),
        Card::Melter(false),
        Card::SelfRepair(false),
        Card::Loop(false),
        Card::Chill(false),
        Card::BootSequence(false),
        Card::StaticDischarge(false),
        Card::Heatsinks(false),
        Card::Tempest(false),
        Card::Equilibrium(false),
        Card::ForceField(false),
        Card::Ftl(false),
        Card::RipAndTear(false),
        Card::Darkness(false),
        Card::DoubleEnergy(false),
        Card::ReinforcedBody(false),
        Card::AutoShields(false),
        Card::Reprogram(false),
        Card::Bullseye(false),
        Card::Scrape(false),
        Card::Recycle(false),
        Card::Skim(false),
        Card::WhiteNoise(false),
        Card::Capacitor(false),
        Card::Defragment(false),
        Card::DoomAndGloom(false),
    ],
    rare_card_pool: &[
        Card::CoreSurge(false),
        Card::Fission(false),
        Card::CreativeAi(false),
        Card::Amplify(false),
        Card::Reboot(false),
        Card::AllForOne(false),
        Card::EchoForm(false),
        Card::MeteorStrike(false),
        Card::Seek(false),
        Card::Rainbow(false),
        Card::Buffer(false),
        Card::Electrodynamics(false),
        Card::MachineLearning(false),
        Card::BiasedCognition(false),
        Card::ThunderStrike(false),
        Card::Hyperbeam(false),
        Card::MultiCast(false),
    ],
    attack_card_pool: &[], // TODO: Add attack cards
    skill_card_pool: &[],  // TODO: Add skill cards
    power_card_pool: &[],  // TODO: Add power cards
    potion_pool: &[
        Potion::FocusPotion,
        Potion::PotionOfCapacity,
        Potion::EssenceOfDarkness,
        Potion::BlockPotion,
        Potion::DexterityPotion,
        Potion::EnergyPotion,
        Potion::ExplosivePotion,
        Potion::FirePotion,
        Potion::StrengthPotion,
        Potion::SwiftPotion,
        Potion::WeakPotion,
        Potion::FearPotion,
        Potion::AttackPotion,
        Potion::SkillPotion,
        Potion::PowerPotion,
        Potion::ColorlessPotion,
        Potion::FlexPotion,
        Potion::SpeedPotion,
        Potion::BlessingOfTheForge,
        Potion::RegenPotion,
        Potion::AncientPotion,
        Potion::LiquidBronze,
        Potion::GamblersBrew,
        Potion::EssenceOfSteel,
        Potion::DuplicationPotion,
        Potion::DistilledChaos,
        Potion::LiquidMemories,
        Potion::CultistPotion,
        Potion::FruitJuice,
        Potion::SneckoOil,
        Potion::FairyInABottle,
        Potion::SmokeBomb,
        Potion::EntropicBrew,
    ],
    common_relic_pool: &[
        Relic::Whetstone,
        Relic::TheBoot,
        Relic::BloodVial,
        Relic::MealTicket,
        Relic::PenNib,
        Relic::Akabeko,
        Relic::Lantern,
        Relic::RegalPillow,
        Relic::BagOfPreparation,
        Relic::AncientTeaSet,
        Relic::SmilingMask,
        Relic::PotionBelt,
        Relic::PreservedInsect,
        Relic::Omamori,
        Relic::MawBank,
        Relic::ArtOfWar,
        Relic::ToyOrnithopter,
        Relic::CeramicFish,
        Relic::Vajra,
        Relic::CentennialPuzzle,
        Relic::Strawberry,
        Relic::HappyFlower,
        Relic::OddlySmoothStone,
        Relic::WarPaint,
        Relic::BronzeScales,
        Relic::JuzuBracelet,
        Relic::DreamCatcher,
        Relic::Nunchaku,
        Relic::TinyChest,
        Relic::Orichalcum,
        Relic::Anchor,
        Relic::BagOfMarbles,
        Relic::DataDisk,
    ],
    uncommon_relic_pool: &[
        Relic::BottledTornado,
        Relic::Sundial,
        Relic::Kunai,
        Relic::Pear,
        Relic::BlueCandle,
        Relic::EternalFeather,
        Relic::StrikeDummy,
        Relic::SingingBowl,
        Relic::Matryoshka,
        Relic::InkBottle,
        Relic::TheCourier,
        Relic::FrozenEgg,
        Relic::OrnamentalFan,
        Relic::BottledLightning,
        Relic::GremlinHorn,
        Relic::HornCleat,
        Relic::ToxicEgg,
        Relic::LetterOpener,
        Relic::QuestionCard,
        Relic::BottledFlame,
        Relic::Shuriken,
        Relic::MoltenEgg,
        Relic::MeatOnTheBone,
        Relic::DarkstonePeriapt,
        Relic::MummifiedHand,
        Relic::Pantograph,
        Relic::WhiteBeastStatue,
        Relic::MercuryHourglass,
        Relic::SymbioticVirus,
        Relic::GoldPlatedCables,
    ],
    rare_relic_pool: &[
        Relic::Ginger,
        Relic::OldCoin,
        Relic::BirdFacedUrn,
        Relic::UnceasingTop,
        Relic::Torii,
        Relic::StoneCalendar,
        Relic::Shovel,
        Relic::WingBoots,
        Relic::ThreadAndNeedle,
        Relic::Turnip,
        Relic::IceCream,
        Relic::Calipers,
        Relic::LizardTail,
        Relic::PrayerWheel,
        Relic::Girya,
        Relic::DeadBranch,
        Relic::DuVuDoll,
        Relic::Pocketwatch,
        Relic::Mango,
        Relic::IncenseBurner,
        Relic::GamblingChip,
        Relic::PeacePipe,
        Relic::CaptainsWheel,
        Relic::FossilizedHelix,
        Relic::TungstenRod,
        Relic::EmotionChip,
    ],
    shop_relic_pool: &[
        Relic::SlingOfCourage,
        Relic::HandDrill,
        Relic::Toolbox,
        Relic::ChemicalX,
        Relic::LeesWaffle,
        Relic::Orrery,
        Relic::DollysMirror,
        Relic::OrangePellets,
        Relic::PrismaticShard,
        Relic::ClockworkSouvenir,
        Relic::FrozenEye,
        Relic::TheAbacus,
        Relic::MedicalKit,
        Relic::Cauldron,
        Relic::StrangeSpoon,
        Relic::MembershipCard,
        Relic::RunicCapacitor,
    ],
    boss_relic_pool: &[
        Relic::FusionHammer,
        Relic::VelvetChoker,
        Relic::RunicDome,
        Relic::SlaversCollar,
        Relic::SneckoEye,
        Relic::PandorasBox,
        Relic::CursedKey,
        Relic::BustedCrown,
        Relic::Ectoplasm,
        Relic::TinyHouse,
        Relic::Sozu,
        Relic::PhilosophersStone,
        Relic::Astrolabe,
        Relic::BlackStar,
        Relic::SacredBark,
        Relic::EmptyCage,
        Relic::RunicPyramid,
        Relic::CallingBell,
        Relic::CoffeeDripper,
        Relic::Inserter,
        Relic::FrozenCore,
        Relic::NuclearBattery,
    ],
};

pub const WATCHER: &Character = &Character {
    starting_hp: 72,
    starting_relic: Relic::PureWater,
    starting_deck: &[
        Card::Strike(false),
        Card::Strike(false),
        Card::Strike(false),
        Card::Strike(false),
        Card::Defend(false),
        Card::Defend(false),
        Card::Defend(false),
        Card::Defend(false),
        Card::Eruption(false),
        Card::Vigilance(false),
    ],
    common_card_pool: &[
        Card::Consecrate(false),
        Card::BowlingBash(false),
        Card::FlyingSleeves(false),
        Card::Halt(false),
        Card::JustLucky(false),
        Card::FlurryOfBlows(false),
        Card::Protect(false),
        Card::ThirdEye(false),
        Card::Crescendo(false),
        Card::Tranquility(false),
        Card::EmptyBody(false),
        Card::SashWhip(false),
        Card::CutThroughFate(false),
        Card::FollowUp(false),
        Card::PressurePoints(false),
        Card::CrushJoints(false),
        Card::Evaluate(false),
        Card::Prostrate(false),
        Card::EmptyFist(false),
    ],
    uncommon_card_pool: &[
        Card::WheelKick(false),
        Card::SimmeringFury(false),
        Card::Foresight(false),
        Card::Sanctity(false),
        Card::TalkToTheHand(false),
        Card::BattleHymn(false),
        Card::Indignation(false),
        Card::WindmillStrike(false),
        Card::ForeignInfluence(false),
        Card::LikeWater(false),
        Card::Fasting(false),
        Card::CarveReality(false),
        Card::Wallop(false),
        Card::WreathOfFlame(false),
        Card::Collect(false),
        Card::InnerPeace(false),
        Card::Rushdown(false),
        Card::DeceiveReality(false),
        Card::MentalFortress(false),
        Card::ReachHeaven(false),
        Card::FearNoEvil(false),
        Card::SandsOfTime(false),
        Card::WaveOfTheHand(false),
        Card::Study(false),
        Card::Meditate(false),
        Card::Perserverance(false),
        Card::Swivel(false),
        Card::Worship(false),
        Card::Conclude(false),
        Card::Tantrum(false),
        Card::Nirvana(false),
        Card::EmptyMind(false),
        Card::Weave(false),
        Card::SignatureMove(false),
        Card::Pray(false),
    ],
    rare_card_pool: &[
        Card::DeusExMachina(false),
        Card::DevaForm(false),
        Card::SpiritShield(false),
        Card::Establishment(false),
        Card::Omniscience(false),
        Card::Wish(false),
        Card::Alpha(false),
        Card::Vault(false),
        Card::Scrawl(false),
        Card::LessonLearned(false),
        Card::Ragnarok(false),
        Card::Blasphemy(false),
        Card::Devotion(false),
        Card::Brilliance(false),
        Card::MasterReality(false),
        Card::ConjureBlade(false),
        Card::Judgment(false),
    ],
    attack_card_pool: &[], // TODO: Add attack cards
    skill_card_pool: &[],  // TODO: Add skill cards
    power_card_pool: &[],  // TODO: Add power cards
    potion_pool: &[
        Potion::BottledMiracle,
        Potion::StancePotion,
        Potion::Ambrosia,
        Potion::BlockPotion,
        Potion::DexterityPotion,
        Potion::EnergyPotion,
        Potion::ExplosivePotion,
        Potion::FirePotion,
        Potion::StrengthPotion,
        Potion::SwiftPotion,
        Potion::WeakPotion,
        Potion::FearPotion,
        Potion::AttackPotion,
        Potion::SkillPotion,
        Potion::PowerPotion,
        Potion::ColorlessPotion,
        Potion::FlexPotion,
        Potion::SpeedPotion,
        Potion::BlessingOfTheForge,
        Potion::RegenPotion,
        Potion::AncientPotion,
        Potion::LiquidBronze,
        Potion::GamblersBrew,
        Potion::EssenceOfSteel,
        Potion::DuplicationPotion,
        Potion::DistilledChaos,
        Potion::LiquidMemories,
        Potion::CultistPotion,
        Potion::FruitJuice,
        Potion::SneckoOil,
        Potion::FairyInABottle,
        Potion::SmokeBomb,
        Potion::EntropicBrew,
    ],
    common_relic_pool: &[
        Relic::Whetstone,
        Relic::TheBoot,
        Relic::BloodVial,
        Relic::MealTicket,
        Relic::PenNib,
        Relic::Akabeko,
        Relic::Lantern,
        Relic::RegalPillow,
        Relic::BagOfPreparation,
        Relic::AncientTeaSet,
        Relic::SmilingMask,
        Relic::PotionBelt,
        Relic::PreservedInsect,
        Relic::Omamori,
        Relic::MawBank,
        Relic::ArtOfWar,
        Relic::ToyOrnithopter,
        Relic::CeramicFish,
        Relic::Vajra,
        Relic::CentennialPuzzle,
        Relic::Strawberry,
        Relic::HappyFlower,
        Relic::OddlySmoothStone,
        Relic::WarPaint,
        Relic::BronzeScales,
        Relic::JuzuBracelet,
        Relic::DreamCatcher,
        Relic::Nunchaku,
        Relic::TinyChest,
        Relic::Orichalcum,
        Relic::Anchor,
        Relic::BagOfMarbles,
        Relic::Damaru,
    ],
    uncommon_relic_pool: &[
        Relic::BottledTornado,
        Relic::Sundial,
        Relic::Kunai,
        Relic::Pear,
        Relic::BlueCandle,
        Relic::EternalFeather,
        Relic::StrikeDummy,
        Relic::SingingBowl,
        Relic::Matryoshka,
        Relic::InkBottle,
        Relic::TheCourier,
        Relic::FrozenEgg,
        Relic::OrnamentalFan,
        Relic::BottledLightning,
        Relic::GremlinHorn,
        Relic::HornCleat,
        Relic::ToxicEgg,
        Relic::LetterOpener,
        Relic::QuestionCard,
        Relic::BottledFlame,
        Relic::Shuriken,
        Relic::MoltenEgg,
        Relic::MeatOnTheBone,
        Relic::DarkstonePeriapt,
        Relic::MummifiedHand,
        Relic::Pantograph,
        Relic::WhiteBeastStatue,
        Relic::MercuryHourglass,
        Relic::Duality,
        Relic::TeardropLocket,
    ],
    rare_relic_pool: &[
        Relic::Ginger,
        Relic::OldCoin,
        Relic::BirdFacedUrn,
        Relic::UnceasingTop,
        Relic::Torii,
        Relic::StoneCalendar,
        Relic::Shovel,
        Relic::WingBoots,
        Relic::ThreadAndNeedle,
        Relic::Turnip,
        Relic::IceCream,
        Relic::Calipers,
        Relic::LizardTail,
        Relic::PrayerWheel,
        Relic::Girya,
        Relic::DeadBranch,
        Relic::DuVuDoll,
        Relic::Pocketwatch,
        Relic::Mango,
        Relic::IncenseBurner,
        Relic::GamblingChip,
        Relic::PeacePipe,
        Relic::CaptainsWheel,
        Relic::FossilizedHelix,
        Relic::TungstenRod,
        Relic::CloakClasp,
        Relic::GoldenEye,
    ],
    shop_relic_pool: &[
        Relic::SlingOfCourage,
        Relic::HandDrill,
        Relic::Toolbox,
        Relic::ChemicalX,
        Relic::LeesWaffle,
        Relic::Orrery,
        Relic::DollysMirror,
        Relic::OrangePellets,
        Relic::PrismaticShard,
        Relic::ClockworkSouvenir,
        Relic::FrozenEye,
        Relic::TheAbacus,
        Relic::MedicalKit,
        Relic::Cauldron,
        Relic::StrangeSpoon,
        Relic::MembershipCard,
        Relic::Melange,
    ],
    boss_relic_pool: &[
        Relic::FusionHammer,
        Relic::VelvetChoker,
        Relic::RunicDome,
        Relic::SlaversCollar,
        Relic::SneckoEye,
        Relic::PandorasBox,
        Relic::CursedKey,
        Relic::BustedCrown,
        Relic::Ectoplasm,
        Relic::TinyHouse,
        Relic::Sozu,
        Relic::PhilosophersStone,
        Relic::Astrolabe,
        Relic::BlackStar,
        Relic::SacredBark,
        Relic::EmptyCage,
        Relic::RunicPyramid,
        Relic::CallingBell,
        Relic::CoffeeDripper,
        Relic::HolyWater,
        Relic::VioletLotus,
    ],
};

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_character_try_from() {
        assert_eq!(
            <&'static Character>::try_from("Ironclad").unwrap(),
            IRONCLAD
        );
        assert_eq!(<&'static Character>::try_from("Silent").unwrap(), SILENT);
        assert_eq!(<&'static Character>::try_from("Defect").unwrap(), DEFECT);
        assert_eq!(<&'static Character>::try_from("Watcher").unwrap(), WATCHER);
        assert_eq!(<&'static Character>::try_from("watcher").unwrap(), WATCHER);
        assert!(<&'static Character>::try_from("Unknown").is_err());
        assert!(<&'static Character>::try_from("").is_err());
    }
}
