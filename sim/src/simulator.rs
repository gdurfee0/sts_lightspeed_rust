use std::sync::mpsc::{Receiver, Sender};

use anyhow::{anyhow, Ok};

use crate::data::{Act, Ascension, Character, NeowBlessing, NeowBonus, NeowPenalty};
use crate::input::{Choice, Prompt};
use crate::map::{MapBuilder, NodeGrid};
use crate::rng::{EncounterGenerator, NeowGenerator, Seed};

/// Message type for communication from the Simualtor to a client (human operator or AI agent).
/// The Simulator will send any number of these messages to the client, concluding with a
/// `Choose` message, at which point control passes to the client and the Simulator waits
/// for a response.
#[derive(Debug)]
pub enum StsMessage {
    /// ASCII representation of the current map.
    Map(String),

    // TODO: less frequently changing information such as deck composition, relics, etc.
    //
    // All information that might change on a move-by-move basis, such as the player's HP and gold.
    View(PlayerView),

    /// A list of `Choice`s, each representing a possible action; the client must select one
    /// using zero-indexing and return its response as `usize` via its input_tx channel.
    Choose(Prompt, Vec<Choice>),

    /// Indicates that the game is over. The boolean indicates whether the player won or lost.
    GameOver(bool),
}

#[derive(Clone, Debug)]
pub struct PlayerView {
    // TODO: keys
    // TODO: character? or expect client to remember?
    pub hp: u32,
    pub hp_max: u32,
    pub gold: u32,
    // TODO: potions
}

pub struct StsSimulator {
    // Information typically set on the command line
    seed: Seed,
    character: &'static Character,
    ascension: Ascension,

    // Communication channels
    input_rx: Receiver<usize>,
    output_tx: Sender<StsMessage>,

    // Random number generators for various game elements
    neow_generator: NeowGenerator,
    encounter_generator: EncounterGenerator,

    // Current map layout
    map: NodeGrid,
    player_row_col: Option<(usize, usize)>,

    // Current player state
    player_hp: u32,
    player_hp_max: u32,
    player_gold: u32,

    // State machine node for the simulator
    state: State,
}

#[derive(Clone, Copy, Debug)]
enum State {
    NeowBlessing,
    Halt,
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
        let neow_generator = NeowGenerator::new(&seed);
        let encounter_generator = EncounterGenerator::new(&seed);
        Self {
            seed,
            character,
            ascension,
            input_rx,
            output_tx,
            neow_generator,
            encounter_generator,
            map,
            player_row_col: None,
            player_hp: character.start_hp,
            player_hp_max: character.start_hp,
            player_gold: 99,
            state: State::NeowBlessing,
        }
    }

    pub fn run(mut self) -> Result<(), anyhow::Error> {
        println!(
            "[Simulator] Starting simulator of size {} with messages of size {}",
            std::mem::size_of::<StsSimulator>(),
            std::mem::size_of::<StsMessage>(),
        );
        self.send_map()?;
        self.send_player_view()?;
        let mut prompt = Prompt::NeowBlessing;
        let mut choices = self
            .neow_generator
            .blessing_choices()
            .map(Choice::NeowBlessing)
            .to_vec();
        self.send_prompt_and_choices(prompt, &choices)?;
        loop {
            let choice_index = self.input_rx.recv()?;
            if let Some(choice) = choices.get(choice_index) {
                (prompt, choices) = self.handle_response(choice)?;
                self.send_player_view()?;
                self.send_prompt_and_choices(prompt, &choices)?;
            } else {
                return Err(anyhow!(
                    "[Simulator] Invalid choice index {} from client; expected 0..{}",
                    choice_index,
                    choices.len()
                ));
            }
        }
    }

    fn send_map(&self) -> Result<(), anyhow::Error> {
        self.output_tx.send(StsMessage::Map(self.map.to_string()))?;
        Ok(())
    }

    fn send_player_view(&self) -> Result<(), anyhow::Error> {
        self.output_tx.send(StsMessage::View(PlayerView {
            hp: self.player_hp,
            hp_max: self.player_hp_max,
            gold: self.player_gold,
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

    fn handle_response(&mut self, choice: &Choice) -> Result<(Prompt, Vec<Choice>), anyhow::Error> {
        match (self.state, choice) {
            (State::NeowBlessing, Choice::NeowBlessing(blessing)) => {
                self.handle_neow_blessing(blessing)?;
                self.state = State::Halt;
                Ok((Prompt::HaltAndCatchFire, vec![Choice::CatchFire]))
            }
            (State::NeowBlessing, _) => unreachable!(),
            (State::Halt, Choice::CatchFire) => {
                Ok((Prompt::HaltAndCatchFire, vec![Choice::CatchFire]))
            }
            (State::Halt, _) => unreachable!(),
        }
    }

    fn handle_neow_blessing(&mut self, blessing: &NeowBlessing) -> Result<(), anyhow::Error> {
        match blessing {
            NeowBlessing::ChooseOneOfThreeCards => unimplemented!(),
            NeowBlessing::ChooseUncommonColorlessCard => todo!(),
            NeowBlessing::GainOneHundredGold => {
                self.player_gold += 100;
            }
            NeowBlessing::IncreaseMaxHpByTenPercent => todo!(),
            NeowBlessing::NeowsLament => todo!(),
            NeowBlessing::ObtainRandomCommonRelic => todo!(),
            NeowBlessing::ObtainRandomRareCard => todo!(),
            NeowBlessing::ObtainThreeRandomPotions => todo!(),
            NeowBlessing::RemoveCard => todo!(),
            NeowBlessing::ReplaceStarterRelic => todo!(),
            NeowBlessing::TransformCard => todo!(),
            NeowBlessing::UpgradeCard => todo!(),
            NeowBlessing::Composite(bonus, penalty) => {
                match bonus {
                    NeowBonus::ChooseRareCard => todo!(),
                    NeowBonus::ChooseRareColorlessCard => todo!(),
                    NeowBonus::GainTwoHundredFiftyGold => {
                        self.player_gold += 250;
                    }
                    NeowBonus::IncreaseMaxHpByTwentyPercent => todo!(),
                    NeowBonus::ObtainRandomRareRelic => todo!(),
                    NeowBonus::RemoveTwoCards => todo!(),
                    NeowBonus::TransformTwoCards => todo!(),
                }
                match penalty {
                    NeowPenalty::DecreaseMaxHpByTenPercent => todo!(),
                    NeowPenalty::LoseAllGold => todo!(),
                    NeowPenalty::ObtainCurse => todo!(),
                    NeowPenalty::TakeDamage => {
                        self.player_hp -= (self.player_hp / 10) * 3;
                    }
                }
            }
        }
        Ok(())
    }
}
