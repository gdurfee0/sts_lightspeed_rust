use std::cmp::Ordering;
use std::fmt;

use crate::act::Act;
use crate::random::StsRandom;

use super::exit::Exit;
use super::map::{Map, Node};
use super::room::Room;
use super::{COLUMN_COUNT, ROW_COUNT};

const COLUMN_MAX: usize = COLUMN_COUNT - 1;
const PATH_DENSITY: usize = 6;

pub struct MapBuilder {
    nodes: [[Option<NodeBuilder>; COLUMN_COUNT]; ROW_COUNT],
    sts_random: StsRandom,
}

struct NodeBuilder {
    room: Option<Room>,
    exit: Exit,
    entrance_cols: Vec<usize>,
}

impl MapBuilder {
    pub fn for_act(act: Act) -> Self {
        let offset = if act.0 == 1 {
            1
        } else {
            act.0 * (100 * (act.0 - 1))
        };
        Self {
            nodes: Default::default(),
            sts_random: StsRandom::with_offset(offset as u64),
        }
    }

    pub fn build(mut self) -> Map {
        self.embed_paths();
        self.prune_bottom_row();
        Map(self
            .nodes
            .iter_mut()
            .map(|row| {
                row.iter_mut()
                    .map(|maybe_node| {
                        maybe_node
                            .take()
                            // TODO: Actual room assignments instead of hard coded value
                            .and_then(|node| node.set_room(Room::Monster).build())
                    })
                    .collect::<Vec<Option<Node>>>()
            })
            .collect::<Vec<Vec<Option<Node>>>>()
            .into_iter()
            .map(|row| row.try_into().unwrap())
            .collect::<Vec<[Option<Node>; COLUMN_COUNT]>>()
            .try_into()
            .unwrap())
    }

    fn embed_paths(&mut self) {
        let first_path_start_col = self.sts_random.gen_range(0..COLUMN_COUNT);
        self.embed_path(first_path_start_col);
        for i in 1..PATH_DENSITY {
            let mut path_start_col = self.sts_random.gen_range(0..COLUMN_COUNT);
            while i == 1 && path_start_col == first_path_start_col {
                path_start_col = self.sts_random.gen_range(0..COLUMN_COUNT);
            }
            self.embed_path(path_start_col);
        }
    }

    fn prune_bottom_row(&mut self) {
        let mut row1_seen = [false; COLUMN_COUNT];
        for col in 0..COLUMN_COUNT {
            let mut exits_to_keep = Exit::empty();
            if let Some(node) = self.nodes[0][col].as_ref() {
                if col > 0 && node.exit.contains(Exit::Left) && !row1_seen[col - 1] {
                    exits_to_keep |= Exit::Left;
                    row1_seen[col - 1] = true;
                }
                if node.exit.contains(Exit::Straight) && !row1_seen[col] {
                    exits_to_keep |= Exit::Straight;
                    row1_seen[col] = true;
                }
                if col < COLUMN_MAX && node.exit.contains(Exit::Right) && !row1_seen[col + 1] {
                    exits_to_keep |= Exit::Right;
                    row1_seen[col + 1] = true;
                }
            }
            self.nodes[0][col] = if exits_to_keep != Exit::empty() {
                Some(NodeBuilder::default().add_exit(exits_to_keep))
            } else {
                None
            };
        }
    }

    fn embed_path(&mut self, mut col: usize) {
        for row in 0..(ROW_COUNT - 1) {
            let (exit, next_col) = self.propose_exit(col);
            let (exit, next_col) = self.avoid_small_cycles(row, col, exit, next_col);
            let (exit, next_col) = self.prevent_crossed_paths(row, col, exit, next_col);
            self.nodes[row][col] = Some(
                self.nodes[row][col]
                    .take()
                    .unwrap_or_default()
                    .add_exit(exit),
            );
            self.nodes[row + 1][next_col] = Some(
                self.nodes[row + 1][next_col]
                    .take()
                    .unwrap_or_default()
                    .add_entrance_col(col),
            );
            col = next_col;
        }
        self.nodes[ROW_COUNT - 1][col] = Some(
            self.nodes[ROW_COUNT - 1][col]
                .take()
                .unwrap_or_default()
                .add_exit(match col.cmp(&(COLUMN_COUNT / 2)) {
                    Ordering::Less => Exit::Right,
                    Ordering::Equal => Exit::Straight,
                    Ordering::Greater => Exit::Left,
                }),
        );
    }

    fn propose_exit(&mut self, col: usize) -> (Exit, usize) {
        match col {
            0 => *self
                .sts_random
                .choose(&[(Exit::Straight, 0), (Exit::Right, 1)]),
            1..COLUMN_MAX => *self.sts_random.choose(&[
                (Exit::Left, col - 1),
                (Exit::Straight, col),
                (Exit::Right, col + 1),
            ]),
            COLUMN_MAX => *self
                .sts_random
                .choose(&[(Exit::Left, COLUMN_MAX - 1), (Exit::Straight, COLUMN_MAX)]),
            _ => unreachable!(),
        }
    }

