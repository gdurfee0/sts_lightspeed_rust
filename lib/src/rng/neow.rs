use crate::data::card::{Card, CURSE_CARD_POOL, UNCOMMON_COLORLESS_CARDS};
use crate::data::character::Character;
use crate::data::neow::{NeowBlessing, FIRST_NEOW_POOL, SECOND_NEOW_POOL, THIRD_NEOW_POOL};

use super::{Seed, StsRandom};

pub struct NeowGenerator<'a> {
    neow_rng: StsRandom,
    card_rng: &'a mut StsRandom,
    character: &'static Character,
    blessing_choices: [NeowBlessing; 4],
}

impl<'a> NeowGenerator<'a> {
    pub fn new(seed: Seed, character: &'static Character, card_rng: &'a mut StsRandom) -> Self {
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
            neow_rng,
            card_rng,
            character,
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
        // Intentionally using card_rng here for fidelity to the original game
        self.card_rng
            .sample_without_replacement(UNCOMMON_COLORLESS_CARDS, 3)
    }

    pub fn one_random_rare_card(&mut self) -> Card {
        let pool = self.character.rare_card_pool;
        *self.neow_rng.choose(pool)
    }

    pub fn one_curse(&mut self) -> Card {
        // Intentionally using card_rng here for fidelity to the original game
        *self.card_rng.choose(CURSE_CARD_POOL)
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use crate::data::character::{DEFECT, IRONCLAD, SILENT, WATCHER};
    use crate::data::neow::{NeowBonus, NeowPenalty};

    use super::*;

    #[test]
    fn test_blessing_choices() {
        let seed = 3.into();
        let mut card_rng = StsRandom::from(seed);
        let mut generator = NeowGenerator::new(seed, IRONCLAD, &mut card_rng);
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

        let seed = 15.into();
        let mut card_rng = StsRandom::from(seed);
        let mut generator = NeowGenerator::new(seed, IRONCLAD, &mut card_rng);
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
        let seed = 3.into();
        let mut card_rng = StsRandom::from(seed);
        let mut generator = NeowGenerator::new(seed, IRONCLAD, &mut card_rng);
        assert_eq!(
            generator.three_card_choices(),
            vec![Card::SeeingRed, Card::Clothesline, Card::BloodForBlood]
        );
        let seed = 40.into();
        let mut card_rng = StsRandom::from(seed);
        let mut generator = NeowGenerator::new(seed, IRONCLAD, &mut card_rng);
        assert_eq!(
            generator.three_card_choices(),
            vec![Card::IronWave, Card::Cleave, Card::Headbutt]
        );
        let seed = 3.into();
        let mut card_rng = StsRandom::from(seed);
        let mut generator = NeowGenerator::new(seed, SILENT, &mut card_rng);
        assert_eq!(
            generator.three_card_choices(),
            vec![Card::Dash, Card::Backflip, Card::Choke]
        );
        let mut card_rng = StsRandom::from(seed);
        let mut generator = NeowGenerator::new(seed, DEFECT, &mut card_rng);
        assert_eq!(
            generator.three_card_choices(),
            vec![Card::Equilibrium, Card::CompileDriver, Card::Aggregate]
        );
        let mut card_rng = StsRandom::from(seed);
        let mut generator = NeowGenerator::new(seed, WATCHER, &mut card_rng);
        assert_eq!(
            generator.three_card_choices(),
            vec![Card::Worship, Card::CutThroughFate, Card::WheelKick]
        );
        let seed = 40.into();
        let mut card_rng = StsRandom::from(seed);
        let mut generator = NeowGenerator::new(seed, WATCHER, &mut card_rng);
        assert_eq!(
            generator.three_card_choices(),
            vec![Card::FlyingSleeves, Card::Tranquility, Card::Evaluate]
        );
    }

    #[test]
    fn test_one_random_rare_card() {
        let seed = 2.into();
        let mut card_rng = StsRandom::from(seed);
        let mut generator = NeowGenerator::new(seed, IRONCLAD, &mut card_rng);
        assert_eq!(generator.one_random_rare_card(), Card::DemonForm);
        let seed = 13.into();
        let mut card_rng = StsRandom::from(seed);
        let mut generator = NeowGenerator::new(seed, IRONCLAD, &mut card_rng);
        assert_eq!(generator.one_random_rare_card(), Card::Reaper);
    }

    #[test]
    fn test_one_curse() {
        let seed = 2.into();
        let mut card_rng = StsRandom::from(seed);
        let mut generator = NeowGenerator::new(seed, IRONCLAD, &mut card_rng);
        assert_eq!(generator.one_curse(), Card::Clumsy);
        let seed = 11.into();
        let mut card_rng = StsRandom::from(seed);
        let mut generator = NeowGenerator::new(seed, IRONCLAD, &mut card_rng);
        assert_eq!(generator.one_curse(), Card::Clumsy);
        let seed = 12.into();
        let mut card_rng = StsRandom::from(seed);
        let mut generator = NeowGenerator::new(seed, IRONCLAD, &mut card_rng);
        assert_eq!(generator.one_curse(), Card::Decay); // TODO: Writhe?
        let seed = 13.into();
        let mut card_rng = StsRandom::from(seed);
        let mut generator = NeowGenerator::new(seed, IRONCLAD, &mut card_rng);
        assert_eq!(generator.one_curse(), Card::Parasite);
    }
}
