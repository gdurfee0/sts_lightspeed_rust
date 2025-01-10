use std::cmp::Ordering;
use std::fmt;

use crate::rng::StsRandom;

use super::exit::ExitBits;
use super::grid::NodeBuilderGrid;
use super::{COLUMN_COUNT, COLUMN_MAX, PATH_DENSITY, ROW_COUNT};

pub struct GraphBuilder<'a> {
    sts_random: &'a mut StsRandom,
    node_grid: NodeBuilderGrid,
}

impl<'a> GraphBuilder<'a> {
    pub fn new(sts_random: &'a mut StsRandom) -> Self {
        Self {
            sts_random,
            node_grid: NodeBuilderGrid::new(),
        }
    }

    pub fn build(mut self) -> NodeBuilderGrid {
        self.embed_paths();
        self.prune_bottom_row();
        self.node_grid
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
            let mut exits_to_keep = ExitBits::empty();
            if col > 0 && !row1_seen[col - 1] && self.node_grid.has_exit(0, col, ExitBits::Left) {
                exits_to_keep |= ExitBits::Left;
                row1_seen[col - 1] = true;
            }
            if !row1_seen[col] && self.node_grid.has_exit(0, col, ExitBits::Up) {
                exits_to_keep |= ExitBits::Up;
                row1_seen[col] = true;
            }
            if col < COLUMN_MAX
                && !row1_seen[col + 1]
                && self.node_grid.has_exit(0, col, ExitBits::Right)
            {
                exits_to_keep |= ExitBits::Right;
                row1_seen[col + 1] = true;
            }
            self.node_grid.remove(0, col);
            if exits_to_keep != ExitBits::empty() {
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
            self.node_grid.record_parent_col(row + 1, next_col, col);
            col = next_col;
        }
        self.node_grid.add_exit(
            ROW_COUNT - 1,
            col,
            match col.cmp(&(COLUMN_COUNT / 2)) {
                Ordering::Less => ExitBits::Right,
                Ordering::Equal => ExitBits::Up,
                Ordering::Greater => ExitBits::Left,
            },
        );
    }

    fn propose_exit(&mut self, col: usize) -> (ExitBits, usize) {
        match col {
            0 => *self
                .sts_random
                .choose(&[(ExitBits::Up, 0), (ExitBits::Right, 1)]),
            1..COLUMN_MAX => *self.sts_random.choose(&[
                (ExitBits::Left, col - 1),
                (ExitBits::Up, col),
                (ExitBits::Right, col + 1),
            ]),
            COLUMN_MAX => *self
                .sts_random
                .choose(&[(ExitBits::Left, COLUMN_MAX - 1), (ExitBits::Up, COLUMN_MAX)]),
            _ => unreachable!(),
        }
    }

    fn avoid_small_cycles(
        &mut self,
        row: usize,
        my_col: usize,
        mut exit: ExitBits,
        mut next_col: usize,
    ) -> (ExitBits, usize) {
        if row == 0 {
            return (exit, next_col);
        }
        let dest_col = next_col;
        // The use of a for loop here is almost certainly a bug in the original code. The
        // intent seems to be to avoid small cycles* like tiny parallelograms, isosceles
        // triangles, and diamonds.
        //
        // *Cycles in the sense that they would be cycles if the graph were not directed,
        // and small in the sense that they would encompass areas of 1 or 2 units.
        //
        // However, this loop is problematic for a few reasons:
        //   (1) While one iteration of the loop might avoid an unwanted cycle, the next
        //       iteration might reintroduce it.
        //   (2) The loop over parent edge list depends on the order in which those
        //       edges were introduced, and often continues unnecessarily after a new exit
        //       has been chosen.
        //   (3) The parent/edge list often contains duplicate entries for the same edge.
        //   (4) Iterations of the loop, although they change next_col, do not change the
        //       dest_col from which the parent information was originally sourced, so ancestor
        //       detection doesn't reflect newly-proposed new_col generated inside the loop.
        //
        // Checking for small cycles could be done much more easily by just checking neighbors'
        // exits. This would allow us to pick an exit that doesn't introduce a small cycle
        // (if such an edge exists).
        //
        // As it's implemented now (and we keep this implementation for fidelity with the
        // original game), as a result of these issues, along with the buggy implementation of
        // shares_parent_with, this is basically an rng that *occasionally* avoids small cycles,
        // but usually doesn't.
        for &other_col in self.node_grid.recorded_parent_cols_iter(row + 1, dest_col) {
            if other_col == my_col {
                continue;
            }
            if self
                .node_grid
                .buggy_implementation_of_shares_parent_with(row, my_col, other_col)
            {
                (exit, next_col) = match next_col.cmp(&my_col) {
                    Ordering::Less => *self.sts_random.choose(&[
                        (ExitBits::Up, my_col),
                        match my_col {
                            COLUMN_MAX => (ExitBits::Up, COLUMN_MAX),
                            _ => (ExitBits::Right, my_col + 1),
                        },
                    ]),
                    Ordering::Equal => *self.sts_random.choose(&[
                        match my_col {
                            0 => (ExitBits::Right, 1),
                            _ => (ExitBits::Left, my_col - 1), // bounce instead of clamp; intended?
                        },
                        (ExitBits::Up, my_col),
                        match my_col {
                            COLUMN_MAX => (ExitBits::Left, COLUMN_MAX - 1),
                            _ => (ExitBits::Right, my_col + 1), // bounce not clamp; intended?
                        },
                    ]),
                    Ordering::Greater => *self.sts_random.choose(&[
                        match my_col {
                            0 => (ExitBits::Up, 0),
                            _ => (ExitBits::Left, my_col - 1),
                        },
                        (ExitBits::Up, my_col),
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
        exit: ExitBits,
        next_col: usize,
    ) -> (ExitBits, usize) {
        match exit {
            ExitBits::Left => {
                if self.node_grid.has_exit(row, col - 1, ExitBits::Right) {
                    (ExitBits::Up, col)
                } else {
                    (exit, next_col)
                }
            }
            ExitBits::Up => (exit, next_col),
            ExitBits::Right => {
                if self.node_grid.has_exit(row, col + 1, ExitBits::Left) {
                    (ExitBits::Up, col)
                } else {
                    (exit, next_col)
                }
            }
            _ => unreachable!(),
        }
    }
}

impl fmt::Display for GraphBuilder<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.node_grid.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use super::*;

    const TEST_VECTORS: &str = include_str!("map-exit-data.txt");

    #[test]
    fn test_connection_graph() {
        let mut sts_random = StsRandom::from(2);
        let node_grid = GraphBuilder::new(&mut sts_random).build();
        assert_eq!(
            node_grid.exit_bits_as_vec(),
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
            node_grid.exit_bits_as_vec(),
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
            node_grid.exit_bits_as_base64(),
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
            .collect::<Vec<NodeBuilderGrid>>();
        println!(
            "Time taken to generate {} graphs: {:?}",
            node_grids.len(),
            now.elapsed()
        );
        for (i, vector) in TEST_VECTORS.lines().enumerate() {
            assert_eq!(
                (i, node_grids[i].exit_bits_as_base64().as_str()),
                (i, vector)
            );
        }
    }
}
