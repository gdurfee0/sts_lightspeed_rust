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
    state: State,
    map: NodeGrid,
    encounter_generator: EncounterGenerator,
}

#[derive(Clone, Copy, Debug)]
enum State {
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
        let state = State::Event(Event::Neow);
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

    pub fn run(mut self) -> Result<(), anyhow::Error> {
        println!("Simulator started");
        self.output_tx
            .send(SimulatorOutput::Map(self.map.to_string()))?;
        let mut neow_generator = NeowGenerator::new(&self.seed);
        let mut choices: Vec<Choice> = neow_generator
            .blessing_choices()
            .map(Choice::NeowBlessing)
            .to_vec();
        let mut prompt = Prompt::NeowBlessing;
        loop {
            self.output_tx
                .send(SimulatorOutput::Choose(prompt, choices.clone()))?;
            let choice_index = self.input_rx.recv()?;
            match choices.get(choice_index) {
                Some(choice) => match self.evolve_state(*choice) {
                    Some((new_prompt, new_choices)) => {
                        prompt = new_prompt;
                        choices = new_choices;
                    }
                    None => break,
                },
                None => {
                    println!("Invalid choice index {}", choice_index);
                }
            }
        }
        println!("Simulator finished");
        Ok(())
    }

    fn evolve_state(&mut self, choice: Choice) -> Option<(Prompt, Vec<Choice>)> {
        println!("User chose \"{}\"", choice);
        None
    }
}
