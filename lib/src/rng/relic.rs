use std::collections::VecDeque;

use crate::data::{Character, Relic};

use super::{Seed, StsRandom};

pub struct RelicGenerator {
    common_relic_pool: VecDeque<Relic>,
    uncommon_relic_pool: VecDeque<Relic>,
    rare_relic_pool: VecDeque<Relic>,
    _shop_relic_pool: VecDeque<Relic>,
    _boss_relic_pool: VecDeque<Relic>,
}

impl RelicGenerator {
    pub fn new(seed: Seed, character: &'static Character) -> Self {
        let mut common_relic_pool = character.common_relic_pool.to_vec();
        let mut uncommon_relic_pool = character.uncommon_relic_pool.to_vec();
        let mut rare_relic_pool = character.rare_relic_pool.to_vec();
        let mut shop_relic_pool = character.shop_relic_pool.to_vec();
        let mut boss_relic_pool = character.boss_relic_pool.to_vec();
        let mut relic_rng = StsRandom::from(seed);
        relic_rng.java_compat_shuffle(&mut common_relic_pool);
        relic_rng.java_compat_shuffle(&mut uncommon_relic_pool);
        relic_rng.java_compat_shuffle(&mut rare_relic_pool);
        relic_rng.java_compat_shuffle(&mut shop_relic_pool);
        relic_rng.java_compat_shuffle(&mut boss_relic_pool);
        Self {
            common_relic_pool: common_relic_pool.into_iter().collect(),
            uncommon_relic_pool: uncommon_relic_pool.into_iter().collect(),
            rare_relic_pool: rare_relic_pool.into_iter().collect(),
            _shop_relic_pool: shop_relic_pool.into_iter().collect(),
            _boss_relic_pool: boss_relic_pool.into_iter().collect(),
        }
    }

    // TODO: Add checks that the relic is valid for the current situation
    pub fn common_relic(&mut self) -> Relic {
        self.common_relic_pool
            .pop_front()
            .unwrap_or_else(|| self.uncommon_relic())
    }

    pub fn uncommon_relic(&mut self) -> Relic {
        self.uncommon_relic_pool
            .pop_front()
            .unwrap_or_else(|| self.rare_relic())
    }

    pub fn rare_relic(&mut self) -> Relic {
        self.rare_relic_pool.pop_front().unwrap_or(Relic::Circlet)
    }

    pub fn _shop_relic(&mut self) -> Relic {
        self._shop_relic_pool
            .pop_front()
            .unwrap_or_else(|| self.uncommon_relic())
    }

    pub fn _boss_relic(&mut self) -> Relic {
        self._boss_relic_pool.pop_front().unwrap_or(Relic::Circlet)
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use crate::data::IRONCLAD;

    use super::*;

    #[test]
    fn test_relic_generator() {
        let mut generator = RelicGenerator::new(12.into(), IRONCLAD);
        assert_eq!(generator.common_relic(), Relic::RedSkull);
        let mut generator = RelicGenerator::new(1.into(), IRONCLAD);
        assert_eq!(generator._boss_relic(), Relic::SneckoEye);
        let mut generator = RelicGenerator::new(2.into(), IRONCLAD);
        assert_eq!(generator._boss_relic(), Relic::RunicDome);
    }
}
