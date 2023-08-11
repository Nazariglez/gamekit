use fastrand::Rng as RawRng;
use std::ops::RangeBounds;

pub struct Rng {
    raw: RawRng,
}

impl Rng {
    pub fn new() -> Self {
        Self { raw: RawRng::new() }
    }

    pub fn with_seed(seed: u64) -> Self {
        Self {
            raw: RawRng::with_seed(seed),
        }
    }

    pub fn gen<T: Generator>(&mut self) -> T {
        T::gen(self)
    }

    pub fn range<T: RangeGenerator>(&mut self, range: impl RangeBounds<T>) -> T {
        T::range(self, range)
    }

    pub fn seed(&self) -> u64 {
        self.raw.get_seed()
    }

    pub fn shuffle<T>(&mut self, slice: &mut [T]) {
        self.raw.shuffle(slice)
    }

    pub fn pick<I>(&mut self, iter: I) -> Option<I::Item>
    where
        I: IntoIterator,
        I::IntoIter: ExactSizeIterator,
    {
        self.raw.choice(iter)
    }
}

pub trait Generator {
    fn gen(rng: &mut Rng) -> Self;
}

macro_rules! impl_generator {
    ($($t:ty, $method:ident),*) => {
        $(
            impl Generator for $t {
                fn gen(rng: &mut Rng) -> Self {
                    rng.raw.$method()
                }
            }
        )*
    };
}

impl_generator!(f32, f32, f64, f64, bool, bool);

pub trait RangeGenerator {
    fn range(rng: &mut Rng, range: impl RangeBounds<Self>) -> Self;
}

macro_rules! impl_range_generator {
    ($($t:ty, $method:ident),*) => {
        $(
            impl RangeGenerator for $t {
                fn range(rng: &mut Rng, range: impl RangeBounds<Self>) -> Self {
                    rng.raw.$method(range)
                }
            }
        )*
    };
}

// Usage example
impl_range_generator!(
    char, char, i8, i8, i16, i16, i32, i32, i64, i64, i128, i128, isize, isize, u8, u8, u16, u16,
    u32, u32, u64, u64, u128, u128, usize, usize
);

#[cfg(test)]
mod test {
    use super::*;
    use std::ops::Range;

    #[test]
    fn test_with_seed() {
        let seed = 42;
        let mut rng1 = Rng::with_seed(seed);
        let mut rng2 = Rng::with_seed(seed);

        assert_eq!(rng1.gen::<f32>(), rng2.gen::<f32>());
        assert_eq!(rng1.gen::<f64>(), rng2.gen::<f64>());
        assert_eq!(rng1.gen::<bool>(), rng2.gen::<bool>());
    }

    #[test]
    fn test_gen_range() {
        let mut rng = Rng::new();
        let range: Range<i32> = 10..20;
        let number = rng.range(range.clone());
        assert!(range.contains(&number));
    }

    // #[test]
    // fn test_range_types() {
    //     let mut rng = Rng::new();
    //
    //     for _ in 0..100 {
    //         assert!(rng.range(0i8..=127).is_i8());
    //         assert!(rng.range(0i16..=32767).is_i16());
    //         assert!(rng.range(0i32..=2_147_483_647).is_i32());
    //         assert!(rng.range(0i64..=9_223_372_036_854_775_807).is_i64());
    //         assert!(rng.range(0u8..=255).is_u8());
    //         assert!(rng.range(0u16..=65_535).is_u16());
    //         assert!(rng.range(0u32..=4_294_967_295).is_u32());
    //         assert!(rng.range(0u64..=18_446_744_073_709_551_615).is_u64());
    //         assert!(rng.range('a'..'z').is_ascii_lowercase());
    //     }
    // }

    #[test]
    fn test_shuffle() {
        let mut rng = Rng::new();
        let mut data = [1, 2, 3, 4, 5];
        rng.shuffle(&mut data);
        assert_ne!(data, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_pick() {
        let mut rng = Rng::new();
        let data = vec![10, 20, 30, 40, 50];
        let picked = rng.pick(data.iter());
        assert!(picked.is_some());
        assert!(data.contains(picked.unwrap()));
    }
}
