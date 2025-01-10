mod data;
mod game_context;
mod map;
mod rng;

use map::MapBuilder;

fn main() {
    println!("{}", MapBuilder::for_act(1).build());
}
