// Source: Slay the Spire Wiki (https://slay-the-spire.fandom.com/wiki/Category:Relic)

#[derive(Clone, Copy, Debug)]
#[cfg_attr(test, derive(Eq, Hash, PartialEq))]
pub enum Relic {
    /// Start each combat with 8 Vigor.
    Akabeko,
    /// Start each combat with 10 Block.
    Anchor,
    /// Whenever you enter a Rest Site, start the next combat with 2 extra Energy.
    AncientTeaSet,
    /// If you do not play any Attacks during your turn, gain an extra Energy next turn.
    ArtOfWar,
    /// Upon pickup, choose and Transform 3 cards, then Upgrade them.
    Astrolabe,
    /// At the start of each combat, apply 1 Vulnerable to ALL enemies.
    BagOfMarbles,
    /// At the start of each combat, draw 2 additional cards.
    BagOfPreparation,
    /// Whenever you play a Power, heal 2 HP.
    BirdFacedUrn,
    /// Replaces BurningBlood Burning Blood. At the end of combat, heal 12 HP.
    BlackBlood,
    /// Elites drop an additional Relic when defeated.
    BlackStar,
    /// At the start of each combat, heal 2 HP.
    BloodVial,
    /// Whenever you gain Gold, heal 5 HP.
    BloodyIdol,
    /// Curse cards can now be played. Playing a Curse will make you lose 1 HP & Exhausts the card.
    BlueCandle,
    /// Upon pick up, choose an Attack. Start each combat with this card in your hand.
    BottledFlame,
    /// Upon pick up, choose a Skill. Start each combat with this card in your hand.
    BottledLightning,
    /// Upon pick up, choose a Power card. Start each combat with this card in your hand.
    BottledTornado,
    /// At the start of your turn, gain 2 Strength and ALL enemies gain 1 Strength.
    Brimstone,
    /// Start each combat with 3 Thorns.
    BronzeScales,
    /// At the end of combat, heal 6 HP.
    BurningBlood,
    /// Gain 1 Energy at the start of each turn. On Card Reward screens, you have 2 fewer cards to
    /// choose from.
    BustedCrown,
    /// At the start of your turn, lose 15 Block rather than all of your Block.
    Calipers,
    /// Upon pickup, obtain a unique Curse and 3 relics.
    CallingBell,
    /// At the start of your 3rd turn, gain 18 Block.
    CaptainsWheel,
    /// Upon pickup, brews 5 random potions.
    Cauldron,
    /// The first time you lose HP each combat, draw 3 cards.
    CentennialPuzzle,
    /// Whenever you add a card to your deck, gain 9 gold.
    CeramicFish,
    /// Whenever you apply Vulnerable, also apply 1 Weak.
    ChampionBelt,
    /// Whenever you Exhaust a card, deal 3 damage to ALL enemies.
    CharonsAshes,
    /// Whenever you play a cost X card, its effects are increased by 2.
    ChemicalX,
    /// Collect as many as you can.
    Circlet,
    /// At the end of your turn, gain 1 Block for each card in your hand.
    CloakClasp,
    /// At the start of each combat, gain 1 Artifact.
    ClockworkSouvenir,
    /// Gain 1 Energy at the start of each turn. You can no longer Rest at Rest Sites.
    CoffeeDripper,
    /// At the start of each combat, Channel 1 Lightning.
    CrackedCore,
    /// You feel more talkative.
    CultistHeadpiece,
    /// Gain 1 Energy at the start of each turn. Whenever you open a non-boss Chest, obtain a Curse.
    CursedKey,
    /// At the start of your turn, gain 1 Mantra.
    Damaru,
    /// Whenever you obtain a Curse, increase your Max HP by 6.
    DarkstonePeriapt,
    /// Start each combat with 1 Focus.
    DataDisk,
    /// Whenever you Exhaust a card, add a random card to your hand.
    DeadBranch,
    /// Upon pickup, obtain an additional copy of a card in your deck.
    DollysMirror,
    /// Whenever you rest, you may add a card to your deck.
    DreamCatcher,
    /// For each Curse in your deck, start each combat with 1 additional Strength.
    DuVuDoll,
    /// Whenever you play an Attack, gain 1 temporary Dexterity.
    Duality,
    /// Gain 1 Energy at the start of each turn. You can no longer gain Gold.
    Ectoplasm,
    /// If you lost HP during the previous turn, trigger the passive ability of all Orbs
    /// at the start of your turn.
    EmotionChip,
    /// Upon pickup, remove 2 cards from your Deck.
    EmptyCage,
    /// At the start of each combat, add a random Power card to your hand. It costs 0 until the
    /// end of turn.
    Enchiridion,
    /// For every 5 cards in your deck, heal 3 HP whenever you enter a Rest Site.
    EternalFeather,
    /// Raise your Max HP by 1 after each combat.
    FaceOfCleric,
    /// Prevent the first time you would lose HP in combat.
    FossilizedHelix,
    /// Replaces Cracked Core. If you end your turn with empty Orb slots, channel 1 Frost.
    FrozenCore,
    /// Whenever you add a Power card to your deck, it is Upgraded.
    FrozenEgg,
    /// When viewing your Draw Pile, the cards are now shown in order.
    FrozenEye,
    /// Gain 1 Energy at the start of each turn. You can no longer Smith at Rest Sites.
    FusionHammer,
    /// At the start of each combat, discard any number of cards then draw that many.
    GamblingChip,
    /// You can no longer become Weakened.
    Ginger,
    /// You can now gain Icon Strength Strength at Rest Sites. (3 times max)
    Girya,
    /// Your rightmost Orb triggers its passive an additional time.
    GoldPlatedCables,
    /// Whenever you Scry, Scry 2 additional cards.
    GoldenEye,
    /// Enemies drop 25% more gold.
    GoldenIdol,
    /// Whenever an enemy dies, gain 1 Energy and draw 1 card.
    GremlinHorn,
    /// Start each combat with 1 Weak.
    GremlinVisage,
    /// Whenever you break an enemy's Block, apply 2 Vulnerable.
    HandDrill,
    /// Every 3 turns, gain 1 Energy.
    HappyFlower,
    /// Replaces Pure Water. At the start of each combat, add 3 Miracles to your hand.
    HolyWater,
    /// At the start of your 2nd turn, gain 14 Block.
    HornCleat,
    /// The first time you discard a card each turn, gain 1 Energy.
    HoveringKite,
    /// Energy is now conserved between turns.
    IceCream,
    /// Every 6 turns, gain 1 Intangible.
    IncenseBurner,
    /// Whenever you play 10 cards, draw 1 card.
    InkBottle,
    /// Every 2 turns, gain 1 Orb slot.
    Inserter,
    /// Regular enemy combats are no longer encountered in Event rooms.
    JuzuBracelet,
    /// Every time you play 3 Attacks in a single turn, gain 1 Dexterity.
    Kunai,
    /// Gain 1 Energy on the first turn of each combat.
    Lantern,
    /// Raise your Max HP by 7 and heal all of your HP.
    LeesWaffle,
    /// Every time you play 3 Skills in a single turn, deal 5 damage to ALL enemies.
    LetterOpener,
    /// When you would die, heal to 50% of your Max HP instead (works once).
    LizardTail,
    MagicFlower,
    Mango,
    MarkOfPain,
    MarkOfTheBloom,
    Matryoshka,
    MawBank,
    MealTicket,
    MeatOnTheBone,
    MedicalKit,
    Melange,
    MembershipCard,
    MercuryHourglass,
    MoltenEgg,
    MummifiedHand,
    MutagenicStrength,
    NlothsGift,
    NlothsHungryFace,
    Necronomicon,
    NeowsLament,
    NilrysCodex,
    NinjaScroll,
    NuclearBattery,
    Nunchaku,
    OddMushroom,
    OddlySmoothStone,
    OldCoin,
    Omamori,
    OrangePellets,
    Orichalcum,
    OrnamentalFan,
    Orrery,
    PandorasBox,
    Pantograph,
    PaperKrane,
    PaperPhrog,
    PeacePipe,
    Pear,
    PenNib,
    PhilosophersStone,
    Pocketwatch,
    PotionBelt,
    PrayerWheel,
    PreservedInsect,
    PrismaticShard,
    PureWater,
    QuestionCard,
    RedMask,
    RedSkull,
    RegalPillow,
    RingOfTheSerpent,
    RingOfTheSnake,
    RunicCapacitor,
    RunicCube,
    RunicDome,
    RunicPyramid,
    SacredBark,
    SelfFormingClay,
    Shovel,
    Shuriken,
    SingingBowl,
    SlaversCollar,
    SlingOfCourage,
    SmilingMask,
    SneckoEye,
    SneckoSkull,
    Sozu,
    SpiritPoop,
    SsserpentHead,
    StoneCalendar,
    StrangeSpoon,
    Strawberry,
    StrikeDummy,
    Sundial,
    SymbioticVirus,
    TeardropLocket,
    TheAbacus,
    TheBoot,
    TheCourier,
    TheSpecimen,
    ThreadAndNeedle,
    Tingsha,
    TinyChest,
    TinyHouse,
    Toolbox,
    Torii,
    ToughBandages,
    ToxicEgg,
    ToyOrnithopter,
    TungstenRod,
    Turnip,
    TwistedFunnel,
    UnceasingTop,
    Vajra,
    VelvetChoker,
    VioletLotus,
    WarPaint,
    WarpedTongs,
    Whetstone,
    WhiteBeastStatue,
    WingBoots,
    WristBlade,
}
