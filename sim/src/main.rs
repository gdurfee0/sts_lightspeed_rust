mod act;
mod ascension;
mod character;
mod encounter;
mod game_context;
mod map;
mod rng;

use crate::act::Act;
use crate::encounter::MonsterEncounter;
use crate::game_context::GAME_CONTEXT;
use crate::map::MapBuilder;
use crate::rng::StsRandom;

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

fn generate_monster_list(mut sts_random: StsRandom, act: Act) -> Vec<MonsterEncounter> {
    let details = act.get_details();
    let mut result = Vec::new();
    sample_monsters(
        &mut sts_random,
        details.weak_monster_encounters_and_probs,
        details.weak_monster_encounter_count,
        &mut result,
    );
    println!(
        "Counter after weak monster assignment: {}",
        sts_random.get_counter()
    );
    let last_weak_monster = result.last().copied().expect("No weak monsters");
    loop {
        let first_strong_monster =
            *sts_random.weighted_choose(details.strong_monster_encounters_and_probs);
        match (last_weak_monster, first_strong_monster) {
            (MonsterEncounter::SmallSlimes, MonsterEncounter::LargeSlime) => continue,
            (MonsterEncounter::TwoLice, MonsterEncounter::ThreeLice) => continue,
            _ => {
                result.push(first_strong_monster);
                break;
            }
        }
    }
    println!(
        "Counter after first strong monster assignment: {}",
        sts_random.get_counter()
    );
    sample_monsters(
        &mut sts_random,
        details.strong_monster_encounters_and_probs,
        12,
        &mut result,
    );
    println!(
        "Counter after strong monster assignment: {}",
        sts_random.get_counter()
    );
    result
}

fn main() {
    println!("Seed: {:?}", GAME_CONTEXT.seed);
    println!("Map for act 1:");
    println!("{}", MapBuilder::for_act(Act(1)).build());
    println!("Monster list for act 1:");
    println!(
        "{:?}",
        generate_monster_list(GAME_CONTEXT.seed.clone().into(), Act(1))
    );
}
