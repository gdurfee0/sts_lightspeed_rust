use std::collections::VecDeque;

use crate::data::{Character, Relic};

use super::{Seed, StsRandom};

pub struct RelicGenerator {
    sts_random: StsRandom,
    common_relic_pool: VecDeque<Relic>,
    uncommon_relic_pool: VecDeque<Relic>,
    rare_relic_pool: VecDeque<Relic>,
    shop_relic_pool: VecDeque<Relic>,
    boss_relic_pool: VecDeque<Relic>,
}

impl RelicGenerator {
    pub fn new(seed: &Seed, character: &'static Character) -> Self {
        let mut sts_random: StsRandom = seed.into();
        let mut common_relic_pool = character.common_relic_pool.to_vec();
        let mut uncommon_relic_pool = character.uncommon_relic_pool.to_vec();
        let mut rare_relic_pool = character.rare_relic_pool.to_vec();
        let mut shop_relic_pool = character.shop_relic_pool.to_vec();
        let mut boss_relic_pool = character.boss_relic_pool.to_vec();
        sts_random.java_compat_shuffle(&mut common_relic_pool);
        sts_random.java_compat_shuffle(&mut uncommon_relic_pool);
        sts_random.java_compat_shuffle(&mut rare_relic_pool);
        sts_random.java_compat_shuffle(&mut shop_relic_pool);
        sts_random.java_compat_shuffle(&mut boss_relic_pool);
        Self {
            sts_random,
            common_relic_pool: common_relic_pool.into_iter().collect(),
            uncommon_relic_pool: uncommon_relic_pool.into_iter().collect(),
            rare_relic_pool: rare_relic_pool.into_iter().collect(),
            shop_relic_pool: shop_relic_pool.into_iter().collect(),
            boss_relic_pool: boss_relic_pool.into_iter().collect(),
        }
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_relic_generator() {
        let mut generator = RelicGenerator::new(&12u64.into(), "i".try_into().unwrap());
        println!("{:?}", generator.common_relic_pool);
        assert_eq!(
            generator.common_relic_pool.pop_front(),
            Some(Relic::RedSkull)
        );
        let mut generator = RelicGenerator::new(&1u64.into(), "i".try_into().unwrap());
        println!("{:?}", generator.boss_relic_pool);
        assert_eq!(
            generator.boss_relic_pool.pop_front(),
            Some(Relic::SneckoEye)
        );
        let mut generator = RelicGenerator::new(&2u64.into(), "i".try_into().unwrap());
        println!("{:?}", generator.boss_relic_pool);
        assert_eq!(
            generator.boss_relic_pool.pop_front(),
            Some(Relic::RunicDome)
        );
    }
}
