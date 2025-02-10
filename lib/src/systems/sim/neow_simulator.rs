use std::iter::once;

use anyhow::Error;

use crate::components::{Choice, Interaction, PlayerPersistentState, Prompt};
use crate::data::{Card, Character, NeowBlessing, NeowBonus, NeowPenalty, Relic};
use crate::systems::base::{DeckSystem, GoldSystem};
use crate::systems::rng::{CardGenerator, NeowGenerator, PotionGenerator, RelicGenerator, Seed};

pub struct NeowSimulator<'a, I: Interaction> {
    neow_generator: NeowGenerator<'a>,
    starting_relic: Relic,
    comms: &'a I,
}

impl<'a, I: Interaction> NeowSimulator<'a, I> {
    pub fn new(
        seed: Seed,
        character: &'static Character,
        card_generator: &'a mut CardGenerator,
        potion_generator: &'a mut PotionGenerator,
        relic_generator: &'a mut RelicGenerator,
        comms: &'a I,
    ) -> Self {
        let neow_generator = NeowGenerator::new(
            seed,
            character,
            card_generator,
            potion_generator,
            relic_generator,
        );
        let starting_relic = character.starting_relic;
        Self {
            starting_relic,
            neow_generator,
            comms,
        }
    }

    pub fn run(mut self, player_persistent_state: &mut PlayerPersistentState) -> Result<(), Error> {
        let choices = self
            .neow_generator
            .blessing_choices()
            .iter()
            .copied()
            .map(Choice::NeowBlessing)
            .collect::<Vec<_>>();
        match self.comms.prompt_for_choice(Prompt::ChooseNeow, &choices)? {
            Choice::NeowBlessing(blessing) => {
                self.handle_neow_blessing(*blessing, player_persistent_state)
            }
            invalid => unreachable!("{:?}", invalid),
        }
    }

    fn handle_neow_blessing(
        &mut self,
        blessing: NeowBlessing,
        player_persistent_state: &mut PlayerPersistentState,
    ) -> Result<(), Error> {
        match blessing {
            NeowBlessing::ChooseCard => {
                let cards = self.neow_generator.three_card_choices();
                DeckSystem::choose_card_to_obtain(
                    self.comms,
                    &mut player_persistent_state.deck,
                    &cards,
                )
            }
            NeowBlessing::ChooseColorlessCard => {
                let cards = self.neow_generator.three_colorless_card_choices();
                DeckSystem::choose_card_to_obtain(
                    self.comms,
                    &mut player_persistent_state.deck,
                    &cards,
                )
            }
            NeowBlessing::GainOneHundredGold => {
                GoldSystem::increase_gold(self.comms, &mut player_persistent_state.gold, 100)
            }
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
            NeowBlessing::RemoveCard => self.player.choose_card_to_remove(),
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
}
