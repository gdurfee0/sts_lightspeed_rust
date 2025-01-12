mod data;
mod map;
mod params;
mod rng;
mod sim;

use std::io::stdin;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

use anyhow::anyhow;

use crate::params::GameParameters;
use crate::sim::{Choice, Prompt, StsMessage, StsSimulator};

fn main() -> Result<(), anyhow::Error> {
    let (seed, character, ascension) = GameParameters::from_command_line();
    let (input_tx, input_rx) = channel();
    let (output_tx, output_rx) = channel();
    let simulator = StsSimulator::new(seed, character, ascension, input_rx, output_tx);
    let simulator_handle = thread::spawn(move || {
        let _ = simulator.run();
    });
    main_input_loop(input_tx, output_rx)?;
    println!("[Main] Exiting.");
    simulator_handle
        .join()
        .map_err(|e| anyhow!("Simulator thread panicked: {:?}", e))?;
    Ok(())
}

fn main_input_loop(
    input_tx: Sender<usize>,
    output_rx: Receiver<StsMessage>,
) -> Result<(), anyhow::Error> {
    loop {
        match output_rx.recv()? {
            StsMessage::Map(map) => println!("{}\n", map),
            StsMessage::Relics(relics) => {
                println!(
                    "Relics([{}])",
                    relics
                        .iter()
                        .map(|relic| format!("{:?}", relic))
                        .collect::<Vec<_>>()
                        .join(",")
                );
            }
            StsMessage::Deck(deck) => {
                println!(
                    "Deck([{}])",
                    deck.iter()
                        .map(|card| format!("{:?}", card))
                        .collect::<Vec<_>>()
                        .join(",")
                );
            }
            StsMessage::View(view) => println!("{:?}", view),
            StsMessage::Choices(prompt, choices) => {
                input_tx.send(collect_user_choice(prompt, choices)?)?;
            }
            StsMessage::GameOver(result) => {
                println!(
                    "[Main] Game Over; the player was {}victorious",
                    if result { "" } else { "not " }
                );
                break;
            }
            StsMessage::Rewards(prompt, vec) => todo!(),
        }
    }
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
