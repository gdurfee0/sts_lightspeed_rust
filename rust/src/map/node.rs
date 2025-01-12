use std::fmt;

use crate::map::PATH_DENSITY;

use super::exit::ExitBits;
use super::room::Room;

#[derive(Debug)]
pub struct Node {
    pub room: Room,
    pub exit_bits: ExitBits,
}

#[derive(Debug)]
pub struct NodeBuilder {
    pub room: Option<Room>,
    exit_bits: ExitBits,
    // The columns of the parent nodes that this node is connected to, unsorted. May contain
    // duplicates. Needed only to replicate quirks of the game's path generation algorithm.
    recorded_parent_cols: Vec<usize>,
}

impl Node {
    pub fn new(room: Room, exit_bits: ExitBits) -> Self {
        Self { room, exit_bits }
    }

    pub fn has_exit(&self, exit: ExitBits) -> bool {
        self.exit_bits.contains(exit)
    }
}

impl NodeBuilder {
    pub fn has_exit(&self, exit: ExitBits) -> bool {
        self.exit_bits.contains(exit)
    }

    pub fn add_exit(&mut self, exit: ExitBits) -> &mut Self {
        self.exit_bits |= exit;
        self
    }

    pub fn record_parent_col(&mut self, parent_col: usize) -> &mut Self {
        self.recorded_parent_cols.push(parent_col);
        self
    }

    pub fn recorded_parent_cols_iter(&self) -> impl Iterator<Item = &usize> {
        self.recorded_parent_cols.iter()
    }

    pub fn leftmost_recorded_parent_col(&self) -> Option<usize> {
        self.recorded_parent_cols_iter().min().copied()
    }

    pub fn rightmost_recorded_parent_col(&self) -> Option<usize> {
        self.recorded_parent_cols_iter().max().copied()
    }

    pub fn build(&self) -> Node {
        Node::new(self.room.unwrap_or(Room::Monster), self.exit_bits)
    }
}

impl fmt::Display for NodeBuilder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let parent_cols = self
            .recorded_parent_cols
            .iter()
            .map(|&col| col.to_string())
            .chain(
                std::iter::repeat("-".to_string())
                    .take(PATH_DENSITY - self.recorded_parent_cols.len()),
            );
        write!(f, "{}", parent_cols.collect::<String>())
    }
}

impl Default for NodeBuilder {
    fn default() -> Self {
        Self {
            room: None,
            exit_bits: ExitBits::empty(),
            recorded_parent_cols: Vec::with_capacity(PATH_DENSITY),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    impl NodeBuilder {
        pub fn exit_bits(&self) -> ExitBits {
            self.exit_bits
        }
    }
}
