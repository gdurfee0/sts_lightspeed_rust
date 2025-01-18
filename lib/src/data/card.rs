// Source: Slay the Spire Wiki (https://slay-the-spire.fandom.com/wiki/Category:Cards)

use once_cell::sync::Lazy;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::Energy;

use super::buff::Buff;
use super::debuff::Debuff;
use super::effect::PlayerEffect;
use super::orb::Orb;
use super::stance::Stance;

#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, Ord, PartialEq, PartialOrd)]
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

pub const CURSE_CARD_POOL: &[Card] = &[
    Card::Regret,
    Card::Injury,
    Card::Shame,
    Card::Parasite,
    Card::Normality,
    Card::Doubt,
    Card::Writhe,
    Card::Pain,
    Card::Decay,
    Card::Clumsy,
];

#[derive(Clone, Copy, Debug)]
pub enum CardType {
    Attack,
    Curse,
    Power,
    Skill,
    Status,
}

#[derive(Debug)]
pub struct CardDetails {
    pub card: Card,
    pub type_: CardType,
    pub cost: Energy,
    pub effect_chain: Vec<PlayerEffect>,
    pub ethereal: bool,
    pub exhaust: bool,
    pub innate: bool,
    pub irremovable: bool,
    pub requires_target: bool,
    pub special_cost: bool,
    pub unplayable: bool,
}

#[allow(dead_code)] // Clippy doesn't seem to notice that all methods are used in the macro.
impl CardDetails {
    pub fn for_card(card: Card) -> &'static CardDetails {
        CARD_DETAILS[card as usize]
    }

    fn new(card: Card, type_: CardType, cost: Energy) -> CardDetails {
        CardDetails {
            card,
            type_,
            cost,
            ethereal: false,
            effect_chain: Vec::new(),
            exhaust: false,
            innate: false,
            irremovable: false,
            requires_target: false,
            special_cost: false,
            unplayable: false,
        }
    }

    fn ethereal(mut self) -> CardDetails {
        self.ethereal = true;
        self
    }

    fn exhaust(mut self) -> CardDetails {
        self.exhaust = true;
        self
    }

    fn irremovable(mut self) -> CardDetails {
        self.irremovable = true;
        self
    }

    fn innate(mut self) -> CardDetails {
        self.innate = true;
        self
    }

    fn requires_target(mut self) -> CardDetails {
        self.requires_target = true;
        self
    }

    fn special_cost(mut self) -> CardDetails {
        self.special_cost = true;
        self
    }

    fn unplayable(mut self) -> CardDetails {
        self.unplayable = true;
        self
    }

    fn push(mut self, effect: PlayerEffect) -> CardDetails {
        self.effect_chain.push(effect);
        self
    }
}

macro_rules! define_card {
    (($card:ident, $type:ident, $cost:expr), [$($eff:ident($($param:expr),*)),*]) => {
        CardDetails::new(Card::$card, CardType::$type, $cost)
            $(.push(PlayerEffect::$eff($($param,)*)))*
    };
    (
        ($card:ident, $type:ident, $cost:expr),
        [$($eff:ident($($param:expr),*)),*],
        $($extra:ident)*
    ) => {
        CardDetails::new(Card::$card, CardType::$type, $cost)
            $(.push(PlayerEffect::$eff($($param,)*)))* $(.$extra())*
    };
    (
        ($card:ident, $type:ident, $cost:expr),
        [$($eff:ident($($param:expr),*)),*],
        [$($extra:ident),*]
    ) => {
        CardDetails::new(Card::$card, CardType::$type, $cost)
            $(.push(PlayerEffect::$eff($($param,)*)))* $(.$extra())*
    };
}

