use anyhow::Error;

use crate::components::{Choice, Interaction, PlayerPersistentState, Prompt};
use crate::data::{Character, NeowBlessing, NeowBonus, NeowPenalty, Relic};
use crate::systems::base::{DeckSystem, GoldSystem, HealthSystem, PotionSystem, RelicSystem};
use crate::systems::rng::{CardGenerator, NeowGenerator, PotionGenerator, RelicGenerator, Seed};

pub struct NeowSimulator<'a, I: Interaction> {
    neow_generator: NeowGenerator<'a>,
    starting_relic: Relic,
    comms: &'a I,
}

impl<'a, I: Interaction> NeowSimulator<'a, I> {
    /// Creates a new Neow simulator instance.
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

    /// Runs the Neow simulator.
    pub fn run(mut self, pps: &mut PlayerPersistentState) -> Result<(), Error> {
        let choices = self
            .neow_generator
            .blessing_choices()
            .iter()
            .copied()
            .map(Choice::NeowBlessing)
            .collect::<Vec<_>>();
        match self.comms.prompt_for_choice(Prompt::ChooseNeow, &choices)? {
            Choice::NeowBlessing(blessing) => self.handle_neow_blessing(*blessing, pps),
            invalid => unreachable!("{:?}", invalid),
        }
    }

    fn handle_neow_blessing(
        &mut self,
        blessing: NeowBlessing,
        pps: &mut PlayerPersistentState,
    ) -> Result<(), Error> {
        match blessing {
            NeowBlessing::ChooseCard => {
                let cards = self.neow_generator.three_card_choices();
                DeckSystem::choose_card_to_obtain(self.comms, pps, &cards)
            }
            NeowBlessing::ChooseColorlessCard => {
                let cards = self.neow_generator.three_colorless_card_choices();
                DeckSystem::choose_card_to_obtain(self.comms, pps, &cards)
            }
            NeowBlessing::GainOneHundredGold => GoldSystem::increase_gold(self.comms, pps, 100),
            NeowBlessing::IncreaseMaxHpByTenPercent => {
                HealthSystem::increase_hp_max(self.comms, pps, pps.hp_max / 10)
            }
            NeowBlessing::NeowsLament => {
                RelicSystem::obtain_relic(self.comms, pps, Relic::NeowsLament)
            }
            NeowBlessing::ObtainRandomCommonRelic => {
                RelicSystem::obtain_relic(self.comms, pps, self.neow_generator.common_relic())
            }
            NeowBlessing::ObtainRandomRareCard => DeckSystem::choose_card_to_obtain(
                self.comms,
                pps,
                &[self.neow_generator.one_random_rare_card()],
            ),
            NeowBlessing::ObtainThreeRandomPotions => PotionSystem::choose_potions_to_obtain(
                self.comms,
                pps,
                &self.neow_generator.three_random_potions(),
                3,
            ),
            NeowBlessing::RemoveCard => DeckSystem::choose_card_to_remove(self.comms, pps),
            NeowBlessing::ReplaceStarterRelic => {
                let replacement_relic = self.neow_generator.boss_relic();
                RelicSystem::replace_relic(self.comms, pps, self.starting_relic, replacement_relic)
            }
            NeowBlessing::TransformCard => todo!(),
            NeowBlessing::UpgradeCard => todo!(),
            NeowBlessing::Composite(bonus, penalty) => {
                match penalty {
                    NeowPenalty::DecreaseMaxHpByTenPercent => {
                        HealthSystem::decrease_hp_max(self.comms, pps, pps.hp_max / 10)?;
                    }
                    NeowPenalty::LoseAllGold => {
                        GoldSystem::decrease_gold(self.comms, pps, pps.gold)?;
                    }
                    NeowPenalty::ObtainCurse => {
                        DeckSystem::obtain_card(self.comms, pps, self.neow_generator.one_curse())?;
                    }
                    NeowPenalty::TakeDamage => {
                        HealthSystem::decrease_hp(self.comms, pps, pps.hp / 10 * 3)?;
                    }
                }
                match bonus {
                    NeowBonus::ChooseRareCard => todo!(),
                    NeowBonus::ChooseRareColorlessCard => todo!(),
                    NeowBonus::GainTwoHundredFiftyGold => {
                        GoldSystem::increase_gold(self.comms, pps, 250)
                    }
                    NeowBonus::IncreaseMaxHpByTwentyPercent => {
                        HealthSystem::increase_hp_max(self.comms, pps, pps.hp_max / 5)
                    }
                    NeowBonus::ObtainRandomRareRelic => todo!(),
                    NeowBonus::RemoveTwoCards => todo!(),
                    NeowBonus::TransformTwoCards => todo!(),
                }
            }
        }
    }
}
