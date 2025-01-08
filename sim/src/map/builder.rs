use std::iter::repeat;

use crate::act::Act;
use crate::ascension::Ascension;
use crate::game_context::GAME_CONTEXT;
use crate::random::StsRandom;
use crate::seed::Seed;

use super::graph::GraphBuilder;
use super::grid::{NodeBuilderGrid, NodeGrid};
use super::room::Room;
use super::{COLUMN_COUNT, ROW_COUNT};

const SHOP_ROOM_CHANCE: f32 = 0.05;
const REST_ROOM_CHANCE: f32 = 0.12;
const TREASURE_ROOM_CHANCE: f32 = 0.0;
const ELITE_ROOM_CHANCE_A0: f32 = 0.08;
const ELITE_ROOM_CHANCE_A1: f32 = ELITE_ROOM_CHANCE_A0 * 1.6;
const EVENT_ROOM_CHANCE: f32 = 0.22;

pub struct MapBuilder {
    act: Act,
    ascension: Ascension,
    sts_random: StsRandom,
}

impl MapBuilder {
    pub fn for_act(act: Act) -> Self {
        Self::from(&GAME_CONTEXT.seed, GAME_CONTEXT.ascension, act)
    }

    fn from(seed: &Seed, ascension: Ascension, act: Act) -> Self {
        let offset = if act.0 == 1 {
            1
        } else {
            act.0 * (100 * (act.0 - 1))
        };
        Self {
            act,
            ascension,
            sts_random: seed.with_offset(offset as u64).into(),
        }
    }

    pub fn build(mut self) -> NodeGrid {
        let mut node_grid = GraphBuilder::new(&mut self.sts_random).build();
        self.assign_rooms(&mut node_grid);
        node_grid.into()
    }

    fn assign_rooms(&mut self, node_grid: &mut NodeBuilderGrid) {
        self.assign_predetermined_rooms_to_row(0, Room::Monster, node_grid);
        self.assign_predetermined_rooms_to_row(8, Room::Treasure, node_grid);
        self.assign_predetermined_rooms_to_row(ROW_COUNT - 1, Room::Campfire, node_grid);
        let unassigned_room_count = node_grid.unassigned_room_count();
        let room_total = node_grid.room_total();
        let shop_room_count = (SHOP_ROOM_CHANCE * room_total as f32).round() as usize;
        let rest_room_count = (REST_ROOM_CHANCE * room_total as f32).round() as usize;
        let treasure_room_count = (TREASURE_ROOM_CHANCE * room_total as f32).round() as usize;
        let elite_room_count = if GAME_CONTEXT.ascension.0 == 0 {
            (ELITE_ROOM_CHANCE_A0 * room_total as f32).round() as usize
        } else {
            (ELITE_ROOM_CHANCE_A1 * room_total as f32).round() as usize
        };
        let event_room_count = (EVENT_ROOM_CHANCE * room_total as f32).round() as usize;
        let rooms: Vec<Room> = repeat(Room::Shop)
            .take(shop_room_count)
            .chain(repeat(Room::Campfire).take(rest_room_count))
            .chain(repeat(Room::Treasure).take(treasure_room_count))
            .chain(repeat(Room::Elite).take(elite_room_count))
            .chain(repeat(Room::Event).take(event_room_count))
            .chain(repeat(Room::Monster).take(unassigned_room_count))
            .collect();
    }

    fn assign_predetermined_rooms_to_row(
        &mut self,
        row: usize,
        room: Room,
        node_grid: &mut NodeBuilderGrid,
    ) {
        for col in 0..COLUMN_COUNT {
            node_grid.set_room(row, col, room);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /*
    #[test]
    fn test_map_0slaythespire() {
        let seed: Seed = "0SLAYTHESPIRE".try_into().unwrap();
        let map_act_1 = MapBuilder::from(&seed, Ascension(0), Act(1)).build();
        assert_eq!(
            map_act_1.to_string(),
            [
                r"  /     /   \  \     ",
                r" R     R     R  R    ",
                r" | \   |   /      \  ",
                r" E  M  $  M        E ",
                r"   \  \|  |      /   ",
                r"    ?  E  R     R    ",
                r"  /  / |    \   |    ",
                r" 1  M  ?     M  ?    ",
                r" |  | \  \ /  /      ",
                r" ?  M  R  ?  M       ",
                r" |/    |/ |  |       ",
                r" R     M  ?  R       ",
                r" | \ /  /      \     ",
                r" T  T  T        T    ",
                r" |  | \|          \  ",
                r" M  $  M           M ",
                r" |  |/   \       /   ",
                r" M  M     ?     M    ",
                r" |  | \     \ /      ",
                r" R  E  R     M       ",
                r" |    \  \ / |       ",
                r" M     ?  M  ?       ",
                r" |     |/ |  |       ",
                r" ?     M  ?  M       ",
                r" |   / |  | \  \     ",
                r" ?  M  M  $  ?  M    ",
                r" |/    |/      \|    ",
                r" M     M        M    ",
                r" |     |        |    ",
                r" M     M        M    ",
            ]
            .join("\n")
        );
    }
    */
}
