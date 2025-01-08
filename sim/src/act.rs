use anyhow::anyhow;

#[derive(Debug, PartialEq)]
pub struct Act(pub u8);

impl TryFrom<&str> for Act {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let act = value.parse()?;
        if !(1..=4).contains(&act) {
            Err(anyhow!("Act must be between 1 and 4"))
        } else {
            Ok(Self(act))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ascension_try_from() {
        assert_eq!(Act::try_from("1").unwrap(), Act(1));
        assert_eq!(Act::try_from("4").unwrap(), Act(4));
        assert!(Act::try_from("0").is_err());
        assert!(Act::try_from("5").is_err());
        assert!(Act::try_from("").is_err());
        assert!(Act::try_from("-1").is_err());
        assert!(Act::try_from("ZZZ").is_err());
    }
}
