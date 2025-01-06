use anyhow::anyhow;

#[derive(Debug, PartialEq)]
pub enum Character {
    Ironclad,
    Silent,
    Defect,
    Watcher,
}

impl TryFrom<String> for Character {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
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
            Character::try_from("Ironclad".to_string()).unwrap(),
            Character::Ironclad
        );
        assert_eq!(
            Character::try_from("Silent".to_string()).unwrap(),
            Character::Silent
        );
        assert_eq!(
            Character::try_from("Defect".to_string()).unwrap(),
            Character::Defect
        );
        assert_eq!(
            Character::try_from("Watcher".to_string()).unwrap(),
            Character::Watcher
        );
        assert_eq!(
            Character::try_from("watcher".to_string()).unwrap(),
            Character::Watcher
        );
        assert!(Character::try_from("Unknown".to_string()).is_err());
    }

    #[test]
    fn test_from_empty_string() {
        assert!(Character::try_from("".to_string()).is_err());
    }
}
