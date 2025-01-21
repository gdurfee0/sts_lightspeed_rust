use std::sync::mpsc::{Receiver, Sender};

use super::encounter::EncounterSimulator;
use super::map::MapSimulator;
use super::neow::NeowSimulator;

use crate::data::{Act, Character};
use crate::map::Room;
use crate::message::StsMessage;
use crate::player::PlayerController;
use crate::rng::{
    CardGenerator, EncounterGenerator, EventGenerator, RelicGenerator, Seed, StsRandom,
};
use crate::types::Floor;

pub struct StsSimulator {
    // Information typically set on the command line
    seed: Seed,
    character: &'static Character,

    // Random number generators for various game elements
    encounter_generator: EncounterGenerator,
    card_generator: CardGenerator,
    misc_rng: StsRandom,
    potion_rng: StsRandom,
    treasure_rng: StsRandom,
    relic_generator: RelicGenerator,
    event_generator: EventGenerator,

    // Connection to the player state and player I/O
    player: PlayerController,
}

impl StsSimulator {
    pub fn new(
        seed: Seed,
        character: &'static Character,
        from_client: Receiver<usize>,
        to_client: Sender<StsMessage>,
    ) -> Self {
        let encounter_generator = EncounterGenerator::new(seed);
        let card_generator = CardGenerator::new(seed, character, Act::get(1));
        let misc_rng = StsRandom::from(seed);
        let potion_rng = StsRandom::from(seed);
        let treasure_rng = StsRandom::from(seed);
        let relic_generator = RelicGenerator::new(seed, character);
        let event_generator = EventGenerator::new(seed);
        let player = PlayerController::new(character, from_client, to_client);
        Self {
            seed,
            character,
            encounter_generator,
            card_generator,
            misc_rng,
            potion_rng,
            treasure_rng,
            relic_generator,
            event_generator,
            player,
        }
    }

