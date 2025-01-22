use std::fmt;

use crate::components::map::{ExitBits, Node, Room};
use crate::types::ColumnIndex;

use super::PATH_DENSITY;

#[derive(Debug)]
pub struct NodeBuilder {
    pub room: Option<Room>,
    exit_bits: ExitBits,
    // The columns of the parent nodes that this node is connected to, unsorted. May contain
    // duplicates. Needed only to replicate quirks of the game's path generation algorithm.
    recorded_parent_columns: Vec<ColumnIndex>,
}

impl NodeBuilder {
    pub fn has_exit(&self, exit: ExitBits) -> bool {
        self.exit_bits.contains(exit)
    }

    pub fn add_exit(&mut self, exit: ExitBits) -> &mut Self {
        self.exit_bits |= exit;
        self
    }

    pub fn record_parent_column(&mut self, parent_column_index: ColumnIndex) -> &mut Self {
        self.recorded_parent_columns.push(parent_column_index);
        self
    }

    pub fn recorded_parent_columns_iter(&self) -> impl Iterator<Item = &ColumnIndex> {
        self.recorded_parent_columns.iter()
    }

    pub fn leftmost_recorded_parent_column(&self) -> Option<ColumnIndex> {
        self.recorded_parent_columns_iter().min().copied()
    }

    pub fn rightmost_recorded_parent_column(&self) -> Option<ColumnIndex> {
        self.recorded_parent_columns_iter().max().copied()
    }

    pub fn build(&self) -> Node {
        Node::new(self.room.unwrap_or(Room::Monster), self.exit_bits)
    }
}

impl fmt::Display for NodeBuilder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let parent_columns = self
            .recorded_parent_columns
            .iter()
            .map(|&column_index| column_index.to_string())
            .chain(
                std::iter::repeat("-".to_string())
                    .take(PATH_DENSITY - self.recorded_parent_columns.len()),
            );
        write!(f, "{}", parent_columns.collect::<String>())
    }
}

impl Default for NodeBuilder {
    fn default() -> Self {
        Self {
            room: None,
            exit_bits: ExitBits::empty(),
            recorded_parent_columns: Vec::with_capacity(PATH_DENSITY),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // Helper methods for tests elsewhere in the codebase
    impl NodeBuilder {
        pub fn exit_bits(&self) -> ExitBits {
            self.exit_bits
        }
    }
}
