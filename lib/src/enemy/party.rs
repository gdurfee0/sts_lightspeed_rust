use crate::data::{Encounter, EnemyType};
use crate::rng::{Seed, StsRandom};

use super::state::EnemyState;

pub struct EnemyPartyGenerator<'a> {
    encounter: Encounter,
    hp_rng: StsRandom,
    ai_rng: &'a mut StsRandom,
    misc_rng: &'a mut StsRandom,
}

impl<'a> EnemyPartyGenerator<'a> {
    pub fn new(
        seed_for_floor: Seed,
        encounter: Encounter,
        ai_rng: &'a mut StsRandom,
        misc_rng: &'a mut StsRandom,
    ) -> Self {
        Self {
            encounter,
            hp_rng: StsRandom::from(seed_for_floor),
            ai_rng,
            misc_rng,
        }
    }

    pub fn generate(&mut self, enemy_party: &mut [Option<EnemyState>; 5]) {
        macro_rules! enemy_party {
            ( $( $enemy_type:ident ),* ) => {{
                let mut iter = enemy_party.iter_mut();
                $(
                    if let Some(slot) = iter.next() {
                        *slot = Some(
                            EnemyState::new(EnemyType::$enemy_type, &mut self.hp_rng, self.ai_rng)
                        );
                    }
                )*
            }};
        }
        match self.encounter {
            Encounter::AwakenedOne => enemy_party!(AwakenedOne),
            Encounter::BlueSlaver => todo!("{:?}", self.encounter),
            Encounter::BookOfStabbing => todo!("{:?}", self.encounter),
            Encounter::BronzeAutomaton => todo!("{:?}", self.encounter),
            Encounter::CenturionAndMystic => todo!("{:?}", self.encounter),
            Encounter::Chosen => todo!("{:?}", self.encounter),
            Encounter::ChosenAndByrd => todo!("{:?}", self.encounter),
            Encounter::CorruptHeart => todo!("{:?}", self.encounter),
            Encounter::Cultist => enemy_party!(Cultist),
            Encounter::CultistAndChosen => todo!("{:?}", self.encounter),
            Encounter::DonuAndDeca => todo!("{:?}", self.encounter),
            Encounter::ExordiumThugs => todo!("{:?}", self.encounter),
            Encounter::ExordiumWildlife => todo!("{:?}", self.encounter),
            Encounter::FourShapes => todo!("{:?}", self.encounter),
            Encounter::GiantHead => todo!("{:?}", self.encounter),
            Encounter::GremlinGang => todo!("{:?}", self.encounter),
            Encounter::GremlinLeader => todo!("{:?}", self.encounter),
            Encounter::GremlinNob => todo!("{:?}", self.encounter),
            Encounter::Hexaghost => todo!("{:?}", self.encounter),
            Encounter::JawWorm => todo!("{:?}", self.encounter),
            Encounter::JawWormHorde => todo!("{:?}", self.encounter),
            Encounter::Lagavulin => todo!("{:?}", self.encounter),
            Encounter::LargeSlime => todo!("{:?}", self.encounter),
            Encounter::Looter => todo!("{:?}", self.encounter),
            Encounter::LotsOfSlimes => todo!("{:?}", self.encounter),
            Encounter::Maw => todo!("{:?}", self.encounter),
            Encounter::Nemesis => todo!("{:?}", self.encounter),
            Encounter::OrbWalker => todo!("{:?}", self.encounter),
            Encounter::RedSlaver => todo!("{:?}", self.encounter),
            Encounter::Reptomancer => todo!("{:?}", self.encounter),
            Encounter::SentryAndSphericGuardian => todo!("{:?}", self.encounter),
            Encounter::ShelledParasite => todo!("{:?}", self.encounter),
            Encounter::ShelledParasiteAndFungiBeast => todo!("{:?}", self.encounter),
            Encounter::SlimeBoss => todo!("{:?}", self.encounter),
            Encounter::SmallSlimes => {
                if self.misc_rng.next_bool() {
                    enemy_party!(SpikeSlimeS, AcidSlimeM)
                } else {
                    enemy_party!(AcidSlimeS, SpikeSlimeM)
                }
            }
            Encounter::SnakePlant => todo!("{:?}", self.encounter),
            Encounter::Snecko => todo!("{:?}", self.encounter),
            Encounter::SphericGuardian => todo!("{:?}", self.encounter),
            Encounter::SphericGuardianAndTwoShapes => todo!("{:?}", self.encounter),
            Encounter::SpireGrowth => todo!("{:?}", self.encounter),
            Encounter::SpireShieldAndSpireSpear => todo!("{:?}", self.encounter),
            Encounter::Taskmaster => todo!("{:?}", self.encounter),
            Encounter::TheChamp => todo!("{:?}", self.encounter),
            Encounter::TheCollector => todo!("{:?}", self.encounter),
            Encounter::TheGuardian => todo!("{:?}", self.encounter),
            Encounter::ThreeByrds => todo!("{:?}", self.encounter),
            Encounter::ThreeCultists => todo!("{:?}", self.encounter),
            Encounter::ThreeDarklings => todo!("{:?}", self.encounter),
            Encounter::ThreeLouses => todo!("{:?}", self.encounter),
            Encounter::ThreeSentries => todo!("{:?}", self.encounter),
            Encounter::ThreeShapes => todo!("{:?}", self.encounter),
            Encounter::TimeEater => todo!("{:?}", self.encounter),
            Encounter::Transient => todo!("{:?}", self.encounter),
            Encounter::TwoFungiBeasts => todo!("{:?}", self.encounter),
            Encounter::TwoLouses => todo!("{:?}", self.encounter),
            Encounter::TwoThieves => todo!("{:?}", self.encounter),
            Encounter::WrithingMass => todo!("{:?}", self.encounter),
        }
    }
}
