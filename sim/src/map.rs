use std::cmp::Ordering;
use std::fmt;

use bitflags::bitflags;

use crate::act::Act;
use crate::random::StsRandom;

const COLUMN_COUNT: usize = 7;
const COLUMN_MAX: usize = COLUMN_COUNT - 1;
const ROW_COUNT: usize = 15;
const PATH_DENSITY: usize = 6;

#[derive(Default)]
pub struct Map([[Option<Node>; COLUMN_COUNT]; ROW_COUNT]);

#[derive(Debug)]
pub struct Node(pub Room, pub Exit);

#[derive(Debug)]
pub enum Room {
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

bitflags! {
    #[derive(Clone, Copy, Debug, Default, PartialEq)]
    pub struct Exit: u8 {
        const Left     = 0b100;
        const Straight = 0b010;
        const Right    = 0b001;
    }
}

#[derive(Debug)]
pub struct NodeBuilder {
    room: Option<Room>,
    exit: Exit,
    entrance_cols: Vec<usize>,
}

impl NodeBuilder {
    pub fn set_room(mut self, room: Room) -> Self {
        self.room = Some(room);
        self
    }

    pub fn add_exit(mut self, exit: Exit) -> Self {
        self.exit |= exit;
        self
    }

    pub fn add_entrance_col(mut self, entrance_col: usize) -> Self {
        self.entrance_cols.push(entrance_col);
        self
    }

    pub fn leftmost_entrance_col(&self) -> Option<usize> {
        self.entrance_cols.iter().min().copied()
    }

    pub fn rightmost_entrance_col(&self) -> Option<usize> {
        self.entrance_cols.iter().max().copied()
    }

    pub fn build(self) -> Option<Node> {
        self.room.map(|room| Node(room, self.exit))
    }
}

impl Default for NodeBuilder {
    fn default() -> Self {
        Self {
            room: None,
            exit: Exit::empty(),
            entrance_cols: Vec::with_capacity(PATH_DENSITY),
        }
    }
}

impl fmt::Display for NodeBuilder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let entrance_cols = self.entrance_cols.iter().map(|&col| col.to_string()).chain(
            std::iter::repeat("-".to_string()).take(PATH_DENSITY - self.entrance_cols.len()),
        );
        write!(f, "{}", entrance_cols.collect::<String>())
    }
}

pub struct MapBuilder {
    nodes: [[Option<NodeBuilder>; COLUMN_COUNT]; ROW_COUNT],
    sts_random: StsRandom,
}

