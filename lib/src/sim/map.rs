use anyhow::{anyhow, Error};

use crate::data::act::Act;
use crate::map::{ExitBits, MapBuilder, MapHighlighter, NodeGrid, Room, ROW_COUNT};
use crate::player::PlayerController;
use crate::rng::Seed;
use crate::{ColumnIndex, RowIndex};

pub struct MapSimulator {
    // Current player location (row, column) in the map
    player_location: Option<(RowIndex, ColumnIndex)>,

    // Computed map for this act
    map: NodeGrid,
}

impl MapSimulator {
    pub fn new(seed: Seed) -> Self {
        let map = MapBuilder::from(seed, Act::get(1)).build();
        Self {
            player_location: None,
            map,
        }
    }

    pub fn send_map_to_player(&self, player: &mut PlayerController) -> Result<(), Error> {
        player.send_map_string(self.map_string())?;
        Ok(())
    }

    pub fn advance(&mut self, player: &mut PlayerController) -> Result<Room, Error> {
        let (next_row_index, movement_options) = match self.player_location {
            // Player is not yet on the map, so may select any room in the bottom row.
            None => (
                0,
                self.map
                    .nonempty_columns_for_row(0)
                    .into_iter()
                    .collect::<Vec<_>>(),
            ),
            // Player is at the top of the map, and will move to the boss next.
            Some((row_index, _)) if row_index == ROW_COUNT - 1 => {
                self.player_location = None;
                // TODO: something about advancing to the next Act
                player.send_map_string(self.map_string())?;
                return Ok(Room::Boss);
            }
            // Player is already on the board, and needs to move to a new room via an exit.
            Some((row_index, column_index)) => {
                let node = self.map.get(row_index, column_index).unwrap_or_else(|| {
                    panic!(
                        "Player is in an impossible location! {} {}",
                        row_index, column_index
                    )
                });
                let mut columns = vec![];
                if node.has_exit(ExitBits::Left) {
                    columns.push(column_index - 1);
                }
                if node.has_exit(ExitBits::Up) {
                    columns.push(column_index);
                }
                if node.has_exit(ExitBits::Right) {
                    columns.push(column_index + 1);
                }
                (row_index + 1, columns.into_iter().collect::<Vec<_>>())
            }
        };
        player.send_map_string(
            self.highlighted_map_string(
                &movement_options
                    .iter()
                    .map(|column_index| (next_row_index, *column_index))
                    .collect::<Vec<_>>(),
            ),
        )?;
        let next_column_index = player.climb_floor(&movement_options)?;
        self.player_location = Some((next_row_index, next_column_index));
        if let Some(node) = self.map.get(next_row_index, next_column_index) {
            player.send_map_string(self.map_string())?;
            Ok(node.room)
        } else {
            Err(anyhow!(
                "Player is in an impossible location! {} {}",
                next_row_index,
                next_column_index
            ))
        }
    }

    fn map_string(&self) -> String {
        format!(
            "{}\n\n a  b  c  d  e  f  g",
            self.map.to_string_with_highlighter(StsMapHighlighter {
                player_location: self.player_location,
                highlights: &[],
            })
        )
    }

    fn highlighted_map_string(&self, highlights: &[(RowIndex, ColumnIndex)]) -> String {
        let mut suffix = String::new();
        for (i, c) in ('a'..'g').enumerate() {
            if highlights
                .iter()
                .any(|(_, column_index)| i == *column_index)
            {
                suffix.push('{');
                suffix.push(c);
                suffix.push('}');
            } else {
                suffix.push(' ');
                suffix.push(c);
                suffix.push(' ');
            }
        }
        let mut map_string = self.map.to_string_with_highlighter(StsMapHighlighter {
            player_location: self.player_location,
            highlights,
        });
        map_string.push_str("\n\n");
        map_string.push_str(&suffix);
        map_string
    }
}

struct StsMapHighlighter<'a> {
    player_location: Option<(RowIndex, ColumnIndex)>,
    highlights: &'a [(RowIndex, ColumnIndex)],
}

impl MapHighlighter for StsMapHighlighter<'_> {
    fn left(&self, row_index: RowIndex, column_index: ColumnIndex) -> char {
        if self.highlights.contains(&(row_index, column_index)) {
            '{'
        } else if let Some((player_row_index, player_column_index)) = self.player_location {
            if row_index == player_row_index && column_index == player_column_index {
                '['
            } else {
                ' '
            }
        } else {
            ' '
        }
    }

    fn right(&self, row_index: RowIndex, column_index: ColumnIndex) -> char {
        if self.highlights.contains(&(row_index, column_index)) {
            '}'
        } else if let Some((player_row_index, player_column_index)) = self.player_location {
            if row_index == player_row_index && column_index == player_column_index {
                ']'
            } else {
                ' '
            }
        } else {
            ' '
        }
    }
}
