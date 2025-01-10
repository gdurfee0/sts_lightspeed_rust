use std::collections::VecDeque;

use crate::data::{Act, BossEncounter, EliteEncounter, MonsterEncounter};

use super::{Seed, StsRandom};

/// A struct that generates the encounters for a run based on the provided seed.
///
/// The encounters are generated in advance and stored in queues for easy retrieval.
/// The generator is stateful and resets with each new Act as the run progresses.
///
/// Side note: Ideally we would use an Iterator pattern here, with an Iterator type for each
/// encounter class (Monster, Elite, and Boss).  But for rng fidelity to the original game,
/// we need to share the same StsRandom generator across all encounter classes. This means that
/// the different Iterator types would need to hold mutable references to the same underlying
/// StsGenerator. This is an antipattern in Rust and would require use of RefCell or Mutex
/// to work around. So we're using a more manual approach here.
pub struct EnemyEncounterGenerator {
    act: &'static Act,
    sts_random: StsRandom,
    monster_queue: VecDeque<MonsterEncounter>,
    elite_queue: VecDeque<EliteEncounter>,
    boss_queue: VecDeque<BossEncounter>,
}

impl EnemyEncounterGenerator {
    pub fn new(seed: &Seed) -> Self {
        let act = Act::get(1);
        let sts_random: StsRandom = seed.into();
        let mut result = Self {
            act,
            sts_random,
            monster_queue: VecDeque::new(),
            elite_queue: VecDeque::new(),
            boss_queue: VecDeque::new(),
        };
        result.sample_all();
        result
    }

    /// Returns the next monster encounter in the queue, sampling more if necessary.
    pub fn next_monster_encounter(&mut self) -> MonsterEncounter {
        if let Some(encounter) = self.monster_queue.pop_front() {
            encounter
        } else {
            self.sample_strong_monster_encounters();
            self.next_monster_encounter()
        }
    }

    /// Returns the next elite encounter in the queue, sampling more if necessary.
    pub fn next_elite_encounter(&mut self) -> EliteEncounter {
        if let Some(encounter) = self.elite_queue.pop_front() {
            encounter
        } else {
            self.sample_elite_encounters();
            self.next_elite_encounter()
        }
    }

    /// Returns the next boss encounter in the queue.
    pub fn next_boss_encounter(&mut self) -> BossEncounter {
        if let Some(encounter) = self.boss_queue.pop_front() {
            encounter
        } else {
            panic!("It shouldn't be possible to run out of bosses in a run.");
        }
    }

    /// Advances the Act and samples new encounters for the queues.
    pub fn advance_act(&mut self) {
        self.act = self.act.next_act();
        self.sample_all();
    }

    // Samples all classes of encounters for the Act and adds them to the respective queues.
    fn sample_all(&mut self) {
        self.sample_weak_monster_encounters();
        self.sample_first_strong_monster_encounter();
        self.sample_strong_monster_encounters();
        self.sample_elite_encounters();
        self.sample_boss_encounters();
    }

    // Adds an Act-dependent number of weak monster encounters to the monster encounter queue.
    fn sample_weak_monster_encounters(&mut self) {
        self.sample_monster_encounters(
            self.act.weak_monster_encounter_count(),
            self.act.weak_monster_pool_with_probs(),
        );
    }

    // Adds the first strong monster encounter to the monster encounter queue, avoiding some
    // embarassing repetition.
    fn sample_first_strong_monster_encounter(&mut self) {
        let mut proposed_encounter = *self
            .sts_random
            .weighted_choose(self.act.strong_monster_pool_with_probs());
        while match self.monster_queue.back() {
            Some(prev_encounter) => matches!(
                (*prev_encounter, proposed_encounter),
                (
                    MonsterEncounter::SmallSlimes,
                    MonsterEncounter::LotsOfSlimes
                ) | (MonsterEncounter::SmallSlimes, MonsterEncounter::LargeSlime)
                    | (MonsterEncounter::TwoLice, MonsterEncounter::ThreeLice)
            ),
            None => false,
        } {
            proposed_encounter = *self
                .sts_random
                .weighted_choose(self.act.strong_monster_pool_with_probs());
        }
        self.monster_queue.push_back(proposed_encounter);
    }

    // Adds twelve strong monster encounters to the monster encounter queue.
    fn sample_strong_monster_encounters(&mut self) {
        self.sample_monster_encounters(12, self.act.strong_monster_pool_with_probs());
    }

