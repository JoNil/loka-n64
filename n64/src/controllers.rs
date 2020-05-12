use crate::graphics::Graphics;
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
    pub fn update(&mut self, _graphics: &Graphics) {
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

    #[inline]
    pub fn start(&self) -> bool {
        self.data[0] & 0x1000_0000 > 0
    }

    #[inline]
    pub fn up(&self) -> bool {
        self.data[0] & 0x0800_0000 > 0
    }

    #[inline]
    pub fn down(&self) -> bool {
        self.data[0] & 0x0400_0000 > 0
    }

    #[inline]
    pub fn left(&self) -> bool {
        self.data[0] & 0x0200_0000 > 0
    }

    #[inline]
    pub fn right(&self) -> bool {
        self.data[0] & 0x0100_0000 > 0
    }

    #[inline]
    pub fn l(&self) -> bool {
        self.data[0] & 0x0020_0000 > 0
    }

    #[inline]
    pub fn r(&self) -> bool {
        self.data[0] & 0x0010_0000 > 0
    }

    #[inline]
    pub fn c_up(&self) -> bool {
        self.data[0] & 0x0008_0000 > 0
    }

    #[inline]
    pub fn c_down(&self) -> bool {
        self.data[0] & 0x0004_0000 > 0
    }

    #[inline]
    pub fn c_left(&self) -> bool {
        self.data[0] & 0x0002_0000 > 0
    }

    #[inline]
    pub fn c_right(&self) -> bool {
        self.data[0] & 0x0001_0000 > 0
    }
}
