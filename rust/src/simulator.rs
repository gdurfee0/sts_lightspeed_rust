use std::sync::mpsc::{Receiver, Sender};

use anyhow::{anyhow, Ok};

use crate::data::{Act, Ascension, Character, NeowBlessing, NeowBonus, NeowPenalty};
use crate::map::{MapBuilder, NodeGrid};
use crate::message::{Choice, PlayerView, Prompt, StsMessage};
use crate::rng::{EncounterGenerator, NeowGenerator, Seed};

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
    Neow,
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
            state: State::Neow,
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
            (State::Neow, Choice::NeowBlessing(blessing)) => {
                self.handle_neow_blessing(blessing)?;
                self.state = State::Halt;
                Ok((Prompt::HaltAndCatchFire, vec![Choice::CatchFire]))
            }
            (State::Neow, _) => unreachable!(),
            (State::Halt, Choice::CatchFire) => {
                Ok((Prompt::HaltAndCatchFire, vec![Choice::CatchFire]))
            }
            (State::Halt, _) => unreachable!(),
        }
    }

    fn handle_neow_blessing(&mut self, blessing: &NeowBlessing) -> Result<(), anyhow::Error> {
        match blessing {
            NeowBlessing::ChooseOneOfThreeCards => todo!(),
            NeowBlessing::ChooseUncommonColorlessCard => todo!(),
            NeowBlessing::GainOneHundredGold => {
                self.player_gold += 100;
            }
            NeowBlessing::IncreaseMaxHpByTenPercent => {
                self.player_hp_max += self.player_hp_max / 10;
                self.player_hp = self.player_hp_max;
            }
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
                    NeowBonus::IncreaseMaxHpByTwentyPercent => {
                        self.player_hp_max += self.player_hp_max / 5;
                        self.player_hp = self.player_hp_max;
                    }
                    NeowBonus::ObtainRandomRareRelic => todo!(),
                    NeowBonus::RemoveTwoCards => todo!(),
                    NeowBonus::TransformTwoCards => todo!(),
                }
                match penalty {
                    NeowPenalty::DecreaseMaxHpByTenPercent => {
                        self.player_hp_max -= self.player_hp_max / 10;
                        self.player_hp = self.player_hp_max;
                    }
                    NeowPenalty::LoseAllGold => {
                        self.player_gold = 0;
                    }
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
