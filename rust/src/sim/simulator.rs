use std::sync::mpsc::{Receiver, Sender};

use super::message::StsMessage;
use super::neow::NeowSimulator;
use super::player::Player;

use crate::data::{Ascension, Character};
use crate::map::Room;
use crate::rng::{EncounterGenerator, RelicGenerator, Seed, StsRandom};
use crate::sim::map::MapSimulator;

pub struct StsSimulator {
    // Information typically set on the command line
    seed: Seed,
    character: &'static Character,
    ascension: Ascension,

    // Random number generators for various game elements
    encounter_generator: EncounterGenerator,
    card_sts_random: StsRandom,
    potion_sts_random: StsRandom,
    relic_generator: RelicGenerator,

    // Current player state
    player: Player,
}

impl StsSimulator {
    pub fn new(
        seed: Seed,
        character: &'static Character,
        ascension: Ascension,
        input_rx: Receiver<usize>,
        output_tx: Sender<StsMessage>,
    ) -> Self {
        let encounter_generator = EncounterGenerator::new(&seed);
        let card_sts_random = StsRandom::from(&seed);
        let potion_sts_random = StsRandom::from(&seed);
        let relic_generator = RelicGenerator::new(&seed, character);
        let player = Player::new(character, input_rx, output_tx);
        Self {
            seed,
            character,
            ascension,
            encounter_generator,
            card_sts_random,
            potion_sts_random,
            relic_generator,
            player,
        }
    }

    pub fn run(mut self) -> Result<(), anyhow::Error> {
        println!(
            "[Simulator] Starting simulator of size {} with messages of size {}",
            std::mem::size_of::<StsSimulator>(),
            std::mem::size_of::<StsMessage>(),
        );
        self.player.send_initial_state()?;
        let mut map_simulator = MapSimulator::new(&self.seed, self.ascension);
        map_simulator.send_map_to_player(&mut self.player)?;
        let neow_simulator = NeowSimulator::new(
            self.seed.clone(),
            self.character,
            &mut self.card_sts_random,
            &mut self.potion_sts_random,
            &mut self.relic_generator,
            &mut self.player,
        );
        neow_simulator.run()?;

        loop {
            match map_simulator.advance(&mut self.player)? {
                Room::Boss => todo!(),
                Room::BurningElite1 => todo!(),
                Room::BurningElite2 => todo!(),
                Room::BurningElite3 => todo!(),
                Room::BurningElite4 => todo!(),
                Room::Campfire => todo!(),
                Room::Elite => todo!(),
                Room::Event => todo!(),
                Room::Monster => {
                    let m = self.encounter_generator.next_monster_encounter();
                    println!("[Simulator] Encountering monster {:?}", m);
                    todo!();
                }
                Room::Shop => todo!(),
                Room::Treasure => todo!(),
            }
        }

        //self.output_tx.send(StsMessage::GameOver(true))?;
        //println!("[Simulator] Exiting.");
        //Ok(())
    }
}
