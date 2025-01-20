use crate::data::{Card, Character, CURSE_CARD_POOL, UNCOMMON_COLORLESS_CARDS};

use super::{Seed, StsRandom};

pub struct CardGenerator {
    character: &'static Character,
    card_rng: StsRandom,

    // Kudos to gamerpuppy for figuring out how this rarity_bias business works
    // (called `cardRarityFactor` in their code).
    rarity_bias: i32,
}

impl CardGenerator {
    pub fn new(seed: Seed, character: &'static Character) -> Self {
        Self {
            character,
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

    pub fn three_card_choices(&mut self) -> Vec<Card> {
        let mut result: Vec<Card> = Vec::with_capacity(3);
        for i in 0..3 {
            let pool = self.pool_for_class();
            let mut card = self.card_rng.choose(pool);
            while result.contains(card) {
                card = self.card_rng.choose(pool);
            }
            println!(
                "card reward {} assigned with card_rng counter {}",
                i,
                self.card_rng.get_counter()
            );
            result.push(*card);
        }
        result
    }

    fn pool_for_class(&mut self) -> &'static [Card] {
        let roll = self.card_rng.gen_range(0..100) + self.rarity_bias;
        println!(
            "roll: {}, counter: {}, seed: {}",
            roll,
            self.card_rng.get_counter(),
            self.card_rng.get_initial_seed()
        );
        if roll < 3 {
            self.rarity_bias = 5;
            self.character.rare_card_pool
        } else if roll < 40 {
            self.character.uncommon_card_pool
        } else {
            self.rarity_bias = (self.rarity_bias - 1).max(-40);
            self.character.common_card_pool
        }
    }
}
