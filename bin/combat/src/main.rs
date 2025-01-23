use std::sync::mpsc::channel;
use std::{env, thread};

mod combat;

use anyhow::anyhow;
use combat::str_to_encounter;
use sts_lib::data::{Character, Encounter};
use sts_lib::types::Floor;
use sts_lib::ui::combat::CombatClient;
use sts_lib::{Seed, StsSimulator};

fn main() -> Result<(), anyhow::Error> {
    let (seed, character, floor, encounter) = parse_command_line();
    let (to_server, from_client) = channel();
    let (to_client, from_server) = channel();
    let mut simulator = StsSimulator::new(seed, character, from_client, to_client);
    let simulator_handle = thread::spawn(move || {
        let _ = simulator.run_encounter(floor, encounter);
    });
    let game_controller = CombatClient::new(&from_server, &to_server);
    game_controller.run()?;
    println!("[Main] Exiting.");
    simulator_handle
        .join()
        .map_err(|e| anyhow!("Simulator thread panicked: {:?}", e))?;
    Ok(())
}

fn parse_command_line() -> (Seed, &'static Character, Floor, Encounter) {
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
    let floor = args
        .next()
        .unwrap_or_else(|| panic!("No floor provided"))
        .parse::<u64>()
        .unwrap_or_else(|e| panic!("Invalid floor: {}", e));
    let encounter = str_to_encounter(
        args.next()
            .unwrap_or_else(|| panic!("No encounter provided"))
            .as_str(),
    );
    if args.next().is_some() {
        panic!("Too many arguments provided");
    }
    (seed, character, floor, encounter)
}
