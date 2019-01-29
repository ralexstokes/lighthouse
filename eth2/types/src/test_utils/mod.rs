use rand::RngCore;

pub use rand::{prng::XorShiftRng, SeedableRng};

pub mod address;
pub mod aggregate_signature;
pub mod bitfield;
pub mod hash256;
pub mod public_key;
pub mod secret_key;
pub mod signature;

pub trait TestRandom<T>
where
    T: RngCore,
{
    fn random_for_test(rng: &mut T) -> Self;
}

impl<T: RngCore> TestRandom<T> for u64 {
    fn random_for_test(rng: &mut T) -> Self {
        rng.next_u64()
    }
}

impl<T: RngCore> TestRandom<T> for u32 {
    fn random_for_test(rng: &mut T) -> Self {
        rng.next_u32()
    }
}

impl<T: RngCore> TestRandom<T> for usize {
    fn random_for_test(rng: &mut T) -> Self {
        rng.next_u32() as usize
    }
}

impl<T: RngCore, U> TestRandom<T> for Vec<U>
where
    U: TestRandom<T>,
{
    fn random_for_test(rng: &mut T) -> Self {
        vec![
            <U>::random_for_test(rng),
            <U>::random_for_test(rng),
            <U>::random_for_test(rng),
        ]
    }
}

impl<T: RngCore, U, V> TestRandom<T> for (U, V)
where
    U: TestRandom<T>,
    V: TestRandom<T>,
{
    fn random_for_test(rng: &mut T) -> Self {
        (<U>::random_for_test(rng), <V>::random_for_test(rng))
    }
}
