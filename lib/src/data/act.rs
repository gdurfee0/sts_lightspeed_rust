use anyhow::anyhow;

use super::encounter::Encounter;
use super::event::Event;

#[derive(Debug, PartialEq)]
pub struct Act {
    pub number: usize,

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

    pub event_pool: &'static [Event],
    pub shrine_pool: &'static [Event],
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

// Special thanks to gamerpuppy for the probability distributions listed below.

static ACTS: &[Act] = &[
    // ACT 1
    Act {
        number: 1,
        map_seed_offset: 1,
        weak_monster_encounter_count: 3,
        weak_monster_encounter_pool: &[
            (Encounter::Cultist, 1. / 4.),
            (Encounter::JawWorm, 1. / 4.),
            (Encounter::TwoLouses, 1. / 4.),
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
            (Encounter::ThreeLouses, 2. / 16.),
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
        event_pool: &[
            Event::BigFish,
            Event::TheCleric,
            Event::DeadAdventurer,
            Event::GoldenIdol,
            Event::WingStatue,
            Event::WorldOfGoop,
            Event::TheSsssserpent,
            Event::LivingWall,
            Event::HypnotizingColoredMushrooms,
            Event::ScrapOoze,
            Event::ShiningLight,
        ],
        shrine_pool: &[
            Event::MatchAndKeep,
            Event::GoldenShrine,
            Event::Transmogrifier,
            Event::Purifier,
            Event::UpgradeShrine,
            Event::WheelOfChange,
        ],
    },
    // ACT 2
    Act {
        number: 2,
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
        event_pool: &[
            Event::PleadingVagrant,
            Event::AncientWriting,
            Event::OldBeggar,
            Event::TheColosseum,
            Event::CursedTome,
            Event::Augmenter,
            Event::ForgottenAltar,
            Event::CouncilOfGhosts,
            Event::MaskedBandits,
            Event::TheNest,
            Event::TheLibrary,
            Event::TheMausoleum,
            Event::Vampires,
        ],
        shrine_pool: &[
            Event::MatchAndKeep,
            Event::WheelOfChange,
            Event::GoldenShrine,
            Event::Transmogrifier,
            Event::Purifier,
            Event::UpgradeShrine,
        ],
    },
    // ACT 3
    Act {
        number: 3,
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
        event_pool: &[
            Event::Falling,
            Event::MindBloom,
            Event::TheMoaiHead,
            Event::MysteriousSphere,
            Event::SensoryStone,
            Event::TombOfLordRedMask,
            Event::WindingHalls,
        ],
        shrine_pool: &[
            Event::MatchAndKeep,
            Event::WheelOfChange,
            Event::GoldenShrine,
            Event::Transmogrifier,
            Event::Purifier,
            Event::UpgradeShrine,
        ],
    },
    // ACT 4
    Act {
        number: 4,
        map_seed_offset: 1200,
        weak_monster_encounter_count: 0,
        weak_monster_encounter_pool: &[],
        strong_monster_encounter_pool: &[],
        elite_encounter_pool: &[(Encounter::SpireShieldAndSpireSpear, 1.)],
        boss_encounter_pool: &[Encounter::CorruptHeart],
        event_pool: &[],
        shrine_pool: &[],
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
