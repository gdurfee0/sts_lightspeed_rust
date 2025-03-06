use std::sync::mpsc::{Receiver, Sender};

use anyhow::Error;

use crate::components::{Choice, Interaction, PlayerPersistentState, Prompt, Room, StsMessage};
use crate::data::{Act, Character, Encounter};
use crate::systems::base::HealthSystem;
use crate::systems::rng::{
    CardGenerator, EncounterGenerator, EventGenerator, PotionGenerator, RelicGenerator, Seed,
    StsRandom,
};
use crate::types::{Floor, Hp};

use super::combat_simulator::CombatSimulator;
use super::event_simulator::EventSimulator;
use super::main_screen_system::MainScreenSystem;
use super::map_navigation_simulator::MapNavigationSimulator;
use super::neow_simulator::NeowSimulator;
use super::player_interaction::PlayerInteraction;

pub struct StsSimulator {
    // Information typically set on the command line
    seed: Seed,
    character: &'static Character,

    // Random number generators for various game elements
    card_generator: CardGenerator,
    encounter_generator: EncounterGenerator,
    event_generator: EventGenerator,
    potion_generator: PotionGenerator,
    relic_generator: RelicGenerator,
    misc_rng: StsRandom,
    treasure_rng: StsRandom,
}

impl StsSimulator {
    pub fn new(seed: Seed, character: &'static Character) -> Self {
        let card_generator = CardGenerator::new(seed, character, Act::get(1));
        let encounter_generator = EncounterGenerator::new(seed);
        let event_generator = EventGenerator::new(seed);
        let potion_generator = PotionGenerator::new(seed, character);
        let relic_generator = RelicGenerator::new(seed, character);
        let misc_rng = StsRandom::from(seed);
        let treasure_rng = StsRandom::from(seed);
        Self {
            seed,
            character,
            card_generator,
            encounter_generator,
            event_generator,
            potion_generator,
            relic_generator,
            misc_rng,
            treasure_rng,
        }
    }

    pub fn run(
        mut self,
        from_client: Receiver<usize>,
        to_client: Sender<StsMessage>,
    ) -> Result<(), Error> {
        let comms = PlayerInteraction::new(from_client, to_client);
        let mut pps = PlayerPersistentState::new(self.character);
        MainScreenSystem::notify_player(&comms, &pps)?;
        println!(
            "[Simulator] Starting simulator of size {} with messages of size {}",
            std::mem::size_of::<StsSimulator>(),
            std::mem::size_of::<StsMessage>(),
        );
        // todo - self.player.send_full_player_state()?;
        let mut map_simulator = MapNavigationSimulator::new(self.seed, &comms);
        map_simulator.send_map_to_player()?;
        let neow_simulator = NeowSimulator::new(
            self.seed,
            self.character,
            &mut self.card_generator,
            &mut self.potion_generator,
            &mut self.relic_generator,
            &comms,
        );
        neow_simulator.run(&mut pps)?;
        let mut floor = 1;
        loop {
            self.misc_rng = self.seed.with_offset(floor).into();
            let room = map_simulator.advance(&mut pps)?;
            match room {
                Room::Boss => {
                    let boss = self.encounter_generator.next_boss_encounter();
                    if !self.run_encounter(&comms, floor, boss, &mut pps, None)? {
                        break;
                    }
                }
                Room::RestSite => {
                    let choices = vec![Choice::Rest, Choice::Smith];
                    match comms.prompt_for_choice(Prompt::ChooseRestSiteAction, &choices)? {
                        Choice::Rest => {
                            let heal_amt = (pps.hp_max as f32 * 0.3).floor() as Hp;
                            HealthSystem::heal(&comms, &mut pps, heal_amt)?;
                        }
                        Choice::Smith => todo!(),
                        invalid => unreachable!("{:?}", invalid),
                    }
                }
                Room::BurningElite1
                | Room::BurningElite2
                | Room::BurningElite3
                | Room::BurningElite4
                | Room::Elite => {
                    let encounter = self.encounter_generator.next_elite_encounter();
                    if !self.run_encounter(
                        &comms,
                        floor,
                        encounter,
                        &mut pps,
                        match room {
                            Room::BurningElite1 => Some(1),
                            Room::BurningElite2 => Some(2),
                            Room::BurningElite3 => Some(3),
                            Room::BurningElite4 => Some(4),
                            _ => None,
                        },
                    )? {
                        break;
                    }
                }
                Room::Event => match self.event_generator.next_event(floor, &pps) {
                    (Room::Event, Some(event)) => {
                        EventSimulator::new(&comms, &mut self.potion_generator)
                            .run_event(event, &mut pps)?
                    }
                    (Room::Monster, None) => {
                        let encounter = self.encounter_generator.next_monster_encounter();
                        if !self.run_encounter(&comms, floor, encounter, &mut pps, None)? {
                            break;
                        }
                    }
                    (Room::Shop, None) => self.run_shop(&comms, floor, &mut pps)?,
                    (Room::Treasure, None) => self.run_treasure_room(&comms, floor, &mut pps)?,
                    invalid => unreachable!("{:?}", invalid),
                },
                Room::Monster => {
                    let encounter = self.encounter_generator.next_monster_encounter();
                    if !self.run_encounter(&comms, floor, encounter, &mut pps, None)? {
                        break;
                    }
                }
                Room::Shop => self.run_shop(&comms, floor, &mut pps)?,
                Room::Treasure => self.run_treasure_room(&comms, floor, &mut pps)?,
            }
            floor += 1;
        }
        comms.send_game_over(pps.hp > 0)
    }

