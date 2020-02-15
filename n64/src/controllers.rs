use n64_sys::si;

pub struct Controllers {
    data: [u64; 8],
}

impl Controllers {
    #[inline]
    pub fn new() -> Controllers {
        Controllers { data: [0; 8] }
    }

    #[inline]
    pub fn update(&mut self) {
        si::read_controllers(&mut self.data);
    }

    #[inline]
    pub fn x(&self) -> i8 {
        ((self.data[0] >> 8) & 0xff) as i8
    }

    #[inline]
    pub fn y(&self) -> i8 {
        (self.data[0] & 0xff) as i8
    }

    #[inline]
    pub fn a(&self) -> bool {
        self.data[0] & 0x8000_0000 > 0
    }

    #[inline]
    pub fn b(&self) -> bool {
        self.data[0] & 0x4000_0000 > 0
    }

    #[inline]
    pub fn z(&self) -> bool {
        self.data[0] & 0x2000_0000 > 0
    }
}
