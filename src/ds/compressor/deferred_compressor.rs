use super::traits::*;

#[derive(Debug, Default)]
pub struct PendingCompressor<T: Ord> {
    store: Vec<T>,
}

#[derive(Debug)]
pub struct FinalizedCompressor<T: Ord> {
    store: Vec<T>,
}

impl<T: Ord> PendingCompressor<T> {
    pub fn new() -> Self {
        Self { store: Vec::new() }
    }

    pub fn finalize(mut self) -> FinalizedCompressor<T> {
        self.store.sort_unstable();
        self.store.dedup();
        FinalizedCompressor { store: self.store }
    }
}

impl<T: Ord> FromIterator<T> for PendingCompressor<T> {
    fn from_iter<U: IntoIterator<Item = T>>(iter: U) -> Self {
        let mut result = PendingCompressor::new();
        result.insert_iter(iter);
        return result;
    }
}

impl<T: Ord> InsertableCompressor<T> for PendingCompressor<T> {
    fn insert(&mut self, value: T) {
        self.store.push(value);
    }
}

impl<T: Ord> UsableCompressor<T> for FinalizedCompressor<T> {
    fn to_compressed(&self, value: &T) -> Option<usize> {
        match self.store.binary_search(value) {
            Ok(idx) => Some(idx),
            Err(_) => None,
        }
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

        let mut with_new = PendingCompressor::<i32>::new();
        with_new.insert_iter(nums);

        let with_from = PendingCompressor::from_iter(nums);

        assert_eq!(with_new.store, with_from.store);
    }

    #[test]
    fn test_functionality() {
        let nums = [69, 69, 420, 42, 42, 1337];

        let compressor = PendingCompressor::from_iter(nums).finalize();
        assert_eq!(compressor.len(), 4);

        let compressed_nums: Vec<_> = compressor
            .to_compressed_iter(nums.iter())
            .flatten()
            .collect();

        assert_eq!(compressed_nums, [1, 1, 2, 0, 0, 3]);

        assert_eq!(compressor.to_compressed(&-1), None);
    }
}
