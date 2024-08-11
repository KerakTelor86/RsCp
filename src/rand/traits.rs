pub trait RandNext<T: Copy> {
    fn next(&mut self) -> T;
}

pub trait RandNextRanged<T: Copy>: RandNext<T> {
    /// Inclusive range
    fn next_ranged(&mut self, low: T, high: T) -> T;
}
