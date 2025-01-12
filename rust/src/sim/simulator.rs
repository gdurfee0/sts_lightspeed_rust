use std::sync::mpsc::{Receiver, Sender};

use anyhow::{Error, Ok};

use super::message::{Choice, PlayerView, Prompt, StsMessage};
use super::neow::NeowSimulator;
use super::player::Player;

use crate::data::{Act, Ascension, Character};
use crate::map::{MapBuilder, MapHighlighter, NodeGrid, Room};
use crate::rng::{EncounterGenerator, Seed, StsRandom};
use crate::sim::map::MapSimulator;

pub struct StsSimulator {
    // Information typically set on the command line
    seed: Seed,
    character: &'static Character,
    ascension: Ascension,

    // Communication channels
    input_rx: Receiver<usize>,
    output_tx: Sender<StsMessage>,

    // Random number generators for various game elements
    encounter_generator: EncounterGenerator,
    card_sts_random: StsRandom,

    // Current map layout
    map: NodeGrid,

    // Current player state
    player: Player,

    // Current player row and column in the map
    player_row_col: Option<(usize, usize)>,
}

impl StsSimulator {
    pub fn new(
        seed: Seed,
        character: &'static Character,
        ascension: Ascension,
        input_rx: Receiver<usize>,
        output_tx: Sender<StsMessage>,
    ) -> Self {
        let map = MapBuilder::from(&seed, ascension, Act::get(1)).build();
        let encounter_generator = EncounterGenerator::new(&seed);
        let card_sts_random = StsRandom::from(&seed);
        Self {
            seed,
            character,
            ascension,
            input_rx,
            output_tx,
            encounter_generator,
            card_sts_random,
            map,
            player: Player {
                hp: character.starting_hp,
                hp_max: character.starting_hp,
                gold: 99,
                relics: vec![character.starting_relic],
                deck: character.starting_deck.to_vec(),
            },
            player_row_col: None,
        }
    }

    pub fn run(mut self) -> Result<(), anyhow::Error> {
        println!(
            "[Simulator] Starting simulator of size {} with messages of size {}",
            std::mem::size_of::<StsSimulator>(),
            std::mem::size_of::<StsMessage>(),
        );
        let mut map_simulator = MapSimulator::new(&self.seed, self.ascension);
        map_simulator.send_map(&mut self.output_tx)?;
        self.send_relics()?;
        self.send_deck()?;
        self.send_player_view()?;
        let neow_simulator = NeowSimulator::new(
            self.seed.clone(),
            self.character,
            &mut self.input_rx,
            &mut self.output_tx,
            &mut self.card_sts_random,
            &mut self.player,
        );
        neow_simulator.run()?;
        self.send_relics()?;
        self.send_deck()?;
        self.send_player_view()?;

        loop {
            match map_simulator.advance(&mut self.input_rx, &mut self.output_tx)? {
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

    fn send_relics(&self) -> Result<(), anyhow::Error> {
        self.output_tx
            .send(StsMessage::Relics(self.player.relics.clone()))?;
        Ok(())
    }

    fn send_deck(&self) -> Result<(), anyhow::Error> {
        self.output_tx
            .send(StsMessage::Deck(self.player.deck.clone()))?;
        Ok(())
    }

    fn send_player_view(&self) -> Result<(), anyhow::Error> {
        self.output_tx.send(StsMessage::View(PlayerView {
            hp: self.player.hp,
            hp_max: self.player.hp_max,
            gold: self.player.gold,
        }))?;
        Ok(())
    }
}
