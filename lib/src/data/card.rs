// Source: Slay the Spire Wiki (https://slay-the-spire.fandom.com/wiki/Category:Cards)

use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::data::{EnemyCondition, PlayerCondition};
use crate::types::Energy;

use super::effect::PlayerEffect;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Card {
    Accuracy(bool),
    Acrobatics(bool),
    Adrenaline(bool),
    AfterImage(bool),
    Aggregate(bool),
    Alchemize(bool),
    AllForOne(bool),
    AllOutAttack(bool),
    Alpha(bool),
    Amplify(bool),
    Anger(bool),
    Apotheosis(bool),
    Apparition(bool),
    Armaments(bool),
    AscendersBane(bool),
    AThousandCuts(bool),
    AutoShields(bool),
    Backflip(bool),
    Backstab(bool),
    BallLightning(bool),
    BandageUp(bool),
    Bane(bool),
    Barrage(bool),
    Barricade(bool),
    Bash(bool),
    BattleHymn(bool),
    BattleTrance(bool),
    BeamCell(bool),
    Berserk(bool),
    Beta(bool),
    BiasedCognition(bool),
    Bite(bool),
    BladeDance(bool),
    Blasphemy(bool),
    Blind(bool),
    Blizzard(bool),
    BloodForBlood(bool),
    Bloodletting(bool),
    Bludgeon(bool),
    Blur(bool),
    BodySlam(bool),
    BootSequence(bool),
    BouncingFlask(bool),
    BowlingBash(bool),
    Brilliance(bool),
    Brutality(bool),
    Buffer(bool),
    BulletTime(bool),
    Bullseye(bool),
    Burn(bool),
    BurningPact(bool),
    Burst(bool),
    CalculatedGamble(bool),
    Caltrops(bool),
    Capacitor(bool),
    Carnage(bool),
    CarveReality(bool),
    Catalyst(bool),
    Chaos(bool),
    ChargeBattery(bool),
    Chill(bool),
    Choke(bool),
    Chrysalis(bool),
    Clash(bool),
    Claw(bool),
    Cleave(bool),
    CloakAndDagger(bool),
    Clothesline(bool),
    Clumsy(bool),
    ColdSnap(bool),
    Collect(bool),
    Combust(bool),
    CompileDriver(bool),
    Concentrate(bool),
    Conclude(bool),
    ConjureBlade(bool),
    Consecrate(bool),
    Consume(bool),
    Coolheaded(bool),
    CoreSurge(bool),
    CorpseExplosion(bool),
    Corruption(bool),
    CreativeAi(bool),
    Crescendo(bool),
    CripplingCloud(bool),
    CrushJoints(bool),
    CurseOfTheBell(bool),
    CutThroughFate(bool),
    DaggerSpray(bool),
    DaggerThrow(bool),
    DarkEmbrace(bool),
    Darkness(bool),
    DarkShackles(bool),
    Dash(bool),
    Dazed(bool),
    DeadlyPoison(bool),
    Decay(bool),
    DeceiveRealiry(bool),
    DeepBreath(bool),
    Defend(bool),
    Deflect(bool),
    Defragment(bool),
    DemonForm(bool),
    DeusExMachina(bool),
    DevaForm(bool),
    Devotion(bool),
    DieDieDie(bool),
    Disarm(bool),
    Discovery(bool),
    Distraction(bool),
    DodgeAndRoll(bool),
    DoomAndGloom(bool),
    Doppelganger(bool),
    DoubleEnergy(bool),
    DoubleTap(bool),
    Doubt(bool),
    DramaticEntrance(bool),
    Dropkick(bool),
    Dualcast(bool),
    DualWield(bool),
    EchoForm(bool),
    Electrodynamics(bool),
    EmptyBody(bool),
    EmptyFist(bool),
    EmptyMind(bool),
    EndlessAgony(bool),
    Enlightenment(bool),
    Entrench(bool),
    Envenom(bool),
    Equilibrium(bool),
    Eruption(bool),
    EscapePlan(bool),
    Establishment(bool),
    Evaluate(bool),
    Eviscerate(bool),
    Evolve(bool),
    Exhume(bool),
    Expertise(bool),
    Expunger(bool),
    Fasting(bool),
    FearNoEvil(bool),
    Feed(bool),
    FeelNoPain(bool),
    FiendFire(bool),
    Finesse(bool),
    Finisher(bool),
    FireBreathing(bool),
    Fission(bool),
    FlameBarrier(bool),
    FlashOfSteel(bool),
    Flechettes(bool),
    Flex(bool),
    FlurryOfBlows(bool),
    FlyingKnee(bool),
    FlyingSleeves(bool),
    FollowUp(bool),
    Footwork(bool),
    ForceField(bool),
    ForeignInfluence(bool),
    Foresight(bool),
    Forethought(bool),
    Ftl(bool),
    Fusion(bool),
    GeneticAlgorithm(bool),
    GhostlyArmor(bool),
    Glacier(bool),
    GlassKnife(bool),
    GoForTheEyes(bool),
    GoodInstincts(bool),
    GrandFinale(bool),
    Halt(bool),
    HandOfGreed(bool),
    Havoc(bool),
    Headbutt(bool),
    Heatsinks(bool),
    HeavyBlade(bool),
    HeelHook(bool),
    HelloWorld(bool),
    Hemokinesis(bool),
    Hologram(bool),
    Hyperbeam(bool),
    Immolate(bool),
    Impatience(bool),
    Impervious(bool),
    Indignation(bool),
    InfernalBlade(bool),
    InfiniteBlades(bool),
    Inflame(bool),
    Injury(bool),
    InnerPeace(bool),
    Insight(bool),
    Intimidate(bool),
    IronWave(bool),
    JackOfAllTrades(bool),
    Jax(bool),
    Judgment(bool),
    Juggernaut(bool),
    JustLucky(bool),
    Leap(bool),
    LegSweep(bool),
    LessonLearned(bool),
    LikeWater(bool),
    LimitBreak(bool),
    Loop(bool),
    MachineLearning(bool),
    Madness(bool),
    Magnetism(bool),
    Malaise(bool),
    MasterfulStab(bool),
    MasterOfStrategy(bool),
    MasterReality(bool),
    Mayhem(bool),
    Meditate(bool),
    Melter(bool),
    MentalFortress(bool),
    Metallicize(bool),
    Metamorphosis(bool),
    MeteorStrike(bool),
    MindBlast(bool),
    Miracle(bool),
    MultiCast(bool),
    Necronomicurse(bool),
    Neutralize(bool),
    Nightmare(bool),
    Nirvana(bool),
    Normality(bool),
    NoxiousFumes(bool),
    Offering(bool),
    Omega(bool),
    Omniscience(bool),
    Outmaneuver(bool),
    Overclock(bool),
    Pain(bool),
    Panacea(bool),
    Panache(bool),
    PanicButton(bool),
    Parasite(bool),
    PerfectedStrike(bool),
    Perserverance(bool),
    PhantasmalKiller(bool),
    PiercingWail(bool),
    PoisonedStab(bool),
    PommelStrike(bool),
    PowerThrough(bool),
    Pray(bool),
    Predator(bool),
    Prepared(bool),
    PressurePoints(bool),
    Pride(bool),
    Prostrate(bool),
    Protect(bool),
    Pummel(bool),
    Purity(bool),
    QuickSlash(bool),
    Rage(bool),
    Ragnarok(bool),
    Rainbow(bool),
    Rampage(bool),
    ReachHeaven(bool),
    Reaper(bool),
    Reboot(bool),
    Rebound(bool),
    RecklessCharge(bool),
    Recursion(bool),
    Recycle(bool),
    Reflex(bool),
    Regret(bool),
    ReinforcedBody(bool),
    Reprogram(bool),
    RiddleWithHoles(bool),
    RipAndTear(bool),
    RitualDagger(bool),
    Rupture(bool),
    Rushdown(bool),
    SadisticNature(bool),
    Safety(bool),
    Sanctity(bool),
    SandsOfTime(bool),
    SashWhip(bool),
    Scrape(bool),
    Scrawl(bool),
    SearingBlow(u32),
    SecondWind(bool),
    SecretTechnique(bool),
    SecretWeapon(bool),
    SeeingRed(bool),
    Seek(bool),
    SelfRepair(bool),
    Sentinel(bool),
    Setup(bool),
    SeverSoul(bool),
    Shame(bool),
    SimmeringFury(bool),
    Shiv(bool),
    Shockwave(bool),
    ShrugItOff(bool),
    SignatureMove(bool),
    Skewer(bool),
    Skim(bool),
    Slice(bool),
    Slimed(bool),
    Smite(bool),
    SneakyStrike(bool),
    SpiritShield(bool),
    SpotWeakness(bool),
    Stack(bool),
    StaticDischarge(bool),
    SteamBarrier(bool),
    Storm(bool),
    StormOfSteel(bool),
    Streamline(bool),
    Strike(bool),
    Study(bool),
    SuckerPunch(bool),
    Sunder(bool),
    Survivor(bool),
    SweepingBeam(bool),
    SwiftStrike(bool),
    Swivel(bool),
    SwordBoomerang(bool),
    Tactician(bool),
    TalkToTheHand(bool),
    Tantrum(bool),
    Tempest(bool),
    Terror(bool),
    TheBomb(bool),
    ThinkingAhead(bool),
    ThirdEye(bool),
    ThroughViolence(bool),
    Thunderclap(bool),
    ThunderStrike(bool),
    ToolsOfTheTrade(bool),
    Tranquility(bool),
    Transmutation(bool),
    Trip(bool),
    TrueGrit(bool),
    Turbo(bool),
    TwinStrike(bool),
    Unload(bool),
    Uppercut(bool),
    Vault(bool),
    Vigilance(bool),
    Violence(bool),
    Void(bool),
    Wallop(bool),
    Warcry(bool),
    WaveOfTheHand(bool),
    Weave(bool),
    WellLaidPlans(bool),
    WheelKick(bool),
    Whirlwind(bool),
    WhiteNoise(bool),
    WildStrike(bool),
    WindmillStrike(bool),
    Wish(bool),
    Worship(bool),
    Wound(bool),
    WraithForm(bool),
    WreathOfFlame(bool),
    Writhe(bool),
    Zap(bool),
}