impl fmt::Display for MapBuilder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.nodes.iter().rev() {
            for maybe_node in row.iter() {
                match maybe_node {
                    Some(node) => {
                        write!(f, "{} ", node)?;
                    }
                    None => {
                        write!(f, "       ")?;
                    }
                }
            }
            if row.as_ptr() != self.nodes[0].as_ptr() {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

impl MapBuilder {
    pub fn for_act(act: Act) -> Self {
        let offset = if act.0 == 1 {
            1
        } else {
            act.0 * (100 * (act.0 - 1))
        };
        Self {
            nodes: Default::default(),
            sts_random: StsRandom::with_offset(offset as u64),
        }
    }

    pub fn create_paths(&mut self) {
        let first_col = self.sts_random.gen_range(0..COLUMN_COUNT);
        self.create_paths_step(first_col);
        for i in 1..PATH_DENSITY {
            let mut col = self.sts_random.gen_range(0..COLUMN_COUNT);
            while col == first_col && i == 1 {
                col = self.sts_random.gen_range(0..COLUMN_COUNT);
            }
            self.create_paths_step(col);
        }
    }

    fn prune_bottom_row(&mut self) {
        let mut row1_seen = [false; COLUMN_COUNT];
        for col in 0..COLUMN_COUNT {
            let mut keep = Exit::empty();
            if let Some(node) = self.nodes[0][col].as_ref() {
                if col > 0 && node.exit.contains(Exit::Left) && !row1_seen[col - 1] {
                    keep |= Exit::Left;
                    row1_seen[col - 1] = true;
                }
                if node.exit.contains(Exit::Straight) && !row1_seen[col] {
                    keep |= Exit::Straight;
                    row1_seen[col] = true;
                }
                if col < COLUMN_MAX && node.exit.contains(Exit::Right) && !row1_seen[col + 1] {
                    keep |= Exit::Right;
                    row1_seen[col + 1] = true;
                }
            }
            self.nodes[0][col] = if keep != Exit::empty() {
                Some(NodeBuilder::default().add_exit(keep))
            } else {
                None
            };
        }
    }

    fn create_paths_step(&mut self, mut col: usize) {
        for row in 0..(ROW_COUNT - 1) {
            let (exit, next_col) = self.propose_exit(col);
            let (exit, next_col) = self.avoid_small_cycles(row, col, exit, next_col);
            let (exit, next_col) = self.prevent_crossed_paths(row, col, exit, next_col);
            self.nodes[row][col] = Some(
                self.nodes[row][col]
                    .take()
                    .unwrap_or_default()
                    .add_exit(exit),
            );
            self.nodes[row + 1][next_col] = Some(
                self.nodes[row + 1][next_col]
                    .take()
                    .unwrap_or_default()
                    .add_entrance_col(col),
            );
            col = next_col;
        }
        self.nodes[ROW_COUNT - 1][col] = Some(
            self.nodes[ROW_COUNT - 1][col]
                .take()
                .unwrap_or_default()
                .add_exit(match col {
                    0..=2 => Exit::Right,
                    3 => Exit::Straight,
                    _ => Exit::Left,
                }),
        );
    }

    fn propose_exit(&mut self, col: usize) -> (Exit, usize) {
        match col {
            0 => *self
                .sts_random
                .choose(&[(Exit::Straight, 0), (Exit::Right, 1)]),
            1..COLUMN_MAX => *self.sts_random.choose(&[
                (Exit::Left, col - 1),
                (Exit::Straight, col),
                (Exit::Right, col + 1),
            ]),
            COLUMN_MAX => *self
                .sts_random
                .choose(&[(Exit::Left, COLUMN_MAX - 1), (Exit::Straight, COLUMN_MAX)]),
            _ => unreachable!(),
        }
    }

    fn avoid_small_cycles(
        &mut self,
        row: usize,
        col: usize,
        mut exit: Exit,
        mut next_col: usize,
    ) -> (Exit, usize) {
        if row == 0 {
            return (exit, next_col);
        }
        if let Some(dest_node) = self.nodes[row + 1][next_col].as_ref() {
            let left_clamp = match col {
                0 => (Exit::Straight, 0),
                _ => (Exit::Left, col - 1),
            };
            let left_bounce = match col {
                0 => (Exit::Right, 1),
                _ => (Exit::Left, col - 1),
            };
            let straight = (Exit::Straight, col);
            let right_clamp = match col {
                COLUMN_MAX => (Exit::Straight, COLUMN_MAX),
                _ => (Exit::Right, col + 1),
            };
            let right_bounce = match col {
                COLUMN_MAX => (Exit::Left, COLUMN_MAX - 1),
                _ => (Exit::Right, col + 1),
            };
            let node = self.nodes[row][col]
                .as_ref()
                .expect("Node already populated");
            for &entrance in &dest_node.entrance_cols {
                if entrance == col {
                    continue;
                }
                let other_node = self.nodes[row][entrance]
                    .as_ref()
                    .expect("Node already populated");
                if Self::entrances_collide(node, other_node, entrance >= row) {
                    (exit, next_col) = match next_col.cmp(&col) {
                        Ordering::Greater => *self.sts_random.choose(&[left_clamp, straight]),
                        Ordering::Equal => {
                            *self
                                .sts_random
                                .choose(&[left_bounce, straight, right_bounce])
                        }
                        Ordering::Less => *self.sts_random.choose(&[straight, right_clamp]),
                    };
                }
            }
        }
        (exit, next_col)
    }

    fn prevent_crossed_paths(
        &self,
        row: usize,
        col: usize,
        exit: Exit,
        next_col: usize,
    ) -> (Exit, usize) {
        match exit {
            Exit::Left => {
                if self.nodes[row]
                    .get(col - 1)
                    .and_then(|node| node.as_ref().map(|n| &n.exit))
                    .map_or(false, |exit| exit.contains(Exit::Right))
                {
                    (Exit::Straight, col)
                } else {
                    (exit, next_col)
                }
            }
            Exit::Right => {
                if self.nodes[row]
                    .get(col + 1)
                    .and_then(|node| node.as_ref().map(|n| &n.exit))
                    .map_or(false, |exit| exit.contains(Exit::Left))
                {
                    (Exit::Straight, col)
                } else {
                    (exit, next_col)
                }
            }
            _ => (exit, next_col),
        }
    }

    fn entrances_collide(node: &NodeBuilder, other_node: &NodeBuilder, flip: bool) -> bool {
        if flip {
            node.rightmost_entrance_col() == other_node.leftmost_entrance_col()
        } else {
            other_node.rightmost_entrance_col() == node.leftmost_entrance_col()
        }
    }

    pub fn build(mut self) -> Map {
        self.create_paths();
        self.prune_bottom_row();
        Map(self
            .nodes
            .iter_mut()
            .map(|row| {
                row.iter_mut()
                    .map(|maybe_node| {
                        maybe_node
                            .take()
                            .and_then(|node| node.set_room(Room::Monster).build())
                    })
                    .collect::<Vec<Option<Node>>>()
            })
            .collect::<Vec<Vec<Option<Node>>>>()
            .into_iter()
            .map(|row| row.try_into().unwrap())
            .collect::<Vec<[Option<Node>; COLUMN_COUNT]>>()
            .try_into()
            .unwrap())
    }
}

impl fmt::Display for Room {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Room::BurningElite1 => " 1 ",
                Room::BurningElite2 => " 2 ",
                Room::BurningElite3 => " 3 ",
                Room::BurningElite4 => " 4 ",
                Room::Campfire => " R ",
                Room::Elite => " E ",
                Room::Event => " ? ",
                Room::Monster => " M ",
                Room::Shop => " $ ",
                Room::Treasure => " T ",
            }
        )
    }
}

