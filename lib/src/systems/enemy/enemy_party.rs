use crate::data::{Encounter, Enemy};
use crate::systems::rng::{Seed, StsRandom};

use super::enemy_characteristics::gen_characteristics;
use super::enemy_state::EnemyState;

pub struct EnemyParty(pub [Option<EnemyState>; 5]);

impl EnemyParty {
    pub fn generate(
        seed_for_floor: Seed,
        encounter: Encounter,
        ai_rng: &mut StsRandom,
        misc_rng: &mut StsRandom,
    ) -> EnemyParty {
        let mut hp_rng = StsRandom::from(seed_for_floor);
        let mut enemy_party: [Option<EnemyState>; 5] = [None, None, None, None, None];
        macro_rules! enemy_party {
            ( $( $enemy:ident ),* ) => {{
                let mut iter = enemy_party.iter_mut();
                $(
                    if let Some(slot) = iter.next() {
                        let characteristics = gen_characteristics(Enemy::$enemy, &mut hp_rng);
                        *slot = Some(EnemyState::new(Enemy::$enemy, characteristics, ai_rng));
                    }
                )*
                EnemyParty(enemy_party)
            }};
        }
        match encounter {
            Encounter::AwakenedOne => enemy_party!(AwakenedOne),
            Encounter::BlueSlaver => todo!("{:?}", encounter),
            Encounter::BookOfStabbing => todo!("{:?}", encounter),
            Encounter::BronzeAutomaton => todo!("{:?}", encounter),
            Encounter::CenturionAndMystic => todo!("{:?}", encounter),
            Encounter::Chosen => todo!("{:?}", encounter),
            Encounter::ChosenAndByrd => todo!("{:?}", encounter),
            Encounter::CorruptHeart => todo!("{:?}", encounter),
            Encounter::Cultist => enemy_party!(Cultist),
            Encounter::CultistAndChosen => todo!("{:?}", encounter),
            Encounter::DonuAndDeca => todo!("{:?}", encounter),
            Encounter::ExordiumThugs => todo!("{:?}", encounter),
            Encounter::ExordiumWildlife => {
                // This must have been one of their earlier ideas for the game, as it's implemented
                // in a more wasteful way than the other encounters.
                let fungi_beast = gen_characteristics(Enemy::FungiBeast, &mut hp_rng);
                let jaw_worm = gen_characteristics(Enemy::JawWorm, &mut hp_rng);
                let choice = misc_rng.gen_range(0..=1);

                enemy_party[0] = if choice == 0 {
                    Some(EnemyState::new(Enemy::FungiBeast, fungi_beast, ai_rng))
                } else {
                    Some(EnemyState::new(Enemy::JawWorm, jaw_worm, ai_rng))
                };
                let louse = if misc_rng.next_bool() {
                    (
                        Enemy::RedLouse,
                        gen_characteristics(Enemy::RedLouse, &mut hp_rng),
                    )
                } else {
                    (
                        Enemy::GreenLouse,
                        gen_characteristics(Enemy::GreenLouse, &mut hp_rng),
                    )
                };
                let spike_slime_m = gen_characteristics(Enemy::SpikeSlimeM, &mut hp_rng);
                let acid_slime_m = gen_characteristics(Enemy::AcidSlimeM, &mut hp_rng);
                let choice = misc_rng.gen_range(0..=2);
                enemy_party[1] = Some(match choice {
                    0 => EnemyState::new(louse.0, louse.1, ai_rng),
                    1 => EnemyState::new(Enemy::SpikeSlimeM, spike_slime_m, ai_rng),
                    2 => EnemyState::new(Enemy::AcidSlimeM, acid_slime_m, ai_rng),
                    _ => unreachable!(),
                });
                EnemyParty(enemy_party)
            }
            Encounter::FourShapes => todo!("{:?}", encounter),
            Encounter::GiantHead => todo!("{:?}", encounter),
            Encounter::GremlinGang => todo!("{:?}", encounter),
            Encounter::GremlinLeader => todo!("{:?}", encounter),
            Encounter::GremlinNob => enemy_party!(GremlinNob),
            Encounter::Hexaghost => todo!("{:?}", encounter),
            Encounter::JawWorm => enemy_party!(JawWorm),
            Encounter::JawWormHorde => todo!("{:?}", encounter),
            Encounter::Lagavulin => todo!("{:?}", encounter),
            Encounter::LargeSlime => todo!("{:?}", encounter),
            Encounter::Looter => todo!("{:?}", encounter),
            Encounter::LotsOfSlimes => todo!("{:?}", encounter),
            Encounter::Maw => todo!("{:?}", encounter),
            Encounter::Nemesis => todo!("{:?}", encounter),
            Encounter::OrbWalker => todo!("{:?}", encounter),
            Encounter::RedSlaver => todo!("{:?}", encounter),
            Encounter::Reptomancer => todo!("{:?}", encounter),
            Encounter::SentryAndSphericGuardian => todo!("{:?}", encounter),
            Encounter::ShelledParasite => todo!("{:?}", encounter),
            Encounter::ShelledParasiteAndFungiBeast => todo!("{:?}", encounter),
            Encounter::SlimeBoss => todo!("{:?}", encounter),
            Encounter::SmallSlimes => {
                if misc_rng.next_bool() {
                    enemy_party!(SpikeSlimeS, AcidSlimeM)
                } else {
                    enemy_party!(AcidSlimeS, SpikeSlimeM)
                }
            }
            Encounter::SnakePlant => todo!("{:?}", encounter),
            Encounter::Snecko => todo!("{:?}", encounter),
            Encounter::SphericGuardian => todo!("{:?}", encounter),
            Encounter::SphericGuardianAndTwoShapes => todo!("{:?}", encounter),
            Encounter::SpireGrowth => todo!("{:?}", encounter),
            Encounter::SpireShieldAndSpireSpear => todo!("{:?}", encounter),
            Encounter::Taskmaster => todo!("{:?}", encounter),
            Encounter::TheChamp => todo!("{:?}", encounter),
            Encounter::TheCollector => todo!("{:?}", encounter),
            Encounter::TheGuardian => todo!("{:?}", encounter),
            Encounter::ThreeByrds => todo!("{:?}", encounter),
            Encounter::ThreeCultists => todo!("{:?}", encounter),
            Encounter::ThreeDarklings => todo!("{:?}", encounter),
            Encounter::ThreeLouses => todo!("{:?}", encounter),
            Encounter::ThreeSentries => todo!("{:?}", encounter),
            Encounter::ThreeShapes => todo!("{:?}", encounter),
            Encounter::TimeEater => todo!("{:?}", encounter),
            Encounter::Transient => todo!("{:?}", encounter),
            Encounter::TwoFungiBeasts => todo!("{:?}", encounter),
            Encounter::TwoLouses => todo!("{:?}", encounter),
            Encounter::TwoThieves => todo!("{:?}", encounter),
            Encounter::WrithingMass => todo!("{:?}", encounter),
        }
    }
}
