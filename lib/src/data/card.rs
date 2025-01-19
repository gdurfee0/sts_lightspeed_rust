// Source: Slay the Spire Wiki (https://slay-the-spire.fandom.com/wiki/Category:Cards)

use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::types::Energy;

use super::buff::Buff;
use super::debuff::Debuff;
use super::effect::PlayerEffect;
use super::orb::Orb;
use super::stance::Stance;

// TODO: Use Card(Energy, bool) for all of these, and Card(Energy, u32) for SearingBlow.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
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

impl Card {
    pub fn cost(&self) -> Energy {
        CardDetails::for_card(*self).cost
    }

    pub fn exhausts(&self) -> bool {
        CardDetails::for_card(*self).exhaust
    }

    pub fn is_innate(&self) -> bool {
        CardDetails::for_card(*self).innate
    }
}

#[derive(Debug)]
pub struct CardDetails {
    pub card: Card,
    pub type_: CardType,
    pub cost: Energy,
    pub effect_chain: Vec<PlayerEffect>,
    pub custom_requirements: bool,
    pub dup_on_draw: bool,
    pub enact_on_draw: bool,
    pub enact_on_turn_end: bool,
    pub ethereal: bool,
    pub exhaust: bool,
    pub innate: bool,
    pub irremovable: bool,
    pub requires_target: bool,
    pub retain: bool,
    pub special_cost: bool,
    pub unplayable: bool,
}

