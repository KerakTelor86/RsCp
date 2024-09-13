use std::cmp::Ordering;
use std::collections::HashMap;
use std::hash::Hash;
use std::vec::IntoIter;

use crate::rand::rand::Rand;
use crate::rand::rng::traits::Rng;

// Most things here are not lazy!
// Refer to https://github.com/rust-lang/rust/issues/91611 for why

pub trait FluentIterator: Iterator + Sized
where
    Self::Item: Clone,
{
    fn running_fold<T: Clone>(
        self,
        init: T,
        predicate: impl FnMut(T, Self::Item) -> T,
    ) -> IntoIter<T>;

    fn running_reduce(
        mut self,
        predicate: impl FnMut(Self::Item, Self::Item) -> Self::Item,
    ) -> IntoIter<Self::Item> {
        let Some(init) = self.next() else {
            return Vec::new().into_iter();
        };
        self.running_fold(init, predicate)
    }

    fn group<K: Eq + Hash>(
        self,
        mut key_fn: impl FnMut(&Self::Item) -> K,
    ) -> IntoIter<(K, IntoIter<Self::Item>)> {
        let mut dict: HashMap<K, Vec<Self::Item>> = HashMap::new();
        for item in self {
            let key = key_fn(&item);
            dict.entry(key).or_default().push(item);
        }
        let res: Vec<_> = dict
            .into_iter()
            .map(|(key, value)| (key, value.into_iter()))
            .collect();
        res.into_iter()
    }

    fn group_count<K: Eq + Hash>(
        self,
        mut key_fn: impl FnMut(&Self::Item) -> K,
    ) -> IntoIter<(K, usize)> {
        let mut dict: HashMap<K, usize> = HashMap::new();
        for item in self {
            let key = key_fn(&item);
            *dict.entry(key).or_insert(0) += 1;
        }
        let res: Vec<_> = dict.into_iter().collect();
        res.into_iter()
    }

    fn shuffled<const B: usize, R: Rng<B>>(
        self,
        rand: &mut Rand<R, B>,
    ) -> IntoIter<Self::Item> {
        let mut vec: Vec<_> = self.collect();
        rand.shuffle(&mut vec);
        vec.into_iter()
    }

    fn sorted_by(
        self,
        compare: impl FnMut(&Self::Item, &Self::Item) -> Ordering,
    ) -> IntoIter<Self::Item> {
        let mut vec: Vec<_> = self.collect();
        vec.sort_by(compare);
        vec.into_iter()
    }

    fn sorted_by_key<K: Ord>(
        self,
        key_fn: impl FnMut(&Self::Item) -> K,
    ) -> IntoIter<Self::Item> {
        let mut vec: Vec<_> = self.collect();
        vec.sort_by_key(key_fn);
        vec.into_iter()
    }
}

pub trait FluentIteratorOrd: FluentIterator + Sized
where
    Self::Item: Clone + Ord,
{
    fn sorted(self) -> IntoIter<Self::Item> {
        let mut vec: Vec<_> = self.collect();
        vec.sort();
        vec.into_iter()
    }

    fn unique(self) -> IntoIter<Self::Item> {
        let mut vec: Vec<_> = self.collect();
        vec.sort();
        vec.dedup();
        vec.into_iter()
    }
}

impl<Item: Clone, Iter> FluentIterator for Iter
where
    Iter: Iterator<Item = Item>,
{
    fn running_fold<T: Clone>(
        self,
        init: T,
        mut predicate: impl FnMut(T, Self::Item) -> T,
    ) -> IntoIter<T> {
        let (low_count, maybe_high_count) = self.size_hint();
        let capacity = match maybe_high_count {
            Some(high_count) => high_count,
            _ => low_count,
        };
        let mut result = Vec::with_capacity(1 + capacity);
        result.push(init);
        for item in self {
            result.push(predicate(result.last().unwrap().clone(), item));
        }
        result.into_iter()
    }
}

impl<Item: Clone + Ord, Iter> FluentIteratorOrd for Iter where
    Iter: Iterator<Item = Item>
{
}

#[cfg(test)]
mod test {
    use crate::rand::rng::wyrand::WyRand;

    use super::*;

    #[test]
    fn test_fold_reduce() {
        let vec = vec![1, 2, 3, 4, 5];
        let res: Vec<_> =
            vec.into_iter().running_reduce(|a, b| a + b).collect();
        assert_eq!(res, vec![1, 3, 6, 10, 15]);

        let vec: Vec<i32> = vec![];
        let res: Vec<_> =
            vec.into_iter().running_reduce(|a, b| a + b).collect();
        assert_eq!(res, vec![]);
    }

    #[test]
    fn test_group() {
        let vec = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
        let expected_res = [vec![0, 3, 6], vec![1, 4, 7], vec![2, 5, 8]];
        for (key, iter) in vec.into_iter().group(|x| x % 3) {
            let group: Vec<_> = iter.collect();
            assert_eq!(group, expected_res[key]);
        }
    }

    #[test]
    fn test_group_count() {
        let vec = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
        let expected_res = [vec![0, 3, 6], vec![1, 4, 7], vec![2, 5, 8]];
        for (key, cnt) in vec.into_iter().group_count(|x| x % 3) {
            assert_eq!(cnt, expected_res[key].len());
        }
    }

    #[test]
    fn test_sort() {
        let vec = vec![0, 7, 6, 9, 3, 1, 2, 4, 5, 8];
        let expected_res = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

        let sorted: Vec<_> = vec.iter().sorted().map(|x| *x).collect();
        assert_eq!(sorted, expected_res);

        let sorted: Vec<_> =
            vec.iter().sorted_by(|a, b| a.cmp(b)).map(|x| *x).collect();
        assert_eq!(sorted, expected_res);

        let sorted: Vec<_> = vec.into_iter().sorted_by_key(|x| *x).collect();
        assert_eq!(sorted, expected_res);
    }

    #[test]
    fn test_shuffle() {
        let vec = vec![0, 1, 2, 3];
        let mut rand = Rand::new(WyRand::new(420691337));
        let res: Vec<_> = vec.clone().into_iter().shuffled(&mut rand).collect();
        assert_ne!(res, vec);
    }

    #[test]
    fn test_unique() {
        let vec = vec![2, 3, 1, 2, 2, 2, 1, 1, 1, 3, 1, 2, 3, 2, 1, 3, 4, 5, 4];
        let res: Vec<_> = vec.into_iter().unique().collect();
        assert_eq!(res, vec![1, 2, 3, 4, 5]);
    }
}
