use std::io::stdin;

use crate::data::{Act, Ascension, Character, Event};
use crate::map::{MapBuilder, NodeGrid};
use crate::rng::{EncounterGenerator, Seed};

pub struct Simulator {
    seed: Seed,
    character: Character,
    ascension: Ascension,
    map: NodeGrid,
    encounter_generator: EncounterGenerator,
    state: State,
}

#[derive(Debug)]
enum State {
    Event(Event),
}

impl Simulator {
    pub fn new(seed: Seed, character: Character, ascension: Ascension) -> Self {
        let map = MapBuilder::from(&seed, ascension, Act::get(1)).build();
        let encounter_generator = EncounterGenerator::new(&seed);
        let state = State::Event(Event::Neow);
        Self {
            seed,
            character,
            ascension,
            map,
            encounter_generator,
            state,
        }
    }

    pub fn run(mut self) {
        let mut user_input = String::new();
        loop {
            self.display_state();
            match stdin().read_line(&mut user_input) {
                Ok(0) | Err(_) => {
                    break;
                }
                Ok(_) => {
                    self.evolve_state(user_input.trim());
                }
            }
            user_input.clear();
        }
    }

    fn display_state(&self) {
        println!("Current state: {:?}", self.state);
    }

    fn evolve_state(&mut self, user_input: &str) {
        println!("Got {}", user_input);
    }
}