    pub fn run(mut self) -> Result<(), anyhow::Error> {
        println!(
            "[Simulator] Starting simulator of size {} with messages of size {}",
            std::mem::size_of::<StsSimulator>(),
            std::mem::size_of::<StsMessage>(),
        );
        self.player.send_full_player_state()?;
        let mut map_simulator = MapSimulator::new(self.seed);
        map_simulator.send_map_to_player(&mut self.player)?;
        let neow_simulator = NeowSimulator::new(
            self.seed,
            self.character,
            &mut self.card_generator,
            &mut self.potion_rng,
            &mut self.relic_generator,
            &mut self.player,
        );
        neow_simulator.run()?;
        let mut floor: Floor = 1;
        loop {
            //self.card_generator = CardGenerator::new(self.seed.with_offset(floor), self.character);
            self.misc_rng = self.seed.with_offset(floor).into();
            match map_simulator.advance(&mut self.player)? {
                Room::Boss => todo!(),
                Room::BurningElite1 => todo!(),
                Room::BurningElite2 => todo!(),
                Room::BurningElite3 => todo!(),
                Room::BurningElite4 => todo!(),
                Room::RestSite => todo!(),
                Room::Elite => todo!(),
                Room::Event => {
                    let (room, maybe_event) = self.event_generator.next_event(
                        floor,
                        self.player.deck(),
                        self.player.gold(),
                        self.player.hp(),
                        self.player.relics(),
                    );
                    println!("? room: {:?}", room);
                    if let Some(event) = maybe_event {
                        println!("Event: {:?}", event);
                    } else if room == Room::Monster {
                        println!(
                            "Monster room: {:?}",
                            self.encounter_generator.next_monster_encounter()
                        );
                    }
                }
                Room::Monster => {
                    if !EncounterSimulator::new(
                        self.seed.with_offset(floor),
                        self.encounter_generator.next_monster_encounter(),
                        &mut self.misc_rng,
                        &mut self.player,
                    )
                    .run()?
                    {
                        break;
                    }
                    let gold_reward = self.treasure_rng.gen_range(10..=20);
                    let card_rewards = self.card_generator.combat_rewards();
                    self.player
                        .choose_combat_rewards(gold_reward, &card_rewards)?;
                }
                Room::Shop => todo!(),
                Room::Treasure => todo!(),
            }
            floor += 1;
        }
        self.player.send_game_over()
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use super::*;

    use std::collections::HashMap;
    use std::sync::mpsc::channel;
    use std::thread;
    use std::time::Duration;

    use crate::data::{
        Card, EnemyCondition, EnemyType, NeowBlessing, NeowBonus, NeowPenalty, PlayerCondition,
        IRONCLAD,
    };
    use crate::enemy::{EnemyStatus, Intent};
    use crate::message::{Choice, Prompt};

    #[track_caller]
    pub fn next_prompt(
        from_server: &Receiver<StsMessage>,
        expected_in_any_order: &[StsMessage],
    ) -> StsMessage {
        let mut expected = HashMap::new();
        for message in expected_in_any_order {
            *expected.entry(message).or_insert_with(|| 0) += 1;
        }
        let mut maybe_message;
        let mut unrecognized_messages = HashMap::new();
        loop {
            let message = from_server.recv_timeout(Duration::from_secs(5)).unwrap();
            if let StsMessage::Choices(_, _) = message {
                maybe_message = Some(message);
                break;
            }
            if expected
                .get_mut(&message)
                .map(|count| *count -= 1)
                .is_none()
            {
                *unrecognized_messages.entry(message).or_insert_with(|| 0) += 1;
            }
            expected.retain(|_, count| *count > 0);
        }
        assert!(
            expected.is_empty(),
            "did not see: {:?}; maybe mismatched one of these? {:?}",
            expected,
            unrecognized_messages
        );
        maybe_message.take().expect("expected a prompt")
    }

    // TODO: This test depends on `EncounterSimulator` behavior. Should test combat separately.
    #[test]
    pub fn test_game_3_ironclad() {
        let seed = Seed::from(3);
        let character = &IRONCLAD;
        let (to_server, from_client) = channel();
        let (to_client, from_server) = channel();
        let simulator = StsSimulator::new(seed, character, from_client, to_client);
        let simulator_thread = thread::spawn(move || simulator.run());

        assert_eq!(
            next_prompt(&from_server, &[StsMessage::Health((80, 80))]),
            StsMessage::Choices(
                Prompt::ChooseNeow,
                vec![
                    Choice::NeowBlessing(NeowBlessing::ChooseCard),
                    Choice::NeowBlessing(NeowBlessing::GainOneHundredGold),
                    Choice::NeowBlessing(NeowBlessing::Composite(
                        NeowBonus::GainTwoHundredFiftyGold,
                        NeowPenalty::TakeDamage
                    )),
                    Choice::NeowBlessing(NeowBlessing::ReplaceStarterRelic)
                ]
            )
        );
        to_server.send(1).unwrap(); // Gain 100 gold
        assert_eq!(
            next_prompt(&from_server, &[]),
            StsMessage::Choices(
                Prompt::ClimbFloor,
                vec![
                    Choice::ClimbFloor(0),
                    Choice::ClimbFloor(1),
                    Choice::ClimbFloor(3)
                ]
            )
        );
        to_server.send(0).unwrap(); // Column 0
        assert_eq!(
            next_prompt(
                &from_server,
                &[
                    StsMessage::StartingCombat,
                    StsMessage::EnemyParty(vec![
                        Some(EnemyStatus::new(
                            EnemyType::AcidSlimeS,
                            (12, 12),
                            Intent::StrategicDebuff
                        )),
                        Some(EnemyStatus::new(
                            EnemyType::SpikeSlimeM,
                            (31, 31),
                            Intent::StrategicDebuff
                        )),
                        None,
                        None,
                        None
                    ]),
                    StsMessage::Energy(3)
                ]
            ),
            StsMessage::Choices(
                Prompt::CombatAction,
                vec![
                    Choice::PlayCardFromHand(0, Card::Strike),
                    Choice::PlayCardFromHand(1, Card::Defend),
                    Choice::PlayCardFromHand(2, Card::Bash),
                    Choice::PlayCardFromHand(3, Card::Defend),
                    Choice::PlayCardFromHand(4, Card::Strike),
                    Choice::EndTurn,
                ]
            )
        );
        to_server.send(0).unwrap(); // Play "Strike"
        assert_eq!(
            next_prompt(&from_server, &[]),
            StsMessage::Choices(
                Prompt::TargetEnemy,
                vec![
                    Choice::TargetEnemy(0, EnemyType::AcidSlimeS),
                    Choice::TargetEnemy(1, EnemyType::SpikeSlimeM),
                ]
            )
        );
        to_server.send(0).unwrap(); // Target AcidSlimeS
        assert_eq!(
            next_prompt(
                &from_server,
                &[
                    StsMessage::Energy(2),
                    StsMessage::EnemyStatus(
                        0,
                        EnemyStatus::new(EnemyType::AcidSlimeS, (6, 12), Intent::StrategicDebuff)
                    )
                ]
            ),
            StsMessage::Choices(
                Prompt::CombatAction,
                vec![
                    Choice::PlayCardFromHand(0, Card::Defend),
                    Choice::PlayCardFromHand(1, Card::Bash),
                    Choice::PlayCardFromHand(2, Card::Defend),
                    Choice::PlayCardFromHand(3, Card::Strike),
                    Choice::EndTurn,
                ]
            )
        );
        to_server.send(3).unwrap(); // Play "Strike"
        assert_eq!(
            next_prompt(&from_server, &[]),
            StsMessage::Choices(
                Prompt::TargetEnemy,
                vec![
                    Choice::TargetEnemy(0, EnemyType::AcidSlimeS),
                    Choice::TargetEnemy(1, EnemyType::SpikeSlimeM),
                ]
            )
        );
        to_server.send(0).unwrap(); // Target AcidSlimeS
        assert_eq!(
            next_prompt(
                &from_server,
                &[
                    StsMessage::Energy(1),
                    StsMessage::EnemyDied(0, EnemyType::AcidSlimeS) // AcidSlimeS dies
                ]
            ),
            StsMessage::Choices(
                Prompt::CombatAction,
                vec![
                    Choice::PlayCardFromHand(0, Card::Defend),
                    Choice::PlayCardFromHand(2, Card::Defend),
                    Choice::EndTurn,
                ]
            )
        );
        to_server.send(2).unwrap(); // End Turn
        assert_eq!(
            next_prompt(
                &from_server,
                &[
                    // 1 stack of Frail on player
                    StsMessage::Conditions(vec![PlayerCondition::Frail(1)]),
                    StsMessage::EnemyParty(vec![
                        None, // AcidSlimeS now gone
                        Some(EnemyStatus::new(
                            EnemyType::SpikeSlimeM,
                            (31, 31),
                            Intent::Aggressive(8, 1)
                        )),
                        None,
                        None,
                        None
                    ])
                ]
            ),
            StsMessage::Choices(
                Prompt::CombatAction,
                vec![
                    Choice::PlayCardFromHand(0, Card::Strike),
                    Choice::PlayCardFromHand(1, Card::Strike),
                    Choice::PlayCardFromHand(2, Card::Strike),
                    Choice::PlayCardFromHand(3, Card::Defend),
                    Choice::PlayCardFromHand(4, Card::Defend),
                    Choice::EndTurn,
                ]
            )
        );
        to_server.send(3).unwrap(); // Play "Defend"
        assert_eq!(
            next_prompt(&from_server, &[StsMessage::BlockGained(3)]), // 5 -> 3 because Frail
            StsMessage::Choices(
                Prompt::CombatAction,
                vec![
                    Choice::PlayCardFromHand(0, Card::Strike),
                    Choice::PlayCardFromHand(1, Card::Strike),
                    Choice::PlayCardFromHand(2, Card::Strike),
                    Choice::PlayCardFromHand(3, Card::Defend),
                    Choice::EndTurn,
                ]
            )
        );
        to_server.send(3).unwrap(); // Play "Defend"
        assert_eq!(
            next_prompt(&from_server, &[StsMessage::BlockGained(3)]),
            StsMessage::Choices(
                Prompt::CombatAction,
                vec![
                    Choice::PlayCardFromHand(0, Card::Strike),
                    Choice::PlayCardFromHand(1, Card::Strike),
                    Choice::PlayCardFromHand(2, Card::Strike),
                    Choice::EndTurn,
                ]
            )
        );
        to_server.send(3).unwrap(); // End Turn
        assert_eq!(
            next_prompt(
                &from_server,
                &[
                    StsMessage::BlockLost(6),     // Attack took off all block
                    StsMessage::Block(0),         // No block remaining
                    StsMessage::Health((78, 80)), // Took 2 damage
                    StsMessage::EnemyParty(vec![
                        None,
                        Some(EnemyStatus::new(
                            EnemyType::SpikeSlimeM,
                            (31, 31),
                            Intent::StrategicDebuff
                        )),
                        None,
                        None,
                        None
                    ])
                ]
            ),
            StsMessage::Choices(
                Prompt::CombatAction,
                vec![
                    Choice::PlayCardFromHand(0, Card::Defend),
                    Choice::PlayCardFromHand(1, Card::Slimed),
                    Choice::PlayCardFromHand(2, Card::Strike),
                    Choice::PlayCardFromHand(3, Card::Bash),
                    Choice::PlayCardFromHand(4, Card::Strike),
                    Choice::EndTurn,
                ]
            )
        );
        to_server.send(3).unwrap(); // Play "Bash"
        assert_eq!(
            next_prompt(&from_server, &[]),
            StsMessage::Choices(
                Prompt::TargetEnemy,
                vec![Choice::TargetEnemy(1, EnemyType::SpikeSlimeM)]
            )
        );
        to_server.send(0).unwrap(); // Target SpikeSlimeM
        assert_eq!(
            next_prompt(
                &from_server,
                &[
                    StsMessage::Energy(1),
                    StsMessage::EnemyStatus(
                        1,
                        EnemyStatus::new(EnemyType::SpikeSlimeM, (23, 31), Intent::StrategicDebuff)
                            .with_condition(EnemyCondition::Vulnerable(2))
                    )
                ]
            ),
            StsMessage::Choices(
                Prompt::CombatAction,
                vec![
                    Choice::PlayCardFromHand(0, Card::Defend),
                    Choice::PlayCardFromHand(1, Card::Slimed),
                    Choice::PlayCardFromHand(2, Card::Strike),
                    Choice::PlayCardFromHand(3, Card::Strike),
                    Choice::EndTurn,
                ]
            )
        );
        to_server.send(1).unwrap(); // Play "Slimed"
        assert_eq!(
            next_prompt(&from_server, &[StsMessage::CardExhausted(1, Card::Slimed)]),
            StsMessage::Choices(Prompt::CombatAction, vec![Choice::EndTurn])
        );
        to_server.send(0).unwrap(); // End Turn
        assert_eq!(
            next_prompt(
                &from_server,
                &[
                    // 1 stack of Frail on player
                    StsMessage::Conditions(vec![PlayerCondition::Frail(1)]),
                    StsMessage::EnemyParty(vec![
                        None,
                        Some(
                            EnemyStatus::new(
                                EnemyType::SpikeSlimeM,
                                (23, 31),
                                Intent::StrategicDebuff
                            )
                            // One stack of Vulnerable drops off by the next turn
                            .with_condition(EnemyCondition::Vulnerable(1))
                        ),
                        None,
                        None,
                        None
                    ])
                ]
            ),
            StsMessage::Choices(
                Prompt::CombatAction,
                vec![
                    Choice::PlayCardFromHand(0, Card::Defend),
                    Choice::PlayCardFromHand(1, Card::Defend),
                    Choice::PlayCardFromHand(2, Card::Strike),
                    Choice::PlayCardFromHand(3, Card::Strike),
                    Choice::PlayCardFromHand(4, Card::Strike),
                    Choice::EndTurn,
                ]
            )
        );
        to_server.send(2).unwrap(); // Strike
        assert_eq!(
            next_prompt(&from_server, &[]),
            StsMessage::Choices(
                Prompt::TargetEnemy,
                vec![Choice::TargetEnemy(1, EnemyType::SpikeSlimeM)]
            )
        );
        to_server.send(0).unwrap(); // Target SpikeSlimeM
        assert_eq!(
            next_prompt(
                &from_server,
                &[
                    StsMessage::Energy(2),
                    StsMessage::EnemyStatus(
                        1,
                        EnemyStatus::new(EnemyType::SpikeSlimeM, (14, 31), Intent::StrategicDebuff)
                            .with_condition(EnemyCondition::Vulnerable(1))
                    )
                ]
            ),
            StsMessage::Choices(
                Prompt::CombatAction,
                vec![
                    Choice::PlayCardFromHand(0, Card::Defend),
                    Choice::PlayCardFromHand(1, Card::Defend),
                    Choice::PlayCardFromHand(2, Card::Strike),
                    Choice::PlayCardFromHand(3, Card::Strike),
                    Choice::EndTurn,
                ]
            )
        );
        to_server.send(2).unwrap(); // Strike
        assert_eq!(
            next_prompt(&from_server, &[]),
            StsMessage::Choices(
                Prompt::TargetEnemy,
                vec![Choice::TargetEnemy(1, EnemyType::SpikeSlimeM)]
            )
        );
        to_server.send(0).unwrap(); // Target SpikeSlimeM
        assert_eq!(
            next_prompt(
                &from_server,
                &[
                    StsMessage::Energy(1),
                    StsMessage::EnemyStatus(
                        1,
                        EnemyStatus::new(EnemyType::SpikeSlimeM, (5, 31), Intent::StrategicDebuff)
                            .with_condition(EnemyCondition::Vulnerable(1))
                    )
                ]
            ),
            StsMessage::Choices(
                Prompt::CombatAction,
                vec![
                    Choice::PlayCardFromHand(0, Card::Defend),
                    Choice::PlayCardFromHand(1, Card::Defend),
                    Choice::PlayCardFromHand(2, Card::Strike),
                    Choice::EndTurn,
                ]
            )
        );
        to_server.send(2).unwrap(); // Strike
        assert_eq!(
            next_prompt(&from_server, &[]),
            StsMessage::Choices(
                Prompt::TargetEnemy,
                vec![Choice::TargetEnemy(1, EnemyType::SpikeSlimeM)]
            )
        );
        to_server.send(0).unwrap(); // Target SpikeSlimeM
        assert_eq!(
            next_prompt(
                &from_server,
                &[
                    StsMessage::EnemyDied(1, EnemyType::SpikeSlimeM),
                    StsMessage::Health((80, 80)), // Burning Blood heals 6
                    StsMessage::EndingCombat
                ]
            ),
            StsMessage::Choices(
                Prompt::ChooseNext,
                vec![
                    Choice::ObtainGold(11),
                    Choice::ObtainCard(Card::Thunderclap),
                    Choice::ObtainCard(Card::HeavyBlade),
                    Choice::ObtainCard(Card::Armaments)
                ]
            )
        );
        to_server.send(0).unwrap(); // Take Gold
        assert_eq!(
            next_prompt(&from_server, &[StsMessage::Gold(99 + 100 + 11)]),
            StsMessage::Choices(
                Prompt::ChooseOne,
                vec![
                    Choice::ObtainCard(Card::Thunderclap),
                    Choice::ObtainCard(Card::HeavyBlade),
                    Choice::ObtainCard(Card::Armaments)
                ]
            )
        );
        to_server.send(0).unwrap(); // Take Thunderclap
        assert_eq!(
            next_prompt(&from_server, &[StsMessage::CardObtained(Card::Thunderclap)]),
            StsMessage::Choices(Prompt::ClimbFloor, vec![Choice::ClimbFloor(0)])
        );
        to_server.send(0).unwrap(); // Column 0
        assert_eq!(
            next_prompt(
                &from_server,
                &[
                    StsMessage::StartingCombat,
                    StsMessage::Energy(3),
                    StsMessage::EnemyParty(vec![
                        Some(EnemyStatus::new(
                            EnemyType::Cultist,
                            (50, 50),
                            Intent::StrategicBuff
                        )),
                        None,
                        None,
                        None,
                        None
                    ])
                ]
            ),
            StsMessage::Choices(
                Prompt::CombatAction,
                vec![
                    Choice::PlayCardFromHand(0, Card::Strike),
                    Choice::PlayCardFromHand(1, Card::Strike),
                    Choice::PlayCardFromHand(2, Card::Strike),
                    Choice::PlayCardFromHand(3, Card::Defend),
                    Choice::PlayCardFromHand(4, Card::Defend),
                    Choice::EndTurn,
                ]
            )
        );
        to_server.send(0).unwrap(); // Play "Strike"
        assert_eq!(
            next_prompt(&from_server, &[]),
            StsMessage::Choices(
                Prompt::TargetEnemy,
                vec![Choice::TargetEnemy(0, EnemyType::Cultist)]
            )
        );
        to_server.send(0).unwrap(); // Target Cultist
        assert_eq!(
            next_prompt(
                &from_server,
                &[
                    StsMessage::Energy(2),
                    StsMessage::EnemyStatus(
                        0,
                        EnemyStatus::new(EnemyType::Cultist, (44, 50), Intent::StrategicBuff)
                    )
                ]
            ),
            StsMessage::Choices(
                Prompt::CombatAction,
                vec![
                    Choice::PlayCardFromHand(0, Card::Strike),
                    Choice::PlayCardFromHand(1, Card::Strike),
                    Choice::PlayCardFromHand(2, Card::Defend),
                    Choice::PlayCardFromHand(3, Card::Defend),
                    Choice::EndTurn,
                ]
            )
        );
        to_server.send(0).unwrap(); // Play "Strike"
        assert_eq!(
            next_prompt(&from_server, &[]),
            StsMessage::Choices(
                Prompt::TargetEnemy,
                vec![Choice::TargetEnemy(0, EnemyType::Cultist)]
            )
        );
        to_server.send(0).unwrap(); // Target Cultist
        assert_eq!(
            next_prompt(
                &from_server,
                &[
                    StsMessage::Energy(1),
                    StsMessage::EnemyStatus(
                        0,
                        EnemyStatus::new(EnemyType::Cultist, (38, 50), Intent::StrategicBuff)
                    )
                ]
            ),
            StsMessage::Choices(
                Prompt::CombatAction,
                vec![
                    Choice::PlayCardFromHand(0, Card::Strike),
                    Choice::PlayCardFromHand(1, Card::Defend),
                    Choice::PlayCardFromHand(2, Card::Defend),
                    Choice::EndTurn,
                ]
            )
        );
        to_server.send(0).unwrap(); // Play "Strike"
        assert_eq!(
            next_prompt(&from_server, &[]),
            StsMessage::Choices(
                Prompt::TargetEnemy,
                vec![Choice::TargetEnemy(0, EnemyType::Cultist)]
            )
        );
        to_server.send(0).unwrap(); // Target Cultist
        assert_eq!(
            next_prompt(
                &from_server,
                &[
                    StsMessage::Energy(0),
                    StsMessage::EnemyStatus(
                        0,
                        EnemyStatus::new(EnemyType::Cultist, (32, 50), Intent::StrategicBuff)
                    )
                ]
            ),
            StsMessage::Choices(Prompt::CombatAction, vec![Choice::EndTurn])
        );
        to_server.send(0).unwrap(); // End Turn
        assert_eq!(
            next_prompt(
                &from_server,
                &[
                    StsMessage::Energy(3),
                    StsMessage::EnemyParty(vec![
                        Some(
                            EnemyStatus::new(
                                EnemyType::Cultist,
                                (32, 50),
                                Intent::Aggressive(6, 1)
                            )
                            .with_condition(EnemyCondition::Ritual(3, false))
                        ),
                        None,
                        None,
                        None,
                        None
                    ])
                ]
            ),
            StsMessage::Choices(
                Prompt::CombatAction,
                vec![
                    Choice::PlayCardFromHand(0, Card::Defend),
                    Choice::PlayCardFromHand(1, Card::Strike),
                    Choice::PlayCardFromHand(2, Card::Bash),
                    Choice::PlayCardFromHand(3, Card::Defend),
                    Choice::PlayCardFromHand(4, Card::Strike),
                    Choice::EndTurn,
                ]
            )
        );
        to_server.send(2).unwrap(); // Play "Bash"
        assert_eq!(
            next_prompt(&from_server, &[]),
            StsMessage::Choices(
                Prompt::TargetEnemy,
                vec![Choice::TargetEnemy(0, EnemyType::Cultist)]
            )
        );
        to_server.send(0).unwrap(); // Target Cultist
        assert_eq!(
            next_prompt(
                &from_server,
                &[
                    StsMessage::Energy(1),
                    StsMessage::EnemyStatus(
                        0,
                        EnemyStatus::new(EnemyType::Cultist, (24, 50), Intent::Aggressive(6, 1))
                            .with_condition(EnemyCondition::Ritual(3, false))
                            .with_condition(EnemyCondition::Vulnerable(2))
                    )
                ]
            ),
            StsMessage::Choices(
                Prompt::CombatAction,
                vec![
                    Choice::PlayCardFromHand(0, Card::Defend),
                    Choice::PlayCardFromHand(1, Card::Strike),
                    Choice::PlayCardFromHand(2, Card::Defend),
                    Choice::PlayCardFromHand(3, Card::Strike),
                    Choice::EndTurn,
                ]
            )
        );
        to_server.send(1).unwrap(); // Play "Strike"
        assert_eq!(
            next_prompt(&from_server, &[]),
            StsMessage::Choices(
                Prompt::TargetEnemy,
                vec![Choice::TargetEnemy(0, EnemyType::Cultist)]
            )
        );
        to_server.send(0).unwrap(); // Target Cultist
        assert_eq!(
            next_prompt(
                &from_server,
                &[
                    StsMessage::Energy(0),
                    StsMessage::EnemyStatus(
                        0,
                        EnemyStatus::new(EnemyType::Cultist, (15, 50), Intent::Aggressive(6, 1))
                            .with_condition(EnemyCondition::Ritual(3, false))
                            .with_condition(EnemyCondition::Vulnerable(2))
                    )
                ]
            ),
            StsMessage::Choices(Prompt::CombatAction, vec![Choice::EndTurn])
        );
        to_server.send(0).unwrap(); // End Turn
        assert_eq!(
            next_prompt(
                &from_server,
                &[
                    StsMessage::Energy(3),
                    StsMessage::EnemyParty(vec![
                        Some(
                            EnemyStatus::new(
                                EnemyType::Cultist,
                                (15, 50),
                                Intent::Aggressive(6, 1)
                            )
                            .with_condition(EnemyCondition::Ritual(3, false))
                            .with_condition(EnemyCondition::Vulnerable(1))
                            .with_strength(3)
                        ),
                        None,
                        None,
                        None,
                        None
                    ])
                ]
            ),
            StsMessage::Choices(
                Prompt::CombatAction,
                vec![
                    Choice::PlayCardFromHand(0, Card::Thunderclap),
                    Choice::PlayCardFromHand(1, Card::Defend),
                    Choice::PlayCardFromHand(2, Card::Defend),
                    Choice::PlayCardFromHand(3, Card::Strike),
                    Choice::PlayCardFromHand(4, Card::Defend),
                    Choice::EndTurn,
                ]
            )
        );
        to_server.send(0).unwrap(); // Play "Thunderclap"
        assert_eq!(
            next_prompt(
                &from_server,
                &[
                    StsMessage::Energy(2),
                    StsMessage::EnemyStatus(
                        0,
                        EnemyStatus::new(EnemyType::Cultist, (9, 50), Intent::Aggressive(6, 1))
                            .with_condition(EnemyCondition::Ritual(3, false))
                            .with_condition(EnemyCondition::Vulnerable(2))
                            .with_strength(3)
                    )
                ]
            ),
            StsMessage::Choices(
                Prompt::CombatAction,
                vec![
                    Choice::PlayCardFromHand(0, Card::Defend),
                    Choice::PlayCardFromHand(1, Card::Defend),
                    Choice::PlayCardFromHand(2, Card::Strike),
                    Choice::PlayCardFromHand(3, Card::Defend),
                    Choice::EndTurn,
                ]
            )
        );
        to_server.send(0).unwrap(); // Play "Defend"
        assert_eq!(
            next_prompt(
                &from_server,
                &[
                    StsMessage::Energy(1),
                    StsMessage::BlockGained(5),
                    StsMessage::Block(5)
                ]
            ),
            StsMessage::Choices(
                Prompt::CombatAction,
                vec![
                    Choice::PlayCardFromHand(0, Card::Defend),
                    Choice::PlayCardFromHand(1, Card::Strike),
                    Choice::PlayCardFromHand(2, Card::Defend),
                    Choice::EndTurn,
                ]
            )
        );
        to_server.send(0).unwrap(); // Play "Defend"
        assert_eq!(
            next_prompt(
                &from_server,
                &[
                    StsMessage::Energy(0),
                    StsMessage::BlockGained(5),
                    StsMessage::Block(10)
                ]
            ),
            StsMessage::Choices(Prompt::CombatAction, vec![Choice::EndTurn])
        );
        to_server.send(0).unwrap(); // End Turn
        assert_eq!(
            next_prompt(
                &from_server,
                &[
                    StsMessage::Energy(3),
                    StsMessage::BlockLost(9),
                    StsMessage::Block(1),
                    StsMessage::Block(0),
                    StsMessage::EnemyParty(vec![
                        Some(
                            EnemyStatus::new(EnemyType::Cultist, (9, 50), Intent::Aggressive(6, 1))
                                .with_condition(EnemyCondition::Ritual(3, false))
                                .with_condition(EnemyCondition::Vulnerable(1))
                                .with_strength(6)
                        ),
                        None,
                        None,
                        None,
                        None
                    ])
                ]
            ),
            StsMessage::Choices(
                Prompt::CombatAction,
                vec![
                    Choice::PlayCardFromHand(0, Card::Strike),
                    Choice::PlayCardFromHand(1, Card::Strike),
                    Choice::PlayCardFromHand(2, Card::Strike),
                    Choice::PlayCardFromHand(3, Card::Defend),
                    Choice::PlayCardFromHand(4, Card::Bash),
                    Choice::EndTurn,
                ]
            )
        );
        to_server.send(0).unwrap(); // Play "Strike"
        assert_eq!(
            next_prompt(&from_server, &[]),
            StsMessage::Choices(
                Prompt::TargetEnemy,
                vec![Choice::TargetEnemy(0, EnemyType::Cultist)]
            )
        );
        to_server.send(0).unwrap(); // Target Cultist
        assert_eq!(
            next_prompt(
                &from_server,
                &[
                    StsMessage::EnemyDied(0, EnemyType::Cultist),
                    StsMessage::Health((80, 80)), // Burning Blood heals 6
                    StsMessage::EndingCombat
                ]
            ),
            StsMessage::Choices(
                Prompt::ChooseNext,
                vec![
                    Choice::ObtainGold(10),
                    Choice::ObtainCard(Card::Anger),
                    Choice::ObtainCard(Card::HeavyBlade),
                    Choice::ObtainCard(Card::Armaments)
                ]
            )
        );
        to_server.send(0).unwrap(); // Take Gold
        assert_eq!(
            next_prompt(&from_server, &[StsMessage::Gold(99 + 100 + 11 + 10)]),
            StsMessage::Choices(
                Prompt::ChooseOne,
                vec![
                    Choice::ObtainCard(Card::Anger),
                    Choice::ObtainCard(Card::HeavyBlade),
                    Choice::ObtainCard(Card::Armaments)
                ]
            )
        );
        to_server.send(2).unwrap(); // Take Armaments
        assert_eq!(
            next_prompt(&from_server, &[StsMessage::CardObtained(Card::Armaments)]),
            StsMessage::Choices(
                Prompt::ClimbFloor,
                vec![Choice::ClimbFloor(0), Choice::ClimbFloor(1)]
            )
        );
        to_server.send(0).unwrap(); // Column 0

        /*
        assert_eq!(
            next_prompt(&from_server, &[]),
            StsMessage::Choices(Prompt::ClimbFloor, vec![Choice::ClimbFloor(0)])
        );
        to_server.send(0).unwrap(); // Column 0
        assert_eq!(
            next_prompt(&from_server, &[]),
            StsMessage::Choices(Prompt::ClimbFloor, vec![Choice::ClimbFloor(0)])
        );
        to_server.send(0).unwrap(); // Column 0
        assert_eq!(
            next_prompt(&from_server, &[]),
            StsMessage::Choices(
                Prompt::ClimbFloor,
                vec![Choice::ClimbFloor(0), Choice::ClimbFloor(1)]
            )
        );
        to_server.send(1).unwrap(); // Column 1
        assert_eq!(
            next_prompt(&from_server, &[]),
            StsMessage::Choices(
                Prompt::ClimbFloor,
                vec![
                    Choice::ClimbFloor(0),
                    Choice::ClimbFloor(1),
                    Choice::ClimbFloor(2)
                ]
            )
        );
        to_server.send(2).unwrap(); // Column 2
        assert_eq!(
            next_prompt(&from_server, &[]),
            StsMessage::Choices(
                Prompt::ClimbFloor,
                vec![Choice::ClimbFloor(2), Choice::ClimbFloor(3)]
            )
        );
        to_server.send(1).unwrap(); // Column 3

        assert_eq!(next_prompt(&from_server, &[]), StsMessage::GameOver(true));
        */
        drop(to_server);
        let _ = simulator_thread.join();
    }
}
