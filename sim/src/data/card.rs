#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Card {
    Accuracy,
    Acrobatics,
    Adrenaline,
    AfterImmage,
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
    Blizazrd,
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
    Judgement,
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
    PiercingWall,
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
    Sanctify,
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
    ShimmeringFury,
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
