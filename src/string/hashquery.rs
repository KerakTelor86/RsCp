use super::hash::*;
use crate::misc::fluent::FluentIterator;
use std::ops::Range;

pub struct HashRangeQuery<'a, const N: usize, H: RollingHasher<N>> {
    prefix_hash: Vec<RollingHash<'a, N, H>>,
}

impl<'a, const N: usize, H: RollingHasher<N>> HashRangeQuery<'a, N, H> {
    pub fn new(hasher: &'a H, s: &str) -> Self {
        Self {
            prefix_hash: (0..s.len())
                .running_fold(RollingHash::new(hasher), |acc, idx| {
                    acc + hasher.hash(&s[idx..=idx])
                })
                .collect(),
        }
    }

    pub fn query(&self, range: Range<usize>) -> RollingHash<N, H> {
        let Range { start, end } = range;
        self.prefix_hash[end] - self.prefix_hash[start]
    }
}

pub struct HashRangeQueryDoubleEnded<'a, const N: usize, H: RollingHasher<N>> {
    len: usize,
    forward: HashRangeQuery<'a, N, H>,
    backward: HashRangeQuery<'a, N, H>,
}

impl<'a, const N: usize, H: RollingHasher<N>>
    HashRangeQueryDoubleEnded<'a, N, H>
{
    pub fn new(hasher: &'a H, s: &str) -> Self {
        let reversed: String = s.chars().rev().collect();
        Self {
            len: s.len(),
            forward: HashRangeQuery::new(hasher, s),
            backward: HashRangeQuery::new(hasher, &reversed),
        }
    }

    pub fn query_forward(&self, range: Range<usize>) -> RollingHash<N, H> {
        self.forward.query(range)
    }

    pub fn query_backward(&self, range: Range<usize>) -> RollingHash<N, H> {
        let Range { start, end } = range;
        let right = self.len - start;
        let left = self.len - end;
        self.backward.query(left..right)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        const STR: &str = "ananas";

        let hasher = Hasher::<8>::with_random_mul(20);
        let ds = HashRangeQueryDoubleEnded::new(&hasher, STR);

        assert_eq!(ds.query_forward(0..3), ds.query_backward(2..5));
    }
}
