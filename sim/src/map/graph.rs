use std::cmp::Ordering;
use std::fmt;

use crate::random::StsRandom;

use super::exit::Exit;
use super::map::Node;
use super::room::Room;
use super::{COLUMN_COUNT, ROW_COUNT};

const COLUMN_MAX: usize = COLUMN_COUNT - 1;
const PATH_DENSITY: usize = 6;

pub struct GraphBuilder<'a> {
    sts_random: &'a mut StsRandom,
    node_grid: NodeGridBuilder,
}

pub struct NodeBuilder {
    exit: Exit,
    entrance_cols: Vec<usize>,
}

#[derive(Default)]
pub struct NodeGridBuilder {
    grid: [[Option<NodeBuilder>; COLUMN_COUNT]; ROW_COUNT],
}

impl NodeGridBuilder {
    pub fn new() -> Self {
        Self {
            grid: Default::default(),
        }
    }

    fn remove(&mut self, row: usize, col: usize) -> Option<NodeBuilder> {
        self.grid[row][col].take()
    }

    fn has_exit(&self, row: usize, col: usize, exit: Exit) -> bool {
        self.grid[row][col]
            .as_ref()
            .map_or(false, |node| node.exit.contains(exit))
    }

    fn add_entrance(&mut self, row: usize, col: usize, entrance_col: usize) {
        self.grid[row][col] = Some(
            self.grid[row][col]
                .take()
                .unwrap_or_default()
                .add_entrance_col(entrance_col),
        );
    }

    fn add_exit(&mut self, row: usize, col: usize, exit: Exit) {
        self.grid[row][col] = Some(
            self.grid[row][col]
                .take()
                .unwrap_or_default()
                .add_exit(exit),
        );
    }

    fn entrance_cols(&self, row: usize, col: usize) -> impl Iterator<Item = &usize> {
        self.grid[row][col]
            .as_ref()
            .map(|node| node.entrance_cols.iter())
            .unwrap_or_else(|| [].iter())
    }

    fn entrances_collide(&self, row: usize, my_col: usize, other_col: usize, flip: bool) -> bool {
        if let (Some(my_node), Some(other_node)) = (
            &self.grid[row][my_col].as_ref(),
            &self.grid[row][other_col].as_ref(),
        ) {
            if flip {
                my_node.rightmost_entrance_col() == other_node.leftmost_entrance_col()
            } else {
                other_node.rightmost_entrance_col() == my_node.leftmost_entrance_col()
            }
        } else {
            false
        }
    }

    pub fn build(mut self) -> [[Option<Node>; COLUMN_COUNT]; ROW_COUNT] {
        self.grid
            .iter_mut()
            .map(|row| {
                row.iter_mut()
                    .map(|maybe_node| maybe_node.take().map(|node| node.build()))
                    .collect::<Vec<Option<Node>>>()
                    .try_into()
                    .expect("Builder should have built rows of the correct length")
            })
            .collect::<Vec<[Option<Node>; COLUMN_COUNT]>>()
            .try_into()
            .expect("Builder should have built the correct number of rows")
    }
}

impl<'a> GraphBuilder<'a> {
    pub fn new(sts_random: &'a mut StsRandom) -> Self {
        Self {
            sts_random,
            node_grid: NodeGridBuilder::new(),
        }
    }