static ALL_CARDS: Lazy<Vec<CardDetails>> = Lazy::new(|| {
    vec![
        define_card!((Accuracy, Power, 1), [Buff(Buff::Accuracy, 4)]),
        define_card!((Acrobatics, Skill, 1), [Draw(3), Discard(1)]),
        define_card!((Adrenaline, Skill, 1), [GainEnergy(1), Draw(2)], exhaust),
        define_card!((AfterImage, Power, 1), [Buff(Buff::AfterImage, 1)]),
        define_card!((Aggregate, Skill, 1), [GainEnergyCustom(Card::Aggregate)]),
        define_card!((Alchemize, Skill, 1), [ObtainRandomPotion()], exhaust),
        define_card!(
            (AllForOne, Attack, 2),
            [DealDamage(10), HandCustom(Card::AllForOne)],
            requires_target
        ),
        define_card!(
            (AllOutAttack, Attack, 1),
            [DealDamageToAll(10), DiscardAtRandom()],
            requires_target
        ),
        define_card!(
            (Alpha, Skill, 1),
            [ShuffleIntoDrawPile(&[Card::Alpha])],
            exhaust
        ),
        define_card!((Amplify, Skill, 1), [Buff(Buff::Amplify, 1)]),
        define_card!(
            (Anger, Attack, 0),
            [DealDamage(6), AddToDiscardPile(&[Card::Anger])],
            requires_target
        ),
        define_card!((Apotheosis, Skill, 2), [UpgradeAllCardsInCombat()], exhaust),
        define_card!(
            (Apparition, Skill, 1),
            [Buff(Buff::Intangible, 1)],
            [ethereal, exhaust]
        ),
        define_card!(
            (Armaments, Skill, 1),
            [GainBlock(5), UpgradeOneCardInCombat()]
        ),
        define_card!(
            (AscendersBane, Curse, 0),
            [],
            [ethereal, irremovable, unplayable]
        ),
        define_card!((AThousandCuts, Power, 1), [Buff(Buff::ThousandCuts, 1)]),
        define_card!(
            (AutoShields, Skill, 1),
            [GainBlockCustom(Card::AutoShields)]
        ),
        define_card!((Backflip, Skill, 1), [GainBlock(5), Draw(2)]),
        define_card!(
            (Backstab, Attack, 0),
            [DealDamage(11)],
            [exhaust, innate, requires_target]
        ),
        define_card!(
            (BallLightning, Attack, 1),
            [DealDamage(7), Channel(Orb::Lightning, 1)],
            requires_target
        ),
        define_card!((BandageUp, Skill, 0), [Heal(4)], exhaust),
        define_card!(
            (Barrage, Attack, 1),
            [DealDamageCustom(Card::Barrage)],
            requires_target
        ),
        define_card!((Barricade, Power, 3), [Buff(Buff::Barricade, 1)]),
        define_card!(
            (Bash, Attack, 2),
            [DealDamage(8), Debuff(Debuff::Vulnerable, 2)],
            requires_target
        ),
        define_card!((BattleHymn, Power, 1), [Buff(Buff::BattleHymn, 1)]),
        define_card!(
            (BattleTrance, Skill, 0),
            [Draw(3), DebuffSelf(Debuff::NoDraw, 1)]
        ),
        define_card!(
            (BeamCell, Skill, 0),
            [DealDamage(3), Debuff(Debuff::Vulnerable, 1)]
        ),
        define_card!(
            (Berserk, Power, 0),
            [DebuffSelf(Debuff::Vulnerable, 2), Buff(Buff::Berserk, 1)]
        ),
        define_card!(
            (Beta, Skill, 1),
            [ShuffleIntoDrawPile(&[Card::Omega])],
            exhaust
        ),
        define_card!(
            (BiasedCognition, Power, 1),
            [GainFocus(4), DebuffSelf(Debuff::Bias, 1)]
        ),
        define_card!((Bite, Attack, 1), [DealDamage(7), Heal(2)], requires_target),
        define_card!(
            (BladeDance, Skill, 1),
            [AddToHand(&[Card::Shiv, Card::Shiv, Card::Shiv])]
        ),
        define_card!(
            (Blasphemy, Skill, 3),
            [EnterStance(Stance::Divinity), Buff(Buff::Blasphemer, 1)]
        ),
        define_card!(
            (Blind, Skill, 0),
            [Debuff(Debuff::Weak, 1)],
            requires_target
        ),
        define_card!(
            (Blizzard, Skill, 1),
            [DealDamageToAllCustom(Card::Blizzard)]
        ),
        define_card!(
            (BloodForBlood, Attack, 4),
            [DealDamage(18)],
            [requires_target, special_cost]
        ),
        define_card!(
            (Bloodletting, Skill, 0),
            [LoseHp(3), GainEnergy(2)],
            exhaust
        ),
        define_card!((Bludgeon, Attack, 3), [DealDamage(32)], requires_target),
        define_card!((Blur, Skill, 1), [GainBlock(5), Buff(Buff::Blur, 1)]),
        define_card!(
            (BodySlam, Attack, 1),
            [DealDamageCustom(Card::BodySlam)],
            requires_target
        ),
        define_card!((BootSequence, Skill, 0), [GainBlock(10)], innate),
        define_card!(
            (BouncingFlask, Skill, 2),
            [DealDamageToAllCustom(Card::BouncingFlask)]
        ),
        define_card!(
            (BowlingBash, Attack, 1),
            [DealDamageCustom(Card::BowlingBash)],
            requires_target
        ),
        define_card!(
            (Brilliance, Skill, 1),
            [DealDamageCustom(Card::Brilliance)],
            requires_target
        ),
        define_card!((Brutality, Power, 0), [Buff(Buff::Brutality, 1)]),
        define_card!((Buffer, Skill, 2), [Buff(Buff::Buffer, 1)]),
        define_card!(
            (BulletTime, Skill, 3),
            [HandCustom(Card::BulletTime), DebuffSelf(Debuff::NoDraw, 1)]
        ),
        define_card!(
            (Bullseye, Attack, 1),
            [DealDamage(8), Debuff(Debuff::LockOn, 2)],
            requires_target
        ),
        define_card!((Burn, Status, 0), [], unplayable),
        define_card!((BurningPact, Skill, 1), [ExhaustCard(), Draw(2)]),
        define_card!((Burst, Skill, 1), [Buff(Buff::Burst, 1)]),
    ]
});

static CARD_DETAILS: Lazy<Vec<&'static CardDetails>> = Lazy::new(|| {
    let mut cards = Vec::new();
    for card in Card::iter() {
        if let Some(details) = ALL_CARDS.iter().find(|details| details.card == card) {
            cards.push(details);
        } else {
            panic!("CardDetails not found for {:?}", card);
        }
    }
    cards
});
