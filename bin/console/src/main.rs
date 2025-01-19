use std::io::stdin;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::{env, thread};

use anyhow::anyhow;
use sts_lib::data::Character;
use sts_lib::message::{Choice, Prompt, StsMessage};
use sts_lib::rng::Seed;
use sts_lib::sim::StsSimulator;

fn main() -> Result<(), anyhow::Error> {
    let (seed, character) = parse_command_line();
    let (to_server, from_client) = channel();
    let (to_client, from_server) = channel();
    let simulator = StsSimulator::new(seed, character, from_client, to_client);
    let simulator_handle = thread::spawn(move || {
        let _ = simulator.run();
    });
    main_input_loop(to_server, from_server)?;
    println!("[Main] Exiting.");
    simulator_handle
        .join()
        .map_err(|e| anyhow!("Simulator thread panicked: {:?}", e))?;
    Ok(())
}

fn main_input_loop(
    to_server: Sender<usize>,
    from_server: Receiver<StsMessage>,
) -> Result<(), anyhow::Error> {
    loop {
        match from_server.recv()? {
            StsMessage::Choices(prompt, choices) => {
                to_server.send(collect_user_choice(prompt, choices)?)?;
            }
            StsMessage::EnemyParty(party) => {
                for enemy_status in party.iter().flatten() {
                    println!("Enemy: {}", enemy_status);
                }
            }
            StsMessage::Map(map) => println!("{}\n", map),
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
            Ok(_) => {
                let index = match user_input.trim().parse::<usize>() {
                    Ok(i) if i <= choices.len() && i > 0 => i,
                    _ => {
                        // Help the user out if this involves movement.
                        if prompt == Prompt::ClimbFloor {
                            if let Some(index) =
                                letter_to_choice(user_input.trim().chars().last(), &choices)
                            {
                                return Ok(index);
                            }
                        }
                        println!(
                            "Invalid input: must be an integer in the range {}..={}",
                            1,
                            choices.len()
                        );
                        continue;
                    }
                };
                return Ok(index - 1);
            }
            Err(e) => return Err(anyhow!("Error reading input: {}", e)),
        }
    }
}

fn letter_to_choice(letter: Option<char>, choices: &[Choice]) -> Option<usize> {
    for (choice_index, choice) in choices.iter().enumerate() {
        match (letter, choice) {
            (Some('a'), Choice::ClimbFloor(0)) => return Some(choice_index),
            (Some('b'), Choice::ClimbFloor(1)) => return Some(choice_index),
            (Some('c'), Choice::ClimbFloor(2)) => return Some(choice_index),
            (Some('d'), Choice::ClimbFloor(3)) => return Some(choice_index),
            (Some('e'), Choice::ClimbFloor(4)) => return Some(choice_index),
            (Some('f'), Choice::ClimbFloor(5)) => return Some(choice_index),
            (Some('g'), Choice::ClimbFloor(6)) => return Some(choice_index),
            _ => {}
        }
    }
    None
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
    if args.next().is_some() {
        panic!("Too many arguments provided");
    }
    (seed, character)
}
