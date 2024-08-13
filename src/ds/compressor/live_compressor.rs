use super::traits::*;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, Default)]
pub struct LiveCompressor<T: Eq + Hash> {
    store: HashMap<T, usize>,
}

impl<T: Eq + Hash> LiveCompressor<T> {
    pub fn new() -> Self {
        Self {
            store: HashMap::<T, usize>::new(),
        }
    }
}

impl<T: Eq + Hash> FromIterator<T> for LiveCompressor<T> {
    fn from_iter<U: IntoIterator<Item = T>>(iter: U) -> Self {
        let mut result = LiveCompressor::new();
        result.insert_iter(iter);
        result
    }
}

impl<T: Eq + Hash> InsertableCompressor<T> for LiveCompressor<T> {
    fn insert(&mut self, value: T) {
        if !self.store.contains_key(&value) {
            self.store.insert(value, self.store.len());
        }
    }
}

impl<T: Eq + Hash> UsableCompressor<T> for LiveCompressor<T> {
    fn to_compressed(&self, value: &T) -> Option<usize> {
        self.store.get(value).cloned()
    }

    fn len(&self) -> usize {
        self.store.len()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_init() {
        let nums = [2, 3, 1, 4, 5, 6, 1, 2, 7, 2, 3];

        let mut with_new = LiveCompressor::<i32>::new();
        with_new.insert_iter(nums);

        let with_from = LiveCompressor::from_iter(nums);

        assert_eq!(with_new.store, with_from.store);
    }

    #[test]
    fn test_functionality() {
        let nums = [69, 69, 420, 42, 42, 69, 42, 1337, 420, 1337];

        let compressor = LiveCompressor::from_iter(nums);
        assert_eq!(compressor.len(), 4);

        let compressed_nums: Vec<_> = compressor
            .to_compressed_iter(nums.iter())
            .flatten()
            .collect();

        assert_eq!(compressed_nums, [0, 0, 1, 2, 2, 0, 2, 3, 1, 3]);
    }
}
