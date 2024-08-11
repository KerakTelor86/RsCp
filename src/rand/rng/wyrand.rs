use super::traits::Rng;
use std::time::{SystemTime, UNIX_EPOCH};

// Refs:
// https://github.com/lemire/testingRNG/blob/master/source/wyrand.h
// https://github.com/wangyi-fudan/wyhash/tree/master
pub struct WyRand {
    state: u64,
}

impl WyRand {
    pub fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    pub fn with_time_seed() -> Self {
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;
        Self { state: time }
    }
}

impl Rng<8> for WyRand {
    fn generate(&mut self) -> [u8; 8] {
        self.state = self.state.wrapping_add(0xa0761d6478bd642f);
        let t = (self.state as u128)
            .wrapping_mul((self.state ^ 0xe7037ed1a0b428dbu64) as u128);
        let ret = (t.wrapping_shr(64) ^ t) as u64;
        ret.to_ne_bytes()
    }
}
