use std::fmt;
use std::ops::Bound::{Excluded, Included};
use std::ops::RangeBounds;

use super::java::JavaRandom;
use super::seed::Seed;

/// A pseudo-random number generator that leverages a global game context seed to produce
/// deterministic-yet-customizable random values, shuffling, and range generation.
///
/// This struct provides extensive capabilities, including:
/// - Offset-based instantiation for unique sequences derived from a shared global seed.
/// - Fine-grained generation of 32-bit and 64-bit integers, floats, and bool.
/// - Support for generating random values within specified ranges (both inclusive and exclusive).
/// - In-place shuffling (Fisher-Yates) for slice randomization.
/// - Element selection from slices for quick sampling.
/// - Weighted selection from slices based on provided probabilities.
///
/// # Basic Usage
///
/// // Create a new random generator with the specified seed.
/// let mut rng = StsRandom::from(100.into());
///
/// // Generate a random number from 1 to 6 (inclusive), akin to a dice roll.
/// let dice_roll = rng.gen_range(1..=6);
///
/// // Shuffle the contents of an array.
/// let mut arr = [1, 2, 3, 4, 5];
/// rng.shuffle(&mut arr);
///
/// // Choose a random element from the array.
/// let random_element = rng.choose(&arr);
///
/// // Weighted selection from a slice.
/// let choices = &[(1, 0.1), (2, 0.2), (3, 0.3), (4, 0.4)];
/// let weighted_choice = rng.weighted_choose(choices);
///
/// # Implementation Details
///
/// This class uses a variant of the Xorshift128+ algorithm for pseudorandom number
/// generation.
///
/// Some methods will panic if invalid ranges are provided (e.g., a start bound
/// that is greater than or equal to the end bound). Casting logic is enforced
/// when converting between `u64` and target types, and will panic on failure.
///
/// Overall, this generator is designed for scenarios requiring fast,
/// deterministic random sequences—particularly useful for simulations,
/// testing, and gaming environments.
#[derive(Debug)]
pub struct StsRandom {
    initial_seed: u64,
    state0: u64,
    state1: u64,
    counter: usize,
}

impl StsRandom {
    // For debugging only
    #[allow(dead_code)]
    pub fn get_counter(&self) -> usize {
        self.counter
    }

    // For debugging only
    #[allow(dead_code)]
    pub fn get_initial_seed(&self) -> u64 {
        self.initial_seed
    }

    /// Implements the Xorshift128+ algorithm for generating random numbers.
    fn next_u64(&mut self) -> u64 {
        let mut s1 = self.state0;
        let s0 = self.state1;
        self.state0 = s0;
        s1 ^= s1 << 23;
        self.state1 = s1 ^ s0 ^ (s1 >> 17) ^ (s0 >> 26);
        self.counter += 1;
        self.state1.wrapping_add(s0)
    }

    /// Generates a random value of type `u64` within [0, bound) using rejection sampling.
    fn next_u64_bounded(&mut self, bound: u64) -> u64 {
        loop {
            let bits = self.next_u64() >> 1;
            let value = bits % bound;
            let t = bits.wrapping_sub(value).wrapping_add(bound).wrapping_sub(1);
            if (t & (1 << 63)) == 0 {
                return value;
            }
        }
    }

    /// Advances the generator by generating one u64 and ignoring the result.
    pub fn advance(&mut self) {
        self.next_u64();
    }

