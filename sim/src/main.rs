mod act;
mod ascension;
mod character;
mod encounter;
mod game_context;
mod map;
mod rng;

use crate::act::Act;
use crate::ascension::Ascension;
use crate::character::Character;
use crate::game_context::GameContext;

fn main() {
    for seed in 1u64..=10000u64 {
        let mut game_context = GameContext::from(seed.into(), Character::Ironclad, Ascension(0));
        println!(
            "seed: {} act: 1 rng: {} monsterList: {} eliteList: {} bossList: {}",
            seed,
            game_context.monster_rng.get_counter(),
            game_context
                .monster_encounters
                .iter()
                .map(|monster| format!("{:?}", monster))
                .collect::<Vec<String>>()
                .join(","),
            game_context
                .elite_encounters
                .iter()
                .map(|elite| format!("{:?}", elite))
                .collect::<Vec<String>>()
                .join(","),
            game_context
                .boss_encounters
                .iter()
                .map(|elite| format!("{:?}", elite))
                .collect::<Vec<String>>()
                .join(",")
        );
        game_context.transition_to_act(Act(2));
        println!(
            "seed: {} act: 2 rng: {} monsterList: {} eliteList: {} bossList: {}",
            seed,
            game_context.monster_rng.get_counter(),
            game_context
                .monster_encounters
                .iter()
                .map(|monster| format!("{:?}", monster))
                .collect::<Vec<String>>()
                .join(","),
            game_context
                .elite_encounters
                .iter()
                .map(|elite| format!("{:?}", elite))
                .collect::<Vec<String>>()
                .join(","),
            game_context
                .boss_encounters
                .iter()
                .map(|elite| format!("{:?}", elite))
                .collect::<Vec<String>>()
                .join(",")
        );
        game_context.transition_to_act(Act(3));
        println!(
            "seed: {} act: 3 rng: {} monsterList: {} eliteList: {} bossList: {}",
            seed,
            game_context.monster_rng.get_counter(),
            game_context
                .monster_encounters
                .iter()
                .map(|monster| format!("{:?}", monster))
                .collect::<Vec<String>>()
                .join(","),
            game_context
                .elite_encounters
                .iter()
                .map(|elite| format!("{:?}", elite))
                .collect::<Vec<String>>()
                .join(","),
            game_context
                .boss_encounters
                .iter()
                .map(|elite| format!("{:?}", elite))
                .collect::<Vec<String>>()
                .join(",")
        );
    }
}
