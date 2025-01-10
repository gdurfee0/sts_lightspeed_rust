use std::fmt;

use crate::data::NeowBlessing;

#[derive(Clone, Copy, Debug)]
pub enum Prompt {
    NeowBlessing,
}

#[derive(Clone, Copy, Debug)]
pub enum Choice {
    NeowBlessing(NeowBlessing),
}

impl fmt::Display for Prompt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Prompt::NeowBlessing => write!(f, "Choose Neow's Blessing"),
        }
    }
}

impl fmt::Display for Choice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Choice::NeowBlessing(blessing) => write!(f, "{}", blessing),
        }
    }
}
