use std::iter::repeat;

use crate::data::act::Act;
use crate::rng::{Seed, StsRandom};
use crate::{ColumnIndex, RowIndex};

use super::graph::GraphBuilder;
use super::grid::{NodeBuilderGrid, NodeGrid};
use super::room::Room;
use super::ROW_COUNT;

const SHOP_ROOM_CHANCE: f32 = 0.05;
const REST_ROOM_CHANCE: f32 = 0.12;
const TREASURE_ROOM_CHANCE: f32 = 0.0;
const ELITE_ROOM_CHANCE: f32 = 0.08;
const EVENT_ROOM_CHANCE: f32 = 0.22;
const TREASURE_ROW_INDEX: RowIndex = 8;
const REST_ROW_INDEX: RowIndex = ROW_COUNT - 1;
const MONSTER_ROW_INDEX: RowIndex = 0;

pub struct MapBuilder {
    act: &'static Act,
    map_rng: StsRandom,
}

impl MapBuilder {
    pub fn from(seed: Seed, act: &'static Act) -> Self {
        let offset = act.map_seed_offset;
        Self {
            act,
            map_rng: StsRandom::from(seed.with_offset(offset)),
        }
    }

    pub fn build(mut self) -> NodeGrid {
        if self.act == Act::get(4) {
            todo!("Act 4");
        }
        let node_grid = GraphBuilder::new(&mut self.map_rng).build();
        RoomAssigner::new(node_grid, &mut self.map_rng)
            .assign_rooms()
            .finish()
    }
}

struct RoomAssigner<'a> {
    node_grid: NodeBuilderGrid,
    elite_rooms: Vec<(RowIndex, ColumnIndex)>,
    map_rng: &'a mut StsRandom,
}

impl<'a> RoomAssigner<'a> {
    pub fn new(node_grid: NodeBuilderGrid, map_rng: &'a mut StsRandom) -> Self {
        Self {
            node_grid,
            elite_rooms: vec![],
            map_rng,
        }
    }

    pub fn assign_rooms(mut self) -> Self {
        self.node_grid
            .set_all_rooms_in_row(MONSTER_ROW_INDEX, Room::Monster);
        self.node_grid
            .set_all_rooms_in_row(TREASURE_ROW_INDEX, Room::Treasure);
        self.node_grid
            .set_all_rooms_in_row(REST_ROW_INDEX, Room::RestSite);
        let unassigned_room_count = self.node_grid.unassigned_room_count();
        let room_total = self.node_grid.room_almost_total();
        let shop_room_count = (SHOP_ROOM_CHANCE * room_total as f32).round() as usize;
        let rest_room_count = (REST_ROOM_CHANCE * room_total as f32).round() as usize;
        let treasure_room_count = (TREASURE_ROOM_CHANCE * room_total as f32).round() as usize;
        // TODO: ascension
        let elite_room_count = (ELITE_ROOM_CHANCE * room_total as f32).round() as usize;
        let event_room_count = (EVENT_ROOM_CHANCE * room_total as f32).round() as usize;
        let mut unassigned_rooms = repeat(Room::Shop)
            .take(shop_room_count)
            .chain(repeat(Room::RestSite).take(rest_room_count))
            .chain(repeat(Room::Treasure).take(treasure_room_count))
            .chain(repeat(Room::Elite).take(elite_room_count))
            .chain(repeat(Room::Event).take(event_room_count))
            .chain(repeat(Room::Monster))
            .take(unassigned_room_count)
            .map(Some)
            .collect::<Vec<Option<Room>>>();
        self.map_rng.shuffle(&mut unassigned_rooms);
        let mut start_index = 0;
        for row_index in 0..(ROW_COUNT - 1) {
            if row_index == MONSTER_ROW_INDEX || row_index == TREASURE_ROW_INDEX {
                continue;
            }
            for column_index in self.node_grid.nonempty_columns_for_row(row_index) {
                let mut rooms_already_considered: [bool; 10] = [false; 10];
                let mut some_room_already_rejected = false;
                for (i, entry) in unassigned_rooms[start_index..].iter_mut().enumerate() {
                    if let Some(room) = entry {
                        if rooms_already_considered[*room as usize] {
                            continue;
                        }
                        rooms_already_considered[*room as usize] = true;
                        let (reject_outright, parent_must_be_different) = match room {
                            Room::RestSite => (!(5..13).contains(&row_index), true),
                            Room::Elite => (row_index <= 4, true),
                            Room::Event | Room::Monster => (false, false),
                            Room::Shop => (false, true),
                            _ => unreachable!(),
                        };
                        if reject_outright
                            || (parent_must_be_different
                                && self.node_grid.has_parent_room_of(
                                    row_index,
                                    column_index,
                                    *room,
                                ))
                            || (self.node_grid.has_left_sibling_room_of(
                                row_index,
                                column_index,
                                *room,
                            ))
                        {
                            some_room_already_rejected = true;
                            continue;
                        }
                        // If we make it here, the room is valid for this node.
                        self.node_grid.set_room(row_index, column_index, *room);
                        if *room == Room::Elite {
                            self.elite_rooms.push((row_index, column_index));
                        }
                        entry.take();
                        if !some_room_already_rejected {
                            start_index += i;
                        }
                        break;
                    }
                }
            }
        }
        self.assign_burning_elite();
        self
    }

