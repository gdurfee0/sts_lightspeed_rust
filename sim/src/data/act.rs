use anyhow::anyhow;

use super::encounter::{BossEncounter, EliteEncounter, MonsterEncounter};

#[derive(Debug, PartialEq)]
pub struct Act {
    map_seed_offset: u64,
    weak_monster_encounter_count: usize,
    weak_monster_encounters_and_probs: &'static [(MonsterEncounter, f32)],
    strong_monster_encounters_and_probs: &'static [(MonsterEncounter, f32)],
    elite_encounters: &'static [(EliteEncounter, f32)],
    boss_encounters: &'static [BossEncounter],
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
    pub fn weak_monster_pool_with_probs(&self) -> &'static [(MonsterEncounter, f32)] {
        self.weak_monster_encounters_and_probs
    }

    /// Pool of possible "strong" monster encounters and their associated probabilities.
    pub fn strong_monster_pool_with_probs(&self) -> &'static [(MonsterEncounter, f32)] {
        self.strong_monster_encounters_and_probs
    }

    /// Pool of elite encounters and their associated probabilities.
    pub fn elite_pool_with_probs(&self) -> &'static [(EliteEncounter, f32)] {
        self.elite_encounters
    }

    /// Pool of boss encounters for the Act.
    pub fn boss_pool(&self) -> &'static [BossEncounter] {
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
            (MonsterEncounter::Cultist, 1. / 4.),
            (MonsterEncounter::JawWorm, 1. / 4.),
            (MonsterEncounter::TwoLice, 1. / 4.),
            (MonsterEncounter::SmallSlimes, 1. / 4.),
        ],
        strong_monster_encounters_and_probs: &[
            (MonsterEncounter::GremlinGang, 1. / 16.),
            (MonsterEncounter::LotsOfSlimes, 1. / 16.),
            (MonsterEncounter::RedSlaver, 1. / 16.),
            (MonsterEncounter::ExordiumThugs, 1.5 / 16.),
            (MonsterEncounter::ExordiumWildlife, 1.5 / 16.),
            (MonsterEncounter::BlueSlaver, 2. / 16.),
            (MonsterEncounter::Looter, 2. / 16.),
            (MonsterEncounter::LargeSlime, 2. / 16.),
            (MonsterEncounter::ThreeLice, 2. / 16.),
            (MonsterEncounter::TwoFungiBeasts, 2. / 16.),
        ],
        elite_encounters: &[
            (EliteEncounter::GremlinNob, 1. / 3.),
            (EliteEncounter::Lagavulin, 1. / 3.),
            (EliteEncounter::ThreeSentries, 1. / 3.),
        ],
        boss_encounters: &[
            BossEncounter::TheGuardian,
            BossEncounter::Hexaghost,
            BossEncounter::SlimeBoss,
        ],
    },
    // ACT 2
    Act {
        map_seed_offset: 200,
        weak_monster_encounter_count: 2,
        weak_monster_encounters_and_probs: &[
            (MonsterEncounter::SphericGuardian, 1. / 5.),
            (MonsterEncounter::Chosen, 1. / 5.),
            (MonsterEncounter::ShelledParasite, 1. / 5.),
            (MonsterEncounter::ThreeByrds, 1. / 5.),
            (MonsterEncounter::TwoThieves, 1. / 5.),
        ],
        strong_monster_encounters_and_probs: &[
            (MonsterEncounter::ChosenAndByrd, 2. / 29.),
            (MonsterEncounter::SentryAndSphericGuardian, 2. / 29.),
            (MonsterEncounter::CultistAndChosen, 3. / 29.),
            (MonsterEncounter::ThreeCultists, 3. / 29.),
            (MonsterEncounter::ShelledParasiteAndFungiBeast, 3. / 29.),
            (MonsterEncounter::Snecko, 4. / 29.),
            (MonsterEncounter::SnakePlant, 6. / 29.),
            (MonsterEncounter::CenturionAndMystic, 6. / 29.),
        ],
        elite_encounters: &[
            (EliteEncounter::GremlinLeader, 1. / 3.),
            (EliteEncounter::Taskmaster, 1. / 3.),
            (EliteEncounter::BookOfStabbing, 1. / 3.),
        ],
        boss_encounters: &[
            BossEncounter::BronzeAutomaton,
            BossEncounter::TheCollector,
            BossEncounter::TheChamp,
        ],
    },
    // ACT 3
    Act {
        map_seed_offset: 600,
        weak_monster_encounter_count: 2,
        weak_monster_encounters_and_probs: &[
            (MonsterEncounter::ThreeDarklings, 1. / 3.),
            (MonsterEncounter::OrbWalker, 1. / 3.),
            (MonsterEncounter::ThreeShapes, 1. / 3.),
        ],
        strong_monster_encounters_and_probs: &[
            (MonsterEncounter::SpireGrowth, 1. / 8.),
            (MonsterEncounter::Transient, 1. / 8.),
            (MonsterEncounter::FourShapes, 1. / 8.),
            (MonsterEncounter::Maw, 1. / 8.),
            (MonsterEncounter::SphericGuardianAndTwoShapes, 1. / 8.),
            (MonsterEncounter::JawWormHorde, 1. / 8.),
            (MonsterEncounter::ThreeDarklings, 1. / 8.),
            (MonsterEncounter::WrithingMass, 1. / 8.),
        ],
        elite_encounters: &[
            (EliteEncounter::GiantHead, 1. / 3.),
            (EliteEncounter::Nemesis, 1. / 3.),
            (EliteEncounter::Reptomancer, 1. / 3.),
        ],
        boss_encounters: &[
            BossEncounter::AwakenedOne,
            BossEncounter::TimeEater,
            BossEncounter::DonuAndDeca,
        ],
    },
    // ACT 4
    Act {
        map_seed_offset: 1200,
        weak_monster_encounter_count: 0,
        weak_monster_encounters_and_probs: &[],
        strong_monster_encounters_and_probs: &[],
        elite_encounters: &[(EliteEncounter::SpireShieldAndSpireSpear, 1.)],
        boss_encounters: &[BossEncounter::CorruptHeart],
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
