use super::rng::Rng;
use crate::random::{Generator, RangeGenerator};
use std::cell::RefCell;
use std::ops::RangeBounds;

thread_local! {
    static LOCAL_RNG: RefCell<Rng> = RefCell::new(Rng::new());
}

/// Returns the current global seed
pub fn seed() -> u64 {
    LOCAL_RNG.with(|rng| rng.borrow().seed())
}

/// Set a new seed for the global RNG
pub fn set_seed(seed: u64) {
    LOCAL_RNG.with(|rng| rng.replace(Rng::with_seed(seed)));
}

/// Generate a random value for T
/// booleans will be true|false while floats will be a number between 0 and 1
pub fn gen<T: Generator>() -> T {
    LOCAL_RNG.with(|rng| rng.borrow_mut().gen())
}

/// Generate a random value between the range passed
pub fn range<T: RangeGenerator>(range: impl RangeBounds<T>) -> T {
    LOCAL_RNG.with(|rng| rng.borrow_mut().range(range))
}

/// Sort randomly a slice
pub fn shuffle<T>(slice: &mut [T]) {
    LOCAL_RNG.with(|rng| rng.borrow_mut().shuffle(slice))
}

/// Pick a value randomly
pub fn pick<I>(iter: I) -> Option<I::Item>
where
    I: IntoIterator,
    I::IntoIter: ExactSizeIterator,
{
    LOCAL_RNG.with(|rng| rng.borrow_mut().pick(iter))
}
