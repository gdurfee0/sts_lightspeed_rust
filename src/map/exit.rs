use std::fmt;

use bitflags::bitflags;

bitflags! {
    #[derive(Clone, Copy, Debug, Default, PartialEq)]
    pub struct ExitBits: u8 {
        const Left  = 0b100;
        const Up    = 0b010;
        const Right = 0b001;
    }
}

impl fmt::Display for ExitBits {
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
