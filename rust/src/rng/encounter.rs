use std::collections::VecDeque;

use crate::data::{Act, Encounter};

use super::{Seed, StsRandom};

/// A struct that generates the enemy encounters for a run based on the provided seed.
///
/// Enemy encounters are generated in advance and stored in queues for easy retrieval.
/// The generator is stateful and resets with each new Act as the run progresses.
///
/// Side note: Ideally we would use an Iterator pattern here, with an Iterator type for each
/// encounter class (Monster, Elite, and Boss). But for rng fidelity to the original game,
/// we need to share the same StsRandom generator across all encounter classes. This means that
/// the different Iterator types would need to hold mutable references to the same underlying
/// StsRandom. This is an antipattern in Rust and would require use of RefCell or Mutex
/// to work around. So we're using a more manual approach here.
pub struct EncounterGenerator {
    act: &'static Act,
    encounter_rng: StsRandom,
    monster_queue: VecDeque<Encounter>,
    elite_queue: VecDeque<Encounter>,
    boss_queue: VecDeque<Encounter>,
}

impl EncounterGenerator {
    /// Constructs a new EncounterGenerator with the provided seed, prepopulating encounter
    /// queues for Act 1.
    pub fn new(seed: Seed) -> Self {
        let mut result = Self {
            act: Act::get(1),
            encounter_rng: StsRandom::from(seed),
            monster_queue: VecDeque::new(),
            elite_queue: VecDeque::new(),
            boss_queue: VecDeque::new(),
        };
        result.sample_all();
        result
    }

    /// Returns the next monster encounter in the queue, sampling more if necessary.
    pub fn next_monster_encounter(&mut self) -> Encounter {
        if let Some(encounter) = self.monster_queue.pop_front() {
            encounter
        } else {
            self.sample_strong_monster_encounters();
            self.next_monster_encounter()
        }
    }

    /// Returns the next elite encounter in the queue, sampling more if necessary.
    pub fn next_elite_encounter(&mut self) -> Encounter {
        if let Some(encounter) = self.elite_queue.pop_front() {
            encounter
        } else {
            self.sample_elite_encounters();
            self.next_elite_encounter()
        }
    }

