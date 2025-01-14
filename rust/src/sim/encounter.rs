use std::collections::VecDeque;
use std::fmt;

use anyhow::Error;
use once_cell::sync::Lazy;

use crate::data::{Card, Encounter, EnemyType, Intent};
use crate::rng::{Seed, StsRandom};

use super::action::{Action, Debuff};
use super::player::Player;

pub struct EncounterSimulator<'a> {
    encounter: Encounter,
    misc_rng: &'a mut StsRandom,
    hp_rng: StsRandom,
    ai_rng: StsRandom,
    player: &'a mut Player,
}

fn not_thrice(action: &'static Action, past_actions: &VecDeque<&'static Action>) -> bool {
    past_actions.len() < 2 || past_actions.iter().take(2).any(|&pa| pa != action)
}

fn not_twice(action: &'static Action, past_actions: &VecDeque<&'static Action>) -> bool {
    past_actions.iter().last().map_or(true, |pa| *pa != action)
}

trait Enemy: fmt::Debug {
    fn enemy_type(&self) -> EnemyType;
    fn health(&self) -> (u32, u32);
    fn intent(&self) -> Intent;
    fn act(&mut self, ai_rng: &mut StsRandom, player: &mut Player) -> Result<(), Error>;
}

#[derive(Debug)]
struct AcidSlimeM {
    hp: u32,
    hp_max: u32,
    past_actions: VecDeque<&'static Action>,
    next_action: &'static Action,
}

static ACID_SLIME_M_CORROSIVE_SPIT: Lazy<Action> = Lazy::new(|| {
    Action::deal_damage(11, 1)
        .then()
        .add_to_discard_pile(&[Card::Slimed, Card::Slimed])
});
static ACID_SLIME_M_LICK: Lazy<Action> = Lazy::new(|| Action::inflict(Debuff::Frail, 1));
static ACID_SLIME_M_TACKLE: Lazy<Action> = Lazy::new(|| Action::deal_damage(16, 1));

impl AcidSlimeM {
    pub fn new_boxed(hp_rng: &mut StsRandom, ai_rng: &mut StsRandom) -> Box<Self> {
        let hp = hp_rng.gen_range(28..=32);
        let hp_max = hp;
        let past_actions = VecDeque::with_capacity(2);
        let next_action = Self::next_action(ai_rng.gen_range(0..100), ai_rng, &past_actions);
        Box::new(Self {
            hp,
            hp_max,
            past_actions,
            next_action,
        })
    }

    #[allow(clippy::explicit_auto_deref)]
    pub fn next_action(
        roll: u8,
        ai_rng: &mut StsRandom,
        pa: &VecDeque<&'static Action>,
    ) -> &'static Action {
        match roll {
            0..40 if not_thrice(&ACID_SLIME_M_CORROSIVE_SPIT, pa) => &ACID_SLIME_M_CORROSIVE_SPIT,
            0..40 => {
                if ai_rng.next_bool() {
                    &ACID_SLIME_M_TACKLE
                } else {
                    &ACID_SLIME_M_LICK
                }
            }
            40..80 if not_thrice(&ACID_SLIME_M_TACKLE, pa) => &ACID_SLIME_M_TACKLE,
            40..80 => *ai_rng.weighted_choose(&[
                (&*ACID_SLIME_M_CORROSIVE_SPIT, 0.5),
                (&*ACID_SLIME_M_LICK, 0.5),
            ]),
            _ if not_twice(&ACID_SLIME_M_LICK, pa) => &ACID_SLIME_M_LICK,
            _ => *ai_rng.weighted_choose(&[
                (&*ACID_SLIME_M_CORROSIVE_SPIT, 0.4),
                (&*ACID_SLIME_M_TACKLE, 0.6),
            ]),
        }
    }
}

impl Enemy for AcidSlimeM {
    fn enemy_type(&self) -> EnemyType {
        EnemyType::AcidSlimeM
    }

