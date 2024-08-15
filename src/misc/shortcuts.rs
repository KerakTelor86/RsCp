pub fn min_max<T: Ord>(a: T, b: T) -> [T; 2] {
    if a < b {
        [a, b]
    } else {
        [b, a]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_min_max() {
        let [a, b] = min_max(7, 3);
        assert_eq!(a, 3);
        assert_eq!(b, 7);
    }
}