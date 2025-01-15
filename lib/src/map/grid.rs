/// The `NodeGrid` and `NodeBuilderGrid` structs represent a grid of nodes used in a map simulation.
///
/// # Layout
/// The grid is a 2D array with dimensions `ROW_COUNT` x `COLUMN_COUNT`. Each cell in the grid can
/// contain an `Option<Node>` or `Option<NodeBuilder>`, representing the presence or absence of a
/// node at that position.
///
/// # NodeBuilder Elements
/// - `NodeBuilder` elements in the grid can contain room type and available exit directions.
/// - `NodeBuilder` elements also maintain backward ("parent") edges in a way that is inefficient
///   but true to the original game's implementation.
/// - The `NodeBuilder` can be used to construct a `Node` which is a finalized version of the node.
///
/// # Connections
/// - Nodes in the grid are connected based on their exit bits. The exit bits determine the
///   direction of connections between nodes (e.g., left, up, right).
/// - The `NodeBuilderGrid` provides methods to manipulate the grid, such as adding parents,
///   exits, and setting rooms.
/// - The `NodeGrid` provides methods to display the grid and check for connections between nodes.
///
/// # Display
/// - The `fmt::Display` implementations for `NodeGrid` and `NodeBuilderGrid` provide a visual
///   representation of the grid, showing the connections between nodes and the types of rooms.
/// - The display format for `NodeGrid` in particular nearly matches the
///   C++ reference implementation.
use std::fmt;

use super::exit::ExitBits;
use super::node::{Node, NodeBuilder};
use super::room::Room;
use super::{COLUMN_COUNT, COLUMN_MAX, ROW_COUNT};

pub struct NodeGrid {
    grid: [[Option<Node>; COLUMN_COUNT]; ROW_COUNT],
}

#[derive(Default)]
pub struct NodeBuilderGrid {
    grid: [[Option<NodeBuilder>; COLUMN_COUNT]; ROW_COUNT],
}

// Convenience trait for highlighting the a node grid for its string representation.
pub trait MapHighlighter {
    fn left(&self, row: u8, col: u8) -> char;
    fn right(&self, row: u8, col: u8) -> char;
}

impl NodeGrid {
    pub fn new(grid: [[Option<Node>; COLUMN_COUNT]; ROW_COUNT]) -> Self {
        Self { grid }
    }

    pub fn get(&self, row: u8, col: u8) -> Option<&Node> {
        self.grid[row as usize][col as usize].as_ref()
    }

    /// Returns a Vec<u8> holding column indices of nodes in the given row that are not empty.
    pub fn nonempty_cols_for_row(&self, row: u8) -> Vec<u8> {
        self.grid[row as usize]
            .iter()
            .enumerate()
            .filter_map(|(col, maybe_node)| maybe_node.as_ref().map(|_| col as u8))
            .collect()
    }

    /// Displays a string representation of the grid, with rooms properly arranged and exits
    /// (edges) connecting them. The `highlight_fn` parameter is a function that takes the row
    /// and column indices of a node and returns a tuple of characters to highlight the node.
    pub fn to_string_with_highlighter<T: MapHighlighter>(&self, highlighter: T) -> String {
        let mut result = String::new();
        for (row, row_slice) in self.grid.iter().enumerate().rev() {
            for maybe_node in row_slice.iter() {
                match maybe_node {
                    Some(node) => {
                        result.push_str(node.exit_bits.to_string().as_str());
                    }
                    None => {
                        result.push_str("   ");
                    }
                }
            }
            result.push('\n');
            for (col, maybe_node) in row_slice.iter().enumerate() {
                match maybe_node {
                    Some(node) => {
                        result.push(highlighter.left(row as u8, col as u8));
                        result.push_str(node.room.to_string().as_str());
                        result.push(highlighter.right(row as u8, col as u8));
                    }
                    None => {
                        result.push_str("   ");
                    }
                }
            }
            // Avoid adding an extra newline after the last row
            if row_slice.as_ptr() != self.grid[0].as_ptr() {
                result.push('\n');
            }
        }
        result
    }
}

