use std::ops::RangeInclusive;

#[derive(Clone, Copy, Debug)]
pub enum EnemyType {
    AcidSlimeL,
    AcidSlimeM,
    AcidSlimeS,
    AwakenedOne,
    Bear,
    BlueSlaver,
    BookOfStabbing,
    BronzeAutomaton,
    BronzeOrb,
    Byrd,
    Centurion,
    Chosen,
    CorruptHeart,
    Cultist,
    Dagger,
    Darkling,
    Deca,
    Donu,
    Exploder,
    FatGremlin,
    FungiBeast,
    GiantHead,
    GreenLouse,
    GremlinLeader,
    GremlinNob,
    GremlinWizard,
    Hexaghost,
    JawWorm,
    Lagavulin,
    Looter,
    MadGremlin,
    Mugger,
    Mystic,
    Nemesis,
    OrbWalker,
    Pointy,
    RedLouse,
    RedSlaver,
    Reptomancer,
    Repulsor,
    Romeo,
    Sentry,
    ShelledParasite,
    ShieldGremlin,
    SlimeBoss,
    SnakePlant,
    SneakyGremlin,
    Snecko,
    SphericGuardian,
    Spiker,
    SpikeSlimeL,
    SpikeSlimeM,
    SpikeSlimeS,
    SpireGrowth,
    SpireShield,
    SpireSpear,
    Taskmaster,
    TheChamp,
    TheCollector,
    TheGuardian,
    TheMaw,
    TimeEater,
    TorchHead,
    Transient,
    WrithingMass,
}

#[derive(Debug)]
pub struct EnemyTemplate {
    // TODO: ascension-dependent HP ranges
    pub enemy_type: EnemyType,
    pub hp: RangeInclusive<u32>,
}

impl From<EnemyType> for EnemyTemplate {
    // TODO: revisit this pattern if memory usage becomes a concern
    fn from(enemy_type: EnemyType) -> EnemyTemplate {
        let hp = match enemy_type {
            EnemyType::AcidSlimeM => 28..=32,
            EnemyType::AcidSlimeS => 8..=12,
            EnemyType::SpikeSlimeM => 28..=32,
            EnemyType::SpikeSlimeS => 10..=14,
            _ => todo!(),
        };
        EnemyTemplate { enemy_type, hp }
    }
}
