use super::rng::traits::Rng;
use super::traits::{RandNext, RandNextRanged};

pub struct Rand<R, const B: usize>
where
    R: Rng<B>,
{
    rng: R,
    buffer_left: usize,
    buffer: [u8; B],
}

impl<R, const B: usize> Rand<R, B>
where
    R: Rng<B>,
{
    pub fn new(rng: R) -> Self {
        Self {
            rng,
            buffer_left: 0,
            buffer: [0; B],
        }
    }

    fn fill_buffer(&mut self) {
        self.buffer_left = B;
        self.buffer = self.rng.generate();
    }

    pub fn next_byte(&mut self) -> u8 {
        if self.buffer_left == 0 {
            self.fill_buffer();
        }
        self.buffer_left -= 1;
        return self.buffer[self.buffer_left];
    }

    pub fn next_ne_bytes<const N: usize>(&mut self) -> [u8; N] {
        let mut res = [0; N];
        for i in 0..N {
            res[i] = self.next_byte();
        }
        return res;
    }
}

impl<R, const B: usize> RandNext<bool> for Rand<R, B>
where
    R: Rng<B>,
{
    fn next(&mut self) -> bool {
        self.next_byte() < 0b10000000
    }
}

macro_rules! impl_int_rand {
    ($this:ident, $($type:ident),+) => {
        $(
            impl<R, const B: usize> RandNext<$type> for $this<R, B>
            where
                R: Rng<B>,
            {
                fn next(&mut self) -> $type {
                    $type::from_ne_bytes(self.next_ne_bytes())
                }
            }

            // Ref: https://stackoverflow.com/a/17554531/10661599
            /// Warning: For signed integers, range must be <= type::MAX
            impl<R, const B: usize> RandNextRanged<$type> for $this<R, B>
            where
                R: Rng<B>,
            {
                fn next_ranged(&mut self, low: $type, high: $type) -> $type {
                    let range = high - low + 1;
                    let buckets = $type::MAX / range;
                    let limit = buckets * range;

                    loop {
                        let x: $type = self.next();
                        if (0..=limit).contains(&x) {
                            return low + (x / buckets);
                        }
                    }
                }
            }
        )+
    };
}

macro_rules! impl_float_rand {
    ($this:ident, $(($type:ident, $derive_type:ident)),+) => {
        $(
            impl<R, const B: usize> RandNext<$type> for $this<R, B>
            where
                R: Rng<B>,
            {
                fn next(&mut self) -> $type {
                    let val: $derive_type = self.next();
                    val as $type / u64::MAX as $type
                }
            }

            impl<R, const B: usize> RandNextRanged<$type> for $this<R, B>
            where
                R: Rng<B>,
            {
                fn next_ranged(&mut self, low: $type, high: $type) -> $type {
                    let range = high - low;
                    let value: $type = self.next();
                    value * range + low
                }
            }
        )+
    };
}

impl_int_rand!(
    Rand, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, usize, u128
);
impl_float_rand!(Rand, (f32, u32), (f64, u64));

impl<R, const B: usize> Rand<R, B>
where
    R: Rng<B>,
{
    pub fn shuffle<T>(&mut self, container: &mut [T]) {
        for i in (1..container.len()).rev() {
            let idx = self.next_ranged(0, i);
            container.swap(i, idx);
        }
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::rand::rng::wyrand::WyRand;

    use super::*;

    #[test]
    fn test_distribution() {
        const TIMES: usize = 10000;
        const RANGE: usize = 10;

        const EXPECTED: usize = TIMES / RANGE;
        const TOLERANCE: usize = EXPECTED / 10; // 10%

        let mut rand = Rand::new(WyRand::new(420691337));
        let mut counts = [0; RANGE];
        for _ in 0..TIMES {
            let idx = rand.next_ranged(0, 9);
            counts[idx] += 1;
        }

        for i in 0..10 {
            let diff = usize::abs_diff(counts[i], EXPECTED);
            assert!(diff <= TOLERANCE);
        }
    }

    #[test]
    fn test_shuffle() {
        let mut rand = Rand::new(WyRand::new(420691337));

        let mut arr: Vec<i32> = (0..100).map(|_| rand.next()).collect();
        let copy = arr.clone();

        rand.shuffle(&mut arr);
        assert_ne!(arr, copy);

        let to_counts = |vec| {
            let mut counts = HashMap::new();
            for i in vec {
                *counts.entry(i).or_insert(0) += 1;
            }
            return counts;
        };

        assert_eq!(to_counts(arr), to_counts(copy));
    }
}