#[allow(dead_code)] // Clippy doesn't seem to notice that all methods are used in the macro.
impl CardDetails {
    pub fn for_card(card: Card) -> &'static CardDetails {
        CARD_DETAILS
            .get(&card)
            .unwrap_or_else(|| panic!("No details for card {:?}", card))
    }

    fn new(card: Card, type_: CardType, cost: Energy) -> CardDetails {
        CardDetails {
            card,
            type_,
            cost,
            effect_chain: Vec::new(),
            custom_requirements: false,
            dup_on_draw: false,
            enact_on_draw: false,
            enact_on_turn_end: false,
            ethereal: false,
            exhaust: false,
            innate: false,
            irremovable: false,
            requires_target: false,
            retain: false,
            special_cost: false,
            unplayable: false,
        }
    }

    fn custom_requirements(mut self) -> CardDetails {
        self.custom_requirements = true;
        self
    }

    fn dup_on_draw(mut self) -> CardDetails {
        self.dup_on_draw = true;
        self
    }

    fn enact_on_draw(mut self) -> CardDetails {
        self.enact_on_draw = true;
        self
    }

    fn enact_on_turn_end(mut self) -> CardDetails {
        self.enact_on_turn_end = true;
        self
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

    fn retain(mut self) -> CardDetails {
        self.retain = true;
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
    (($card:ident, $type:ident, $cost:expr), [$($effect:ident($($param:expr),*)),*]) => {
        CardDetails::new(Card::$card, CardType::$type, $cost)
            $(.push(PlayerEffect::$effect($($param,)*)))*
    };
    (
        ($card:ident, $type:ident, $cost:expr),
        [$($effect:ident($($param:expr),*)),*],
        $($extra:ident)*
    ) => {
        CardDetails::new(Card::$card, CardType::$type, $cost)
            $(.push(PlayerEffect::$effect($($param,)*)))* $(.$extra())*
    };
    (
        ($card:ident, $type:ident, $cost:expr),
        [$($effect:ident($($param:expr),*)),*],
        [$($extra:ident),*]
    ) => {
        CardDetails::new(Card::$card, CardType::$type, $cost)
            $(.push(PlayerEffect::$effect($($param,)*)))* $(.$extra())*
    };
}

static ALL_CARDS: Lazy<Vec<CardDetails>> = Lazy::new(|| {
    vec![
        define_card!((Accuracy, Power, 1), [Buff(Buff::Accuracy, 4)]),
        define_card!((Acrobatics, Skill, 1), [Draw(3), Discard(1)]),
        define_card!((Adrenaline, Skill, 1), [GainEnergy(1), Draw(2)], exhaust),
        define_card!((AfterImage, Power, 1), [Buff(Buff::AfterImage, 1)]),
        define_card!((Aggregate, Skill, 1), [GainEnergyCustom()]),
        define_card!((Alchemize, Skill, 1), [ObtainRandomPotion()], exhaust),
        define_card!(
            (AllForOne, Attack, 2),
            [DealDamage(10), HandCustom()],
            requires_target
        ),
        define_card!(
            (AllOutAttack, Attack, 1),
            [DealDamageToAll(10), DiscardAtRandom()],
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
            (AscendersBane, Curse, 1),
            [],
            [ethereal, irremovable, unplayable]
        ),
        define_card!((AThousandCuts, Power, 1), [Buff(Buff::ThousandCuts, 1)]),
        define_card!((AutoShields, Skill, 1), [GainBlockCustom()]),
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
            (Bane, Attack, 1),
            [DealDamage(7), DealDamageCustom()],
            requires_target
        ),
        define_card!((Barrage, Attack, 1), [DealDamageCustom()], requires_target),
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
            [DealDamage(3), Debuff(Debuff::Vulnerable, 1)],
            requires_target
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
        define_card!((Blizzard, Skill, 1), [DealDamageToAllCustom()]),
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
        define_card!((BodySlam, Attack, 1), [DealDamageCustom()], requires_target),
        define_card!((BootSequence, Skill, 0), [GainBlock(10)], innate),
        define_card!((BouncingFlask, Skill, 2), [DealDamageToAllCustom()]),
        define_card!(
            (BowlingBash, Attack, 1),
            [DealDamageCustom()],
            requires_target
        ),
        define_card!(
            (Brilliance, Skill, 1),
            [DealDamageCustom()],
            requires_target
        ),
        define_card!((Brutality, Power, 0), [Buff(Buff::Brutality, 1)]),
        define_card!((Buffer, Skill, 2), [Buff(Buff::Buffer, 1)]),
        define_card!(
            (BulletTime, Skill, 3),
            [HandCustom(), DebuffSelf(Debuff::NoDraw, 1)]
        ),
        define_card!(
            (Bullseye, Attack, 1),
            [DealDamage(8), Debuff(Debuff::LockOn, 2)],
            requires_target
        ),
        define_card!(
            (Burn, Status, 1),
            [TakeDamage(2)],
            [enact_on_turn_end, unplayable]
        ),
        define_card!((BurningPact, Skill, 1), [ExhaustCard(), Draw(2)]),
        define_card!((Burst, Skill, 1), [Buff(Buff::Burst, 1)]),
        define_card!(
            (CalculatedGamble, Skill, 0),
            [DiscardCustom(), DrawCustom()],
            exhaust
        ),
        define_card!((Caltrops, Power, 1), [Buff(Buff::Thorns, 3)]),
        define_card!((Capacitor, Power, 1), [GainOrbSlots(2)]),
        define_card!(
            (Carnage, Attack, 2),
            [DealDamage(20)],
            [ethereal, requires_target]
        ),
        define_card!(
            (CarveReality, Attack, 1),
            [DealDamage(6), AddToHand(&[Card::Smite])],
            requires_target
        ),
        define_card!(
            (Catalyst, Skill, 1),
            [DebuffCustom()],
            [exhaust, requires_target]
        ),
        define_card!((Chaos, Skill, 1), [ChannelRandom(1)]),
        define_card!(
            (ChargeBattery, Skill, 1),
            [GainBlock(7), Buff(Buff::Energized, 1)]
        ),
        define_card!((Chill, Skill, 0), [ChannelCustom()], exhaust),
        define_card!(
            (Choke, Attack, 2),
            [DealDamage(12), Debuff(Debuff::Choked, 3)],
            requires_target
        ),
        define_card!(
            (Chrysalis, Skill, 2),
            [ShuffleIntoDrawPileCustom()],
            exhaust
        ),
        define_card!(
            (Clash, Attack, 0),
            [DealDamage(14)],
            [custom_requirements, requires_target]
        ),
        define_card!(
            (Claw, Attack, 0),
            [DealDamageCustom(), Buff(Buff::ClawsPlayed, 1)],
            requires_target
        ),
        define_card!((Cleave, Attack, 1), [DealDamageToAll(8)]),
        define_card!(
            (CloakAndDagger, Skill, 1),
            [GainBlock(6), AddToHand(&[Card::Shiv])]
        ),
        define_card!(
            (Clothesline, Attack, 2),
            [DealDamage(12), Debuff(Debuff::Weak, 2)],
            requires_target
        ),
        define_card!((Clumsy, Curse, 1), [], [ethereal, unplayable]),
        define_card!(
            (ColdSnap, Attack, 1),
            [DealDamage(6), Channel(Orb::Frost, 1)],
            requires_target
        ),
        define_card!((Collect, Power, 0), [BuffCustom()], exhaust),
        define_card!((Combust, Power, 1), [Buff(Buff::Combust, 5)]),
        define_card!(
            (CompileDriver, Attack, 1),
            [DealDamage(7), DrawCustom()],
            requires_target
        ),
        define_card!((Concentrate, Skill, 0), [Discard(3), GainEnergy(2)]),
        define_card!((Conclude, Attack, 1), [DealDamageToAll(12), EndTurn()]),
        define_card!(
            (ConjureBlade, Skill, 0),
            [ShuffleIntoDrawPileCustom()],
            exhaust
        ),
        define_card!((Consecrate, Attack, 0), [DealDamageToAll(5)]),
        define_card!((Consume, Skill, 2), [GainFocus(2), LoseOrbSlots(1)]),
        define_card!((Coolheaded, Skill, 1), [Channel(Orb::Frost, 1), Draw(1)]),
        define_card!(
            (CoreSurge, Attack, 1),
            [DealDamage(11), Buff(Buff::Artifact, 1)],
            [exhaust, requires_target]
        ),
        define_card!(
            (CorpseExplosion, Attack, 2),
            [
                Debuff(Debuff::Poison, 6),
                Debuff(Debuff::CorpseExplosion, 1)
            ],
            requires_target
        ),
        define_card!((Corruption, Power, 3), [Buff(Buff::Corruption, 1)]),
        define_card!((CreativeAi, Power, 3), [Buff(Buff::CreativeAi, 1)]),
        define_card!((Crescendo, Skill, 1), [EnterStance(Stance::Wrath)], retain),
        define_card!(
            (CripplingCloud, Skill, 2),
            [DebuffAll(Debuff::Poison, 4), DebuffAll(Debuff::Weak, 2)],
            exhaust
        ),
        define_card!(
            (CrushJoints, Attack, 1),
            [DealDamage(8), DebuffCustom()],
            requires_target
        ),
        define_card!((CurseOfTheBell, Curse, 1), [], [irremovable, unplayable]),
        define_card!(
            (CutThroughFate, Attack, 1),
            [DealDamage(7), Scry(2), Draw(1)],
            requires_target
        ),
        define_card!(
            (DaggerSpray, Attack, 1),
            [DealDamageToAll(4), DealDamageToAll(4)]
        ),
        define_card!(
            (DaggerThrow, Attack, 1),
            [DealDamage(9), Draw(1), Discard(1)],
            requires_target
        ),
        define_card!((DarkEmbrace, Power, 2), [Buff(Buff::DarkEmbrace, 1)]),
        define_card!((Darkness, Skill, 1), [Channel(Orb::Dark, 1)]),
        define_card!(
            (DarkShackles, Skill, 0),
            [Debuff(Debuff::StrengthLossThisTurn, 9)],
            [exhaust, requires_target]
        ),
        define_card!(
            (Dash, Attack, 2),
            [DealDamage(10), GainBlock(10)],
            requires_target
        ),
        define_card!((Dazed, Status, 1), [], [ethereal, unplayable]),
        define_card!(
            (DeadlyPoison, Skill, 1),
            [Debuff(Debuff::Poison, 5)],
            requires_target
        ),
        define_card!(
            (Decay, Curse, 1),
            [TakeDamage(2)],
            [enact_on_turn_end, unplayable]
        ),
        define_card!(
            (DeceiveRealiry, Skill, 1),
            [GainBlock(4), AddToHand(&[Card::Safety])]
        ),
        define_card!(
            (DeepBreath, Skill, 0),
            [ShuffleIntoDrawPileCustom(), Draw(1)]
        ),
        define_card!((Defend, Skill, 1), [GainBlock(5)]),
        define_card!((Deflect, Skill, 0), [GainBlock(4)]),
        define_card!((Defragment, Power, 1), [GainFocus(1)]),
        define_card!((DemonForm, Power, 3), [Buff(Buff::DemonForm, 2)]),
        define_card!(
            (DeusExMachina, Skill, 0),
            [AddToHand(&[Card::Miracle, Card::Miracle])],
            [exhaust, enact_on_draw]
        ),
        define_card!((DevaForm, Power, 3), [Buff(Buff::Deva, 1)]),
        define_card!((Devotion, Power, 1), [Buff(Buff::Devotion, 2)]),
        define_card!((DieDieDie, Attack, 1), [DealDamageToAll(13)], exhaust),
        define_card!(
            (Disarm, Skill, 1),
            [SapStrength(2)],
            [exhaust, requires_target]
        ),
        define_card!((Discovery, Skill, 1), [HandCustom()], exhaust),
        define_card!((Distraction, Skill, 1), [HandCustom()], exhaust),
        define_card!(
            (DodgeAndRoll, Skill, 1),
            [GainBlock(4), Buff(Buff::BlockNextTurn, 4)]
        ),
        define_card!(
            (DoomAndGloom, Attack, 2),
            [DealDamageToAll(10), Channel(Orb::Dark, 1)]
        ),
        define_card!((Doppelganger, Skill, 0), [DrawCustom(), GainEnergyCustom()]),
        define_card!((DoubleEnergy, Skill, 1), [GainEnergyCustom()]),
        define_card!((DoubleTap, Skill, 1), [Buff(Buff::DoubleTap, 1)]),
        define_card!(
            (Doubt, Curse, 1),
            [DebuffSelf(Debuff::Weak, 1)],
            [enact_on_turn_end, unplayable]
        ),
        define_card!(
            (DramaticEntrance, Skill, 0),
            [DealDamageToAll(8)],
            [exhaust, innate]
        ),
        define_card!(
            (Dropkick, Attack, 1),
            [DealDamage(5), GainEnergyCustom(), DrawCustom()],
            requires_target
        ),
        define_card!((Dualcast, Skill, 1), [EvokeCustom()]),
        define_card!((DualWield, Skill, 1), [HandCustom()]),
        define_card!((EchoForm, Power, 3), [Buff(Buff::EchoForm, 1)]),
        define_card!(
            (Electrodynamics, Power, 2),
            [Buff(Buff::Electro, 1), Channel(Orb::Lightning, 2)]
        ),
        define_card!((EmptyBody, Skill, 1), [GainBlock(7), ExitStance()]),
        define_card!(
            (EmptyFist, Attack, 1),
            [DealDamage(9), ExitStance()],
            requires_target
        ),
        define_card!((EmptyMind, Skill, 1), [Draw(2), ExitStance()]),
        define_card!(
            (EndlessAgony, Attack, 0),
            [DealDamage(4)],
            [dup_on_draw, exhaust, requires_target]
        ),
        define_card!((Enlightenment, Skill, 0), [HandCustom()]),
        define_card!((Entrench, Skill, 1), [GainBlockCustom()]),
        define_card!((Envenom, Power, 1), [Buff(Buff::Envenom, 1)]),
        define_card!((Equilibrium, Skill, 2), [GainBlock(13), HandCustom()]),
        define_card!(
            (Eruption, Attack, 2),
            [DealDamage(9), EnterStance(Stance::Wrath)],
            requires_target
        ),
        define_card!((EscapePlan, Skill, 0), [DrawCustom()]),
        define_card!((Establishment, Power, 1), [Buff(Buff::Establishment, 1)]),
        define_card!(
            (Evaluate, Skill, 1),
            [GainBlock(6), ShuffleIntoDrawPile(&[Card::Insight])]
        ),
        define_card!(
            (Eviscerate, Attack, 3),
            [DealDamage(7), DealDamage(7), DealDamage(7)],
            [requires_target, special_cost]
        ),
        define_card!((Evolve, Power, 1), [Buff(Buff::Evolve, 1)]),
        define_card!((Exhume, Skill, 1), [Exhume()], exhaust),
        define_card!((Expertise, Skill, 1), [DrawCustom()]),
        define_card!((Expunger, Attack, 1), [DealDamageCustom()], requires_target),
        define_card!(
            (Fasting, Power, 2),
            [
                GainStrength(3),
                GainDexterity(3),
                DebuffSelf(Debuff::Fasting, 1)
            ]
        ),
        define_card!(
            (FearNoEvil, Attack, 1),
            [DealDamage(8), StanceCustom()],
            requires_target
        ),
        define_card!(
            (Feed, Attack, 1),
            [DealDamage(10), HealCustom()],
            [exhaust, requires_target]
        ),
        define_card!((FeelNoPain, Power, 1), [Buff(Buff::FeelNoPain, 1)]),
        define_card!(
            (FiendFire, Attack, 2),
            [DealDamageCustom()],
            [exhaust, requires_target]
        ),
        define_card!((Finesse, Skill, 0), [GainBlock(2), Draw(1)]),
        define_card!((Slimed, Status, 1), [], exhaust),
        define_card!((Strike, Attack, 1), [DealDamage(6)], requires_target),
    ]
});

static CARD_DETAILS: Lazy<HashMap<Card, &'static CardDetails>> =
    Lazy::new(|| ALL_CARDS.iter().map(|card| (card.card, card)).collect());

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_requires_target_properly_set() {
        for card in ALL_CARDS.iter() {
            let should_require_target = card.effect_chain.iter().any(|effect| {
                matches!(
                    effect,
                    PlayerEffect::DealDamage(_)
                        | PlayerEffect::Debuff(_, _)
                        | PlayerEffect::SapStrength(_)
                )
            });
            if card.effect_chain.iter().any(|effect| {
                matches!(
                    effect,
                    PlayerEffect::DealDamageCustom() | PlayerEffect::DebuffCustom()
                )
            }) {
                continue;
            }
            assert_eq!(
                (card.card, card.requires_target),
                (card.card, should_require_target)
            );
        }
    }
}
