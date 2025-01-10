use std::env;

use once_cell::sync::Lazy;

use crate::act::Act;
use crate::ascension::Ascension;
use crate::character::Character;
use crate::encounter::{BossEncounter, EliteEncounter, MonsterEncounter};
use crate::rng::{JavaRandom, Seed, StsRandom};

pub static GAME_CONTEXT: Lazy<GameContext> = Lazy::new(GameContext::from_args);

#[derive(Debug)]
pub struct GameContext {
    pub seed: Seed,
    pub character: Character,
    pub ascension: Ascension,

    pub monster_rng: StsRandom,

    pub monster_encounters: Vec<MonsterEncounter>,
    pub elite_encounters: Vec<EliteEncounter>,
    pub boss_encounters: Vec<BossEncounter>,
}

impl GameContext {
    fn from_args() -> Self {
        let mut args = env::args();
        args.next(); // Skip the program name
        let seed = args
            .next()
            .unwrap_or_else(|| panic!("No seed provided"))
            .as_str()
            .try_into()
            .unwrap_or_else(|e| panic!("Invalid seed: {}", e));
        let character = args
            .next()
            .unwrap_or_else(|| panic!("No character provided"))
            .as_str()
            .try_into()
            .unwrap_or_else(|e| panic!("Invalid character: {}", e));
        let ascension = args
            .next()
            .unwrap_or_else(|| panic!("No ascension level provided"))
            .as_str()
            .try_into()
            .unwrap_or_else(|e| panic!("Invalid ascension: {}", e));
        Self::from(seed, character, ascension)
    }

    pub fn from(seed: Seed, character: Character, ascension: Ascension) -> Self {
        let mut monster_rng = (&seed).into();
        let monster_encounters = Self::generate_monster_encounters(&mut monster_rng, Act(1));
        let elite_encounters = Self::generate_elite_encounters(&mut monster_rng, Act(1));
        let boss_encounters = Self::generate_boss_encounters(&mut monster_rng, Act(1));
        Self {
            seed,
            character,
            ascension,
            monster_rng,
            monster_encounters,
            elite_encounters,
            boss_encounters,
        }
    }

    pub fn transition_to_act(&mut self, act: Act) {
        self.monster_encounters = Self::generate_monster_encounters(&mut self.monster_rng, act);
        self.elite_encounters = Self::generate_elite_encounters(&mut self.monster_rng, act);
        self.boss_encounters = Self::generate_boss_encounters(&mut self.monster_rng, act);
    }

    // TODO: This could probably be optimized by maintaining `prev` and `prev_prev` Option<Monster>s.
    fn sample_monsters(
        sts_random: &mut StsRandom,
        choices: &[(MonsterEncounter, f32)],
        count: usize,
        output_vec: &mut Vec<MonsterEncounter>,
    ) {
        let initial_len = output_vec.len();
        let mut i = initial_len;
        while i < initial_len + count {
            // Don't use this monster if it's the previous or previous previous in the list
            let monster = *sts_random.weighted_choose(choices);
            //println!("Iteration {}: {:?}", i, monster);
            match i {
                0 => {
                    output_vec.push(monster);
                    i += 1;
                }
                1 => {
                    if output_vec[0] != monster {
                        output_vec.push(monster);
                        i += 1;
                    }
                }
                _ => {
                    if output_vec[i - 1] != monster && output_vec[i - 2] != monster {
                        output_vec.push(monster);
                        i += 1;
                    }
                }
            }
        }
    }

    fn generate_monster_encounters(sts_random: &mut StsRandom, act: Act) -> Vec<MonsterEncounter> {
        let details = act.get_details();
        let mut result = Vec::new(); // TODO: with_capacity
        Self::sample_monsters(
            sts_random,
            details.weak_monster_encounters_and_probs,
            details.weak_monster_encounter_count,
            &mut result,
        );
        let last_weak_monster = result.last().copied().expect("No weak monsters");
        loop {
            let first_strong_monster =
                *sts_random.weighted_choose(details.strong_monster_encounters_and_probs);
            match (last_weak_monster, first_strong_monster) {
                (MonsterEncounter::SmallSlimes, MonsterEncounter::LotsOfSlimes) => continue,
                (MonsterEncounter::SmallSlimes, MonsterEncounter::LargeSlime) => continue,
                (MonsterEncounter::TwoLice, MonsterEncounter::ThreeLice) => continue,
                _ => {
                    result.push(first_strong_monster);
                    break;
                }
            }
        }
        Self::sample_monsters(
            sts_random,
            details.strong_monster_encounters_and_probs,
            12,
            &mut result,
        );
        result
    }

