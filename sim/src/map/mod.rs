mod builder;
mod exit;
mod graph;
mod grid;
mod node;
mod room;

const COLUMN_COUNT: usize = 7;
const ROW_COUNT: usize = 15;
const PATH_DENSITY: usize = 6;

pub use builder::MapBuilder;
