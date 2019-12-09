use core::mem;
use core::num::Wrapping;

pub struct Rng {
    seed: Wrapping<u32>,
}

impl Rng {
    #[inline]
    pub fn new_unseeded() -> Rng {
        Rng {
            seed: Wrapping(0x66126c8d),
        }
    }

    #[inline]
    pub fn next_u32(&mut self) -> u32 {
        self.seed = self.seed*Wrapping(214013) + Wrapping(2531011);
        self.seed.0
    }

    #[inline]
    pub fn next_u64(&mut self) -> u64 {
        ((self.next_u32() as u64) << 32) | (self.next_u32() as u64)
    }

    #[inline]
    pub fn next_f32(&mut self) -> f32 {
        const UPPER_MASK: u32 = 0x3F800000;
        const LOWER_MASK: u32 = 0x7FFFFF;
        let tmp = UPPER_MASK | (self.next_u32() & LOWER_MASK);
        let result: f32 = unsafe { mem::transmute(tmp) };
        result - 1.0
    }

    #[inline]
    pub fn next_f64(&mut self) -> f64 {
        const UPPER_MASK: u64 = 0x3FF0000000000000;
        const LOWER_MASK: u64 = 0xFFFFFFFFFFFFF;
        let tmp = UPPER_MASK | (self.next_u64() & LOWER_MASK);
        let result: f64 = unsafe { mem::transmute(tmp) };
        result - 1.0
    }
}