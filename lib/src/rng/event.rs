use crate::data::{Act, Card, Event, Relic};
use crate::map::Room;
use crate::types::{Floor, Gold, Hp};

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
    one_time_event_pool: Vec<Event>,
}

impl EventGenerator {
    pub fn new(seed: Seed) -> Self {
        let event_rng = StsRandom::from(seed);
        Self {
            act: Act::get(1),
            event_rng,
            monster_room_probability: 0.1,
            shop_probability: 0.03,
            treasure_room_probability: 0.02,
            one_time_event_pool: ONE_TIME_EVENTS.to_vec(),
        }
    }

    pub fn next_event(
        &mut self,
        floor: Floor,
        deck: &[Card],
        gold: Gold,
        hp: Hp,
        relics: &[Relic],
    ) -> (Room, Option<Event>) {
        // TODO: Last room was a shop
        // TODO: Relic::TinyChest
        // TODO: Relic::JuzuBracelet

        let room = *self.event_rng.weighted_choose(&[
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
                let shrine_and_special_pool = self
                    .act
                    .shrine_pool
                    .iter()
                    .copied()
                    .chain(self.one_time_event_pool.iter().copied())
                    .filter(|&event| self.filter_event(event, floor, deck, gold, hp, relics))
                    .collect::<Vec<_>>();
                let regular_event_pool = self
                    .act
                    .event_pool
                    .iter()
                    .copied()
                    .filter(|&event| self.filter_event(event, floor, deck, gold, hp, relics))
                    .collect::<Vec<_>>();
                let pool = *event_rng_clone.weighted_choose(&[
                    (shrine_and_special_pool.as_slice(), SHRINE_PROBABILITY),
                    (regular_event_pool.as_slice(), 1. - SHRINE_PROBABILITY),
                ]);
                (Room::Event, Some(*event_rng_clone.choose(pool)))
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
        hp: Hp,
        relics: &[Relic],
    ) -> bool {
        match event {
            Event::DesignerInSpire => self.act.number > 1 && hp > 3,
            Event::Duplicator => self.act.number > 1 && deck.len() >= 5,
            Event::FaceTrader => self.act.number < 3,
            Event::HypnotizingColoredMushrooms => floor >= 7,
            Event::KnowingSkull => self.act.number == 2 && hp >= 13,
            Event::Nloth => self.act.number == 2 && relics.len() >= 2,
            Event::OldBeggar => gold >= 75,
            Event::SecretPortal => false, // 13m 20s
            Event::TheDivineFountain => deck.iter().any(|card| card.is_curse()),
            Event::TheJoust => self.act.number == 2 && gold >= 50,
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
        let mut event_generator = EventGenerator::new(seed);
        let (room, event) =
            event_generator.next_event(3, IRONCLAD.starting_deck, 99, 80, &[Relic::BurningBlood]);
        assert_eq!(room, Room::Event);
        assert_eq!(event, Some(Event::UpgradeShrine));
        let (room, _) =
            event_generator.next_event(4, IRONCLAD.starting_deck, 99, 80, &[Relic::BurningBlood]);
        assert_eq!(room, Room::Shop);
        let (room, _) =
            event_generator.next_event(7, IRONCLAD.starting_deck, 99, 80, &[Relic::BurningBlood]);
        assert_eq!(room, Room::Monster);
        let (room, event) =
            event_generator.next_event(7, IRONCLAD.starting_deck, 99, 80, &[Relic::BurningBlood]);
        assert_eq!(room, Room::Event);
        assert_eq!(event, Some(Event::DeadAdventurer));
    }
}
