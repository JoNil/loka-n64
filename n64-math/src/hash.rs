use core::default::Default;
use core::hash::{BuildHasher, Hasher};

pub struct FnvHasher(u64);

impl Default for FnvHasher {
    fn default() -> FnvHasher {
        FnvHasher(0xcbf29ce484222325)
    }
}

impl Hasher for FnvHasher {
    fn write(&mut self, bytes: &[u8]) {
        let FnvHasher(mut hash) = *self;
        for byte in bytes {
            hash = hash ^ (*byte as u64);
            hash = hash.wrapping_mul(0x100000001b3);
        }
        *self = FnvHasher(hash);
    }
    fn finish(&self) -> u64 {
        self.0
    }
}

pub struct BuildFnvHasher;

impl BuildHasher for BuildFnvHasher {
    type Hasher = FnvHasher;

    fn build_hasher(&self) -> Self::Hasher {
        Default::default()
    }
}
