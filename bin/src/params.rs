use std::env;

use lib::data::Character;
use lib::rng::Seed;

pub struct GameParameters;

impl GameParameters {
    pub fn from_command_line() -> (Seed, &'static Character) {
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
        (seed, character)
    }
}