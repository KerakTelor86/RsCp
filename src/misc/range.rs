use std::ops::{Add, Range, RangeInclusive, Sub};

pub trait RangeWrapper<T> {
    fn half_open_bounds(&self) -> (T, T);
    fn closed_bounds(&self) -> (T, T);
}

impl<T: Copy + Sub<Output = T> + From<u8>> RangeWrapper<T> for Range<T> {
    fn half_open_bounds(&self) -> (T, T) {
        (self.start, self.end)
    }
    fn closed_bounds(&self) -> (T, T) {
        (self.start, self.end - T::from(1))
    }
}

impl<T: Copy + Add<Output = T> + From<u8>> RangeWrapper<T>
    for RangeInclusive<T>
{
    fn half_open_bounds(&self) -> (T, T) {
        (self.start().clone(), self.end().clone() + T::from(1))
    }
    fn closed_bounds(&self) -> (T, T) {
        (self.start().clone(), self.end().clone())
    }
}

#[cfg(test)]
mod test {
    use crate::misc::range::RangeWrapper;

    #[test]
    fn test_ranges() {
        assert_eq!((0..5).half_open_bounds(), (0, 5));
        assert_eq!((0..5).closed_bounds(), (0, 4));
        assert_eq!((0..=5).half_open_bounds(), (0, 6));
        assert_eq!((0..=5).closed_bounds(), (0, 5));
    }
}
