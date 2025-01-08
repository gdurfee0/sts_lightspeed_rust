use anyhow::anyhow;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Ascension(pub u8);

impl TryFrom<&str> for Ascension {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse::<u8>()?.try_into()
    }
}

impl TryFrom<u8> for Ascension {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value > 20 {
            Err(anyhow!("Ascension level must be between 0 and 20"))
        } else {
            Ok(Self(value))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ascension_try_from() {
        assert_eq!(Ascension::try_from("0").unwrap(), Ascension(0));
        assert_eq!(Ascension::try_from("20").unwrap(), Ascension(20));
        assert!(Ascension::try_from("21").is_err());
        assert!(Ascension::try_from("").is_err());
        assert!(Ascension::try_from("-1").is_err());
        assert!(Ascension::try_from("ZZZ").is_err());
    }
}
