mod data;
mod map;
mod params;
mod rng;
mod simulator;

use std::io::stdin;
use std::sync::mpsc::channel;
use std::thread;

use crate::params::GameParameters;
use crate::simulator::{Simulator, SimulatorInput, SimulatorOutput};

fn main() {
    let (seed, character, ascension) = GameParameters::from_command_line();
    let (input_tx, input_rx) = channel();
    let (output_tx, output_rx) = channel();
    let simulator = Simulator::new(seed, character, ascension, input_rx, output_tx);
    thread::spawn(move || {
        simulator.run();
    });
    loop {
        match output_rx.recv() {
            Ok(SimulatorOutput::Map(map)) => println!("{}", map),
            Ok(SimulatorOutput::AwaitingInput(state)) => {
                println!("{:?}", state);
                let mut user_input = String::new();
                match stdin().read_line(&mut user_input) {
                    Ok(_) => {
                        if let Err(e) = input_tx.send(user_input) {
                            panic!("Error sending input: {}", e);
                        }
                    }
                    Err(e) => panic!("Error reading input: {}", e),
                }
            }
            Err(e) => panic!("Error receiving output: {}", e),
        }
    }
}
