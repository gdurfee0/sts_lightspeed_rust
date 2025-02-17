use std::fmt;

use crate::components::map::{ExitBits, Map, Node, Room, COLUMN_COUNT, COLUMN_MAX, ROW_COUNT};
use crate::types::{ColumnIndex, RowIndex};

use super::node_builder::NodeBuilder;

#[derive(Default)]
pub struct NodeBuilderGrid {
    grid: [[Option<NodeBuilder>; COLUMN_COUNT]; ROW_COUNT],
}

impl NodeBuilderGrid {
    pub fn remove(
        &mut self,
        row_index: RowIndex,
        column_index: ColumnIndex,
    ) -> Option<NodeBuilder> {
        self.grid[row_index][column_index].take()
    }

    /// Returns a Vec<usize> holding column indices of nodes in the given row that are not empty.
    pub fn nonempty_columns_for_row(&self, row_index: RowIndex) -> Vec<ColumnIndex> {
        self.grid[row_index]
            .iter()
            .enumerate()
            .filter_map(|(column_index, maybe_node)| maybe_node.as_ref().map(|_| column_index))
            .collect()
    }

    pub fn set_all_rooms_in_row(&mut self, row_index: RowIndex, room: Room) {
        for maybe_node in self.grid[row_index].iter_mut() {
            if let Some(node) = maybe_node.as_mut() {
                node.room = Some(room);
            }
        }
    }

    pub fn set_room(&mut self, row_index: RowIndex, column_index: ColumnIndex, room: Room) {
        self.grid[row_index][column_index]
            .get_or_insert_default()
            .room = Some(room);
    }

    /// Records the column of the parent node for the given node.
    pub fn receord_parent_column(
        &mut self,
        row_index: RowIndex,
        column_index: ColumnIndex,
        parent_column_index: ColumnIndex,
    ) {
        self.grid[row_index][column_index]
            .get_or_insert_default()
            .record_parent_column(parent_column_index);
    }

    pub fn add_exit(&mut self, row_index: RowIndex, column_index: ColumnIndex, exit: ExitBits) {
        self.grid[row_index][column_index]
            .get_or_insert_default()
            .add_exit(exit);
    }

    pub fn has_exit(&self, row_index: RowIndex, column_index: ColumnIndex, exit: ExitBits) -> bool {
        self.grid[row_index][column_index]
            .as_ref()
            .map_or(false, |node| node.has_exit(exit))
    }

    /// Returns true iff the node at the given row and column has a parent node with the given room.
    pub fn has_parent_room_of(
        &self,
        row_index: RowIndex,
        column_index: ColumnIndex,
        room: Room,
    ) -> bool {
        row_index > 0
            && (self
                .maybe_down_left_parent(row_index, column_index)
                .map_or(false, |node| node.room.map(|r| r == room).unwrap_or(false))
                || self
                    .maybe_down_parent(row_index, column_index)
                    .map_or(false, |node| node.room.map(|r| r == room).unwrap_or(false))
                || self
                    .maybe_down_right_parent(row_index, column_index)
                    .map_or(false, |node| node.room.map(|r| r == room).unwrap_or(false)))
    }

    /// Returns the node below and to the left, provided it connects to node at the given
    /// coordinates.
    fn maybe_down_left_parent(
        &self,
        row_index: RowIndex,
        column_index: ColumnIndex,
    ) -> Option<&NodeBuilder> {
        if column_index > 0 {
            self.grid[row_index - 1][column_index - 1]
                .as_ref()
                .filter(|node| node.has_exit(ExitBits::Right))
        } else {
            None
        }
    }

    /// Returns the node below the given coordinates, provided it connects to the node at the given
    /// coordinates.
    fn maybe_down_parent(
        &self,
        row_index: RowIndex,
        column_index: ColumnIndex,
    ) -> Option<&NodeBuilder> {
        self.grid[row_index - 1][column_index]
            .as_ref()
            .filter(|node| node.has_exit(ExitBits::Up))
    }

    /// Returns the node below and to the right, provided it connects to the node at the given
    /// coordinates.
    fn maybe_down_right_parent(
        &self,
        row_index: RowIndex,
        column_index: ColumnIndex,
    ) -> Option<&NodeBuilder> {
        if column_index < COLUMN_MAX {
            self.grid[row_index - 1][column_index + 1]
                .as_ref()
                .filter(|node| node.has_exit(ExitBits::Left))
        } else {
            None
        }
    }

    /// Returns true iff the node at the given row and column shares a parent with another node
    /// that has the given room type.
    pub fn has_left_sibling_room_of(
        &self,
        row_index: RowIndex,
        column_index: ColumnIndex,
        room: Room,
    ) -> bool {
        self.maybe_down_left_parent(row_index, column_index)
            .map_or(false, |_| {
                self.has_child_room_of(row_index - 1, column_index - 1, room)
            })
            || self
                .maybe_down_parent(row_index, column_index)
                .map_or(false, |_| {
                    self.has_child_room_of(row_index - 1, column_index, room)
                })
    }

    /// Returns true iff the node at the given row and column has a child node with the given
    /// room type.
    fn has_child_room_of(
        &self,
        row_index: RowIndex,
        column_index: ColumnIndex,
        room: Room,
    ) -> bool {
        [ExitBits::Left, ExitBits::Up, ExitBits::Right]
            .iter()
            .any(|&exit| {
                self.has_exit(row_index, column_index, exit)
                    && self.grid[row_index + 1][match exit {
                        ExitBits::Left => column_index - 1,
                        ExitBits::Up => column_index,
                        ExitBits::Right => column_index + 1,
                        _ => unreachable!(),
                    }]
                    .as_ref()
                    .map_or(false, |node| node.room.map(|r| r == room).unwrap_or(false))
            })
    }

