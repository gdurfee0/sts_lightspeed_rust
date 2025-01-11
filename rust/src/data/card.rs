use std::fmt;

pub const UNCOMMON_COLORLESS_CARDS: &[Card] = &[
    Card::BandageUp,
    Card::Blind,
    Card::DarkShackles,
    Card::DeepBreath,
    Card::Discovery,
    Card::DramaticEntrance,
    Card::Enlightenment,
    Card::Finesse,
    Card::FlashOfSteel,
    Card::Forethought,
    Card::GoodInstincts,
    Card::Impatience,
    Card::JackOfAllTrades,
    Card::Madness,
    Card::MindBlast,
    Card::Panacea,
    Card::PanicButton,
    Card::Purity,
    Card::SwiftStrike,
    Card::Trip,
];

pub const RARE_COLORLESS_CARDS: &[Card] = &[
    Card::Apotheosis,
    Card::Chrysalis,
    Card::HandOfGreed,
    Card::MasterOfStrategy,
    Card::Metamorphosis,
    Card::SecretTechnique,
    Card::SecretWeapon,
    Card::TheBomb,
    Card::ThinkingAhead,
    Card::Transmutation,
    Card::Violence,
];

pub const SPECIAL_COLORLESS_CARDS: &[Card] = &[
    Card::Apparition,
    Card::Beta,
    Card::Bite,
    Card::Expunger,
    Card::Insight,
    Card::Miracle,
    Card::Omega,
    Card::RitualDagger,
    Card::Safety,
    Card::Shiv,
    Card::Smite,
    Card::ThroughViolence,
];

pub const STATUS_CARDS: &[Card] = &[
    Card::Burn,
    Card::Dazed,
    Card::Slimed,
    Card::Void,
    Card::Wound,
];

