#[derive(Debug)]
pub struct SparseTable<T, F>
where
    F: Fn(T, T) -> T,
{
    nil: T,
    operation: F,
    store: Vec<Vec<T>>,
}

impl<T: Clone, F> SparseTable<T, F>
where
    F: Fn(T, T) -> T,
{
    pub fn from_iter<U: IntoIterator<Item = T>>(
        iter: U,
        nil: T,
        operation: F,
    ) -> Self {
        let source: Vec<_> = iter.into_iter().collect();
        let size = source.len();
        let lg_size = (usize::BITS - size.leading_zeros()) as usize;

        let mut store = Vec::with_capacity(lg_size);
        store.push(source);

        for i in 1..lg_size {
            let mut curr = vec![nil.clone(); size];
            let prev = &store[i - 1];
            for j in 0..=size - (1 << i) {
                curr[j] = operation(
                    prev[j].clone(),
                    prev[j + (1 << (i - 1))].clone(),
                );
            }
            store.push(curr);
        }

        Self {
            nil,
            operation,
            store,
        }
    }

    pub fn query(&self, left: usize, right: usize) -> T {
        let lg = (63 - (right - left + 1).leading_zeros()) as usize;
        (self.operation)(
            self.store[lg][left].clone(),
            self.store[lg][1 + right - (1 << lg)].clone(),
        )
    }

    pub fn query_forward(&self, start_idx: usize, num_steps: usize) -> T {
        let mut cur_idx = start_idx;
        let mut ans = self.nil.clone();
        for i in (0..self.store.len()).rev() {
            if num_steps & (1 << i) > 0 {
                ans = (self.operation)(ans, self.store[i][cur_idx].clone());
                cur_idx += 1 << i;
            }
        }
        return ans;
    }

    pub fn len(&self) -> usize {
        self.store[0].len()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::rand::rand::Rand;
    use crate::rand::rng::wyrand::WyRand;
    use crate::rand::traits::{RandNext, RandNextRanged};
    use std::cmp::max;

    #[test]
    fn test_query() {
        const LEN: usize = 100;

        let mut rand = Rand::new(WyRand::new(420691337));

        let mut vec = vec![0; LEN];
        for i in vec.iter_mut() {
            *i = rand.next();
        }
        let sparse = SparseTable::from_iter(vec.clone(), i32::MIN, max);

        for l in 0..LEN {
            for r in l..LEN {
                assert_eq!(
                    sparse.query(l, r),
                    *vec[l..r + 1].iter().max().unwrap()
                );
            }
        }
    }

    #[test]
    fn test_query_forward() {
        const LEN: usize = 100;

        let mut rand = Rand::new(WyRand::new(420691337));

        let mut vec = vec![0; LEN];
        for i in vec.iter_mut() {
            *i = rand.next_ranged(-100, 100);
        }
        let sparse = SparseTable::from_iter(vec.clone(), 0, |a, b| a + b);

        for l in 0..LEN {
            for r in l..LEN {
                assert_eq!(
                    sparse.query_forward(l, r - l + 1),
                    vec[l..r + 1].iter().sum()
                );
            }
        }
    }
}
