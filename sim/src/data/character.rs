use anyhow::anyhow;

#[derive(Debug, PartialEq)]
pub struct Character {
    /// The character's starting max hit points.
    pub start_hp: u32,
}

impl TryFrom<&str> for &'static Character {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().chars().next() {
            Some('i') => Ok(&CHARACTERS[0]),
            Some('s') => Ok(&CHARACTERS[1]),
            Some('d') => Ok(&CHARACTERS[2]),
            Some('w') => Ok(&CHARACTERS[3]),
            _ => Err(anyhow!(
                "Character options are (I)ronclad, (S)ilent, (D)efect, and (W)atcher"
            )),
        }
    }
}

static CHARACTERS: &[Character] = &[
    // Ironclad
    Character { start_hp: 80 },
    // Silent
    Character { start_hp: 70 },
    // Defect
    Character { start_hp: 75 },
    // Watcher
    Character { start_hp: 72 },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_character_try_from() {
        assert_eq!(
            <&'static Character>::try_from("Ironclad").unwrap(),
            &CHARACTERS[0]
        );
        assert_eq!(
            <&'static Character>::try_from("Silent").unwrap(),
            &CHARACTERS[1]
        );
        assert_eq!(
            <&'static Character>::try_from("Defect").unwrap(),
            &CHARACTERS[2]
        );
        assert_eq!(
            <&'static Character>::try_from("Watcher").unwrap(),
            &CHARACTERS[3]
        );
        assert_eq!(
            <&'static Character>::try_from("watcher").unwrap(),
            &CHARACTERS[3]
        );
        assert!(<&'static Character>::try_from("Unknown").is_err());
        assert!(<&'static Character>::try_from("").is_err());
    }
}
