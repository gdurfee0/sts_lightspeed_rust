mod builder;
mod exit;
mod graph;
mod grid;
mod node;
mod room;

const COLUMN_COUNT: usize = 7;
const COLUMN_MAX: usize = COLUMN_COUNT - 1;
const ROW_COUNT: usize = 15;
const PATH_DENSITY: usize = 6;

pub use builder::MapBuilder;
pub use exit::ExitBits;
pub use grid::{MapHighlighter, NodeGrid};
pub use node::Node;
pub use room::Room;
