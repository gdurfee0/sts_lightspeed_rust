use anyhow::Error;

use crate::data::{Character, NeowBlessing, NeowBonus, NeowPenalty, Relic};
use crate::systems::rng::{CardGenerator, NeowGenerator, RelicGenerator, Seed, StsRandom};

use super::player::Player;

pub struct NeowSimulator<'a> {
    neow_generator: NeowGenerator<'a>,
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
        Self {
            neow_generator,
            player,
        }
    }

    pub fn run(mut self) -> Result<(), Error> {
        let blessing_choices = self.neow_generator.blessing_choices();
        let blessing_choice = self.player.choose_neow_blessing(blessing_choices)?;
        self.handle_neow_blessing(blessing_choice)
    }

    fn handle_neow_blessing(&mut self, blessing: NeowBlessing) -> Result<(), Error> {
        match blessing {
            NeowBlessing::ChooseCard => self
                .player
                .choose_card_to_obtain(&self.neow_generator.three_card_choices()),
            NeowBlessing::ChooseColorlessCard => self
                .player
                .choose_card_to_obtain(&self.neow_generator.three_colorless_card_choices()),
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
            NeowBlessing::RemoveCard => self.player.choose_card_to_remove(),
            NeowBlessing::ReplaceStarterRelic => todo!(),
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
