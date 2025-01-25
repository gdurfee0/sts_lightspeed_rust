use crate::data::{Encounter, Enemy};
use crate::systems::rng::{Seed, StsRandom};

use super::enemy_in_combat::EnemyInCombat;

pub struct EnemyPartyGenerator<'a> {
    encounter: Encounter,
    hp_rng: StsRandom, // TODO: Should this persist past the 1-shot party generation?
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

    pub fn generate(&mut self, enemy_party: &mut [Option<EnemyInCombat>; 5]) {
        macro_rules! enemy_party {
            ( $( $enemy:ident ),* ) => {{
                let mut iter = enemy_party.iter_mut();
                $(
                    if let Some(slot) = iter.next() {
                        *slot = Some(
                            EnemyInCombat::new(Enemy::$enemy, &mut self.hp_rng, self.ai_rng)
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
            Encounter::ExordiumWildlife => {
                // This must have been one of their earlier ideas for the game, as it's implemented
                // in a more wasteful way than the other encounters.
                let fungi_beast =
                    EnemyInCombat::new(Enemy::FungiBeast, &mut self.hp_rng, self.ai_rng);
                let jaw_worm = EnemyInCombat::new(Enemy::JawWorm, &mut self.hp_rng, self.ai_rng);
                let choice = self.misc_rng.gen_range(0..=1);
                enemy_party[0] = Some(if choice == 0 { fungi_beast } else { jaw_worm });
                let louse = EnemyInCombat::new(
                    if self.misc_rng.next_bool() {
                        Enemy::RedLouse
                    } else {
                        Enemy::GreenLouse
                    },
                    &mut self.hp_rng,
                    self.ai_rng,
                );
                let spike_slime_m =
                    EnemyInCombat::new(Enemy::SpikeSlimeM, &mut self.hp_rng, self.ai_rng);
                let acid_slime_m =
                    EnemyInCombat::new(Enemy::AcidSlimeM, &mut self.hp_rng, self.ai_rng);
                let choice = self.misc_rng.gen_range(0..=2);
                enemy_party[1] = Some(match choice {
                    0 => louse,
                    1 => spike_slime_m,
                    2 => acid_slime_m,
                    _ => unreachable!(),
                });
            }
            Encounter::FourShapes => todo!("{:?}", self.encounter),
            Encounter::GiantHead => todo!("{:?}", self.encounter),
            Encounter::GremlinGang => todo!("{:?}", self.encounter),
            Encounter::GremlinLeader => todo!("{:?}", self.encounter),
            Encounter::GremlinNob => todo!("{:?}", self.encounter),
            Encounter::Hexaghost => todo!("{:?}", self.encounter),
            Encounter::JawWorm => enemy_party!(JawWorm),
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
