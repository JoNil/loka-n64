#![allow(dead_code)]

use strum_macros::FromRepr;

// color = (a*p + b*m) / (a + b)

#[repr(u8)]
#[derive(Clone, Copy, Debug, FromRepr)]
enum PMCycleOne {
    ColorCombinerRgb = 0,
    Memory = 1,
    BlendColor = 2,
    FogColor = 3,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, FromRepr)]
enum PMCycleTwo {
    FirstCycleNumerator = 0,
    Memory = 1,
    BlendColor = 2,
    FogColor = 3,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, FromRepr)]
enum ASrc {
    ColorCombinerAlpha = 0,
    FogAlpha = 1,
    SteppedAlpha = 2,
    Zero = 3,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, FromRepr)]
enum BSrc {
    OneMinusA = 0,
    MemoryAlpha = 1,
    One = 2,
    Zero = 3,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, FromRepr)]
enum RgbDither {
    MagicSquareMatrix = 0,
    StandardBayerMatrix = 1,
    Noise = 2,
    NoDither = 3,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, FromRepr)]
enum AlphaDither {
    Pattern = 0,
    PatternInverted = 1,
    Noise = 2,
    NoDither = 3,
}

#[derive(Clone, Copy, Debug)]
pub struct Blender {
    p_0: PMCycleOne,
    m_0: PMCycleOne,
    a_0: ASrc,
    b_0: BSrc,

    p_1: PMCycleTwo,
    m_1: PMCycleTwo,
    a_1: ASrc,
    b_1: BSrc,

    rgb_dither: RgbDither,
    alpha_dither: AlphaDither,
}

impl Blender {
    pub const fn default() -> Self {
        Self {
            p_0: PMCycleOne::ColorCombinerRgb,
            m_0: PMCycleOne::Memory,
            a_0: ASrc::ColorCombinerAlpha,
            b_0: BSrc::OneMinusA,

            p_1: PMCycleTwo::FirstCycleNumerator,
            m_1: PMCycleTwo::Memory,
            a_1: ASrc::ColorCombinerAlpha,
            b_1: BSrc::OneMinusA,

            rgb_dither: RgbDither::NoDither,
            alpha_dither: AlphaDither::NoDither,
        }
    }
}

impl Blender {
    pub fn to_command(&self) -> u64 {
        let a_0 = (self.a_0 as u64) << 30;
        let p_0 = (self.p_0 as u64) << 26;
        let b_0 = (self.b_0 as u64) << 22;
        let m_0 = (self.m_0 as u64) << 18;

        let a_1 = (self.a_1 as u64) << 28;
        let p_1 = (self.p_1 as u64) << 24;
        let b_1 = (self.b_1 as u64) << 20;
        let m_1 = (self.m_1 as u64) << 16;

        let rgb_dither = (self.rgb_dither as u64) << 38;
        let alpha_dither = (self.alpha_dither as u64) << 36;

        a_0 | p_0 | b_0 | m_0 | a_1 | p_1 | b_1 | m_1 | rgb_dither | alpha_dither
    }
}

impl Default for Blender {
    fn default() -> Self {
        Self::default()
    }
}
