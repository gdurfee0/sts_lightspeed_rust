use crate::data::{
    Card, Character, NeowBlessing, Potion, Relic, FIRST_NEOW_POOL, SECOND_NEOW_POOL,
    THIRD_NEOW_POOL,
};

use super::card_generator::CardGenerator;
use super::potion_generator::PotionGenerator;
use super::relic_generator::RelicGenerator;
use super::seed::Seed;
use super::sts_random::StsRandom;

pub struct NeowGenerator<'a> {
    character: &'static Character,
    neow_rng: StsRandom,
    card_generator: &'a mut CardGenerator,
    potion_generator: &'a mut PotionGenerator,
    relic_generator: &'a mut RelicGenerator,
    blessing_choices: [NeowBlessing; 4],
}

impl<'a> NeowGenerator<'a> {
    pub fn new(
        seed: Seed,
        character: &'static Character,
        card_generator: &'a mut CardGenerator,
        potion_generator: &'a mut PotionGenerator,
        relic_generator: &'a mut RelicGenerator,
    ) -> Self {
        let mut neow_rng = StsRandom::from(seed);
        let first_blessing = *neow_rng.choose(FIRST_NEOW_POOL);
        let second_blessing = *neow_rng.choose(SECOND_NEOW_POOL);
        let penalty_and_bonuses = neow_rng.choose(THIRD_NEOW_POOL);
        let penalty = penalty_and_bonuses.0;
        let bonus = *neow_rng.choose(penalty_and_bonuses.1);
        let blessing_choices = [
            first_blessing,
            second_blessing,
            NeowBlessing::Composite(bonus, penalty),
            NeowBlessing::ReplaceStarterRelic,
        ];
        // Reference code advances the rng an extra tick, so so shall we.
        neow_rng.advance();
        Self {
            character,
            neow_rng,
            card_generator,
            potion_generator,
            relic_generator,
            blessing_choices,
        }
    }

    pub fn blessing_choices(&mut self) -> &[NeowBlessing; 4] {
        &self.blessing_choices
    }

    pub fn three_card_choices(&mut self) -> Vec<Card> {
        let mut result: Vec<Card> = Vec::with_capacity(3);
        let pools = &[
            (self.character.uncommon_card_pool, 0.33), // Should this be 1. / 3. instead?
            (self.character.common_card_pool, 0.67),
        ];
        for _ in 0..3 {
            let pool = self.neow_rng.weighted_choose(pools);
            let mut card = self.neow_rng.choose(pool);
            while result.contains(card) {
                card = self.neow_rng.choose(pool);
            }
            result.push(*card);
        }
        result
    }

    pub fn three_colorless_card_choices(&mut self) -> Vec<Card> {
        // Intentionally using card_generator here for fidelity to the original game
        self.card_generator.three_colorless_card_choices()
    }

    pub fn one_random_rare_card(&mut self) -> Card {
        *self.neow_rng.choose(self.character.rare_card_pool)
    }

    pub fn one_curse(&mut self) -> Card {
        // Intentionally using card_generator here for fidelity to the original game
        self.card_generator.one_curse()
    }

    pub fn common_relic(&mut self) -> Relic {
        self.relic_generator.common_relic()
    }

    pub fn boss_relic(&mut self) -> Relic {
        self.relic_generator.boss_relic()
    }

