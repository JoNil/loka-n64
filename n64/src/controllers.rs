#[cfg(target_vendor = "nintendo64")]
use n64_sys::si;

pub struct Controllers {
    pub data: [u64; 8],
}

impl Controllers {
    #[inline]
    pub fn new() -> Controllers {
        Controllers { data: [0; 8] }
    }

    #[inline]
    pub fn update(&mut self) {
        #[cfg(target_vendor = "nintendo64")]
        si::read_controllers(&mut self.data);
    }

    #[inline]
    pub fn up_pressed(&self) -> bool {
        let buttons = (self.data[0] >> 32) as u32;

        buttons & 0b0000_1000_0000_0000_0000_0000_0000_0000 > 0
    }
}