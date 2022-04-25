#![allow(dead_code)]

// color = (a*p + b*m) / (a + b)

#[derive(Clone, Copy, Debug)]
enum PMCycleOne {
    ColorCombinerRgb = 0,
    Memory = 1,
    BlendColor = 2,
    FogColor = 3,
}

#[derive(Clone, Copy, Debug)]
enum PMCycleTwo {
    FirstCycleNumerator = 0,
    Memory = 1,
    BlendColor = 2,
    FogColor = 3,
}

#[derive(Clone, Copy, Debug)]
enum ASrc {
    ColorCombinerAlpha = 0,
    FogAlpha = 1,
    SteppedAlpha = 2,
    Zero = 3,
}

#[derive(Clone, Copy, Debug)]
enum BSrc {
    OneMinusA = 0,
    MemoryAlpha = 1,
    One = 2,
    Zero = 3,
}

#[derive(Clone, Copy, Debug)]
enum RgbDither {
    MagicSquareMatrix = 0,
    StandardBayerMatrix = 1,
    Noise = 2,
    NoDither = 3,
}

#[derive(Clone, Copy, Debug)]
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
            m_0: PMCycleOne::ColorCombinerRgb,
            a_0: ASrc::ColorCombinerAlpha,
            b_0: BSrc::OneMinusA,

            p_1: PMCycleTwo::FirstCycleNumerator,
            m_1: PMCycleTwo::FirstCycleNumerator,
            a_1: ASrc::ColorCombinerAlpha,
            b_1: BSrc::OneMinusA,

            rgb_dither: RgbDither::NoDither,
            alpha_dither: AlphaDither::NoDither,
        }
    }
}

impl Blender {
    pub fn to_command(&self) -> u64 {
        0
    }
}

impl Default for Blender {
    fn default() -> Self {
        Self::default()
    }
}