pub const UNCOMMON_COLORLESS_CARDS: &[Card] = &[
    Card::BandageUp(false),
    Card::Blind(false),
    Card::DarkShackles(false),
    Card::DeepBreath(false),
    Card::Discovery(false),
    Card::DramaticEntrance(false),
    Card::Enlightenment(false),
    Card::Finesse(false),
    Card::FlashOfSteel(false),
    Card::Forethought(false),
    Card::GoodInstincts(false),
    Card::Impatience(false),
    Card::JackOfAllTrades(false),
    Card::Madness(false),
    Card::MindBlast(false),
    Card::Panacea(false),
    Card::PanicButton(false),
    Card::Purity(false),
    Card::SwiftStrike(false),
    Card::Trip(false),
];

pub const CURSE_CARD_POOL: &[Card] = &[
    Card::Regret(false),
    Card::Injury(false),
    Card::Shame(false),
    Card::Parasite(false),
    Card::Normality(false),
    Card::Doubt(false),
    Card::Writhe(false),
    Card::Pain(false),
    Card::Decay(false),
    Card::Clumsy(false),
];

#[derive(Clone, Copy, Debug)]
#[cfg_attr(test, derive(Eq, Hash, PartialEq))]
pub enum CardType {
    Attack,
    Curse,
    Power,
    Skill,
    Status,
}

