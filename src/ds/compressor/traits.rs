use std::vec::IntoIter;

pub trait InsertableCompressor<T> {
    fn insert(&mut self, value: T);

    fn insert_iter(&mut self, values: impl IntoIterator<Item = T>) {
        values.into_iter().for_each(|x| self.insert(x))
    }
}

pub trait UsableCompressor<T> {
    fn to_compressed(&self, value: &T) -> Option<usize>;

    // Collect to vector first, then return into_iter()
    // Workaround for https://github.com/rust-lang/rust/issues/91611
    fn to_compressed_iter<'a>(
        &'a self,
        values: impl IntoIterator<Item = &'a T>,
    ) -> IntoIter<Option<usize>>
    where
        T: 'a,
    {
        let values: Vec<_> =
            values.into_iter().map(|x| self.to_compressed(x)).collect();
        values.into_iter()
    }

    fn len(&self) -> usize;
}

pub trait Compressor<T>: InsertableCompressor<T> + UsableCompressor<T> {}
