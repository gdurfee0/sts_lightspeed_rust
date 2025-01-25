use crate::components::{PlayerState, Room};
use crate::data::{Act, CardDetails, CardType, Event, ONE_TIME_EVENTS};
use crate::types::Floor;

use super::seed::Seed;
use super::sts_random::StsRandom;

const SHRINE_PROBABILITY: f32 = 0.25;

pub struct EventGenerator {
    act: &'static Act,
    monster_room_probability: f32,
    shop_probability: f32,
    treasure_room_probability: f32,
    event_rng: StsRandom,
    event_pool: Vec<Event>,
    shrine_pool: Vec<Event>,
    one_time_event_pool: Vec<Event>,
}

impl EventGenerator {
    pub fn new(seed: Seed) -> Self {
        let event_rng = StsRandom::from(seed);
        let act = Act::get(1);
        Self {
            act,
            event_rng,
            monster_room_probability: 0.1,
            shop_probability: 0.03,
            treasure_room_probability: 0.02,
            event_pool: act.event_pool.to_vec(),
            shrine_pool: act.shrine_pool.to_vec(),
            one_time_event_pool: ONE_TIME_EVENTS.to_vec(),
        }
    }

    pub fn advance_act(&mut self) {
        self.act = self.act.next_act();
        self.event_pool = self.act.event_pool.to_vec();
        self.shrine_pool = self.act.shrine_pool.to_vec();
        self.monster_room_probability = 0.1;
        self.shop_probability = 0.03;
        self.treasure_room_probability = 0.02;
    }

    pub fn next_event(&mut self, floor: Floor, state: &PlayerState) -> (Room, Option<Event>) {
        // TODO: Last room was a shop
        // TODO: Relic::TinyChest
        // TODO: Relic::JuzuBracelet

        let room = *self.event_rng.weighted_choose(&[
            (
                Room::Monster,
                floor_to_hundredths(self.monster_room_probability),
            ),
            (Room::Shop, floor_to_hundredths(self.shop_probability)),
            (
                Room::Treasure,
                floor_to_hundredths(self.treasure_room_probability),
            ),
            (Room::Event, 1.),
        ]);
        // Game bug or intended behavior? Kudos to gamerpuppy for spotting this
        let mut event_rng_clone = self.event_rng.clone();
        match room {
            Room::Monster => {
                self.monster_room_probability = 0.1;
                self.shop_probability += 0.03;
                self.treasure_room_probability += 0.02;
                (room, None)
            }
            Room::Shop => {
                self.monster_room_probability += 0.1;
                self.shop_probability = 0.03;
                self.treasure_room_probability += 0.02;
                (room, None)
            }
            Room::Treasure => {
                self.monster_room_probability += 0.1;
                self.shop_probability += 0.03;
                self.treasure_room_probability = 0.02;
                (room, None)
            }
            Room::Event => {
                self.monster_room_probability += 0.1;
                self.shop_probability += 0.03;
                self.treasure_room_probability += 0.02;
                let event_pool = self
                    .event_pool
                    .iter()
                    .copied()
                    .filter(|&event| self.filter_event(event, floor, state))
                    .collect::<Vec<_>>();
                let shrine_and_special_pool = self
                    .shrine_pool
                    .iter()
                    .copied()
                    .chain(self.one_time_event_pool.iter().copied())
                    .filter(|&event| self.filter_event(event, floor, state))
                    .collect::<Vec<_>>();
                let pool = *event_rng_clone.weighted_choose(&[
                    (shrine_and_special_pool.as_slice(), SHRINE_PROBABILITY),
                    (event_pool.as_slice(), 1. - SHRINE_PROBABILITY),
                ]);
                let choice = *event_rng_clone.choose(pool);
                self.event_pool = self
                    .event_pool
                    .iter()
                    .copied()
                    .filter(|&event| event != choice)
                    .collect();
                self.shrine_pool = self
                    .shrine_pool
                    .iter()
                    .copied()
                    .filter(|&event| event != choice)
                    .collect();
                self.one_time_event_pool = self
                    .one_time_event_pool
                    .iter()
                    .copied()
                    .filter(|&event| event != choice)
                    .collect();
                (Room::Event, Some(choice))
            }
            _ => unreachable!(),
        }
    }

