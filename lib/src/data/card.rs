// Source: Slay the Spire Wiki (https://slay-the-spire.fandom.com/wiki/Category:Cards)

use std::collections::HashMap;

use once_cell::sync::Lazy;

use super::condition::{EnemyCondition, PlayerCondition};
use super::damage::Damage;
use super::effect::{
    CardDestination, CardPool, CardSelection, CardSource, CostModifier, PlayerEffect,
    PlayerEffectCondition, Resource, TargetCondition, TargetEffect,
};

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
    AscendersBane,
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
    Clumsy,
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
    CurseOfTheBell,
    CutThroughFate(bool),
    DaggerSpray(bool),
    DaggerThrow(bool),
    DarkEmbrace(bool),
    Darkness(bool),
    DarkShackles(bool),
    Dash(bool),
    Dazed,
    DeadlyPoison(bool),
    Decay,
    DeceiveReality(bool),
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
    Doubt,
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
    Injury,
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
    Necronomicurse,
    Neutralize(bool),
    Nightmare(bool),
    Nirvana(bool),
    Normality,
    NoxiousFumes(bool),
    Offering(bool),
    Omega(bool),
    Omniscience(bool),
    Outmaneuver(bool),
    Overclock(bool),
    Pain,
    Panacea(bool),
    Panache(bool),
    PanicButton(bool),
    Parasite,
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
    Regret,
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
    Shame,
    SimmeringFury(bool),
    Shiv(bool),
    Shockwave(bool),
    ShrugItOff(bool),
    SignatureMove(bool),
    Skewer(bool),
    Skim(bool),
    Slice(bool),
    Slimed,
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
    Void,
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
    Wound,
    WraithForm(bool),
    WreathOfFlame(bool),
    Writhe,
    Zap(bool),
}