impl fmt::Display for Exit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self.bits() {
                0b001 => r"  /",
                0b010 => r" | ",
                0b011 => r" |/",
                0b100 => r"\  ",
                0b101 => r"\ /",
                0b110 => r"\| ",
                0b111 => r"\|/",
                _ => unreachable!(),
            }
        )
    }
}

impl Node {
    fn as_u8(&self) -> u8 {
        (self.1.bits() << 4)
            | match self.0 {
                Room::BurningElite1 => 0x1,
                Room::BurningElite2 => 0x2,
                Room::BurningElite3 => 0x3,
                Room::BurningElite4 => 0x4,
                Room::Campfire => 0x5,
                Room::Elite => 0x6,
                Room::Event => 0x7,
                Room::Monster => 0x8,
                Room::Shop => 0x9,
                Room::Treasure => 0xa,
            }
    }
}

impl fmt::Debug for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Map(")?;
        for row in self.0.iter().rev() {
            for maybe_node in row.iter() {
                write!(
                    f,
                    "{:02x}",
                    maybe_node.as_ref().map(|node| node.as_u8()).unwrap_or(0u8)
                )?;
            }
        }
        write!(f, ")")
    }
}

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
    use std::time::Instant;

    use crate::seed::Seed;

    use super::*;

    use base64::engine::general_purpose::STANDARD;
    use base64::Engine;
    use once_cell::sync::Lazy;

    static MAP_0SLAYTHESPIRE: Lazy<Map> = Lazy::new(|| {
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

    #[test]
    fn test_map_debug() {
        assert_eq!(
            format!("{:?}", *MAP_0SLAYTHESPIRE),
            "Map(1500150045450026482918000046004766250015001118270048270027284557180000350038272500002a5a1a00004a002829680000004828380047001800252645005800002800475827000027003827280000271828294748003800380000680028002800002800)"
        );
        assert_eq!(
            format!("{:?}", *MAP_1SLAYTHESPIRE),
            "Map(00150025454500002719183800480028007800180018001728472800005818002724000028255700004800007a2a00002a0000254845180000160048662700002516007500000028005827480000270028684748000047280067280000280057296900180018181800)"
        );
    }

    impl Map {
        fn exits(&self) -> [[u8; COLUMN_COUNT]; ROW_COUNT - 1] {
            let mut exits = [[0; COLUMN_COUNT]; ROW_COUNT - 1];
            for (row, row_nodes) in self.0.iter().enumerate().take(ROW_COUNT - 1) {
                for (col, maybe_node) in row_nodes.iter().enumerate() {
                    if let Some(node) = maybe_node {
                        exits[row][col] = node.1.bits();
                    }
                }
            }
            exits
        }

        fn exits_compact(&self) -> String {
            STANDARD.encode(
                self.exits()
                    .as_flattened()
                    .chunks(21)
                    .map(|chunk| chunk.iter().fold(0, |acc, &exit| (acc << 3) | exit as u64))
                    .flat_map(|val64| val64.to_be_bytes())
                    .collect::<Vec<u8>>(),
            )
        }
    }

    #[test]
    fn test_connection_graph() {
        let map = MapBuilder {
            nodes: Default::default(),
            sts_random: StsRandom::from(2),
        }
        .build();
        // base64 encode the exit vector to make it easier to compare
        assert_eq!(
            map.exits(),
            [
                [0, 6, 0, 1, 0, 0, 0,],
                [1, 2, 0, 0, 6, 0, 0,],
                [0, 2, 0, 6, 5, 0, 0,],
                [0, 6, 4, 5, 0, 1, 0,],
                [1, 1, 2, 0, 1, 0, 4,],
                [0, 4, 3, 0, 0, 3, 0,],
                [2, 0, 5, 2, 0, 4, 2,],
                [2, 4, 0, 6, 4, 0, 4,],
                [1, 0, 2, 6, 0, 1, 0,],
                [0, 1, 2, 2, 0, 0, 4,],
                [0, 0, 6, 4, 0, 4, 0,],
                [0, 2, 7, 0, 4, 0, 0,],
                [0, 3, 1, 2, 0, 0, 0,],
                [0, 1, 2, 5, 0, 0, 0,],
            ]
        );

        let map = MapBuilder {
            nodes: Default::default(),
            sts_random: StsRandom::from(3),
        }
        .build();
        assert_eq!(
            map.exits(),
            [
                [2, 0, 1, 1, 0, 0, 2,],
                [1, 0, 0, 2, 6, 0, 4,],
                [0, 4, 0, 3, 1, 2, 0,],
                [1, 0, 0, 2, 2, 7, 0,],
                [0, 1, 0, 1, 3, 2, 2,],
                [0, 0, 4, 0, 2, 3, 2,],
                [0, 4, 0, 0, 6, 4, 6,],
                [2, 0, 0, 4, 6, 4, 4,],
                [1, 0, 1, 1, 1, 2, 0,],
                [0, 4, 0, 1, 1, 3, 0,],
                [1, 0, 0, 0, 2, 2, 4,],
                [0, 1, 0, 0, 2, 7, 0,],
                [0, 0, 1, 0, 6, 3, 2,],
                [0, 0, 0, 6, 4, 1, 2,],
            ]
        );

        let map = MapBuilder {
            nodes: Default::default(),
            sts_random: StsRandom::from(3),
        }
        .build();
        assert_eq!(
            map.exits_compact(),
            "IEgIgLCCBlAQEuAQWkBAmggGmQE0hBJQCAlggBKAgLgAAAACM0ANCg=="
        );
    }

    const TEST_VECTORS: &str = include_str!("map-exit-data.txt");

    #[test]
    fn test_connection_graph_test_vectors() {
        let now = Instant::now();
        let maps: Vec<Map> = (2..1000002)
            .map(|i| {
                MapBuilder {
                    nodes: Default::default(),
                    sts_random: StsRandom::from(i),
                }
                .build()
            })
            .collect();
        println!("Time taken to generate 1M maps: {:?}", now.elapsed());
        for (i, vector) in TEST_VECTORS.lines().enumerate() {
            assert_eq!(maps[i].exits_compact(), vector);
        }
    }
}
