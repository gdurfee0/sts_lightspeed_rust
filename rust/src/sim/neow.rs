use anyhow::Error;

use super::player::Player;

use crate::data::{
    Character, NeowBlessing, NeowBonus, NeowPenalty, Relic, UNCOMMON_COLORLESS_CARDS,
};
use crate::rng::{NeowGenerator, Seed, StsRandom};

pub struct NeowSimulator<'a> {
    // Information typically set on the command line
    character: &'static Character,

    // Random number generators for various game elements
    neow_generator: NeowGenerator,
    card_sts_random: &'a mut StsRandom,
    potion_sts_random: &'a mut StsRandom,

    // Current player state
    player: &'a mut Player,
}

impl<'a> NeowSimulator<'a> {
    pub fn new(
        seed: Seed,
        character: &'static Character,
        card_sts_random: &'a mut StsRandom,
        potion_sts_random: &'a mut StsRandom,
        player: &'a mut Player,
    ) -> Self {
        let neow_generator = NeowGenerator::new(&seed, character);
        Self {
            character,
            neow_generator,
            card_sts_random,
            potion_sts_random,
            player,
        }
    }

    pub fn run(mut self) -> Result<(), Error> {
        let blessing_choices = self.neow_generator.blessing_choices();
        let blessing_choice = self.player.choose_neow_blessing(blessing_choices)?;
        self.handle_neow_blessing(blessing_choice)?;
        self.player.send_relics()?;
        self.player.send_deck()?;
        self.player.send_player_view()
    }

    fn handle_neow_blessing(&mut self, blessing: NeowBlessing) -> Result<(), Error> {
        match blessing {
            NeowBlessing::ChooseCard => self
                .player
                .choose_one_card(self.neow_generator.three_card_choices()),
            NeowBlessing::ChooseColorlessCard => self.player.choose_one_card(
                // Intentionally using the card_sts_random generator here for fidelity
                self.card_sts_random
                    .sample_without_replacement(UNCOMMON_COLORLESS_CARDS, 3),
            ),
            NeowBlessing::GainOneHundredGold => {
                self.player.gold += 100;
                Ok(())
            }
            NeowBlessing::IncreaseMaxHpByTenPercent => {
                self.player.hp_max += self.player.hp_max / 10;
                self.player.hp = self.player.hp_max;
                Ok(())
            }
            NeowBlessing::NeowsLament => {
                self.player.relics.push(Relic::NeowsLament);
                self.player.send_relics()
            }
            NeowBlessing::ObtainRandomCommonRelic => todo!(),
            NeowBlessing::ObtainRandomRareCard => todo!(),
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
                        self.player.hp_max -= self.player.hp_max / 10;
                        self.player.hp = self.player.hp_max;
                    }
                    NeowPenalty::LoseAllGold => {
                        self.player.gold = 0;
                    }
                    NeowPenalty::ObtainCurse => todo!(),
                    NeowPenalty::TakeDamage => {
                        self.player.hp -= (self.player.hp / 10) * 3;
                    }
                }
                match bonus {
                    NeowBonus::ChooseRareCard => todo!(),
                    NeowBonus::ChooseRareColorlessCard => todo!(),
                    NeowBonus::GainTwoHundredFiftyGold => {
                        self.player.gold += 250;
                        Ok(())
                    }
                    NeowBonus::IncreaseMaxHpByTwentyPercent => {
                        self.player.hp_max += self.player.hp_max / 5;
                        self.player.hp = self.player.hp_max;
                        Ok(())
                    }
                    NeowBonus::ObtainRandomRareRelic => todo!(),
                    NeowBonus::RemoveTwoCards => todo!(),
                    NeowBonus::TransformTwoCards => todo!(),
                }
            }
        }
    }
}
