// Source: Slay the Spire Wiki (https://slay-the-spire.fandom.com/wiki/Category:Relic)

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Relic {
    Akabeko,
    Anchor,
    AncientTeaSet,
    ArtOfWar,
    Astrolabe,
    BagOfMarbles,
    BagOfPreparation,
    BirdFacedUrn,
    BlackBlood,
    BlackStar,
    BloodVial,
    BloodyIdol,
    BlueCandle,
    BottledFlame,
    BottledLightning,
    BottledTornado,
    Brimstone,
    BronzeScales,
    BurningBlood,
    BustedCrown,
    Calipers,
    CallingBell,
    CaptainsWheel,
    Cauldron,
    CentennialPuzzle,
    CeramicFish,
    ChampionBelt,
    CharonsAshes,
    ChemicalX,
    Circlet,
    CloakClasp,
    ClockworkSouvenir,
    CoffeeDripper,
    CrackedCore,
    CultistHeadpiece,
    CursedKey,
    Damaru,
    DarkstonePeriapt,
    DataDisk,
    DeadBranch,
    DollysMirror,
    DreamCatcher,
    DuVuDoll,
    Duality,
    Ectoplasm,
    EmotionChip,
    EmptyCage,
    Enchiridion,
    EternalFeather,
    FaceOfCleric,
    FossilizedHelix,
    FrozenCore,
    FrozenEgg,
    FrozenEye,
    FusionHammer,
    GamblingChip,
    Ginger,
    Girya,
    GoldPlatedCables,
    GoldenEye,
    GoldenIdol,
    GremlinHorn,
    GremlinVisage,
    HandDrill,
    HappyFlower,
    HolyWater,
    HornCleat,
    HoveringKite,
    IceCream,
    IncenseBurner,
    InkBottle,
    Inserter,
    JuzuBracelet,
    Kunai,
    Lantern,
    LeesWaffle,
    LetterOpener,
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

impl Relic {
    pub fn description(&self) -> &'static str {
        match self {
            // +8 Vigor
            Relic::Akabeko => "Your first attack each combat deals 8 additional damage",
            // Turn 1 => +10 Block
            Relic::Anchor => "Start each combat with 10 Block",
            // Rest Site => Turn 1 => +2 Energy
            Relic::AncientTeaSet => {
                "Whenever you enter a Rest Site, start the next combat with 2 extra Energy"
            }
            // No Attacks => +1 Energy
            Relic::ArtOfWar => {
                "If you do not play any Attacks during your turn, gain an extra Energy next turn"
            }
            // 3x Transform => Upgrade (TODO: order?)
            Relic::Astrolabe => "Upon pickup, choose and Transform 3 cards, then Upgrade them",
            // Turn 1 => +1 Vulnerable to ALL enemies
            Relic::BagOfMarbles => "At the start of each combat, apply 1 Vulnerable to ALL enemies",
            // Turn 1 => +2 Cards
            Relic::BagOfPreparation => "At the start of each combat, draw 2 additional cards",
            // play Power => +2 HP
            Relic::BirdFacedUrn => "Whenever you play a Power, heal 2 HP",
            // Remove Burning Blood. End of combat => +12 HP
            Relic::BlackBlood => "Replaces Burning Blood. At the end of combat, heal 12 HP",
            // Elite => +1 Relic
            Relic::BlackStar => "Elites drop an additional Relic when defeated",
            // Turn 1 => +2 HP
            Relic::BloodVial => "At the start of each combat, heal 2 HP",
            // Gain gold => +5 HP
            Relic::BloodyIdol => "Whenever you gain Gold, heal 5 HP",
            // Playable Curse cards
            Relic::BlueCandle => concat!(
                "Curse cards can now be played. ",
                "Playing a Curse will make you lose 1 HP and Exhausts the card"
            ),
            // Choose Attack => Start with card in hand
            Relic::BottledFlame => {
                "Upon pick up, choose an Attack. Start each combat with this card in your hand"
            }
            // Choose Skill => Start with card in hand
            Relic::BottledLightning => {
                "Upon pick up, choose an Skill. Start each combat with this card in your hand"
            }
            // Choose Power => Start with card in hand
            Relic::BottledTornado => {
                "Upon pick up, choose a Power card. Start each combat with this card in your hand"
            }
            Relic::Brimstone => todo!("{:?}", self),
            Relic::BronzeScales => todo!("{:?}", self),
            Relic::BurningBlood => todo!("{:?}", self),
            Relic::BustedCrown => todo!("{:?}", self),
            Relic::Calipers => todo!("{:?}", self),
            Relic::CallingBell => todo!("{:?}", self),
            Relic::CaptainsWheel => todo!("{:?}", self),
            Relic::Cauldron => todo!("{:?}", self),
            Relic::CentennialPuzzle => todo!("{:?}", self),
            Relic::CeramicFish => todo!("{:?}", self),
            Relic::ChampionBelt => todo!("{:?}", self),
            Relic::CharonsAshes => todo!("{:?}", self),
            Relic::ChemicalX => todo!("{:?}", self),
            Relic::Circlet => todo!("{:?}", self),
            Relic::CloakClasp => todo!("{:?}", self),
            Relic::ClockworkSouvenir => todo!("{:?}", self),
            Relic::CoffeeDripper => todo!("{:?}", self),
            Relic::CrackedCore => todo!("{:?}", self),
            Relic::CultistHeadpiece => todo!("{:?}", self),
            Relic::CursedKey => todo!("{:?}", self),
            Relic::Damaru => todo!("{:?}", self),
            Relic::DarkstonePeriapt => todo!("{:?}", self),
            Relic::DataDisk => todo!("{:?}", self),
            Relic::DeadBranch => todo!("{:?}", self),
            Relic::DollysMirror => todo!("{:?}", self),
            Relic::DreamCatcher => todo!("{:?}", self),
            Relic::DuVuDoll => todo!("{:?}", self),
            Relic::Duality => todo!("{:?}", self),
            Relic::Ectoplasm => todo!("{:?}", self),
            Relic::EmotionChip => todo!("{:?}", self),
            Relic::EmptyCage => todo!("{:?}", self),
            Relic::Enchiridion => todo!("{:?}", self),
            Relic::EternalFeather => todo!("{:?}", self),
            Relic::FaceOfCleric => todo!("{:?}", self),
            Relic::FossilizedHelix => todo!("{:?}", self),
            Relic::FrozenCore => todo!("{:?}", self),
            Relic::FrozenEgg => todo!("{:?}", self),
            Relic::FrozenEye => todo!("{:?}", self),
            Relic::FusionHammer => todo!("{:?}", self),
            Relic::GamblingChip => todo!("{:?}", self),
            Relic::Ginger => todo!("{:?}", self),
            Relic::Girya => todo!("{:?}", self),
            Relic::GoldPlatedCables => todo!("{:?}", self),
            Relic::GoldenEye => todo!("{:?}", self),
            Relic::GoldenIdol => todo!("{:?}", self),
            Relic::GremlinHorn => todo!("{:?}", self),
            Relic::GremlinVisage => todo!("{:?}", self),
            Relic::HandDrill => todo!("{:?}", self),
            Relic::HappyFlower => todo!("{:?}", self),
            Relic::HolyWater => todo!("{:?}", self),
            Relic::HornCleat => todo!("{:?}", self),
            Relic::HoveringKite => todo!("{:?}", self),
            Relic::IceCream => todo!("{:?}", self),
            Relic::IncenseBurner => todo!("{:?}", self),
            Relic::InkBottle => todo!("{:?}", self),
            Relic::Inserter => todo!("{:?}", self),
            Relic::JuzuBracelet => todo!("{:?}", self),
            Relic::Kunai => todo!("{:?}", self),
            Relic::Lantern => todo!("{:?}", self),
            Relic::LeesWaffle => todo!("{:?}", self),
            Relic::LetterOpener => todo!("{:?}", self),
            Relic::LizardTail => todo!("{:?}", self),
            Relic::MagicFlower => todo!("{:?}", self),
            Relic::Mango => todo!("{:?}", self),
            Relic::MarkOfPain => todo!("{:?}", self),
            Relic::MarkOfTheBloom => todo!("{:?}", self),
            Relic::Matryoshka => todo!("{:?}", self),
            Relic::MawBank => todo!("{:?}", self),
            Relic::MealTicket => todo!("{:?}", self),
            Relic::MeatOnTheBone => todo!("{:?}", self),
            Relic::MedicalKit => todo!("{:?}", self),
            Relic::Melange => todo!("{:?}", self),
            Relic::MembershipCard => todo!("{:?}", self),
            Relic::MercuryHourglass => todo!("{:?}", self),
            Relic::MoltenEgg => todo!("{:?}", self),
            Relic::MummifiedHand => todo!("{:?}", self),
            Relic::MutagenicStrength => todo!("{:?}", self),
            Relic::NlothsGift => todo!("{:?}", self),
            Relic::NlothsHungryFace => todo!("{:?}", self),
            Relic::Necronomicon => todo!("{:?}", self),
            Relic::NeowsLament => todo!("{:?}", self),
            Relic::NilrysCodex => todo!("{:?}", self),
            Relic::NinjaScroll => todo!("{:?}", self),
            Relic::NuclearBattery => todo!("{:?}", self),
            Relic::Nunchaku => todo!("{:?}", self),
            Relic::OddMushroom => todo!("{:?}", self),
            Relic::OddlySmoothStone => todo!("{:?}", self),
            Relic::OldCoin => todo!("{:?}", self),
            Relic::Omamori => todo!("{:?}", self),
            Relic::OrangePellets => todo!("{:?}", self),
            Relic::Orichalcum => todo!("{:?}", self),
            Relic::OrnamentalFan => todo!("{:?}", self),
            Relic::Orrery => todo!("{:?}", self),
            Relic::PandorasBox => todo!("{:?}", self),
            Relic::Pantograph => todo!("{:?}", self),
            Relic::PaperKrane => todo!("{:?}", self),
            Relic::PaperPhrog => todo!("{:?}", self),
            Relic::PeacePipe => todo!("{:?}", self),
            Relic::Pear => todo!("{:?}", self),
            Relic::PenNib => todo!("{:?}", self),
            Relic::PhilosophersStone => todo!("{:?}", self),
            Relic::Pocketwatch => todo!("{:?}", self),
            Relic::PotionBelt => todo!("{:?}", self),
            Relic::PrayerWheel => todo!("{:?}", self),
            Relic::PreservedInsect => todo!("{:?}", self),
            Relic::PrismaticShard => todo!("{:?}", self),
            Relic::PureWater => todo!("{:?}", self),
            Relic::QuestionCard => todo!("{:?}", self),
            Relic::RedMask => todo!("{:?}", self),
            Relic::RedSkull => todo!("{:?}", self),
            Relic::RegalPillow => todo!("{:?}", self),
            Relic::RingOfTheSerpent => todo!("{:?}", self),
            Relic::RingOfTheSnake => todo!("{:?}", self),
            Relic::RunicCapacitor => todo!("{:?}", self),
            Relic::RunicCube => todo!("{:?}", self),
            Relic::RunicDome => todo!("{:?}", self),
            Relic::RunicPyramid => todo!("{:?}", self),
            Relic::SacredBark => todo!("{:?}", self),
            Relic::SelfFormingClay => todo!("{:?}", self),
            Relic::Shovel => todo!("{:?}", self),
            Relic::Shuriken => todo!("{:?}", self),
            Relic::SingingBowl => todo!("{:?}", self),
            Relic::SlaversCollar => todo!("{:?}", self),
            Relic::SlingOfCourage => todo!("{:?}", self),
            Relic::SmilingMask => todo!("{:?}", self),
            Relic::SneckoEye => todo!("{:?}", self),
            Relic::SneckoSkull => todo!("{:?}", self),
            Relic::Sozu => todo!("{:?}", self),
            Relic::SpiritPoop => todo!("{:?}", self),
            Relic::SsserpentHead => todo!("{:?}", self),
            Relic::StoneCalendar => todo!("{:?}", self),
            Relic::StrangeSpoon => todo!("{:?}", self),
            Relic::Strawberry => todo!("{:?}", self),
            Relic::StrikeDummy => todo!("{:?}", self),
            Relic::Sundial => todo!("{:?}", self),
            Relic::SymbioticVirus => todo!("{:?}", self),
            Relic::TeardropLocket => todo!("{:?}", self),
            Relic::TheAbacus => todo!("{:?}", self),
            Relic::TheBoot => todo!("{:?}", self),
            Relic::TheCourier => todo!("{:?}", self),
            Relic::TheSpecimen => todo!("{:?}", self),
            Relic::ThreadAndNeedle => todo!("{:?}", self),
            Relic::Tingsha => todo!("{:?}", self),
            Relic::TinyChest => todo!("{:?}", self),
            Relic::TinyHouse => todo!("{:?}", self),
            Relic::Toolbox => todo!("{:?}", self),
            Relic::Torii => todo!("{:?}", self),
            Relic::ToughBandages => todo!("{:?}", self),
            Relic::ToxicEgg => todo!("{:?}", self),
            Relic::ToyOrnithopter => todo!("{:?}", self),
            Relic::TungstenRod => todo!("{:?}", self),
            Relic::Turnip => todo!("{:?}", self),
            Relic::TwistedFunnel => todo!("{:?}", self),
            Relic::UnceasingTop => todo!("{:?}", self),
            Relic::Vajra => todo!("{:?}", self),
            Relic::VelvetChoker => todo!("{:?}", self),
            Relic::VioletLotus => todo!("{:?}", self),
            Relic::WarPaint => todo!("{:?}", self),
            Relic::WarpedTongs => todo!("{:?}", self),
            Relic::Whetstone => todo!("{:?}", self),
            Relic::WhiteBeastStatue => todo!("{:?}", self),
            Relic::WingBoots => todo!("{:?}", self),
            Relic::WristBlade => todo!("{:?}", self),
        }
    }
}
