use std::env;

use crate::ascension::Ascension;
use crate::character::Character;
use crate::seed::Seed;

#[derive(Debug)]
pub struct GameContext {
    seed: Seed,
    character: Character,
    ascension: Ascension,
}

impl GameContext {
    pub fn from_args() -> Self {
        let mut args = env::args();
        args.next(); // Skip the program name
        let seed = args
            .next()
            .unwrap_or_else(|| panic!("No seed provided"))
            .try_into()
            .unwrap_or_else(|e| panic!("Invalid seed: {}", e));
        let character = args
            .next()
            .unwrap_or_else(|| panic!("No character provided"))
            .try_into()
            .unwrap_or_else(|e| panic!("Invalid character: {}", e));
        let ascension = args
            .next()
            .unwrap_or_else(|| panic!("No ascension level provided"))
            .try_into()
            .unwrap_or_else(|e| panic!("Invalid ascension: {}", e));
        Self {
            seed,
            character,
            ascension,
        }
    }
}
