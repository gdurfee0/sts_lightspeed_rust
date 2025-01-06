use std::fmt;
use std::iter::repeat_n;
use std::result::Result;

use anyhow::anyhow;

#[derive(PartialEq)]
pub struct Seed(u64);

impl TryFrom<String> for Seed {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() != 13 {
            return Err(anyhow!("Seed must be 13 characters long"));
        }
        let mut seed = 0;
        for c in value.to_uppercase().chars() {
            seed *= 35;
            seed += match c {
                '0'..='9' => c as u64 - '0' as u64,
                'A'..='N' => c as u64 - 'A' as u64 + 10,
                'P'..='Z' => c as u64 - 'A' as u64 + 9,
                _ => return Err(anyhow!("Seed must be a number in base 35 (0-9, A-N, P-Z)")),
            };
        }
        Ok(Self(seed))
    }
}

impl From<&Seed> for String {
    fn from(value: &Seed) -> String {
        let mut seed = value.0;
        let mut result = String::new();
        while seed > 0 {
            let c = match seed % 35 {
                0..=9 => (seed % 35 + '0' as u64) as u8 as char,
                10..=23 => (seed % 35 + 'A' as u64 - 10) as u8 as char,
                24..=34 => (seed % 35 + 'A' as u64 - 9) as u8 as char,
                _ => unreachable!(),
            };
            result.push(c);
            seed /= 35;
        }
        result
            .chars()
            .chain(repeat_n('0', 13 - result.len()))
            .rev()
            .collect()
    }
}

impl fmt::Debug for Seed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Seed({}) = {}", String::from(self), self.0)
    }
}

impl fmt::Display for Seed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", String::from(self))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_seed_try_from() {
        assert_eq!(
            Seed::try_from("0000000000000".to_string()).unwrap(),
            Seed(0)
        );
        assert_eq!(
            Seed::try_from("0000000000001".to_string()).unwrap(),
            Seed(1)
        );
        assert_eq!(
            Seed::try_from("0SLAYTHESPIRE".to_string()).unwrap(),
            Seed(2665621045298406349)
        );
        assert!(Seed::try_from("".to_string()).is_err());
        assert!(Seed::try_from("0".to_string()).is_err());
        assert!(Seed::try_from("00SLAYTHESPIRE".to_string()).is_err());
    }

    #[test]
    fn test_seed_to_string() {
        assert_eq!(Seed(0).to_string(), "0000000000000".to_string());
        assert_eq!(Seed(1).to_string(), "0000000000001".to_string());
        assert_eq!(
            Seed(2665621045298406349).to_string(),
            "0SLAYTHESPIRE".to_string()
        );
    }
}
