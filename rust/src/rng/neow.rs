use crate::data::{NeowBlessing, FIRST_NEOW_POOL, SECOND_NEOW_POOL, THIRD_NEOW_POOL};

use super::{Seed, StsRandom};

pub struct NeowGenerator {
    sts_random: StsRandom,
    blessing_choices: [NeowBlessing; 4],
}

impl NeowGenerator {
    pub fn new(seed: &Seed) -> Self {
        let mut sts_random: StsRandom = seed.into();
        let first_blessing = *sts_random.choose(FIRST_NEOW_POOL);
        let second_blessing = *sts_random.choose(SECOND_NEOW_POOL);
        let drawback_and_benefits = sts_random.choose(THIRD_NEOW_POOL);
        let drawback = drawback_and_benefits.0;
        let benefit = *sts_random.choose(drawback_and_benefits.1);
        let blessing_choices = [
            first_blessing,
            second_blessing,
            NeowBlessing::Composite(benefit, drawback),
            NeowBlessing::ReplaceStarterRelic,
        ];
        Self {
            sts_random,
            blessing_choices,
        }
    }

    pub fn blessing_choices(&mut self) -> &[NeowBlessing; 4] {
        &self.blessing_choices
    }
}