    pub fn run_encounter(
        &mut self,
        comms: &PlayerInteraction,
        floor: Floor,
        encounter: Encounter,
        pps: &mut PlayerPersistentState,
        _burning_elite_buff: Option<u8>,
    ) -> Result<bool, Error> {
        if !CombatSimulator::new(self.seed.with_offset(floor), &mut self.misc_rng)
            .run_encounter(comms, encounter, pps)?
        {
            Ok(false)
        } else {
            let gold_reward = self.treasure_rng.gen_range(10..=20);
            // TODO: Relic::WhiteBeastStatue
            let maybe_potion = self.potion_generator.combat_reward();
            let card_rewards = self.card_generator.combat_rewards();
            MainScreenSystem::choose_combat_rewards(
                comms,
                pps,
                gold_reward,
                maybe_potion,
                &card_rewards,
            )?;
            Ok(true)
        }
    }

    fn run_shop(
        &mut self,
        _comms: &PlayerInteraction,
        _floor: Floor,
        _pps: &mut PlayerPersistentState,
    ) -> Result<(), Error> {
        todo!()
    }

    fn run_treasure_room(
        &mut self,
        _comms: &PlayerInteraction,
        _floor: Floor,
        _pps: &mut PlayerPersistentState,
    ) -> Result<(), Error> {
        todo!()
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

    use crate::components::{CardCombatState, Choice, EnemyStatus, Notification, Prompt};
    use crate::data::{
        Card, Enemy, EnemyCondition, EnergyCost, Intent, NeowBlessing, NeowBonus, NeowPenalty,
        PlayerCondition, IRONCLAD,
    };

    #[track_caller]
    pub fn next_prompt(
        from_server: &Receiver<StsMessage>,
        expected_notifications: &[Notification],
    ) -> StsMessage {
        let mut expected = HashMap::new();
        for notification in expected_notifications {
            *expected.entry(notification).or_insert_with(|| 0) += 1;
        }
        let mut maybe_message;
        let mut unrecognized_notifications = HashMap::new();
        loop {
            let message = from_server.recv_timeout(Duration::from_secs(5)).unwrap();
            match message {
                StsMessage::Choices(_, _) | StsMessage::GameOver(_) => {
                    maybe_message = Some(message);
                    break;
                }
                StsMessage::Notification(notification) => {
                    if expected
                        .get_mut(&notification)
                        .map(|count| *count -= 1)
                        .is_none()
                    {
                        *unrecognized_notifications
                            .entry(notification)
                            .or_insert_with(|| 0) += 1;
                    }
                    expected.retain(|_, count| *count > 0);
                }
            }
        }
        assert!(
            expected.is_empty(),
            "did not see: {:#?}; maybe mismatched one of these? {:#?}",
            expected,
            unrecognized_notifications
        );
        maybe_message.take().expect("expected a prompt")
    }

    // TODO: This test depends on `CombatSimulator` behavior. Should test combat separately.
    #[test]
    pub fn test_game_3_ironclad() {
        let seed = Seed::from(3);
        let character = &IRONCLAD;
        let (to_server, from_client) = channel();
        let (to_client, from_server) = channel();
        let simulator = StsSimulator::new(seed, character);
        let simulator_thread = thread::spawn(move || simulator.run(from_client, to_client));

        assert_eq!(
            next_prompt(&from_server, &[Notification::Health((80, 80))]),
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
                    Notification::StartingCombat,
                    Notification::Energy(3),
                    Notification::EnemyParty(vec![
                        Some(EnemyStatus::new(
                            Enemy::AcidSlimeS,
                            (12, 12),
                            Intent::StrategicDebuff
                        )),
                        Some(EnemyStatus::new(
                            Enemy::SpikeSlimeM,
                            (31, 31),
                            Intent::StrategicDebuff
                        )),
                        None,
                        None,
                        None
                    ]),
                ]
            ),
            StsMessage::Choices(
                Prompt::CombatAction,
                vec![
                    Choice::PlayCardFromHand(0, Card::Strike(false), EnergyCost::One),
                    Choice::PlayCardFromHand(1, Card::Defend(false), EnergyCost::One),
                    Choice::PlayCardFromHand(2, Card::Bash(false), EnergyCost::Two),
                    Choice::PlayCardFromHand(3, Card::Defend(false), EnergyCost::One),
                    Choice::PlayCardFromHand(4, Card::Strike(false), EnergyCost::One),
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
                    Choice::TargetEnemy(0, Enemy::AcidSlimeS),
                    Choice::TargetEnemy(1, Enemy::SpikeSlimeM),
                ]
            )
        );
        to_server.send(0).unwrap(); // Target AcidSlimeS
        assert_eq!(
            next_prompt(
                &from_server,
                &[
                    Notification::Energy(2),
                    Notification::EnemyParty(vec![
                        Some(EnemyStatus::new(
                            Enemy::AcidSlimeS,
                            (6, 12),
                            Intent::StrategicDebuff
                        )),
                        Some(EnemyStatus::new(
                            Enemy::SpikeSlimeM,
                            (31, 31),
                            Intent::StrategicDebuff
                        )),
                        None,
                        None,
                        None
                    ]),
                ]
            ),
            StsMessage::Choices(
                Prompt::CombatAction,
                vec![
                    Choice::PlayCardFromHand(0, Card::Defend(false), EnergyCost::One),
                    Choice::PlayCardFromHand(1, Card::Bash(false), EnergyCost::Two),
                    Choice::PlayCardFromHand(2, Card::Defend(false), EnergyCost::One),
                    Choice::PlayCardFromHand(3, Card::Strike(false), EnergyCost::One),
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
                    Choice::TargetEnemy(0, Enemy::AcidSlimeS),
                    Choice::TargetEnemy(1, Enemy::SpikeSlimeM),
                ]
            )
        );
        to_server.send(0).unwrap(); // Target AcidSlimeS
        assert_eq!(
            next_prompt(
                &from_server,
                &[
                    Notification::Energy(1),
                    Notification::EnemyDied(0, Enemy::AcidSlimeS) // AcidSlimeS dies
                ]
            ),
            StsMessage::Choices(
                Prompt::CombatAction,
                vec![
                    Choice::PlayCardFromHand(0, Card::Defend(false), EnergyCost::One),
                    Choice::PlayCardFromHand(2, Card::Defend(false), EnergyCost::One),
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
                    Notification::Conditions(vec![PlayerCondition::Frail(1)]),
                    Notification::EnemyParty(vec![
                        None, // AcidSlimeS now gone
                        Some(EnemyStatus::new(
                            Enemy::SpikeSlimeM,
                            (31, 31),
                            Intent::AggressiveDebuff(8, 1)
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
                    Choice::PlayCardFromHand(0, Card::Strike(false), EnergyCost::One),
                    Choice::PlayCardFromHand(1, Card::Strike(false), EnergyCost::One),
                    Choice::PlayCardFromHand(2, Card::Strike(false), EnergyCost::One),
                    Choice::PlayCardFromHand(3, Card::Defend(false), EnergyCost::One),
                    Choice::PlayCardFromHand(4, Card::Defend(false), EnergyCost::One),
                    Choice::EndTurn,
                ]
            )
        );
        to_server.send(3).unwrap(); // Play "Defend"
        assert_eq!(
            next_prompt(&from_server, &[Notification::BlockGained(3)]), // 5 -> 3 because Frail
            StsMessage::Choices(
                Prompt::CombatAction,
                vec![
                    Choice::PlayCardFromHand(0, Card::Strike(false), EnergyCost::One),
                    Choice::PlayCardFromHand(1, Card::Strike(false), EnergyCost::One),
                    Choice::PlayCardFromHand(2, Card::Strike(false), EnergyCost::One),
                    Choice::PlayCardFromHand(3, Card::Defend(false), EnergyCost::One),
                    Choice::EndTurn,
                ]
            )
        );
        to_server.send(3).unwrap(); // Play "Defend"
        assert_eq!(
            next_prompt(&from_server, &[Notification::BlockGained(3)]),
            StsMessage::Choices(
                Prompt::CombatAction,
                vec![
                    Choice::PlayCardFromHand(0, Card::Strike(false), EnergyCost::One),
                    Choice::PlayCardFromHand(1, Card::Strike(false), EnergyCost::One),
                    Choice::PlayCardFromHand(2, Card::Strike(false), EnergyCost::One),
                    Choice::EndTurn,
                ]
            )
        );
        to_server.send(3).unwrap(); // End Turn
        assert_eq!(
            next_prompt(
                &from_server,
                &[
                    Notification::Block(0),         // No block remaining
                    Notification::Health((78, 80)), // Took 2 damage
                    Notification::EnemyParty(vec![
                        None,
                        Some(EnemyStatus::new(
                            Enemy::SpikeSlimeM,
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
                    Choice::PlayCardFromHand(0, Card::Defend(false), EnergyCost::One),
                    Choice::PlayCardFromHand(1, Card::Slimed, EnergyCost::One),
                    Choice::PlayCardFromHand(2, Card::Strike(false), EnergyCost::One),
                    Choice::PlayCardFromHand(3, Card::Bash(false), EnergyCost::Two),
                    Choice::PlayCardFromHand(4, Card::Strike(false), EnergyCost::One),
                    Choice::EndTurn,
                ]
            )
        );
        to_server.send(3).unwrap(); // Play "Bash"
        assert_eq!(
            next_prompt(&from_server, &[]),
            StsMessage::Choices(
                Prompt::TargetEnemy,
                vec![Choice::TargetEnemy(1, Enemy::SpikeSlimeM)]
            )
        );
        to_server.send(0).unwrap(); // Target SpikeSlimeM
        assert_eq!(
            next_prompt(
                &from_server,
                &[
                    Notification::Energy(1),
                    Notification::EnemyParty(vec![
                        None,
                        Some(
                            EnemyStatus::new(Enemy::SpikeSlimeM, (23, 31), Intent::StrategicDebuff)
                                .with_condition(EnemyCondition::Vulnerable(2))
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
                    Choice::PlayCardFromHand(0, Card::Defend(false), EnergyCost::One),
                    Choice::PlayCardFromHand(1, Card::Slimed, EnergyCost::One),
                    Choice::PlayCardFromHand(2, Card::Strike(false), EnergyCost::One),
                    Choice::PlayCardFromHand(3, Card::Strike(false), EnergyCost::One),
                    Choice::EndTurn,
                ]
            )
        );
        to_server.send(1).unwrap(); // Play "Slimed"
        assert_eq!(
            next_prompt(
                &from_server,
                &[Notification::CardExhausted(
                    1,
                    CardCombatState::new(Card::Slimed, None)
                )]
            ),
            StsMessage::Choices(Prompt::CombatAction, vec![Choice::EndTurn])
        );
        to_server.send(0).unwrap(); // End Turn
        assert_eq!(
            next_prompt(
                &from_server,
                &[
                    // 1 stack of Frail on player
                    Notification::Conditions(vec![PlayerCondition::Frail(1)]),
                    Notification::EnemyParty(vec![
                        None,
                        Some(
                            EnemyStatus::new(Enemy::SpikeSlimeM, (23, 31), Intent::StrategicDebuff)
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
                    Choice::PlayCardFromHand(0, Card::Defend(false), EnergyCost::One),
                    Choice::PlayCardFromHand(1, Card::Defend(false), EnergyCost::One),
                    Choice::PlayCardFromHand(2, Card::Strike(false), EnergyCost::One),
                    Choice::PlayCardFromHand(3, Card::Strike(false), EnergyCost::One),
                    Choice::PlayCardFromHand(4, Card::Strike(false), EnergyCost::One),
                    Choice::EndTurn,
                ]
            )
        );
        to_server.send(2).unwrap(); // Strike
        assert_eq!(
            next_prompt(&from_server, &[]),
            StsMessage::Choices(
                Prompt::TargetEnemy,
                vec![Choice::TargetEnemy(1, Enemy::SpikeSlimeM)]
            )
        );
        to_server.send(0).unwrap(); // Target SpikeSlimeM
        assert_eq!(
            next_prompt(
                &from_server,
                &[
                    Notification::Energy(2),
                    Notification::EnemyParty(vec![
                        None,
                        Some(
                            EnemyStatus::new(Enemy::SpikeSlimeM, (14, 31), Intent::StrategicDebuff)
                                .with_condition(EnemyCondition::Vulnerable(1))
                        ),
                        None,
                        None,
                        None
                    ]),
                ]
            ),
            StsMessage::Choices(
                Prompt::CombatAction,
                vec![
                    Choice::PlayCardFromHand(0, Card::Defend(false), EnergyCost::One),
                    Choice::PlayCardFromHand(1, Card::Defend(false), EnergyCost::One),
                    Choice::PlayCardFromHand(2, Card::Strike(false), EnergyCost::One),
                    Choice::PlayCardFromHand(3, Card::Strike(false), EnergyCost::One),
                    Choice::EndTurn,
                ]
            )
        );
        to_server.send(2).unwrap(); // Strike
        assert_eq!(
            next_prompt(&from_server, &[]),
            StsMessage::Choices(
                Prompt::TargetEnemy,
                vec![Choice::TargetEnemy(1, Enemy::SpikeSlimeM)]
            )
        );
        to_server.send(0).unwrap(); // Target SpikeSlimeM
        assert_eq!(
            next_prompt(
                &from_server,
                &[
                    Notification::Energy(1),
                    Notification::EnemyParty(vec![
                        None,
                        Some(
                            EnemyStatus::new(Enemy::SpikeSlimeM, (5, 31), Intent::StrategicDebuff)
                                .with_condition(EnemyCondition::Vulnerable(1))
                        ),
                        None,
                        None,
                        None
                    ]),
                ]
            ),
            StsMessage::Choices(
                Prompt::CombatAction,
                vec![
                    Choice::PlayCardFromHand(0, Card::Defend(false), EnergyCost::One),
                    Choice::PlayCardFromHand(1, Card::Defend(false), EnergyCost::One),
                    Choice::PlayCardFromHand(2, Card::Strike(false), EnergyCost::One),
                    Choice::EndTurn,
                ]
            )
        );
        to_server.send(2).unwrap(); // Strike
        assert_eq!(
            next_prompt(&from_server, &[]),
            StsMessage::Choices(
                Prompt::TargetEnemy,
                vec![Choice::TargetEnemy(1, Enemy::SpikeSlimeM)]
            )
        );
        to_server.send(0).unwrap(); // Target SpikeSlimeM
        assert_eq!(
            next_prompt(
                &from_server,
                &[
                    Notification::EnemyDied(1, Enemy::SpikeSlimeM),
                    Notification::Health((80, 80)), // Burning Blood heals 6
                    Notification::EndingCombat
                ]
            ),
            StsMessage::Choices(
                Prompt::ChooseNext,
                vec![
                    Choice::ObtainGold(11),
                    Choice::ObtainCard(0, Card::Thunderclap(false)),
                    Choice::ObtainCard(1, Card::HeavyBlade(false)),
                    Choice::ObtainCard(2, Card::Armaments(false)),
                    Choice::Skip,
                ]
            )
        );
        to_server.send(0).unwrap(); // Take Gold
        assert_eq!(
            next_prompt(&from_server, &[Notification::Gold(99 + 100 + 11)]),
            StsMessage::Choices(
                Prompt::ChooseNext,
                vec![
                    Choice::ObtainCard(0, Card::Thunderclap(false)),
                    Choice::ObtainCard(1, Card::HeavyBlade(false)),
                    Choice::ObtainCard(2, Card::Armaments(false)),
                    Choice::Skip,
                ]
            )
        );
        to_server.send(0).unwrap(); // Take Thunderclap
        assert_eq!(
            next_prompt(
                &from_server,
                &[Notification::CardObtained(Card::Thunderclap(false))]
            ),
            StsMessage::Choices(Prompt::ClimbFloor, vec![Choice::ClimbFloor(0)])
        );
        to_server.send(0).unwrap(); // Column 0
        assert_eq!(
            next_prompt(
                &from_server,
                &[
                    Notification::StartingCombat,
                    Notification::Energy(3),
                    Notification::EnemyParty(vec![
                        Some(EnemyStatus::new(
                            Enemy::Cultist,
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
                    Choice::PlayCardFromHand(0, Card::Strike(false), EnergyCost::One),
                    Choice::PlayCardFromHand(1, Card::Strike(false), EnergyCost::One),
                    Choice::PlayCardFromHand(2, Card::Strike(false), EnergyCost::One),
                    Choice::PlayCardFromHand(3, Card::Defend(false), EnergyCost::One),
                    Choice::PlayCardFromHand(4, Card::Defend(false), EnergyCost::One),
                    Choice::EndTurn,
                ]
            )
        );
        to_server.send(0).unwrap(); // Play "Strike"
        assert_eq!(
            next_prompt(&from_server, &[]),
            StsMessage::Choices(
                Prompt::TargetEnemy,
                vec![Choice::TargetEnemy(0, Enemy::Cultist)]
            )
        );
        to_server.send(0).unwrap(); // Target Cultist
        assert_eq!(
            next_prompt(
                &from_server,
                &[
                    Notification::Energy(2),
                    Notification::EnemyParty(vec![
                        Some(EnemyStatus::new(
                            Enemy::Cultist,
                            (44, 50),
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
                    Choice::PlayCardFromHand(0, Card::Strike(false), EnergyCost::One),
                    Choice::PlayCardFromHand(1, Card::Strike(false), EnergyCost::One),
                    Choice::PlayCardFromHand(2, Card::Defend(false), EnergyCost::One),
                    Choice::PlayCardFromHand(3, Card::Defend(false), EnergyCost::One),
                    Choice::EndTurn,
                ]
            )
        );
        to_server.send(0).unwrap(); // Play "Strike"
        assert_eq!(
            next_prompt(&from_server, &[]),
            StsMessage::Choices(
                Prompt::TargetEnemy,
                vec![Choice::TargetEnemy(0, Enemy::Cultist)]
            )
        );
        to_server.send(0).unwrap(); // Target Cultist
        assert_eq!(
            next_prompt(
                &from_server,
                &[
                    Notification::Energy(1),
                    Notification::EnemyParty(vec![
                        Some(EnemyStatus::new(
                            Enemy::Cultist,
                            (38, 50),
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
                    Choice::PlayCardFromHand(0, Card::Strike(false), EnergyCost::One),
                    Choice::PlayCardFromHand(1, Card::Defend(false), EnergyCost::One),
                    Choice::PlayCardFromHand(2, Card::Defend(false), EnergyCost::One),
                    Choice::EndTurn,
                ]
            )
        );
        to_server.send(0).unwrap(); // Play "Strike"
        assert_eq!(
            next_prompt(&from_server, &[]),
            StsMessage::Choices(
                Prompt::TargetEnemy,
                vec![Choice::TargetEnemy(0, Enemy::Cultist)]
            )
        );
        to_server.send(0).unwrap(); // Target Cultist
        assert_eq!(
            next_prompt(
                &from_server,
                &[
                    Notification::Energy(0),
                    Notification::EnemyParty(vec![
                        Some(EnemyStatus::new(
                            Enemy::Cultist,
                            (32, 50),
                            Intent::StrategicBuff
                        )),
                        None,
                        None,
                        None,
                        None
                    ])
                ]
            ),
            StsMessage::Choices(Prompt::CombatAction, vec![Choice::EndTurn])
        );
        to_server.send(0).unwrap(); // End Turn
        assert_eq!(
            next_prompt(
                &from_server,
                &[
                    Notification::Energy(3),
                    Notification::EnemyParty(vec![
                        Some(
                            EnemyStatus::new(Enemy::Cultist, (32, 50), Intent::Aggressive(6, 1))
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
                    Choice::PlayCardFromHand(0, Card::Defend(false), EnergyCost::One),
                    Choice::PlayCardFromHand(1, Card::Strike(false), EnergyCost::One),
                    Choice::PlayCardFromHand(2, Card::Bash(false), EnergyCost::Two),
                    Choice::PlayCardFromHand(3, Card::Defend(false), EnergyCost::One),
                    Choice::PlayCardFromHand(4, Card::Strike(false), EnergyCost::One),
                    Choice::EndTurn,
                ]
            )
        );
        to_server.send(2).unwrap(); // Play "Bash"
        assert_eq!(
            next_prompt(&from_server, &[]),
            StsMessage::Choices(
                Prompt::TargetEnemy,
                vec![Choice::TargetEnemy(0, Enemy::Cultist)]
            )
        );
        to_server.send(0).unwrap(); // Target Cultist
        assert_eq!(
            next_prompt(
                &from_server,
                &[
                    Notification::Energy(1),
                    Notification::EnemyParty(vec![
                        Some(
                            EnemyStatus::new(Enemy::Cultist, (24, 50), Intent::Aggressive(6, 1))
                                .with_condition(EnemyCondition::Ritual(3, false))
                                .with_condition(EnemyCondition::Vulnerable(2))
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
                    Choice::PlayCardFromHand(0, Card::Defend(false), EnergyCost::One),
                    Choice::PlayCardFromHand(1, Card::Strike(false), EnergyCost::One),
                    Choice::PlayCardFromHand(2, Card::Defend(false), EnergyCost::One),
                    Choice::PlayCardFromHand(3, Card::Strike(false), EnergyCost::One),
                    Choice::EndTurn,
                ]
            )
        );
        to_server.send(1).unwrap(); // Play "Strike"
        assert_eq!(
            next_prompt(&from_server, &[]),
            StsMessage::Choices(
                Prompt::TargetEnemy,
                vec![Choice::TargetEnemy(0, Enemy::Cultist)]
            )
        );
        to_server.send(0).unwrap(); // Target Cultist
        assert_eq!(
            next_prompt(
                &from_server,
                &[
                    Notification::Energy(0),
                    Notification::EnemyParty(vec![
                        Some(
                            EnemyStatus::new(Enemy::Cultist, (15, 50), Intent::Aggressive(6, 1))
                                .with_condition(EnemyCondition::Ritual(3, false))
                                .with_condition(EnemyCondition::Vulnerable(2))
                        ),
                        None,
                        None,
                        None,
                        None
                    ])
                ]
            ),
            StsMessage::Choices(Prompt::CombatAction, vec![Choice::EndTurn])
        );
        to_server.send(0).unwrap(); // End Turn
        assert_eq!(
            next_prompt(
                &from_server,
                &[
                    Notification::Energy(3),
                    Notification::EnemyParty(vec![
                        Some(
                            EnemyStatus::new(Enemy::Cultist, (15, 50), Intent::Aggressive(6, 1))
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
                    Choice::PlayCardFromHand(0, Card::Thunderclap(false), EnergyCost::One),
                    Choice::PlayCardFromHand(1, Card::Defend(false), EnergyCost::One),
                    Choice::PlayCardFromHand(2, Card::Defend(false), EnergyCost::One),
                    Choice::PlayCardFromHand(3, Card::Strike(false), EnergyCost::One),
                    Choice::PlayCardFromHand(4, Card::Defend(false), EnergyCost::One),
                    Choice::EndTurn,
                ]
            )
        );
        to_server.send(0).unwrap(); // Play "Thunderclap"
        assert_eq!(
            next_prompt(
                &from_server,
                &[
                    Notification::Energy(2),
                    Notification::EnemyParty(vec![
                        Some(
                            EnemyStatus::new(Enemy::Cultist, (9, 50), Intent::Aggressive(6, 1))
                                .with_condition(EnemyCondition::Ritual(3, false))
                                .with_condition(EnemyCondition::Vulnerable(2))
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
                    Choice::PlayCardFromHand(0, Card::Defend(false), EnergyCost::One),
                    Choice::PlayCardFromHand(1, Card::Defend(false), EnergyCost::One),
                    Choice::PlayCardFromHand(2, Card::Strike(false), EnergyCost::One),
                    Choice::PlayCardFromHand(3, Card::Defend(false), EnergyCost::One),
                    Choice::EndTurn,
                ]
            )
        );
        to_server.send(0).unwrap(); // Play "Defend"
        assert_eq!(
            next_prompt(
                &from_server,
                &[
                    Notification::Energy(1),
                    Notification::BlockGained(5),
                    Notification::Block(5)
                ]
            ),
            StsMessage::Choices(
                Prompt::CombatAction,
                vec![
                    Choice::PlayCardFromHand(0, Card::Defend(false), EnergyCost::One),
                    Choice::PlayCardFromHand(1, Card::Strike(false), EnergyCost::One),
                    Choice::PlayCardFromHand(2, Card::Defend(false), EnergyCost::One),
                    Choice::EndTurn,
                ]
            )
        );
        to_server.send(0).unwrap(); // Play "Defend"
        assert_eq!(
            next_prompt(
                &from_server,
                &[
                    Notification::Energy(0),
                    Notification::BlockGained(5),
                    Notification::Block(10)
                ]
            ),
            StsMessage::Choices(Prompt::CombatAction, vec![Choice::EndTurn])
        );
        to_server.send(0).unwrap(); // End Turn
        assert_eq!(
            next_prompt(
                &from_server,
                &[
                    Notification::Energy(3),
                    Notification::Block(1),
                    Notification::Block(0),
                    Notification::EnemyParty(vec![
                        Some(
                            EnemyStatus::new(Enemy::Cultist, (9, 50), Intent::Aggressive(6, 1))
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
                    Choice::PlayCardFromHand(0, Card::Strike(false), EnergyCost::One),
                    Choice::PlayCardFromHand(1, Card::Strike(false), EnergyCost::One),
                    Choice::PlayCardFromHand(2, Card::Strike(false), EnergyCost::One),
                    Choice::PlayCardFromHand(3, Card::Defend(false), EnergyCost::One),
                    Choice::PlayCardFromHand(4, Card::Bash(false), EnergyCost::Two),
                    Choice::EndTurn,
                ]
            )
        );
        to_server.send(0).unwrap(); // Play "Strike"
        assert_eq!(
            next_prompt(&from_server, &[]),
            StsMessage::Choices(
                Prompt::TargetEnemy,
                vec![Choice::TargetEnemy(0, Enemy::Cultist)]
            )
        );
        to_server.send(0).unwrap(); // Target Cultist
        assert_eq!(
            next_prompt(
                &from_server,
                &[
                    Notification::EnemyDied(0, Enemy::Cultist),
                    Notification::Health((80, 80)), // Burning Blood heals 6
                    Notification::EndingCombat
                ]
            ),
            StsMessage::Choices(
                Prompt::ChooseNext,
                vec![
                    Choice::ObtainGold(10),
                    Choice::ObtainCard(0, Card::Anger(false)),
                    Choice::ObtainCard(1, Card::HeavyBlade(false)),
                    Choice::ObtainCard(2, Card::Armaments(false)),
                    Choice::Skip,
                ]
            )
        );
        to_server.send(0).unwrap(); // Take Gold
        assert_eq!(
            next_prompt(&from_server, &[Notification::Gold(99 + 100 + 11 + 10)]),
            StsMessage::Choices(
                Prompt::ChooseNext,
                vec![
                    Choice::ObtainCard(0, Card::Anger(false)),
                    Choice::ObtainCard(1, Card::HeavyBlade(false)),
                    Choice::ObtainCard(2, Card::Armaments(false)),
                    Choice::Skip,
                ]
            )
        );
        to_server.send(2).unwrap(); // Take Armaments
        assert_eq!(
            next_prompt(
                &from_server,
                &[Notification::CardObtained(Card::Armaments(false))]
            ),
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

    #[test]
    fn test_prerecorded_game() {
        let seed = Seed::from(2);
        let character = &IRONCLAD;
        let (to_server, from_client) = channel();
        let (to_client, from_server) = channel();
        let simulator = StsSimulator::new(seed, character);
        let simulator_thread = thread::spawn(move || simulator.run(from_client, to_client));

        let choices = [1, 0, 0, 3, 3, 1, 0, 1, 0, 7, 0, 0, 0, 0, 0, 0, 4, 6];
        for choice in choices.iter() {
            loop {
                let message = from_server.recv_timeout(Duration::from_secs(5)).unwrap();
                println!("{:?}", message);
                match message {
                    StsMessage::Choices(_, _) | StsMessage::GameOver(_) => {
                        break;
                    }
                    StsMessage::Notification(_) => {}
                }
            }
            to_server.send(*choice).unwrap();
        }
        drop(to_server);
        let result = simulator_thread.join();
        assert!(result.is_ok(), "Thread panicked: {:?}", result.unwrap_err());
    }

    #[test]
    fn test_random_walk_for_ironclad() {
        let seed = Seed::from(1);
        let mut my_rng = StsRandom::from(seed);
        let character = &IRONCLAD;
        let (to_server, from_client) = channel();
        let (to_client, from_server) = channel();
        let simulator = StsSimulator::new(seed, character);
        let simulator_thread = thread::spawn(move || simulator.run(from_client, to_client));
        let mut choice_seq = vec![];
        let mut steps = 0;
        while steps < 1000 {
            match from_server.recv_timeout(Duration::from_secs(5)) {
                Ok(message) => {
                    println!("{:?}", message);
                    match message {
                        StsMessage::Choices(_, choices) => {
                            let num_choices = choices.len();
                            let choice = my_rng.gen_range(0..num_choices);
                            choice_seq.push(choices[choice].clone());
                            to_server.send(choice).unwrap()
                        }
                        StsMessage::Notification(_) => {}
                        StsMessage::GameOver(_) => break,
                    }
                }
                Err(_) => panic!(
                    concat!(
                        "Timed out waiting for message, or channel closed. ",
                        "Choice history: {:#?}, total steps: {}",
                    ),
                    choice_seq, steps
                ),
            }
            steps += 1;
        }
        drop(to_server);
        let result = simulator_thread.join();
        assert!(result.is_ok(), "Thread panicked: {:?}", result.unwrap_err());
    }
}
