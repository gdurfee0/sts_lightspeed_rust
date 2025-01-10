use std::sync::mpsc::{Receiver, Sender};

use crate::data::{Act, Ascension, Character, Event};
use crate::input::{Choice, Prompt};
use crate::map::{MapBuilder, NodeGrid};
use crate::rng::{EncounterGenerator, NeowGenerator, Seed};

/// The output of the simulator. This is sent to the main thread for display and to await user
/// input, or to an AI for training and evaluation.
#[derive(Clone, Debug)]
pub enum SimulatorOutput {
    /// ASCII representation of the current map
    Map(String),

    /// A list of strings, each representing a possible action; the agent must select one
    /// using zero-indexing.
    Choose(Prompt, Vec<Choice>),
}

pub struct Simulator {
    seed: Seed,
    character: Character,
    ascension: Ascension,
    input_rx: Receiver<usize>,
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
        input_rx: Receiver<usize>,
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
        let mut neow_generator = NeowGenerator::new(&self.seed);
        let neow_choices: Vec<Choice> = neow_generator
            .blessing_choices()
            .map(Choice::NeowBlessing)
            .to_vec();
        if let Err(e) = self
            .output_tx
            .send(SimulatorOutput::Choose(Prompt::NeowBlessing, neow_choices))
        {
            println!("Error sending choices: {}", e);
            return;
        }
        loop {
            if let Some((prompt, choices)) = match self.input_rx.recv() {
                Ok(choice) => self.evolve_state(choice),
                Err(e) => {
                    println!("Error receiving choice: {}", e);
                    break;
                }
            } {
                if let Err(e) = self
                    .output_tx
                    .send(SimulatorOutput::Choose(prompt, choices))
                {
                    println!("Error sending choices: {}", e);
                    break;
                }
            } else {
                println!("Game over");
                break;
            }
        }
        println!("Simulator finished");
    }

    fn evolve_state(&mut self, choice: usize) -> Option<(Prompt, Vec<Choice>)> {
        println!("Got {}", choice);
        None
    }
}
