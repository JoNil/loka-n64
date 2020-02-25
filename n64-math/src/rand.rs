use core::mem;
use core::num::Wrapping;
use spin::Mutex;

static GLOBAL_RNG: Mutex<Rng> = Mutex::new(Rng::new_unseeded());

pub struct Rng {
    seed: Wrapping<u32>,
}

impl Rng {
    #[inline]
    pub const fn new_unseeded() -> Rng {
        Rng {
            seed: Wrapping(0x66126c8d),
        }
    }

    #[inline]
    pub fn next_u32(&mut self) -> u32 {
        self.seed = self.seed * Wrapping(214013) + Wrapping(2531011);
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

#[inline]
pub fn random_u32() -> u32 {
    GLOBAL_RNG.lock().next_u32()
}

#[inline]
pub fn random_u64() -> u64 {
    GLOBAL_RNG.lock().next_u64()
}

#[inline]
pub fn random_f32() -> f32 {
    GLOBAL_RNG.lock().next_f32()
}

#[inline]
pub fn random_f64() -> f64 {
    GLOBAL_RNG.lock().next_f64()
}
