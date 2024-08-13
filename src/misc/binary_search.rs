use std::ops::*;

pub trait BinarySearchable:
    Copy
    + Ord
    + AddAssign
    + Add<Output = Self>
    + Sub<Output = Self>
    + DivAssign
    + From<u8>
{
}

impl<
        T: Copy
            + Ord
            + AddAssign
            + Add<Output = T>
            + Sub<Output = T>
            + DivAssign
            + From<u8>,
    > BinarySearchable for T
{
}

pub fn tf_last_t<T: BinarySearchable>(
    range: Range<T>,
    predicate: impl Fn(T) -> bool,
) -> Option<T> {
    let Range { start, end } = range;
    if !predicate(start) {
        return None;
    }

    let mut jump = end - start;
    let mut pos = start;
    while jump > 0.into() {
        while pos + jump < end && predicate(pos + jump) {
            pos += jump;
        }
        jump /= 2.into();
    }
    Some(pos)
}

pub fn tf_first_f<T: BinarySearchable>(
    range: Range<T>,
    predicate: impl Fn(T) -> bool,
) -> Option<T> {
    let Range { start, end } = range;
    match tf_last_t(range, predicate) {
        None => Some(start),
        Some(x) => {
            if x + 1.into() == end {
                None
            } else {
                Some(x + 1.into())
            }
        }
    }
}

pub fn ft_last_f<T: BinarySearchable>(
    range: Range<T>,
    predicate: impl Fn(T) -> bool,
) -> Option<T> {
    tf_last_t(range, |x| !predicate(x))
}

pub fn ft_first_t<T: BinarySearchable>(
    range: Range<T>,
    predicate: impl Fn(T) -> bool,
) -> Option<T> {
    tf_first_f(range, |x| !predicate(x))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bs() {
        let ff = [false, false, false, false, false, false];
        let tt = [true, true, true, true, true, true];
        let ft = [false, false, false, true, true, true];

        assert_eq!(ft_last_f(0..ff.len(), |idx| ff[idx]), Some(5));
        assert_eq!(ft_first_t(0..ff.len(), |idx| ff[idx]), None);
        assert_eq!(tf_first_f(0..ff.len(), |idx| ff[idx]), Some(0));
        assert_eq!(tf_last_t(0..ff.len(), |idx| ff[idx]), None);

        assert_eq!(tf_first_f(0..tt.len(), |idx| tt[idx]), None);
        assert_eq!(tf_last_t(0..ft.len(), |idx| tt[idx]), Some(5));

        assert_eq!(ft_last_f(0..ft.len(), |idx| ft[idx]), Some(2));
        assert_eq!(ft_first_t(0..ft.len(), |idx| ft[idx]), Some(3));
    }
}
