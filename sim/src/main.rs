mod ascension;
mod character;
mod game_context;
mod map;
mod random;
mod seed;

use crate::game_context::GameContext;
use crate::random::JavaRandom;

fn main() {
    let game_context = GameContext::from_args();
    let mut java_rng: JavaRandom = game_context.seed.into();
    for _ in 0..10 {
        println!("{}", java_rng.next_i32());
    }
    for i in 0..10 {
        println!("{}", java_rng.next_i32_bounded(42 + (1 << i)));
    }
}