    /// Generates a random value of type `T` within the specified range.
    ///
    /// The range can be either:
    /// - `start..end`, which is lower-bound inclusive and upper-bound exclusive, or
    /// - `start..=end`, which is lower-bound inclusive and upper-bound inclusive.
    ///
    /// # Panics
    ///
    /// Panics if:
    /// - The provided range is invalid (lower bound >= upper bound).
    /// - Any conversions from `u64` to `T` fail due to type constraints.
    /// - The provided range type is not supported (only `[Included, Excluded]` and
    ///   `[Included, Included]` are supported).
    ///
    /// # Examples
    ///
    /// // Generates a random number between 1 and 9 (inclusive).
    /// let rng = StsRandom::from(100.into());
    /// let random_number = rng.gen_range(1..=9);
    pub fn gen_range<R, T>(&mut self, range: R) -> T
    where
        R: RangeBounds<T>,
        T: Copy + TryFrom<u64> + TryInto<u64>,
        <T as TryInto<u64>>::Error: fmt::Debug,
        <T as TryFrom<u64>>::Error: fmt::Debug,
    {
        let (start_u64, end_u64) = match (range.start_bound(), range.end_bound()) {
            (Included(start), Excluded(end)) => {
                let s: u64 = (*start)
                    .try_into()
                    .expect("Conversion failed for range start");
                let e: u64 = (*end).try_into().expect("Conversion failed for range end");
                (s, e)
            }
            (Included(start), Included(end)) => {
                let s: u64 = (*start)
                    .try_into()
                    .expect("Conversion failed for range start");
                let e: u64 = (*end).try_into().expect("Conversion failed for range end");
                (s, e + 1)
            }
            _ => unimplemented!("Only a..b and a..=b are supported"),
        };
        assert!(
            start_u64 < end_u64,
            "Invalid range: range.end must be greater than range.start"
        );
        T::try_from(self.next_u64_bounded(end_u64 - start_u64) + start_u64)
            .expect("Conversion failed for random number")
    }

    /// Shuffles the given slice in place using the Fisher-Yates algorithm.
    pub fn shuffle<T>(&mut self, slice: &mut [T]) {
        for i in (1..slice.len()).rev() {
            slice.swap(i, self.gen_range(0..=i));
        }
    }

    /// Generates a boolean value with equal probability of being true or false.
    pub fn next_bool(&mut self) -> bool {
        self.next_u64() & 1 == 1
    }

    /// Shuffles the given slice in place using the Fisher-Yates algorithm, but via a detour
    /// through a JavaRandom instance seeded with the next u64 from this generator.
    ///
    /// I'm guessing that the generator is forked into a JavaRandom so no more than
    /// one tick is consumed of the original rng, maybe for backward compatibility with
    /// save files or something. Or perhaps they didn't yet have Fisher-Yates implemented
    /// for StsRandom at the point this was written.
    pub fn java_compat_shuffle<T>(&mut self, slice: &mut [T]) {
        JavaRandom::from(self.next_u64()).shuffle(slice);
    }

    /// Chooses an element from the given slice unfiformly at random.
    pub fn choose<'a, T>(&mut self, slice: &'a [T]) -> &'a T
    where
        T: fmt::Debug,
    {
        &slice[self.gen_range(0..slice.len())]
    }

    /// Chooses an element from the given slice randomly using weighted probabilities.
    pub fn weighted_choose<'a, T, N>(&mut self, choices: &'a [(T, N)]) -> &'a T
    where
        N: Copy + Into<f32>,
    {
        let mut choice = self.next_f32();
        for (item, weight) in choices {
            choice -= (*weight).into();
            if choice <= 0.0f32 {
                return item;
            }
        }
        &choices.last().unwrap().0
    }

    /// Samples from the given input slice, without duplicates.
    ///
    /// Fisher-Yates would be better here, but we implement it this way for consistency with the
    /// original game.
    ///
    /// TODO: For performance, consider a mutable input slice instead of returning a Vec<T>
    pub fn sample_without_replacement<T>(&mut self, choices: &[T], n: usize) -> Vec<T>
    where
        T: Copy + fmt::Debug + PartialEq,
    {
        assert!(
            n <= choices.len(),
            "Cannot sample more elements than are available in the input slice"
        );
        let mut result = Vec::with_capacity(n);
        for _ in 0..n {
            let mut choice = *self.choose(choices);
            while result.contains(&choice) {
                choice = *self.choose(choices);
            }
            result.push(choice);
        }
        result
    }

    /// Pulls a value of type `f32` uniformly at random from [0, 1) by scaling a u64.
    fn next_f32(&mut self) -> f32 {
        (self.next_u64() >> 40) as f32 * 5.9604645e-8
    }

    /// MurmurHash3 implementation for generating a 64-bit hash from a 64-bit seed.
    fn murmur_hash_3(mut x: u64) -> u64 {
        x ^= x >> 33;
        x = x.wrapping_mul(0xff51afd7ed558ccd);
        x ^= x >> 33;
        x = x.wrapping_mul(0xc4ceb9fe1a85ec53);
        x ^ (x >> 33)
    }
}