    fn generate_elite_encounters(sts_random: &mut StsRandom, act: Act) -> Vec<EliteEncounter> {
        let details = act.get_details();
        let mut result = Vec::with_capacity(10);
        let mut i = 0;
        while i < 10 {
            let elite = *sts_random.weighted_choose(details.elite_encounters);
            match i {
                0 => {
                    result.push(elite);
                    i += 1;
                }
                _ => {
                    if result[i - 1] != elite {
                        result.push(elite);
                        i += 1;
                    }
                }
            }
        }
        result
    }

    fn generate_boss_encounters(sts_random: &mut StsRandom, act: Act) -> Vec<BossEncounter> {
        let details = act.get_details();
        let mut result = Vec::new();
        let mut bosses = details.boss_encounters.to_vec();
        // I'm guessing that the generator is forked into a java random so no more than
        // one tick is consumed of the original rng, maybe for backward compatibility with
        // save files or something. Or maybe they didn't yet have Fisher-Yates implemented
        // for StsRandom at the point this was written.
        sts_random.fork_java_random().shuffle(bosses.as_mut());
        result.push(bosses[0]);
        result
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_monster_encounters() {
        let mut game_context = GameContext::from(1u64.into(), Character::Ironclad, Ascension(0));
        assert_eq!(
            game_context.monster_encounters,
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
        game_context.transition_to_act(Act(2));
        assert_eq!(
            game_context.monster_encounters,
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
        game_context.transition_to_act(Act(3));
        assert_eq!(
            game_context.monster_encounters,
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
        let game_context = GameContext::from(2u64.into(), Character::Ironclad, Ascension(0));
        assert_eq!(
            game_context.monster_encounters,
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
        let game_context = GameContext::from(3u64.into(), Character::Ironclad, Ascension(0));
        assert_eq!(
            game_context.monster_encounters,
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
        let game_context = GameContext::from(4u64.into(), Character::Ironclad, Ascension(0));
        assert_eq!(
            game_context.monster_encounters,
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
        let mut game_context = GameContext::from(5u64.into(), Character::Ironclad, Ascension(0));
        assert_eq!(
            game_context.monster_encounters,
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
        game_context.transition_to_act(Act(2));
        assert_eq!(
            game_context.monster_encounters,
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
        game_context.transition_to_act(Act(3));
        assert_eq!(
            game_context.monster_encounters,
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
        let mut game_context = GameContext::from(1u64.into(), Character::Ironclad, Ascension(0));
        assert_eq!(
            game_context.elite_encounters,
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
        game_context.transition_to_act(Act(2));
        assert_eq!(
            game_context.elite_encounters,
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
        game_context.transition_to_act(Act(3));
        assert_eq!(
            game_context.elite_encounters,
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
        let game_context = GameContext::from(2u64.into(), Character::Ironclad, Ascension(0));
        assert_eq!(
            game_context.elite_encounters,
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
        let game_context = GameContext::from(3u64.into(), Character::Ironclad, Ascension(0));
        assert_eq!(
            game_context.elite_encounters,
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
        let game_context = GameContext::from(4u64.into(), Character::Ironclad, Ascension(0));
        assert_eq!(
            game_context.elite_encounters,
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
        let mut game_context = GameContext::from(5u64.into(), Character::Ironclad, Ascension(0));
        assert_eq!(
            game_context.elite_encounters,
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
        game_context.transition_to_act(Act(2));
        assert_eq!(
            game_context.elite_encounters,
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
        game_context.transition_to_act(Act(3));
        assert_eq!(
            game_context.elite_encounters,
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
        let mut game_context = GameContext::from(1u64.into(), Character::Ironclad, Ascension(0));
        assert_eq!(game_context.boss_encounters, [BossEncounter::SlimeBoss]);
        game_context.transition_to_act(Act(2));
        assert_eq!(
            game_context.boss_encounters,
            [BossEncounter::BronzeAutomaton]
        );
        game_context.transition_to_act(Act(3));
        assert_eq!(game_context.boss_encounters, [BossEncounter::AwakenedOne]);
        let game_context = GameContext::from(2u64.into(), Character::Ironclad, Ascension(0));
        assert_eq!(game_context.boss_encounters, [BossEncounter::SlimeBoss]);
        let game_context = GameContext::from(3u64.into(), Character::Ironclad, Ascension(0));
        assert_eq!(game_context.boss_encounters, [BossEncounter::TheGuardian]);
        let game_context = GameContext::from(4u64.into(), Character::Ironclad, Ascension(0));
        assert_eq!(game_context.boss_encounters, [BossEncounter::Hexaghost]);
        let mut game_context = GameContext::from(5u64.into(), Character::Ironclad, Ascension(0));
        assert_eq!(game_context.boss_encounters, [BossEncounter::Hexaghost]);
        game_context.transition_to_act(Act(2));
        assert_eq!(game_context.boss_encounters, [BossEncounter::TheChamp]);
        game_context.transition_to_act(Act(3));
        assert_eq!(game_context.boss_encounters, [BossEncounter::DonuAndDeca]);
    }
}
