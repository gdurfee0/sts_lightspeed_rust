use anyhow::Error;

use crate::data::{Encounter, EnemyTemplate, EnemyType};
use crate::rng::{Seed, StsRandom};

use super::player::Player;

pub struct EncounterSimulator<'a> {
    encounter: Encounter,
    misc_sts_random: &'a mut StsRandom,
    enemy_hp_sts_random: StsRandom,
    player: &'a mut Player,
}

impl<'a> EncounterSimulator<'a> {
    pub fn new(
        seed_for_floor: &Seed,
        encounter: Encounter,
        misc_sts_random: &'a mut StsRandom,
        player: &'a mut Player,
    ) -> Self {
        let enemy_hp_sts_random = StsRandom::from(seed_for_floor);
        Self {
            encounter,
            misc_sts_random,
            enemy_hp_sts_random,
            player,
        }
    }

    pub fn run(mut self) -> Result<(), Error> {
        println!(
            "[EncounterSimulator] Running encounter: {:?}",
            self.encounter
        );
        match self.encounter {
            Encounter::AwakenedOne => todo!(),
            Encounter::BlueSlaver => todo!(),
            Encounter::BookOfStabbing => todo!(),
            Encounter::BronzeAutomaton => todo!(),
            Encounter::CenturionAndMystic => todo!(),
            Encounter::Chosen => todo!(),
            Encounter::ChosenAndByrd => todo!(),
            Encounter::CorruptHeart => todo!(),
            Encounter::Cultist => todo!(),
            Encounter::CultistAndChosen => todo!(),
            Encounter::DonuAndDeca => todo!(),
            Encounter::ExordiumThugs => todo!(),
            Encounter::ExordiumWildlife => todo!(),
            Encounter::FourShapes => todo!(),
            Encounter::GiantHead => todo!(),
            Encounter::GremlinGang => todo!(),
            Encounter::GremlinLeader => todo!(),
            Encounter::GremlinNob => todo!(),
            Encounter::Hexaghost => todo!(),
            Encounter::JawWorm => todo!(),
            Encounter::JawWormHorde => todo!(),
            Encounter::Lagavulin => todo!(),
            Encounter::LargeSlime => todo!(),
            Encounter::Looter => todo!(),
            Encounter::LotsOfSlimes => todo!(),
            Encounter::Maw => todo!(),
            Encounter::Nemesis => todo!(),
            Encounter::OrbWalker => todo!(),
            Encounter::RedSlaver => todo!(),
            Encounter::Reptomancer => todo!(),
            Encounter::SentryAndSphericGuardian => todo!(),
            Encounter::ShelledParasite => todo!(),
            Encounter::ShelledParasiteAndFungiBeast => todo!(),
            Encounter::SlimeBoss => todo!(),
            Encounter::SmallSlimes => {
                let (e1, e2) = if self.misc_sts_random.next_bool() {
                    (
                        EnemyTemplate::from(EnemyType::SpikeSlimeS),
                        EnemyTemplate::from(EnemyType::AcidSlimeM),
                    )
                } else {
                    (
                        EnemyTemplate::from(EnemyType::AcidSlimeS),
                        EnemyTemplate::from(EnemyType::SpikeSlimeM),
                    )
                };
                println!("Enemy templates: {:?}, {:?}", e1, e2);
                let hp1 = self.enemy_hp_sts_random.gen_range(e1.hp);
                let hp2 = self.enemy_hp_sts_random.gen_range(e2.hp);
                println!("Enemy HPs: {}, {}", hp1, hp2);
            }
            Encounter::SnakePlant => todo!(),
            Encounter::Snecko => todo!(),
            Encounter::SphericGuardian => todo!(),
            Encounter::SphericGuardianAndTwoShapes => todo!(),
            Encounter::SpireGrowth => todo!(),
            Encounter::SpireShieldAndSpireSpear => todo!(),
            Encounter::Taskmaster => todo!(),
            Encounter::TheChamp => todo!(),
            Encounter::TheCollector => todo!(),
            Encounter::TheGuardian => todo!(),
            Encounter::ThreeByrds => todo!(),
            Encounter::ThreeCultists => todo!(),
            Encounter::ThreeDarklings => todo!(),
            Encounter::ThreeLice => todo!(),
            Encounter::ThreeSentries => todo!(),
            Encounter::ThreeShapes => todo!(),
            Encounter::TimeEater => todo!(),
            Encounter::Transient => todo!(),
            Encounter::TwoFungiBeasts => todo!(),
            Encounter::TwoLice => todo!(),
            Encounter::TwoThieves => todo!(),
            Encounter::WrithingMass => todo!(),
        }
        Ok(())
    }
}
