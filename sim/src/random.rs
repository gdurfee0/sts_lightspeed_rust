use std::{
    fmt::{self, Debug},
    ops::Range,
};

use crate::{game_context::GAME_CONTEXT, seed::Seed};

#[derive(Clone)]
pub struct JavaRandom {
    state: u64,
}

pub struct StsRandom {
    state0: u64,
    state1: u64,
    counter: usize,
}

impl JavaRandom {
    pub fn next(&mut self, bits: usize) -> i32 {
        self.state = (self.state.wrapping_mul(0x5DEECE66D).wrapping_add(0xB)) & ((1 << 48) - 1);
        (self.state >> (48 - bits)) as i32
    }

    pub fn next_i32(&mut self) -> i32 {
        self.next(32)
    }

    pub fn next_i32_bounded(&mut self, bound: i32) -> i32 {
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

    // Does not change the internal state of the random number generator.
    pub fn shuffle<T>(&self, slice: &mut [T]) {
        let mut self_clone = self.clone();
        for i in (1..slice.len()).rev() {
            let j = self_clone.next_i32_bounded((i + 1) as i32) as usize;
            slice.swap(i, j);
        }
    }
}

impl From<u64> for JavaRandom {
    fn from(seed: u64) -> Self {
        let state = (seed ^ 0x5DEECE66D) & ((1 << 48) - 1);
        Self { state }
    }
}

impl From<Seed> for JavaRandom {
    fn from(seed: Seed) -> Self {
        Self::from(u64::from(&seed))
    }
}

impl StsRandom {
    pub fn with_offset(offset: u64) -> Self {
        GAME_CONTEXT.seed.with_offset(offset).into()
    }

    pub fn get_counter(&self) -> usize {
        self.counter
    }

    pub fn next_u64(&mut self) -> u64 {
        let mut s1 = self.state0;
        let s0 = self.state1;
        self.state0 = s0;
        s1 ^= s1 << 23;
        self.state1 = s1 ^ s0 ^ (s1 >> 17) ^ (s0 >> 26);
        println!(
            "rng iter {}: {}",
            self.counter,
            self.state1.wrapping_add(s0)
        );
        self.counter += 1;
        self.state1.wrapping_add(s0)
    }

    pub fn next_u64_bounded(&mut self, bound: u64) -> u64 {
        loop {
            let bits = self.next_u64() >> 1;
            let value = bits % bound;
            let t = bits.wrapping_sub(value).wrapping_add(bound).wrapping_sub(1);
            if (t & (1 << 63)) == 0 {
                return value;
            }
        }
    }

    pub fn gen_range<T>(&mut self, range: Range<T>) -> T
    where
        T: Copy + TryFrom<u64> + TryInto<u64>,
        <T as TryInto<u64>>::Error: Debug,
        <T as TryFrom<u64>>::Error: Debug,
    {
        let start_u64 = range
            .start
            .try_into()
            .expect("Conversion failed for range start");
        let end_u64 = range
            .end
            .try_into()
            .expect("Conversion failed for range end");
        assert!(
            start_u64 < end_u64,
            "Invalid range: range.end must be greater than range.start"
        );
        T::try_from(self.next_u64_bounded(end_u64 - start_u64) + start_u64)
            .expect("Conversion failed for random number")
    }

    pub fn choose<'a, T>(&mut self, slice: &'a [T]) -> &'a T
    where
        T: fmt::Debug,
    {
        let result = &slice[self.gen_range(0..slice.len())];
        println!("rng choose: {:?}", result);
        result
    }

    // TODO: Review which of these are actually needed and cull the rest.
    pub fn next_i32(&mut self) -> i32 {
        self.next_u64() as i32
    }

    pub fn next_i32_bounded(&mut self, bound: i32) -> i32 {
        // WTF is this + 1 all about?
        self.next_u64_bounded(bound as u64 + 1) as i32
    }

    pub fn next_i32_range(&mut self, lbound: i32, ubound: i32) -> i32 {
        lbound + self.next_i32_bounded(ubound - lbound)
    }

    pub fn next_i64(&mut self) -> i64 {
        self.next_u64() as i64
    }

    pub fn next_i64_bounded(&mut self, bound: i64) -> i64 {
        (self.next_f64() * (bound as f64)) as i64
    }

    pub fn next_i64_range(&mut self, lbound: i64, ubound: i64) -> i64 {
        (self.next_f64() * (ubound - lbound) as f64 + lbound as f64).floor() as i64
    }

    pub fn next_f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 * 1.1102230246251565e-16
    }

    pub fn next_f32(&mut self) -> f32 {
        (self.next_u64() >> 40) as f32 * 5.9604645e-8
    }

    pub fn next_f32_bounded(&mut self, bound: f32) -> f32 {
        self.next_f32() * bound
    }

    pub fn next_f32_range(&mut self, lbound: f32, ubound: f32) -> f32 {
        lbound + self.next_f32() * (ubound - lbound)
    }

    pub fn next_bool(&mut self) -> bool {
        self.next_u64() & 1 == 1
    }

    pub fn gen_bool(&mut self, p: f32) -> bool {
        self.next_f32() < p
    }

    pub fn murmur_hash_3(mut x: u64) -> u64 {
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
            state0,
            state1,
            counter: 0,
        }
    }
}

impl From<Seed> for StsRandom {
    fn from(seed: Seed) -> Self {
        Self::from(u64::from(&seed))
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
        r.shuffle(&mut arr);
        assert_eq!(to_string(&arr), "13 0 8 7 3 11 5 1 14 2 12 6 4 10 9");
        r.shuffle(&mut arr);
        assert_eq!(to_string(&arr), "10 13 14 1 7 6 11 0 9 8 4 5 3 12 2");
        r.shuffle(&mut arr);
        assert_eq!(to_string(&arr), "12 10 9 0 1 5 6 13 2 14 3 11 7 4 8");
        for _ in 0..21 {
            r.shuffle(&mut arr);
        }
        assert_eq!(to_string(&arr), "0 1 2 3 4 5 6 7 8 9 10 11 12 13 14");
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