    fn health(&self) -> (u32, u32) {
        (self.hp, self.hp_max)
    }

    fn intent(&self) -> Intent {
        self.next_action.intent
    }

    fn act(&mut self, ai_rng: &mut StsRandom, player: &mut Player) -> Result<(), Error> {
        if self.past_actions.len() >= 2 {
            self.past_actions.pop_front();
        }
        for effect in self.next_action.effects.iter() {
            player.apply_effect(*effect)?;
        }
        self.past_actions.push_back(self.next_action);
        self.next_action = Self::next_action(ai_rng.gen_range(0..100), ai_rng, &self.past_actions);
        Ok(())
    }
}

#[derive(Debug)]
struct AcidSlimeS {
    hp: u32,
    hp_max: u32,
    next_action: &'static Action,
}

static ACID_SLIME_S_TACKLE: Lazy<Action> = Lazy::new(|| Action::deal_damage(3, 1));
static ACID_SLIME_S_LICK: Lazy<Action> = Lazy::new(|| Action::inflict(Debuff::Weak, 1));

impl AcidSlimeS {
    pub fn new_boxed(hp_rng: &mut StsRandom, ai_rng: &mut StsRandom) -> Box<Self> {
        let hp = hp_rng.gen_range(8..=12);
        let hp_max = hp;
        let d100 = ai_rng.gen_range(0..100);
        let next_action = Self::next_action(d100, ai_rng);
        Box::new(Self {
            hp,
            hp_max,
            next_action,
        })
    }

    fn next_action(roll: u8, ai_rng: &mut StsRandom) -> &'static Action {
        if ai_rng.next_bool() {
            &ACID_SLIME_S_TACKLE
        } else {
            &ACID_SLIME_S_LICK
        }
    }
}

impl Enemy for AcidSlimeS {
    fn enemy_type(&self) -> EnemyType {
        EnemyType::AcidSlimeS
    }

    fn health(&self) -> (u32, u32) {
        (self.hp, self.hp_max)
    }

    fn intent(&self) -> Intent {
        self.next_action.intent
    }

    fn act(&mut self, ai_rng: &mut StsRandom, player: &mut Player) -> Result<(), Error> {
        for effect in self.next_action.effects.iter() {
            player.apply_effect(*effect)?;
        }
        self.next_action = Self::next_action(ai_rng.gen_range(0..100), ai_rng);
        Ok(())
    }
}

#[derive(Debug)]
struct SpikeSlimeM {
    hp: u32,
    hp_max: u32,
    past_actions: VecDeque<&'static Action>,
    next_action: &'static Action,
}

static SPIKE_SLIME_M_FLAME_TACKLE: Lazy<Action> = Lazy::new(|| {
    Action::deal_damage(8, 1)
        .then()
        .add_to_discard_pile(&[Card::Slimed])
});
static SPIKE_SLIME_M_LICK: Lazy<Action> = Lazy::new(|| Action::inflict(Debuff::Frail, 1));

impl SpikeSlimeM {
    pub fn new_boxed(hp_rng: &mut StsRandom, ai_rng: &mut StsRandom) -> Box<Self> {
        let hp = hp_rng.gen_range(28..=32);
        let hp_max = hp;
        let past_actions = VecDeque::with_capacity(2);
        let d100 = ai_rng.gen_range(0..100);
        let next_action = Self::next_action(d100, ai_rng, &past_actions);
        Box::new(Self {
            hp,
            hp_max,
            past_actions,
            next_action,
        })
    }

    pub fn next_action(
        d100: u8,
        ai_rng: &mut StsRandom,
        pa: &VecDeque<&'static Action>,
    ) -> &'static Action {
        println!("Roll for SpikeSlimeM: {}", d100);
        match d100 {
            0..30 if not_thrice(&SPIKE_SLIME_M_FLAME_TACKLE, pa) => &SPIKE_SLIME_M_FLAME_TACKLE,
            0..30 => &SPIKE_SLIME_M_LICK,
            _ if not_thrice(&SPIKE_SLIME_M_LICK, pa) => &SPIKE_SLIME_M_LICK,
            _ => &SPIKE_SLIME_M_FLAME_TACKLE,
        }
    }
}

