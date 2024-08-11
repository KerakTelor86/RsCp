pub trait Rng<const N: usize> {
    fn generate(&mut self) -> [u8; N];
}