    /// Returns the next boss encounter in the queue.
    pub fn next_boss_encounter(&mut self) -> Encounter {
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
            self.act.weak_monster_encounter_count,
            self.act.weak_monster_encounter_pool,
        );
    }

    // Adds the first strong monster encounter to the monster encounter queue, avoiding some
    // embarassing repetition.
    fn sample_first_strong_monster_encounter(&mut self) {
        let mut proposed_encounter = *self
            .encounter_rng
            .weighted_choose(self.act.strong_monster_encounter_pool);
        while match self.monster_queue.back() {
            Some(prev_encounter) => matches!(
                (*prev_encounter, proposed_encounter),
                (Encounter::SmallSlimes, Encounter::LotsOfSlimes)
                    | (Encounter::SmallSlimes, Encounter::LargeSlime)
                    | (Encounter::TwoLice, Encounter::ThreeLice)
            ),
            None => false,
        } {
            proposed_encounter = *self
                .encounter_rng
                .weighted_choose(self.act.strong_monster_encounter_pool);
        }
        self.monster_queue.push_back(proposed_encounter);
    }

    // Adds twelve strong monster encounters to the monster encounter queue.
    fn sample_strong_monster_encounters(&mut self) {
        self.sample_monster_encounters(12, self.act.strong_monster_encounter_pool);
    }

    // Helper method that samples the specified number of encounters from the provided pool
    // and adds them to the monster encounter queue.
    //
    // Avoids repetition by examining the previous and doubly-previous encounters and avoiding
    // duplicates.
    fn sample_monster_encounters(&mut self, count: usize, pool: &'static [(Encounter, f32)]) {
        for _ in 0..count {
            let mut proposed_encounter = *self.encounter_rng.weighted_choose(pool);
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
                proposed_encounter = *self.encounter_rng.weighted_choose(pool);
            }
            self.monster_queue.push_back(proposed_encounter);
        }
    }

    // Adds ten elite encounters to the elite encounter queue.
    fn sample_elite_encounters(&mut self) {
        for _ in 0..10 {
            let mut proposed_encounter = *self
                .encounter_rng
                .weighted_choose(self.act.elite_encounter_pool);
            while match self.elite_queue.back() {
                Some(prev_encounter) => proposed_encounter == *prev_encounter,
                None => false,
            } {
                proposed_encounter = *self
                    .encounter_rng
                    .weighted_choose(self.act.elite_encounter_pool);
            }
            self.elite_queue.push_back(proposed_encounter);
        }
    }

    // Adds one boss encounter to the boss encounter queue.
    // TODO: Make this two bosses for higher Ascensions.
    fn sample_boss_encounters(&mut self) {
        let mut bosses = self.act.boss_encounter_pool.to_vec();
        self.encounter_rng.java_compat_shuffle(bosses.as_mut());
        self.boss_queue.push_back(bosses[0]);
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_monster_encounters() {
        let mut generator = EncounterGenerator::new(1.into());
        assert_eq!(
            (0..16)
                .map(|_| generator.next_monster_encounter())
                .collect::<Vec<_>>(),
            [
                Encounter::Cultist,
                Encounter::JawWorm,
                Encounter::SmallSlimes,
                Encounter::ExordiumWildlife,
                Encounter::TwoFungiBeasts,
                Encounter::ThreeLice,
                Encounter::Looter,
                Encounter::ExordiumThugs,
                Encounter::LotsOfSlimes,
                Encounter::ThreeLice,
                Encounter::BlueSlaver,
                Encounter::RedSlaver,
                Encounter::TwoFungiBeasts,
                Encounter::Looter,
                Encounter::ExordiumWildlife,
                Encounter::BlueSlaver
            ]
        );
        generator.advance_act();
        assert_eq!(
            (0..15)
                .map(|_| generator.next_monster_encounter())
                .collect::<Vec<_>>(),
            [
                Encounter::Chosen,
                Encounter::ShelledParasite,
                Encounter::CenturionAndMystic,
                Encounter::SnakePlant,
                Encounter::CultistAndChosen,
                Encounter::Snecko,
                Encounter::SnakePlant,
                Encounter::ChosenAndByrd,
                Encounter::CultistAndChosen,
                Encounter::CenturionAndMystic,
                Encounter::ChosenAndByrd,
                Encounter::SnakePlant,
                Encounter::Snecko,
                Encounter::ShelledParasiteAndFungiBeast,
                Encounter::ChosenAndByrd
            ]
        );
        generator.advance_act();
        assert_eq!(
            (0..15)
                .map(|_| generator.next_monster_encounter())
                .collect::<Vec<_>>(),
            [
                Encounter::ThreeShapes,
                Encounter::OrbWalker,
                Encounter::Transient,
                Encounter::FourShapes,
                Encounter::JawWormHorde,
                Encounter::SpireGrowth,
                Encounter::SphericGuardianAndTwoShapes,
                Encounter::Maw,
                Encounter::FourShapes,
                Encounter::ThreeDarklings,
                Encounter::Transient,
                Encounter::WrithingMass,
                Encounter::JawWormHorde,
                Encounter::Maw,
                Encounter::ThreeDarklings
            ]
        );
        let mut generator = EncounterGenerator::new(2.into());
        assert_eq!(
            (0..16)
                .map(|_| generator.next_monster_encounter())
                .collect::<Vec<_>>(),
            [
                Encounter::Cultist,
                Encounter::JawWorm,
                Encounter::SmallSlimes,
                Encounter::BlueSlaver,
                Encounter::LargeSlime,
                Encounter::Looter,
                Encounter::LotsOfSlimes,
                Encounter::ThreeLice,
                Encounter::GremlinGang,
                Encounter::LargeSlime,
                Encounter::ExordiumThugs,
                Encounter::Looter,
                Encounter::ThreeLice,
                Encounter::GremlinGang,
                Encounter::RedSlaver,
                Encounter::ExordiumThugs
            ]
        );
        let mut generator = EncounterGenerator::new(3.into());
        assert_eq!(
            (0..16)
                .map(|_| generator.next_monster_encounter())
                .collect::<Vec<_>>(),
            [
                Encounter::SmallSlimes,
                Encounter::Cultist,
                Encounter::TwoLice,
                Encounter::RedSlaver,
                Encounter::ExordiumThugs,
                Encounter::Looter,
                Encounter::LotsOfSlimes,
                Encounter::ThreeLice,
                Encounter::Looter,
                Encounter::LargeSlime,
                Encounter::RedSlaver,
                Encounter::LotsOfSlimes,
                Encounter::TwoFungiBeasts,
                Encounter::RedSlaver,
                Encounter::ThreeLice,
                Encounter::ExordiumThugs
            ]
        );
        let mut generator = EncounterGenerator::new(4.into());
        assert_eq!(
            (0..16)
                .map(|_| generator.next_monster_encounter())
                .collect::<Vec<_>>(),
            [
                Encounter::JawWorm,
                Encounter::Cultist,
                Encounter::TwoLice,
                Encounter::ExordiumWildlife,
                Encounter::TwoFungiBeasts,
                Encounter::ThreeLice,
                Encounter::RedSlaver,
                Encounter::TwoFungiBeasts,
                Encounter::ThreeLice,
                Encounter::LotsOfSlimes,
                Encounter::ExordiumThugs,
                Encounter::Looter,
                Encounter::BlueSlaver,
                Encounter::ExordiumThugs,
                Encounter::ThreeLice,
                Encounter::GremlinGang
            ]
        );
        let mut generator = EncounterGenerator::new(5.into());
        assert_eq!(
            (0..16)
                .map(|_| generator.next_monster_encounter())
                .collect::<Vec<_>>(),
            [
                Encounter::JawWorm,
                Encounter::Cultist,
                Encounter::SmallSlimes,
                Encounter::GremlinGang,
                Encounter::Looter,
                Encounter::TwoFungiBeasts,
                Encounter::RedSlaver,
                Encounter::ExordiumThugs,
                Encounter::Looter,
                Encounter::GremlinGang,
                Encounter::LargeSlime,
                Encounter::BlueSlaver,
                Encounter::ExordiumWildlife,
                Encounter::Looter,
                Encounter::RedSlaver,
                Encounter::ThreeLice
            ]
        );
        generator.advance_act();
        assert_eq!(
            (0..15)
                .map(|_| generator.next_monster_encounter())
                .collect::<Vec<_>>(),
            [
                Encounter::TwoThieves,
                Encounter::ThreeByrds,
                Encounter::SnakePlant,
                Encounter::Snecko,
                Encounter::CenturionAndMystic,
                Encounter::ShelledParasiteAndFungiBeast,
                Encounter::ChosenAndByrd,
                Encounter::SnakePlant,
                Encounter::CenturionAndMystic,
                Encounter::ChosenAndByrd,
                Encounter::Snecko,
                Encounter::SnakePlant,
                Encounter::CenturionAndMystic,
                Encounter::CultistAndChosen,
                Encounter::ThreeCultists
            ]
        );
        generator.advance_act();
        assert_eq!(
            (0..15)
                .map(|_| generator.next_monster_encounter())
                .collect::<Vec<_>>(),
            [
                Encounter::OrbWalker,
                Encounter::ThreeShapes,
                Encounter::Transient,
                Encounter::ThreeDarklings,
                Encounter::Maw,
                Encounter::JawWormHorde,
                Encounter::ThreeDarklings,
                Encounter::SphericGuardianAndTwoShapes,
                Encounter::Transient,
                Encounter::WrithingMass,
                Encounter::JawWormHorde,
                Encounter::Transient,
                Encounter::SphericGuardianAndTwoShapes,
                Encounter::Maw,
                Encounter::JawWormHorde
            ]
        );
    }

    #[test]
    fn test_elite_encounters() {
        let mut generator = EncounterGenerator::new(1.into());
        assert_eq!(
            (0..10)
                .map(|_| generator.next_elite_encounter())
                .collect::<Vec<_>>(),
            [
                Encounter::GremlinNob,
                Encounter::ThreeSentries,
                Encounter::Lagavulin,
                Encounter::ThreeSentries,
                Encounter::GremlinNob,
                Encounter::Lagavulin,
                Encounter::GremlinNob,
                Encounter::Lagavulin,
                Encounter::GremlinNob,
                Encounter::ThreeSentries
            ]
        );
        generator.advance_act();
        assert_eq!(
            (0..10)
                .map(|_| generator.next_elite_encounter())
                .collect::<Vec<_>>(),
            [
                Encounter::Taskmaster,
                Encounter::GremlinLeader,
                Encounter::Taskmaster,
                Encounter::BookOfStabbing,
                Encounter::Taskmaster,
                Encounter::GremlinLeader,
                Encounter::BookOfStabbing,
                Encounter::GremlinLeader,
                Encounter::BookOfStabbing,
                Encounter::GremlinLeader
            ]
        );
        generator.advance_act();
        assert_eq!(
            (0..10)
                .map(|_| generator.next_elite_encounter())
                .collect::<Vec<_>>(),
            [
                Encounter::GiantHead,
                Encounter::Nemesis,
                Encounter::Reptomancer,
                Encounter::Nemesis,
                Encounter::Reptomancer,
                Encounter::GiantHead,
                Encounter::Nemesis,
                Encounter::Reptomancer,
                Encounter::Nemesis,
                Encounter::GiantHead
            ]
        );
        let mut generator = EncounterGenerator::new(2.into());
        assert_eq!(
            (0..10)
                .map(|_| generator.next_elite_encounter())
                .collect::<Vec<_>>(),
            [
                Encounter::Lagavulin,
                Encounter::GremlinNob,
                Encounter::ThreeSentries,
                Encounter::Lagavulin,
                Encounter::ThreeSentries,
                Encounter::GremlinNob,
                Encounter::ThreeSentries,
                Encounter::GremlinNob,
                Encounter::ThreeSentries,
                Encounter::Lagavulin
            ]
        );
        let mut generator = EncounterGenerator::new(3.into());
        assert_eq!(
            (0..10)
                .map(|_| generator.next_elite_encounter())
                .collect::<Vec<_>>(),
            [
                Encounter::GremlinNob,
                Encounter::Lagavulin,
                Encounter::GremlinNob,
                Encounter::ThreeSentries,
                Encounter::GremlinNob,
                Encounter::ThreeSentries,
                Encounter::GremlinNob,
                Encounter::Lagavulin,
                Encounter::ThreeSentries,
                Encounter::Lagavulin
            ]
        );
        let mut generator = EncounterGenerator::new(4.into());
        assert_eq!(
            (0..10)
                .map(|_| generator.next_elite_encounter())
                .collect::<Vec<_>>(),
            [
                Encounter::Lagavulin,
                Encounter::ThreeSentries,
                Encounter::Lagavulin,
                Encounter::GremlinNob,
                Encounter::Lagavulin,
                Encounter::ThreeSentries,
                Encounter::Lagavulin,
                Encounter::ThreeSentries,
                Encounter::GremlinNob,
                Encounter::ThreeSentries
            ]
        );
        let mut generator = EncounterGenerator::new(5.into());
        assert_eq!(
            (0..10)
                .map(|_| generator.next_elite_encounter())
                .collect::<Vec<_>>(),
            [
                Encounter::GremlinNob,
                Encounter::Lagavulin,
                Encounter::ThreeSentries,
                Encounter::Lagavulin,
                Encounter::ThreeSentries,
                Encounter::GremlinNob,
                Encounter::Lagavulin,
                Encounter::ThreeSentries,
                Encounter::GremlinNob,
                Encounter::Lagavulin
            ]
        );
        generator.advance_act();
        assert_eq!(
            (0..10)
                .map(|_| generator.next_elite_encounter())
                .collect::<Vec<_>>(),
            [
                Encounter::Taskmaster,
                Encounter::GremlinLeader,
                Encounter::BookOfStabbing,
                Encounter::Taskmaster,
                Encounter::BookOfStabbing,
                Encounter::Taskmaster,
                Encounter::BookOfStabbing,
                Encounter::Taskmaster,
                Encounter::GremlinLeader,
                Encounter::BookOfStabbing
            ]
        );
        generator.advance_act();
        assert_eq!(
            (0..10)
                .map(|_| generator.next_elite_encounter())
                .collect::<Vec<_>>(),
            [
                Encounter::GiantHead,
                Encounter::Reptomancer,
                Encounter::Nemesis,
                Encounter::GiantHead,
                Encounter::Nemesis,
                Encounter::GiantHead,
                Encounter::Nemesis,
                Encounter::GiantHead,
                Encounter::Reptomancer,
                Encounter::Nemesis
            ]
        );
    }

    #[test]
    fn test_boss_encounters() {
        let mut generator = EncounterGenerator::new(1.into());
        assert_eq!(generator.next_boss_encounter(), Encounter::SlimeBoss);
        generator.advance_act();
        assert_eq!(generator.next_boss_encounter(), Encounter::BronzeAutomaton);
        generator.advance_act();
        assert_eq!(generator.next_boss_encounter(), Encounter::AwakenedOne);
        let mut generator = EncounterGenerator::new(2.into());
        assert_eq!(generator.next_boss_encounter(), Encounter::SlimeBoss);
        let mut generator = EncounterGenerator::new(3.into());
        assert_eq!(generator.next_boss_encounter(), Encounter::TheGuardian);
        let mut generator = EncounterGenerator::new(4.into());
        assert_eq!(generator.next_boss_encounter(), Encounter::Hexaghost);
        let mut generator = EncounterGenerator::new(5.into());
        assert_eq!(generator.next_boss_encounter(), Encounter::Hexaghost);
        generator.advance_act();
        assert_eq!(generator.next_boss_encounter(), Encounter::TheChamp);
        generator.advance_act();
        assert_eq!(generator.next_boss_encounter(), Encounter::DonuAndDeca);
    }
}
