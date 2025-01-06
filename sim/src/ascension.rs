use anyhow::anyhow;

#[derive(Debug, PartialEq)]
pub struct Ascension(u8);

impl TryFrom<String> for Ascension {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let ascension = value.parse()?;
        if ascension > 20 {
            Err(anyhow!("Ascension level must be between 0 and 20"))
        } else {
            Ok(Self(ascension))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ascension_try_from() {
        assert_eq!(Ascension::try_from("0".to_string()).unwrap(), Ascension(0));
        assert_eq!(
            Ascension::try_from("20".to_string()).unwrap(),
            Ascension(20)
        );
        assert!(Ascension::try_from("21".to_string()).is_err());
        assert!(Ascension::try_from("".to_string()).is_err());
        assert!(Ascension::try_from("-1".to_string()).is_err());
    }
}