pub const CURSE_CARDS: &[Card] = &[
    Card::AscendersBane,
    Card::Clumsy,
    Card::CurseOfTheBell,
    Card::Decay,
    Card::Doubt,
    Card::Injury,
    Card::Necronomicurse,
    Card::Normality,
    Card::Pain,
    Card::Parasite,
    Card::Pride,
    Card::Regret,
    Card::Shame,
    Card::Writhe,
];

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Card {
    Accuracy,
    Acrobatics,
    Adrenaline,
    AfterImage,
    Aggregate,
    Alchemize,
    AllForOne,
    AllOutAttack,
    Alpha,
    Amplify,
    Anger,
    Apotheosis,
    Apparition,
    Armaments,
    AscendersBane,
    AThousandCuts,
    AutoShields,
    Backflip,
    Backstab,
    BallLightning,
    BandageUp,
    Bane,
    Barrage,
    Barricade,
    Bash,
    BattleHymn,
    BattleTrance,
    BeamCell,
    Berserk,
    Beta,
    BiasedCognition,
    Bite,
    BladeDance,
    Blasphemy,
    Blind,
    Blizzard,
    BloodForBlood,
    Bloodletting,
    Bludgeon,
    Blur,
    BodySlam,
    BootSequence,
    BouncingFlask,
    BowlingBash,
    Brilliance,
    Brutality,
    Buffer,
    BulletTime,
    Bullseye,
    Burn,
    BurningPact,
    Burst,
    CalculatedGamble,
    Caltrops,
    Capacitor,
    Carnage,
    CarveReality,
    Catalyst,
    Chaos,
    ChargeBattery,
    Chill,
    Choke,
    Chrysalis,
    Clash,
    Claw,
    Cleave,
    CloakAndDagger,
    Clothesline,
    Clumsy,
    ColdSnap,
    Collect,
    Combust,
    CompileDriver,
    Concentrate,
    Conclude,
    ConjureBlade,
    Consecrate,
    Consume,
    Coolheaded,
    CoreSurge,
    CorpseExplosion,
    Corruption,
    CreativeAi,
    Crescendo,
    CripplingCloud,
    CrushJoints,
    CurseOfTheBell,
    CutThroughFate,
    DaggerSpray,
    DaggerThrow,
    DarkEmbrace,
    Darkness,
    DarkShackles,
    Dash,
    Dazed,
    DeadlyPoison,
    Decay,
    DeceiveRealiry,
    DeepBreath,
    Defend,
    Deflect,
    Defragment,
    DemonForm,
    DeusExMachina,
    DevaForm,
    Devotion,
    DieDieDie,
    Disarm,
    Discovery,
    Distraction,
    DodgeAndRoll,
    DoomAndGloom,
    Doppelganger,
    DoubleEnergy,
    DoubleTap,
    Doubt,
    DramaticEntrance,
    Dropkick,
    Dualcast,
    DualWield,
    EchoForm,
    Electrodynamics,
    EmptyBody,
    EmptyFist,
    EmptyMind,
    EndlessAgony,
    Enlightenment,
    Entrench,
    Envenom,
    Equilibrium,
    Eruption,
    EscapePlan,
    Establishment,
    Evaluate,
    Eviscerate,
    Evolve,
    Exhume,
    Expertise,
    Expunger,
    Fasting,
    FearNoEvil,
    Feed,
    FeelNoPain,
    FiendFire,
    Finesse,
    Finisher,
    FireBreathing,
    Fission,
    FlameBarrier,
    FlashOfSteel,
    Flechettes,
    Flex,
    FlurryOfBlows,
    FlyingKnee,
    FlyingSleeves,
    FollowUp,
    Footwork,
    ForceField,
    ForeignInfluence,
    Foresight,
    Forethought,
    Ftl,
    Fusion,
    GeneticAlgorithm,
    GhostlyArmor,
    Glacier,
    GlassKnife,
    GoForTheEyes,
    GoodInstincts,
    GrandFinale,
    Halt,
    HandOfGreed,
    Havoc,
    Headbutt,
    Heatsinks,
    HeavyBlade,
    HeelHook,
    HelloWorld,
    Hemokinesis,
    Hologram,
    Hyperbeam,
    Immolate,
    Impatience,
    Impervious,
    Indignation,
    InfernalBlade,
    InfiniteBlades,
    Inflame,
    Injury,
    InnerPeace,
    Insight,
    Intimidate,
    IronWave,
    JackOfAllTrades,
    Jax,
    Judgment,
    Juggernaut,
    JustLucky,
    Leap,
    LegSweep,
    LessonLearned,
    LikeWater,
    LimitBreak,
    Loop,
    MachineLearning,
    Madness,
    Magnetism,
    Malaise,
    MasterfulStab,
    MasterOfStrategy,
    MasterReality,
    Mayhem,
    Meditate,
    Melter,
    MentalFortress,
    Metallicize,
    Metamorphosis,
    MeteorStrike,
    MindBlast,
    Miracle,
    MultiCast,
    Necronomicurse,
    Neutralize,
    Nightmare,
    Nirvana,
    Normality,
    NoxiousFumes,
    Offering,
    Omega,
    Omniscience,
    Outmaneuver,
    Overclock,
    Pain,
    Panacea,
    Panache,
    PanicButton,
    Parasite,
    PerfectedStrike,
    Perserverance,
    PhantasmalKiller,
    PiercingWail,
    PoisonedStab,
    PommelStrike,
    PowerThrough,
    Pray,
    Predator,
    Prepared,
    PressurePoints,
    Pride,
    Prostrate,
    Protect,
    Pummel,
    Purity,
    QuickSlash,
    Rage,
    Ragnarok,
    Rainbow,
    Rampage,
    ReachHeaven,
    Reaper,
    Reboot,
    Rebound,
    RecklessCharge,
    Recursion,
    Recycle,
    Reflex,
    Regret,
    ReinforcedBody,
    Reprogram,
    RiddleWithHoles,
    RipAndTear,
    RitualDagger,
    Rupture,
    Rushdown,
    SadisticNature,
    Safety,
    Sanctity,
    SandsOfTime,
    SashWhip,
    Scrape,
    Scrawl,
    SearingBlow,
    SecondWind,
    SecretTechnique,
    SecretWeapon,
    SeeingRed,
    Seek,
    SelfRepair,
    Sentinel,
    Setup,
    SeverSoul,
    Shame,
    SimmeringFury,
    Shiv,
    Shockwave,
    ShrugItOff,
    SignatureMove,
    Skewer,
    Skim,
    Slice,
    Slimed,
    Smite,
    SneakyStrike,
    SpiritShield,
    SpotWeakness,
    Stack,
    StaticDischarge,
    SteamBarrier,
    Storm,
    StormOfSteel,
    Streamline,
    Strike,
    Study,
    SuckerPunch,
    Sunder,
    Survivor,
    SweepingBeam,
    SwiftStrike,
    Swivel,
    SwordBoomerang,
    Tactician,
    TalkToTheHand,
    Tantrum,
    Tempest,
    Terror,
    TheBomb,
    ThinkingAhead,
    ThirdEye,
    ThroughViolence,
    Thunderclap,
    ThunderStrike,
    ToolsOfTheTrade,
    Tranquility,
    Transmutation,
    Trip,
    TrueGrit,
    Turbo,
    TwinStrike,
    Unload,
    Uppercut,
    Vault,
    Vigilance,
    Violence,
    Void,
    Wallop,
    Warcry,
    WaveOfTheHand,
    Weave,
    WellLaidPlans,
    WheelKick,
    Whirlwind,
    WhiteNoise,
    WildStrike,
    WindmillStrike,
    Wish,
    Worship,
    Wound,
    WraithForm,
    WreathOfFlame,
    Writhe,
    Zap,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: add en-us names for my convenience
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_no_duplicates() {
        let mut all_cards = UNCOMMON_COLORLESS_CARDS
            .iter()
            .chain(RARE_COLORLESS_CARDS.iter())
            .chain(SPECIAL_COLORLESS_CARDS.iter())
            .chain(STATUS_CARDS.iter())
            .chain(CURSE_CARDS.iter())
            .collect::<Vec<_>>();
        all_cards.sort();
        let initial_cards = all_cards.clone();
        all_cards.dedup();
        assert_eq!(all_cards, initial_cards);
    }
}