pub const UNCOMMON_COLORLESS_CARD_POOL: &[Card] = &[
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

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CardRarity {
    Starter,
    Common,
    Uncommon,
    Rare,
    Special,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CardType {
    Attack,
    Curse,
    Power,
    Skill,
    Status,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum EnergyCost {
    Zero,
    One,
    Two,
    Three,
    ThreeMinusHpLossCount,
    Four,
    FourMinusHpLossCount,
    Five,
    X,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct CardDetails {
    pub card: Card,
    pub type_: CardType,
    pub rarity: CardRarity,
    pub cost: EnergyCost,
    pub on_draw: Option<PlayerEffect>,
    pub on_exhaust: Option<PlayerEffect>,
    pub on_play: Vec<PlayerEffect>,
    pub if_in_hand_at_end_of_turn: Option<PlayerEffect>,

    // Card properties
    pub ethereal: bool,
    pub exhaust: bool,
    pub exhaust_when_played: bool,
    pub innate: bool,
    pub irremovable: bool,
    pub pain: bool,     // Lose 1 HP (unblockable) when other cards are played.
    pub parasite: bool, // If transformed or removed from your deck, lose 3 max HP.
    pub playable_only_if_all_cards_in_hand_are_attacks: bool,
    pub requires_target: bool,
    pub retain: bool,
    pub upgrade: Option<Card>,
    pub unplayable: bool,
}

#[allow(dead_code)] // Clippy doesn't seem to notice that all methods are used in the macro.
impl CardDetails {
    pub fn for_card(card: Card) -> &'static Self {
        CARD_DETAILS
            .get(&card)
            .unwrap_or_else(|| panic!("No details for card {:?}", card))
    }

    fn new(card: Card, type_: CardType, rarity: CardRarity, cost: EnergyCost) -> Self {
        CardDetails {
            card,
            type_,
            rarity,
            cost,
            on_draw: None,
            on_exhaust: None,
            on_play: Vec::new(),
            if_in_hand_at_end_of_turn: None,
            ethereal: false,
            exhaust: false,
            exhaust_when_played: false,
            innate: false,
            irremovable: false,
            pain: false,
            parasite: false,
            playable_only_if_all_cards_in_hand_are_attacks: false,
            requires_target: false,
            retain: false,
            upgrade: calculate_upgrade(card),
            unplayable: false,
        }
    }

    fn on_draw(mut self, effect: PlayerEffect) -> Self {
        self.on_draw = Some(effect);
        self
    }

    fn on_exhaust(mut self, effect: PlayerEffect) -> Self {
        self.on_exhaust = Some(effect);
        self
    }

    fn effect_chain_requires_target(effect_chain: &[PlayerEffect]) -> bool {
        effect_chain
            .iter()
            .any(|e| matches!(e, PlayerEffect::ToSingleTarget(_)))
    }

    fn push_on_play(mut self, effect: PlayerEffect) -> Self {
        if matches!(effect, PlayerEffect::ToSingleTarget(_)) {
            self.requires_target = true;
        }
        if let PlayerEffect::Conditional(_, effect_chain) = effect {
            if Self::effect_chain_requires_target(effect_chain) {
                self.requires_target = true;
            }
        }
        if let PlayerEffect::ForEachExhausted(effect_chain) = effect {
            if Self::effect_chain_requires_target(effect_chain) {
                self.requires_target = true;
            }
        }
        self.on_play.push(effect);
        self
    }

    fn if_in_hand_at_end_of_turn(mut self, effect: PlayerEffect) -> Self {
        self.if_in_hand_at_end_of_turn = Some(effect);
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

    fn exhaust_when_played(mut self) -> Self {
        self.exhaust_when_played = true;
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

    fn pain(mut self) -> Self {
        self.pain = true;
        self
    }

    fn parasite(mut self) -> Self {
        self.parasite = true;
        self
    }

    fn playable_only_if_all_hand_cards_are_attacks(mut self) -> Self {
        self.playable_only_if_all_cards_in_hand_are_attacks = true;
        self
    }

    fn retain(mut self) -> Self {
        self.retain = true;
        self
    }

    fn unplayable(mut self) -> Self {
        self.unplayable = true;
        self
    }
}

macro_rules! define_card {
    (
        // First tuple containing card info.
        ( $card_name:ident ( $($card_args:tt)* ), $card_type:ident, $rarity:ident, $energy_cost:ident ),
        // Second group: a list of on-play effects.
        [ $( $on_play:ident $( ( $($on_play_args:tt)* ) )? ),* $(,)? ],
        // Third group: a list of extra method calls.
        [ $( $method:ident $( ( $($method_args:tt)* ) )? ),* $(,)? ]
    ) => {
        CardDetails::new(
            Card::$card_name($($card_args)*),
            CardType::$card_type,
            CardRarity::$rarity,
            EnergyCost::$energy_cost,
        )
        $(
            .push_on_play(PlayerEffect::$on_play $( ( $($on_play_args)* ) )? )
        )*
        $(
            .$method( $( $($method_args)* )? )
        )*
    };

    (
        // First tuple containing card info (no upgrade bit)
        ( $card_name:ident, $card_type:ident, $rarity:ident, $energy_cost:ident ),
        // Second group: a list of on-play effects.
        [ $( $on_play:ident $( ( $($on_play_args:tt)* ) )? ),* $(,)? ],
        // Third group: a list of extra method calls.
        [ $( $method:ident $( ( $($method_args:tt)* ) )? ),* $(,)? ]
    ) => {
        CardDetails::new(
            Card::$card_name,
            CardType::$card_type,
            CardRarity::$rarity,
            EnergyCost::$energy_cost,
        )
        $(
            .push_on_play(PlayerEffect::$on_play $( ( $($on_play_args)* ) )? )
        )*
        $(
            .$method( $( $($method_args)* )? )
        )*
    };


    (
        // First “tuple” containing card info.
        ( $card_name:ident ( $($card_args:tt)* ), $card_type:ident, $rarity:ident, $energy_cost:ident ),
        // Second group: a list of on-play effects.
        [ $( $on_play:ident $( ( $($on_play_args:tt)* ) )? ),* $(,)? ],
        // Third group: one extra method call.
        $( $method:ident $( ( $($method_args:tt)* ) )? ),* $(,)?
    ) => {
        CardDetails::new(
            Card::$card_name($($card_args)*),
            CardType::$card_type,
            CardRarity::$rarity,
            EnergyCost::$energy_cost,

        )
        $(
            .push_on_play(PlayerEffect::$on_play $( ( $($on_play_args)* ) )? )
        )*
        $(
            .$method( $( $($method_args)* )? )
        )*
    };

    (
        // First “tuple” containing card info.
        ( $card_name:ident ( $($card_args:tt)* ), $card_type:ident, $rarity:ident, $energy_cost:ident ),
        // Second group: a list of on-play effects.
        [ $( $on_play:ident $( ( $($on_play_args:tt)* ) )? ),* $(,)? ]
    ) => {
        CardDetails::new(
            Card::$card_name($($card_args)*),
            CardType::$card_type,
            CardRarity::$rarity,
            EnergyCost::$energy_cost,

        )
        $(
            .push_on_play(PlayerEffect::$on_play $( ( $($on_play_args)* ) )? )
        )*
    };
}

// TODO: Fluent API
static ALL_CARDS: Lazy<Vec<CardDetails>> = Lazy::new(|| {
    vec![
        define_card!(
            (Anger(false), Attack, Common, Zero),
            [
                ToSingleTarget(TargetEffect::Deal(Damage::Blockable(6))),
                CreateCards(
                    CardPool::CardInPlay,
                    CardSelection::All,
                    CardDestination::DiscardPile,
                    CostModifier::None
                )
            ]
        ),
        define_card!(
            (Anger(true), Attack, Common, Zero),
            [
                ToSingleTarget(TargetEffect::Deal(Damage::Blockable(8))),
                CreateCards(
                    CardPool::CardInPlay,
                    CardSelection::All,
                    CardDestination::DiscardPile,
                    CostModifier::None
                )
            ]
        ),
        define_card!(
            (Apotheosis(false), Skill, Rare, Two),
            [Upgrade(CardSource::AllCardsInCombat, CardSelection::All)],
            exhaust
        ),
        define_card!(
            (Apotheosis(true), Skill, Rare, One),
            [Upgrade(CardSource::AllCardsInCombat, CardSelection::All)],
            exhaust
        ),
        define_card!(
            (Armaments(false), Skill, Common, One),
            [
                Gain(Resource::Block(5)),
                Upgrade(CardSource::Hand, CardSelection::PlayerChoice(1))
            ]
        ),
        define_card!(
            (Armaments(true), Skill, Common, One),
            [
                Gain(Resource::Block(5)),
                Upgrade(CardSource::Hand, CardSelection::All)
            ]
        ),
        define_card!(
            (BandageUp(false), Skill, Uncommon, Zero),
            [Gain(Resource::Hp(4))],
            exhaust
        ),
        define_card!(
            (BandageUp(true), Skill, Uncommon, Zero),
            [Gain(Resource::Hp(6))],
            exhaust
        ),
        define_card!(
            (Barricade(false), Power, Rare, Three),
            [Apply(PlayerCondition::Barricade)]
        ),
        define_card!(
            (Barricade(true), Power, Rare, Two),
            [Apply(PlayerCondition::Barricade)]
        ),
        define_card!(
            (Bash(false), Attack, Starter, Two),
            [
                ToSingleTarget(TargetEffect::Deal(Damage::Blockable(8))),
                ToSingleTarget(TargetEffect::Inflict(EnemyCondition::Vulnerable(2)))
            ]
        ),
        define_card!(
            (Bash(true), Attack, Starter, Two),
            [
                ToSingleTarget(TargetEffect::Deal(Damage::Blockable(10))),
                ToSingleTarget(TargetEffect::Inflict(EnemyCondition::Vulnerable(3)))
            ]
        ),
        define_card!(
            (BattleTrance(false), Skill, Uncommon, Zero),
            [Draw(3), Apply(PlayerCondition::NoDraw)]
        ),
        define_card!(
            (BattleTrance(true), Skill, Uncommon, Zero),
            [Draw(4), Apply(PlayerCondition::NoDraw)]
        ),
        define_card!(
            (Berserk(false), Power, Rare, Zero),
            [
                Apply(PlayerCondition::Vulnerable(2)),
                Apply(PlayerCondition::Berserk(1))
            ]
        ),
        define_card!(
            (Berserk(true), Power, Rare, Zero),
            [
                Apply(PlayerCondition::Vulnerable(1)),
                Apply(PlayerCondition::Berserk(1))
            ]
        ),
        define_card!(
            (Blind(false), Skill, Uncommon, Zero),
            [ToSingleTarget(TargetEffect::Inflict(EnemyCondition::Weak(
                1
            )))]
        ),
        define_card!(
            (Blind(true), Skill, Uncommon, Zero),
            [ToAllEnemies(TargetEffect::Inflict(EnemyCondition::Weak(1)))]
        ),
        define_card!(
            (BloodForBlood(false), Attack, Uncommon, FourMinusHpLossCount),
            [ToSingleTarget(TargetEffect::Deal(Damage::Blockable(18)))]
        ),
        define_card!(
            (BloodForBlood(true), Attack, Uncommon, ThreeMinusHpLossCount),
            [ToSingleTarget(TargetEffect::Deal(Damage::Blockable(22)))]
        ),
        define_card!(
            (Bloodletting(false), Skill, Uncommon, Zero),
            [TakeDamage(Damage::HpLoss(3)), Gain(Resource::Energy(2))],
            exhaust
        ),
        define_card!(
            (Bloodletting(true), Skill, Uncommon, Zero),
            [TakeDamage(Damage::HpLoss(3)), Gain(Resource::Energy(3))],
            exhaust
        ),
        define_card!(
            (Bludgeon(false), Attack, Rare, Three),
            [ToSingleTarget(TargetEffect::Deal(Damage::Blockable(32)))]
        ),
        define_card!(
            (Bludgeon(true), Attack, Rare, Three),
            [ToSingleTarget(TargetEffect::Deal(Damage::Blockable(42)))]
        ),
        define_card!(
            (BodySlam(false), Attack, Common, One),
            [ToSingleTarget(TargetEffect::Deal(
                Damage::BlockableEqualToPlayerBlock
            ))],
        ),
        define_card!(
            (BodySlam(true), Attack, Common, One),
            [ToSingleTarget(TargetEffect::Deal(
                Damage::BlockableEqualToPlayerBlock
            ))]
        ),
        define_card!(
            (Brutality(false), Power, Rare, Zero),
            [Apply(PlayerCondition::Brutality(1))]
        ),
        define_card!(
            (Brutality(true), Power, Rare, Zero),
            [Apply(PlayerCondition::Brutality(1))],
            innate
        ),
        define_card!(
            (Burn(false), Status, Special, Zero),
            [],
            [
                if_in_hand_at_end_of_turn(PlayerEffect::TakeDamage(Damage::Blockable(2))),
                unplayable
            ]
        ),
        define_card!(
            (Burn(true), Status, Special, Zero),
            [],
            [
                if_in_hand_at_end_of_turn(PlayerEffect::TakeDamage(Damage::Blockable(4))),
                unplayable
            ]
        ),
        define_card!(
            (BurningPact(false), Skill, Uncommon, One),
            [
                ManipulateCards(
                    CardSource::Hand,
                    CardSelection::PlayerChoice(1),
                    CardDestination::ExhaustPile,
                    CostModifier::None
                ),
                Draw(2)
            ]
        ),
        define_card!(
            (BurningPact(true), Skill, Uncommon, One),
            [
                ManipulateCards(
                    CardSource::Hand,
                    CardSelection::PlayerChoice(1),
                    CardDestination::ExhaustPile,
                    CostModifier::None
                ),
                Draw(3)
            ]
        ),
        define_card!(
            (Carnage(false), Attack, Uncommon, Two),
            [ToSingleTarget(TargetEffect::Deal(Damage::Blockable(20)))],
            ethereal
        ),
        define_card!(
            (Carnage(true), Attack, Uncommon, Two),
            [ToSingleTarget(TargetEffect::Deal(Damage::Blockable(28)))],
            ethereal
        ),
        define_card!(
            (Chrysalis(false), Skill, Rare, Two),
            [CreateCards(
                CardPool::CharacterSkillPool,
                CardSelection::Random(3),
                CardDestination::ShuffledIntoDrawPile,
                CostModifier::ZeroThisCombat
            )],
            exhaust
        ),
        define_card!(
            (Chrysalis(true), Skill, Rare, Two),
            [CreateCards(
                CardPool::CharacterSkillPool,
                CardSelection::Random(5),
                CardDestination::ShuffledIntoDrawPile,
                CostModifier::ZeroThisCombat
            )],
            exhaust
        ),
        define_card!(
            (Clash(false), Attack, Common, Zero),
            [ToSingleTarget(TargetEffect::Deal(Damage::Blockable(14)))],
            playable_only_if_all_hand_cards_are_attacks
        ),
        define_card!(
            (Clash(true), Attack, Common, Zero),
            [ToSingleTarget(TargetEffect::Deal(Damage::Blockable(18)))],
            playable_only_if_all_hand_cards_are_attacks
        ),
        define_card!(
            (Cleave(false), Attack, Common, One),
            [ToAllEnemies(TargetEffect::Deal(Damage::Blockable(8)))]
        ),
        define_card!(
            (Cleave(true), Attack, Common, One),
            [ToAllEnemies(TargetEffect::Deal(Damage::Blockable(11)))]
        ),
        define_card!(
            (Clothesline(false), Attack, Common, Two),
            [
                ToSingleTarget(TargetEffect::Deal(Damage::Blockable(12))),
                ToSingleTarget(TargetEffect::Inflict(EnemyCondition::Weak(2)))
            ]
        ),
        define_card!(
            (Clothesline(true), Attack, Common, Two),
            [
                ToSingleTarget(TargetEffect::Deal(Damage::Blockable(14))),
                ToSingleTarget(TargetEffect::Inflict(EnemyCondition::Weak(3)))
            ]
        ),
        define_card!((Clumsy, Curse, Special, Zero), [], [ethereal, unplayable]),
        define_card!(
            (Combust(false), Power, Uncommon, One),
            [Apply(PlayerCondition::Combust(1, 5))]
        ),
        define_card!(
            (Combust(true), Power, Uncommon, One),
            [Apply(PlayerCondition::Combust(1, 7))]
        ),
        define_card!(
            (Corruption(false), Power, Rare, Three),
            [Apply(PlayerCondition::Corruption)]
        ),
        define_card!(
            (Corruption(true), Power, Rare, Two),
            [Apply(PlayerCondition::Corruption)]
        ),
        define_card!(
            (CurseOfTheBell, Curse, Special, Zero),
            [],
            [irremovable, unplayable]
        ),
        define_card!(
            (DarkEmbrace(false), Power, Uncommon, Two),
            [Apply(PlayerCondition::DarkEmbrace(1))]
        ),
        define_card!(
            (DarkEmbrace(true), Power, Uncommon, One),
            [Apply(PlayerCondition::DarkEmbrace(1))]
        ),
        define_card!(
            (DarkShackles(false), Skill, Uncommon, Zero),
            [ToSingleTarget(TargetEffect::Inflict(
                EnemyCondition::StrengthLossThisTurn(9)
            ))],
            exhaust
        ),
        define_card!(
            (DarkShackles(true), Skill, Uncommon, Zero),
            [ToSingleTarget(TargetEffect::Inflict(
                EnemyCondition::StrengthLossThisTurn(15)
            ))],
            exhaust
        ),
        define_card!((Dazed, Status, Special, Zero), [], [ethereal, unplayable]),
        define_card!(
            (Decay, Curse, Special, Zero),
            [],
            [
                if_in_hand_at_end_of_turn(PlayerEffect::TakeDamage(Damage::Blockable(2))),
                unplayable
            ]
        ),
        define_card!(
            (DeepBreath(false), Skill, Uncommon, Zero),
            [
                ManipulateCards(
                    CardSource::DiscardPile,
                    CardSelection::All,
                    CardDestination::ShuffledIntoDrawPile,
                    CostModifier::None
                ),
                Draw(1)
            ]
        ),
        define_card!(
            (DeepBreath(true), Skill, Uncommon, Zero),
            [
                ManipulateCards(
                    CardSource::DiscardPile,
                    CardSelection::All,
                    CardDestination::ShuffledIntoDrawPile,
                    CostModifier::None
                ),
                Draw(2)
            ]
        ),
        define_card!(
            (Defend(false), Skill, Starter, One),
            [Gain(Resource::Block(5))]
        ),
        define_card!(
            (Defend(true), Skill, Starter, One),
            [Gain(Resource::Block(8))]
        ),
        define_card!(
            (DemonForm(false), Power, Rare, Three),
            [Apply(PlayerCondition::DemonForm(2))]
        ),
        define_card!(
            (DemonForm(true), Power, Rare, Three),
            [Apply(PlayerCondition::DemonForm(3))]
        ),
        define_card!(
            (Disarm(false), Skill, Uncommon, One),
            [ToSingleTarget(TargetEffect::SapStrength(2))],
            exhaust
        ),
        define_card!(
            (Disarm(true), Skill, Uncommon, One),
            [ToSingleTarget(TargetEffect::SapStrength(3))],
            exhaust
        ),
        define_card!(
            (Discovery(false), Skill, Uncommon, One),
            [CreateCards(
                CardPool::CharacterCardPool,
                CardSelection::RandomThenPlayerChoice(3, 1),
                CardDestination::Hand,
                CostModifier::ZeroThisTurn
            )],
            exhaust
        ),
        define_card!(
            (Discovery(true), Skill, Uncommon, One),
            [CreateCards(
                CardPool::CharacterCardPool,
                CardSelection::RandomThenPlayerChoice(3, 1),
                CardDestination::Hand,
                CostModifier::ZeroThisTurn
            )]
        ),
        define_card!(
            (DoubleTap(false), Skill, Rare, One),
            [Apply(PlayerCondition::DoubleTap(1))]
        ),
        define_card!(
            (DoubleTap(true), Skill, Rare, One),
            [Apply(PlayerCondition::DoubleTap(2))]
        ),
        define_card!(
            (Doubt, Curse, Special, Zero),
            [],
            [
                if_in_hand_at_end_of_turn(PlayerEffect::Apply(PlayerCondition::Weak(1))),
                unplayable
            ]
        ),
        define_card!(
            (DramaticEntrance(false), Skill, Uncommon, Zero),
            [ToAllEnemies(TargetEffect::Deal(Damage::Blockable(8)))],
            [exhaust, innate]
        ),
        define_card!(
            (DramaticEntrance(true), Skill, Uncommon, Zero),
            [ToAllEnemies(TargetEffect::Deal(Damage::Blockable(12)))],
            [exhaust, innate]
        ),
        define_card!(
            (Dropkick(false), Attack, Uncommon, One),
            [
                ToSingleTarget(TargetEffect::Deal(Damage::Blockable(5))),
                ToSingleTarget(TargetEffect::Conditional(
                    TargetCondition::IsVulnerable,
                    &[
                        PlayerEffect::Gain(Resource::Energy(1)),
                        PlayerEffect::Draw(1)
                    ]
                ))
            ]
        ),
        define_card!(
            (Dropkick(true), Attack, Uncommon, One),
            [
                ToSingleTarget(TargetEffect::Deal(Damage::Blockable(8))),
                ToSingleTarget(TargetEffect::Conditional(
                    TargetCondition::IsVulnerable,
                    &[
                        PlayerEffect::Gain(Resource::Energy(1)),
                        PlayerEffect::Draw(1)
                    ]
                ))
            ]
        ),
        define_card!(
            (DualWield(false), Skill, Uncommon, One),
            [CreateCards(
                CardPool::AttacksAndPowersInHand,
                CardSelection::PlayerChoice(1),
                CardDestination::Hand,
                CostModifier::None
            )]
        ),
        define_card!(
            (DualWield(true), Skill, Uncommon, One),
            [CreateCards(
                CardPool::AttacksAndPowersInHand,
                CardSelection::PlayerChoice(1),
                CardDestination::TwoCopiesInHand,
                CostModifier::None
            )]
        ),
        define_card!(
            (Enlightenment(false), Skill, Uncommon, Zero),
            [ManipulateCards(
                CardSource::Hand,
                CardSelection::All,
                CardDestination::Hand,
                CostModifier::ZeroThisTurn
            )]
        ),
        define_card!(
            (Enlightenment(true), Skill, Uncommon, Zero),
            [ManipulateCards(
                CardSource::Hand,
                CardSelection::All,
                CardDestination::Hand,
                CostModifier::ZeroThisCombat
            )]
        ),
        define_card!(
            (Entrench(false), Skill, Uncommon, Two),
            [Gain(Resource::CurrentBlockIsDoubled)]
        ),
        define_card!(
            (Entrench(true), Skill, Uncommon, One),
            [Gain(Resource::CurrentBlockIsDoubled)]
        ),
        define_card!(
            (Evolve(false), Power, Uncommon, One),
            [Apply(PlayerCondition::Evolve(1))]
        ),
        define_card!(
            (Evolve(true), Power, Uncommon, One),
            [Apply(PlayerCondition::Evolve(2))]
        ),
        define_card!(
            (Exhume(false), Skill, Rare, One),
            [ManipulateCards(
                CardSource::ExhaustPile,
                CardSelection::PlayerChoice(1),
                CardDestination::Hand,
                CostModifier::None
            )],
            exhaust
        ),
        define_card!(
            (Exhume(true), Skill, Rare, Zero),
            [ManipulateCards(
                CardSource::ExhaustPile,
                CardSelection::PlayerChoice(1),
                CardDestination::Hand,
                CostModifier::None
            )],
            exhaust
        ),
        define_card!(
            (Feed(false), Attack, Rare, One),
            [
                ToSingleTarget(TargetEffect::Deal(Damage::Blockable(10))),
                ToSingleTarget(TargetEffect::Conditional(
                    TargetCondition::AttackWasFatal,
                    &[PlayerEffect::Gain(Resource::HpMax(3))]
                ))
            ],
            exhaust
        ),
        define_card!(
            (Feed(true), Attack, Rare, One),
            [
                ToSingleTarget(TargetEffect::Deal(Damage::Blockable(12))),
                ToSingleTarget(TargetEffect::Conditional(
                    TargetCondition::AttackWasFatal,
                    &[PlayerEffect::Gain(Resource::HpMax(4))]
                ))
            ],
            exhaust
        ),
        define_card!(
            (FeelNoPain(false), Power, Uncommon, One),
            [Apply(PlayerCondition::FeelNoPain(3))]
        ),
        define_card!(
            (FeelNoPain(true), Power, Uncommon, One),
            [Apply(PlayerCondition::FeelNoPain(4))]
        ),
        define_card!(
            (FiendFire(false), Attack, Rare, Two),
            [
                ManipulateCards(
                    CardSource::Hand,
                    CardSelection::All,
                    CardDestination::ExhaustPile,
                    CostModifier::None
                ),
                ForEachExhausted(&[PlayerEffect::ToSingleTarget(TargetEffect::Deal(
                    Damage::Blockable(7)
                ))])
            ],
            exhaust
        ),
        define_card!(
            (FiendFire(true), Attack, Rare, Two),
            [
                ManipulateCards(
                    CardSource::Hand,
                    CardSelection::All,
                    CardDestination::ExhaustPile,
                    CostModifier::None
                ),
                ForEachExhausted(&[PlayerEffect::ToSingleTarget(TargetEffect::Deal(
                    Damage::Blockable(10)
                ))])
            ],
            exhaust
        ),
        define_card!(
            (Finesse(false), Skill, Uncommon, Zero),
            [Gain(Resource::Block(2)), Draw(1)]
        ),
        define_card!(
            (Finesse(true), Skill, Uncommon, Zero),
            [Gain(Resource::Block(4)), Draw(1)]
        ),
        define_card!(
            (FireBreathing(false), Power, Uncommon, One),
            [Apply(PlayerCondition::FireBreathing(6))]
        ),
        define_card!(
            (FireBreathing(true), Power, Uncommon, One),
            [Apply(PlayerCondition::FireBreathing(10))]
        ),
        define_card!(
            (FlameBarrier(false), Skill, Uncommon, Two),
            [
                Gain(Resource::Block(12)),
                Apply(PlayerCondition::FlameBarrier(4))
            ]
        ),
        define_card!(
            (FlameBarrier(true), Skill, Uncommon, One),
            [
                Gain(Resource::Block(16)),
                Apply(PlayerCondition::FlameBarrier(6))
            ]
        ),
        define_card!(
            (FlashOfSteel(false), Attack, Uncommon, Zero),
            [
                ToSingleTarget(TargetEffect::Deal(Damage::Blockable(3))),
                Draw(1)
            ]
        ),
        define_card!(
            (FlashOfSteel(true), Attack, Uncommon, Zero),
            [
                ToSingleTarget(TargetEffect::Deal(Damage::Blockable(6))),
                Draw(1)
            ]
        ),
        define_card!(
            (Flex(false), Skill, Common, Zero),
            [
                Gain(Resource::Strength(2)),
                Apply(PlayerCondition::StrengthDown(2))
            ],
        ),
        define_card!(
            (Flex(true), Skill, Common, Zero),
            [
                Gain(Resource::Strength(4)),
                Apply(PlayerCondition::StrengthDown(4))
            ],
        ),
        define_card!(
            (Forethought(false), Skill, Uncommon, Zero),
            [ManipulateCards(
                CardSource::Hand,
                CardSelection::PlayerChoice(1),
                CardDestination::BottomOfDrawPile,
                CostModifier::ZeroUntilPlayed
            )]
        ),
        define_card!(
            (Forethought(true), Skill, Uncommon, Zero),
            [ManipulateCards(
                CardSource::Hand,
                CardSelection::PlayerChoiceUnlimited,
                CardDestination::BottomOfDrawPile,
                CostModifier::ZeroUntilPlayed
            )]
        ),
        define_card!(
            (GhostlyArmor(false), Skill, Uncommon, One),
            [Gain(Resource::Block(10))],
            ethereal
        ),
        define_card!(
            (GhostlyArmor(true), Skill, Uncommon, One),
            [Gain(Resource::Block(13))],
            ethereal
        ),
        define_card!(
            (GoodInstincts(false), Skill, Uncommon, Zero),
            [Gain(Resource::Block(6))]
        ),
        define_card!(
            (GoodInstincts(true), Skill, Uncommon, Zero),
            [Gain(Resource::Block(9))]
        ),
        define_card!(
            (HandOfGreed(false), Attack, Rare, Two),
            [
                ToSingleTarget(TargetEffect::Deal(Damage::Blockable(20))),
                ToSingleTarget(TargetEffect::Conditional(
                    TargetCondition::AttackWasFatal,
                    &[PlayerEffect::Gain(Resource::Gold(20))]
                ))
            ]
        ),
        define_card!(
            (HandOfGreed(true), Attack, Rare, Two),
            [
                ToSingleTarget(TargetEffect::Deal(Damage::Blockable(25))),
                ToSingleTarget(TargetEffect::Conditional(
                    TargetCondition::AttackWasFatal,
                    &[PlayerEffect::Gain(Resource::Gold(25))]
                ))
            ]
        ),
        define_card!(
            (Havoc(false), Skill, Common, One),
            [PlayThenExhaustTopCardOfDrawPile]
        ),
        define_card!(
            (Havoc(true), Skill, Common, Zero),
            [PlayThenExhaustTopCardOfDrawPile]
        ),
        define_card!(
            (Headbutt(false), Attack, Common, One),
            [
                ToSingleTarget(TargetEffect::Deal(Damage::Blockable(9))),
                ManipulateCards(
                    CardSource::DiscardPile,
                    CardSelection::PlayerChoice(1),
                    CardDestination::TopOfDrawPile,
                    CostModifier::None
                )
            ]
        ),
        define_card!(
            (Headbutt(true), Attack, Common, One),
            [
                ToSingleTarget(TargetEffect::Deal(Damage::Blockable(12))),
                ManipulateCards(
                    CardSource::DiscardPile,
                    CardSelection::PlayerChoice(1),
                    CardDestination::TopOfDrawPile,
                    CostModifier::None
                )
            ]
        ),
        define_card!(
            (HeavyBlade(false), Attack, Common, Two),
            [ToSingleTarget(TargetEffect::Deal(
                Damage::BlockableWithStrengthMultiplier(14, 3)
            ))]
        ),
        define_card!(
            (HeavyBlade(true), Attack, Common, Two),
            [ToSingleTarget(TargetEffect::Deal(
                Damage::BlockableWithStrengthMultiplier(14, 5)
            ))]
        ),
        define_card!(
            (Hemokinesis(false), Attack, Uncommon, One),
            [
                TakeDamage(Damage::HpLoss(2)),
                ToSingleTarget(TargetEffect::Deal(Damage::Blockable(15)))
            ]
        ),
        define_card!(
            (Hemokinesis(true), Attack, Uncommon, One),
            [
                TakeDamage(Damage::HpLoss(2)),
                ToSingleTarget(TargetEffect::Deal(Damage::Blockable(20)))
            ]
        ),
        define_card!(
            (Immolate(false), Attack, Rare, Two),
            [
                ToAllEnemies(TargetEffect::Deal(Damage::Blockable(21))),
                CreateCards(
                    CardPool::Fixed(&[Card::Burn(false)]),
                    CardSelection::All,
                    CardDestination::DiscardPile,
                    CostModifier::None
                )
            ]
        ),
        define_card!(
            (Immolate(true), Attack, Rare, Two),
            [
                ToAllEnemies(TargetEffect::Deal(Damage::Blockable(28))),
                CreateCards(
                    CardPool::Fixed(&[Card::Burn(false)]),
                    CardSelection::All,
                    CardDestination::DiscardPile,
                    CostModifier::None
                )
            ]
        ),
        define_card!(
            (Impatience(false), Skill, Uncommon, Zero),
            [Conditional(
                PlayerEffectCondition::IfHandContainsNoAttackCards,
                &[PlayerEffect::Draw(2)]
            )]
        ),
        define_card!(
            (Impatience(true), Skill, Uncommon, Zero),
            [Conditional(
                PlayerEffectCondition::IfHandContainsNoAttackCards,
                &[PlayerEffect::Draw(3)]
            )]
        ),
        define_card!(
            (Impervious(false), Skill, Rare, Two),
            [Gain(Resource::Block(30))],
            exhaust
        ),
        define_card!(
            (Impervious(true), Skill, Rare, Two),
            [Gain(Resource::Block(40))],
            exhaust
        ),
        define_card!(
            (InfernalBlade(false), Skill, Uncommon, One),
            [CreateCards(
                CardPool::CharacterAttackPool,
                CardSelection::Random(1),
                CardDestination::Hand,
                CostModifier::ZeroThisTurn
            )],
            exhaust
        ),
        define_card!(
            (InfernalBlade(true), Skill, Uncommon, Zero),
            [CreateCards(
                CardPool::CharacterAttackPool,
                CardSelection::Random(1),
                CardDestination::Hand,
                CostModifier::ZeroThisTurn
            )],
            exhaust
        ),
        define_card!(
            (Inflame(false), Power, Uncommon, One),
            [Gain(Resource::Strength(2))]
        ),
        define_card!(
            (Inflame(true), Power, Uncommon, One),
            [Gain(Resource::Strength(3))]
        ),
        define_card!((Injury, Curse, Special, Zero), [], [unplayable]),
        define_card!(
            (Intimidate(false), Skill, Uncommon, Zero),
            [ToAllEnemies(TargetEffect::Inflict(EnemyCondition::Weak(1)))],
            exhaust
        ),
        define_card!(
            (Intimidate(true), Skill, Uncommon, Zero),
            [ToAllEnemies(TargetEffect::Inflict(EnemyCondition::Weak(2)))],
            exhaust
        ),
        define_card!(
            (IronWave(false), Attack, Common, One),
            [
                Gain(Resource::Block(5)),
                ToSingleTarget(TargetEffect::Deal(Damage::Blockable(5)))
            ]
        ),
        define_card!(
            (IronWave(true), Attack, Common, One),
            [
                Gain(Resource::Block(7)),
                ToSingleTarget(TargetEffect::Deal(Damage::Blockable(7)))
            ]
        ),
        define_card!(
            (JackOfAllTrades(false), Skill, Uncommon, Zero),
            [CreateCards(
                CardPool::ColorlessCardPool,
                CardSelection::Random(1),
                CardDestination::Hand,
                CostModifier::None
            )],
            exhaust
        ),
        define_card!(
            (JackOfAllTrades(true), Skill, Uncommon, Zero),
            [CreateCards(
                CardPool::ColorlessCardPool,
                CardSelection::Random(2),
                CardDestination::Hand,
                CostModifier::None
            )],
            exhaust
        ),
        define_card!(
            (Juggernaut(false), Power, Rare, Two),
            [Apply(PlayerCondition::Juggernaut(5))],
        ),
        define_card!(
            (Juggernaut(true), Power, Rare, Two),
            [Apply(PlayerCondition::Juggernaut(7))],
        ),
        define_card!(
            (LimitBreak(false), Skill, Rare, One),
            [Gain(Resource::CurrentStrengthIsDoubled)],
            exhaust
        ),
        define_card!(
            (LimitBreak(true), Skill, Rare, One),
            [Gain(Resource::CurrentStrengthIsDoubled)]
        ),
        define_card!(
            (Madness(false), Skill, Uncommon, One),
            [ManipulateCards(
                CardSource::Hand,
                CardSelection::Random(1),
                CardDestination::Hand,
                CostModifier::ZeroThisCombat
            )],
            exhaust
        ),
        define_card!(
            (Madness(true), Skill, Uncommon, Zero),
            [ManipulateCards(
                CardSource::Hand,
                CardSelection::Random(1),
                CardDestination::Hand,
                CostModifier::ZeroThisCombat
            )],
            exhaust
        ),
        define_card!(
            (Magnetism(false), Power, Rare, Two),
            [Apply(PlayerCondition::Magnetism(1))]
        ),
        define_card!(
            (Magnetism(true), Power, Rare, One),
            [Apply(PlayerCondition::Magnetism(1))]
        ),
        define_card!(
            (MasterOfStrategy(false), Skill, Rare, Zero),
            [Draw(3)],
            exhaust
        ),
        define_card!(
            (MasterOfStrategy(true), Skill, Rare, Zero),
            [Draw(4)],
            exhaust
        ),
        define_card!(
            (Mayhem(false), Power, Rare, Two),
            [Apply(PlayerCondition::Mayhem(1))]
        ),
        define_card!(
            (Mayhem(true), Power, Rare, One),
            [Apply(PlayerCondition::Mayhem(1))]
        ),
        define_card!(
            (Metallicize(false), Power, Uncommon, One),
            [Apply(PlayerCondition::Metallicize(3))]
        ),
        define_card!(
            (Metallicize(true), Power, Uncommon, One),
            [Apply(PlayerCondition::Metallicize(4))]
        ),
        define_card!(
            (Metamorphosis(false), Skill, Rare, Two),
            [CreateCards(
                CardPool::CharacterAttackPool,
                CardSelection::Random(3),
                CardDestination::ShuffledIntoDrawPile,
                CostModifier::ZeroThisCombat
            )],
            exhaust
        ),
        define_card!(
            (Metamorphosis(true), Skill, Rare, Two),
            [CreateCards(
                CardPool::CharacterAttackPool,
                CardSelection::Random(5),
                CardDestination::ShuffledIntoDrawPile,
                CostModifier::ZeroThisCombat
            )],
            exhaust
        ),
        define_card!(
            (MindBlast(false), Attack, Uncommon, Two),
            [ToSingleTarget(TargetEffect::Deal(
                Damage::BlockableEqualToDrawPileSize
            ))],
            innate
        ),
        define_card!(
            (MindBlast(true), Attack, Uncommon, One),
            [ToSingleTarget(TargetEffect::Deal(
                Damage::BlockableEqualToDrawPileSize
            ))],
            innate
        ),
        define_card!(
            (Necronomicurse, Curse, Special, Zero),
            [],
            [
                on_exhaust(PlayerEffect::CreateCards(
                    CardPool::Fixed(&[Card::Necronomicurse]),
                    CardSelection::All,
                    CardDestination::Hand,
                    CostModifier::None
                )),
                unplayable
            ]
        ),
        define_card!((Normality, Curse, Special, Zero), [], [unplayable]),
        define_card!(
            (Offering(false), Skill, Rare, Zero),
            [
                TakeDamage(Damage::HpLoss(6)),
                Gain(Resource::Energy(2)),
                Draw(3)
            ],
            exhaust
        ),
        define_card!(
            (Offering(true), Skill, Rare, Zero),
            [
                TakeDamage(Damage::HpLoss(6)),
                Gain(Resource::Energy(2)),
                Draw(5)
            ],
            exhaust
        ),
        define_card!((Pain, Curse, Special, Zero), [], [pain, unplayable]),
        define_card!(
            (Panacea(false), Skill, Uncommon, Zero),
            [Apply(PlayerCondition::Artifact(1))],
            exhaust
        ),
        define_card!(
            (Panacea(true), Skill, Uncommon, Zero),
            [Apply(PlayerCondition::Artifact(2))],
            exhaust
        ),
        define_card!(
            (Panache(false), Power, Rare, Zero),
            [Apply(PlayerCondition::Panache(5, 10))],
        ),
        define_card!(
            (Panache(true), Power, Rare, Zero),
            [Apply(PlayerCondition::Panache(5, 14))],
        ),
        define_card!(
            (PanicButton(false), Skill, Uncommon, Zero),
            [Gain(Resource::Block(30))],
            exhaust
        ),
        define_card!(
            (PanicButton(true), Skill, Uncommon, Zero),
            [Gain(Resource::Block(40))],
            exhaust
        ),
        define_card!((Parasite, Curse, Special, Zero), [], [parasite, unplayable]),
        define_card!(
            (PerfectedStrike(false), Attack, Common, Two),
            [ToSingleTarget(TargetEffect::Deal(
                Damage::BlockableCountingStrikeCards(6, 2)
            ))]
        ),
        define_card!(
            (PerfectedStrike(true), Attack, Common, Two),
            [ToSingleTarget(TargetEffect::Deal(
                Damage::BlockableCountingStrikeCards(6, 3)
            ))]
        ),
        define_card!(
            (PommelStrike(false), Attack, Common, One),
            [
                ToSingleTarget(TargetEffect::Deal(Damage::Blockable(9))),
                Draw(1)
            ]
        ),
        define_card!(
            (PommelStrike(true), Attack, Common, One),
            [
                ToSingleTarget(TargetEffect::Deal(Damage::Blockable(10))),
                Draw(2)
            ]
        ),
        define_card!(
            (PowerThrough(false), Skill, Uncommon, One),
            [
                CreateCards(
                    CardPool::Fixed(&[Card::Wound, Card::Wound]),
                    CardSelection::All,
                    CardDestination::Hand,
                    CostModifier::None
                ),
                Gain(Resource::Block(15))
            ]
        ),
        define_card!(
            (PowerThrough(true), Skill, Uncommon, One),
            [
                CreateCards(
                    CardPool::Fixed(&[Card::Wound, Card::Wound]),
                    CardSelection::All,
                    CardDestination::Hand,
                    CostModifier::None
                ),
                Gain(Resource::Block(20))
            ]
        ),
        define_card!(
            (Pummel(false), Attack, Uncommon, One),
            [
                ToSingleTarget(TargetEffect::Deal(Damage::Blockable(2))),
                ToSingleTarget(TargetEffect::Deal(Damage::Blockable(2))),
                ToSingleTarget(TargetEffect::Deal(Damage::Blockable(2))),
                ToSingleTarget(TargetEffect::Deal(Damage::Blockable(2))),
            ]
        ),
        define_card!(
            (Pummel(true), Attack, Uncommon, One),
            [
                ToSingleTarget(TargetEffect::Deal(Damage::Blockable(2))),
                ToSingleTarget(TargetEffect::Deal(Damage::Blockable(2))),
                ToSingleTarget(TargetEffect::Deal(Damage::Blockable(2))),
                ToSingleTarget(TargetEffect::Deal(Damage::Blockable(2))),
                ToSingleTarget(TargetEffect::Deal(Damage::Blockable(2))),
            ]
        ),
        define_card!(
            (Purity(false), Skill, Uncommon, Zero),
            [ManipulateCards(
                CardSource::Hand,
                CardSelection::PlayerChoiceUpTo(3),
                CardDestination::ExhaustPile,
                CostModifier::None
            )],
            exhaust
        ),
        define_card!(
            (Purity(true), Skill, Uncommon, Zero),
            [ManipulateCards(
                CardSource::Hand,
                CardSelection::PlayerChoiceUpTo(5),
                CardDestination::ExhaustPile,
                CostModifier::None
            )],
            exhaust
        ),
        define_card!(
            (Rage(false), Skill, Uncommon, Zero),
            [Apply(PlayerCondition::Rage(3))]
        ),
        define_card!(
            (Rage(true), Skill, Uncommon, Zero),
            [Apply(PlayerCondition::Rage(5))]
        ),
        define_card!(
            (Regret, Curse, Special, Zero),
            [],
            [
                if_in_hand_at_end_of_turn(PlayerEffect::TakeDamage(Damage::HpLossEqualToHandSize)),
                unplayable
            ]
        ),
        define_card!(
            (Rampage(false), Attack, Uncommon, One),
            [
                ToSingleTarget(TargetEffect::Deal(Damage::Blockable(8))),
                RampUpCardDamage(5)
            ]
        ),
        define_card!(
            (Rampage(true), Attack, Uncommon, One),
            [
                ToSingleTarget(TargetEffect::Deal(Damage::Blockable(8))),
                RampUpCardDamage(8)
            ]
        ),
        define_card!(
            (Reaper(false), Attack, Rare, Two),
            [
                ToAllEnemies(TargetEffect::Deal(Damage::Blockable(4))),
                Gain(Resource::HpEqualToUnblockedDamage)
            ],
            exhaust
        ),
        define_card!(
            (Reaper(true), Attack, Rare, Two),
            [
                ToAllEnemies(TargetEffect::Deal(Damage::Blockable(5))),
                Gain(Resource::HpEqualToUnblockedDamage)
            ],
            exhaust
        ),
        define_card!(
            (RecklessCharge(false), Attack, Uncommon, Zero),
            [
                ToSingleTarget(TargetEffect::Deal(Damage::Blockable(7))),
                CreateCards(
                    CardPool::Fixed(&[Card::Dazed]),
                    CardSelection::All,
                    CardDestination::ShuffledIntoDrawPile,
                    CostModifier::None
                )
            ]
        ),
        define_card!(
            (RecklessCharge(true), Attack, Uncommon, Zero),
            [
                ToSingleTarget(TargetEffect::Deal(Damage::Blockable(10))),
                CreateCards(
                    CardPool::Fixed(&[Card::Dazed]),
                    CardSelection::All,
                    CardDestination::ShuffledIntoDrawPile,
                    CostModifier::None
                )
            ]
        ),
        define_card!(
            (Rupture(false), Power, Uncommon, One),
            [Apply(PlayerCondition::Rupture(1))]
        ),
        define_card!(
            (Rupture(true), Power, Uncommon, One),
            [Apply(PlayerCondition::Rupture(2))]
        ),
        define_card!(
            (SadisticNature(false), Power, Rare, Zero),
            [Apply(PlayerCondition::Sadistic(5))]
        ),
        define_card!(
            (SadisticNature(true), Power, Rare, Zero),
            [Apply(PlayerCondition::Sadistic(7))]
        ),
        // SearingBlow defined in the lazy constructor of CARD_DETAILS
        define_card!(
            (SecondWind(false), Skill, Uncommon, One),
            [
                ManipulateCards(
                    CardSource::NonAttackCardsInHand,
                    CardSelection::All,
                    CardDestination::ExhaustPile,
                    CostModifier::None
                ),
                ForEachExhausted(&[PlayerEffect::Gain(Resource::Block(5))])
            ]
        ),
        define_card!(
            (SecondWind(true), Skill, Uncommon, One),
            [
                ManipulateCards(
                    CardSource::NonAttackCardsInHand,
                    CardSelection::All,
                    CardDestination::ExhaustPile,
                    CostModifier::None
                ),
                ForEachExhausted(&[PlayerEffect::Gain(Resource::Block(7))])
            ]
        ),
        define_card!(
            (SecretWeapon(false), Skill, Rare, Zero),
            [ManipulateCards(
                CardSource::AttacksInDrawPile,
                CardSelection::PlayerChoice(1),
                CardDestination::Hand,
                CostModifier::None
            )],
            exhaust
        ),
        define_card!(
            (SecretWeapon(true), Skill, Rare, Zero),
            [ManipulateCards(
                CardSource::AttacksInDrawPile,
                CardSelection::PlayerChoice(1),
                CardDestination::Hand,
                CostModifier::None
            )],
            exhaust
        ),
        define_card!(
            (SeeingRed(false), Skill, Uncommon, One),
            [Gain(Resource::Energy(2))],
            exhaust
        ),
        define_card!(
            (SeeingRed(true), Skill, Uncommon, Zero),
            [Gain(Resource::Energy(2))],
            exhaust
        ),
        define_card!(
            (Sentinel(false), Skill, Uncommon, One),
            [Gain(Resource::Block(5))],
            on_exhaust(PlayerEffect::Gain(Resource::Energy(2)))
        ),
        define_card!(
            (Sentinel(true), Skill, Uncommon, One),
            [Gain(Resource::Block(8))],
            on_exhaust(PlayerEffect::Gain(Resource::Energy(3)))
        ),
        define_card!(
            (SeverSoul(false), Attack, Uncommon, Two),
            [
                ManipulateCards(
                    CardSource::NonAttackCardsInHand,
                    CardSelection::All,
                    CardDestination::ExhaustPile,
                    CostModifier::None
                ),
                ToSingleTarget(TargetEffect::Deal(Damage::Blockable(16)))
            ]
        ),
        define_card!(
            (SeverSoul(true), Attack, Uncommon, Two),
            [
                ManipulateCards(
                    CardSource::NonAttackCardsInHand,
                    CardSelection::All,
                    CardDestination::ExhaustPile,
                    CostModifier::None
                ),
                ToSingleTarget(TargetEffect::Deal(Damage::Blockable(22)))
            ]
        ),
        define_card!(
            (Shame, Curse, Special, Zero),
            [],
            [
                if_in_hand_at_end_of_turn(PlayerEffect::Apply(PlayerCondition::Frail(1))),
                unplayable
            ]
        ),
        define_card!(
            (Shockwave(false), Skill, Uncommon, Two),
            [
                ToAllEnemies(TargetEffect::Inflict(EnemyCondition::Weak(3))),
                ToAllEnemies(TargetEffect::Inflict(EnemyCondition::Vulnerable(3)))
            ]
        ),
        define_card!(
            (Shockwave(true), Skill, Uncommon, Two),
            [
                ToAllEnemies(TargetEffect::Inflict(EnemyCondition::Weak(5))),
                ToAllEnemies(TargetEffect::Inflict(EnemyCondition::Vulnerable(5)))
            ]
        ),
        define_card!(
            (ShrugItOff(false), Skill, Common, One),
            [Gain(Resource::Block(8)), Draw(1)]
        ),
        define_card!(
            (ShrugItOff(true), Skill, Common, One),
            [Gain(Resource::Block(11)), Draw(1)]
        ),
        define_card!((Slimed, Status, Common, One), [], [exhaust_when_played]),
        define_card!(
            (SpotWeakness(false), Skill, Uncommon, One),
            [ToSingleTarget(TargetEffect::Conditional(
                TargetCondition::IntendsToAttack,
                &[PlayerEffect::Gain(Resource::Strength(3))]
            ))]
        ),
        define_card!(
            (SpotWeakness(true), Skill, Uncommon, One),
            [ToSingleTarget(TargetEffect::Conditional(
                TargetCondition::IntendsToAttack,
                &[PlayerEffect::Gain(Resource::Strength(4))]
            ))]
        ),
        define_card!(
            (Strike(false), Attack, Starter, One),
            [ToSingleTarget(TargetEffect::Deal(Damage::Blockable(6)))]
        ),
        define_card!(
            (Strike(true), Attack, Starter, One),
            [ToSingleTarget(TargetEffect::Deal(Damage::Blockable(9)))]
        ),
        define_card!(
            (SwiftStrike(false), Attack, Common, Zero),
            [ToSingleTarget(TargetEffect::Deal(Damage::Blockable(7)))]
        ),
        define_card!(
            (SwiftStrike(true), Attack, Common, Zero),
            [ToSingleTarget(TargetEffect::Deal(Damage::Blockable(10)))]
        ),
        define_card!(
            (SwordBoomerang(false), Attack, Common, One),
            [
                ToRandomEnemy(TargetEffect::Deal(Damage::Blockable(3))),
                ToRandomEnemy(TargetEffect::Deal(Damage::Blockable(3))),
                ToRandomEnemy(TargetEffect::Deal(Damage::Blockable(3))),
            ],
        ),
        define_card!(
            (SwordBoomerang(true), Attack, Common, One),
            [
                ToRandomEnemy(TargetEffect::Deal(Damage::Blockable(3))),
                ToRandomEnemy(TargetEffect::Deal(Damage::Blockable(3))),
                ToRandomEnemy(TargetEffect::Deal(Damage::Blockable(3))),
                ToRandomEnemy(TargetEffect::Deal(Damage::Blockable(3))),
            ],
        ),
        define_card!(
            (ThinkingAhead(false), Skill, Rare, Zero),
            [
                Draw(2),
                ManipulateCards(
                    CardSource::Hand,
                    CardSelection::PlayerChoice(1),
                    CardDestination::TopOfDrawPile,
                    CostModifier::None
                )
            ],
            exhaust
        ),
        define_card!(
            (ThinkingAhead(true), Skill, Rare, Zero),
            [
                Draw(2),
                ManipulateCards(
                    CardSource::Hand,
                    CardSelection::PlayerChoice(1),
                    CardDestination::TopOfDrawPile,
                    CostModifier::None
                )
            ]
        ),
        define_card!(
            (Thunderclap(false), Attack, Common, One),
            [
                ToAllEnemies(TargetEffect::Deal(Damage::Blockable(4))),
                ToAllEnemies(TargetEffect::Inflict(EnemyCondition::Vulnerable(1)))
            ]
        ),
        define_card!(
            (Thunderclap(true), Attack, Common, One),
            [
                ToAllEnemies(TargetEffect::Deal(Damage::Blockable(7))),
                ToAllEnemies(TargetEffect::Inflict(EnemyCondition::Vulnerable(1)))
            ]
        ),
        define_card!(
            (Transmutation(false), Skill, Rare, X),
            [CreateCards(
                CardPool::ColorlessCardPool,
                CardSelection::RandomX,
                CardDestination::Hand,
                CostModifier::ZeroThisTurn
            )],
            exhaust
        ),
        define_card!(
            (Transmutation(true), Skill, Rare, X),
            [CreateCards(
                CardPool::UpgradedColorlessCardPool,
                CardSelection::RandomX,
                CardDestination::Hand,
                CostModifier::ZeroThisTurn
            )],
            exhaust
        ),
        define_card!(
            (Trip(false), Skill, Uncommon, Zero),
            [ToSingleTarget(TargetEffect::Inflict(
                EnemyCondition::Vulnerable(2)
            ))]
        ),
        define_card!(
            (Trip(true), Skill, Uncommon, Zero),
            [ToAllEnemies(TargetEffect::Inflict(
                EnemyCondition::Vulnerable(2)
            ))]
        ),
        define_card!(
            (TrueGrit(false), Skill, Common, One),
            [
                Gain(Resource::Block(7)),
                ManipulateCards(
                    CardSource::Hand,
                    CardSelection::Random(1),
                    CardDestination::ExhaustPile,
                    CostModifier::None
                )
            ]
        ),
        define_card!(
            (TrueGrit(true), Skill, Common, One),
            [
                Gain(Resource::Block(9)),
                ManipulateCards(
                    CardSource::Hand,
                    CardSelection::PlayerChoice(1),
                    CardDestination::ExhaustPile,
                    CostModifier::None
                )
            ]
        ),
        define_card!(
            (TwinStrike(false), Attack, Common, One),
            [
                ToSingleTarget(TargetEffect::Deal(Damage::Blockable(5))),
                ToSingleTarget(TargetEffect::Deal(Damage::Blockable(5)))
            ]
        ),
        define_card!(
            (TwinStrike(true), Attack, Common, One),
            [
                ToSingleTarget(TargetEffect::Deal(Damage::Blockable(7))),
                ToSingleTarget(TargetEffect::Deal(Damage::Blockable(7)))
            ]
        ),
        define_card!(
            (Uppercut(false), Attack, Uncommon, Two),
            [
                ToSingleTarget(TargetEffect::Deal(Damage::Blockable(13))),
                ToSingleTarget(TargetEffect::Inflict(EnemyCondition::Weak(1))),
                ToSingleTarget(TargetEffect::Inflict(EnemyCondition::Vulnerable(1)))
            ]
        ),
        define_card!(
            (Uppercut(true), Attack, Uncommon, Two),
            [
                ToSingleTarget(TargetEffect::Deal(Damage::Blockable(13))),
                ToSingleTarget(TargetEffect::Inflict(EnemyCondition::Weak(2))),
                ToSingleTarget(TargetEffect::Inflict(EnemyCondition::Vulnerable(2)))
            ]
        ),
        define_card!(
            (Violence(false), Skill, Rare, Zero),
            [ManipulateCards(
                CardSource::AttacksInDrawPile,
                CardSelection::Random(3),
                CardDestination::Hand,
                CostModifier::None
            )],
            exhaust
        ),
        define_card!(
            (Violence(true), Skill, Rare, Zero),
            [ManipulateCards(
                CardSource::AttacksInDrawPile,
                CardSelection::Random(4),
                CardDestination::Hand,
                CostModifier::None
            )],
            exhaust
        ),
        define_card!(
            (Void, Status, Special, Zero),
            [],
            [
                ethereal,
                on_draw(PlayerEffect::Lose(Resource::Energy(1))),
                unplayable
            ]
        ),
        define_card!(
            (Warcry(false), Skill, Common, Zero),
            [
                Draw(1),
                ManipulateCards(
                    CardSource::Hand,
                    CardSelection::PlayerChoice(1),
                    CardDestination::TopOfDrawPile,
                    CostModifier::None
                )
            ],
            exhaust
        ),
        define_card!(
            (Warcry(true), Skill, Common, Zero),
            [
                Draw(2),
                ManipulateCards(
                    CardSource::Hand,
                    CardSelection::PlayerChoice(1),
                    CardDestination::TopOfDrawPile,
                    CostModifier::None
                )
            ],
            exhaust
        ),
        define_card!(
            (Whirlwind(false), Attack, Uncommon, X),
            [ToAllEnemies(TargetEffect::DealXTimes(Damage::Blockable(5)))]
        ),
        define_card!(
            (Whirlwind(true), Attack, Uncommon, X),
            [ToAllEnemies(TargetEffect::DealXTimes(Damage::Blockable(8)))]
        ),
        define_card!(
            (WildStrike(false), Attack, Common, One),
            [
                ToSingleTarget(TargetEffect::Deal(Damage::Blockable(12))),
                CreateCards(
                    CardPool::Fixed(&[Card::Wound]),
                    CardSelection::All,
                    CardDestination::DiscardPile,
                    CostModifier::None
                )
            ]
        ),
        define_card!(
            (WildStrike(true), Attack, Common, One),
            [
                ToSingleTarget(TargetEffect::Deal(Damage::Blockable(17))),
                CreateCards(
                    CardPool::Fixed(&[Card::Wound]),
                    CardSelection::All,
                    CardDestination::DiscardPile,
                    CostModifier::None
                )
            ]
        ),
        define_card!((Wound, Status, Special, Zero), [], [unplayable]),
        define_card!((Writhe, Curse, Special, Zero), [], [innate, unplayable]),
    ]
});

static CARD_DETAILS: Lazy<HashMap<Card, &'static CardDetails>> = Lazy::new(|| {
    let mut map: HashMap<Card, &'static CardDetails> =
        ALL_CARDS.iter().map(|card| (card.card, card)).collect();
    for i in 0..30 {
        let card_details = Box::leak(Box::new(
            CardDetails::new(
                Card::SearingBlow(i),
                CardType::Attack,
                CardRarity::Uncommon,
                EnergyCost::Two,
            )
            .push_on_play(PlayerEffect::ToSingleTarget(TargetEffect::Deal(
                Damage::Blockable(12 + i * (i + 7) / 2),
            ))),
        ));
        map.insert(Card::SearingBlow(i), card_details);
    }
    map
});

fn calculate_upgrade(card: Card) -> Option<Card> {
    match card {
        Card::Accuracy(false) => Some(Card::Accuracy(true)),
        Card::Acrobatics(false) => Some(Card::Acrobatics(true)),
        Card::Adrenaline(false) => Some(Card::Adrenaline(true)),
        Card::AfterImage(false) => Some(Card::AfterImage(true)),
        Card::Aggregate(false) => Some(Card::Aggregate(true)),
        Card::Alchemize(false) => Some(Card::Alchemize(true)),
        Card::AllForOne(false) => Some(Card::AllForOne(true)),
        Card::AllOutAttack(false) => Some(Card::AllOutAttack(true)),
        Card::Alpha(false) => Some(Card::Alpha(true)),
        Card::Amplify(false) => Some(Card::Amplify(true)),
        Card::Anger(false) => Some(Card::Anger(true)),
        Card::Apotheosis(false) => Some(Card::Apotheosis(true)),
        Card::Apparition(false) => Some(Card::Apparition(true)),
        Card::Armaments(false) => Some(Card::Armaments(true)),
        Card::AscendersBane => None,
        Card::AThousandCuts(false) => Some(Card::AThousandCuts(true)),
        Card::AutoShields(false) => Some(Card::AutoShields(true)),
        Card::Backflip(false) => Some(Card::Backflip(true)),
        Card::Backstab(false) => Some(Card::Backstab(true)),
        Card::BallLightning(false) => Some(Card::BallLightning(true)),
        Card::BandageUp(false) => Some(Card::BandageUp(true)),
        Card::Bane(false) => Some(Card::Bane(true)),
        Card::Barrage(false) => Some(Card::Barrage(true)),
        Card::Barricade(false) => Some(Card::Barricade(true)),
        Card::Bash(false) => Some(Card::Bash(true)),
        Card::BattleHymn(false) => Some(Card::BattleHymn(true)),
        Card::BattleTrance(false) => Some(Card::BattleTrance(true)),
        Card::BeamCell(false) => Some(Card::BeamCell(true)),
        Card::Berserk(false) => Some(Card::Berserk(true)),
        Card::Beta(false) => Some(Card::Beta(true)),
        Card::BiasedCognition(false) => Some(Card::BiasedCognition(true)),
        Card::Bite(false) => Some(Card::Bite(true)),
        Card::BladeDance(false) => Some(Card::BladeDance(true)),
        Card::Blasphemy(false) => Some(Card::Blasphemy(true)),
        Card::Blind(false) => Some(Card::Blind(true)),
        Card::Blizzard(false) => Some(Card::Blizzard(true)),
        Card::BloodForBlood(false) => Some(Card::BloodForBlood(true)),
        Card::Bloodletting(false) => Some(Card::Bloodletting(true)),
        Card::Bludgeon(false) => Some(Card::Bludgeon(true)),
        Card::Blur(false) => Some(Card::Blur(true)),
        Card::BodySlam(false) => Some(Card::BodySlam(true)),
        Card::BootSequence(false) => Some(Card::BootSequence(true)),
        Card::BouncingFlask(false) => Some(Card::BouncingFlask(true)),
        Card::BowlingBash(false) => Some(Card::BowlingBash(true)),
        Card::Brilliance(false) => Some(Card::Brilliance(true)),
        Card::Brutality(false) => Some(Card::Brutality(true)),
        Card::Buffer(false) => Some(Card::Buffer(true)),
        Card::BulletTime(false) => Some(Card::BulletTime(true)),
        Card::Bullseye(false) => Some(Card::Bullseye(true)),
        Card::Burn(false) => Some(Card::Burn(true)),
        Card::BurningPact(false) => Some(Card::BurningPact(true)),
        Card::Burst(false) => Some(Card::Burst(true)),
        Card::CalculatedGamble(false) => Some(Card::CalculatedGamble(true)),
        Card::Caltrops(false) => Some(Card::Caltrops(true)),
        Card::Capacitor(false) => Some(Card::Capacitor(true)),
        Card::Carnage(false) => Some(Card::Carnage(true)),
        Card::CarveReality(false) => Some(Card::CarveReality(true)),
        Card::Catalyst(false) => Some(Card::Catalyst(true)),
        Card::Chaos(false) => Some(Card::Chaos(true)),
        Card::ChargeBattery(false) => Some(Card::ChargeBattery(true)),
        Card::Chill(false) => Some(Card::Chill(true)),
        Card::Choke(false) => Some(Card::Choke(true)),
        Card::Chrysalis(false) => Some(Card::Chrysalis(true)),
        Card::Clash(false) => Some(Card::Clash(true)),
        Card::Claw(false) => Some(Card::Claw(true)),
        Card::Cleave(false) => Some(Card::Cleave(true)),
        Card::CloakAndDagger(false) => Some(Card::CloakAndDagger(true)),
        Card::Clothesline(false) => Some(Card::Clothesline(true)),
        Card::Clumsy => None,
        Card::ColdSnap(false) => Some(Card::ColdSnap(true)),
        Card::Collect(false) => Some(Card::Collect(true)),
        Card::Combust(false) => Some(Card::Combust(true)),
        Card::CompileDriver(false) => Some(Card::CompileDriver(true)),
        Card::Concentrate(false) => Some(Card::Concentrate(true)),
        Card::Conclude(false) => Some(Card::Conclude(true)),
        Card::ConjureBlade(false) => Some(Card::ConjureBlade(true)),
        Card::Consecrate(false) => Some(Card::Consecrate(true)),
        Card::Consume(false) => Some(Card::Consume(true)),
        Card::Coolheaded(false) => Some(Card::Coolheaded(true)),
        Card::CoreSurge(false) => Some(Card::CoreSurge(true)),
        Card::CorpseExplosion(false) => Some(Card::CorpseExplosion(true)),
        Card::Corruption(false) => Some(Card::Corruption(true)),
        Card::CreativeAi(false) => Some(Card::CreativeAi(true)),
        Card::Crescendo(false) => Some(Card::Crescendo(true)),
        Card::CripplingCloud(false) => Some(Card::CripplingCloud(true)),
        Card::CrushJoints(false) => Some(Card::CrushJoints(true)),
        Card::CurseOfTheBell => None,
        Card::CutThroughFate(false) => Some(Card::CutThroughFate(true)),
        Card::DaggerSpray(false) => Some(Card::DaggerSpray(true)),
        Card::DaggerThrow(false) => Some(Card::DaggerThrow(true)),
        Card::DarkEmbrace(false) => Some(Card::DarkEmbrace(true)),
        Card::Darkness(false) => Some(Card::Darkness(true)),
        Card::DarkShackles(false) => Some(Card::DarkShackles(true)),
        Card::Dash(false) => Some(Card::Dash(true)),
        Card::Dazed => None,
        Card::DeadlyPoison(false) => Some(Card::DeadlyPoison(true)),
        Card::Decay => None,
        Card::DeceiveReality(false) => Some(Card::DeceiveReality(true)),
        Card::DeepBreath(false) => Some(Card::DeepBreath(true)),
        Card::Defend(false) => Some(Card::Defend(true)),
        Card::Deflect(false) => Some(Card::Deflect(true)),
        Card::Defragment(false) => Some(Card::Defragment(true)),
        Card::DemonForm(false) => Some(Card::DemonForm(true)),
        Card::DeusExMachina(false) => Some(Card::DeusExMachina(true)),
        Card::DevaForm(false) => Some(Card::DevaForm(true)),
        Card::Devotion(false) => Some(Card::Devotion(true)),
        Card::DieDieDie(false) => Some(Card::DieDieDie(true)),
        Card::Disarm(false) => Some(Card::Disarm(true)),
        Card::Discovery(false) => Some(Card::Discovery(true)),
        Card::Distraction(false) => Some(Card::Distraction(true)),
        Card::DodgeAndRoll(false) => Some(Card::DodgeAndRoll(true)),
        Card::DoomAndGloom(false) => Some(Card::DoomAndGloom(true)),
        Card::Doppelganger(false) => Some(Card::Doppelganger(true)),
        Card::DoubleEnergy(false) => Some(Card::DoubleEnergy(true)),
        Card::DoubleTap(false) => Some(Card::DoubleTap(true)),
        Card::Doubt => None,
        Card::DramaticEntrance(false) => Some(Card::DramaticEntrance(true)),
        Card::Dropkick(false) => Some(Card::Dropkick(true)),
        Card::Dualcast(false) => Some(Card::Dualcast(true)),
        Card::DualWield(false) => Some(Card::DualWield(true)),
        Card::EchoForm(false) => Some(Card::EchoForm(true)),
        Card::Electrodynamics(false) => Some(Card::Electrodynamics(true)),
        Card::EmptyBody(false) => Some(Card::EmptyBody(true)),
        Card::EmptyFist(false) => Some(Card::EmptyFist(true)),
        Card::EmptyMind(false) => Some(Card::EmptyMind(true)),
        Card::EndlessAgony(false) => Some(Card::EndlessAgony(true)),
        Card::Enlightenment(false) => Some(Card::Enlightenment(true)),
        Card::Entrench(false) => Some(Card::Entrench(true)),
        Card::Envenom(false) => Some(Card::Envenom(true)),
        Card::Equilibrium(false) => Some(Card::Equilibrium(true)),
        Card::Eruption(false) => Some(Card::Eruption(true)),
        Card::EscapePlan(false) => Some(Card::EscapePlan(true)),
        Card::Establishment(false) => Some(Card::Establishment(true)),
        Card::Evaluate(false) => Some(Card::Evaluate(true)),
        Card::Eviscerate(false) => Some(Card::Eviscerate(true)),
        Card::Evolve(false) => Some(Card::Evolve(true)),
        Card::Exhume(false) => Some(Card::Exhume(true)),
        Card::Expertise(false) => Some(Card::Expertise(true)),
        Card::Expunger(false) => Some(Card::Expunger(true)),
        Card::Fasting(false) => Some(Card::Fasting(true)),
        Card::FearNoEvil(false) => Some(Card::FearNoEvil(true)),
        Card::Feed(false) => Some(Card::Feed(true)),
        Card::FeelNoPain(false) => Some(Card::FeelNoPain(true)),
        Card::FiendFire(false) => Some(Card::FiendFire(true)),
        Card::Finesse(false) => Some(Card::Finesse(true)),
        Card::Finisher(false) => Some(Card::Finisher(true)),
        Card::FireBreathing(false) => Some(Card::FireBreathing(true)),
        Card::Fission(false) => Some(Card::Fission(true)),
        Card::FlameBarrier(false) => Some(Card::FlameBarrier(true)),
        Card::FlashOfSteel(false) => Some(Card::FlashOfSteel(true)),
        Card::Flechettes(false) => Some(Card::Flechettes(true)),
        Card::Flex(false) => Some(Card::Flex(true)),
        Card::FlurryOfBlows(false) => Some(Card::FlurryOfBlows(true)),
        Card::FlyingKnee(false) => Some(Card::FlyingKnee(true)),
        Card::FlyingSleeves(false) => Some(Card::FlyingSleeves(true)),
        Card::FollowUp(false) => Some(Card::FollowUp(true)),
        Card::Footwork(false) => Some(Card::Footwork(true)),
        Card::ForceField(false) => Some(Card::ForceField(true)),
        Card::ForeignInfluence(false) => Some(Card::ForeignInfluence(true)),
        Card::Foresight(false) => Some(Card::Foresight(true)),
        Card::Forethought(false) => Some(Card::Forethought(true)),
        Card::Ftl(false) => Some(Card::Ftl(true)),
        Card::Fusion(false) => Some(Card::Fusion(true)),
        Card::GeneticAlgorithm(false) => Some(Card::GeneticAlgorithm(true)),
        Card::GhostlyArmor(false) => Some(Card::GhostlyArmor(true)),
        Card::Glacier(false) => Some(Card::Glacier(true)),
        Card::GlassKnife(false) => Some(Card::GlassKnife(true)),
        Card::GoForTheEyes(false) => Some(Card::GoForTheEyes(true)),
        Card::GoodInstincts(false) => Some(Card::GoodInstincts(true)),
        Card::GrandFinale(false) => Some(Card::GrandFinale(true)),
        Card::Halt(false) => Some(Card::Halt(true)),
        Card::HandOfGreed(false) => Some(Card::HandOfGreed(true)),
        Card::Havoc(false) => Some(Card::Havoc(true)),
        Card::Headbutt(false) => Some(Card::Headbutt(true)),
        Card::Heatsinks(false) => Some(Card::Heatsinks(true)),
        Card::HeavyBlade(false) => Some(Card::HeavyBlade(true)),
        Card::HeelHook(false) => Some(Card::HeelHook(true)),
        Card::HelloWorld(false) => Some(Card::HelloWorld(true)),
        Card::Hemokinesis(false) => Some(Card::Hemokinesis(true)),
        Card::Hologram(false) => Some(Card::Hologram(true)),
        Card::Hyperbeam(false) => Some(Card::Hyperbeam(true)),
        Card::Immolate(false) => Some(Card::Immolate(true)),
        Card::Impatience(false) => Some(Card::Impatience(true)),
        Card::Impervious(false) => Some(Card::Impervious(true)),
        Card::Indignation(false) => Some(Card::Indignation(true)),
        Card::InfernalBlade(false) => Some(Card::InfernalBlade(true)),
        Card::InfiniteBlades(false) => Some(Card::InfiniteBlades(true)),
        Card::Inflame(false) => Some(Card::Inflame(true)),
        Card::Injury => None,
        Card::InnerPeace(false) => Some(Card::InnerPeace(true)),
        Card::Insight(false) => Some(Card::Insight(true)),
        Card::Intimidate(false) => Some(Card::Intimidate(true)),
        Card::IronWave(false) => Some(Card::IronWave(true)),
        Card::JackOfAllTrades(false) => Some(Card::JackOfAllTrades(true)),
        Card::Jax(false) => Some(Card::Jax(true)),
        Card::Judgment(false) => Some(Card::Judgment(true)),
        Card::Juggernaut(false) => Some(Card::Juggernaut(true)),
        Card::JustLucky(false) => Some(Card::JustLucky(true)),
        Card::Leap(false) => Some(Card::Leap(true)),
        Card::LegSweep(false) => Some(Card::LegSweep(true)),
        Card::LessonLearned(false) => Some(Card::LessonLearned(true)),
        Card::LikeWater(false) => Some(Card::LikeWater(true)),
        Card::LimitBreak(false) => Some(Card::LimitBreak(true)),
        Card::Loop(false) => Some(Card::Loop(true)),
        Card::MachineLearning(false) => Some(Card::MachineLearning(true)),
        Card::Madness(false) => Some(Card::Madness(true)),
        Card::Magnetism(false) => Some(Card::Magnetism(true)),
        Card::Malaise(false) => Some(Card::Malaise(true)),
        Card::MasterfulStab(false) => Some(Card::MasterfulStab(true)),
        Card::MasterOfStrategy(false) => Some(Card::MasterOfStrategy(true)),
        Card::MasterReality(false) => Some(Card::MasterReality(true)),
        Card::Mayhem(false) => Some(Card::Mayhem(true)),
        Card::Meditate(false) => Some(Card::Meditate(true)),
        Card::Melter(false) => Some(Card::Melter(true)),
        Card::MentalFortress(false) => Some(Card::MentalFortress(true)),
        Card::Metallicize(false) => Some(Card::Metallicize(true)),
        Card::Metamorphosis(false) => Some(Card::Metamorphosis(true)),
        Card::MeteorStrike(false) => Some(Card::MeteorStrike(true)),
        Card::MindBlast(false) => Some(Card::MindBlast(true)),
        Card::Miracle(false) => Some(Card::Miracle(true)),
        Card::MultiCast(false) => Some(Card::MultiCast(true)),
        Card::Necronomicurse => None,
        Card::Neutralize(false) => Some(Card::Neutralize(true)),
        Card::Nightmare(false) => Some(Card::Nightmare(true)),
        Card::Nirvana(false) => Some(Card::Nirvana(true)),
        Card::Normality => None,
        Card::NoxiousFumes(false) => Some(Card::NoxiousFumes(true)),
        Card::Offering(false) => Some(Card::Offering(true)),
        Card::Omega(false) => Some(Card::Omega(true)),
        Card::Omniscience(false) => Some(Card::Omniscience(true)),
        Card::Outmaneuver(false) => Some(Card::Outmaneuver(true)),
        Card::Overclock(false) => Some(Card::Overclock(true)),
        Card::Pain => None,
        Card::Panacea(false) => Some(Card::Panacea(true)),
        Card::Panache(false) => Some(Card::Panache(true)),
        Card::PanicButton(false) => Some(Card::PanicButton(true)),
        Card::Parasite => None,
        Card::PerfectedStrike(false) => Some(Card::PerfectedStrike(true)),
        Card::Perserverance(false) => Some(Card::Perserverance(true)),
        Card::PhantasmalKiller(false) => Some(Card::PhantasmalKiller(true)),
        Card::PiercingWail(false) => Some(Card::PiercingWail(true)),
        Card::PoisonedStab(false) => Some(Card::PoisonedStab(true)),
        Card::PommelStrike(false) => Some(Card::PommelStrike(true)),
        Card::PowerThrough(false) => Some(Card::PowerThrough(true)),
        Card::Pray(false) => Some(Card::Pray(true)),
        Card::Predator(false) => Some(Card::Predator(true)),
        Card::Prepared(false) => Some(Card::Prepared(true)),
        Card::PressurePoints(false) => Some(Card::PressurePoints(true)),
        Card::Pride(false) => Some(Card::Pride(true)),
        Card::Prostrate(false) => Some(Card::Prostrate(true)),
        Card::Protect(false) => Some(Card::Protect(true)),
        Card::Pummel(false) => Some(Card::Pummel(true)),
        Card::Purity(false) => Some(Card::Purity(true)),
        Card::QuickSlash(false) => Some(Card::QuickSlash(true)),
        Card::Rage(false) => Some(Card::Rage(true)),
        Card::Ragnarok(false) => Some(Card::Ragnarok(true)),
        Card::Rainbow(false) => Some(Card::Rainbow(true)),
        Card::Rampage(false) => Some(Card::Rampage(true)),
        Card::ReachHeaven(false) => Some(Card::ReachHeaven(true)),
        Card::Reaper(false) => Some(Card::Reaper(true)),
        Card::Reboot(false) => Some(Card::Reboot(true)),
        Card::Rebound(false) => Some(Card::Rebound(true)),
        Card::RecklessCharge(false) => Some(Card::RecklessCharge(true)),
        Card::Recursion(false) => Some(Card::Recursion(true)),
        Card::Recycle(false) => Some(Card::Recycle(true)),
        Card::Reflex(false) => Some(Card::Reflex(true)),
        Card::Regret => None,
        Card::ReinforcedBody(false) => Some(Card::ReinforcedBody(true)),
        Card::Reprogram(false) => Some(Card::Reprogram(true)),
        Card::RiddleWithHoles(false) => Some(Card::RiddleWithHoles(true)),
        Card::RipAndTear(false) => Some(Card::RipAndTear(true)),
        Card::RitualDagger(false) => Some(Card::RitualDagger(true)),
        Card::Rupture(false) => Some(Card::Rupture(true)),
        Card::Rushdown(false) => Some(Card::Rushdown(true)),
        Card::SadisticNature(false) => Some(Card::SadisticNature(true)),
        Card::Safety(false) => Some(Card::Safety(true)),
        Card::Sanctity(false) => Some(Card::Sanctity(true)),
        Card::SandsOfTime(false) => Some(Card::SandsOfTime(true)),
        Card::SashWhip(false) => Some(Card::SashWhip(true)),
        Card::Scrape(false) => Some(Card::Scrape(true)),
        Card::Scrawl(false) => Some(Card::Scrawl(true)),
        Card::SearingBlow(i) => Some(Card::SearingBlow(i + 1)),
        Card::SecondWind(false) => Some(Card::SecondWind(true)),
        Card::SecretTechnique(false) => Some(Card::SecretTechnique(true)),
        Card::SecretWeapon(false) => Some(Card::SecretWeapon(true)),
        Card::SeeingRed(false) => Some(Card::SeeingRed(true)),
        Card::Seek(false) => Some(Card::Seek(true)),
        Card::SelfRepair(false) => Some(Card::SelfRepair(true)),
        Card::Sentinel(false) => Some(Card::Sentinel(true)),
        Card::Setup(false) => Some(Card::Setup(true)),
        Card::SeverSoul(false) => Some(Card::SeverSoul(true)),
        Card::Shame => None,
        Card::SimmeringFury(false) => Some(Card::SimmeringFury(true)),
        Card::Shiv(false) => Some(Card::Shiv(true)),
        Card::Shockwave(false) => Some(Card::Shockwave(true)),
        Card::ShrugItOff(false) => Some(Card::ShrugItOff(true)),
        Card::SignatureMove(false) => Some(Card::SignatureMove(true)),
        Card::Skewer(false) => Some(Card::Skewer(true)),
        Card::Skim(false) => Some(Card::Skim(true)),
        Card::Slice(false) => Some(Card::Slice(true)),
        Card::Slimed => None,
        Card::Smite(false) => Some(Card::Smite(true)),
        Card::SneakyStrike(false) => Some(Card::SneakyStrike(true)),
        Card::SpiritShield(false) => Some(Card::SpiritShield(true)),
        Card::SpotWeakness(false) => Some(Card::SpotWeakness(true)),
        Card::Stack(false) => Some(Card::Stack(true)),
        Card::StaticDischarge(false) => Some(Card::StaticDischarge(true)),
        Card::SteamBarrier(false) => Some(Card::SteamBarrier(true)),
        Card::Storm(false) => Some(Card::Storm(true)),
        Card::StormOfSteel(false) => Some(Card::StormOfSteel(true)),
        Card::Streamline(false) => Some(Card::Streamline(true)),
        Card::Strike(false) => Some(Card::Strike(true)),
        Card::Study(false) => Some(Card::Study(true)),
        Card::SuckerPunch(false) => Some(Card::SuckerPunch(true)),
        Card::Sunder(false) => Some(Card::Sunder(true)),
        Card::Survivor(false) => Some(Card::Survivor(true)),
        Card::SweepingBeam(false) => Some(Card::SweepingBeam(true)),
        Card::SwiftStrike(false) => Some(Card::SwiftStrike(true)),
        Card::Swivel(false) => Some(Card::Swivel(true)),
        Card::SwordBoomerang(false) => Some(Card::SwordBoomerang(true)),
        Card::Tactician(false) => Some(Card::Tactician(true)),
        Card::TalkToTheHand(false) => Some(Card::TalkToTheHand(true)),
        Card::Tantrum(false) => Some(Card::Tantrum(true)),
        Card::Tempest(false) => Some(Card::Tempest(true)),
        Card::Terror(false) => Some(Card::Terror(true)),
        Card::TheBomb(false) => Some(Card::TheBomb(true)),
        Card::ThinkingAhead(false) => Some(Card::ThinkingAhead(true)),
        Card::ThirdEye(false) => Some(Card::ThirdEye(true)),
        Card::ThroughViolence(false) => Some(Card::ThroughViolence(true)),
        Card::Thunderclap(false) => Some(Card::Thunderclap(true)),
        Card::ThunderStrike(false) => Some(Card::ThunderStrike(true)),
        Card::ToolsOfTheTrade(false) => Some(Card::ToolsOfTheTrade(true)),
        Card::Tranquility(false) => Some(Card::Tranquility(true)),
        Card::Transmutation(false) => Some(Card::Transmutation(true)),
        Card::Trip(false) => Some(Card::Trip(true)),
        Card::TrueGrit(false) => Some(Card::TrueGrit(true)),
        Card::Turbo(false) => Some(Card::Turbo(true)),
        Card::TwinStrike(false) => Some(Card::TwinStrike(true)),
        Card::Unload(false) => Some(Card::Unload(true)),
        Card::Uppercut(false) => Some(Card::Uppercut(true)),
        Card::Vault(false) => Some(Card::Vault(true)),
        Card::Vigilance(false) => Some(Card::Vigilance(true)),
        Card::Violence(false) => Some(Card::Violence(true)),
        Card::Void => None,
        Card::Wallop(false) => Some(Card::Wallop(true)),
        Card::Warcry(false) => Some(Card::Warcry(true)),
        Card::WaveOfTheHand(false) => Some(Card::WaveOfTheHand(true)),
        Card::Weave(false) => Some(Card::Weave(true)),
        Card::WellLaidPlans(false) => Some(Card::WellLaidPlans(true)),
        Card::WheelKick(false) => Some(Card::WheelKick(true)),
        Card::Whirlwind(false) => Some(Card::Whirlwind(true)),
        Card::WhiteNoise(false) => Some(Card::WhiteNoise(true)),
        Card::WildStrike(false) => Some(Card::WildStrike(true)),
        Card::WindmillStrike(false) => Some(Card::WindmillStrike(true)),
        Card::Wish(false) => Some(Card::Wish(true)),
        Card::Worship(false) => Some(Card::Worship(true)),
        Card::Wound => None,
        Card::WraithForm(false) => Some(Card::WraithForm(true)),
        Card::WreathOfFlame(false) => Some(Card::WreathOfFlame(true)),
        Card::Writhe => None,
        Card::Zap(false) => Some(Card::Zap(true)),
        _ => None,
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use super::*;

    fn has_x_effect(card: &CardDetails) -> bool {
        card.on_play.iter().any(|effect| {
            matches!(
                effect,
                PlayerEffect::ToAllEnemies(TargetEffect::DealXTimes(_))
                    | PlayerEffect::CreateCards(_, CardSelection::RandomX, _, _)
                    | PlayerEffect::ManipulateCards(_, CardSelection::RandomX, _, _)
            )
        })
    }

    #[test]
    fn test_cost_x_cards() {
        for card in ALL_CARDS.iter() {
            if card.cost == EnergyCost::X {
                assert!(has_x_effect(card));
            }

            if has_x_effect(card) {
                assert_eq!(card.cost, EnergyCost::X);
            }
        }
    }

    #[test]
    fn test_no_duplicates() {
        let mut seen = HashSet::new();
        for card in ALL_CARDS.iter() {
            assert!(seen.insert(card.card), "Duplicate card: {:?}", card.card);
        }
    }
}
