use anyhow::anyhow;

use crate::encounter::{BossEncounter, EliteEncounter, MonsterEncounter};

#[derive(Debug, PartialEq)]
pub struct Act(pub u8);

impl Act {
    pub fn get_details(&self) -> &'static ActDetails {
        &ACT_DETAILS[(self.0 - 1) as usize]
    }
}

impl TryFrom<&str> for Act {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let act = value.parse()?;
        if !(1..=4).contains(&act) {
            Err(anyhow!("Act must be between 1 and 4"))
        } else {
            Ok(Act(act))
        }
    }
}

pub struct ActDetails {
    pub map_seed_offset: u64,
    pub weak_monster_encounter_count: usize,
    pub weak_monster_encounters_and_probs: &'static [(MonsterEncounter, f32)],
    pub strong_monster_encounters_and_probs: &'static [(MonsterEncounter, f32)],
    pub elite_encounters: &'static [EliteEncounter],
    pub boss_encounters: &'static [BossEncounter],
}

pub static ACT_DETAILS: &[ActDetails] = &[
    // ACT 1
    ActDetails {
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
            EliteEncounter::GremlinNob,
            EliteEncounter::Lagavulin,
            EliteEncounter::ThreeSentries,
        ],
        boss_encounters: &[
            BossEncounter::TheGuardian,
            BossEncounter::Hexaghost,
            BossEncounter::SlimeBoss,
        ],
    },
    // ACT 2
    ActDetails {
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
            EliteEncounter::BookOfStabbing,
            EliteEncounter::GremlinLeader,
            EliteEncounter::Taskmaster,
        ],
        boss_encounters: &[
            BossEncounter::BronzeAutomaton,
            BossEncounter::TheCollector,
            BossEncounter::TheChamp,
        ],
    },
    // ACT 3
    ActDetails {
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
            EliteEncounter::GiantHead,
            EliteEncounter::Nemesis,
            EliteEncounter::Reptomancer,
        ],
        boss_encounters: &[
            BossEncounter::AwakenedOne,
            BossEncounter::TimeEater,
            BossEncounter::DonuAndDeca,
        ],
    },
    // ACT 4
    ActDetails {
        map_seed_offset: 1200,
        weak_monster_encounter_count: 0,
        weak_monster_encounters_and_probs: &[],
        strong_monster_encounters_and_probs: &[],
        elite_encounters: &[EliteEncounter::SpireShieldAndSpireSpear],
        boss_encounters: &[BossEncounter::CorruptHeart],
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ascension_try_from() {
        assert_eq!(Act::try_from("1").unwrap(), Act(1));
        assert_eq!(Act::try_from("4").unwrap(), Act(4));
        assert!(Act::try_from("0").is_err());
        assert!(Act::try_from("5").is_err());
        assert!(Act::try_from("").is_err());
        assert!(Act::try_from("-1").is_err());
        assert!(Act::try_from("ZZZ").is_err());
    }
}
