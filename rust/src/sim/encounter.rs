use std::collections::VecDeque;
use std::fmt;

use anyhow::Error;

use crate::data::{Encounter, EnemyType, Intent};
use crate::rng::{Seed, StsRandom};

use super::player::Player;

pub struct EncounterSimulator<'a> {
    encounter: Encounter,
    misc_rng: &'a mut StsRandom,
    hp_rng: StsRandom,
    ai_rng: StsRandom,
    player: &'a mut Player,
}

#[derive(Clone, Copy, Debug)]
pub enum Action {
    CorrosiveSpit,
    FlameTackle,
    Lick,
    Tackle,
}

pub trait Enemy: fmt::Debug {
    fn next_action(&self) -> Action;
    fn health(&self) -> (u32, u32);
}

#[derive(Debug)]
struct AcidSlimeM {
    hp: u32,
    hp_max: u32,
    next_action: Action,
}

impl AcidSlimeM {
    pub fn new_boxed(hp_rng: &mut StsRandom, ai_rng: &mut StsRandom) -> Box<Self> {
        let hp = hp_rng.gen_range(28..=32);
        let hp_max = hp;
        let next_action = match ai_rng.gen_range(0..100) {
            0..30 => Action::CorrosiveSpit,
            30..70 => Action::Tackle,
            _ => Action::Lick,
        };
        Box::new(Self {
            hp,
            hp_max,
            next_action,
        })
    }
}

impl Enemy for AcidSlimeM {
    fn next_action(&self) -> Action {
        self.next_action
    }

    fn health(&self) -> (u32, u32) {
        (self.hp, self.hp_max)
    }
}

#[derive(Debug)]
struct AcidSlimeS {
    hp: u32,
    hp_max: u32,
    next_action: Action,
}

impl AcidSlimeS {
    pub fn new_boxed(hp_rng: &mut StsRandom, ai_rng: &mut StsRandom) -> Box<Self> {
        let hp = hp_rng.gen_range(8..=12);
        let hp_max = hp;
        let next_action = if ai_rng.next_bool() {
            Action::Tackle
        } else {
            Action::Lick
        };
        Box::new(Self {
            hp,
            hp_max,
            next_action,
        })
    }
}

impl Enemy for AcidSlimeS {
    fn next_action(&self) -> Action {
        self.next_action
    }

    fn health(&self) -> (u32, u32) {
        (self.hp, self.hp_max)
    }
}

#[derive(Debug)]
struct SpikeSlimeM {
    hp: u32,
    hp_max: u32,
    next_action: Action,
}

impl SpikeSlimeM {
    pub fn new_boxed(hp_rng: &mut StsRandom, ai_rng: &mut StsRandom) -> Box<Self> {
        let hp = hp_rng.gen_range(28..=32);
        let hp_max = hp;
        let next_action = match ai_rng.gen_range(0..100) {
            0..30 => Action::FlameTackle,
            _ => Action::Lick,
        };
        Box::new(Self {
            hp,
            hp_max,
            next_action,
        })
    }
}

impl Enemy for SpikeSlimeM {
    fn next_action(&self) -> Action {
        self.next_action
    }

    fn health(&self) -> (u32, u32) {
        (self.hp, self.hp_max)
    }
}

#[derive(Debug)]
struct SpikeSlimeS {
    hp: u32,
    hp_max: u32,
    next_action: Action,
}

impl SpikeSlimeS {
    pub fn new_boxed(hp_rng: &mut StsRandom, ai_rng: &mut StsRandom) -> Box<Self> {
        let hp = hp_rng.gen_range(10..=14);
        let hp_max = hp;
        let next_action = Action::Tackle;
        Box::new(Self {
            hp,
            hp_max,
            next_action,
        })
    }
}

impl Enemy for SpikeSlimeS {
    fn next_action(&self) -> Action {
        self.next_action
    }

    fn health(&self) -> (u32, u32) {
        (self.hp, self.hp_max)
    }
}

/*
macro_rules! impl_enemy {
    ($name:ident, $hp:expr) => {
        #[derive(Debug)]
        pub struct $name {
            hp: u32,
            hp_max: u32,
            next_action: Action,
            action_history: VecDeque<Action>,
        }

        impl $name {
            pub fn new(hp_rng: &mut StsRandom) -> Self {
                let hp = hp_rng.gen_range($hp);
                let hp_max = hp;
                let next_action = Action::Attack;
                let action_history = VecDeque::with_capacity(2);
                Self {
                    hp,
                    hp_max,
                    next_action,
                    action_history,
                }
            }
        }

        impl Enemy for $name {
            fn next_action(&self) -> Action {
                self.next_action
            }

            fn health(&self) -> (u32, u32) {
                (self.hp, self.hp_max)
            }
        }
    };
}

impl_enemy!(AcidSlimeM, 28..=32);
impl_enemy!(AcidSlimeS, 8..=12);
impl_enemy!(SpikeSlimeM, 28..=32);
impl_enemy!(SpikeSlimeS, 10..=14);

*/

pub struct EnemyParty {
    enemies: [Option<Box<dyn Enemy>>; 5],
}

impl<'a> EncounterSimulator<'a> {
    pub fn new(
        seed_for_floor: Seed,
        encounter: Encounter,
        misc_rng: &'a mut StsRandom,
        player: &'a mut Player,
    ) -> Self {
        let hp_rng = StsRandom::from(seed_for_floor);
        let ai_rng = StsRandom::from(seed_for_floor);
        Self {
            encounter,
            misc_rng,
            hp_rng,
            ai_rng,
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
                let (e1, e2): (Box<dyn Enemy>, Box<dyn Enemy>) = if self.misc_rng.next_bool() {
                    (
                        SpikeSlimeS::new_boxed(&mut self.hp_rng, &mut self.ai_rng),
                        AcidSlimeM::new_boxed(&mut self.hp_rng, &mut self.ai_rng),
                    )
                } else {
                    (
                        AcidSlimeS::new_boxed(&mut self.hp_rng, &mut self.ai_rng),
                        SpikeSlimeM::new_boxed(&mut self.hp_rng, &mut self.ai_rng),
                    )
                };
                println!("Enemies: {:?}, {:?}", e1, e2);
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
