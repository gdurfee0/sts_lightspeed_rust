use anyhow::{anyhow, Error};

use crate::data::Act;
use crate::map::{ExitBits, MapBuilder, MapHighlighter, NodeGrid, Room, ROW_COUNT};
use crate::rng::Seed;

use super::player::Player;

pub struct MapSimulator {
    // Current player location (row, column) in the map
    player_row_col: Option<(u8, u8)>,

    // Computed map for this act
    map: NodeGrid,
}

impl MapSimulator {
    pub fn new(seed: Seed) -> Self {
        let map = MapBuilder::from(seed, Act::get(1)).build();
        Self {
            player_row_col: None,
            map,
        }
    }

    pub fn send_map_to_player(&self, player: &mut Player) -> Result<(), Error> {
        player.send_map_string(self.map_string())?;
        Ok(())
    }

    pub fn advance(&mut self, player: &mut Player) -> Result<Room, Error> {
        let (next_row, movement_options) = match self.player_row_col {
            // Player is not yet on the map, so may select any room in the bottom row.
            None => (
                0,
                self.map
                    .nonempty_cols_for_row(0)
                    .into_iter()
                    .collect::<Vec<_>>(),
            ),
            // Player is at the top of the map, and will move to the boss next.
            Some((row, _)) if row == ROW_COUNT as u8 - 1 => {
                self.player_row_col = None;
                // TODO: something about advancing to the next Act
                player.send_map_string(self.map_string())?;
                return Ok(Room::Boss);
            }
            // Player is already on the board, and needs to move to a new room via an exit.
            Some((row, col)) => {
                let node = self.map.get(row, col).unwrap_or_else(|| {
                    panic!("Player is in an impossible location! {} {}", row, col)
                });
                let mut columns = vec![];
                if node.has_exit(ExitBits::Left) {
                    columns.push(col - 1);
                }
                if node.has_exit(ExitBits::Up) {
                    columns.push(col);
                }
                if node.has_exit(ExitBits::Right) {
                    columns.push(col + 1);
                }
                (row + 1, columns.into_iter().collect::<Vec<_>>())
            }
        };
        player.send_map_string(
            self.highlighted_map_string(
                &movement_options
                    .iter()
                    .map(|col| (next_row, *col))
                    .collect::<Vec<_>>(),
            ),
        )?;
        let next_col = player.choose_movement_option(movement_options)?;
        self.player_row_col = Some((next_row, next_col));
        if let Some(node) = self.map.get(next_row, next_col) {
            player.send_map_string(self.map_string())?;
            Ok(node.room)
        } else {
            Err(anyhow!(
                "Player is in an impossible location! {} {}",
                next_row,
                next_col
            ))
        }
    }

    fn map_string(&self) -> String {
        format!(
            "{}\n\n a  b  c  d  e  f  g",
            self.map.to_string_with_highlighter(StsMapHighlighter {
                player_row_col: self.player_row_col,
                row_col_highlights: &[],
            })
        )
    }

    fn highlighted_map_string(&self, row_col_highlights: &[(u8, u8)]) -> String {
        let mut suffix = String::new();
        for (i, c) in ('a'..'g').enumerate() {
            if row_col_highlights.iter().any(|(_, col)| i as u8 == *col) {
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
            player_row_col: self.player_row_col,
            row_col_highlights,
        });
        map_string.push_str("\n\n");
        map_string.push_str(&suffix);
        map_string
    }
}

struct StsMapHighlighter<'a> {
    player_row_col: Option<(u8, u8)>,
    row_col_highlights: &'a [(u8, u8)],
}

impl MapHighlighter for StsMapHighlighter<'_> {
    fn left(&self, row: u8, col: u8) -> char {
        if self.row_col_highlights.contains(&(row, col)) {
            '{'
        } else if let Some((player_row, player_col)) = self.player_row_col {
            if row == player_row && col == player_col {
                '['
            } else {
                ' '
            }
        } else {
            ' '
        }
    }

    fn right(&self, row: u8, col: u8) -> char {
        if self.row_col_highlights.contains(&(row, col)) {
            '}'
        } else if let Some((player_row, player_col)) = self.player_row_col {
            if row == player_row && col == player_col {
                ']'
            } else {
                ' '
            }
        } else {
            ' '
        }
    }
}