    fn avoid_small_cycles(
        &mut self,
        row: usize,
        my_col: usize,
        mut exit: Exit,
        mut next_col: usize,
    ) -> (Exit, usize) {
        if row == 0 {
            return (exit, next_col);
        }
        if let Some(dest_node) = self.nodes[row + 1][next_col].as_ref() {
            let my_node = self.nodes[row][my_col]
                .as_ref()
                .expect("Node already populated");
            // The use of a for loop here is almost certainly a bug in the original code. The
            // intent seems to be to avoid small cycles like tiny parallelograms, isosceles
            // triangles, and diamonds.
            //
            // However, this loop is problematic for a few reasons:
            //   (1) While one iteration of the loop might avoid an unwanted cycle, the next
            //       iteration might reintroduce it.
            //   (2) The loop over parent/entrance edge list depends on the order in which those
            //       edges were introduced.
            //   (3) The parent/edge list often contains duplicate entries for the same edge.
            //
            // Checking for small cycles could be done much more easily by just checking neighbors'
            // exits. This would allow us to pick an exit that doesn't introduce a small cycle
            // (if such an edge exists). As it's implemented now, we might end up with a small
            // cycle even if we could have chosen an edge that avoided it.
            for &other_col in &dest_node.entrance_cols {
                if other_col == my_col {
                    continue;
                }
                let other_node = self.nodes[row][other_col]
                    .as_ref()
                    .expect("Other node already populated");
                // "other_col >= row" is almost certainly a bug in the original code; it's
                // probably supposed to be "other_col >= my_col". This causes a lot of small
                // cycles to be missed because the wrong edges are being checked.
                if Self::entrances_collide(my_node, other_node, other_col >= row) {
                    (exit, next_col) = match next_col.cmp(&my_col) {
                        Ordering::Less => *self.sts_random.choose(&[
                            (Exit::Straight, my_col),
                            match my_col {
                                COLUMN_MAX => (Exit::Straight, COLUMN_MAX),
                                _ => (Exit::Right, my_col + 1),
                            },
                        ]),
                        Ordering::Equal => *self.sts_random.choose(&[
                            match my_col {
                                0 => (Exit::Right, 1),
                                _ => (Exit::Left, my_col - 1), // bounce instead of clamp; intended?
                            },
                            (Exit::Straight, my_col),
                            match my_col {
                                COLUMN_MAX => (Exit::Left, COLUMN_MAX - 1),
                                _ => (Exit::Right, my_col + 1), // bounce not clamp; intended?
                            },
                        ]),
                        Ordering::Greater => *self.sts_random.choose(&[
                            match my_col {
                                0 => (Exit::Straight, 0),
                                _ => (Exit::Left, my_col - 1),
                            },
                            (Exit::Straight, my_col),
                        ]),
                    };
                }
            }
        }
        (exit, next_col)
    }

    fn prevent_crossed_paths(
        &self,
        row: usize,
        col: usize,
        exit: Exit,
        next_col: usize,
    ) -> (Exit, usize) {
        match exit {
            Exit::Left => {
                if self.nodes[row]
                    .get(col - 1)
                    .and_then(|node| node.as_ref().map(|n| &n.exit))
                    .map_or(false, |exit| exit.contains(Exit::Right))
                {
                    (Exit::Straight, col)
                } else {
                    (exit, next_col)
                }
            }
            Exit::Right => {
                if self.nodes[row]
                    .get(col + 1)
                    .and_then(|node| node.as_ref().map(|n| &n.exit))
                    .map_or(false, |exit| exit.contains(Exit::Left))
                {
                    (Exit::Straight, col)
                } else {
                    (exit, next_col)
                }
            }
            _ => (exit, next_col),
        }
    }

    fn entrances_collide(node: &NodeBuilder, other_node: &NodeBuilder, flip: bool) -> bool {
        if flip {
            node.rightmost_entrance_col() == other_node.leftmost_entrance_col()
        } else {
            other_node.rightmost_entrance_col() == node.leftmost_entrance_col()
        }
    }
}

impl fmt::Display for MapBuilder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.nodes.iter().rev() {
            for maybe_node in row.iter() {
                match maybe_node {
                    Some(node) => {
                        write!(f, "{} ", node)?;
                    }
                    None => {
                        write!(f, "       ")?;
                    }
                }
            }
            if row.as_ptr() != self.nodes[0].as_ptr() {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

impl NodeBuilder {
    pub fn set_room(mut self, room: Room) -> Self {
        self.room = Some(room);
        self
    }

    pub fn add_exit(mut self, exit: Exit) -> Self {
        self.exit |= exit;
        self
    }

    pub fn add_entrance_col(mut self, entrance_col: usize) -> Self {
        self.entrance_cols.push(entrance_col);
        self
    }

    pub fn leftmost_entrance_col(&self) -> Option<usize> {
        self.entrance_cols.iter().min().copied()
    }

    pub fn rightmost_entrance_col(&self) -> Option<usize> {
        self.entrance_cols.iter().max().copied()
    }

    pub fn build(self) -> Option<Node> {
        self.room.map(|room| Node(room, self.exit))
    }
}

impl Default for NodeBuilder {
    fn default() -> Self {
        Self {
            room: None,
            exit: Exit::empty(),
            entrance_cols: Vec::with_capacity(PATH_DENSITY),
        }
    }
}

impl fmt::Display for NodeBuilder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let entrance_cols = self.entrance_cols.iter().map(|&col| col.to_string()).chain(
            std::iter::repeat("-".to_string()).take(PATH_DENSITY - self.entrance_cols.len()),
        );
        write!(f, "{}", entrance_cols.collect::<String>())
    }
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use super::*;

