use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Room {
    Boss,
    BurningElite1,
    BurningElite2,
    BurningElite3,
    BurningElite4,
    Campfire,
    Elite,
    Event,
    Monster,
    Shop,
    Treasure,
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
                Room::Campfire => "R",
                Room::Elite => "E",
                Room::Event => "?",
                Room::Monster => "M",
                Room::Shop => "$",
                Room::Treasure => "T",
            }
        )
    }
}
