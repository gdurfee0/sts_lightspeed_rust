use crate::data::{
    Card, Character, NeowBlessing, FIRST_NEOW_POOL, SECOND_NEOW_POOL, THIRD_NEOW_POOL,
    UNCOMMON_COLORLESS_CARDS,
};

use super::{Seed, StsRandom};

pub struct NeowGenerator {
    sts_random: StsRandom,
    character: &'static Character,
    blessing_choices: [NeowBlessing; 4],
}

impl NeowGenerator {
    pub fn new(seed: &Seed, character: &'static Character) -> Self {
        let mut sts_random: StsRandom = seed.into();
        let first_blessing = *sts_random.choose(FIRST_NEOW_POOL);
        let second_blessing = *sts_random.choose(SECOND_NEOW_POOL);
        let penalty_and_bonuses = sts_random.choose(THIRD_NEOW_POOL);
        let penalty = penalty_and_bonuses.0;
        let bonus = *sts_random.choose(penalty_and_bonuses.1);
        let blessing_choices = [
            first_blessing,
            second_blessing,
            NeowBlessing::Composite(bonus, penalty),
            NeowBlessing::ReplaceStarterRelic,
        ];
        // Reference code advances the rng an extra tick, so so shall we.
        sts_random.advance();
        Self {
            sts_random,
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
            let pool = self.sts_random.weighted_choose(pools);
            let mut card = self.sts_random.choose(pool);
            while result.contains(card) {
                card = self.sts_random.choose(pool);
            }
            result.push(*card);
        }
        result
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use crate::data::{NeowBonus, NeowPenalty};

    use super::*;

    #[test]
    fn test_blessing_choices() {
        let seed = 3u64.into();
        let character = <&'static Character>::try_from("i").unwrap();
        let mut generator = NeowGenerator::new(&seed, character);
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

        let seed = 15u64.into();
        let mut generator = NeowGenerator::new(&seed, character);
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
        let mut generator = NeowGenerator::new(&3.into(), "i".try_into().unwrap());
        assert_eq!(
            generator.three_card_choices(),
            vec![Card::SeeingRed, Card::Clothesline, Card::BloodForBlood]
        );
        let mut generator = NeowGenerator::new(&40.into(), "i".try_into().unwrap());
        assert_eq!(
            generator.three_card_choices(),
            vec![Card::IronWave, Card::Cleave, Card::Headbutt]
        );
        let mut generator = NeowGenerator::new(&3.into(), "s".try_into().unwrap());
        assert_eq!(
            generator.three_card_choices(),
            vec![Card::Dash, Card::Backflip, Card::Choke]
        );
        let mut generator = NeowGenerator::new(&3.into(), "d".try_into().unwrap());
        assert_eq!(
            generator.three_card_choices(),
            vec![Card::Equilibrium, Card::CompileDriver, Card::Aggregate]
        );
        let mut generator = NeowGenerator::new(&3.into(), "w".try_into().unwrap());
        assert_eq!(
            generator.three_card_choices(),
            vec![Card::Worship, Card::CutThroughFate, Card::WheelKick]
        );
        let mut generator = NeowGenerator::new(&"15".try_into().unwrap(), "w".try_into().unwrap());
        assert_eq!(
            generator.three_card_choices(),
            vec![Card::FlyingSleeves, Card::Tranquility, Card::Evaluate]
        );
    }
}
