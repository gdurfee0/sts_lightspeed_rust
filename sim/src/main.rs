mod data;
mod map;
mod params;
mod rng;

use crate::data::Act;
use crate::map::MapBuilder;
use crate::params::GAME_PARAMS;
use crate::rng::EnemyEncounterGenerator;

fn main() {
    let seed = &GAME_PARAMS.seed;
    let character = GAME_PARAMS.character;
    let ascension = GAME_PARAMS.ascension;
    println!(
        "Seed: {:?}, Character: {:?}, Ascension: {:?}",
        seed, character, ascension,
    );
    let node_grid = MapBuilder::from(seed, ascension, Act::get(1)).build();
    println!("{}", node_grid);
    let mut enemy_encounter_generator = EnemyEncounterGenerator::new(seed);
    println!(
        "First monster: {:?}",
        enemy_encounter_generator.next_monster_encounter()
    );
    println!(
        "Second monster: {:?}",
        enemy_encounter_generator.next_monster_encounter()
    );
    println!(
        "First elite: {:?}",
        enemy_encounter_generator.next_elite_encounter()
    );
    println!(
        "Act 1 Boss: {:?}",
        enemy_encounter_generator.next_boss_encounter()
    );
    enemy_encounter_generator.advance_act();
    println!(
        "Act 2 Boss: {:?}",
        enemy_encounter_generator.next_boss_encounter()
    );
}
