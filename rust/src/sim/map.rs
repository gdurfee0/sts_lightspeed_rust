use std::sync::mpsc::{Receiver, Sender};

use anyhow::{anyhow, Ok};

use crate::data::{Act, Ascension};
use crate::map::{ExitBits, MapBuilder, MapHighlighter, NodeGrid, Room, ROW_COUNT};
use crate::rng::Seed;

use super::message::StsMessage;
use super::{Choice, Prompt};

pub struct MapSimulator {
    // Game seed used to create maps for every act
    seed: Seed,

    // Current player location (row, column) in the map
    player_row_col: Option<(usize, usize)>,

    // Computed map for this act
    map: NodeGrid,
}

impl MapSimulator {
    pub fn new(seed: &Seed, ascension: Ascension) -> Self {
        let map = MapBuilder::from(seed, ascension, Act::get(1)).build();
        Self {
            seed: seed.clone(),
            player_row_col: None,
            map,
        }
    }

    pub fn send_map(&self, output_tx: &mut Sender<StsMessage>) -> Result<(), anyhow::Error> {
        output_tx.send(StsMessage::Map(format!(
            "{}\n\n a  b  c  d  e  f  g",
            self.map.to_string_with_highlighter(StsMapHighlighter {
                player_row_col: self.player_row_col,
                choices: vec![],
            })
        )))?;
        Ok(())
    }

    pub fn advance(
        &mut self,
        input_rx: &mut Receiver<usize>,
        output_tx: &mut Sender<StsMessage>,
    ) -> Result<Room, anyhow::Error> {
        let (prompt, choices, next_row, map_choices) = match self.player_row_col {
            None => {
                // Player is not yet on the map, so may select any room in the bottom row.
                (
                    Prompt::MoveUp,
                    self.map
                        .nonempty_cols_for_row(0)
                        .into_iter()
                        .map(Choice::Column)
                        .collect::<Vec<_>>(),
                    0,
                    self.map
                        .nonempty_cols_for_row(0)
                        .into_iter()
                        .map(|col| (0, col))
                        .collect::<Vec<_>>(),
                )
            }
            Some((row, _)) if row == ROW_COUNT - 1 => {
                // Player is at the top of the map, and will move to the boss next.
                self.player_row_col = None;
                // TODO: something about advancing to the next Act
                self.send_map(output_tx)?;
                return Ok(Room::Boss);
            }
            Some((row, col)) => {
                // Player is already on the board, and needs to move to a new room via an
                // existing exit.
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
                (
                    Prompt::MoveUp,
                    columns
                        .iter()
                        .copied()
                        .map(Choice::Column)
                        .collect::<Vec<_>>(),
                    row + 1,
                    columns
                        .into_iter()
                        .map(|col| (row + 1, col))
                        .collect::<Vec<_>>(),
                )
            }
        };
        self.send_map_with_choices(output_tx, map_choices)?;
        output_tx.send(StsMessage::Choose(prompt, choices.to_vec()))?;
        let choice_index = input_rx.recv()?;
        if let Some(Choice::Column(col)) = choices.get(choice_index) {
            self.player_row_col = Some((next_row, *col));
            let node = self.map.get(next_row, *col).unwrap_or_else(|| {
                panic!("Player is in an impossible location! {} {}", next_row, col)
            });
            self.send_map(output_tx)?;
            Ok(node.room)
        } else {
            Err(anyhow!(
                "[MapSimulator] Invalid choice index {} from client; expected 0..{}",
                choice_index,
                choices.len()
            ))
        }
    }

    fn send_map_with_choices(
        &self,
        output_tx: &mut Sender<StsMessage>,
        map_choices: Vec<(usize, usize)>,
    ) -> Result<(), anyhow::Error> {
        let mut suffix = String::new();
        for (i, c) in ('a'..'g').enumerate() {
            if map_choices.iter().any(|(_, col)| i == *col) {
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
            choices: map_choices,
        });
        map_string.push_str("\n\n");
        map_string.push_str(&suffix);
        output_tx.send(StsMessage::Map(map_string))?;
        Ok(())
    }
}

struct StsMapHighlighter {
    player_row_col: Option<(usize, usize)>,
    choices: Vec<(usize, usize)>,
}

impl MapHighlighter for StsMapHighlighter {
    fn left(&self, row: usize, col: usize) -> char {
        if self.choices.contains(&(row, col)) {
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

    fn right(&self, row: usize, col: usize) -> char {
        if self.choices.contains(&(row, col)) {
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
