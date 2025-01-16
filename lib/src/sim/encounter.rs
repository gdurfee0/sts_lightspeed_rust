use anyhow::Error;

use crate::data::Encounter;
use crate::enemy::{Enemy, EnemyType};
use crate::player::{CombatController, PlayerAction, PlayerController};
use crate::rng::{Seed, StsRandom};

pub struct EncounterSimulator<'a> {
    encounter: Encounter,
    seed_for_floor: Seed,
    misc_rng: &'a mut StsRandom,
    player: CombatController<'a>,
}

impl<'a> EncounterSimulator<'a> {
    pub fn new(
        seed_for_floor: Seed,
        encounter: Encounter,
        misc_rng: &'a mut StsRandom,
        player: &'a mut PlayerController,
    ) -> Self {
        let combat_controller = player.start_combat(StsRandom::from(seed_for_floor));
        Self {
            encounter,
            seed_for_floor,
            misc_rng,
            player: combat_controller,
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
            Encounter::ThreeLouses => todo!(),
            Encounter::ThreeSentries => todo!(),
            Encounter::ThreeShapes => todo!(),
            Encounter::TimeEater => todo!(),
            Encounter::Transient => todo!(),
            Encounter::TwoFungiBeasts => todo!(),
            Encounter::TwoLouses => todo!(),
            Encounter::TwoThieves => todo!(),
            Encounter::WrithingMass => todo!(),
        };

        #[allow(clippy::never_loop, clippy::while_let_loop)]
        loop {
            for (i, status) in enemy_party.iter().map(Enemy::status).enumerate() {
                self.player.update_enemy_status(i, status)?;
            }
            self.player.start_turn()?;
            loop {
                let enemies = enemy_party
                    .iter()
                    .map(Enemy::enemy_type)
                    .collect::<Vec<_>>();
                match self.player.choose_next_action(&enemies)? {
                    PlayerAction::PlayCard(action) => todo!(),
                    PlayerAction::PlayCardAgainstEnemy(action, target_index) => {
                        for effect in action.effects.iter() {
                            let enemy = &mut enemy_party[target_index];
                            if !enemy.apply_effect(*effect) {
                                // Remove this enemy from the party
                                self.player.enemy_died(target_index, enemy.enemy_type())?;
                                enemy_party.remove(target_index);
                                break;
                            }
                            self.player
                                .update_enemy_status(target_index, enemy.status())?;
                        }
                    }
                    PlayerAction::EndTurn => break,
                    _ => todo!(),
                }
                if enemy_party.is_empty() {
                    // Battle is over!
                    return Ok(());
                }
                self.player.discard_card_just_played()?;
            }
            for enemy in enemy_party.iter_mut() {
                // TODO: check for death and remove
                let _ = enemy.start_turn();
            }
            //enemy_party.retain(|enemy| enemy.health().0 > 0);
            for enemy in enemy_party.iter_mut() {
                let action = enemy.next_action(&mut ai_rng);
                for effect in action.effects.iter() {
                    match effect {
                        crate::Effect::AddToDiscardPile(_) => todo!(),
                        crate::Effect::DealDamage(_) => todo!(),
                        crate::Effect::GainBlock(_) => todo!(),
                        crate::Effect::Inflict(debuff, stacks) => {
                            self.player.apply_debuff(*debuff, *stacks)?
                        }
                    }
                }
            }
        }
    }
}