    fn filter_event(&self, event: Event, floor: Floor, state: &PlayerState) -> bool {
        match event {
            Event::DeadAdventurer => floor >= 7,
            Event::DesignerInSpire => self.act.number > 1 && state.hp > 3,
            Event::Duplicator => self.act.number > 1 && state.deck.len() >= 5,
            Event::FaceTrader => self.act.number < 3,
            Event::HypnotizingColoredMushrooms => floor >= 7,
            Event::KnowingSkull => self.act.number == 2 && state.hp >= 13,
            Event::Nloth => self.act.number == 2 && state.relics.len() >= 2,
            Event::OldBeggar => state.gold >= 75,
            Event::SecretPortal => false, // 13m 20s
            Event::TheCleric => state.gold >= 35,
            Event::TheColosseum => floor - ((self.act.number as u64 - 1) * 17) >= 7,
            Event::TheDivineFountain => state
                .deck
                .iter()
                .any(|card| matches!(CardDetails::for_card(*card).type_, CardType::Curse)),
            Event::TheJoust => self.act.number == 2 && state.gold >= 50,
            Event::TheMoaiHead => state.hp <= state.hp_max / 2,
            Event::TheWomanInBlue => state.gold >= 20,
            _ => true,
        }
    }
}

fn floor_to_hundredths(x: f32) -> f32 {
    (x * 100.).floor() / 100.
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use crate::data::IRONCLAD;

    use super::*;

    #[test]
    fn test_event_generator() {
        let seed = Seed::from(3);
        let mut event_generator = EventGenerator::new(seed);
        let state = PlayerState::new(IRONCLAD);
        let (room, event) = event_generator.next_event(3, &state);
        assert_eq!(room, Room::Event);
        assert_eq!(event, Some(Event::UpgradeShrine));
        let (room, _) = event_generator.next_event(4, &state);
        assert_eq!(room, Room::Shop);
        let (room, _) = event_generator.next_event(7, &state);
        assert_eq!(room, Room::Monster);
        let (room, event) = event_generator.next_event(8, &state);
        assert_eq!(room, Room::Event);
        assert_eq!(event, Some(Event::DeadAdventurer));
    }

    #[test]
    fn test_event_generator_test_vector() {
        let seed = Seed::from(3);
        let mut event_generator = EventGenerator::new(seed);
        let state = PlayerState::new(IRONCLAD);
        let mut test_vector = vec![];
        for i in 3..15 {
            let (room, event) = event_generator.next_event(i, &state);
            test_vector.push((room, event));
        }
        event_generator.advance_act();
        for i in 18..30 {
            let (room, event) = event_generator.next_event(i, &state);
            test_vector.push((room, event));
        }
        event_generator.advance_act();
        for i in 33..45 {
            let (room, event) = event_generator.next_event(i, &state);
            test_vector.push((room, event));
        }
        assert_eq!(
            test_vector,
            vec![
                // Act 1
                (Room::Event, Some(Event::UpgradeShrine)),
                (Room::Shop, None),
                (Room::Monster, None),
                (Room::Event, Some(Event::GoldenIdol)),
                (Room::Event, Some(Event::FaceTrader)),
                (Room::Monster, None),
                (Room::Shop, None),
                (Room::Event, Some(Event::HypnotizingColoredMushrooms)),
                (Room::Treasure, None),
                (Room::Monster, None),
                (Room::Event, Some(Event::WorldOfGoop)),
                (Room::Event, Some(Event::WingStatue)),
                // Act 2
                (Room::Event, Some(Event::TheJoust)),
                (Room::Monster, None),
                (Room::Shop, None),
                (Room::Event, Some(Event::Transmogrifier)),
                (Room::Monster, None),
                (Room::Event, Some(Event::OminousForge)),
                (Room::Monster, None),
                (Room::Shop, None),
                (Room::Monster, None),
                (Room::Event, Some(Event::UpgradeShrine)),
                (Room::Shop, None),
                (Room::Event, Some(Event::Augmenter)),
                // Act 3
                (Room::Event, Some(Event::TombOfLordRedMask)),
                (Room::Event, Some(Event::SensoryStone)),
                (Room::Event, Some(Event::Transmogrifier)),
                (Room::Monster, None),
                (Room::Shop, None),
                (Room::Event, Some(Event::MysteriousSphere)),
                (Room::Monster, None),
                (Room::Event, Some(Event::Falling)),
                (Room::Event, Some(Event::WindingHalls)),
                (Room::Event, Some(Event::Duplicator)),
                (Room::Monster, None),
                (Room::Shop, None),
            ]
        );
    }
}