    use base64::engine::general_purpose::STANDARD;
    use base64::Engine;

    impl Map {
        fn exits_as_vec(&self) -> [[u8; COLUMN_COUNT]; ROW_COUNT - 1] {
            let mut exits = [[0; COLUMN_COUNT]; ROW_COUNT - 1];
            for (row, row_nodes) in self.0.iter().enumerate().take(ROW_COUNT - 1) {
                for (col, maybe_node) in row_nodes.iter().enumerate() {
                    if let Some(node) = maybe_node {
                        exits[row][col] = node.1.bits();
                    }
                }
            }
            exits
        }

        fn exits_as_base64(&self) -> String {
            STANDARD.encode(
                self.exits_as_vec()
                    .as_flattened()
                    .chunks(21)
                    .map(|chunk| chunk.iter().fold(0, |acc, &exit| (acc << 3) | exit as u64))
                    .flat_map(|val64| val64.to_be_bytes())
                    .collect::<Vec<u8>>(),
            )
        }
    }

    #[test]
    fn test_connection_graph() {
        let map = MapBuilder {
            nodes: Default::default(),
            sts_random: StsRandom::from(2),
        }
        .build();
        // base64 encode the exit vector to make it easier to compare
        assert_eq!(
            map.exits_as_vec(),
            [
                [0, 6, 0, 1, 0, 0, 0,],
                [1, 2, 0, 0, 6, 0, 0,],
                [0, 2, 0, 6, 5, 0, 0,],
                [0, 6, 4, 5, 0, 1, 0,],
                [1, 1, 2, 0, 1, 0, 4,],
                [0, 4, 3, 0, 0, 3, 0,],
                [2, 0, 5, 2, 0, 4, 2,],
                [2, 4, 0, 6, 4, 0, 4,],
                [1, 0, 2, 6, 0, 1, 0,],
                [0, 1, 2, 2, 0, 0, 4,],
                [0, 0, 6, 4, 0, 4, 0,],
                [0, 2, 7, 0, 4, 0, 0,],
                [0, 3, 1, 2, 0, 0, 0,],
                [0, 1, 2, 5, 0, 0, 0,],
            ]
        );

        let map = MapBuilder {
            nodes: Default::default(),
            sts_random: StsRandom::from(3),
        }
        .build();
        assert_eq!(
            map.exits_as_vec(),
            [
                [2, 0, 1, 1, 0, 0, 2,],
                [1, 0, 0, 2, 6, 0, 4,],
                [0, 4, 0, 3, 1, 2, 0,],
                [1, 0, 0, 2, 2, 7, 0,],
                [0, 1, 0, 1, 3, 2, 2,],
                [0, 0, 4, 0, 2, 3, 2,],
                [0, 4, 0, 0, 6, 4, 6,],
                [2, 0, 0, 4, 6, 4, 4,],
                [1, 0, 1, 1, 1, 2, 0,],
                [0, 4, 0, 1, 1, 3, 0,],
                [1, 0, 0, 0, 2, 2, 4,],
                [0, 1, 0, 0, 2, 7, 0,],
                [0, 0, 1, 0, 6, 3, 2,],
                [0, 0, 0, 6, 4, 1, 2,],
            ]
        );

        let map = MapBuilder {
            nodes: Default::default(),
            sts_random: StsRandom::from(3),
        }
        .build();
        assert_eq!(
            map.exits_as_base64(),
            "IEgIgLCCBlAQEuAQWkBAmggGmQE0hBJQCAlggBKAgLgAAAACM0ANCg=="
        );
    }

    const TEST_VECTORS: &str = include_str!("map-exit-data.txt");

    #[test]
    fn test_connection_graph_test_vectors() {
        let now = Instant::now();
        let maps: Vec<Map> = (2..10002) // (2..10000002)
            .map(|i| {
                MapBuilder {
                    nodes: Default::default(),
                    sts_random: StsRandom::from(i),
                }
                .build()
            })
            .collect();
        println!(
            "Time taken to generate {} maps: {:?}",
            maps.len(),
            now.elapsed()
        );
        for (i, vector) in TEST_VECTORS.lines().enumerate() {
            assert_eq!(maps[i].exits_as_base64(), vector);
        }
    }
}
