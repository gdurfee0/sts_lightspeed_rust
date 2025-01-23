use std::iter::once;

use anyhow::Error;

use crate::data::{Card, Character, NeowBlessing, NeowBonus, NeowPenalty, Relic};
use crate::systems::rng::{CardGenerator, NeowGenerator, RelicGenerator, Seed, StsRandom};
use crate::{Choice, Prompt};

use super::player::Player;

pub struct NeowSimulator<'a> {
    neow_generator: NeowGenerator<'a>,
    starting_relic: Relic,
    player: &'a mut Player,
}

impl<'a> NeowSimulator<'a> {
    pub fn new(
        seed: Seed,
        character: &'static Character,
        card_generator: &'a mut CardGenerator,
        potion_rng: &'a mut StsRandom,
        relic_generator: &'a mut RelicGenerator,
        player: &'a mut Player,
    ) -> Self {
        let neow_generator =
            NeowGenerator::new(seed, character, potion_rng, card_generator, relic_generator);
        let starting_relic = character.starting_relic;
        Self {
            starting_relic,
            neow_generator,
            player,
        }
    }

    pub fn run(mut self) -> Result<(), Error> {
        let choices = self
            .neow_generator
            .blessing_choices()
            .iter()
            .copied()
            .map(Choice::NeowBlessing)
            .collect::<Vec<_>>();
        match self
            .player
            .comms
            .prompt_for_choice(Prompt::ChooseNeow, &choices)?
        {
            Choice::NeowBlessing(blessing) => self.handle_neow_blessing(*blessing),
            invalid => unreachable!("{:?}", invalid),
        }
    }

    fn handle_neow_blessing(&mut self, blessing: NeowBlessing) -> Result<(), Error> {
        match blessing {
            NeowBlessing::ChooseCard => {
                let cards = self.neow_generator.three_card_choices();
                self.choose_card_to_obtain(cards)
            }
            NeowBlessing::ChooseColorlessCard => {
                let cards = self.neow_generator.three_colorless_card_choices();
                self.choose_card_to_obtain(cards)
            }
            NeowBlessing::GainOneHundredGold => self.player.increase_gold(100),
            NeowBlessing::IncreaseMaxHpByTenPercent => {
                self.player.increase_hp_max(self.player.state.hp_max / 10)
            }
            NeowBlessing::NeowsLament => self.player.obtain_relic(Relic::NeowsLament),
            NeowBlessing::ObtainRandomCommonRelic => {
                self.player.obtain_relic(self.neow_generator.common_relic())
            }
            NeowBlessing::ObtainRandomRareCard => self
                .player
                .obtain_card(self.neow_generator.one_random_rare_card()),
            NeowBlessing::ObtainThreeRandomPotions => self
                .player
                .choose_potions_to_obtain(&self.neow_generator.three_random_potions(), 3),
            NeowBlessing::RemoveCard => self.choose_card_to_remove(),
            NeowBlessing::ReplaceStarterRelic => {
                let replacement_relic = self.neow_generator.boss_relic();
                self.player
                    .replace_relic(self.starting_relic, replacement_relic)
            }
            NeowBlessing::TransformCard => todo!(),
            NeowBlessing::UpgradeCard => todo!(),
            NeowBlessing::Composite(bonus, penalty) => {
                match penalty {
                    NeowPenalty::DecreaseMaxHpByTenPercent => {
                        self.player.decrease_hp_max(self.player.state.hp_max / 10)?;
                    }
                    NeowPenalty::LoseAllGold => {
                        self.player.decrease_gold(self.player.state.gold)?;
                    }
                    NeowPenalty::ObtainCurse => {
                        self.player.obtain_card(self.neow_generator.one_curse())?;
                    }
                    NeowPenalty::TakeDamage => {
                        self.player.decrease_hp(self.player.state.hp / 10 * 3)?;
                    }
                }
                match bonus {
                    NeowBonus::ChooseRareCard => todo!(),
                    NeowBonus::ChooseRareColorlessCard => todo!(),
                    NeowBonus::GainTwoHundredFiftyGold => self.player.increase_gold(250),
                    NeowBonus::IncreaseMaxHpByTwentyPercent => {
                        self.player.increase_hp_max(self.player.state.hp_max / 5)
                    }
                    NeowBonus::ObtainRandomRareRelic => todo!(),
                    NeowBonus::RemoveTwoCards => todo!(),
                    NeowBonus::TransformTwoCards => todo!(),
                }
            }
        }
    }

    fn choose_card_to_obtain(&mut self, cards: Vec<Card>) -> Result<(), Error> {
        let choices = cards
            .iter()
            .copied()
            .map(Choice::ObtainCard)
            .chain(once(Choice::Skip))
            .collect::<Vec<_>>();
        match self
            .player
            .comms
            .prompt_for_choice(Prompt::ChooseOne, &choices)?
        {
            Choice::ObtainCard(card) => self.player.obtain_card(*card),
            Choice::Skip => Ok(()),
            invalid => unreachable!("{:?}", invalid),
        }
    }

    fn choose_card_to_remove(&mut self) -> Result<(), Error> {
        let deck = self.player.state.deck.clone();
        let choices = deck
            .iter()
            .copied()
            .enumerate()
            .map(|(deck_index, card)| Choice::RemoveCard(deck_index, card))
            .collect::<Vec<_>>();
        match self
            .player
            .comms
            .prompt_for_choice(Prompt::RemoveCard, &choices)?
        {
            Choice::RemoveCard(deck_index, _) => self.player.remove_card(*deck_index),
            invalid => unreachable!("{:?}", invalid),
        }
    }
}
