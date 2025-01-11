use std::fmt;

use crate::data::NeowBlessing;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Prompt {
    HaltAndCatchFire,
    NeowBlessing,
}

#[derive(Clone, Debug)]
pub enum Choice {
    CatchFire,
    NeowBlessing(NeowBlessing),
}

impl fmt::Display for Prompt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Prompt::HaltAndCatchFire => write!(f, "You halt. Now decide your fate"),
            Prompt::NeowBlessing => write!(f, "Choose Neow's Blessing"),
        }
    }
}

impl fmt::Display for Choice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Choice::CatchFire => write!(f, "Catch Fire"),
            Choice::NeowBlessing(blessing) => write!(f, "{}", blessing),
        }
    }
}
