use std::fmt;

use super::exit::ExitBits;
use super::node::{Node, NodeBuilder};
use super::room::Room;
use super::{COLUMN_COUNT, ROW_COUNT};

pub struct NodeGrid {
    grid: [[Option<Node>; COLUMN_COUNT]; ROW_COUNT],
}

pub struct NodeBuilderGrid {
    grid: [[Option<NodeBuilder>; COLUMN_COUNT]; ROW_COUNT],
}

impl NodeGrid {
    pub fn new(grid: [[Option<Node>; COLUMN_COUNT]; ROW_COUNT]) -> Self {
        Self { grid }
    }
}

impl fmt::Display for NodeGrid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.grid.iter().rev() {
            for maybe_node in row.iter() {
                match maybe_node {
                    Some(node) => {
                        write!(f, "{}", node.exit_bits())?;
                    }
                    None => {
                        write!(f, "   ")?;
                    }
                }
            }
            writeln!(f)?;
            for maybe_node in row.iter() {
                match maybe_node {
                    Some(node) => {
                        write!(f, "{}", node.room())?;
                    }
                    None => {
                        write!(f, "   ")?;
                    }
                }
            }
            // Avoid adding an extra newline after the last row
            if row.as_ptr() != self.grid[0].as_ptr() {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

impl NodeBuilderGrid {
    pub fn new() -> Self {
        Self {
            grid: Default::default(),
        }
    }

    pub fn remove(&mut self, row: usize, col: usize) -> Option<NodeBuilder> {
        self.grid[row][col].take()
    }

    pub fn set_room(&mut self, row: usize, col: usize, room: Room) {
        self.grid[row][col] = Some(
            self.grid[row][col]
                .take()
                .expect("set_room called on an empty node")
                .set_room(room),
        );
    }

    pub fn add_entrance(&mut self, row: usize, col: usize, entrance_col: usize) {
        self.grid[row][col] = Some(
            self.grid[row][col]
                .take()
                .unwrap_or_default()
                .add_entrance_col(entrance_col),
        );
    }

    pub fn add_exit(&mut self, row: usize, col: usize, exit: ExitBits) {
        self.grid[row][col] = Some(
            self.grid[row][col]
                .take()
                .unwrap_or_default()
                .add_exit(exit),
        );
    }

    pub fn has_exit(&self, row: usize, col: usize, exit: ExitBits) -> bool {
        self.grid[row][col]
            .as_ref()
            .map_or(false, |node| node.has_exit(exit))
    }

    pub fn entrance_cols(&self, row: usize, col: usize) -> impl Iterator<Item = &usize> {
        self.grid[row][col]
            .as_ref()
            .map(|node| node.entrance_col_iter())
            .into_iter()
            .flatten()
    }

    pub fn entrances_collide(&self, row: usize, my_col: usize, other_col: usize) -> bool {
        if let (Some(my_node), Some(other_node)) = (
            &self.grid[row][my_col].as_ref(),
            &self.grid[row][other_col].as_ref(),
        ) {
            // "other_col >= row" is almost certainly a bug in the original code; it's
            // probably supposed to be "other_col >= my_col". This causes a lot of small
            // cycles to be missed because the wrong comparisons are being performed.
            if other_col >= row {
                my_node.rightmost_entrance_col() == other_node.leftmost_entrance_col()
            } else {
                other_node.rightmost_entrance_col() == my_node.leftmost_entrance_col()
            }
        } else {
            false
        }
    }

    pub fn unassigned_room_count(&self) -> usize {
        self.grid
            .iter()
            .flatten()
            .filter(|maybe_node| {
                maybe_node
                    .as_ref()
                    .map(|node| node.room().is_none())
                    .unwrap_or(false)
            })
            .count()
    }

    pub fn room_total(&self) -> usize {
        self.grid
            .iter()
            .enumerate()
            .filter(|&(row, _)| row != ROW_COUNT - 2) // Original code calls this "restRowBug"
            .flat_map(|(_, row)| row)
            .filter(|maybe_node| maybe_node.is_some())
            .count()
    }
}

impl From<NodeBuilderGrid> for NodeGrid {
    fn from(mut builder: NodeBuilderGrid) -> Self {
        NodeGrid::new(
            builder
                .grid
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
                .expect("Builder should have built the correct number of rows"),
        )
    }
}

impl fmt::Display for NodeBuilderGrid {
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

#[cfg(test)]
mod test {
    use base64::engine::general_purpose::STANDARD;
    use base64::Engine;
    use once_cell::sync::Lazy;

    use super::*;

    impl NodeBuilderGrid {
        pub fn exit_bits_as_vec(&self) -> [[u8; COLUMN_COUNT]; ROW_COUNT - 1] {
            let mut exits = [[0; COLUMN_COUNT]; ROW_COUNT - 1];
            for (row, row_nodes) in self.grid.iter().enumerate().take(ROW_COUNT - 1) {
                for (col, maybe_node) in row_nodes.iter().enumerate() {
                    if let Some(node) = maybe_node {
                        exits[row][col] = node.exit_bits().bits();
                    }
                }
            }
            exits
        }

        pub fn exit_bits_as_base64(&self) -> String {
            STANDARD.encode(
                self.exit_bits_as_vec()
                    .as_flattened()
                    .chunks(21)
                    .map(|chunk| chunk.iter().fold(0, |acc, &exit| (acc << 3) | exit as u64))
                    .flat_map(|val64| val64.to_be_bytes())
                    .collect::<Vec<u8>>(),
            )
        }
    }

    pub static MAP_0SLAYTHESPIRE: Lazy<NodeGrid> = Lazy::new(|| NodeGrid {
        grid: [
            [
                Some(Node::new(Room::Monster, ExitBits::Straight)),
                None,
                Some(Node::new(Room::Monster, ExitBits::Straight)),
                None,
                None,
                Some(Node::new(Room::Monster, ExitBits::Straight)),
                None,
            ],
            [
                Some(Node::new(
                    Room::Monster,
                    ExitBits::Straight | ExitBits::Right,
                )),
                None,
                Some(Node::new(
                    Room::Monster,
                    ExitBits::Straight | ExitBits::Right,
                )),
                None,
                None,
                Some(Node::new(
                    Room::Monster,
                    ExitBits::Left | ExitBits::Straight,
                )),
                None,
            ],
            [
                Some(Node::new(Room::Event, ExitBits::Straight)),
                Some(Node::new(Room::Monster, ExitBits::Right)),
                Some(Node::new(Room::Monster, ExitBits::Straight)),
                Some(Node::new(Room::Shop, ExitBits::Straight)),
                Some(Node::new(Room::Event, ExitBits::Left)),
                Some(Node::new(Room::Monster, ExitBits::Left)),
                None,
            ],
            [
                Some(Node::new(Room::Event, ExitBits::Straight)),
                None,
                Some(Node::new(
                    Room::Monster,
                    ExitBits::Straight | ExitBits::Right,
                )),
                Some(Node::new(Room::Event, ExitBits::Straight)),
                Some(Node::new(Room::Monster, ExitBits::Straight)),
                None,
                None,
            ],
            [
                Some(Node::new(Room::Monster, ExitBits::Straight)),
                None,
                Some(Node::new(Room::Event, ExitBits::Left)),
                Some(Node::new(Room::Monster, ExitBits::Left | ExitBits::Right)),
                Some(Node::new(Room::Event, ExitBits::Straight)),
                None,
                None,
            ],
            [
                Some(Node::new(Room::Campfire, ExitBits::Straight)),
                Some(Node::new(Room::Elite, ExitBits::Straight)),
                Some(Node::new(Room::Campfire, ExitBits::Left)),
                None,
                Some(Node::new(Room::Monster, ExitBits::Left | ExitBits::Right)),
                None,
                None,
            ],
            [
                Some(Node::new(Room::Monster, ExitBits::Straight)),
                Some(Node::new(
                    Room::Monster,
                    ExitBits::Straight | ExitBits::Right,
                )),
                None,
                Some(Node::new(Room::Event, ExitBits::Left)),
                None,
                Some(Node::new(Room::Monster, ExitBits::Right)),
                None,
            ],
            [
                Some(Node::new(Room::Monster, ExitBits::Straight)),
                Some(Node::new(Room::Shop, ExitBits::Straight)),
                Some(Node::new(
                    Room::Monster,
                    ExitBits::Left | ExitBits::Straight,
                )),
                None,
                None,
                None,
                Some(Node::new(Room::Monster, ExitBits::Left)),
            ],
            [
                Some(Node::new(Room::Treasure, ExitBits::Straight)),
                Some(Node::new(Room::Treasure, ExitBits::Left | ExitBits::Right)),
                Some(Node::new(Room::Treasure, ExitBits::Right)),
                None,
                None,
                Some(Node::new(Room::Treasure, ExitBits::Left)),
                None,
            ],
            [
                Some(Node::new(
                    Room::Campfire,
                    ExitBits::Straight | ExitBits::Right,
                )),
                None,
                Some(Node::new(
                    Room::Monster,
                    ExitBits::Straight | ExitBits::Right,
                )),
                Some(Node::new(Room::Event, ExitBits::Straight)),
                Some(Node::new(Room::Campfire, ExitBits::Straight)),
                None,
                None,
            ],
            [
                Some(Node::new(Room::Event, ExitBits::Straight)),
                Some(Node::new(Room::Monster, ExitBits::Straight)),
                Some(Node::new(Room::Campfire, ExitBits::Left)),
                Some(Node::new(Room::Event, ExitBits::Left | ExitBits::Right)),
                Some(Node::new(Room::Monster, ExitBits::Right)),
                None,
                None,
            ],
            [
                Some(Node::new(Room::BurningElite1, ExitBits::Right)),
                Some(Node::new(Room::Monster, ExitBits::Right)),
                Some(Node::new(Room::Event, ExitBits::Straight)),
                None,
                Some(Node::new(Room::Monster, ExitBits::Left)),
                Some(Node::new(Room::Event, ExitBits::Straight)),
                None,
            ],
            [
                None,
                Some(Node::new(Room::Event, ExitBits::Left)),
                Some(Node::new(Room::Elite, ExitBits::Left | ExitBits::Straight)),
                Some(Node::new(Room::Campfire, ExitBits::Straight)),
                None,
                Some(Node::new(Room::Campfire, ExitBits::Right)),
                None,
            ],
            [
                Some(Node::new(Room::Elite, ExitBits::Straight)),
                Some(Node::new(Room::Monster, ExitBits::Left)),
                Some(Node::new(Room::Shop, ExitBits::Straight)),
                Some(Node::new(Room::Monster, ExitBits::Right)),
                None,
                None,
                Some(Node::new(Room::Elite, ExitBits::Left)),
            ],
            [
                Some(Node::new(Room::Campfire, ExitBits::Right)),
                None,
                Some(Node::new(Room::Campfire, ExitBits::Right)),
                None,
                Some(Node::new(Room::Campfire, ExitBits::Left)),
                Some(Node::new(Room::Campfire, ExitBits::Left)),
                None,
            ],
        ],
    });

    static MAP_1SLAYTHESPIRE: Lazy<NodeGrid> = Lazy::new(|| NodeGrid {
        grid: [
            [
                None,
                Some(Node::new(Room::Monster, ExitBits::Right)),
                None,
                Some(Node::new(Room::Monster, ExitBits::Right)),
                Some(Node::new(Room::Monster, ExitBits::Right)),
                Some(Node::new(Room::Monster, ExitBits::Right)),
                None,
            ],
            [
                None,
                None,
                Some(Node::new(Room::Monster, ExitBits::Straight)),
                None,
                Some(Node::new(Room::Event, ExitBits::Left | ExitBits::Right)),
                Some(Node::new(Room::Shop, ExitBits::Straight)),
                Some(Node::new(Room::Shop, ExitBits::Left | ExitBits::Straight)),
            ],
            [
                None,
                None,
                Some(Node::new(Room::Event, ExitBits::Left)),
                Some(Node::new(Room::Monster, ExitBits::Straight)),
                None,
                Some(Node::new(Room::Event, ExitBits::Left | ExitBits::Straight)),
                Some(Node::new(Room::Monster, ExitBits::Straight)),
            ],
            [
                None,
                Some(Node::new(Room::Event, ExitBits::Straight)),
                None,
                Some(Node::new(Room::Monster, ExitBits::Straight)),
                Some(Node::new(
                    Room::Monster,
                    ExitBits::Left | ExitBits::Straight,
                )),
                Some(Node::new(Room::Event, ExitBits::Left)),
                Some(Node::new(Room::Monster, ExitBits::Left)),
            ],
            [
                None,
                Some(Node::new(Room::Monster, ExitBits::Straight)),
                None,
                Some(Node::new(Room::Monster, ExitBits::Left | ExitBits::Right)),
                Some(Node::new(Room::Event, ExitBits::Straight)),
                Some(Node::new(Room::Monster, ExitBits::Left)),
                None,
            ],
            [
                None,
                Some(Node::new(Room::Campfire, ExitBits::Straight)),
                Some(Node::new(Room::Elite, ExitBits::Right)),
                None,
                Some(Node::new(
                    Room::Campfire,
                    ExitBits::Left | ExitBits::Straight | ExitBits::Right,
                )),
                None,
                None,
            ],
            [
                None,
                Some(Node::new(Room::Elite, ExitBits::Right)),
                None,
                Some(Node::new(Room::Monster, ExitBits::Left)),
                Some(Node::new(Room::Elite, ExitBits::Left | ExitBits::Straight)),
                Some(Node::new(Room::Event, ExitBits::Straight)),
                None,
            ],
            [
                None,
                None,
                Some(Node::new(Room::Campfire, ExitBits::Straight)),
                Some(Node::new(Room::Monster, ExitBits::Left)),
                Some(Node::new(Room::Campfire, ExitBits::Left)),
                Some(Node::new(Room::Monster, ExitBits::Right)),
                None,
            ],
            [
                None,
                None,
                Some(Node::new(
                    Room::Treasure,
                    ExitBits::Left | ExitBits::Straight | ExitBits::Right,
                )),
                Some(Node::new(Room::Treasure, ExitBits::Straight)),
                None,
                None,
                Some(Node::new(Room::Treasure, ExitBits::Straight)),
            ],
            [
                None,
                Some(Node::new(Room::Monster, ExitBits::Straight)),
                Some(Node::new(Room::Campfire, ExitBits::Straight)),
                Some(Node::new(Room::Event, ExitBits::Left | ExitBits::Right)),
                None,
                None,
                Some(Node::new(Room::Monster, ExitBits::Left)),
            ],
            [
                None,
                Some(Node::new(Room::Monster, ExitBits::Left | ExitBits::Right)),
                Some(Node::new(Room::Monster, ExitBits::Right)),
                None,
                Some(Node::new(Room::Event, ExitBits::Straight)),
                Some(Node::new(Room::BurningElite4, ExitBits::Straight)),
                None,
            ],
            [
                Some(Node::new(Room::Monster, ExitBits::Right)),
                None,
                Some(Node::new(Room::Event, ExitBits::Right)),
                Some(Node::new(Room::Monster, ExitBits::Straight)),
                Some(Node::new(Room::Event, ExitBits::Left)),
                Some(Node::new(Room::Monster, ExitBits::Straight)),
                None,
            ],
            [
                None,
                Some(Node::new(Room::Monster, ExitBits::Straight)),
                None,
                Some(Node::new(
                    Room::Monster,
                    ExitBits::Left | ExitBits::Straight | ExitBits::Right,
                )),
                None,
                Some(Node::new(Room::Monster, ExitBits::Right)),
                None,
            ],
            [
                None,
                Some(Node::new(Room::Event, ExitBits::Straight)),
                Some(Node::new(Room::Shop, ExitBits::Right)),
                Some(Node::new(Room::Monster, ExitBits::Right)),
                Some(Node::new(
                    Room::Monster,
                    ExitBits::Straight | ExitBits::Right,
                )),
                None,
                Some(Node::new(Room::Monster, ExitBits::Left)),
            ],
            [
                None,
                Some(Node::new(Room::Campfire, ExitBits::Right)),
                None,
                Some(Node::new(Room::Campfire, ExitBits::Straight)),
                Some(Node::new(Room::Campfire, ExitBits::Left)),
                Some(Node::new(Room::Campfire, ExitBits::Left)),
                None,
            ],
        ],
    });

    #[test]
    fn test_map_display() {
        assert_eq!(
            MAP_0SLAYTHESPIRE.to_string(),
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

        assert_eq!(
            MAP_1SLAYTHESPIRE.to_string(),
            [
                r"     /    | \  \     ",
                r"    R     R  R  R    ",
                r"    |   /  / |/   \  ",
                r"    ?  $  M  M     M ",
                r"    |    \|/     /   ",
                r"    M     M     M    ",
                r"  /     / | \   |    ",
                r" M     ?  M  ?  M    ",
                r"   \ /  /    |  |    ",
                r"    M  M     ?  4    ",
                r"    |  | \ /      \  ",
                r"    M  R  ?        M ",
                r"      \|/ |        | ",
                r"       T  T        T ",
                r"       | \  \    /   ",
                r"       R  M  R  M    ",
                r"     /   \  \|  |    ",
                r"    E     M  E  ?    ",
                r"    |   /   \|/      ",
                r"    R  E     R       ",
                r"    |    \ / | \     ",
                r"    M     M  ?  M    ",
                r"    |     | \| \  \  ",
                r"    ?     M  M  ?  M ",
                r"      \   |    \|  | ",
                r"       ?  M     ?  M ",
                r"       |    \ / | \| ",
                r"       M     ?  $  $ ",
                r"     /     /  /  /   ",
                r"    M     M  M  M    ",
            ]
            .join("\n")
        );
    }
}
