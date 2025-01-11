use std::sync::mpsc::{Receiver, Sender};

use anyhow::{Error, Ok};

use super::message::{Choice, PlayerView, Prompt, StsMessage};
use super::neow::NeowSimulator;
use super::player::Player;

use crate::data::{Act, Ascension, Character};
use crate::map::{MapBuilder, NodeGrid, Room};
use crate::rng::{EncounterGenerator, Seed, StsRandom};

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
        self.send_map()?;
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

        // Player needs to enter the board
        self.send_map()?;
        let mut prompt = Prompt::EnterMap;
        let mut choices = self
            .map
            .nonempty_cols_for_row(0)
            .into_iter()
            .map(Choice::MapEntryColumn)
            .collect::<Vec<_>>();
        self.send_prompt_and_choices(prompt, &choices)?;
        loop {
            let choice_index = self.input_rx.recv()?;
            if let Some(choice) = choices.get(choice_index) {
                if let Some((p, c)) = self.handle_response(choice)? {
                    prompt = p;
                    choices = c;
                } else {
                    break;
                }
                self.send_player_view()?;
            } else {
                eprintln!(
                    "[Simulator] Invalid choice index {} from client; expected 0..{}",
                    choice_index,
                    choices.len()
                );
            }
            self.send_prompt_and_choices(prompt, &choices)?;
        }
        self.output_tx.send(StsMessage::GameOver(true))?;
        println!("[Simulator] Exiting.");
        Ok(())
    }

    fn send_map(&self) -> Result<(), anyhow::Error> {
        self.output_tx.send(StsMessage::Map(
            self.map
                .to_string_with_highlighted_row_col(self.player_row_col)
                + "\n\n a  b  c  d  e  f  g",
        ))?;
        Ok(())
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

    fn send_prompt_and_choices(
        &self,
        prompt: Prompt,
        choices: &[Choice],
    ) -> Result<(), anyhow::Error> {
        self.output_tx
            .send(StsMessage::Choose(prompt, choices.to_vec()))?;
        Ok(())
    }

    fn handle_response(&mut self, choice: &Choice) -> Result<Option<(Prompt, Vec<Choice>)>, Error> {
        match choice {
            Choice::MapEntryColumn(col) => {
                self.player_row_col = Some((0, *col));
                self.send_map()?;
                let room = self
                    .map
                    .get(0, *col)
                    .expect("We offered an invalid column")
                    .room;
                self.enter_room(room)
            }
            _ => unreachable!(),
        }
    }

    fn enter_room(&mut self, room: Room) -> Result<Option<(Prompt, Vec<Choice>)>, Error> {
        println!("[Simulator] Player entered room {:?}", room);
        Ok(None)
    }
}
