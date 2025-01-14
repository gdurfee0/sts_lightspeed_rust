use std::collections::VecDeque;
use std::fmt;

use anyhow::Error;
use once_cell::sync::Lazy;

use crate::data::{Card, Encounter, EnemyType, Intent};
use crate::rng::{Seed, StsRandom};

use super::action::{Action, Debuff};
use super::player::{Player, PlayerAction};

pub struct EncounterSimulator<'a> {
    encounter: Encounter,
    seed_for_floor: Seed,
    misc_rng: &'a mut StsRandom,
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
    fn act(&mut self, ai_rng: &mut StsRandom) -> &'static Action;
}

#[derive(Debug)]
struct AcidSlimeM {
    hp: u32,
    hp_max: u32,
    past_actions: VecDeque<&'static Action>,
    next_action: &'static Action,
}

static ACID_SLIME_M_CORROSIVE_SPIT: Lazy<Action> = Lazy::new(|| {
    Action::deal_damage(7, 1)
        .then()
        .add_to_discard_pile(&[Card::Slimed])
});
static ACID_SLIME_M_LICK: Lazy<Action> = Lazy::new(|| Action::inflict(Debuff::Weak, 1));
static ACID_SLIME_M_TACKLE: Lazy<Action> = Lazy::new(|| Action::deal_damage(10, 1));

impl AcidSlimeM {
    pub fn new_boxed(hp_rng: &mut StsRandom, ai_rng: &mut StsRandom) -> Box<Self> {
        let hp = hp_rng.gen_range(28..=32);
        let hp_max = hp;
        let past_actions = VecDeque::with_capacity(2);
        let next_action = Self::next_action(ai_rng, &past_actions);
        Box::new(Self {
            hp,
            hp_max,
            past_actions,
            next_action,
        })
    }

    fn next_action(ai_rng: &mut StsRandom, pa: &VecDeque<&'static Action>) -> &'static Action {
        let d100 = ai_rng.gen_range(0..100);
        let (debug_str, result) = match d100 {
            0..30 if not_thrice(&ACID_SLIME_M_CORROSIVE_SPIT, pa) => {
                ("ACID_SLIME_M_CORROSIVE_SPIT", &ACID_SLIME_M_CORROSIVE_SPIT)
            }
            0..30 => {
                if ai_rng.next_bool() {
                    ("ACID_SLIME_M_TACKLE(CS)", &ACID_SLIME_M_TACKLE)
                } else {
                    ("ACID_SLIME_M_LICK(CS)", &ACID_SLIME_M_LICK)
                }
            }
            30..70 if not_twice(&ACID_SLIME_M_TACKLE, pa) => {
                ("ACID_SLIME_M_TACKLE", &ACID_SLIME_M_TACKLE)
            }
            30..70 => *ai_rng.weighted_choose(&[
                (
                    (
                        "ACID_SLIME_M_CORROSIVE_SPIT(T)",
                        &ACID_SLIME_M_CORROSIVE_SPIT,
                    ),
                    0.5,
                ),
                (("ACID_SLIME_M_CORROSIVE_LICK(T)", &ACID_SLIME_M_LICK), 0.5),
            ]),
            _ if not_thrice(&ACID_SLIME_M_LICK, pa) => ("ACID_SLIME_M_LICK", &ACID_SLIME_M_LICK),
            _ => *ai_rng.weighted_choose(&[
                (
                    (
                        "ACID_SLIME_M_CORROSIVE_SPIT(L)",
                        &ACID_SLIME_M_CORROSIVE_SPIT,
                    ),
                    0.4,
                ),
                (("ACID_SLIME_M_TACKLE(L)", &ACID_SLIME_M_TACKLE), 0.6),
            ]),
        };
        println!("{}: {}: {}", debug_str, ai_rng.get_counter(), d100);
        result
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

    fn act(&mut self, ai_rng: &mut StsRandom) -> &'static Action {
        if self.past_actions.len() >= 2 {
            self.past_actions.pop_front();
        }
        self.past_actions.push_back(self.next_action);
        let result = self.next_action;
        self.next_action = Self::next_action(ai_rng, &self.past_actions);
        result
    }
}

#[derive(Debug)]
struct AcidSlimeS {
    hp: u32,
    hp_max: u32,
    next_action: &'static Action,
    past_actions: VecDeque<&'static Action>,
}

static ACID_SLIME_S_TACKLE: Lazy<Action> = Lazy::new(|| Action::deal_damage(3, 1));
static ACID_SLIME_S_LICK: Lazy<Action> = Lazy::new(|| Action::inflict(Debuff::Weak, 1));

impl AcidSlimeS {
    pub fn new_boxed(hp_rng: &mut StsRandom, ai_rng: &mut StsRandom) -> Box<Self> {
        let hp = hp_rng.gen_range(8..=12);
        let hp_max = hp;
        let _ = ai_rng.gen_range(0..100);
        let past_actions = VecDeque::with_capacity(2);
        let next_action = Self::next_action(ai_rng, &past_actions);
        Box::new(Self {
            hp,
            hp_max,
            past_actions,
            next_action,
        })
    }