    pub fn three_random_potions(&mut self) -> Vec<Potion> {
        let _ = self.card_generator.combat_rewards(); // For fidelity to the game's rng
        self.potion_generator.gen_potions(3)
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use crate::data::{Act, NeowBonus, NeowPenalty, DEFECT, IRONCLAD, SILENT, WATCHER};

    use super::*;

    struct NeowGeneratorEnvironment {
        seed: Seed,
        card_generator: CardGenerator,
        potion_generator: PotionGenerator,
        relic_generator: RelicGenerator,
    }

    impl NeowGeneratorEnvironment {
        fn new(seed: Seed) -> Self {
            let card_generator = CardGenerator::new(seed, IRONCLAD, Act::get(1));
            let potion_generator = PotionGenerator::new(seed, IRONCLAD);
            let relic_generator = RelicGenerator::new(seed, IRONCLAD);
            Self {
                seed,
                card_generator,
                potion_generator,
                relic_generator,
            }
        }

        fn generator(&mut self, character: &'static Character) -> NeowGenerator {
            NeowGenerator::new(
                self.seed,
                character,
                &mut self.card_generator,
                &mut self.potion_generator,
                &mut self.relic_generator,
            )
        }
    }

    #[test]
    fn test_blessing_choices() {
        let mut nge = NeowGeneratorEnvironment::new(3.into());
        let mut generator = nge.generator(IRONCLAD);
        assert_eq!(
            generator.blessing_choices().to_vec(),
            vec![
                NeowBlessing::ChooseCard,
                NeowBlessing::GainOneHundredGold,
                NeowBlessing::Composite(
                    NeowBonus::GainTwoHundredFiftyGold,
                    NeowPenalty::TakeDamage,
                ),
                NeowBlessing::ReplaceStarterRelic
            ]
        );
        let mut nge = NeowGeneratorEnvironment::new(15.into());
        let mut generator = nge.generator(IRONCLAD);
        assert_eq!(
            generator.blessing_choices().to_vec(),
            vec![
                NeowBlessing::ChooseColorlessCard,
                NeowBlessing::IncreaseMaxHpByTenPercent,
                NeowBlessing::Composite(NeowBonus::TransformTwoCards, NeowPenalty::TakeDamage),
                NeowBlessing::ReplaceStarterRelic
            ]
        );
    }

    #[test]
    fn test_three_cards() {
        let mut nge = NeowGeneratorEnvironment::new(3.into());
        let mut generator = nge.generator(IRONCLAD);
        assert_eq!(
            generator.three_card_choices(),
            vec![
                Card::SeeingRed(false),
                Card::Clothesline(false),
                Card::BloodForBlood(false)
            ]
        );
        let mut nge = NeowGeneratorEnvironment::new(40.into());
        let mut generator = nge.generator(IRONCLAD);
        assert_eq!(
            generator.three_card_choices(),
            vec![
                Card::IronWave(false),
                Card::Cleave(false),
                Card::Headbutt(false)
            ]
        );
        let mut nge = NeowGeneratorEnvironment::new(3.into());
        let mut generator = nge.generator(SILENT);
        assert_eq!(
            generator.three_card_choices(),
            vec![Card::Dash(false), Card::Backflip(false), Card::Choke(false)]
        );
        let mut nge = NeowGeneratorEnvironment::new(3.into());
        let mut generator = nge.generator(DEFECT);
        assert_eq!(
            generator.three_card_choices(),
            vec![
                Card::Equilibrium(false),
                Card::CompileDriver(false),
                Card::Aggregate(false)
            ]
        );
        let mut nge = NeowGeneratorEnvironment::new(3.into());
        let mut generator = nge.generator(WATCHER);
        assert_eq!(
            generator.three_card_choices(),
            vec![
                Card::Worship(false),
                Card::CutThroughFate(false),
                Card::WheelKick(false)
            ]
        );
        let mut nge = NeowGeneratorEnvironment::new(40.into());
        let mut generator = nge.generator(WATCHER);
        assert_eq!(
            generator.three_card_choices(),
            vec![
                Card::FlyingSleeves(false),
                Card::Tranquility(false),
                Card::Evaluate(false)
            ]
        );
    }

    #[test]
    fn test_one_random_rare_card() {
        let mut nge = NeowGeneratorEnvironment::new(2.into());
        let mut generator = nge.generator(IRONCLAD);
        assert_eq!(generator.one_random_rare_card(), Card::DemonForm(false));
        let mut nge = NeowGeneratorEnvironment::new(13.into());
        let mut generator = nge.generator(IRONCLAD);
        assert_eq!(generator.one_random_rare_card(), Card::Reaper(false));
    }

    #[test]
    fn test_one_curse() {
        let mut nge = NeowGeneratorEnvironment::new(2.into());
        let mut generator = nge.generator(IRONCLAD);
        assert_eq!(generator.one_curse(), Card::Clumsy);
        let mut nge = NeowGeneratorEnvironment::new(11.into());
        let mut generator = nge.generator(IRONCLAD);
        assert_eq!(generator.one_curse(), Card::Clumsy);
        let mut nge = NeowGeneratorEnvironment::new(12.into());
        let mut generator = nge.generator(IRONCLAD);
        assert_eq!(generator.one_curse(), Card::Decay); // TODO: Writhe?
        let mut nge = NeowGeneratorEnvironment::new(13.into());
        let mut generator = nge.generator(IRONCLAD);
        assert_eq!(generator.one_curse(), Card::Parasite);
    }

    #[test]
    fn test_three_random_potions() {
        let mut nge = NeowGeneratorEnvironment::new(2.into());
        let mut generator = nge.generator(IRONCLAD);
        assert_eq!(
            generator.three_random_potions(),
            vec![
                Potion::DexterityPotion,
                Potion::EnergyPotion,
                Potion::DexterityPotion
            ]
        );
    }

    #[test]
    fn test_seed_1_ironclad() {
        let mut nge = NeowGeneratorEnvironment::new(1.into());
        let mut neow_generator = nge.generator(IRONCLAD);
        assert_eq!(
            neow_generator.blessing_choices().to_vec(),
            vec![
                NeowBlessing::ChooseColorlessCard,
                NeowBlessing::ObtainThreeRandomPotions,
                NeowBlessing::Composite(
                    NeowBonus::ChooseRareColorlessCard,
                    NeowPenalty::DecreaseMaxHpByTenPercent
                ),
                NeowBlessing::ReplaceStarterRelic
            ]
        );
        assert_eq!(neow_generator.boss_relic(), Relic::SneckoEye);
    }
}