struct DummyHighlighter;

impl MapHighlighter for DummyHighlighter {
    fn left(&self, _: u8, _: u8) -> char {
        ' '
    }

    fn right(&self, _: u8, _: u8) -> char {
        ' '
    }
}

impl fmt::Display for NodeGrid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string_with_highlighter(DummyHighlighter))
    }
}

impl NodeBuilderGrid {
    pub fn remove(&mut self, row: usize, col: usize) -> Option<NodeBuilder> {
        self.grid[row][col].take()
    }

    /// Returns a Vec<usize> holding column indices of nodes in the given row that are not empty.
    pub fn nonempty_cols_for_row(&self, row: usize) -> Vec<usize> {
        self.grid[row]
            .iter()
            .enumerate()
            .filter_map(|(col, maybe_node)| maybe_node.as_ref().map(|_| col))
            .collect()
    }

    pub fn set_all_rooms_in_row(&mut self, row: usize, room: Room) {
        for maybe_node in self.grid[row].iter_mut() {
            if let Some(node) = maybe_node.as_mut() {
                node.room = Some(room);
            }
        }
    }

    pub fn set_room(&mut self, row: usize, col: usize, room: Room) {
        self.grid[row][col].get_or_insert_default().room = Some(room);
    }

    /// Records the column of the parent node for the given node.
    pub fn record_parent_col(&mut self, row: usize, col: usize, parent_col: usize) {
        self.grid[row][col]
            .get_or_insert_default()
            .record_parent_col(parent_col);
    }

    pub fn add_exit(&mut self, row: usize, col: usize, exit: ExitBits) {
        self.grid[row][col].get_or_insert_default().add_exit(exit);
    }

    pub fn has_exit(&self, row: usize, col: usize, exit: ExitBits) -> bool {
        self.grid[row][col]
            .as_ref()
            .map_or(false, |node| node.has_exit(exit))
    }

    /// Returns true iff the node at the given row and column has a parent node with the given room.
    pub fn has_parent_room_of(&self, row: usize, col: usize, room: Room) -> bool {
        row > 0
            && (self
                .maybe_down_left_parent(row, col)
                .map_or(false, |node| node.room.map(|r| r == room).unwrap_or(false))
                || self
                    .maybe_down_parent(row, col)
                    .map_or(false, |node| node.room.map(|r| r == room).unwrap_or(false))
                || self
                    .maybe_down_right_parent(row, col)
                    .map_or(false, |node| node.room.map(|r| r == room).unwrap_or(false)))
    }

    /// Returns the node below and to the left, provided it connects to node at the given
    /// coordinates.
    fn maybe_down_left_parent(&self, row: usize, col: usize) -> Option<&NodeBuilder> {
        if col > 0 {
            self.grid[row - 1][col - 1]
                .as_ref()
                .filter(|node| node.has_exit(ExitBits::Right))
        } else {
            None
        }
    }

    /// Returns the node below the given coordinates, provided it connects to the node at the given
    /// coordinates.
    fn maybe_down_parent(&self, row: usize, col: usize) -> Option<&NodeBuilder> {
        self.grid[row - 1][col]
            .as_ref()
            .filter(|node| node.has_exit(ExitBits::Up))
    }

    /// Returns the node below and to the right, provided it connects to the node at the given
    /// coordinates.
    fn maybe_down_right_parent(&self, row: usize, col: usize) -> Option<&NodeBuilder> {
        if col < COLUMN_MAX {
            self.grid[row - 1][col + 1]
                .as_ref()
                .filter(|node| node.has_exit(ExitBits::Left))
        } else {
            None
        }
    }

    /// Returns true iff the node at the given row and column shares a parent with another node
    /// that has the given room type.
    pub fn has_left_sibling_room_of(&self, row: usize, col: usize, room: Room) -> bool {
        self.maybe_down_left_parent(row, col)
            .map_or(false, |_| self.has_child_room_of(row - 1, col - 1, room))
            || self
                .maybe_down_parent(row, col)
                .map_or(false, |_| self.has_child_room_of(row - 1, col, room))
    }

