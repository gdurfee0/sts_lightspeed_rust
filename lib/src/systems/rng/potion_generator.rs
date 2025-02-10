use crate::data::{Character, Potion, PotionRarity};

use super::seed::Seed;
use super::sts_random::StsRandom;

pub struct PotionGenerator {
    character: &'static Character,
    potion_rng: StsRandom,
    potion_awarded_d100_threshold: i32,
}

impl PotionGenerator {
    pub fn new(seed: Seed, character: &'static Character) -> Self {
        Self {
            character,
            potion_rng: StsRandom::from(seed),
            potion_awarded_d100_threshold: 40,
        }
    }

    pub fn gen_potions(&mut self, count: usize) -> Vec<Potion> {
        let mut result: Vec<Potion> = Vec::with_capacity(count);
        for _ in 0..count {
            let potion = self.potion_rng.choose(self.character.potion_pool);
            result.push(*potion);
        }
        result
    }

    pub fn combat_reward(&mut self) -> Option<Potion> {
        let potion_awarded_d100 = self.potion_rng.gen_range(0..100);
        if potion_awarded_d100 < self.potion_awarded_d100_threshold {
            self.potion_awarded_d100_threshold -= 10;
            let potion_rarity_d100 = self.potion_rng.gen_range(0..100);
            let target_rarity = if potion_rarity_d100 < 65 {
                PotionRarity::Common
            } else if potion_rarity_d100 < 90 {
                PotionRarity::Uncommon
            } else {
                PotionRarity::Rare
            };
            // Lol, this is quite the hack, but it's what the game does. Great job figuring
            // this one out, gamerpuppy!
            let mut potion = *self.potion_rng.choose(self.character.potion_pool);
            while potion.rarity() != target_rarity {
                potion = *self.potion_rng.choose(self.character.potion_pool);
            }
            Some(potion)
        } else {
            self.potion_awarded_d100_threshold += 10;
            None
        }
    }
}
