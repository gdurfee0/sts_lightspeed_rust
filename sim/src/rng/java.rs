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

    fn next(&mut self, bits: usize) -> i32 {
        self.state = (self.state.wrapping_mul(0x5DEECE66D).wrapping_add(0xB)) & ((1 << 48) - 1);
        (self.state >> (48 - bits)) as i32
    }

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

    // TODO: Move this method out of cfg(test) if it's actually needed, or just delete it.
    impl JavaRandom {
        fn next_i32(&mut self) -> i32 {
            self.next(32)
        }
    }

    fn to_string(slice: &[i32]) -> String {
        slice
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    }

    #[test]
    fn test_java_random_next_i32() {
        let mut r = JavaRandom::from(2665621045298406349u64);
        assert_eq!(r.next_i32(), 1435554138);
        assert_eq!(r.next_i32(), -685876420);
        assert_eq!(r.next_i32(), 980167561);
        assert_eq!(r.next_i32(), 1620812725);
        assert_eq!(r.next_i32(), -1708755396);
        assert_eq!(r.next_i32(), -220472312);
        assert_eq!(r.next_i32(), 303297683);
        assert_eq!(r.next_i32(), 631505519);
        assert_eq!(r.next_i32(), 1207798239);
        assert_eq!(r.next_i32(), -898299774);
        for _ in 0..1000000 {
            let _ = r.next_i32();
        }
        assert_eq!(r.next_i32(), -826284903);
        assert_eq!(r.next_i32(), -13980690);
        assert_eq!(r.next_i32(), -1295521124);
        assert_eq!(r.next_i32(), -161793911);
        assert_eq!(r.next_i32(), -2051575420);
        assert_eq!(r.next_i32(), 62780344);
        assert_eq!(r.next_i32(), -458419070);
        assert_eq!(r.next_i32(), -1651388872);
        assert_eq!(r.next_i32(), -1273357138);
        assert_eq!(r.next_i32(), -1018115670);

        assert_eq!(r.next_i32_bounded(42 + (1 << 0)), 7);
        assert_eq!(r.next_i32_bounded(42 + (1 << 3)), 41);
        assert_eq!(r.next_i32_bounded(42 + (1 << 6)), 64);
        assert_eq!(r.next_i32_bounded(42 + (1 << 9)), 169);
        assert_eq!(r.next_i32_bounded(42 + (1 << 12)), 3471);
        assert_eq!(r.next_i32_bounded(42 + (1 << 15)), 7577);
        assert_eq!(r.next_i32_bounded(42 + (1 << 18)), 35786);
        assert_eq!(r.next_i32_bounded(42 + (1 << 21)), 1224367);
        assert_eq!(r.next_i32_bounded(42 + (1 << 24)), 7614339);
        assert_eq!(r.next_i32_bounded(42 + (1 << 27)), 54347671);
        for _ in 0..1000000 {
            let _ = r.next_i32();
        }
        assert_eq!(r.next_i32_bounded(42 + (1 << 0)), 27);
        assert_eq!(r.next_i32_bounded(42 + (1 << 3)), 22);
        assert_eq!(r.next_i32_bounded(42 + (1 << 6)), 70);
        assert_eq!(r.next_i32_bounded(42 + (1 << 9)), 3);
        assert_eq!(r.next_i32_bounded(42 + (1 << 12)), 128);
        assert_eq!(r.next_i32_bounded(42 + (1 << 15)), 17674);
        assert_eq!(r.next_i32_bounded(42 + (1 << 18)), 160210);
        assert_eq!(r.next_i32_bounded(42 + (1 << 21)), 1846018);
        assert_eq!(r.next_i32_bounded(42 + (1 << 24)), 13777708);
        assert_eq!(r.next_i32_bounded(42 + (1 << 27)), 108691387);
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
