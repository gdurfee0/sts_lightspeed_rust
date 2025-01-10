mod data;
mod input;
mod map;
mod params;
mod rng;
mod simulator;

use std::io::stdin;
use std::sync::mpsc::channel;
use std::thread;

use crate::params::GameParameters;
use crate::simulator::{Simulator, SimulatorOutput};

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
            Ok(SimulatorOutput::Map(map)) => println!("{}\n", map),
            Ok(SimulatorOutput::Choose(prompt, choices)) => loop {
                if choices.is_empty() {
                    panic!("Cannot choose from an empty list of choices");
                }
                println!("{}:", prompt);
                for (i, choice) in choices.iter().enumerate() {
                    println!("{}: {}", i + 1, choice);
                }
                let mut user_input = String::new();
                match stdin().read_line(&mut user_input) {
                    Ok(0) => {
                        println!("User closed the input stream");
                        break;
                    }
                    Ok(_) => {
                        let choice = match user_input.trim().parse::<usize>() {
                            Ok(i) if i <= choices.len() && i > 0 => i,
                            _ => {
                                println!(
                                    "Invalid input: must be an integer in the range {}..={}",
                                    1,
                                    choices.len()
                                );
                                continue;
                            }
                        };
                        if let Err(e) = input_tx.send(choice - 1) {
                            panic!("Error sending input: {}", e);
                        }
                        break;
                    }
                    Err(e) => panic!("Error reading input: {}", e),
                }
            },
            Err(e) => panic!("Error receiving output: {}", e),
        }
    }
}
