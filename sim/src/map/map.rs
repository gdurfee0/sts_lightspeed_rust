use std::fmt;

use super::exit::Exit;
use super::room::Room;
use super::{COLUMN_COUNT, ROW_COUNT};

#[derive(Default)]
pub struct Map(pub [[Option<Node>; COLUMN_COUNT]; ROW_COUNT]);

#[derive(Debug)]
pub struct Node(pub Room, pub Exit);

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.0.iter().rev() {
            for maybe_node in row.iter() {
                match maybe_node {
                    Some(node) => {
                        write!(f, "{}", node.1)?;
                    }
                    None => {
                        write!(f, "   ")?;
                    }
                }
            }
            writeln!(f)?;
            for maybe_node in row.iter() {
                match maybe_node {
                    Some(node) => {
                        write!(f, "{}", node.0)?;
                    }
                    None => {
                        write!(f, "   ")?;
                    }
                }
            }
            // Avoid adding an extra newline after the last row
            if row.as_ptr() != self.0[0].as_ptr() {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use once_cell::sync::Lazy;

    pub static MAP_0SLAYTHESPIRE: Lazy<Map> = Lazy::new(|| {
        Map([
            [
                Some(Node(Room::Monster, Exit::Straight)),
                None,
                Some(Node(Room::Monster, Exit::Straight)),
                None,
                None,
                Some(Node(Room::Monster, Exit::Straight)),
                None,
            ],
            [
                Some(Node(Room::Monster, Exit::Straight | Exit::Right)),
                None,
                Some(Node(Room::Monster, Exit::Straight | Exit::Right)),
                None,
                None,
                Some(Node(Room::Monster, Exit::Left | Exit::Straight)),
                None,
            ],
            [
                Some(Node(Room::Event, Exit::Straight)),
                Some(Node(Room::Monster, Exit::Right)),
                Some(Node(Room::Monster, Exit::Straight)),
                Some(Node(Room::Shop, Exit::Straight)),
                Some(Node(Room::Event, Exit::Left)),
                Some(Node(Room::Monster, Exit::Left)),
                None,
            ],
            [
                Some(Node(Room::Event, Exit::Straight)),
                None,
                Some(Node(Room::Monster, Exit::Straight | Exit::Right)),
                Some(Node(Room::Event, Exit::Straight)),
                Some(Node(Room::Monster, Exit::Straight)),
                None,
                None,
            ],
            [
                Some(Node(Room::Monster, Exit::Straight)),
                None,
                Some(Node(Room::Event, Exit::Left)),
                Some(Node(Room::Monster, Exit::Left | Exit::Right)),
                Some(Node(Room::Event, Exit::Straight)),
                None,
                None,
            ],
            [
                Some(Node(Room::Campfire, Exit::Straight)),
                Some(Node(Room::Elite, Exit::Straight)),
                Some(Node(Room::Campfire, Exit::Left)),
                None,
                Some(Node(Room::Monster, Exit::Left | Exit::Right)),
                None,
                None,
            ],
            [
                Some(Node(Room::Monster, Exit::Straight)),
                Some(Node(Room::Monster, Exit::Straight | Exit::Right)),
                None,
                Some(Node(Room::Event, Exit::Left)),
                None,
                Some(Node(Room::Monster, Exit::Right)),
                None,
            ],
            [
                Some(Node(Room::Monster, Exit::Straight)),
                Some(Node(Room::Shop, Exit::Straight)),
                Some(Node(Room::Monster, Exit::Left | Exit::Straight)),
                None,
                None,
                None,
                Some(Node(Room::Monster, Exit::Left)),
            ],
            [
                Some(Node(Room::Treasure, Exit::Straight)),
                Some(Node(Room::Treasure, Exit::Left | Exit::Right)),
                Some(Node(Room::Treasure, Exit::Right)),
                None,
                None,
                Some(Node(Room::Treasure, Exit::Left)),
                None,
            ],
            [
                Some(Node(Room::Campfire, Exit::Straight | Exit::Right)),
                None,
                Some(Node(Room::Monster, Exit::Straight | Exit::Right)),
                Some(Node(Room::Event, Exit::Straight)),
                Some(Node(Room::Campfire, Exit::Straight)),
                None,
                None,
            ],
            [
                Some(Node(Room::Event, Exit::Straight)),
                Some(Node(Room::Monster, Exit::Straight)),
                Some(Node(Room::Campfire, Exit::Left)),
                Some(Node(Room::Event, Exit::Left | Exit::Right)),
                Some(Node(Room::Monster, Exit::Right)),
                None,
                None,
            ],
            [
                Some(Node(Room::BurningElite1, Exit::Right)),
                Some(Node(Room::Monster, Exit::Right)),
                Some(Node(Room::Event, Exit::Straight)),
                None,
                Some(Node(Room::Monster, Exit::Left)),
                Some(Node(Room::Event, Exit::Straight)),
                None,
            ],
            [
                None,
                Some(Node(Room::Event, Exit::Left)),
                Some(Node(Room::Elite, Exit::Left | Exit::Straight)),
                Some(Node(Room::Campfire, Exit::Straight)),
                None,
                Some(Node(Room::Campfire, Exit::Right)),
                None,
            ],
            [
                Some(Node(Room::Elite, Exit::Straight)),
                Some(Node(Room::Monster, Exit::Left)),
                Some(Node(Room::Shop, Exit::Straight)),
                Some(Node(Room::Monster, Exit::Right)),
                None,
                None,
                Some(Node(Room::Elite, Exit::Left)),
            ],
            [
                Some(Node(Room::Campfire, Exit::Right)),
                None,
                Some(Node(Room::Campfire, Exit::Right)),
                None,
                Some(Node(Room::Campfire, Exit::Left)),
                Some(Node(Room::Campfire, Exit::Left)),
                None,
            ],
        ])
    });

    static MAP_1SLAYTHESPIRE: Lazy<Map> = Lazy::new(|| {
        Map([
            [
                None,
                Some(Node(Room::Monster, Exit::Right)),
                None,
                Some(Node(Room::Monster, Exit::Right)),
                Some(Node(Room::Monster, Exit::Right)),
                Some(Node(Room::Monster, Exit::Right)),
                None,
            ],
            [
                None,
                None,
                Some(Node(Room::Monster, Exit::Straight)),
                None,
                Some(Node(Room::Event, Exit::Left | Exit::Right)),
                Some(Node(Room::Shop, Exit::Straight)),
                Some(Node(Room::Shop, Exit::Left | Exit::Straight)),
            ],
            [
                None,
                None,
                Some(Node(Room::Event, Exit::Left)),
                Some(Node(Room::Monster, Exit::Straight)),
                None,
                Some(Node(Room::Event, Exit::Left | Exit::Straight)),
                Some(Node(Room::Monster, Exit::Straight)),
            ],
            [
                None,
                Some(Node(Room::Event, Exit::Straight)),
                None,
                Some(Node(Room::Monster, Exit::Straight)),
                Some(Node(Room::Monster, Exit::Left | Exit::Straight)),
                Some(Node(Room::Event, Exit::Left)),
                Some(Node(Room::Monster, Exit::Left)),
            ],
            [
                None,
                Some(Node(Room::Monster, Exit::Straight)),
                None,
                Some(Node(Room::Monster, Exit::Left | Exit::Right)),
                Some(Node(Room::Event, Exit::Straight)),
                Some(Node(Room::Monster, Exit::Left)),
                None,
            ],
            [
                None,
                Some(Node(Room::Campfire, Exit::Straight)),
                Some(Node(Room::Elite, Exit::Right)),
                None,
                Some(Node(
                    Room::Campfire,
                    Exit::Left | Exit::Straight | Exit::Right,
                )),
                None,
                None,
            ],
            [
                None,
                Some(Node(Room::Elite, Exit::Right)),
                None,
                Some(Node(Room::Monster, Exit::Left)),
                Some(Node(Room::Elite, Exit::Left | Exit::Straight)),
                Some(Node(Room::Event, Exit::Straight)),
                None,
            ],
            [
                None,
                None,
                Some(Node(Room::Campfire, Exit::Straight)),
                Some(Node(Room::Monster, Exit::Left)),
                Some(Node(Room::Campfire, Exit::Left)),
                Some(Node(Room::Monster, Exit::Right)),
                None,
            ],
            [
                None,
                None,
                Some(Node(
                    Room::Treasure,
                    Exit::Left | Exit::Straight | Exit::Right,
                )),
                Some(Node(Room::Treasure, Exit::Straight)),
                None,
                None,
                Some(Node(Room::Treasure, Exit::Straight)),
            ],
            [
                None,
                Some(Node(Room::Monster, Exit::Straight)),
                Some(Node(Room::Campfire, Exit::Straight)),
                Some(Node(Room::Event, Exit::Left | Exit::Right)),
                None,
                None,
                Some(Node(Room::Monster, Exit::Left)),
            ],
            [
                None,
                Some(Node(Room::Monster, Exit::Left | Exit::Right)),
                Some(Node(Room::Monster, Exit::Right)),
                None,
                Some(Node(Room::Event, Exit::Straight)),
                Some(Node(Room::BurningElite4, Exit::Straight)),
                None,
            ],
            [
                Some(Node(Room::Monster, Exit::Right)),
                None,
                Some(Node(Room::Event, Exit::Right)),
                Some(Node(Room::Monster, Exit::Straight)),
                Some(Node(Room::Event, Exit::Left)),
                Some(Node(Room::Monster, Exit::Straight)),
                None,
            ],
            [
                None,
                Some(Node(Room::Monster, Exit::Straight)),
                None,
                Some(Node(
                    Room::Monster,
                    Exit::Left | Exit::Straight | Exit::Right,
                )),
                None,
                Some(Node(Room::Monster, Exit::Right)),
                None,
            ],
            [
                None,
                Some(Node(Room::Event, Exit::Straight)),
                Some(Node(Room::Shop, Exit::Right)),
                Some(Node(Room::Monster, Exit::Right)),
                Some(Node(Room::Monster, Exit::Straight | Exit::Right)),
                None,
                Some(Node(Room::Monster, Exit::Left)),
            ],
            [
                None,
                Some(Node(Room::Campfire, Exit::Right)),
                None,
                Some(Node(Room::Campfire, Exit::Straight)),
                Some(Node(Room::Campfire, Exit::Left)),
                Some(Node(Room::Campfire, Exit::Left)),
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

    #[test]
    fn test_empty_map_display() {
        assert!(Map::default().to_string().trim().is_empty());
    }
}
