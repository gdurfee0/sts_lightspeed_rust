use std::cmp::Ordering;
use std::fmt;

use crate::rng::StsRandom;
use crate::{ColumnIndex, RowIndex};

use super::exit::ExitBits;
use super::grid::NodeBuilderGrid;
use super::{COLUMN_COUNT, COLUMN_MAX, PATH_DENSITY, ROW_COUNT};

pub struct GraphBuilder<'a> {
    map_rng: &'a mut StsRandom,
    node_grid: NodeBuilderGrid,
}

impl<'a> GraphBuilder<'a> {
    pub fn new(map_rng: &'a mut StsRandom) -> Self {
        Self {
            map_rng,
            node_grid: NodeBuilderGrid::default(),
        }
    }

    pub fn build(mut self) -> NodeBuilderGrid {
        self.embed_paths();
        self.prune_bottom_row();
        self.node_grid
    }

    fn embed_paths(&mut self) {
        let first_path_column_index = self.map_rng.gen_range(0..COLUMN_COUNT);
        self.embed_path(first_path_column_index);
        for i in 1..PATH_DENSITY {
            let mut column_index = self.map_rng.gen_range(0..COLUMN_COUNT);
            while i == 1 && column_index == first_path_column_index {
                column_index = self.map_rng.gen_range(0..COLUMN_COUNT);
            }
            self.embed_path(column_index);
        }
    }

    fn prune_bottom_row(&mut self) {
        let mut row1_seen = [false; COLUMN_COUNT];
        for column_index in 0..COLUMN_COUNT {
            let mut exits_to_keep = ExitBits::empty();
            if column_index > 0
                && !row1_seen[column_index - 1]
                && self.node_grid.has_exit(0, column_index, ExitBits::Left)
            {
                exits_to_keep |= ExitBits::Left;
                row1_seen[column_index - 1] = true;
            }
            if !row1_seen[column_index] && self.node_grid.has_exit(0, column_index, ExitBits::Up) {
                exits_to_keep |= ExitBits::Up;
                row1_seen[column_index] = true;
            }
            if column_index < COLUMN_MAX
                && !row1_seen[column_index + 1]
                && self.node_grid.has_exit(0, column_index, ExitBits::Right)
            {
                exits_to_keep |= ExitBits::Right;
                row1_seen[column_index + 1] = true;
            }
            self.node_grid.remove(0, column_index);
            if exits_to_keep != ExitBits::empty() {
                self.node_grid.add_exit(0, column_index, exits_to_keep);
            }
        }
    }

    fn embed_path(&mut self, mut column_index: ColumnIndex) {
        for row_index in 0..(ROW_COUNT - 1) {
            let (exit, next_column_index) = self.propose_exit(column_index);
            let (exit, next_column_index) =
                self.avoid_small_cycles(row_index, column_index, exit, next_column_index);
            let (exit, next_column_index) =
                self.prevent_crossed_paths(row_index, column_index, exit, next_column_index);
            self.node_grid.add_exit(row_index, column_index, exit);
            self.node_grid
                .receord_parent_column(row_index + 1, next_column_index, column_index);
            column_index = next_column_index;
        }
        self.node_grid.add_exit(
            ROW_COUNT - 1,
            column_index,
            match column_index.cmp(&(COLUMN_COUNT / 2)) {
                Ordering::Less => ExitBits::Right,
                Ordering::Equal => ExitBits::Up,
                Ordering::Greater => ExitBits::Left,
            },
        );
    }

    fn propose_exit(&mut self, column_index: ColumnIndex) -> (ExitBits, ColumnIndex) {
        match column_index {
            0 => *self
                .map_rng
                .choose(&[(ExitBits::Up, 0), (ExitBits::Right, 1)]),
            1..COLUMN_MAX => *self.map_rng.choose(&[
                (ExitBits::Left, column_index - 1),
                (ExitBits::Up, column_index),
                (ExitBits::Right, column_index + 1),
            ]),
            COLUMN_MAX => *self
                .map_rng
                .choose(&[(ExitBits::Left, COLUMN_MAX - 1), (ExitBits::Up, COLUMN_MAX)]),
            _ => unreachable!(),
        }
    }

