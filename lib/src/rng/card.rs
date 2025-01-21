use crate::data::{Act, Card, Character, CURSE_CARD_POOL, UNCOMMON_COLORLESS_CARDS};

use super::{Seed, StsRandom};

pub struct CardGenerator {
    character: &'static Character,
    upgrade_probability: f32,
    card_rng: StsRandom,

    // Kudos to gamerpuppy for figuring out how this rarity_bias business works
    // (called `cardRarityFactor` in their code).
    rarity_bias: i32,
}

impl CardGenerator {
    pub fn new(seed: Seed, character: &'static Character, act: &'static Act) -> Self {
        let rare_prob = match act.number {
            1 => 0.0f32,
            2 => 0.25f32,
            3 | 4 => 0.5f32,
            _ => unreachable!(),
        };
        Self {
            character,
            upgrade_probability: rare_prob,
            card_rng: StsRandom::from(seed),
            rarity_bias: 5,
        }
    }

    pub fn three_colorless_card_choices(&mut self) -> Vec<Card> {
        self.card_rng
            .sample_without_replacement(UNCOMMON_COLORLESS_CARDS, 3)
    }

    pub fn one_curse(&mut self) -> Card {
        *self.card_rng.choose(CURSE_CARD_POOL)
    }

    pub fn combat_rewards(&mut self) -> Vec<Card> {
        let mut result: Vec<Card> = Vec::with_capacity(3);
        let mut is_rare_vec: Vec<bool> = Vec::with_capacity(3);
        for _ in 0..3 {
            let is_rare_and_pool = self.pool_for_class();
            is_rare_vec.push(is_rare_and_pool.0);
            let pool = is_rare_and_pool.1;
            let mut card = self.card_rng.choose(pool);
            while result.contains(card) {
                card = self.card_rng.choose(pool);
            }
            result.push(*card);
        }
        for is_rare in is_rare_vec {
            let mut should_upgrade = *self
                .card_rng
                .weighted_choose(&[(true, self.upgrade_probability), (false, 1.0)]);
            should_upgrade = should_upgrade && !is_rare;
            if should_upgrade {
                todo!();
            }
        }
        result
    }

    fn pool_for_class(&mut self) -> (bool, &'static [Card]) {
        let roll = self.card_rng.gen_range(0..100) + self.rarity_bias;
        if roll < 3 {
            self.rarity_bias = 5;
            (true, self.character.rare_card_pool)
        } else if roll < 40 {
            (false, self.character.uncommon_card_pool)
        } else {
            self.rarity_bias = (self.rarity_bias - 1).max(-40);
            (false, self.character.common_card_pool)
        }
    }
}
