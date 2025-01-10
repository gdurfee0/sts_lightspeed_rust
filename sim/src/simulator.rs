use std::sync::mpsc::{Receiver, Sender};

use crate::data::{Act, Ascension, Character, Event};
use crate::map::{MapBuilder, NodeGrid};
use crate::rng::{EncounterGenerator, Seed};

// TODO: Use structured input
pub type SimulatorInput = String;

/// The output of the simulator. This is sent to the main thread for display and to await user
/// input, or to an AI for training and evaluation.
#[derive(Clone, Debug)]
pub enum SimulatorOutput {
    // ASCII representation of the current map
    Map(String),
    // TODO: Limit output to a subset of the internal state
    AwaitingInput(InternalState),
}

pub struct Simulator {
    seed: Seed,
    character: Character,
    ascension: Ascension,
    input_rx: Receiver<SimulatorInput>,
    output_tx: Sender<SimulatorOutput>,
    state: InternalState,
    map: NodeGrid,
    encounter_generator: EncounterGenerator,
}

// TODO: restrict visibility
#[derive(Clone, Copy, Debug)]
pub enum InternalState {
    Event(Event),
}

impl Simulator {
    pub fn new(
        seed: Seed,
        character: Character,
        ascension: Ascension,
        input_rx: Receiver<String>,
        output_tx: Sender<SimulatorOutput>,
    ) -> Self {
        let map = MapBuilder::from(&seed, ascension, Act::get(1)).build();
        let encounter_generator = EncounterGenerator::new(&seed);
        let state = InternalState::Event(Event::Neow);
        Self {
            seed,
            character,
            ascension,
            input_rx,
            output_tx,
            state,
            map,
            encounter_generator,
        }
    }

    pub fn run(mut self) {
        println!("Simulator started");
        if let Err(e) = self
            .output_tx
            .send(SimulatorOutput::Map(self.map.to_string()))
        {
            println!("Error initial sending map: {}", e);
            return;
        }
        loop {
            if let Err(e) = self
                .output_tx
                .send(SimulatorOutput::AwaitingInput(self.state))
            {
                println!("Error sending state: {}", e);
                break;
            }
            match self.input_rx.recv() {
                Ok(input) => self.evolve_state(input.trim()),
                Err(e) => {
                    println!("Error receiving input: {}", e);
                    break;
                }
            }
        }
        println!("Simulator finished");
    }

    fn evolve_state(&mut self, user_input: &str) {
        println!("Got {}", user_input);
    }
}
