use anyhow::Error;

use crate::data::{Encounter, EnemyType};
use crate::rng::{Seed, StsRandom};

use super::combat::PlayerAction;
use super::enemy::Enemy;
use super::player::Player;

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
        let mut player_in_combat = self.player.enter_combat(shuffle_rng);

        #[allow(clippy::never_loop, clippy::while_let_loop)]
        loop {
            for (i, status) in enemy_party.iter().map(Enemy::status).enumerate() {
                player_in_combat.update_enemy_status(status, i)?;
            }
            player_in_combat.start_turn()?;
            loop {
                let enemies = enemy_party
                    .iter()
                    .map(Enemy::enemy_type)
                    .collect::<Vec<_>>();
                let hand_index = match player_in_combat.choose_next_action(&enemies)? {
                    PlayerAction::PlayerMove(_player_move, hand_index) => hand_index,
                    PlayerAction::PlayerMoveWithTarget(player_move, target_index, hand_index) => {
                        println!(
                            "[EncounterSimulator] PlayerMoveWithTarget: {:?} -> {:?}",
                            player_move, enemy_party[target_index]
                        );
                        for effect in player_move.effects.iter() {
                            let enemy = &mut enemy_party[target_index];
                            if !enemy.apply_effect(*effect) {
                                // Remove this enemy from the party
                                player_in_combat.enemy_died(enemy.enemy_type(), target_index)?;
                                enemy_party.remove(target_index);
                                break;
                            }
                            player_in_combat.update_enemy_status(enemy.status(), target_index)?;
                        }
                        hand_index
                    }
                    PlayerAction::EndTurn => break,
                };
                if enemy_party.is_empty() {
                    // Battle is over!
                    return Ok(());
                }
                player_in_combat.discard_card(hand_index)?;
            }
            for enemy in enemy_party.iter_mut() {
                // TODO: check for death and remove
                let _ = enemy.start_turn();
            }
            //enemy_party.retain(|enemy| enemy.health().0 > 0);
            for enemy in enemy_party.iter_mut() {
                let enemy_move = enemy.next_move(&mut ai_rng);
                for effect in enemy_move.effects.iter() {
                    player_in_combat.apply_effect(*effect)?;
                }
            }
        }
    }
}
