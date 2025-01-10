#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MonsterEncounter {
    BlueSlaver,
    CenturionAndMystic,
    Chosen,
    ChosenAndByrd,
    Cultist,
    CultistAndChosen,
    ExordiumThugs,
    ExordiumWildlife,
    FourShapes,
    GremlinGang,
    JawWorm,
    JawWormHorde,
    LargeSlime,
    Looter,
    LotsOfSlimes,
    Maw,
    OrbWalker,
    RedSlaver,
    SentryAndSphericGuardian,
    ShelledParasite,
    ShelledParasiteAndFungiBeast,
    SmallSlimes,
    SnakePlant,
    Snecko,
    SphericGuardian,
    SphericGuardianAndTwoShapes,
    SpireGrowth,
    ThreeByrds,
    ThreeCultists,
    ThreeDarklings,
    ThreeLice,
    ThreeShapes,
    Transient,
    TwoFungiBeasts,
    TwoLice,
    TwoThieves,
    WrithingMass,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EliteEncounter {
    GremlinNob,
    Lagavulin,
    ThreeSentries,

    BookOfStabbing,
    GremlinLeader,
    Taskmaster,

    GiantHead,
    Nemesis,
    Reptomancer,

    SpireShieldAndSpireSpear,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BossEncounter {
    TheGuardian,
    Hexaghost,
    SlimeBoss,

    BronzeAutomaton,
    TheChamp,
    TheCollector,

    AwakenedOne,
    DonuAndDeca,
    TimeEater,

    CorruptHeart,
}
