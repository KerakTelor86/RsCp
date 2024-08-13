use crate::math::gcd::gcd;
use crate::misc::fluent::FluentIteratorOrd;
use crate::rand::traits::RandNextRanged;

pub struct PrimeUtil<R: RandNextRanged<i64>> {
    rand: R,
}

impl<R: RandNextRanged<i64>> PrimeUtil<R> {
    const PRIMES: [i64; 12] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];

    pub fn new(rand: R) -> Self {
        Self { rand }
    }

    pub fn is_prime(&self, x: i64) -> bool {
        if x < 2 {
            return false;
        }
        let mut r = 0;
        let mut d = x - 1;
        while d % 2 == 0 {
            d /= 2;
            r += 1
        }
        for i in Self::PRIMES {
            if x == i {
                return true;
            }
            if self.miller_rabin(x, i, d, r) {
                return false;
            }
        }
        true
    }

    pub fn factorize(&mut self, n: i64) -> Vec<i64> {
        let mut res = Vec::new();
        self.pollard_rho(n, &mut res);
        res.into_iter().sorted().collect()
    }

    fn mod_mul(&self, a: i64, b: i64, m: i64) -> i64 {
        ((a as i128 * b as i128) % m as i128) as i64
    }

    fn pow(&self, a: i64, x: i64, m: i64) -> i64 {
        if x == 0 {
            return 1;
        }
        if x % 2 == 1 {
            return self.mod_mul(a, self.pow(a, x - 1, m), m);
        }
        let temp = self.pow(a, x / 2, m);
        self.mod_mul(temp, temp, m)
    }

    fn miller_rabin(&self, n: i64, a: i64, d: i64, s: i32) -> bool {
        let mut x = self.pow(a, d, n);
        if x == 1 || x == n - 1 {
            return false;
        }
        for _ in 0..s {
            x = self.mod_mul(x, x, n);
            if x == n - 1 {
                return false;
            }
        }
        true
    }

    fn f(&self, x: i64, b: i64, n: i64) -> i64 {
        (self.mod_mul(x, x, n) + b) % n
    }

    fn rho(&mut self, n: i64) -> i64 {
        if n % 2 == 0 {
            return 2;
        }
        let b = self.rand.next_ranged(0, i64::MAX - 1);
        let mut x = self.rand.next_ranged(0, i64::MAX - 1);
        let mut y = x;
        loop {
            x = self.f(x, b, n);
            y = self.f(self.f(y, b, n), b, n);
            let d = gcd((x - y).abs(), n);
            if d != 1 {
                return d;
            }
        }
    }

    fn pollard_rho(&mut self, n: i64, res: &mut Vec<i64>) {
        if n == 1 {
            return;
        }
        if self.is_prime(n) {
            res.push(n);
        } else {
            let d = self.rho(n);
            self.pollard_rho(d, res);
            self.pollard_rho(n / d, res);
        }
    }
}

#[cfg(test)]
mod test {
    use crate::math::primes::PrimeUtil;
    use crate::rand::rand::Rand;
    use crate::rand::rng::wyrand::WyRand;

    #[test]
    fn test_is_prime() {
        let p = PrimeUtil::new(Rand::new(WyRand::new(1337)));
        assert!(!p.is_prime(0));
        assert!(p.is_prime(2));
        assert!(p.is_prime(3));
        assert!(p.is_prime(5));
        assert!(p.is_prime(7));
        assert!(p.is_prime((1i64 << 61) - 1));
        assert!(!p.is_prime(12345678));
        assert!(!p.is_prime(69420));
        assert!(!p.is_prime(21));
        assert!(!p.is_prime(42069));
        assert!(p.is_prime(694201337));
    }

    #[test]
    fn test_factorize() {
        const NUM: i64 = 42069133700000;

        let mut p = PrimeUtil::new(Rand::new(WyRand::new(1337)));
        let res = p.factorize(NUM);

        assert_eq!(res.clone().into_iter().reduce(|a, b| a * b), Some(NUM));
        assert!(res.iter().all(|&x| p.is_prime(x)));

        assert_eq!(res, [2, 2, 2, 2, 2, 5, 5, 5, 5, 5, 11, 38244667]);

        let res = p.factorize(1);
        assert_eq!(res, vec![]);
    }
}
