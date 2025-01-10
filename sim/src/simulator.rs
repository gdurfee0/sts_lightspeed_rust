use crate::data::{Act, Ascension, Character};
use crate::map::MapBuilder;
use crate::rng::{EncounterGenerator, Seed};

pub struct Simulator {
    seed: Seed,
    character: Character,
    ascension: Ascension,
}

impl Simulator {
    pub fn new(seed: Seed, character: Character, ascension: Ascension) -> Self {
        Self {
            seed,
            character,
            ascension,
        }
    }

    pub fn run(&self) {
        println!(
            "Seed: {:?}, Character: {:?}, Ascension: {:?}",
            self.seed, self.character, self.ascension,
        );
        let node_grid = MapBuilder::from(&self.seed, self.ascension, Act::get(1)).build();
        println!("{}", node_grid);
        let mut enemy_encounter_generator = EncounterGenerator::new(&self.seed);
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
}