    pub fn finish(self) -> NodeGrid {
        self.node_grid.into()
    }

    fn assign_burning_elite(&mut self) {
        if self.elite_rooms.len() < 2 {
            eprintln!("Not enough elite rooms; this is a known bug");
            return;
        }
        let (row_index, column_index) = self.map_rng.choose(&self.elite_rooms);
        self.node_grid.set_room(
            *row_index,
            *column_index,
            match self.map_rng.gen_range(0..=3) {
                0 => Room::BurningElite1,
                1 => Room::BurningElite2,
                2 => Room::BurningElite3,
                3 => Room::BurningElite4,
                _ => unreachable!(),
            },
        );
    }
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use pretty_assertions::assert_eq;

    use super::*;

    const TEST_VECTORS_RAW: &str = include_str!("maps.txt");

    #[test]
    fn test_map_0slaythespire() {
        let seed = Seed::try_from("0SLAYTHESPIRE").unwrap();
        let map_act_1 = MapBuilder::from(seed, Act::get(1)).build();
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

    #[test]
    fn test_lots_of_maps() {
        let now = Instant::now();
        let node_grids = (2..10002) // (2..10000002)
            .map(|i| MapBuilder::from(Seed::from(i), Act::get(1)).build())
            .collect::<Vec<NodeGrid>>();
        println!(
            "Time taken to generate {} graph{}: {:?}",
            node_grids.len(),
            if node_grids.len() == 1 { "" } else { "s" },
            now.elapsed()
        );
        let test_vector_lines = TEST_VECTORS_RAW
            .lines()
            .map(str::to_string)
            .collect::<Vec<String>>();
        let test_vectors = test_vector_lines.chunks(31).map(|chunk| {
            chunk
                .iter()
                .skip(1)
                .cloned()
                .collect::<Vec<String>>()
                .join("\n")
        });
        for (i, (node_grid, vector)) in test_vectors.zip(node_grids.iter()).enumerate() {
            let seed_as_u64 = i as u64 + 2;
            if [842, 1820, 3724, 4100, 7459].contains(&seed_as_u64) {
                // The C++ reference implementation, which produced the test vector file,
                // produces incorrect maps for these seeds.
                //
                // TODO: Offer a fix for the C++ implementation and remove this check.
                continue;
            }
            let seed = Seed::from(seed_as_u64);
            let left = format!("{} {:?}", node_grid, seed);
            let right = format!("{} {:?}", vector, seed);
            assert_eq!(left, right);
        }
    }
}
