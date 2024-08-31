use crate::misc::fluent::FluentIteratorOrd;

// Ref:
// https://en.wikipedia.org/wiki/Permutation#Generation_in_lexicographic_order
pub fn next_permutation<T: Ord>(slice: &mut [T]) -> bool {
    let n = slice.len();
    let Some(k) = (0..n - 1).filter(|&i| slice[i] < slice[i + 1]).last() else {
        return false;
    };
    let l = (k + 1..n).filter(|&i| slice[k] < slice[i]).last().unwrap();
    slice.swap(k, l);
    slice[k + 1..n].reverse();
    true
}

pub struct PermutationIter<T: Ord + Clone> {
    order: Vec<T>,
}

impl<T: Ord + Clone> Iterator for PermutationIter<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.order.is_empty() {
            return None;
        }
        let res = self.order.clone();
        if !next_permutation(&mut self.order) {
            self.order.clear();
        }
        Some(res)
    }
}

pub fn generate_permutations<T: Ord>(segment: &[T]) -> PermutationIter<&T> {
    PermutationIter {
        order: segment.into_iter().sorted().collect(),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_next_permutation() {
        let mut vec = vec![1, 2, 3, 4];

        assert!(next_permutation(&mut vec));
        assert_eq!(vec, vec![1, 2, 4, 3]);

        assert!(next_permutation(&mut vec));
        assert_eq!(vec, vec![1, 3, 2, 4]);

        for _ in 0..21 {
            assert!(next_permutation(&mut vec));
        }

        assert_eq!(vec, vec![4, 3, 2, 1]);
        assert!(!next_permutation(&mut vec));
    }

    #[test]
    fn test_permutation_iter() {
        let vec = [2, 1, 3];

        let expected_perm = [
            [1, 2, 3],
            [1, 3, 2],
            [2, 1, 3],
            [2, 3, 1],
            [3, 1, 2],
            [3, 2, 1],
        ];

        for (idx, perm) in generate_permutations(&vec).enumerate() {
            assert_eq!(
                perm.into_iter().cloned().collect::<Vec<_>>(),
                expected_perm[idx]
            );
        }
    }
}
