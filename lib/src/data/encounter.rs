// Source: Slay the Spire Wiki (https://slay-the-spire.fandom.com/wiki/Category:Act)

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Encounter {
    AwakenedOne,
    BlueSlaver,
    BookOfStabbing,
    BronzeAutomaton,
    CenturionAndMystic,
    Chosen,
    ChosenAndByrd,
    CorruptHeart,
    Cultist,
    CultistAndChosen,
    DonuAndDeca,
    ExordiumThugs,
    ExordiumWildlife,
    FourShapes,
    GiantHead,
    GremlinGang,
    GremlinLeader,
    GremlinNob,
    Hexaghost,
    JawWorm,
    JawWormHorde,
    Lagavulin,
    LargeSlime,
    Looter,
    LotsOfSlimes,
    Maw,
    Nemesis,
    OrbWalker,
    RedSlaver,
    Reptomancer,
    SentryAndSphericGuardian,
    ShelledParasite,
    ShelledParasiteAndFungiBeast,
    SlimeBoss,
    SmallSlimes,
    SnakePlant,
    Snecko,
    SphericGuardian,
    SphericGuardianAndTwoShapes,
    SpireGrowth,
    SpireShieldAndSpireSpear,
    Taskmaster,
    TheChamp,
    TheCollector,
    TheGuardian,
    ThreeByrds,
    ThreeCultists,
    ThreeDarklings,
    ThreeLouses,
    ThreeSentries,
    ThreeShapes,
    TimeEater,
    Transient,
    TwoFungiBeasts,
    TwoLouses,
    TwoThieves,
    WrithingMass,
}

/*
pub struct EncounterDetails {
    pub enemy_party: [Option<Enemy>; 5],
}

pub struct EncounterDetailsBuilder {
    enemy_party: Vec<Enemy>,
}

impl EncounterDetailsBuilder {
    pub fn new() -> Self {
        Self {
            enemy_party: vec![],
        }
    }

    pub fn push(mut self, enemy: Enemy) -> Self {
        self.enemy_party.push(enemy);
        assert!(self.enemy_party.len() <= 5);
        self
    }

    pub fn build(self) -> EncounterDetails {
        EncounterDetails {
            enemy_party: {
                let mut party = [None, None, None, None, None];
                for (i, enemy) in self.enemy_party.into_iter().enumerate() {
                    party[i] = Some(enemy);
                }
                party
            },
        }
    }
}

macro_rules! define_encounter {
    ($variant:ident => [$($enemy:ident),*]) => {
        (
            Encounter::$variant,
            EncounterDetailsBuilder::new()$(.push(Enemy::$enemy))*.build()
        )
    };
}

macro_rules! define_encounters {
    ($($variant:ident => [$($enemy:ident),*]),*) => {
        once_cell::sync::Lazy::new(
            || {
                vec![$(define_encounter!($variant => [$($enemy),*]),)*]
                    .into_iter()
                    .collect::<HashMap<_, _>>()
            }
        );
    }
}

static ALL_ENCOUNTERS: Lazy<HashMap<Encounter, EncounterDetails>> = define_encounters!(
    AwakenedOne => [AwakenedOne],

);
*/
