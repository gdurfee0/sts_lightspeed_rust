use anyhow::anyhow;

use super::encounter::Encounter;

#[derive(Debug, PartialEq)]
pub struct Act {
    map_seed_offset: u64,
    weak_monster_encounter_count: usize,
    weak_monster_encounters_and_probs: &'static [(Encounter, f32)],
    strong_monster_encounters_and_probs: &'static [(Encounter, f32)],
    elite_encounters: &'static [(Encounter, f32)],
    boss_encounters: &'static [Encounter],
}

impl Act {
    /// Obtains an Act by its number (1-4). Panics if the number is out of bounds.
    pub fn get(n: i8) -> &'static Act {
        <&'static Act>::try_from(n).unwrap()
    }

    /// Obtains an Act from a string representation of its number. Returns an error if the string is
    /// not a valid number or if the number is out of bounds.
    pub fn from_str(s: &str) -> Result<&'static Act, anyhow::Error> {
        <&'static Act>::try_from(s)
    }

    /// Offset used in the random number generator seed used in map generation.
    pub fn map_seed_offset(&self) -> u64 {
        self.map_seed_offset
    }

    /// How many "weak" monster encounters might occur in the Act (i.e. in the first few rooms).
    pub fn weak_monster_encounter_count(&self) -> usize {
        self.weak_monster_encounter_count
    }

    /// Pool of possible "weak" monster encounters and their associated probabilities.
    pub fn weak_monster_pool_with_probs(&self) -> &'static [(Encounter, f32)] {
        self.weak_monster_encounters_and_probs
    }

    /// Pool of possible "strong" monster encounters and their associated probabilities.
    pub fn strong_monster_pool_with_probs(&self) -> &'static [(Encounter, f32)] {
        self.strong_monster_encounters_and_probs
    }

    /// Pool of elite encounters and their associated probabilities.
    pub fn elite_pool_with_probs(&self) -> &'static [(Encounter, f32)] {
        self.elite_encounters
    }

    /// Pool of boss encounters for the Act.
    pub fn boss_pool(&self) -> &'static [Encounter] {
        self.boss_encounters
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

impl TryFrom<&str> for &'static Act {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        <&'static Act>::try_from(value.parse::<i8>()?)
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

pub static ACTS: &[Act] = &[
    // ACT 1
    Act {
        map_seed_offset: 1,
        weak_monster_encounter_count: 3,
        weak_monster_encounters_and_probs: &[
            (Encounter::Cultist, 1. / 4.),
            (Encounter::JawWorm, 1. / 4.),
            (Encounter::TwoLice, 1. / 4.),
            (Encounter::SmallSlimes, 1. / 4.),
        ],
        strong_monster_encounters_and_probs: &[
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
        elite_encounters: &[
            (Encounter::GremlinNob, 1. / 3.),
            (Encounter::Lagavulin, 1. / 3.),
            (Encounter::ThreeSentries, 1. / 3.),
        ],
        boss_encounters: &[
            Encounter::TheGuardian,
            Encounter::Hexaghost,
            Encounter::SlimeBoss,
        ],
    },
    // ACT 2
    Act {
        map_seed_offset: 200,
        weak_monster_encounter_count: 2,
        weak_monster_encounters_and_probs: &[
            (Encounter::SphericGuardian, 1. / 5.),
            (Encounter::Chosen, 1. / 5.),
            (Encounter::ShelledParasite, 1. / 5.),
            (Encounter::ThreeByrds, 1. / 5.),
            (Encounter::TwoThieves, 1. / 5.),
        ],
        strong_monster_encounters_and_probs: &[
            (Encounter::ChosenAndByrd, 2. / 29.),
            (Encounter::SentryAndSphericGuardian, 2. / 29.),
            (Encounter::CultistAndChosen, 3. / 29.),
            (Encounter::ThreeCultists, 3. / 29.),
            (Encounter::ShelledParasiteAndFungiBeast, 3. / 29.),
            (Encounter::Snecko, 4. / 29.),
            (Encounter::SnakePlant, 6. / 29.),
            (Encounter::CenturionAndMystic, 6. / 29.),
        ],
        elite_encounters: &[
            (Encounter::GremlinLeader, 1. / 3.),
            (Encounter::Taskmaster, 1. / 3.),
            (Encounter::BookOfStabbing, 1. / 3.),
        ],
        boss_encounters: &[
            Encounter::BronzeAutomaton,
            Encounter::TheCollector,
            Encounter::TheChamp,
        ],
    },
    // ACT 3
    Act {
        map_seed_offset: 600,
        weak_monster_encounter_count: 2,
        weak_monster_encounters_and_probs: &[
            (Encounter::ThreeDarklings, 1. / 3.),
            (Encounter::OrbWalker, 1. / 3.),
            (Encounter::ThreeShapes, 1. / 3.),
        ],
        strong_monster_encounters_and_probs: &[
            (Encounter::SpireGrowth, 1. / 8.),
            (Encounter::Transient, 1. / 8.),
            (Encounter::FourShapes, 1. / 8.),
            (Encounter::Maw, 1. / 8.),
            (Encounter::SphericGuardianAndTwoShapes, 1. / 8.),
            (Encounter::JawWormHorde, 1. / 8.),
            (Encounter::ThreeDarklings, 1. / 8.),
            (Encounter::WrithingMass, 1. / 8.),
        ],
        elite_encounters: &[
            (Encounter::GiantHead, 1. / 3.),
            (Encounter::Nemesis, 1. / 3.),
            (Encounter::Reptomancer, 1. / 3.),
        ],
        boss_encounters: &[
            Encounter::AwakenedOne,
            Encounter::TimeEater,
            Encounter::DonuAndDeca,
        ],
    },
    // ACT 4
    Act {
        map_seed_offset: 1200,
        weak_monster_encounter_count: 0,
        weak_monster_encounters_and_probs: &[],
        strong_monster_encounters_and_probs: &[],
        elite_encounters: &[(Encounter::SpireShieldAndSpireSpear, 1.)],
        boss_encounters: &[Encounter::CorruptHeart],
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ascension_from_str() {
        assert_eq!(Act::from_str("1").unwrap(), Act::get(1));
        assert_eq!(Act::from_str("4").unwrap(), Act::get(4));
        assert!(Act::from_str("0").is_err());
        assert!(Act::from_str("5").is_err());
        assert!(Act::from_str("").is_err());
        assert!(Act::from_str("-1").is_err());
        assert!(Act::from_str("ZZZ").is_err());
    }

    #[test]
    fn test_array_indexing() {
        assert_eq!(Act::get(1), &ACTS[0]);
    }
}
