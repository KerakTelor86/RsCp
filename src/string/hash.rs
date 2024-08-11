use crate::math::modint::ModInt64;
use crate::misc::fluent::FluentIterator;
use crate::rand::get_default_rand;
use crate::rand::traits::RandNextRanged;
use std::array;
use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};
use std::ops::{Add, AddAssign, Sub, SubAssign};

const HASH_MOD: i64 = (1i64 << 61) - 1;
const HASH_MUL_MIN: i64 = 1i64 << 17;
const HASH_MUL_MAX: i64 = 1i64 << 27;
type ModIntHash = ModInt64<HASH_MOD>;

pub struct Hasher<const N: usize>([Vec<ModIntHash>; N]);

impl<const N: usize> Hasher<N> {
    pub fn new(max_len: usize, muls: [i64; N]) -> Self {
        let muls = muls.map(ModIntHash::new);

        Self(array::from_fn(|idx| {
            (0..=max_len)
                .running_fold(ModIntHash::new(1), |acc, _| acc * muls[idx])
                .collect()
        }))
    }

    pub fn with_random_mul(max_len: usize) -> Self {
        let mut rand = get_default_rand();
        Self::new(
            max_len,
            array::from_fn(|_| rand.next_ranged(HASH_MUL_MIN, HASH_MUL_MAX)),
        )
    }
}

pub trait RollingHasher<const N: usize>: Sized {
    fn get_mul(&self, idx: usize, len: usize) -> ModIntHash;

    fn hash(&self, s: &str) -> RollingHash<N, Self> {
        let mut res = RollingHash::new(self);
        for &c in s.as_bytes() {
            for i in 0..N {
                res.hashes[i] = res.hashes[i] * self.get_mul(i, 1)
                    + ModIntHash::new(c as i64);
            }
        }
        res.len = s.len();
        return res;
    }
}

impl<const N: usize> RollingHasher<N> for Hasher<N> {
    fn get_mul(&self, idx: usize, len: usize) -> ModIntHash {
        self.0[idx][len]
    }
}

pub struct RollingHash<'a, const N: usize, H> {
    len: usize,
    hashes: [ModIntHash; N],
    hasher: &'a H,
}

impl<'a, const N: usize, H: RollingHasher<N>> Copy for RollingHash<'a, N, H> {}

impl<'a, const N: usize, H: RollingHasher<N>> Clone for RollingHash<'a, N, H> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, const N: usize, H: RollingHasher<N>> PartialEq
    for RollingHash<'a, N, H>
{
    fn eq(&self, other: &Self) -> bool {
        self.len == other.len && self.hashes == other.hashes
    }
}
impl<'a, const N: usize, H: RollingHasher<N>> Eq for RollingHash<'a, N, H> {}

impl<'a, const N: usize, H: RollingHasher<N>> Ord for RollingHash<'a, N, H> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.len
            .cmp(&other.len)
            .then(self.hashes.cmp(&other.hashes))
    }
}

impl<'a, const N: usize, H: RollingHasher<N>> PartialOrd
    for RollingHash<'a, N, H>
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a, const N: usize, H: RollingHasher<N>> Debug for RollingHash<'a, N, H> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[len = {}, hashes = {:?}]", self.len, self.hashes)
    }
}

impl<'a, const N: usize, H: RollingHasher<N>> RollingHash<'a, N, H> {
    pub fn new(hasher: &'a H) -> Self {
        Self {
            len: 0,
            hashes: [ModIntHash::from(0i64); N],
            hasher,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

macro_rules! impl_op {
    (
        $op_assign:ident, $op:ident, $fn_assign:ident, $fn_op:ident,
        $this:ident, $other:ident, $impl:block
    ) => {
        impl<'a, const N: usize, H: RollingHasher<N>> $op_assign
            for RollingHash<'a, N, H>
        {
            fn $fn_assign(&mut $this, $other: Self) {
                $impl
            }
        }

        impl<'a, const N: usize, H: RollingHasher<N>> $op
            for RollingHash<'a, N, H>
        {
            type Output = Self;

            fn $fn_op(self, rhs: Self) -> Self::Output {
                let mut res = self;
                res.$fn_assign(rhs);
                res
            }
        }
    };
}

impl_op!(AddAssign, Add, add_assign, add, self, rhs, {
    for i in 0..N {
        self.hashes[i] =
            self.hashes[i] * self.hasher.get_mul(i, rhs.len) + rhs.hashes[i];
    }
    self.len += rhs.len;
});

impl_op!(SubAssign, Sub, sub_assign, sub, self, rhs, {
    for i in 0..N {
        self.hashes[i] = self.hashes[i]
            - rhs.hashes[i] * self.hasher.get_mul(i, self.len - rhs.len);
    }
    self.len -= rhs.len;
});

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hash() {
        const MAX_LEN: usize = 100;

        let hasher = Hasher::<8>::with_random_mul(MAX_LEN);

        let hash1 = hasher.hash("bruh");
        let hash2 = hasher.hash("moment");

        assert_eq!(hash1.len(), 4);
        assert_ne!(hash1, hash2);

        let hash3 = hasher.hash("bruh_moment");

        assert_eq!(hash1.clone() + hasher.hash("_") + hash2, hash3);

        let hash4 = hasher.hash("bruh_what_is_this???");
        let hash5 = hasher.hash("what_is_this?");

        let hash6 = hash4 - (hash1 + hasher.hash("_"));
        let hash7 = hash5 + hasher.hash("??");

        assert_eq!(hash6, hash7);

        assert_eq!(hash6.len(), 15);
        assert_eq!(hash7.len(), 15);

        assert!(hash1 < hash6);
    }
}
