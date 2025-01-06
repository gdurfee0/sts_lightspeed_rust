mod ascension;
mod character;
mod game_context;
mod seed;

use crate::game_context::GameContext;

fn main() {
    println!("Game Context: {:?}", GameContext::from_args());
}
