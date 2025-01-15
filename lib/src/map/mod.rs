mod builder;
mod exit;
mod graph;
mod grid;
mod node;
mod room;

pub const ROW_COUNT: usize = 15;
pub const COLUMN_COUNT: usize = 7;
pub const COLUMN_MAX: crate::ColumnIndex = COLUMN_COUNT - 1;
const PATH_DENSITY: usize = 6;

pub use builder::MapBuilder;
pub use exit::ExitBits;
pub use grid::{MapHighlighter, NodeGrid};
pub use room::Room;
