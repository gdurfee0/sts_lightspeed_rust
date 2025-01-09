mod act;
mod ascension;
mod character;
mod game_context;
mod map;
mod monster;
mod random;
mod seed;

use crate::act::Act;
use crate::game_context::GAME_CONTEXT;
use crate::map::MapBuilder;

fn main() {
    println!("Seed: {:?}", GAME_CONTEXT.seed);
    println!("Map for act 1:");
    println!("{}", MapBuilder::for_act(Act(1)).build());
}
