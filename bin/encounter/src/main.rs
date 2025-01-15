use std::io::stdin;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::{env, thread};

use anyhow::anyhow;
use lib::{Character, Choice, Prompt, Seed, StsMessage, StsSimulator};

fn main() -> Result<(), anyhow::Error> {
    let (seed, character) = parse_command_line();
    let (input_tx, input_rx) = channel();
    let (output_tx, output_rx) = channel();
    let simulator = StsSimulator::new(seed, character, input_rx, output_tx);
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
            sts_message => println!("{:?}", sts_message),
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
            Ok(_) => match user_input.trim().parse::<usize>() {
                Ok(i) if i <= choices.len() && i > 0 => return Ok(i - 1),
                _ => {
                    println!(
                        "Invalid input: must be an integer in the range {}..={}",
                        1,
                        choices.len()
                    );
                }
            },
            Err(e) => return Err(anyhow!("Error reading input: {}", e)),
        }
    }
}

fn parse_command_line() -> (Seed, &'static Character) {
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
