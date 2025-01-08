use std::fmt;
use std::iter::repeat_n;
use std::result::Result;

use anyhow::anyhow;

#[derive(PartialEq)]
pub struct Seed(u64);

impl Seed {
    pub fn with_offset(&self, offset: u64) -> Self {
        Self(self.0.wrapping_add(offset))
    }
}

impl TryFrom<&str> for Seed {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() != 13 {
            return Err(anyhow!("Seed must be exactly 13 characters long"));
        }
        let mut seed: u64 = 0;
        for c in value.to_uppercase().chars() {
            // check for overflow
            seed = seed
                .checked_mul(35)
                .ok_or_else(|| anyhow!("Seed is too large"))?;
            seed = seed
                .checked_add(match c {
                    '0'..='9' => c as u64 - '0' as u64,
                    'A'..='N' => c as u64 - 'A' as u64 + 10,
                    'P'..='Z' => c as u64 - 'A' as u64 + 9,
                    _ => return Err(anyhow!("Seed must be a number in base 35 (0-9, A-N, P-Z)")),
                })
                .ok_or_else(|| anyhow!("Seed is too large"))?;
        }
        Ok(Self(seed))
    }
}

impl From<&Seed> for String {
    fn from(value: &Seed) -> String {
        let mut seed = value.0;
        let mut result = String::new();
        while seed > 0 {
            let digit = seed % 35;
            let c = match digit {
                0..=9 => (digit + '0' as u64) as u8 as char,
                10..=23 => (digit + 'A' as u64 - 10) as u8 as char,
                24..=34 => (digit + 'A' as u64 - 9) as u8 as char,
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

impl From<&Seed> for u64 {
    fn from(value: &Seed) -> u64 {
        value.0
    }
}

impl From<u64> for Seed {
    fn from(value: u64) -> Self {
        Self(value)
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
        assert_eq!(Seed::try_from("0000000000000").unwrap(), Seed(0));
        assert_eq!(Seed::try_from("0000000000001").unwrap(), Seed(1));
        assert_eq!(
            Seed::try_from("0SLAYTHESPIRE").unwrap(),
            Seed(2665621045298406349)
        );
        assert!(Seed::try_from("").is_err());
        assert!(Seed::try_from("0").is_err());
        assert!(Seed::try_from("ZSLAYTHESPIRE").is_err()); // * overflow
        assert!(Seed::try_from("5G24A25UXKXFG").is_err()); // + overflow
        assert!(Seed::try_from("00SLAYTHESPIRE").is_err());
    }

    #[test]
    fn test_seed_to_string() {
        assert_eq!(Seed(0).to_string(), "0000000000000");
        assert_eq!(Seed(1).to_string(), "0000000000001");
        assert_eq!(Seed(2665621045298406349).to_string(), "0SLAYTHESPIRE");
        assert_eq!(Seed(u64::MAX).to_string(), "5G24A25UXKXFF");
    }
}
