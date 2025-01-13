use anyhow::Error;

use super::player::Player;

use crate::data::{
    Character, NeowBlessing, NeowBonus, NeowPenalty, Relic, UNCOMMON_COLORLESS_CARDS,
};
use crate::rng::{NeowGenerator, RelicGenerator, Seed, StsRandom};

pub struct NeowSimulator<'a> {
    // Information typically set on the command line
    character: &'static Character,

    // Random number generators for various game elements
    neow_generator: NeowGenerator<'a>,
    potion_sts_random: &'a mut StsRandom,
    relic_generator: &'a mut RelicGenerator,

    // Current player state
    player: &'a mut Player,
}

impl<'a> NeowSimulator<'a> {
    pub fn new(
        seed: Seed,
        character: &'static Character,
        card_sts_random: &'a mut StsRandom,
        potion_sts_random: &'a mut StsRandom,
        relic_generator: &'a mut RelicGenerator,
        player: &'a mut Player,
    ) -> Self {
        let neow_generator = NeowGenerator::new(&seed, character, card_sts_random);
        Self {
            character,
            neow_generator,
            potion_sts_random,
            relic_generator,
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
                .choose_one_card(self.neow_generator.three_card_choices()),
            NeowBlessing::ChooseColorlessCard => self
                .player
                .choose_one_card(self.neow_generator.three_colorless_card_choices()),
            NeowBlessing::GainOneHundredGold => self.player.increase_gold(100),
            NeowBlessing::IncreaseMaxHpByTenPercent => {
                self.player.increase_hp_max(self.player.hp_max() / 10)
            }
            NeowBlessing::NeowsLament => self.player.obtain_relic(Relic::NeowsLament),
            NeowBlessing::ObtainRandomCommonRelic => self
                .player
                .obtain_relic(self.relic_generator.common_relic()),
            NeowBlessing::ObtainRandomRareCard => self
                .player
                .obtain_card(self.neow_generator.one_random_rare_card()),
            NeowBlessing::ObtainThreeRandomPotions => self.player.choose_many_potions(
                self.potion_sts_random
                    .sample_without_replacement(self.character.potion_pool, 3),
            ),
            NeowBlessing::RemoveCard => self.player.remove_one_card(),
            NeowBlessing::ReplaceStarterRelic => todo!(),
            NeowBlessing::TransformCard => todo!(),
            NeowBlessing::UpgradeCard => todo!(),
            NeowBlessing::Composite(bonus, penalty) => {
                match penalty {
                    NeowPenalty::DecreaseMaxHpByTenPercent => {
                        self.player.decrease_hp_max(self.player.hp_max() / 10)?;
                    }
                    NeowPenalty::LoseAllGold => {
                        self.player.decrease_gold(self.player.gold())?;
                    }
                    NeowPenalty::ObtainCurse => {
                        self.player.obtain_card(self.neow_generator.one_curse())?;
                    }
                    NeowPenalty::TakeDamage => {
                        self.player.take_damage(self.player.hp() / 10 * 3)?;
                    }
                }
                match bonus {
                    NeowBonus::ChooseRareCard => todo!(),
                    NeowBonus::ChooseRareColorlessCard => todo!(),
                    NeowBonus::GainTwoHundredFiftyGold => self.player.increase_gold(250),
                    NeowBonus::IncreaseMaxHpByTwentyPercent => {
                        self.player.increase_hp_max(self.player.hp_max() / 5)
                    }
                    NeowBonus::ObtainRandomRareRelic => todo!(),
                    NeowBonus::RemoveTwoCards => todo!(),
                    NeowBonus::TransformTwoCards => todo!(),
                }
            }
        }
    }
}
