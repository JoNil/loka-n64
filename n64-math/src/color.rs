#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct Color {
    value: u16,
}

impl Color {
    #[inline]
    pub const fn new(value: u16) -> Color {
        Color { value: value }
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
    pub fn to_u32(&self) -> u32 {
        let r = (self.value >> 11 & 0b11111) as u8 * 8 + 4;
        let g = (self.value >> 6 & 0b11111) as u8 * 8 + 4;
        let b = (self.value >> 1 & 0b11111) as u8 * 8 + 4;

        (r as u32) << 16 | (g as u32) << 8 | (b as u32)
    }

    #[inline]
    pub fn value(&self) -> u16 {
        self.value
    }
}