    // Helper method that samples the specified number of encounters from the provided pool
    // and adds them to the monster encounter queue.
    //
    // Avoids repetition by examining the previous and doubly-previous encounters and avoiding
    // duplicates.
    fn sample_monster_encounters(
        &mut self,
        count: usize,
        pool: &'static [(MonsterEncounter, f32)],
    ) {
        for _ in 0..count {
            let mut proposed_encounter = *self.sts_random.weighted_choose(pool);
            while match self.monster_queue.back() {
                Some(prev_encounter) if self.monster_queue.len() >= 2 => {
                    proposed_encounter == *prev_encounter
                        || self
                            .monster_queue
                            .get(self.monster_queue.len() - 2)
                            .map_or(false, |prev_prev_encounter| {
                                proposed_encounter == *prev_prev_encounter
                            })
                }
                Some(prev_encounter) => proposed_encounter == *prev_encounter,
                None => false,
            } {
                proposed_encounter = *self.sts_random.weighted_choose(pool);
            }
            self.monster_queue.push_back(proposed_encounter);
        }
    }

    // Adds ten elite encounters to the elite encounter queue.
    fn sample_elite_encounters(&mut self) {
        for _ in 0..10 {
            let mut proposed_encounter = *self
                .sts_random
                .weighted_choose(self.act.elite_pool_with_probs());
            while match self.elite_queue.back() {
                Some(prev_encounter) => proposed_encounter == *prev_encounter,
                None => false,
            } {
                proposed_encounter = *self
                    .sts_random
                    .weighted_choose(self.act.elite_pool_with_probs());
            }
            self.elite_queue.push_back(proposed_encounter);
        }
    }

