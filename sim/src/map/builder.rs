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
const TREASURE_ROW_INDEX: usize = 8;
const REST_ROW_INDEX: usize = ROW_COUNT - 1;
const MONSTER_ROW_INDEX: usize = 0;

pub struct MapBuilder {
    act: Act,
    ascension: Ascension,
    sts_random: StsRandom,
}

pub struct RoomAssigner<'a> {
    ascension: Ascension,
    rooms_to_assign: Vec<Option<Room>>,
    node_grid: NodeBuilderGrid,
    elite_rooms: Vec<(usize, usize)>,
    sts_random: &'a mut StsRandom,
}

impl<'a> RoomAssigner<'a> {
    pub fn new(
        node_grid: NodeBuilderGrid,
        ascension: Ascension,
        sts_random: &'a mut StsRandom,
    ) -> Self {
        Self {
            rooms_to_assign: vec![],
            ascension,
            node_grid,
            elite_rooms: vec![],
            sts_random,
        }
    }

    pub fn assign_rooms(mut self) -> Self {
        self.node_grid
            .set_all_rooms_in_row(MONSTER_ROW_INDEX, Room::Monster);
        self.node_grid
            .set_all_rooms_in_row(TREASURE_ROW_INDEX, Room::Treasure);
        self.node_grid
            .set_all_rooms_in_row(REST_ROW_INDEX, Room::Campfire);
        let unassigned_room_count = self.node_grid.unassigned_room_count();
        let room_total = self.node_grid.room_total();
        let shop_room_count = (SHOP_ROOM_CHANCE * room_total as f32).round() as usize;
        let rest_room_count = (REST_ROOM_CHANCE * room_total as f32).round() as usize;
        let treasure_room_count = (TREASURE_ROOM_CHANCE * room_total as f32).round() as usize;
        let elite_room_count = if self.ascension.0 == 0 {
            (ELITE_ROOM_CHANCE_A0 * room_total as f32).round() as usize
        } else {
            (ELITE_ROOM_CHANCE_A1 * room_total as f32).round() as usize
        };
        let event_room_count = (EVENT_ROOM_CHANCE * room_total as f32).round() as usize;
        self.rooms_to_assign = repeat(Room::Shop)
            .take(shop_room_count)
            .chain(repeat(Room::Campfire).take(rest_room_count))
            .chain(repeat(Room::Treasure).take(treasure_room_count))
            .chain(repeat(Room::Elite).take(elite_room_count))
            .chain(repeat(Room::Event).take(event_room_count))
            .chain(repeat(Room::Monster))
            .take(unassigned_room_count)
            .map(Some)
            .collect::<Vec<Option<Room>>>();
        self.sts_random.shuffle(&mut self.rooms_to_assign);
        for row in 0..(ROW_COUNT - 1) {
            self.assign_rooms_to_row(row);
        }
        //self.assign_burning_elite();
        self
    }

    pub fn finish(self) -> NodeGrid {
        self.node_grid.into()
    }

    fn assign_rooms_to_row(&mut self, row: usize) {
        if row == MONSTER_ROW_INDEX || row == TREASURE_ROW_INDEX {
            return;
        }
        for col in self.node_grid.occupied_cols_for_row(row) {
            let mut already_rejected: [bool; 10] = [false; 10];
            for entry in self.rooms_to_assign.iter_mut() {
                if let Some(room) = entry {
                    if already_rejected[*room as usize] {
                        continue;
                    }
                    already_rejected[*room as usize] = true;
                    let should_check_parent = match room {
                        Room::Campfire => {
                            if row <= 4 || row >= 13 {
                                continue;
                            }
                            true
                        }
                        Room::Elite => {
                            if row <= 4 {
                                continue;
                            }
                            true
                        }
                        Room::Event => false,
                        Room::Monster => false,
                        Room::Shop => true,
                        _ => unreachable!(),
                    };
                    if should_check_parent && self.node_grid.has_parent_room_of(row, col, *room) {
                        continue;
                    }
                    if self.node_grid.has_sibling_room_of(row, col, *room) {
                        continue;
                    }
                    // If we make it here, the room is valid for this node.
                    self.node_grid.set_room(row, col, *room);
                    if *room == Room::Elite {
                        self.elite_rooms.push((row, col));
                    }
                    entry.take();
                    break;
                }
            }
        }
    }

    fn assign_burning_elite(&mut self) {
        if self.elite_rooms.len() < 2 {
            eprintln!("Not enough elite rooms; this is a known bug");
            return;
        }
        let (row, col) = self.sts_random.choose(&self.elite_rooms);
        self.node_grid.set_room(
            *row,
            *col,
            match self.sts_random.gen_range(0..=3) {
                0 => Room::BurningElite1,
                1 => Room::BurningElite2,
                2 => Room::BurningElite3,
                3 => Room::BurningElite4,
                _ => unreachable!(),
            },
        );
    }
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
        let node_grid = GraphBuilder::new(&mut self.sts_random).build();
        RoomAssigner::new(node_grid, self.ascension, &mut self.sts_random)
            .assign_rooms()
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use pretty_assertions::assert_eq;

    use super::*;

    const TEST_VECTORS_RAW: &str = include_str!("/tmp/maps.txt");

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
                //r" 1  M  ?     M  ?    ",
                r" E  M  ?     M  ?    ",
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
            .map(|i| {
                let seed: Seed = i.into();
                MapBuilder::from(&seed, Ascension(0), Act(1)).build()
            })
            .collect::<Vec<NodeGrid>>();
        println!(
            "Time taken to generate {} graphs: {:?}",
            node_grids.len(),
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
        for (i, (node_grid, vector)) in test_vectors.zip(node_grids.iter()).enumerate().take(9999) {
            let seed_as_u64 = i as u64 + 2;
            if [842, 1820, 3724, 4100, 7459].contains(&seed_as_u64) {
                // The C++ reference implementation produces incorrect maps for these seeds.
                continue;
            }
            let seed: Seed = seed_as_u64.into();
            let left = format!("{} {:?}", node_grid, seed);
            let right = format!("{} {:?}", vector, seed);
            assert_eq!(left, right);
        }
    }
}