    fn avoid_small_cycles(
        &mut self,
        my_row_index: RowIndex,
        my_column_index: ColumnIndex,
        mut exit: ExitBits,
        mut next_column_index: ColumnIndex,
    ) -> (ExitBits, usize) {
        if my_row_index == 0 {
            return (exit, next_column_index);
        }
        let dest_column_index = next_column_index;
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
        //   (3) The parent edge list often contains duplicate entries for the same parent.
        //   (4) Iterations of the loop, although they change next_column_index, do not change the
        //       dest_column_index from which the parent information was originally sourced,
        //       so ancestor detection doesn't reflect the newly-proposed next_column_index
        //       generated inside the loop.
        //
        // Checking for small cycles could be done much more easily by just checking neighbors'
        // exits. This would allow us to pick an exit that doesn't introduce a small cycle
        // (if such an edge exists).
        //
        // As it's implemented now (and we keep this implementation for fidelity with the
        // original game), as a result of these issues, along with the buggy implementation of
        // shares_parent_with, this is basically an rng that *occasionally* avoids small cycles,
        // but usually doesn't.
        for &other_column_index in self
            .node_grid
            .recorded_parent_columns_iter(my_row_index + 1, dest_column_index)
        {
            if other_column_index == my_column_index {
                continue;
            }
            if self.node_grid.buggy_implementation_of_shares_parent_with(
                my_row_index,
                my_column_index,
                other_column_index,
            ) {
                (exit, next_column_index) = match next_column_index.cmp(&my_column_index) {
                    Ordering::Less => *self.map_rng.choose(&[
                        (ExitBits::Up, my_column_index),
                        match my_column_index {
                            COLUMN_MAX => (ExitBits::Up, COLUMN_MAX),
                            _ => (ExitBits::Right, my_column_index + 1),
                        },
                    ]),
                    Ordering::Equal => *self.map_rng.choose(&[
                        match my_column_index {
                            0 => (ExitBits::Right, 1),
                            // bounce instead of clamp; intended?
                            _ => (ExitBits::Left, my_column_index - 1),
                        },
                        (ExitBits::Up, my_column_index),
                        match my_column_index {
                            COLUMN_MAX => (ExitBits::Left, COLUMN_MAX - 1),
                            // bounce instead of clamp; intended?
                            _ => (ExitBits::Right, my_column_index + 1),
                        },
                    ]),
                    Ordering::Greater => *self.map_rng.choose(&[
                        match my_column_index {
                            0 => (ExitBits::Up, 0),
                            _ => (ExitBits::Left, my_column_index - 1),
                        },
                        (ExitBits::Up, my_column_index),
                    ]),
                };
            }
        }
        (exit, next_column_index)
    }

    fn prevent_crossed_paths(
        &self,
        my_row_index: RowIndex,
        my_column_index: ColumnIndex,
        exit: ExitBits,
        next_column_index: ColumnIndex,
    ) -> (ExitBits, ColumnIndex) {
        match exit {
            ExitBits::Left => {
                if self
                    .node_grid
                    .has_exit(my_row_index, my_column_index - 1, ExitBits::Right)
                {
                    (ExitBits::Up, my_column_index)
                } else {
                    (exit, next_column_index)
                }
            }
            ExitBits::Up => (exit, next_column_index),
            ExitBits::Right => {
                if self
                    .node_grid
                    .has_exit(my_row_index, my_column_index + 1, ExitBits::Left)
                {
                    (ExitBits::Up, my_column_index)
                } else {
                    (exit, next_column_index)
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
        let mut map_rng = StsRandom::from(2);
        let node_grid = GraphBuilder::new(&mut map_rng).build();
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

        let mut map_rng = StsRandom::from(3);
        let node_grid = GraphBuilder::new(&mut map_rng).build();
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
            .map(|i| GraphBuilder::new(&mut StsRandom::from(i)).build())
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
