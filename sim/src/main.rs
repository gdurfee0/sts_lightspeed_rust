mod data;
mod input;
mod map;
mod params;
mod rng;
mod simulator;

use std::io::stdin;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

use anyhow::anyhow;

use crate::input::{Choice, Prompt};
use crate::params::GameParameters;
use crate::simulator::{Simulator, SimulatorOutput};

fn main() {
    let (seed, character, ascension) = GameParameters::from_command_line();
    let (input_tx, input_rx) = channel();
    let (output_tx, output_rx) = channel();
    let simulator = Simulator::new(seed, character, ascension, input_rx, output_tx);
    thread::spawn(move || {
        let _ = simulator.run();
    });
    let _ = main_input_loop(input_tx, output_rx);
}

fn main_input_loop(
    input_tx: Sender<usize>,
    output_rx: Receiver<SimulatorOutput>,
) -> Result<(), anyhow::Error> {
    loop {
        match output_rx.recv() {
            Ok(SimulatorOutput::Map(map)) => println!("{}\n", map),
            Ok(SimulatorOutput::Choose(prompt, choices)) => {
                input_tx.send(collect_user_choice(prompt, choices)?)?;
            }
            Ok(SimulatorOutput::PlayerHp(current, max)) => {
                println!("Player HP: {}/{}", current, max);
            }
            Err(e) => {
                println!("Error receiving output from simulator: {}", e);
                break;
            }
        }
    }
    println!("Main input loop exiting");
    Ok(())
}

fn collect_user_choice(prompt: Prompt, choices: Vec<Choice>) -> Result<usize, anyhow::Error> {
    loop {
        if choices.is_empty() {
            return Err(anyhow!("Cannot choose from an empty list of choices"));
        }
        // Display prompts and choices, converting to 1-indexing for user convenience
        println!("\n{}:", prompt);
        for (i, choice) in choices.iter().enumerate() {
            println!("{}: {}", i + 1, choice);
        }
        let mut user_input = String::new();
        match stdin().read_line(&mut user_input) {
            Ok(0) => {
                return Err(anyhow!("User closed the input stream"));
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
                return Ok(choice - 1);
            }
            Err(e) => return Err(anyhow!("Error reading input: {}", e)),
        }
    }
}
