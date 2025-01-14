use anyhow::Error;

use crate::data::{Encounter, EnemyType};
use crate::rng::{Seed, StsRandom};

use super::enemy::Enemy;
use super::player::{Player, PlayerAction};

pub struct EncounterSimulator<'a> {
    encounter: Encounter,
    seed_for_floor: Seed,
    misc_rng: &'a mut StsRandom,
    player: &'a mut Player,
}

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

    pub fn run(self) -> Result<(), Error> {
        println!(
            "[EncounterSimulator] Running encounter: {:?}",
            self.encounter
        );
        let mut hp_rng = StsRandom::from(self.seed_for_floor);
        let mut ai_rng = StsRandom::from(self.seed_for_floor);
        /*
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
        */
        macro_rules! enemy_party {
            ( $( $enemy:ident ),* ) => {{
                let enemies: Vec<Enemy> = vec![
                    $(
                        Enemy::new(EnemyType::$enemy, &mut hp_rng, &mut ai_rng),
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
                let enemy_action = enemy.next_action(&mut ai_rng);
                for effect in enemy_action.effects.iter() {
                    player_in_combat.apply_effect(*effect)?;
                }
            }
        }
    }
}