    /// Returns an iterator over the recorded parent columns for the given node, unordered and
    /// possibly with duplicates.
    pub fn recorded_parent_columns_iter(
        &self,
        row_index: RowIndex,
        column_index: ColumnIndex,
    ) -> impl Iterator<Item = &ColumnIndex> {
        self.grid[row_index][column_index]
            .as_ref()
            .map(|node| node.recorded_parent_columns_iter())
            .into_iter()
            .flatten()
    }

    /// Attempts to determine if the node at the given row and column shares a parent with another
    /// node in the same row, but fails to do so in most cases.
    pub fn buggy_implementation_of_shares_parent_with(
        &self,
        row_index: RowIndex,
        my_column_index: ColumnIndex,
        other_column_index: usize,
    ) -> bool {
        if let (Some(my_node), Some(other_node)) = (
            &self.grid[row_index][my_column_index].as_ref(),
            &self.grid[row_index][other_column_index].as_ref(),
        ) {
            // "other_column_index >= row_index" is almost certainly a bug in the original code;
            // it's probably supposed to be "other_column_index >= my_column_index". This causes a
            // lot of small cycles to be missed because the wrong comparisons are being performed.
            if other_column_index >= row_index {
                my_node.rightmost_recorded_parent_column()
                    == other_node.leftmost_recorded_parent_column()
            } else {
                other_node.rightmost_recorded_parent_column()
                    == my_node.leftmost_recorded_parent_column()
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
            .filter(|&(row_index, _)| row_index != ROW_COUNT - 2) // Reference code calls this "restRowBug"
            .flat_map(|(_, row)| row)
            .filter(|maybe_node| maybe_node.is_some())
            .count()
    }
}

impl From<NodeBuilderGrid> for Map {
    fn from(mut builder: NodeBuilderGrid) -> Self {
        Map::new(
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

    use crate::components::map::MapHighlighter;

    use super::*;

    impl NodeBuilderGrid {
        pub fn exit_bits_as_vec(&self) -> [[u8; COLUMN_COUNT]; ROW_COUNT - 1] {
            let mut exits = [[0; COLUMN_COUNT]; ROW_COUNT - 1];
            for (row_index, row_nodes) in self.grid.iter().enumerate().take(ROW_COUNT - 1) {
                for (column_index, maybe_node) in row_nodes.iter().enumerate() {
                    if let Some(node) = maybe_node {
                        exits[row_index][column_index] = node.exit_bits().bits();
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
        pub row_index: RowIndex,
        pub column_index: ColumnIndex,
    }

    impl MapHighlighter for SimpleHighlighter {
        fn left(&self, row_index: RowIndex, column_index: ColumnIndex) -> char {
            if row_index == self.row_index && column_index == self.column_index {
                '['
            } else {
                ' '
            }
        }

        fn right(&self, row_index: RowIndex, column_index: ColumnIndex) -> char {
            if row_index == self.row_index && column_index == self.column_index {
                ']'
            } else {
                ' '
            }
        }
    }

    pub static MAP_0SLAYTHESPIRE: Lazy<Map> = Lazy::new(|| Map {
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
                Some(Node::new(Room::RestSite, ExitBits::Up)),
                Some(Node::new(Room::Elite, ExitBits::Up)),
                Some(Node::new(Room::RestSite, ExitBits::Left)),
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
                Some(Node::new(Room::RestSite, ExitBits::Up | ExitBits::Right)),
                None,
                Some(Node::new(Room::Monster, ExitBits::Up | ExitBits::Right)),
                Some(Node::new(Room::Event, ExitBits::Up)),
                Some(Node::new(Room::RestSite, ExitBits::Up)),
                None,
                None,
            ],
            [
                Some(Node::new(Room::Event, ExitBits::Up)),
                Some(Node::new(Room::Monster, ExitBits::Up)),
                Some(Node::new(Room::RestSite, ExitBits::Left)),
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
                Some(Node::new(Room::RestSite, ExitBits::Up)),
                None,
                Some(Node::new(Room::RestSite, ExitBits::Right)),
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
                Some(Node::new(Room::RestSite, ExitBits::Right)),
                None,
                Some(Node::new(Room::RestSite, ExitBits::Right)),
                None,
                Some(Node::new(Room::RestSite, ExitBits::Left)),
                Some(Node::new(Room::RestSite, ExitBits::Left)),
                None,
            ],
        ],
    });

    static MAP_1SLAYTHESPIRE: Lazy<Map> = Lazy::new(|| Map {
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
                Some(Node::new(Room::RestSite, ExitBits::Up)),
                Some(Node::new(Room::Elite, ExitBits::Right)),
                None,
                Some(Node::new(
                    Room::RestSite,
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
                Some(Node::new(Room::RestSite, ExitBits::Up)),
                Some(Node::new(Room::Monster, ExitBits::Left)),
                Some(Node::new(Room::RestSite, ExitBits::Left)),
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
                Some(Node::new(Room::RestSite, ExitBits::Up)),
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
                Some(Node::new(Room::RestSite, ExitBits::Right)),
                None,
                Some(Node::new(Room::RestSite, ExitBits::Up)),
                Some(Node::new(Room::RestSite, ExitBits::Left)),
                Some(Node::new(Room::RestSite, ExitBits::Left)),
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
            MAP_0SLAYTHESPIRE.to_string_with_highlighter(SimpleHighlighter {
                row_index: 4,
                column_index: 3
            }),
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