    /// Returns true iff the node at the given row and column has a child node with the given
    /// room type.
    fn has_child_room_of(&self, row: usize, col: usize, room: Room) -> bool {
        [ExitBits::Left, ExitBits::Up, ExitBits::Right]
            .iter()
            .any(|&exit| {
                self.has_exit(row, col, exit)
                    && self.grid[row + 1][match exit {
                        ExitBits::Left => col - 1,
                        ExitBits::Up => col,
                        ExitBits::Right => col + 1,
                        _ => unreachable!(),
                    }]
                    .as_ref()
                    .map_or(false, |node| node.room.map(|r| r == room).unwrap_or(false))
            })
    }

    /// Returns an iterator over the recorded parent columns for the given node, unordered and
    /// possibly with duplicates.
    pub fn recorded_parent_cols_iter(
        &self,
        row: usize,
        col: usize,
    ) -> impl Iterator<Item = &usize> {
        self.grid[row][col]
            .as_ref()
            .map(|node| node.recorded_parent_cols_iter())
            .into_iter()
            .flatten()
    }

    /// Attempts to determine if the node at the given row and column shares a parent with another
    /// node in the same row, but fails to do so in most cases.
    pub fn buggy_implementation_of_shares_parent_with(
        &self,
        row: usize,
        my_col: usize,
        other_col: usize,
    ) -> bool {
        if let (Some(my_node), Some(other_node)) = (
            &self.grid[row][my_col].as_ref(),
            &self.grid[row][other_col].as_ref(),
        ) {
            // "other_col >= row" is almost certainly a bug in the original code; it's
            // probably supposed to be "other_col >= my_col". This causes a lot of small
            // cycles to be missed because the wrong comparisons are being performed.
            if other_col >= row {
                my_node.rightmost_recorded_parent_col() == other_node.leftmost_recorded_parent_col()
            } else {
                other_node.rightmost_recorded_parent_col() == my_node.leftmost_recorded_parent_col()
            }
        } else {
            false
        }
    }

    /// Counts the number of nodes awaiting room assignment.
    pub fn unassigned_room_count(&self) -> usize {
        self.grid
            .iter()
            .flatten()
            .filter(|maybe_node| {
                maybe_node
                    .as_ref()
                    .map(|node| node.room.is_none())
                    .unwrap_or(false)
            })
            .count()
    }

