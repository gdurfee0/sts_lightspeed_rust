use std::env;

use once_cell::sync::Lazy;

use crate::data::{Ascension, Character};
use crate::rng::Seed;

pub static GAME_PARAMS: Lazy<GameParameters> = Lazy::new(GameParameters::from_args);

#[derive(Debug)]
pub struct GameParameters {
    pub seed: Seed,
    pub character: Character,
    pub ascension: Ascension,
}

impl GameParameters {
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
        Self {
            seed,
            character,
            ascension,
        }
    }
}
