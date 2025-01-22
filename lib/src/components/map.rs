use std::fmt;

use bitflags::bitflags;

use crate::types::{ColumnIndex, RowIndex};

pub const ROW_COUNT: usize = 15;
pub const COLUMN_COUNT: usize = 7;
pub const COLUMN_MAX: crate::types::ColumnIndex = COLUMN_COUNT - 1;

/// Map is a 2D array of nodes representing the currently-visible section of the Spire.
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

#[derive(Debug)]
pub struct Map {
    pub grid: [[Option<Node>; COLUMN_COUNT]; ROW_COUNT],
}

// Convenience trait for highlighting the a node grid for its string representation.
pub trait MapHighlighter {
    fn left(&self, row_index: RowIndex, column_index: ColumnIndex) -> char;
    fn right(&self, row_index: RowIndex, column_index: ColumnIndex) -> char;
}

#[derive(Debug)]
pub struct Node {
    pub room: Room,
    pub exit_bits: ExitBits,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Room {
    Boss,
    BurningElite1,
    BurningElite2,
    BurningElite3,
    BurningElite4,
    Elite,
    Event,
    Monster,
    RestSite,
    Shop,
    Treasure,
}

bitflags! {
    #[derive(Clone, Copy, Debug, Default, PartialEq)]
    pub struct ExitBits: u8 {
        const Left  = 0b100;
        const Up    = 0b010;
        const Right = 0b001;
    }
}

impl Map {
    pub fn new(grid: [[Option<Node>; COLUMN_COUNT]; ROW_COUNT]) -> Self {
        Self { grid }
    }

    pub fn get(&self, row_index: RowIndex, column_index: ColumnIndex) -> Option<&Node> {
        self.grid[row_index][column_index].as_ref()
    }

    /// Returns a Vec<u8> holding column indices of nodes in the given row that are not empty.
    pub fn nonempty_columns_for_row(&self, row_index: RowIndex) -> Vec<ColumnIndex> {
        self.grid[row_index]
            .iter()
            .enumerate()
            .filter_map(|(column_index, maybe_node)| maybe_node.as_ref().map(|_| column_index))
            .collect()
    }

    /// Displays a string representation of the grid, with rooms properly arranged and exits
    /// (edges) connecting them. The `highlighter` parameter is a function that takes the row
    /// and column indices of a node and returns a pair of characters to highlight the node.
    pub fn to_string_with_highlighter<T: MapHighlighter>(&self, highlighter: T) -> String {
        let mut result = String::new();
        for (row_index, row_slice) in self.grid.iter().enumerate().rev() {
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
            for (column_index, maybe_node) in row_slice.iter().enumerate() {
                match maybe_node {
                    Some(node) => {
                        result.push(highlighter.left(row_index, column_index));
                        result.push_str(node.room.to_string().as_str());
                        result.push(highlighter.right(row_index, column_index));
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
    fn left(&self, _: RowIndex, _: ColumnIndex) -> char {
        ' '
    }

    fn right(&self, _: RowIndex, _: ColumnIndex) -> char {
        ' '
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string_with_highlighter(DummyHighlighter))
    }
}

impl Node {
    pub fn new(room: Room, exit_bits: ExitBits) -> Self {
        Self { room, exit_bits }
    }

    pub fn has_exit(&self, exit: ExitBits) -> bool {
        self.exit_bits.contains(exit)
    }
}

impl fmt::Display for Room {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Room::Boss => "B",
                Room::BurningElite1 => "1",
                Room::BurningElite2 => "2",
                Room::BurningElite3 => "3",
                Room::BurningElite4 => "4",
                Room::Elite => "E",
                Room::Event => "?",
                Room::Monster => "M",
                Room::RestSite => "R",
                Room::Shop => "$",
                Room::Treasure => "T",
            }
        )
    }
}

impl fmt::Display for ExitBits {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self.bits() {
                0b001 => r"  /",
                0b010 => r" | ",
                0b011 => r" |/",
                0b100 => r"\  ",
                0b101 => r"\ /",
                0b110 => r"\| ",
                0b111 => r"\|/",
                _ => unreachable!(),
            }
        )
    }
}
