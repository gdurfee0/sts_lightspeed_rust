use std::env;

use once_cell::sync::Lazy;

use crate::ascension::Ascension;
use crate::character::Character;
use crate::rng::Seed;

pub static GAME_CONTEXT: Lazy<GameContext> = Lazy::new(GameContext::from_args);

#[derive(Debug)]
pub struct GameContext {
    pub seed: Seed,
    pub character: Character,
    pub ascension: Ascension,
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
        Self {
            seed,
            character,
            ascension,
        }
    }
}
