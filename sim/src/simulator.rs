use crate::data::{Act, Ascension, Character};
use crate::map::{MapBuilder, NodeGrid};
use crate::rng::{EncounterGenerator, Seed};

pub struct Simulator {
    seed: Seed,
    character: Character,
    ascension: Ascension,
    map: NodeGrid,
    encounter_generator: EncounterGenerator,
}

impl Simulator {
    pub fn new(seed: Seed, character: Character, ascension: Ascension) -> Self {
        let map = MapBuilder::from(&seed, ascension, Act::get(1)).build();
        let encounter_generator = EncounterGenerator::new(&seed);
        Self {
            seed,
            character,
            ascension,
            map,
            encounter_generator,
        }
    }

    pub fn run(mut self) {
        println!(
            "Seed: {:?}, Character: {:?}, Ascension: {:?}",
            self.seed, self.character, self.ascension,
        );
        println!("{}", self.map);
        println!(
            "First monster: {:?}",
            self.encounter_generator.next_monster_encounter()
        );
        println!(
            "Second monster: {:?}",
            self.encounter_generator.next_monster_encounter()
        );
        println!(
            "First elite: {:?}",
            self.encounter_generator.next_elite_encounter()
        );
        println!(
            "Act 1 Boss: {:?}",
            self.encounter_generator.next_boss_encounter()
        );
        self.encounter_generator.advance_act();
        println!(
            "Act 2 Boss: {:?}",
            self.encounter_generator.next_boss_encounter()
        );
    }
}
