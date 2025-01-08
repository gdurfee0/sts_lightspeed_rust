use anyhow::anyhow;

#[derive(Debug, PartialEq)]
pub enum Character {
    Ironclad,
    Silent,
    Defect,
    Watcher,
}

impl TryFrom<&str> for Character {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().chars().next() {
            Some('i') => Ok(Self::Ironclad),
            Some('s') => Ok(Self::Silent),
            Some('d') => Ok(Self::Defect),
            Some('w') => Ok(Self::Watcher),
            _ => Err(anyhow!(
                "Character options are (I)ronclad, (S)ilent, (D)efect, and (W)atcher"
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_character_try_from() {
        assert_eq!(
            Character::try_from("Ironclad").unwrap(),
            Character::Ironclad
        );
        assert_eq!(Character::try_from("Silent").unwrap(), Character::Silent);
        assert_eq!(Character::try_from("Defect").unwrap(), Character::Defect);
        assert_eq!(Character::try_from("Watcher").unwrap(), Character::Watcher);
        assert_eq!(Character::try_from("watcher").unwrap(), Character::Watcher);
        assert!(Character::try_from("Unknown").is_err());
    }

    #[test]
    fn test_from_empty_string() {
        assert!(Character::try_from("").is_err());
    }
}
