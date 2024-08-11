pub mod rand;
pub mod rng;
pub mod traits;

use rand::Rand;
use rng::wyrand::WyRand;

pub fn get_default_rand() -> Rand<WyRand, 8> {
    Rand::new(WyRand::with_time_seed())
}

#[cfg(test)]
mod test {
    use crate::rand::get_default_rand;
    use crate::rand::traits::RandNext;

    #[test]
    fn test_init_default() {
        let mut rand = get_default_rand();
        let _output: u64 = rand.next();
    }
}
