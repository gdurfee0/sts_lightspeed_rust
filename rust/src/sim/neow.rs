use std::sync::mpsc::{Receiver, Sender};

use anyhow::{Error, Ok};

use super::message::{Choice, Prompt, StsMessage};

use crate::data::{
    Card, Character, NeowBlessing, NeowBonus, NeowPenalty, Relic, UNCOMMON_COLORLESS_CARDS,
};
use crate::rng::{NeowGenerator, Seed, StsRandom};

pub struct NeowSimulator<'a> {
    // Information typically set on the command line
    character: &'static Character,

    // Communication channels
    input_rx: &'a mut Receiver<usize>,
    output_tx: &'a mut Sender<StsMessage>,

    // Random number generators for various game elements
    neow_generator: NeowGenerator,
    card_sts_random: &'a mut StsRandom,

    // Current player state
    player_hp: &'a mut u32,
    player_hp_max: &'a mut u32,
    player_gold: &'a mut u32,
    player_relics: &'a mut Vec<Relic>,
    player_deck: &'a mut Vec<Card>,
}

impl<'a> NeowSimulator<'a> {
    pub fn new(
        seed: Seed,
        character: &'static Character,
        input_rx: &'a mut Receiver<usize>,
        output_tx: &'a mut Sender<StsMessage>,
        card_sts_random: &'a mut StsRandom,
        player_hp: &'a mut u32,
        player_hp_max: &'a mut u32,
        player_gold: &'a mut u32,
        player_relics: &'a mut Vec<Relic>,
        player_deck: &'a mut Vec<Card>,
    ) -> Self {
        let neow_generator = NeowGenerator::new(&seed, character);
        Self {
            character,
            input_rx,
            output_tx,
            neow_generator,
            card_sts_random,
            player_hp,
            player_hp_max,
            player_gold,
            player_relics,
            player_deck,
        }
    }
    pub fn run(mut self) -> Result<(), anyhow::Error> {
        println!(
            "[NeowSimulator] Starting simulator of size {}",
            std::mem::size_of::<NeowSimulator>(),
        );
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
                if let Some((p, c)) = self.handle_response(choice)? {
                    prompt = p;
                    choices = c;
                } else {
                    break;
                }
            } else {
                eprintln!(
                    "[Simulator] Invalid choice index {} from client; expected 0..{}",
                    choice_index,
                    choices.len()
                );
            }
            self.send_prompt_and_choices(prompt, &choices)?;
        }
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
            Choice::NeowBlessing(blessing) => self.handle_neow_blessing(blessing),
            Choice::ObtainCard(card) => {
                self.player_deck.push(*card);
                // NeowSimulator is finished.
                Ok(None)
            }
            _ => unreachable!(),
        }
    }

    fn handle_neow_blessing(
        &mut self,
        blessing: &NeowBlessing,
    ) -> Result<Option<(Prompt, Vec<Choice>)>, Error> {
        match blessing {
            NeowBlessing::ChooseOneOfThreeCards => Ok(Some((
                Prompt::ObtainCard,
                self.neow_generator
                    .three_card_choices()
                    .into_iter()
                    .map(Choice::ObtainCard)
                    .collect(),
            ))),
            NeowBlessing::ChooseUncommonColorlessCard => Ok(Some((
                Prompt::ObtainCard,
                self.card_sts_random
                    .sample_without_replacement(UNCOMMON_COLORLESS_CARDS, 3)
                    .into_iter()
                    .map(Choice::ObtainCard)
                    .collect(),
            ))),
            NeowBlessing::GainOneHundredGold => {
                *self.player_gold += 100;
                Ok(None)
            }
            NeowBlessing::IncreaseMaxHpByTenPercent => {
                *self.player_hp_max += *self.player_hp_max / 10;
                *self.player_hp = *self.player_hp_max;
                Ok(None)
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
                match penalty {
                    NeowPenalty::DecreaseMaxHpByTenPercent => {
                        *self.player_hp_max -= *self.player_hp_max / 10;
                        *self.player_hp = *self.player_hp_max;
                    }
                    NeowPenalty::LoseAllGold => {
                        *self.player_gold = 0;
                    }
                    NeowPenalty::ObtainCurse => todo!(),
                    NeowPenalty::TakeDamage => {
                        *self.player_hp -= (*self.player_hp / 10) * 3;
                    }
                }
                match bonus {
                    NeowBonus::ChooseRareCard => todo!(),
                    NeowBonus::ChooseRareColorlessCard => todo!(),
                    NeowBonus::GainTwoHundredFiftyGold => {
                        *self.player_gold += 250;
                        Ok(None)
                    }
                    NeowBonus::IncreaseMaxHpByTwentyPercent => {
                        *self.player_hp_max += *self.player_hp_max / 5;
                        *self.player_hp = *self.player_hp_max;
                        Ok(None)
                    }
                    NeowBonus::ObtainRandomRareRelic => todo!(),
                    NeowBonus::RemoveTwoCards => todo!(),
                    NeowBonus::TransformTwoCards => todo!(),
                }
            }
        }
    }
}