    /// Incorrectly estimates total number of non-empty nodes in the grid.
    pub fn room_almost_total(&self) -> usize {
        self.grid
            .iter()
            .enumerate()
            .filter(|&(row, _)| row != ROW_COUNT - 2) // Reference code calls this "restRowBug"
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

    struct SimpleHighlighter {
        pub row: u8,
        pub col: u8,
    }

    impl MapHighlighter for SimpleHighlighter {
        fn left(&self, row: u8, col: u8) -> char {
            if row == self.row && col == self.col {
                '['
            } else {
                ' '
            }
        }

        fn right(&self, row: u8, col: u8) -> char {
            if row == self.row && col == self.col {
                ']'
            } else {
                ' '
            }
        }
    }

    pub static MAP_0SLAYTHESPIRE: Lazy<NodeGrid> = Lazy::new(|| NodeGrid {
        grid: [
            [
                Some(Node::new(Room::Monster, ExitBits::Up)),
                None,
                Some(Node::new(Room::Monster, ExitBits::Up)),
                None,
                None,
                Some(Node::new(Room::Monster, ExitBits::Up)),
                None,
            ],
            [
                Some(Node::new(Room::Monster, ExitBits::Up | ExitBits::Right)),
                None,
                Some(Node::new(Room::Monster, ExitBits::Up | ExitBits::Right)),
                None,
                None,
                Some(Node::new(Room::Monster, ExitBits::Left | ExitBits::Up)),
                None,
            ],
            [
                Some(Node::new(Room::Event, ExitBits::Up)),
                Some(Node::new(Room::Monster, ExitBits::Right)),
                Some(Node::new(Room::Monster, ExitBits::Up)),
                Some(Node::new(Room::Shop, ExitBits::Up)),
                Some(Node::new(Room::Event, ExitBits::Left)),
                Some(Node::new(Room::Monster, ExitBits::Left)),
                None,
            ],
            [
                Some(Node::new(Room::Event, ExitBits::Up)),
                None,
                Some(Node::new(Room::Monster, ExitBits::Up | ExitBits::Right)),
                Some(Node::new(Room::Event, ExitBits::Up)),
                Some(Node::new(Room::Monster, ExitBits::Up)),
                None,
                None,
            ],
            [
                Some(Node::new(Room::Monster, ExitBits::Up)),
                None,
                Some(Node::new(Room::Event, ExitBits::Left)),
                Some(Node::new(Room::Monster, ExitBits::Left | ExitBits::Right)),
                Some(Node::new(Room::Event, ExitBits::Up)),
                None,
                None,
            ],
            [
                Some(Node::new(Room::Campfire, ExitBits::Up)),
                Some(Node::new(Room::Elite, ExitBits::Up)),
                Some(Node::new(Room::Campfire, ExitBits::Left)),
                None,
                Some(Node::new(Room::Monster, ExitBits::Left | ExitBits::Right)),
                None,
                None,
            ],
            [
                Some(Node::new(Room::Monster, ExitBits::Up)),
                Some(Node::new(Room::Monster, ExitBits::Up | ExitBits::Right)),
                None,
                Some(Node::new(Room::Event, ExitBits::Left)),
                None,
                Some(Node::new(Room::Monster, ExitBits::Right)),
                None,
            ],
            [
                Some(Node::new(Room::Monster, ExitBits::Up)),
                Some(Node::new(Room::Shop, ExitBits::Up)),
                Some(Node::new(Room::Monster, ExitBits::Left | ExitBits::Up)),
                None,
                None,
                None,
                Some(Node::new(Room::Monster, ExitBits::Left)),
            ],
            [
                Some(Node::new(Room::Treasure, ExitBits::Up)),
                Some(Node::new(Room::Treasure, ExitBits::Left | ExitBits::Right)),
                Some(Node::new(Room::Treasure, ExitBits::Right)),
                None,
                None,
                Some(Node::new(Room::Treasure, ExitBits::Left)),
                None,
            ],
            [
                Some(Node::new(Room::Campfire, ExitBits::Up | ExitBits::Right)),
                None,
                Some(Node::new(Room::Monster, ExitBits::Up | ExitBits::Right)),
                Some(Node::new(Room::Event, ExitBits::Up)),
                Some(Node::new(Room::Campfire, ExitBits::Up)),
                None,
                None,
            ],
            [
                Some(Node::new(Room::Event, ExitBits::Up)),
                Some(Node::new(Room::Monster, ExitBits::Up)),
                Some(Node::new(Room::Campfire, ExitBits::Left)),
                Some(Node::new(Room::Event, ExitBits::Left | ExitBits::Right)),
                Some(Node::new(Room::Monster, ExitBits::Right)),
                None,
                None,
            ],
            [
                Some(Node::new(Room::BurningElite1, ExitBits::Right)),
                Some(Node::new(Room::Monster, ExitBits::Right)),
                Some(Node::new(Room::Event, ExitBits::Up)),
                None,
                Some(Node::new(Room::Monster, ExitBits::Left)),
                Some(Node::new(Room::Event, ExitBits::Up)),
                None,
            ],
            [
                None,
                Some(Node::new(Room::Event, ExitBits::Left)),
                Some(Node::new(Room::Elite, ExitBits::Left | ExitBits::Up)),
                Some(Node::new(Room::Campfire, ExitBits::Up)),
                None,
                Some(Node::new(Room::Campfire, ExitBits::Right)),
                None,
            ],
            [
                Some(Node::new(Room::Elite, ExitBits::Up)),
                Some(Node::new(Room::Monster, ExitBits::Left)),
                Some(Node::new(Room::Shop, ExitBits::Up)),
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
                Some(Node::new(Room::Monster, ExitBits::Up)),
                None,
                Some(Node::new(Room::Event, ExitBits::Left | ExitBits::Right)),
                Some(Node::new(Room::Shop, ExitBits::Up)),
                Some(Node::new(Room::Shop, ExitBits::Left | ExitBits::Up)),
            ],
            [
                None,
                None,
                Some(Node::new(Room::Event, ExitBits::Left)),
                Some(Node::new(Room::Monster, ExitBits::Up)),
                None,
                Some(Node::new(Room::Event, ExitBits::Left | ExitBits::Up)),
                Some(Node::new(Room::Monster, ExitBits::Up)),
            ],
            [
                None,
                Some(Node::new(Room::Event, ExitBits::Up)),
                None,
                Some(Node::new(Room::Monster, ExitBits::Up)),
                Some(Node::new(Room::Monster, ExitBits::Left | ExitBits::Up)),
                Some(Node::new(Room::Event, ExitBits::Left)),
                Some(Node::new(Room::Monster, ExitBits::Left)),
            ],
            [
                None,
                Some(Node::new(Room::Monster, ExitBits::Up)),
                None,
                Some(Node::new(Room::Monster, ExitBits::Left | ExitBits::Right)),
                Some(Node::new(Room::Event, ExitBits::Up)),
                Some(Node::new(Room::Monster, ExitBits::Left)),
                None,
            ],
            [
                None,
                Some(Node::new(Room::Campfire, ExitBits::Up)),
                Some(Node::new(Room::Elite, ExitBits::Right)),
                None,
                Some(Node::new(
                    Room::Campfire,
                    ExitBits::Left | ExitBits::Up | ExitBits::Right,
                )),
                None,
                None,
            ],
            [
                None,
                Some(Node::new(Room::Elite, ExitBits::Right)),
                None,
                Some(Node::new(Room::Monster, ExitBits::Left)),
                Some(Node::new(Room::Elite, ExitBits::Left | ExitBits::Up)),
                Some(Node::new(Room::Event, ExitBits::Up)),
                None,
            ],
            [
                None,
                None,
                Some(Node::new(Room::Campfire, ExitBits::Up)),
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
                    ExitBits::Left | ExitBits::Up | ExitBits::Right,
                )),
                Some(Node::new(Room::Treasure, ExitBits::Up)),
                None,
                None,
                Some(Node::new(Room::Treasure, ExitBits::Up)),
            ],
            [
                None,
                Some(Node::new(Room::Monster, ExitBits::Up)),
                Some(Node::new(Room::Campfire, ExitBits::Up)),
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
                Some(Node::new(Room::Event, ExitBits::Up)),
                Some(Node::new(Room::BurningElite4, ExitBits::Up)),
                None,
            ],
            [
                Some(Node::new(Room::Monster, ExitBits::Right)),
                None,
                Some(Node::new(Room::Event, ExitBits::Right)),
                Some(Node::new(Room::Monster, ExitBits::Up)),
                Some(Node::new(Room::Event, ExitBits::Left)),
                Some(Node::new(Room::Monster, ExitBits::Up)),
                None,
            ],
            [
                None,
                Some(Node::new(Room::Monster, ExitBits::Up)),
                None,
                Some(Node::new(
                    Room::Monster,
                    ExitBits::Left | ExitBits::Up | ExitBits::Right,
                )),
                None,
                Some(Node::new(Room::Monster, ExitBits::Right)),
                None,
            ],
            [
                None,
                Some(Node::new(Room::Event, ExitBits::Up)),
                Some(Node::new(Room::Shop, ExitBits::Right)),
                Some(Node::new(Room::Monster, ExitBits::Right)),
                Some(Node::new(Room::Monster, ExitBits::Up | ExitBits::Right)),
                None,
                Some(Node::new(Room::Monster, ExitBits::Left)),
            ],
            [
                None,
                Some(Node::new(Room::Campfire, ExitBits::Right)),
                None,
                Some(Node::new(Room::Campfire, ExitBits::Up)),
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

        assert_eq!(
            MAP_0SLAYTHESPIRE.to_string_with_highlighter(SimpleHighlighter { row: 4, col: 3 }),
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
                r" M     ? [M] ?       ",
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
}