use std::ops::{Div, Mul, Rem};

pub trait Gcdable:
    Copy
    + Eq
    + Mul<Output = Self>
    + Div<Output = Self>
    + Rem<Output = Self>
    + From<u8>
{
}

impl<
        T: Copy
            + Eq
            + Mul<Output = T>
            + Div<Output = T>
            + Rem<Output = T>
            + From<u8>,
    > Gcdable for T
{
}

pub fn gcd<T: Gcdable>(a: T, b: T) -> T {
    if b == 0.into() {
        a
    } else {
        gcd(b, a % b)
    }
}

pub fn lcm<T: Gcdable>(a: T, b: T) -> T {
    a / gcd(a, b) * b
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let a = 16256;
        let b = 8816340;
        assert_eq!(gcd(a, b), 508);
        assert_eq!(lcm(a, b), 282122880);
    }
}
