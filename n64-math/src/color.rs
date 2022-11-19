use zerocopy::{AsBytes, FromBytes, Unaligned};

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Default, AsBytes, FromBytes, Unaligned, PartialEq, Eq)]
pub struct Color {
    value: u16,
}

impl Color {
    #[inline]
    pub const fn new(value: u16) -> Color {
        Color { value }
    }

    #[inline]
    pub fn from_rgb(r: f32, g: f32, b: f32) -> Color {
        Color {
            value: (((r * 31.0) as u16) << 11)
                | (((g * 31.0) as u16) << 6)
                | (((b * 31.0) as u16) << 1)
                | 0x1,
        }
    }

    #[inline]
    pub fn from_bytes(bytes: &[u8; 4]) -> Color {
        Color {
            value: ((((bytes[0] as f32 * 31.0 / 255.0) as u16) & 0b11111) << 11)
                | ((((bytes[1] as f32 * 31.0 / 255.0) as u16) & 0b11111) << 6)
                | ((((bytes[2] as f32 * 31.0 / 255.0) as u16) & 0b11111) << 1)
                | (bytes[3] > 0) as u16,
        }
    }

    #[inline]
    pub fn r(&self) -> f32 {
        (self.value >> 11 & 0b11111) as f32 / 31.0
    }

    #[inline]
    pub fn g(&self) -> f32 {
        (self.value >> 6 & 0b11111) as f32 / 31.0
    }

    #[inline]
    pub fn b(&self) -> f32 {
        (self.value >> 1 & 0b11111) as f32 / 31.0
    }

    #[inline]
    pub fn a(&self) -> f32 {
        (self.value & 0b1) as f32
    }

    #[inline]
    pub fn be_to_le(&self) -> Self {
        Self {
            value: ((self.value) << 8) | ((self.value) >> 8),
        }
    }

    #[inline]
    pub fn to_rgba(&self) -> [f32; 4] {
        let r = (self.value >> 11 & 0b11111) as f32 / 31.0;
        let g = (self.value >> 6 & 0b11111) as f32 / 31.0;
        let b = (self.value >> 1 & 0b11111) as f32 / 31.0;
        let a = (self.value & 0b1) as f32;

        [r, g, b, a]
    }

    #[inline]
    pub fn value(&self) -> u16 {
        self.value
    }
}