#[derive(Clone, Debug)]
#[cfg_attr(test, derive(Eq, Hash, PartialEq))]
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
    pub fn for_card(card: Card) -> &'static Self {
        CARD_DETAILS
            .get(&card)
            .unwrap_or_else(|| panic!("No details for card {:?}", card))
    }

    fn new(card: Card, type_: CardType, cost: Energy) -> Self {
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

    fn custom_requirements(mut self) -> Self {
        self.custom_requirements = true;
        self
    }

    fn dup_on_draw(mut self) -> Self {
        self.dup_on_draw = true;
        self
    }

    fn enact_on_draw(mut self) -> Self {
        self.enact_on_draw = true;
        self
    }

    fn enact_on_turn_end(mut self) -> Self {
        self.enact_on_turn_end = true;
        self
    }

    fn ethereal(mut self) -> Self {
        self.ethereal = true;
        self
    }

    fn exhaust(mut self) -> Self {
        self.exhaust = true;
        self
    }

    fn irremovable(mut self) -> Self {
        self.irremovable = true;
        self
    }

    fn innate(mut self) -> Self {
        self.innate = true;
        self
    }

    fn requires_target(mut self) -> Self {
        self.requires_target = true;
        self
    }

    fn retain(mut self) -> Self {
        self.retain = true;
        self
    }

    fn special_cost(mut self) -> Self {
        self.special_cost = true;
        self
    }

    fn unplayable(mut self) -> Self {
        self.unplayable = true;
        self
    }

    fn push(mut self, effect: PlayerEffect) -> Self {
        self.effect_chain.push(effect);
        self
    }
}

