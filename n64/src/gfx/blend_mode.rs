#![allow(dead_code)]

use strum_macros::FromRepr;

// color = (p*a + m*b) / (a + b)

#[repr(u8)]
#[derive(Clone, Copy, Debug, FromRepr)]
pub enum PMCycleOne {
    ColorCombinerRgb = 0,
    Memory = 1,
    BlendColor = 2,
    FogColor = 3,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, FromRepr)]
pub enum PMCycleTwo {
    FirstCycleNumerator = 0,
    Memory = 1,
    BlendColor = 2,
    FogColor = 3,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, FromRepr)]
pub enum ASrc {
    ColorCombinerAlpha = 0,
    FogAlpha = 1,
    SteppedAlpha = 2,
    Zero = 3,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, FromRepr)]
pub enum BSrc {
    OneMinusA = 0,
    MemoryAlpha = 1,
    One = 2,
    Zero = 3,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, FromRepr)]
pub enum RgbDither {
    MagicSquareMatrix = 0,
    StandardBayerMatrix = 1,
    Noise = 2,
    NoDither = 3,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, FromRepr)]
pub enum AlphaDither {
    Pattern = 0,
    PatternInverted = 1,
    Noise = 2,
    NoDither = 3,
}

#[derive(Clone, Copy, Debug)]
pub struct BlendMode {
    p_0: PMCycleOne,
    a_0: ASrc,
    m_0: PMCycleOne,
    b_0: BSrc,

    p_1: PMCycleTwo,
    a_1: ASrc,
    m_1: PMCycleTwo,
    b_1: BSrc,

    rgb_dither: RgbDither,
    alpha_dither: AlphaDither,
}

impl BlendMode {
    pub const fn default() -> Self {
        Self {
            p_0: PMCycleOne::ColorCombinerRgb,
            a_0: ASrc::ColorCombinerAlpha,
            m_0: PMCycleOne::Memory,
            b_0: BSrc::OneMinusA,

            p_1: PMCycleTwo::FirstCycleNumerator,
            a_1: ASrc::ColorCombinerAlpha,
            m_1: PMCycleTwo::Memory,
            b_1: BSrc::OneMinusA,

            rgb_dither: RgbDither::NoDither,
            alpha_dither: AlphaDither::NoDither,
        }
    }

    pub const fn one(p: PMCycleOne, a: ASrc, m: PMCycleOne, b: BSrc) -> Self {
        Self {
            p_0: p,
            a_0: a,
            m_0: m,
            b_0: b,

            p_1: PMCycleTwo::from_cycle_one(p),
            a_1: a,
            m_1: PMCycleTwo::from_cycle_one(m),
            b_1: b,

            rgb_dither: RgbDither::NoDither,
            alpha_dither: AlphaDither::NoDither,
        }
    }

    pub const fn simple(p: PMCycleOne, m: PMCycleOne) -> Self {
        Self {
            p_0: p,
            a_0: ASrc::ColorCombinerAlpha,
            m_0: m,
            b_0: BSrc::OneMinusA,

            p_1: PMCycleTwo::from_cycle_one(p),
            a_1: ASrc::ColorCombinerAlpha,
            m_1: PMCycleTwo::from_cycle_one(m),
            b_1: BSrc::OneMinusA,

            rgb_dither: RgbDither::NoDither,
            alpha_dither: AlphaDither::NoDither,
        }
    }
}

impl BlendMode {
    pub fn to_command(&self) -> u64 {
        let p_0 = (self.p_0 as u64) << 30;
        let a_0 = (self.a_0 as u64) << 26;
        let m_0 = (self.m_0 as u64) << 22;
        let b_0 = (self.b_0 as u64) << 18;

        let p_1 = (self.p_1 as u64) << 28;
        let a_1 = (self.a_1 as u64) << 24;
        let m_1 = (self.m_1 as u64) << 20;
        let b_1 = (self.b_1 as u64) << 16;

        let rgb_dither = (self.rgb_dither as u64) << 38;
        let alpha_dither = (self.alpha_dither as u64) << 36;

        p_0 | a_0 | m_0 | b_0 | p_1 | a_1 | m_1 | b_1 | rgb_dither | alpha_dither
    }
}

impl Default for BlendMode {
    fn default() -> Self {
        Self::default()
    }
}

impl PMCycleTwo {
    const fn from_cycle_one(one: PMCycleOne) -> PMCycleTwo {
        match one {
            PMCycleOne::ColorCombinerRgb => PMCycleTwo::FirstCycleNumerator,
            PMCycleOne::Memory => PMCycleTwo::Memory,
            PMCycleOne::BlendColor => PMCycleTwo::BlendColor,
            PMCycleOne::FogColor => PMCycleTwo::FogColor,
        }
    }
}