    pub fn build(mut self) -> NodeGridBuilder {
        self.embed_paths();
        self.prune_bottom_row();
        self.node_grid
        /*
            .iter_mut()
            .map(|row| {
                row.iter_mut()
                    .map(|maybe_node| maybe_node.take().map(|node| node.build()))
                    .collect::<Vec<Option<Node>>>()
                    .try_into()
                    .expect("Builder should have built rows of the correct length")
            })
            .collect::<Vec<[Option<Node>; COLUMN_COUNT]>>()
            .try_into()
            .expect("Builder should have built the correct number of rows")
        */
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
            if col > 0 && !row1_seen[col - 1] && self.node_grid.has_exit(0, col, Exit::Left) {
                exits_to_keep |= Exit::Left;
                row1_seen[col - 1] = true;
            }
            if !row1_seen[col] && self.node_grid.has_exit(0, col, Exit::Straight) {
                exits_to_keep |= Exit::Straight;
                row1_seen[col] = true;
            }
            if col < COLUMN_MAX
                && !row1_seen[col + 1]
                && self.node_grid.has_exit(0, col, Exit::Right)
            {
                exits_to_keep |= Exit::Right;
                row1_seen[col + 1] = true;
            }
            self.node_grid.remove(0, col);
            if exits_to_keep != Exit::empty() {
                self.node_grid.add_exit(0, col, exits_to_keep);
            }
        }
    }

    fn embed_path(&mut self, mut col: usize) {
        for row in 0..(ROW_COUNT - 1) {
            let (exit, next_col) = self.propose_exit(col);
            let (exit, next_col) = self.avoid_small_cycles(row, col, exit, next_col);
            let (exit, next_col) = self.prevent_crossed_paths(row, col, exit, next_col);
            self.node_grid.add_exit(row, col, exit);
            self.node_grid.add_entrance(row + 1, next_col, col);
            col = next_col;
        }
        self.node_grid.add_exit(
            ROW_COUNT - 1,
            col,
            match col.cmp(&(COLUMN_COUNT / 2)) {
                Ordering::Less => Exit::Right,
                Ordering::Equal => Exit::Straight,
                Ordering::Greater => Exit::Left,
            },
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
        let dest_col = next_col;
        // The use of a for loop here is almost certainly a bug in the original code. The
        // intent seems to be to avoid small cycles like tiny parallelograms, isosceles
        // triangles, and diamonds with areas of 1 or 2 units.
        //
        // However, this loop is problematic for a few reasons:
        //   (1) While one iteration of the loop might avoid an unwanted cycle, the next
        //       iteration might reintroduce it.
        //   (2) The loop over parent/entrance edge list depends on the order in which those
        //       edges were introduced, and often continues unnecessarily after a new exit
        //       has been chosen.
        //   (3) The parent/edge list often contains duplicate entries for the same edge.
        //   (4) Iterations of the loop, although they change next_col, do not change the
        //       dest_col from which the parent information was originally sourced.
        //
        // Checking for small cycles could be done much more easily by just checking neighbors'
        // exits. This would allow us to pick an exit that doesn't introduce a small cycle
        // (if such an edge exists). As it's implemented now, we might end up with a small
        // cycle even if we could have chosen an edge that avoided it.
        for &other_col in self.node_grid.entrance_cols(row + 1, dest_col) {
            if other_col == my_col {
                continue;
            }
            // "other_col >= row" is almost certainly a bug in the original code; it's
            // probably supposed to be "other_col >= my_col". This causes a lot of small
            // cycles to be missed because the wrong comparisons are being performed.
            if self
                .node_grid
                .entrances_collide(row, my_col, other_col, other_col >= row)
            {
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
                if self.node_grid.has_exit(row, col - 1, Exit::Right) {
                    (Exit::Straight, col)
                } else {
                    (exit, next_col)
                }
            }
            Exit::Right => {
                if self.node_grid.has_exit(row, col + 1, Exit::Left) {
                    (Exit::Straight, col)
                } else {
                    (exit, next_col)
                }
            }
            _ => (exit, next_col),
        }
    }
}

impl fmt::Display for GraphBuilder<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.node_grid.fmt(f)
    }
}

impl fmt::Display for NodeGridBuilder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.grid.iter().rev() {
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
            if row.as_ptr() != self.grid[0].as_ptr() {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

impl NodeBuilder {
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

    pub fn build(self) -> Node {
        Node(Room::Monster, self.exit)
    }
}

impl Default for NodeBuilder {
    fn default() -> Self {
        Self {
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

    const TEST_VECTORS: &str = include_str!("map-exit-data.txt");

    fn exits_as_vec(node_grid: &NodeGridBuilder) -> [[u8; COLUMN_COUNT]; ROW_COUNT - 1] {
        let mut exits = [[0; COLUMN_COUNT]; ROW_COUNT - 1];
        for (row, row_nodes) in node_grid.grid.iter().enumerate().take(ROW_COUNT - 1) {
            for (col, maybe_node) in row_nodes.iter().enumerate() {
                if let Some(node) = maybe_node {
                    exits[row][col] = node.exit.bits();
                }
            }
        }
        exits
    }

    fn exits_as_base64(node_grid: &NodeGridBuilder) -> String {
        STANDARD.encode(
            exits_as_vec(node_grid)
                .as_flattened()
                .chunks(21)
                .map(|chunk| chunk.iter().fold(0, |acc, &exit| (acc << 3) | exit as u64))
                .flat_map(|val64| val64.to_be_bytes())
                .collect::<Vec<u8>>(),
        )
    }

    #[test]
    fn test_connection_graph() {
        let mut sts_random = StsRandom::from(2);
        let node_grid = GraphBuilder::new(&mut sts_random).build();
        assert_eq!(
            exits_as_vec(&node_grid),
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

        let mut sts_random = StsRandom::from(3);
        let node_grid = GraphBuilder::new(&mut sts_random).build();
        assert_eq!(
            exits_as_vec(&node_grid),
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
        assert_eq!(
            exits_as_base64(&node_grid),
            "IEgIgLCCBlAQEuAQWkBAmggGmQE0hBJQCAlggBKAgLgAAAACM0ANCg=="
        );
    }

    #[test]
    fn test_connection_graph_test_vectors() {
        let now = Instant::now();
        let node_grids = (2..10002) // (2..10000002)
            .map(|i| {
                let mut sts_random = StsRandom::from(i);
                GraphBuilder::new(&mut sts_random).build()
            })
            .collect::<Vec<NodeGridBuilder>>();
        println!(
            "Time taken to generate {} graphs: {:?}",
            node_grids.len(),
            now.elapsed()
        );
        for (i, vector) in TEST_VECTORS.lines().enumerate() {
            assert_eq!((i, exits_as_base64(&node_grids[i]).as_str()), (i, vector));
        }
    }
}
