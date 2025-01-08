use std::fmt;

use crate::map::PATH_DENSITY;

use super::exit::ExitBits;
use super::room::Room;

#[derive(Debug)]
pub struct Node {
    room: Room,
    exit_bits: ExitBits,
}

#[derive(Debug)]
pub struct NodeBuilder {
    room: Option<Room>,
    exit_bits: ExitBits,
    entrance_cols: Vec<usize>,
}

impl Node {
    pub fn new(room: Room, exit_bits: ExitBits) -> Self {
        Self { room, exit_bits }
    }

    pub fn room(&self) -> Room {
        self.room
    }

    pub fn exit_bits(&self) -> ExitBits {
        self.exit_bits
    }
}

impl NodeBuilder {
    pub fn room(&self) -> Option<Room> {
        self.room
    }

    pub fn leftmost_entrance_col(&self) -> Option<usize> {
        self.entrance_cols.iter().min().copied()
    }

    pub fn rightmost_entrance_col(&self) -> Option<usize> {
        self.entrance_cols.iter().max().copied()
    }

    pub fn has_exit(&self, exit: ExitBits) -> bool {
        self.exit_bits.contains(exit)
    }

    pub fn entrance_col_iter(&self) -> impl Iterator<Item = &usize> {
        self.entrance_cols.iter()
    }

    pub fn set_room(mut self, room: Room) -> Self {
        self.room = Some(room);
        self
    }

    pub fn add_exit(mut self, exit: ExitBits) -> Self {
        self.exit_bits |= exit;
        self
    }

    pub fn add_entrance_col(mut self, entrance_col: usize) -> Self {
        self.entrance_cols.push(entrance_col);
        self
    }

    pub fn build(self) -> Node {
        Node::new(self.room.unwrap_or(Room::Monster), self.exit_bits)
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

impl Default for NodeBuilder {
    fn default() -> Self {
        Self {
            room: None,
            exit_bits: ExitBits::empty(),
            entrance_cols: Vec::with_capacity(PATH_DENSITY),
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
