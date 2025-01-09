use anyhow::anyhow;

use crate::monster::Monster;

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
    pub act: Act,
    pub map_seed_offset: u64,
    pub weak_monsters_and_probs: &'static [(Monster, f32)],
    pub strong_monsters_and_probs: &'static [(Monster, f32)],
}

pub static ACT_DETAILS: &[ActDetails] = &[
    ActDetails {
        act: Act(1),
        map_seed_offset: 1,
        weak_monsters_and_probs: &[
            (Monster::Cultist, 1. / 4.),
            (Monster::JawWorm, 1. / 4.),
            (Monster::TwoLice, 1. / 4.),
            (Monster::SmallSlimes, 1. / 4.),
        ],
        strong_monsters_and_probs: &[
            (Monster::GremlinGang, 1. / 16.),
            (Monster::LargeSlime, 2. / 16.),
            (Monster::LotsOfSlimes, 1. / 16.),
            (Monster::RedSlaver, 1. / 16.),
            (Monster::ThreeLice, 2. / 16.),
            (Monster::TwoFungiBeasts, 2. / 16.),
            (Monster::ExordiumThugs, 1.5 / 16.),
            (Monster::ExordiumWildlife, 1.5 / 16.),
            (Monster::BlueSlaver, 2. / 16.),
            (Monster::Looter, 2. / 16.),
        ],
    },
    ActDetails {
        act: Act(2),
        map_seed_offset: 200,
        weak_monsters_and_probs: &[
            (Monster::SphericGuardian, 1. / 5.),
            (Monster::Chosen, 1. / 5.),
            (Monster::ShelledParasite, 1. / 5.),
            (Monster::ThreeByrds, 1. / 5.),
            (Monster::TwoThieves, 1. / 5.),
        ],
        strong_monsters_and_probs: &[
            (Monster::ChosenAndByrd, 2. / 29.),
            (Monster::CultistAndChosen, 3. / 29.),
            (Monster::SentryAndSphericGuardian, 2. / 29.),
            (Monster::SnakePlant, 6. / 29.),
            (Monster::Snecko, 4. / 29.),
            (Monster::CenturionAndMystic, 6. / 29.),
            (Monster::ThreeCultists, 3. / 29.),
            (Monster::ShelledParasiteAndFungiBeast, 3. / 29.),
        ],
    },
    ActDetails {
        act: Act(3),
        map_seed_offset: 600,
        weak_monsters_and_probs: &[
            (Monster::ThreeDarklings, 1. / 3.),
            (Monster::OrbWalker, 1. / 3.),
            (Monster::ThreeShapes, 1. / 3.),
        ],
        strong_monsters_and_probs: &[
            (Monster::FourShapes, 1. / 8.),
            (Monster::Maw, 1. / 8.),
            (Monster::SphericGuardianAndTwoShapes, 1. / 8.),
            (Monster::ThreeDarklings, 1. / 8.),
            (Monster::WrithingMass, 1. / 8.),
            (Monster::JawWormHorde, 1. / 8.),
            (Monster::SpireGrowth, 1. / 8.),
            (Monster::Transient, 1. / 8.),
        ],
    },
    ActDetails {
        act: Act(4),
        map_seed_offset: 1200,
        weak_monsters_and_probs: &[],
        strong_monsters_and_probs: &[],
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