macro_rules! define_card {
    (($card:ident($u:expr), $type:ident, $cost:expr), [$($effect:ident($($param:expr),*)),*]) => {
        CardDetails::new(Card::$card($u), CardType::$type, $cost)
            $(.push(PlayerEffect::$effect($($param,)*)))*
    };
    (
        ($card:ident($u:expr), $type:ident, $cost:expr),
        [$($effect:ident($($param:expr),*)),*],
        $($extra:ident)*
    ) => {
        CardDetails::new(Card::$card($u), CardType::$type, $cost)
            $(.push(PlayerEffect::$effect($($param,)*)))* $(.$extra())*
    };
    (
        ($card:ident($u:expr), $type:ident, $cost:expr),
        [$($effect:ident($($param:expr),*)),*],
        [$($extra:ident),*]
    ) => {
        CardDetails::new(Card::$card($u), CardType::$type, $cost)
            $(.push(PlayerEffect::$effect($($param,)*)))* $(.$extra())*
    };
}

static ALL_CARDS: Lazy<Vec<CardDetails>> = Lazy::new(|| {
    vec![
        /*
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
        */
        define_card!(
            (Anger(false), Attack, 0),
            [DealDamage(6), CloneSelfIntoDiscardPile()],
            requires_target
        ),
        define_card!(
            (Anger(true), Attack, 0),
            [DealDamage(8), CloneSelfIntoDiscardPile()],
            requires_target
        ),
        /*
        define_card!((Apotheosis, Skill, 2), [UpgradeAllCardsInCombat()], exhaust),
        define_card!(
            (Apparition, Skill, 1),
            [Buff(Buff::Intangible, 1)],
            [ethereal, exhaust]
        ),
        */
        define_card!(
            (Armaments(false), Skill, 1),
            [GainBlock(5), UpgradeOneCardInHandThisCombat()]
        ),
        define_card!(
            (Armaments(true), Skill, 1),
            [GainBlock(5), UpgradeAllCardsInHandThisCombat()]
        ),
        /*
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
        */
        define_card!(
            (Bash(false), Attack, 2),
            [DealDamage(8), Apply(EnemyCondition::Vulnerable(2))],
            requires_target
        ),
        define_card!(
            (Bash(true), Attack, 2),
            [DealDamage(10), Apply(EnemyCondition::Vulnerable(3))],
            requires_target
        ),
        /*
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
        */
        define_card!(
            (BloodForBlood(false), Attack, 4),
            [DealDamage(18)],
            [requires_target, special_cost]
        ),
        define_card!(
            (BloodForBlood(true), Attack, 3),
            [DealDamage(22)],
            [requires_target, special_cost]
        ),
        /*
        define_card!(
            (Bloodletting, Skill, 0),
            [LoseHp(3), GainEnergy(2)],
            exhaust
        ),
        define_card!((Bludgeon, Attack, 3), [DealDamage(32)], requires_target),
        define_card!((Blur, Skill, 1), [GainBlock(5), Buff(Buff::Blur, 1)]),
        */
        define_card!(
            (BodySlam(false), Attack, 1),
            [DealDamageCustom()],
            requires_target
        ),
        define_card!(
            (BodySlam(true), Attack, 0),
            [DealDamageCustom()],
            requires_target
        ),
        /*
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
        */
        define_card!(
            (Carnage(false), Attack, 2),
            [DealDamage(20)],
            [ethereal, requires_target]
        ),
        define_card!(
            (Carnage(true), Attack, 2),
            [DealDamage(28)],
            [ethereal, requires_target]
        ),
        /*
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
        */
        define_card!(
            (Clothesline(false), Attack, 2),
            [DealDamage(12), Apply(EnemyCondition::Weak(2))],
            requires_target
        ),
        define_card!(
            (Clothesline(true), Attack, 2),
            [DealDamage(14), Apply(EnemyCondition::Weak(3))],
            requires_target
        ),
        /*
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
        */
        define_card!((Defend(false), Skill, 1), [GainBlock(5)]),
        define_card!((Defend(true), Skill, 1), [GainBlock(8)]),
        /*
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
        */
        define_card!(
            (Distraction(false), Skill, 1),
            [AddRandomCardThatCostsZeroThisTurnToHand(CardType::Skill)],
            exhaust
        ),
        define_card!(
            (Distraction(true), Skill, 0),
            [AddRandomCardThatCostsZeroThisTurnToHand(CardType::Skill)],
            exhaust
        ),
        /*
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
        */
        define_card!(
            (DualWield(false), Skill, 1),
            [CloneAttackOrPowerCardIntoHand(1)]
        ),
        define_card!(
            (DualWield(true), Skill, 1),
            [CloneAttackOrPowerCardIntoHand(2)]
        ),
        /*
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
        */
        define_card!(
            (Evolve(false), Power, 1),
            [ApplyToSelf(PlayerCondition::Evolve(1))]
        ),
        define_card!(
            (Evolve(true), Power, 1),
            [ApplyToSelf(PlayerCondition::Evolve(2))]
        ),
        /*
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
        */
        define_card!(
            (FireBreathing(false), Power, 1),
            [ApplyToSelf(PlayerCondition::FireBreathing(6))]
        ),
        define_card!(
            (FireBreathing(true), Power, 1),
            [ApplyToSelf(PlayerCondition::FireBreathing(10))]
        ),
        define_card!(
            (HeavyBlade(false), Attack, 2),
            [DealDamageWithStrengthMultiplier(14, 3)],
            requires_target
        ),
        define_card!(
            (HeavyBlade(true), Attack, 2),
            [DealDamageWithStrengthMultiplier(14, 5)],
            requires_target
        ),
        define_card!(
            (InfernalBlade(false), Skill, 1),
            [AddRandomCardThatCostsZeroThisTurnToHand(CardType::Attack)],
            exhaust
        ),
        define_card!(
            (InfernalBlade(true), Skill, 0),
            [AddRandomCardThatCostsZeroThisTurnToHand(CardType::Attack)],
            exhaust
        ),
        define_card!(
            (Intimidate(false), Skill, 0),
            [ApplyToAll(EnemyCondition::Weak(1))],
            exhaust
        ),
        define_card!(
            (Intimidate(true), Skill, 0),
            [ApplyToAll(EnemyCondition::Weak(2))],
            exhaust
        ),
        define_card!(
            (Rage(false), Skill, 0),
            [ApplyToSelf(PlayerCondition::Rage(3))]
        ),
        define_card!(
            (Rage(true), Skill, 0),
            [ApplyToSelf(PlayerCondition::Rage(5))]
        ),
        define_card!((Slimed(false), Status, 1), [], exhaust),
        define_card!((Slimed(true), Status, 1), [], exhaust),
        define_card!((Strike(false), Attack, 1), [DealDamage(6)], requires_target),
        define_card!((Strike(true), Attack, 1), [DealDamage(9)], requires_target),
        define_card!(
            (Thunderclap(false), Attack, 1),
            [
                DealDamageToAll(4),
                ApplyToAll(EnemyCondition::Vulnerable(1))
            ]
        ),
        define_card!(
            (Thunderclap(true), Attack, 1),
            [
                DealDamageToAll(7),
                ApplyToAll(EnemyCondition::Vulnerable(1))
            ]
        ),
        define_card!(
            (WhiteNoise(false), Skill, 1),
            [AddRandomCardThatCostsZeroThisTurnToHand(CardType::Power)],
            exhaust
        ),
        define_card!(
            (WhiteNoise(true), Skill, 0),
            [AddRandomCardThatCostsZeroThisTurnToHand(CardType::Power)],
            exhaust
        ),
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
                    PlayerEffect::Apply(_)
                        | PlayerEffect::DealDamage(_)
                        | PlayerEffect::DealDamageWithStrengthMultiplier(_, _)
                )
            });
            if card
                .effect_chain
                .iter()
                .any(|effect| matches!(effect, PlayerEffect::DealDamageCustom()))
            {
                continue;
            }
            assert_eq!(
                (card.card, card.requires_target),
                (card.card, should_require_target)
            );
        }
    }
}