    // Adds one boss encounter to the boss encounter queue.
    // TODO: Make this two bosses for higher Ascensions.
    fn sample_boss_encounters(&mut self) {
        let mut bosses = self.act.boss_pool().to_vec();
        self.sts_random.java_compat_shuffle(bosses.as_mut());
        self.boss_queue.push_back(bosses[0]);
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_monster_encounters() {
        let mut generator = EnemyEncounterGenerator::new(&Seed::from(1u64));
        assert_eq!(
            (0..16)
                .map(|_| generator.next_monster_encounter())
                .collect::<Vec<_>>(),
            [
                MonsterEncounter::Cultist,
                MonsterEncounter::JawWorm,
                MonsterEncounter::SmallSlimes,
                MonsterEncounter::ExordiumWildlife,
                MonsterEncounter::TwoFungiBeasts,
                MonsterEncounter::ThreeLice,
                MonsterEncounter::Looter,
                MonsterEncounter::ExordiumThugs,
                MonsterEncounter::LotsOfSlimes,
                MonsterEncounter::ThreeLice,
                MonsterEncounter::BlueSlaver,
                MonsterEncounter::RedSlaver,
                MonsterEncounter::TwoFungiBeasts,
                MonsterEncounter::Looter,
                MonsterEncounter::ExordiumWildlife,
                MonsterEncounter::BlueSlaver
            ]
        );
        generator.advance_act();
        assert_eq!(
            (0..15)
                .map(|_| generator.next_monster_encounter())
                .collect::<Vec<_>>(),
            [
                MonsterEncounter::Chosen,
                MonsterEncounter::ShelledParasite,
                MonsterEncounter::CenturionAndMystic,
                MonsterEncounter::SnakePlant,
                MonsterEncounter::CultistAndChosen,
                MonsterEncounter::Snecko,
                MonsterEncounter::SnakePlant,
                MonsterEncounter::ChosenAndByrd,
                MonsterEncounter::CultistAndChosen,
                MonsterEncounter::CenturionAndMystic,
                MonsterEncounter::ChosenAndByrd,
                MonsterEncounter::SnakePlant,
                MonsterEncounter::Snecko,
                MonsterEncounter::ShelledParasiteAndFungiBeast,
                MonsterEncounter::ChosenAndByrd
            ]
        );
        generator.advance_act();
        assert_eq!(
            (0..15)
                .map(|_| generator.next_monster_encounter())
                .collect::<Vec<_>>(),
            [
                MonsterEncounter::ThreeShapes,
                MonsterEncounter::OrbWalker,
                MonsterEncounter::Transient,
                MonsterEncounter::FourShapes,
                MonsterEncounter::JawWormHorde,
                MonsterEncounter::SpireGrowth,
                MonsterEncounter::SphericGuardianAndTwoShapes,
                MonsterEncounter::Maw,
                MonsterEncounter::FourShapes,
                MonsterEncounter::ThreeDarklings,
                MonsterEncounter::Transient,
                MonsterEncounter::WrithingMass,
                MonsterEncounter::JawWormHorde,
                MonsterEncounter::Maw,
                MonsterEncounter::ThreeDarklings
            ]
        );
        let mut generator = EnemyEncounterGenerator::new(&Seed::from(2u64));
        assert_eq!(
            (0..16)
                .map(|_| generator.next_monster_encounter())
                .collect::<Vec<_>>(),
            [
                MonsterEncounter::Cultist,
                MonsterEncounter::JawWorm,
                MonsterEncounter::SmallSlimes,
                MonsterEncounter::BlueSlaver,
                MonsterEncounter::LargeSlime,
                MonsterEncounter::Looter,
                MonsterEncounter::LotsOfSlimes,
                MonsterEncounter::ThreeLice,
                MonsterEncounter::GremlinGang,
                MonsterEncounter::LargeSlime,
                MonsterEncounter::ExordiumThugs,
                MonsterEncounter::Looter,
                MonsterEncounter::ThreeLice,
                MonsterEncounter::GremlinGang,
                MonsterEncounter::RedSlaver,
                MonsterEncounter::ExordiumThugs
            ]
        );
        let mut generator = EnemyEncounterGenerator::new(&Seed::from(3u64));
        assert_eq!(
            (0..16)
                .map(|_| generator.next_monster_encounter())
                .collect::<Vec<_>>(),
            [
                MonsterEncounter::SmallSlimes,
                MonsterEncounter::Cultist,
                MonsterEncounter::TwoLice,
                MonsterEncounter::RedSlaver,
                MonsterEncounter::ExordiumThugs,
                MonsterEncounter::Looter,
                MonsterEncounter::LotsOfSlimes,
                MonsterEncounter::ThreeLice,
                MonsterEncounter::Looter,
                MonsterEncounter::LargeSlime,
                MonsterEncounter::RedSlaver,
                MonsterEncounter::LotsOfSlimes,
                MonsterEncounter::TwoFungiBeasts,
                MonsterEncounter::RedSlaver,
                MonsterEncounter::ThreeLice,
                MonsterEncounter::ExordiumThugs
            ]
        );
        let mut generator = EnemyEncounterGenerator::new(&Seed::from(4u64));
        assert_eq!(
            (0..16)
                .map(|_| generator.next_monster_encounter())
                .collect::<Vec<_>>(),
            [
                MonsterEncounter::JawWorm,
                MonsterEncounter::Cultist,
                MonsterEncounter::TwoLice,
                MonsterEncounter::ExordiumWildlife,
                MonsterEncounter::TwoFungiBeasts,
                MonsterEncounter::ThreeLice,
                MonsterEncounter::RedSlaver,
                MonsterEncounter::TwoFungiBeasts,
                MonsterEncounter::ThreeLice,
                MonsterEncounter::LotsOfSlimes,
                MonsterEncounter::ExordiumThugs,
                MonsterEncounter::Looter,
                MonsterEncounter::BlueSlaver,
                MonsterEncounter::ExordiumThugs,
                MonsterEncounter::ThreeLice,
                MonsterEncounter::GremlinGang
            ]
        );
        let mut generator = EnemyEncounterGenerator::new(&Seed::from(5u64));
        assert_eq!(
            (0..16)
                .map(|_| generator.next_monster_encounter())
                .collect::<Vec<_>>(),
            [
                MonsterEncounter::JawWorm,
                MonsterEncounter::Cultist,
                MonsterEncounter::SmallSlimes,
                MonsterEncounter::GremlinGang,
                MonsterEncounter::Looter,
                MonsterEncounter::TwoFungiBeasts,
                MonsterEncounter::RedSlaver,
                MonsterEncounter::ExordiumThugs,
                MonsterEncounter::Looter,
                MonsterEncounter::GremlinGang,
                MonsterEncounter::LargeSlime,
                MonsterEncounter::BlueSlaver,
                MonsterEncounter::ExordiumWildlife,
                MonsterEncounter::Looter,
                MonsterEncounter::RedSlaver,
                MonsterEncounter::ThreeLice
            ]
        );
        generator.advance_act();
        assert_eq!(
            (0..15)
                .map(|_| generator.next_monster_encounter())
                .collect::<Vec<_>>(),
            [
                MonsterEncounter::TwoThieves,
                MonsterEncounter::ThreeByrds,
                MonsterEncounter::SnakePlant,
                MonsterEncounter::Snecko,
                MonsterEncounter::CenturionAndMystic,
                MonsterEncounter::ShelledParasiteAndFungiBeast,
                MonsterEncounter::ChosenAndByrd,
                MonsterEncounter::SnakePlant,
                MonsterEncounter::CenturionAndMystic,
                MonsterEncounter::ChosenAndByrd,
                MonsterEncounter::Snecko,
                MonsterEncounter::SnakePlant,
                MonsterEncounter::CenturionAndMystic,
                MonsterEncounter::CultistAndChosen,
                MonsterEncounter::ThreeCultists
            ]
        );
        generator.advance_act();
        assert_eq!(
            (0..15)
                .map(|_| generator.next_monster_encounter())
                .collect::<Vec<_>>(),
            [
                MonsterEncounter::OrbWalker,
                MonsterEncounter::ThreeShapes,
                MonsterEncounter::Transient,
                MonsterEncounter::ThreeDarklings,
                MonsterEncounter::Maw,
                MonsterEncounter::JawWormHorde,
                MonsterEncounter::ThreeDarklings,
                MonsterEncounter::SphericGuardianAndTwoShapes,
                MonsterEncounter::Transient,
                MonsterEncounter::WrithingMass,
                MonsterEncounter::JawWormHorde,
                MonsterEncounter::Transient,
                MonsterEncounter::SphericGuardianAndTwoShapes,
                MonsterEncounter::Maw,
                MonsterEncounter::JawWormHorde
            ]
        );
    }

    #[test]
    fn test_elite_encounters() {
        let mut generator = EnemyEncounterGenerator::new(&Seed::from(1u64));
        assert_eq!(
            (0..10)
                .map(|_| generator.next_elite_encounter())
                .collect::<Vec<_>>(),
            [
                EliteEncounter::GremlinNob,
                EliteEncounter::ThreeSentries,
                EliteEncounter::Lagavulin,
                EliteEncounter::ThreeSentries,
                EliteEncounter::GremlinNob,
                EliteEncounter::Lagavulin,
                EliteEncounter::GremlinNob,
                EliteEncounter::Lagavulin,
                EliteEncounter::GremlinNob,
                EliteEncounter::ThreeSentries
            ]
        );
        generator.advance_act();
        assert_eq!(
            (0..10)
                .map(|_| generator.next_elite_encounter())
                .collect::<Vec<_>>(),
            [
                EliteEncounter::Taskmaster,
                EliteEncounter::GremlinLeader,
                EliteEncounter::Taskmaster,
                EliteEncounter::BookOfStabbing,
                EliteEncounter::Taskmaster,
                EliteEncounter::GremlinLeader,
                EliteEncounter::BookOfStabbing,
                EliteEncounter::GremlinLeader,
                EliteEncounter::BookOfStabbing,
                EliteEncounter::GremlinLeader
            ]
        );
        generator.advance_act();
        assert_eq!(
            (0..10)
                .map(|_| generator.next_elite_encounter())
                .collect::<Vec<_>>(),
            [
                EliteEncounter::GiantHead,
                EliteEncounter::Nemesis,
                EliteEncounter::Reptomancer,
                EliteEncounter::Nemesis,
                EliteEncounter::Reptomancer,
                EliteEncounter::GiantHead,
                EliteEncounter::Nemesis,
                EliteEncounter::Reptomancer,
                EliteEncounter::Nemesis,
                EliteEncounter::GiantHead
            ]
        );
        let mut generator = EnemyEncounterGenerator::new(&Seed::from(2u64));
        assert_eq!(
            (0..10)
                .map(|_| generator.next_elite_encounter())
                .collect::<Vec<_>>(),
            [
                EliteEncounter::Lagavulin,
                EliteEncounter::GremlinNob,
                EliteEncounter::ThreeSentries,
                EliteEncounter::Lagavulin,
                EliteEncounter::ThreeSentries,
                EliteEncounter::GremlinNob,
                EliteEncounter::ThreeSentries,
                EliteEncounter::GremlinNob,
                EliteEncounter::ThreeSentries,
                EliteEncounter::Lagavulin
            ]
        );
        let mut generator = EnemyEncounterGenerator::new(&Seed::from(3u64));
        assert_eq!(
            (0..10)
                .map(|_| generator.next_elite_encounter())
                .collect::<Vec<_>>(),
            [
                EliteEncounter::GremlinNob,
                EliteEncounter::Lagavulin,
                EliteEncounter::GremlinNob,
                EliteEncounter::ThreeSentries,
                EliteEncounter::GremlinNob,
                EliteEncounter::ThreeSentries,
                EliteEncounter::GremlinNob,
                EliteEncounter::Lagavulin,
                EliteEncounter::ThreeSentries,
                EliteEncounter::Lagavulin
            ]
        );
        let mut generator = EnemyEncounterGenerator::new(&Seed::from(4u64));
        assert_eq!(
            (0..10)
                .map(|_| generator.next_elite_encounter())
                .collect::<Vec<_>>(),
            [
                EliteEncounter::Lagavulin,
                EliteEncounter::ThreeSentries,
                EliteEncounter::Lagavulin,
                EliteEncounter::GremlinNob,
                EliteEncounter::Lagavulin,
                EliteEncounter::ThreeSentries,
                EliteEncounter::Lagavulin,
                EliteEncounter::ThreeSentries,
                EliteEncounter::GremlinNob,
                EliteEncounter::ThreeSentries
            ]
        );
        let mut generator = EnemyEncounterGenerator::new(&Seed::from(5u64));
        assert_eq!(
            (0..10)
                .map(|_| generator.next_elite_encounter())
                .collect::<Vec<_>>(),
            [
                EliteEncounter::GremlinNob,
                EliteEncounter::Lagavulin,
                EliteEncounter::ThreeSentries,
                EliteEncounter::Lagavulin,
                EliteEncounter::ThreeSentries,
                EliteEncounter::GremlinNob,
                EliteEncounter::Lagavulin,
                EliteEncounter::ThreeSentries,
                EliteEncounter::GremlinNob,
                EliteEncounter::Lagavulin
            ]
        );
        generator.advance_act();
        assert_eq!(
            (0..10)
                .map(|_| generator.next_elite_encounter())
                .collect::<Vec<_>>(),
            [
                EliteEncounter::Taskmaster,
                EliteEncounter::GremlinLeader,
                EliteEncounter::BookOfStabbing,
                EliteEncounter::Taskmaster,
                EliteEncounter::BookOfStabbing,
                EliteEncounter::Taskmaster,
                EliteEncounter::BookOfStabbing,
                EliteEncounter::Taskmaster,
                EliteEncounter::GremlinLeader,
                EliteEncounter::BookOfStabbing
            ]
        );
        generator.advance_act();
        assert_eq!(
            (0..10)
                .map(|_| generator.next_elite_encounter())
                .collect::<Vec<_>>(),
            [
                EliteEncounter::GiantHead,
                EliteEncounter::Reptomancer,
                EliteEncounter::Nemesis,
                EliteEncounter::GiantHead,
                EliteEncounter::Nemesis,
                EliteEncounter::GiantHead,
                EliteEncounter::Nemesis,
                EliteEncounter::GiantHead,
                EliteEncounter::Reptomancer,
                EliteEncounter::Nemesis
            ]
        );
    }

    #[test]
    fn test_boss_encounters() {
        let mut generator = EnemyEncounterGenerator::new(&Seed::from(1u64));
        assert_eq!(generator.next_boss_encounter(), BossEncounter::SlimeBoss);
        generator.advance_act();
        assert_eq!(
            generator.next_boss_encounter(),
            BossEncounter::BronzeAutomaton
        );
        generator.advance_act();
        assert_eq!(generator.next_boss_encounter(), BossEncounter::AwakenedOne);
        let mut generator = EnemyEncounterGenerator::new(&Seed::from(2u64));
        assert_eq!(generator.next_boss_encounter(), BossEncounter::SlimeBoss);
        let mut generator = EnemyEncounterGenerator::new(&Seed::from(3u64));
        assert_eq!(generator.next_boss_encounter(), BossEncounter::TheGuardian);
        let mut generator = EnemyEncounterGenerator::new(&Seed::from(4u64));
        assert_eq!(generator.next_boss_encounter(), BossEncounter::Hexaghost);
        let mut generator = EnemyEncounterGenerator::new(&Seed::from(5u64));
        assert_eq!(generator.next_boss_encounter(), BossEncounter::Hexaghost);
        generator.advance_act();
        assert_eq!(generator.next_boss_encounter(), BossEncounter::TheChamp);
        generator.advance_act();
        assert_eq!(generator.next_boss_encounter(), BossEncounter::DonuAndDeca);
    }
}