impl From<u64> for StsRandom {
    fn from(seed: u64) -> Self {
        let state0 = StsRandom::murmur_hash_3(if seed == 0 { 1u64 << 63 } else { seed });
        let state1 = StsRandom::murmur_hash_3(state0);
        Self {
            initial_seed: seed,
            state0,
            state1,
            counter: 0,
        }
    }
}

impl From<Seed> for StsRandom {
    fn from(seed: Seed) -> Self {
        Self::from(u64::from(seed))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // TODO: Move these methods out of cfg(test) if they're actually needed, or just delete them.
    impl StsRandom {
        fn next_i32(&mut self) -> i32 {
            self.next_u64() as i32
        }

        fn next_i32_bounded(&mut self, bound: i32) -> i32 {
            // WTF is this + 1 all about?
            self.next_u64_bounded(bound as u64 + 1) as i32
        }

        fn next_i32_range(&mut self, lbound: i32, ubound: i32) -> i32 {
            lbound + self.next_i32_bounded(ubound - lbound)
        }

        fn next_i64(&mut self) -> i64 {
            self.next_u64() as i64
        }

        fn next_i64_bounded(&mut self, bound: i64) -> i64 {
            (self.next_f64() * (bound as f64)) as i64
        }

        fn next_i64_range(&mut self, lbound: i64, ubound: i64) -> i64 {
            (self.next_f64() * (ubound - lbound) as f64 + lbound as f64).floor() as i64
        }

        fn next_f64(&mut self) -> f64 {
            (self.next_u64() >> 11) as f64 * 1.1102230246251565e-16
        }

        fn next_f32_bounded(&mut self, bound: f32) -> f32 {
            self.next_f32() * bound
        }

        fn next_f32_range(&mut self, lbound: f32, ubound: f32) -> f32 {
            lbound + self.next_f32() * (ubound - lbound)
        }

        fn gen_bool(&mut self, p: f32) -> bool {
            self.next_f32() < p
        }
    }

    #[test]
    fn test_sts_random_next_u64() {
        let mut r = StsRandom::from(2665621045298406349u64);
        assert_eq!(r.next_u64(), 6241938426952260625);
        assert_eq!(r.next_u64(), 16912281428050050838);
        assert_eq!(r.next_u64(), 9935128893071954383);
        assert_eq!(r.next_u64(), 10223835979718960854);
        assert_eq!(r.next_u64(), 10988809226805338205);
        for _ in 0..1000000 {
            let _ = r.next_u64();
        }
        assert_eq!(r.next_u64(), 14363862663833285939);
        assert_eq!(r.next_u64(), 1656846756039688891);
        assert_eq!(r.next_u64(), 14913490070073105064);
        assert_eq!(r.next_u64(), 7765683530522584210);
        assert_eq!(r.next_u64(), 9421501282369135542);
    }

    #[test]
    fn test_sts_random_next_u64_bounded() {
        let mut r = StsRandom::from(2665621045298406349u64);
        assert_eq!(r.next_u64_bounded(1 << 2), 0);
        assert_eq!(r.next_u64_bounded(1 << 17), 130955);
        assert_eq!(r.next_u64_bounded(1 << 32), 2057504999);
        assert_eq!(r.next_u64_bounded(1 << 47), 50937817256811);
        assert_eq!(r.next_u64_bounded(1 << 62), 882718594975281198);
        for _ in 0..1000000 {
            let _ = r.next_u64();
        }
        assert_eq!(r.next_u64_bounded(1 << 2), 1);
        assert_eq!(r.next_u64_bounded(1 << 17), 33117);
        assert_eq!(r.next_u64_bounded(1 << 32), 302181716);
        assert_eq!(r.next_u64_bounded(1 << 47), 35199026147913);
        assert_eq!(r.next_u64_bounded(1 << 62), 99064622757179867);
    }

    #[test]
    fn test_sts_random_next_i64() {
        let mut r = StsRandom::from(2665621045298406349u64);
        assert_eq!(r.next_i64(), 6241938426952260625);
        assert_eq!(r.next_i64(), -1534462645659500778);
        assert_eq!(r.next_i64(), -8511615180637597233);
        assert_eq!(r.next_i64(), -8222908093990590762);
        assert_eq!(r.next_i64(), -7457934846904213411);
        for _ in 0..1000000 {
            let _ = r.next_u64();
        }
        assert_eq!(r.next_i64(), -4082881409876265677);
        assert_eq!(r.next_i64(), 1656846756039688891);
        assert_eq!(r.next_i64(), -3533254003636446552);
        assert_eq!(r.next_i64(), 7765683530522584210);
        assert_eq!(r.next_i64(), -9025242791340416074);
    }

    #[test]
    fn test_sts_random_next_i32() {
        let mut r = StsRandom::from(2665621045298406349u64);
        assert_eq!(r.next_i32(), -1738957807);
        assert_eq!(r.next_i32(), -1940127978);
        assert_eq!(r.next_i32(), -179957297);
        assert_eq!(r.next_i32(), -989747498);
        assert_eq!(r.next_i32(), 1145331805);
        for _ in 0..1000000 {
            let _ = r.next_u64();
        }
        assert_eq!(r.next_i32(), 1615295795);
        assert_eq!(r.next_i32(), -1526660421);
        assert_eq!(r.next_i32(), 604363432);
        assert_eq!(r.next_i32(), -756652910);
        assert_eq!(r.next_i32(), -1031806026);
    }

    #[test]
    fn test_sts_random_next_f64() {
        let mut r = StsRandom::from(2665621045298406349u64);
        assert_eq!(format!("{:.16}", r.next_f64()), "0.3383761601511196");
        assert_eq!(format!("{:.16}", r.next_f64()), "0.9168166132989056");
        assert_eq!(format!("{:.16}", r.next_f64()), "0.5385844164896059");
        assert_eq!(format!("{:.16}", r.next_f64()), "0.5542352590173381");
        assert_eq!(format!("{:.16}", r.next_f64()), "0.5957045418365552");
        for _ in 0..1000000 {
            let _ = r.next_u64();
        }
        assert_eq!(format!("{:.16}", r.next_f64()), "0.7786665552705736");
        assert_eq!(format!("{:.16}", r.next_f64()), "0.0898178426186895");
        assert_eq!(format!("{:.16}", r.next_f64()), "0.8084619166657129");
        assert_eq!(format!("{:.16}", r.next_f64()), "0.4209785477313743");
        assert_eq!(format!("{:.16}", r.next_f64()), "0.5107406079209791");
    }

    #[test]
    fn test_sts_random_next_f32() {
        let mut r = StsRandom::from(2665621045298406349u64);
        assert_eq!(format!("{:.9}", r.next_f32()), "0.338376105");
        assert_eq!(format!("{:.9}", r.next_f32()), "0.916816592");
        assert_eq!(format!("{:.9}", r.next_f32()), "0.538584411");
        assert_eq!(format!("{:.9}", r.next_f32()), "0.554235220");
        assert_eq!(format!("{:.9}", r.next_f32()), "0.595704496");
        for _ in 0..1000000 {
            let _ = r.next_u64();
        }
        assert_eq!(format!("{:.9}", r.next_f32()), "0.778666496");
        assert_eq!(format!("{:.9}", r.next_f32()), "0.089817822");
        assert_eq!(format!("{:.9}", r.next_f32()), "0.808461905");
        assert_eq!(format!("{:.9}", r.next_f32()), "0.420978546");
        assert_eq!(format!("{:.9}", r.next_f32()), "0.510740578");
    }

    #[test]
    fn test_sts_random_next_i32_bounded() {
        let mut r = StsRandom::from(2665621045298406349u64);
        assert_eq!(r.next_i32_bounded(1 << 2), 2);
        assert_eq!(r.next_i32_bounded(1 << 9), 356);
        assert_eq!(r.next_i32_bounded(1 << 16), 40672);
        assert_eq!(r.next_i32_bounded(1 << 23), 2443115);
        assert_eq!(r.next_i32_bounded(1 << 30), 824311977);
        for _ in 0..1000000 {
            let _ = r.next_u64();
        }
        assert_eq!(r.next_i32_bounded(1 << 2), 4);
        assert_eq!(r.next_i32_bounded(1 << 9), 210);
        assert_eq!(r.next_i32_bounded(1 << 16), 8688);
        assert_eq!(r.next_i32_bounded(1 << 23), 3408641);
        assert_eq!(r.next_i32_bounded(1 << 30), 465577696);
    }

    #[test]
    fn test_sts_random_next_i32_range() {
        let mut r = StsRandom::from(2665621045298406349u64);
        assert_eq!(r.next_i32_range(-(1 << 1), 1 << 2), 0);
        assert_eq!(r.next_i32_range(-(1 << 8), 1 << 9), 478);
        assert_eq!(r.next_i32_range(-(1 << 15), 1 << 16), 44108);
        assert_eq!(r.next_i32_range(-(1 << 22), 1 << 23), -2563663);
        assert_eq!(r.next_i32_range(-(1 << 29), 1 << 30), 919386922);
        for _ in 0..1000000 {
            let _ = r.next_u64();
        }
        assert_eq!(r.next_i32_range(-(1 << 1), 1 << 2), 1);
        assert_eq!(r.next_i32_range(-(1 << 8), 1 << 9), -154);
        assert_eq!(r.next_i32_range(-(1 << 15), 1 << 16), 55079);
        assert_eq!(r.next_i32_range(-(1 << 22), 1 << 23), -2213837);
        assert_eq!(r.next_i32_range(-(1 << 29), 1 << 30), 854245342);
    }

    #[test]
    fn test_sts_random_next_i64_bounded() {
        let mut r = StsRandom::from(2665621045298406349u64);
        assert_eq!(r.next_i64_bounded(1 << 2), 1);
        assert_eq!(r.next_i64_bounded(1 << 17), 120168);
        assert_eq!(r.next_i64_bounded(1 << 32), 2313202454);
        assert_eq!(r.next_i64_bounded(1 << 47), 78001678312064);
        assert_eq!(r.next_i64_bounded(1 << 62), 2747202306701334528);
        for _ in 0..1000000 {
            let _ = r.next_u64();
        }
        assert_eq!(r.next_i64_bounded(1 << 2), 3);
        assert_eq!(r.next_i64_bounded(1 << 17), 11772);
        assert_eq!(r.next_i64_bounded(1 << 32), 3472317492);
        assert_eq!(r.next_i64_bounded(1 << 47), 59247463459187);
        assert_eq!(r.next_i64_bounded(1 << 62), 2355375320592283648);
    }

    #[test]
    fn test_sts_random_next_i64_range() {
        let mut r = StsRandom::from(2665621045298406349u64);
        assert_eq!(r.next_i64_range(-(1 << 1), 1 << 2), 0);
        assert_eq!(r.next_i64_range(-(1 << 16), 1 << 17), 114717);
        assert_eq!(r.next_i64_range(-(1 << 31), 1 << 32), 1322320034);
        assert_eq!(r.next_i64_range(-(1 << 46), 1 << 47), 46633773290433);
        assert_eq!(r.next_i64_range(-(1 << 61), 1 << 62), 1814960450838307840);
        for _ in 0..1000000 {
            let _ = r.next_u64();
        }
        assert_eq!(r.next_i64_range(-(1 << 1), 1 << 2), 2);
        assert_eq!(r.next_i64_range(-(1 << 16), 1 << 17), -47878);
        assert_eq!(r.next_i64_range(-(1 << 31), 1 << 32), 3060992590);
        assert_eq!(r.next_i64_range(-(1 << 46), 1 << 47), 18502451011116);
        assert_eq!(r.next_i64_range(-(1 << 61), 1 << 62), 1227219971674731520);
    }

    #[test]
    fn test_sts_random_next_f32_bounded() {
        let mut r = StsRandom::from(2665621045298406349u64);
        assert_eq!(
            format!("{:.9}", r.next_f32_bounded(1.0f32 / ((1 << 0) as f32))),
            "0.338376105"
        );
        assert_eq!(
            format!("{:.9}", r.next_f32_bounded(1.0f32 / ((1 << 3) as f32))),
            "0.114602074"
        );
        assert_eq!(
            format!("{:.9}", r.next_f32_bounded(1.0f32 / ((1 << 6) as f32))),
            "0.008415381"
        );
        assert_eq!(
            format!("{:.9}", r.next_f32_bounded(1.0f32 / ((1 << 9) as f32))),
            "0.001082491"
        );
        assert_eq!(
            format!("{:.9}", r.next_f32_bounded(1.0f32 / ((1 << 12) as f32))),
            "0.000145436"
        );
        for _ in 0..1000000 {
            let _ = r.next_u64();
        }
        assert_eq!(
            format!("{:.9}", r.next_f32_bounded(1.0f32 / ((1 << 0) as f32))),
            "0.778666496"
        );
        assert_eq!(
            format!("{:.9}", r.next_f32_bounded(1.0f32 / ((1 << 3) as f32))),
            "0.011227228"
        );
        assert_eq!(
            format!("{:.9}", r.next_f32_bounded(1.0f32 / ((1 << 6) as f32))),
            "0.012632217"
        );
        assert_eq!(
            format!("{:.9}", r.next_f32_bounded(1.0f32 / ((1 << 9) as f32))),
            "0.000822224"
        );
        assert_eq!(
            format!("{:.9}", r.next_f32_bounded(1.0f32 / ((1 << 12) as f32))),
            "0.000124693"
        );
    }

    #[test]
    fn test_sts_random_next_f32_range() {
        let mut r = StsRandom::from(2665621045298406349u64);
        let t = 1.0f32 / ((1 << 0) as f32);
        assert_eq!(
            format!("{:.9}", r.next_f32_range(t, t * 27.0)),
            "9.797779083"
        );
        let t = 1.0f32 / ((1 << 3) as f32);
        assert_eq!(
            format!("{:.9}", r.next_f32_range(t, t * 27.0)),
            "3.104653835"
        );
        let t = 1.0f32 / ((1 << 6) as f32);
        assert_eq!(
            format!("{:.9}", r.next_f32_range(t, t * 27.0)),
            "0.234424919"
        );
        let t = 1.0f32 / ((1 << 9) as f32);
        assert_eq!(
            format!("{:.9}", r.next_f32_range(t, t * 27.0)),
            "0.030097883"
        );
        let t = 1.0f32 / ((1 << 12) as f32);
        assert_eq!(
            format!("{:.9}", r.next_f32_range(t, t * 27.0)),
            "0.004025468"
        );
        for _ in 0..1000000 {
            let _ = r.next_u64();
        }
        let t = 1.0f32 / ((1 << 0) as f32);
        assert_eq!(
            format!("{:.9}", r.next_f32_range(t, t * 27.0)),
            "21.245328903"
        );
        let t = 1.0f32 / ((1 << 3) as f32);
        assert_eq!(
            format!("{:.9}", r.next_f32_range(t, t * 27.0)),
            "0.416907907"
        );
        let t = 1.0f32 / ((1 << 6) as f32);
        assert_eq!(
            format!("{:.9}", r.next_f32_range(t, t * 27.0)),
            "0.344062656"
        );
        let t = 1.0f32 / ((1 << 9) as f32);
        assert_eq!(
            format!("{:.9}", r.next_f32_range(t, t * 27.0)),
            "0.023330942"
        );
        let t = 1.0f32 / ((1 << 12) as f32);
        assert_eq!(
            format!("{:.9}", r.next_f32_range(t, t * 27.0)),
            "0.003486146"
        );
    }

    #[test]
    fn test_sts_random_next_bool() {
        let mut r = StsRandom::from(2665621045298406349u64);
        assert_eq!(
            (0..20).map(|_| r.next_bool()).collect::<Vec<_>>(),
            vec![
                true, false, true, false, true, true, false, true, false, false, false, true,
                false, false, false, true, true, false, false, true
            ]
        );
        for _ in 0..1000000 {
            let _ = r.next_u64();
        }
        assert_eq!(
            (0..20).map(|_| r.next_bool()).collect::<Vec<_>>(),
            vec![
                true, true, true, true, true, true, true, false, false, true, true, true, false,
                true, true, true, true, true, true, false
            ]
        );
    }

    #[test]
    fn test_sts_random_gen_bool() {
        let mut r = StsRandom::from(2665621045298406349u64);
        assert_eq!(
            (0..100).map(|_| r.gen_bool(0.75)).collect::<Vec<_>>(),
            vec![
                true, false, true, true, true, true, true, true, true, true, false, true, false,
                true, true, true, false, true, true, false, true, true, false, true, false, true,
                false, true, true, true, true, true, true, true, false, true, true, false, false,
                false, true, true, true, true, true, true, false, false, true, true, true, true,
                true, false, false, true, true, true, false, true, true, false, true, true, true,
                true, true, false, false, false, true, true, true, true, false, false, false, true,
                false, true, true, false, true, true, true, false, false, true, true, true, true,
                true, false, true, true, false, true, true, false, true,
            ]
        );
        assert_eq!(
            (0..100).map(|_| r.gen_bool(0.5)).collect::<Vec<_>>(),
            vec![
                true, true, true, true, true, false, true, true, false, true, false, false, false,
                false, true, true, true, true, true, false, true, true, false, false, false, false,
                false, true, false, true, false, true, false, false, false, false, true, false,
                false, false, true, true, false, true, true, true, true, true, true, false, false,
                false, false, false, true, false, true, true, true, false, true, false, false,
                false, true, false, true, true, false, false, true, true, false, false, false,
                true, false, false, false, true, false, true, false, true, false, false, true,
                true, false, false, false, false, true, false, false, true, false, false, false,
                false,
            ]
        );
        assert_eq!(
            (0..100).map(|_| r.gen_bool(0.25)).collect::<Vec<_>>(),
            vec![
                false, false, false, false, false, false, false, true, false, false, false, true,
                false, false, true, true, false, true, true, false, false, true, true, false,
                false, false, true, false, false, true, false, false, false, false, false, true,
                false, false, false, false, true, false, true, true, false, false, false, false,
                true, false, true, false, false, false, true, false, true, true, true, false, true,
                false, false, false, false, false, true, false, false, false, false, true, true,
                false, false, false, false, true, false, true, false, false, false, false, false,
                false, false, false, false, false, false, true, false, false, true, false, false,
                false, false, false,
            ]
        );
        assert_eq!(
            (0..100).map(|_| r.gen_bool(0.05)).collect::<Vec<_>>(),
            vec![
                false, true, false, false, false, false, false, false, false, false, false, false,
                false, false, false, false, true, false, false, false, false, false, false, false,
                false, true, false, false, false, false, false, false, false, false, false, false,
                false, false, false, false, false, false, false, false, false, false, false, false,
                false, false, false, false, false, false, true, false, false, false, false, false,
                false, false, false, false, false, false, false, false, false, false, false, false,
                false, false, false, false, false, false, false, false, false, false, false, false,
                false, false, false, false, false, false, false, false, false, false, false, false,
                false, false, false, false,
            ]
        );
    }
}