impl Enemy for SpikeSlimeM {
    fn enemy_type(&self) -> EnemyType {
        EnemyType::SpikeSlimeM
    }

    fn health(&self) -> (u32, u32) {
        (self.hp, self.hp_max)
    }

    fn intent(&self) -> Intent {
        self.next_action.intent
    }

    fn act(&mut self, ai_rng: &mut StsRandom, player: &mut Player) -> Result<(), Error> {
        if self.past_actions.len() >= 2 {
            self.past_actions.pop_front();
        }
        for effect in self.next_action.effects.iter() {
            player.apply_effect(*effect)?;
        }
        self.past_actions.push_back(self.next_action);
        self.next_action = Self::next_action(ai_rng.gen_range(0..100), ai_rng, &self.past_actions);
        Ok(())
    }
}

#[derive(Debug)]
struct SpikeSlimeS {
    hp: u32,
    hp_max: u32,
    next_action: &'static Action,
}

static SPIKE_SLIME_S_TACKLE: Lazy<Action> = Lazy::new(|| Action::deal_damage(5, 1));

impl SpikeSlimeS {
    pub fn new_boxed(hp_rng: &mut StsRandom, ai_rng: &mut StsRandom) -> Box<Self> {
        let hp = hp_rng.gen_range(10..=14);
        let hp_max = hp;
        let past_actions = VecDeque::with_capacity(2);
        let d100 = ai_rng.gen_range(0..100);
        let next_action = Self::next_action(d100, ai_rng, &past_actions);
        Box::new(Self {
            hp,
            hp_max,
            next_action,
        })
    }

    pub fn next_action(
        d100: u8,
        ai_rng: &mut StsRandom,
        pa: &VecDeque<&'static Action>,
    ) -> &'static Action {
        &SPIKE_SLIME_S_TACKLE
    }
}

impl Enemy for SpikeSlimeS {
    fn enemy_type(&self) -> EnemyType {
        EnemyType::SpikeSlimeS
    }

    fn health(&self) -> (u32, u32) {
        (self.hp, self.hp_max)
    }

    fn intent(&self) -> Intent {
        self.next_action.intent
    }

    fn act(&mut self, ai_rng: &mut StsRandom, player: &mut Player) -> Result<(), Error> {
        for effect in self.next_action.effects.iter() {
            player.apply_effect(*effect)?;
        }
        Ok(())
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
        macro_rules! enemy_party {
            ( $( $enemy:ident ),* ) => {{
                let enemies: Vec<Box<dyn Enemy>> = vec![
                    $(
                        $enemy::new_boxed(&mut self.hp_rng, &mut self.ai_rng),
                    )*
                ];
                enemies
            }};
        }
        let mut enemy_party = match self.encounter {
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
                if self.misc_rng.next_bool() {
                    enemy_party!(SpikeSlimeS, AcidSlimeM)
                } else {
                    enemy_party!(AcidSlimeS, SpikeSlimeM)
                }
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
        };
        println!(
            "Enemy party: {:?}, ai_rng: {}",
            enemy_party,
            self.ai_rng.get_counter()
        );
        loop {
            let enemy_party_view: Vec<(EnemyType, u32, u32, Intent)> = enemy_party
                .iter()
                .map(|e| (e.enemy_type(), e.health().0, e.health().1, e.intent()))
                .collect();
            self.player.send_enemy_party(enemy_party_view)?;

            // TODO: collect player's actions

            for enemy in enemy_party.iter_mut() {
                enemy.act(&mut self.ai_rng, self.player)?;
            }
        }
        Ok(())
    }
}
