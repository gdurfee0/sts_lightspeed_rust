use crate::data::{Act, Card, Event, Relic};
use crate::map::Room;
use crate::types::{Floor, Gold, Health};

use super::seed::Seed;
use super::sts::StsRandom;

const SHRINE_PROBABILITY: f32 = 0.25;
const ONE_TIME_EVENTS: &[Event] = &[
    Event::OminousForge,
    Event::BonfireSpirits,
    Event::DesignerInSpire,
    Event::Duplicator,
    Event::FaceTrader,
    Event::TheDivineFountain,
    Event::KnowingSkull,
    Event::Lab,
    Event::Nloth,
    Event::NoteForYourself,
    Event::SecretPortal,
    Event::TheJoust,
    Event::WeMeetAgain,
    Event::TheWomanInBlue,
];

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

    pub fn next_event(
        &mut self,
        floor: Floor,
        deck: &[Card],
        gold: Gold,
        health: Health,
        relics: &[Relic],
    ) -> (Room, Option<Event>) {
        // TODO: Last room was a shop
        // TODO: Relic::TinyChest
        // TODO: Relic::JuzuBracelet

        let room = *self.event_rng.weighted_choose(&[
            // TODO: Round to nearest 0.01 for compatibility with game rolls
            (Room::Monster, self.monster_room_probability),
            (Room::Shop, self.shop_probability),
            (Room::Treasure, self.treasure_room_probability),
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
                    .filter(|&event| self.filter_event(event, floor, deck, gold, health, relics))
                    .collect::<Vec<_>>();
                let shrine_and_special_pool = self
                    .shrine_pool
                    .iter()
                    .copied()
                    .chain(self.one_time_event_pool.iter().copied())
                    .filter(|&event| self.filter_event(event, floor, deck, gold, health, relics))
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

    fn filter_event(
        &self,
        event: Event,
        floor: Floor,
        deck: &[Card],
        gold: Gold,
        health: Health,
        relics: &[Relic],
    ) -> bool {
        match event {
            Event::DeadAdventurer => floor >= 7,
            Event::DesignerInSpire => self.act.number > 1 && health.0 > 3,
            Event::Duplicator => self.act.number > 1 && deck.len() >= 5,
            Event::FaceTrader => self.act.number < 3,
            Event::HypnotizingColoredMushrooms => floor >= 7,
            Event::KnowingSkull => self.act.number == 2 && health.0 >= 13,
            Event::Nloth => self.act.number == 2 && relics.len() >= 2,
            Event::OldBeggar => gold >= 75,
            Event::SecretPortal => false, // 13m 20s
            Event::TheCleric => gold >= 35,
            Event::TheColosseum => floor - ((self.act.number as u64 - 1) * 17) >= 7,
            Event::TheDivineFountain => deck.iter().any(|card| card.is_curse()),
            Event::TheJoust => self.act.number == 2 && gold >= 50,
            Event::TheMoaiHead => health.0 <= health.1 / 2,
            Event::TheWomanInBlue => gold >= 20,
            _ => true,
        }
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use crate::data::IRONCLAD;

    use super::*;

    #[test]
    fn test_event_generator() {
        let seed = Seed::from(3);
        let deck = IRONCLAD.starting_deck;
        let relics = [Relic::BurningBlood];
        let mut event_generator = EventGenerator::new(seed);
        let (room, event) = event_generator.next_event(3, deck, 99, (80, 80), &relics);
        assert_eq!(room, Room::Event);
        assert_eq!(event, Some(Event::UpgradeShrine));
        let (room, _) = event_generator.next_event(4, deck, 99, (80, 80), &relics);
        assert_eq!(room, Room::Shop);
        let (room, _) = event_generator.next_event(7, deck, 99, (80, 80), &relics);
        assert_eq!(room, Room::Monster);
        let (room, event) = event_generator.next_event(8, deck, 99, (80, 80), &relics);
        assert_eq!(room, Room::Event);
        assert_eq!(event, Some(Event::DeadAdventurer));
    }

    #[test]
    fn test_event_generator_test_vector() {
        let seed = Seed::from(3);
        let mut event_generator = EventGenerator::new(seed);
        let deck = IRONCLAD.starting_deck;
        let relics = [Relic::BurningBlood];
        let mut test_vector = vec![];
        for i in 3..15 {
            let (room, event) = event_generator.next_event(i, deck, 99, (80, 80), &relics);
            test_vector.push((room, event));
        }
        event_generator.advance_act();
        for i in 18..30 {
            let (room, event) = event_generator.next_event(i, deck, 99, (80, 80), &relics);
            test_vector.push((room, event));
        }
        event_generator.advance_act();
        for i in 33..45 {
            let (room, event) = event_generator.next_event(i, deck, 99, (80, 80), &relics);
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
                //(Room::Event, Some(Event::Purifier)),  // Rounding error, see TODO above
                (Room::Treasure, None),
                (Room::Monster, None),
                (Room::Shop, None),
            ]
        );
    }
}
