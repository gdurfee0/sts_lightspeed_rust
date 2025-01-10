mod data;
mod map;
mod params;
mod rng;
mod simulator;

use crate::params::GameParameters;
use crate::simulator::Simulator;

fn main() {
    let (seed, character, ascension) = GameParameters::from_command_line();
    let simulator = Simulator::new(seed, character, ascension);
    simulator.run();
}
