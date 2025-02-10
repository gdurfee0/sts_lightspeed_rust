#[derive(Clone)]
pub struct JavaRandom {
    state: u64,
}

impl JavaRandom {
    /// Shuffles a slice in place using the Fisher-Yates algorithm.
    ///
    /// This method is consume-on-use: throughout the reference code base, it appears that
    /// the JavaRandom class is only used for shuffling, and is discarded after a shuffle.
    pub fn shuffle<T>(mut self, slice: &mut [T]) {
        for i in (1..slice.len()).rev() {
            let j = self.next_i32_bounded((i + 1) as i32) as usize;
            slice.swap(i, j);
        }
    }

    /// Pulls 32 bits from the RNG using the srand48 algorithm.
    fn next(&mut self, bits: usize) -> i32 {
        self.state = (self.state.wrapping_mul(0x5DEECE66D).wrapping_add(0xB)) & ((1 << 48) - 1);
        (self.state >> (48 - bits)) as i32
    }

    /// Uses rejection sampling to pull a bounded integer from the RNG.
    fn next_i32_bounded(&mut self, bound: i32) -> i32 {
        let mut r = self.next(31);
        let m = bound - 1;
        if (bound & m) == 0 {
            ((bound as i64 * r as i64) >> 31) as i32
        } else {
            while r.wrapping_add(m) < 0 {
                r = self.next(31);
            }
            r % bound
        }
    }
}

impl From<u64> for JavaRandom {
    fn from(seed: u64) -> Self {
        let state = (seed ^ 0x5DEECE66D) & ((1 << 48) - 1);
        Self { state }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn to_string(slice: &[i32]) -> String {
        slice
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    }

    #[test]
    fn test_shuffle() {
        let r = JavaRandom::from(2665621045298406349u64);
        let mut arr = (0..15).collect::<Vec<_>>();
        r.clone().shuffle(&mut arr);
        assert_eq!(to_string(&arr), "13 0 8 7 3 11 5 1 14 2 12 6 4 10 9");
        r.clone().shuffle(&mut arr);
        assert_eq!(to_string(&arr), "10 13 14 1 7 6 11 0 9 8 4 5 3 12 2");
        r.clone().shuffle(&mut arr);
        assert_eq!(to_string(&arr), "12 10 9 0 1 5 6 13 2 14 3 11 7 4 8");
        for _ in 0..21 {
            r.clone().shuffle(&mut arr);
        }
        assert_eq!(to_string(&arr), "0 1 2 3 4 5 6 7 8 9 10 11 12 13 14");
    }
}