    fn next_action(
        ai_rng: &mut StsRandom,
        past_actions: &VecDeque<&'static Action>,
    ) -> &'static Action {
        // Does not seem to burn an extra random number.
        let (debug_str, result) = if past_actions.is_empty() {
            if ai_rng.next_bool() {
                ("ACID_SLIME_S_TACKLE", &ACID_SLIME_S_TACKLE)
            } else {
                ("ACID_SLIME_S_LICK", &ACID_SLIME_S_LICK)
            }
        } else if not_twice(&ACID_SLIME_S_TACKLE, past_actions) {
            ("ACID_SLIME_S_TACKLE", &ACID_SLIME_S_TACKLE)
        } else {
            ("ACID_SLIME_S_LICK", &ACID_SLIME_S_LICK)
        };
        println!("{}: {}", debug_str, ai_rng.get_counter());
        result
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

    fn act(&mut self, ai_rng: &mut StsRandom) -> &'static Action {
        if self.past_actions.len() >= 2 {
            self.past_actions.pop_front();
        }
        self.past_actions.push_back(self.next_action);
        let result = self.next_action;
        self.next_action = Self::next_action(ai_rng, &self.past_actions);
        result
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
        let next_action = Self::next_action(ai_rng, &past_actions);
        Box::new(Self {
            hp,
            hp_max,
            past_actions,
            next_action,
        })
    }

    fn next_action(ai_rng: &mut StsRandom, pa: &VecDeque<&'static Action>) -> &'static Action {
        let d100 = ai_rng.gen_range(0..100);
        let (debug_str, result) = match d100 {
            0..30 if not_thrice(&SPIKE_SLIME_M_FLAME_TACKLE, pa) => {
                ("SPIKE_SLIME_M_FLAME_TACKLE", &SPIKE_SLIME_M_FLAME_TACKLE)
            }
            0..30 => ("SPIKE_SLIME_M_LICK(T)", &SPIKE_SLIME_M_LICK),
            _ if not_thrice(&SPIKE_SLIME_M_LICK, pa) => ("SPIKE_SLIME_M_LICK", &SPIKE_SLIME_M_LICK),
            _ => ("SPIKE_SLIME_M_FLAME_TACKLE(L)", &SPIKE_SLIME_M_FLAME_TACKLE),
        };
        println!("{}: {}: {}", debug_str, ai_rng.get_counter(), d100);
        result
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

    fn act(&mut self, ai_rng: &mut StsRandom) -> &'static Action {
        if self.past_actions.len() >= 2 {
            self.past_actions.pop_front();
        }
        self.past_actions.push_back(self.next_action);
        let result = self.next_action;
        self.next_action = Self::next_action(ai_rng, &self.past_actions);
        result
    }
}

#[derive(Debug)]
struct SpikeSlimeS {
    hp: u32,
    hp_max: u32,
    next_action: &'static Action,
    past_actions: VecDeque<&'static Action>,
}

static SPIKE_SLIME_S_TACKLE: Lazy<Action> = Lazy::new(|| Action::deal_damage(5, 1));

impl SpikeSlimeS {
    pub fn new_boxed(hp_rng: &mut StsRandom, ai_rng: &mut StsRandom) -> Box<Self> {
        let hp = hp_rng.gen_range(10..=14);
        let hp_max = hp;
        let past_actions = VecDeque::with_capacity(2);
        let next_action = Self::next_action(ai_rng, &past_actions);
        Box::new(Self {
            hp,
            hp_max,
            next_action,
            past_actions,
        })
    }

    fn next_action(ai_rng: &mut StsRandom, pa: &VecDeque<&'static Action>) -> &'static Action {
        let d100 = ai_rng.gen_range(0..100); // Burn a random number for consistency with the game
        println!("SPIKE_SLIME_S_TACKLE: {}: {}", ai_rng.get_counter(), d100);
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

    fn act(&mut self, ai_rng: &mut StsRandom) -> &'static Action {
        if self.past_actions.len() >= 2 {
            self.past_actions.pop_front();
        }
        self.past_actions.push_back(self.next_action);
        let result = self.next_action;
        self.next_action = Self::next_action(ai_rng, &self.past_actions);
        result
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
        Self {
            encounter,
            seed_for_floor,
            misc_rng,
            player,
        }
    }

    pub fn run(mut self) -> Result<(), Error> {
        println!(
            "[EncounterSimulator] Running encounter: {:?}",
            self.encounter
        );
        let mut hp_rng = StsRandom::from(self.seed_for_floor);
        let mut ai_rng = StsRandom::from(self.seed_for_floor);
        macro_rules! enemy_party {
            ( $( $enemy:ident ),* ) => {{
                let enemies: Vec<Box<dyn Enemy>> = vec![
                    $(
                        $enemy::new_boxed(&mut hp_rng, &mut ai_rng),
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

        let shuffle_rng = StsRandom::from(self.seed_for_floor);
        let enemy_party_view = enemy_party
            .iter()
            .map(|e| (e.enemy_type(), e.intent(), e.health()))
            .collect();
        let mut player_in_combat = self.player.enter_combat(shuffle_rng, enemy_party_view)?;

        #[allow(clippy::never_loop, clippy::while_let_loop)]
        loop {
            player_in_combat.start_turn()?;
            loop {
                match player_in_combat.choose_next_action()? {
                    PlayerAction::PlayCard(card) => {
                        println!("Player plays card: {:?}", card);
                    }
                    PlayerAction::EndTurn => break,
                }
            }
            for enemy in enemy_party.iter_mut() {
                let enemy_action = enemy.act(&mut ai_rng);
                for effect in enemy_action.effects.iter() {
                    player_in_combat.apply_effect(*effect)?;
                }
            }
        }
    }
}
