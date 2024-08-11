use crate::misc::fluent::FluentIterator;
use std::ops::{Add, Div, Mul};

pub struct Combinatorics<
    T: Add + Div<Output = T> + Mul<Output = T> + Copy + From<usize>,
> {
    fact: Vec<T>,
    inv_fact: Vec<T>,
}

impl<T: Add + Div<Output = T> + Mul<Output = T> + Copy + From<usize>>
    Combinatorics<T>
{
    pub fn new(max_n: usize) -> Self {
        let fact: Vec<_> = (1..=max_n)
            .running_fold(T::from(1), |a, b| a * T::from(b))
            .collect();
        let inv_fact: Vec<_> = (1..=max_n)
            .rev()
            .running_fold(T::from(1) / fact.last().unwrap().clone(), |a, b| {
                a * T::from(b)
            })
            .rev()
            .collect();
        Self { fact, inv_fact }
    }

    pub fn fact(&self, n: usize) -> T {
        self.fact[n]
    }

    pub fn perm(&self, n: usize, k: usize) -> T {
        self.fact[n] * self.inv_fact[n - k]
    }

    pub fn comb(&self, n: usize, k: usize) -> T {
        self.fact[n] * self.inv_fact[n - k] * self.inv_fact[k]
    }
}

#[cfg(test)]
mod test {
    use crate::math::combinatorics::Combinatorics;
    use crate::math::modint::ModInt32;

    #[test]
    fn test_all() {
        type ModInt = ModInt32<998244353>;
        let comb = Combinatorics::<ModInt>::new(50);

        assert_eq!(comb.comb(50, 17), ModInt::new(697093158));
        assert_eq!(comb.perm(50, 0), ModInt::new(1));
        assert_eq!(comb.fact(42), ModInt::new(966786798));
    }
}
