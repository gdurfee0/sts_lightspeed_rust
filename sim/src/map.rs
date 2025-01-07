use std::fmt;

use bitflags::bitflags;

pub struct Map([[Option<Room>; 7]; 15]);

bitflags! {
    pub struct Exit: u8 {
        const Left     = 0b100;
        const Straight = 0b010;
        const Right    = 0b001;
    }
}

impl Exit {
    fn symbol(&self) -> &str {
        match self.bits() {
            0b001 => r"  /",
            0b010 => r" | ",
            0b011 => r" |/",
            0b100 => r"\  ",
            0b101 => r"\ /",
            0b110 => r"\| ",
            0b111 => r"\|/",
            _ => r"   ",
        }
    }
}

pub enum Room {
    Shop(Exit),
    Campfire(Exit),
    Event(Exit),
    Elite(Exit),
    BurningElite1(Exit),
    BurningElite2(Exit),
    BurningElite3(Exit),
    BurningElite4(Exit),
    Monster(Exit),
    Treasure(Exit),
    Boss,
}

impl Room {
    fn symbols(&self) -> (&str, &str) {
        match self {
            Room::Shop(exits) => (" $ ", exits.symbol()),
            Room::Campfire(exits) => (" R ", exits.symbol()),
            Room::Event(exits) => (" ? ", exits.symbol()),
            Room::Elite(exits) => (" E ", exits.symbol()),
            Room::BurningElite1(exits) => (" 1 ", exits.symbol()),
            Room::BurningElite2(exits) => (" 2 ", exits.symbol()),
            Room::BurningElite3(exits) => (" 3 ", exits.symbol()),
            Room::BurningElite4(exits) => (" 4 ", exits.symbol()),
            Room::Monster(exits) => (" M ", exits.symbol()),
            Room::Treasure(exits) => (" T ", exits.symbol()),
            Room::Boss => (" B ", "   "),
        }
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut rows = Vec::new();
        for row in self.0.iter().rev() {
            let mut exit_row = String::new();
            let mut room_row = String::new();
            for room in row.iter() {
                match room {
                    Some(room) => {
                        let (room_symbol, exit_symbol) = room.symbols();
                        room_row.push_str(room_symbol);
                        exit_row.push_str(exit_symbol);
                    }
                    None => {
                        room_row.push_str("   ");
                        exit_row.push_str("   ");
                    }
                }
            }
            rows.push(exit_row);
            rows.push(room_row);
        }
        write!(f, "{}", rows.join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use once_cell::sync::Lazy;

    static MAP_0SLAYTHESPIRE: Lazy<Map> = Lazy::new(|| {
        Map([
            [
                Some(Room::Monster(Exit::Straight)),
                None,
                Some(Room::Monster(Exit::Straight)),
                None,
                None,
                Some(Room::Monster(Exit::Straight)),
                None,
            ],
            [
                Some(Room::Monster(Exit::Straight | Exit::Right)),
                None,
                Some(Room::Monster(Exit::Straight | Exit::Right)),
                None,
                None,
                Some(Room::Monster(Exit::Left | Exit::Straight)),
                None,
            ],
            [
                Some(Room::Event(Exit::Straight)),
                Some(Room::Monster(Exit::Right)),
                Some(Room::Monster(Exit::Straight)),
                Some(Room::Shop(Exit::Straight)),
                Some(Room::Event(Exit::Left)),
                Some(Room::Monster(Exit::Left)),
                None,
            ],
            [
                Some(Room::Event(Exit::Straight)),
                None,
                Some(Room::Monster(Exit::Straight | Exit::Right)),
                Some(Room::Event(Exit::Straight)),
                Some(Room::Monster(Exit::Straight)),
                None,
                None,
            ],
            [
                Some(Room::Monster(Exit::Straight)),
                None,
                Some(Room::Event(Exit::Left)),
                Some(Room::Monster(Exit::Left | Exit::Right)),
                Some(Room::Event(Exit::Straight)),
                None,
                None,
            ],
            [
                Some(Room::Campfire(Exit::Straight)),
                Some(Room::Elite(Exit::Straight)),
                Some(Room::Campfire(Exit::Left)),
                None,
                Some(Room::Monster(Exit::Left | Exit::Right)),
                None,
                None,
            ],
            [
                Some(Room::Monster(Exit::Straight)),
                Some(Room::Monster(Exit::Straight | Exit::Right)),
                None,
                Some(Room::Event(Exit::Left)),
                None,
                Some(Room::Monster(Exit::Right)),
                None,
            ],
            [
                Some(Room::Monster(Exit::Straight)),
                Some(Room::Shop(Exit::Straight)),
                Some(Room::Monster(Exit::Left | Exit::Straight)),
                None,
                None,
                None,
                Some(Room::Monster(Exit::Left)),
            ],
            [
                Some(Room::Treasure(Exit::Straight)),
                Some(Room::Treasure(Exit::Left | Exit::Right)),
                Some(Room::Treasure(Exit::Right)),
                None,
                None,
                Some(Room::Treasure(Exit::Left)),
                None,
            ],
            [
                Some(Room::Campfire(Exit::Straight | Exit::Right)),
                None,
                Some(Room::Monster(Exit::Straight | Exit::Right)),
                Some(Room::Event(Exit::Straight)),
                Some(Room::Campfire(Exit::Straight)),
                None,
                None,
            ],
            [
                Some(Room::Event(Exit::Straight)),
                Some(Room::Monster(Exit::Straight)),
                Some(Room::Campfire(Exit::Left)),
                Some(Room::Event(Exit::Left | Exit::Right)),
                Some(Room::Monster(Exit::Right)),
                None,
                None,
            ],
            [
                Some(Room::BurningElite1(Exit::Right)),
                Some(Room::Monster(Exit::Right)),
                Some(Room::Event(Exit::Straight)),
                None,
                Some(Room::Monster(Exit::Left)),
                Some(Room::Event(Exit::Straight)),
                None,
            ],
            [
                None,
                Some(Room::Event(Exit::Left)),
                Some(Room::Elite(Exit::Left | Exit::Straight)),
                Some(Room::Campfire(Exit::Straight)),
                None,
                Some(Room::Campfire(Exit::Right)),
                None,
            ],
            [
                Some(Room::Elite(Exit::Straight)),
                Some(Room::Monster(Exit::Left)),
                Some(Room::Shop(Exit::Straight)),
                Some(Room::Monster(Exit::Right)),
                None,
                None,
                Some(Room::Elite(Exit::Left)),
            ],
            [
                Some(Room::Campfire(Exit::Right)),
                None,
                Some(Room::Campfire(Exit::Right)),
                None,
                Some(Room::Campfire(Exit::Left)),
                Some(Room::Campfire(Exit::Left)),
                None,
            ],
        ])
    });

    static MAP_1SLAYTHESPIRE: Lazy<Map> = Lazy::new(|| {
        Map([
            [
                None,
                Some(Room::Monster(Exit::Right)),
                None,
                Some(Room::Monster(Exit::Right)),
                Some(Room::Monster(Exit::Right)),
                Some(Room::Monster(Exit::Right)),
                None,
            ],
            [
                None,
                None,
                Some(Room::Monster(Exit::Straight)),
                None,
                Some(Room::Event(Exit::Left | Exit::Right)),
                Some(Room::Shop(Exit::Straight)),
                Some(Room::Shop(Exit::Left | Exit::Straight)),
            ],
            [
                None,
                None,
                Some(Room::Event(Exit::Left)),
                Some(Room::Monster(Exit::Straight)),
                None,
                Some(Room::Event(Exit::Left | Exit::Straight)),
                Some(Room::Monster(Exit::Straight)),
            ],
            [
                None,
                Some(Room::Event(Exit::Straight)),
                None,
                Some(Room::Monster(Exit::Straight)),
                Some(Room::Monster(Exit::Left | Exit::Straight)),
                Some(Room::Event(Exit::Left)),
                Some(Room::Monster(Exit::Left)),
            ],
            [
                None,
                Some(Room::Monster(Exit::Straight)),
                None,
                Some(Room::Monster(Exit::Left | Exit::Right)),
                Some(Room::Event(Exit::Straight)),
                Some(Room::Monster(Exit::Left)),
                None,
            ],
            [
                None,
                Some(Room::Campfire(Exit::Straight)),
                Some(Room::Elite(Exit::Right)),
                None,
                Some(Room::Campfire(Exit::Left | Exit::Straight | Exit::Right)),
                None,
                None,
            ],
            [
                None,
                Some(Room::Elite(Exit::Right)),
                None,
                Some(Room::Monster(Exit::Left)),
                Some(Room::Elite(Exit::Left | Exit::Straight)),
                Some(Room::Event(Exit::Straight)),
                None,
            ],
            [
                None,
                None,
                Some(Room::Campfire(Exit::Straight)),
                Some(Room::Monster(Exit::Left)),
                Some(Room::Campfire(Exit::Left)),
                Some(Room::Monster(Exit::Right)),
                None,
            ],
            [
                None,
                None,
                Some(Room::Treasure(Exit::Left | Exit::Straight | Exit::Right)),
                Some(Room::Treasure(Exit::Straight)),
                None,
                None,
                Some(Room::Treasure(Exit::Straight)),
            ],
            [
                None,
                Some(Room::Monster(Exit::Straight)),
                Some(Room::Campfire(Exit::Straight)),
                Some(Room::Event(Exit::Left | Exit::Right)),
                None,
                None,
                Some(Room::Monster(Exit::Left)),
            ],
            [
                None,
                Some(Room::Monster(Exit::Left | Exit::Right)),
                Some(Room::Monster(Exit::Right)),
                None,
                Some(Room::Event(Exit::Straight)),
                Some(Room::BurningElite4(Exit::Straight)),
                None,
            ],
            [
                Some(Room::Monster(Exit::Right)),
                None,
                Some(Room::Event(Exit::Right)),
                Some(Room::Monster(Exit::Straight)),
                Some(Room::Event(Exit::Left)),
                Some(Room::Monster(Exit::Straight)),
                None,
            ],
            [
                None,
                Some(Room::Monster(Exit::Straight)),
                None,
                Some(Room::Monster(Exit::Left | Exit::Straight | Exit::Right)),
                None,
                Some(Room::Monster(Exit::Right)),
                None,
            ],
            [
                None,
                Some(Room::Event(Exit::Straight)),
                Some(Room::Shop(Exit::Right)),
                Some(Room::Monster(Exit::Right)),
                Some(Room::Monster(Exit::Straight | Exit::Right)),
                None,
                Some(Room::Monster(Exit::Left)),
            ],
            [
                None,
                Some(Room::Campfire(Exit::Right)),
                None,
                Some(Room::Campfire(Exit::Straight)),
                Some(Room::Campfire(Exit::Left)),
                Some(Room::Campfire(Exit::Left)),
                None,
            ],
        ])
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
    }
}
