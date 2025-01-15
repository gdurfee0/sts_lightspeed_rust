use anyhow::anyhow;

use super::encounter::Encounter;

#[derive(Debug, PartialEq)]
pub struct Act {
    /// Offset used in the random number generator seed used in map generation.
    pub map_seed_offset: u64,

    /// How many "weak" monster encounters might occur in the Act (i.e. in the first few rooms).
    pub weak_monster_encounter_count: usize,

    /// Pool of possible "weak" monster encounters and their associated probabilities.
    pub weak_monster_encounter_pool: &'static [(Encounter, f32)],

    /// Pool of possible "strong" monster encounters and their associated probabilities.
    pub strong_monster_encounter_pool: &'static [(Encounter, f32)],

    /// Pool of elite encounters and their associated probabilities.
    pub elite_encounter_pool: &'static [(Encounter, f32)],

    /// Pool of boss encounters for the Act.
    pub boss_encounter_pool: &'static [Encounter],
}

impl Act {
    /// Obtains an Act by its number (1-4). Panics if the number is out of bounds.
    pub fn get(n: i8) -> &'static Act {
        <&'static Act>::try_from(n).unwrap()
    }

    /// Returns the Act that follows this one. Panics if this is Act 4.
    pub fn next_act(&self) -> &'static Act {
        if self == &ACTS[0] {
            &ACTS[1]
        } else if self == &ACTS[1] {
            &ACTS[2]
        } else if self == &ACTS[2] {
            &ACTS[3]
        } else {
            panic!("No act after Act 4");
        }
    }
}

impl TryFrom<i8> for &'static Act {
    type Error = anyhow::Error;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        match value {
            1..=4 => Ok(&ACTS[value as usize - 1]),
            _ => Err(anyhow!("Act must be between 1 and 4")),
        }
    }
}

static ACTS: &[Act] = &[
    // ACT 1
    Act {
        map_seed_offset: 1,
        weak_monster_encounter_count: 3,
        weak_monster_encounter_pool: &[
            (Encounter::Cultist, 1. / 4.),
            (Encounter::JawWorm, 1. / 4.),
            (Encounter::TwoLice, 1. / 4.),
            (Encounter::SmallSlimes, 1. / 4.),
        ],
        strong_monster_encounter_pool: &[
            (Encounter::GremlinGang, 1. / 16.),
            (Encounter::LotsOfSlimes, 1. / 16.),
            (Encounter::RedSlaver, 1. / 16.),
            (Encounter::ExordiumThugs, 1.5 / 16.),
            (Encounter::ExordiumWildlife, 1.5 / 16.),
            (Encounter::BlueSlaver, 2. / 16.),
            (Encounter::Looter, 2. / 16.),
            (Encounter::LargeSlime, 2. / 16.),
            (Encounter::ThreeLice, 2. / 16.),
            (Encounter::TwoFungiBeasts, 2. / 16.),
        ],
        elite_encounter_pool: &[
            (Encounter::GremlinNob, 1. / 3.),
            (Encounter::Lagavulin, 1. / 3.),
            (Encounter::ThreeSentries, 1. / 3.),
        ],
        boss_encounter_pool: &[
            Encounter::TheGuardian,
            Encounter::Hexaghost,
            Encounter::SlimeBoss,
        ],
    },
    // ACT 2
    Act {
        map_seed_offset: 200,
        weak_monster_encounter_count: 2,
        weak_monster_encounter_pool: &[
            (Encounter::SphericGuardian, 1. / 5.),
            (Encounter::Chosen, 1. / 5.),
            (Encounter::ShelledParasite, 1. / 5.),
            (Encounter::ThreeByrds, 1. / 5.),
            (Encounter::TwoThieves, 1. / 5.),
        ],
        strong_monster_encounter_pool: &[
            (Encounter::ChosenAndByrd, 2. / 29.),
            (Encounter::SentryAndSphericGuardian, 2. / 29.),
            (Encounter::CultistAndChosen, 3. / 29.),
            (Encounter::ThreeCultists, 3. / 29.),
            (Encounter::ShelledParasiteAndFungiBeast, 3. / 29.),
            (Encounter::Snecko, 4. / 29.),
            (Encounter::SnakePlant, 6. / 29.),
            (Encounter::CenturionAndMystic, 6. / 29.),
        ],
        elite_encounter_pool: &[
            (Encounter::GremlinLeader, 1. / 3.),
            (Encounter::Taskmaster, 1. / 3.),
            (Encounter::BookOfStabbing, 1. / 3.),
        ],
        boss_encounter_pool: &[
            Encounter::BronzeAutomaton,
            Encounter::TheCollector,
            Encounter::TheChamp,
        ],
    },
    // ACT 3
    Act {
        map_seed_offset: 600,
        weak_monster_encounter_count: 2,
        weak_monster_encounter_pool: &[
            (Encounter::ThreeDarklings, 1. / 3.),
            (Encounter::OrbWalker, 1. / 3.),
            (Encounter::ThreeShapes, 1. / 3.),
        ],
        strong_monster_encounter_pool: &[
            (Encounter::SpireGrowth, 1. / 8.),
            (Encounter::Transient, 1. / 8.),
            (Encounter::FourShapes, 1. / 8.),
            (Encounter::Maw, 1. / 8.),
            (Encounter::SphericGuardianAndTwoShapes, 1. / 8.),
            (Encounter::JawWormHorde, 1. / 8.),
            (Encounter::ThreeDarklings, 1. / 8.),
            (Encounter::WrithingMass, 1. / 8.),
        ],
        elite_encounter_pool: &[
            (Encounter::GiantHead, 1. / 3.),
            (Encounter::Nemesis, 1. / 3.),
            (Encounter::Reptomancer, 1. / 3.),
        ],
        boss_encounter_pool: &[
            Encounter::AwakenedOne,
            Encounter::TimeEater,
            Encounter::DonuAndDeca,
        ],
    },
    // ACT 4
    Act {
        map_seed_offset: 1200,
        weak_monster_encounter_count: 0,
        weak_monster_encounter_pool: &[],
        strong_monster_encounter_pool: &[],
        elite_encounter_pool: &[(Encounter::SpireShieldAndSpireSpear, 1.)],
        boss_encounter_pool: &[Encounter::CorruptHeart],
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_indexing() {
        assert_eq!(Act::get(1), &ACTS[0]);
    }
}