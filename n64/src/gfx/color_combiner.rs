#![allow(dead_code)]

use strum_macros::FromRepr;

#[repr(u8)]
#[derive(Clone, Copy, Debug, FromRepr)]
pub enum ASrc {
    Combined = 0,
    Texel = 1,
    Primitive = 3,
    Shade = 4,
    Environment = 5,
    Noise = 7,
    One = 6,
    Zero = 8,
    Zero1 = 9,
    Zero2 = 10,
    Zero3 = 11,
    Zero4 = 12,
    Zero5 = 13,
    Zero6 = 14,
    Zero7 = 15,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, FromRepr)]
pub enum BSrc {
    Combined = 0,
    Texel = 1,
    Primitive = 3,
    Shade = 4,
    Environment = 5,
    ConvertK4 = 7,
    Zero = 8,
    Zero1 = 9,
    Zero2 = 10,
    Zero3 = 11,
    Zero4 = 12,
    Zero5 = 13,
    Zero6 = 14,
    Zero7 = 15,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, FromRepr)]
pub enum CSrc {
    Combined = 0,
    Texel = 1,
    Primitive = 3,
    Shade = 4,
    Environment = 5,
    CombinedAlpha = 7,
    TexelAlpha = 8,
    PrimitiveAlpha = 10,
    ShadeAlpha = 11,
    EnvironmentAlpha = 12,
    LodFraction = 13,
    PrimitiveLodFraction = 14,
    ConvertK5 = 15,
    Zero = 16,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, FromRepr)]
pub enum DSrc {
    Combined = 0,
    Texel = 1,
    Primitive = 3,
    Shade = 4,
    Environment = 5,
    One = 6,
    Zero = 7,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, FromRepr)]
pub enum AAlphaSrc {
    CombinedAlpha = 0,
    TexelAlpha = 1,
    PrimitiveAlpha = 3,
    ShadeAlpha = 4,
    EnvironmentAlpha = 5,
    One = 6,
    Zero = 7,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, FromRepr)]
pub enum BAlphaSrc {
    CombinedAlpha = 0,
    TexelAlpha = 1,
    PrimitiveAlpha = 3,
    ShadeAlpha = 4,
    EnvironmentAlpha = 5,
    One = 6,
    Zero = 7,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, FromRepr)]
pub enum CAlphaSrc {
    CombinedAlphaInvalid = 0,
    TexelAlpha = 1,
    PrimitiveAlpha = 3,
    ShadeAlpha = 4,
    EnvironmentAlpha = 5,
    Zero = 7,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, FromRepr)]
pub enum DAlphaSrc {
    CombinedAlpha = 0,
    TexelAlpha = 1,
    PrimitiveAlpha = 3,
    ShadeAlpha = 4,
    EnvironmentAlpha = 5,
    One = 6,
    Zero = 7,
}

#[derive(Clone, Copy, Debug)]
pub struct ColorCombiner {
    pub a_0: ASrc,
    pub b_0: BSrc,
    pub c_0: CSrc,
    pub d_0: DSrc,

    pub a_alpha_0: AAlphaSrc,
    pub b_alpha_0: BAlphaSrc,
    pub c_alpha_0: CAlphaSrc,
    pub d_alpha_0: DAlphaSrc,

    pub a_1: ASrc,
    pub b_1: BSrc,
    pub c_1: CSrc,
    pub d_1: DSrc,

    pub a_alpha_1: AAlphaSrc,
    pub b_alpha_1: BAlphaSrc,
    pub c_alpha_1: CAlphaSrc,
    pub d_alpha_1: DAlphaSrc,
}

impl Default for ColorCombiner {
    fn default() -> Self {
        Self {
            a_0: ASrc::Zero,
            b_0: BSrc::Zero,
            c_0: CSrc::Zero,
            d_0: DSrc::Shade,
            
            a_alpha_0: AAlphaSrc::Zero,
            b_alpha_0: BAlphaSrc::Zero,
            c_alpha_0: CAlphaSrc::Zero,
            d_alpha_0: DAlphaSrc::ShadeAlpha,

            a_1: ASrc::Zero,
            b_1: BSrc::Zero,
            c_1: CSrc::Zero,
            d_1: DSrc::Shade,

            a_alpha_1: AAlphaSrc::Zero,
            b_alpha_1: BAlphaSrc::Zero,
            c_alpha_1: CAlphaSrc::Zero,
            d_alpha_1: DAlphaSrc::ShadeAlpha,
        }
    }
}

impl From<u64> for ColorCombiner {
    fn from(mode: u64) -> Self {
        let a_0 = (mode >> 52) & 0xf;
        let b_0 = (mode >> 28) & 0xf;
        let c_0 = (mode >> 47) & 0x1f;
        let d_0 = (mode >> 15) & 0x7;

        let a_alpha_0 = (mode >> 44) & 0x7;
        let b_alpha_0 = (mode >> 12) & 0x7;
        let c_alpha_0 = (mode >> 41) & 0x7;
        let d_alpha_0 = (mode >> 9) & 0x7;

        let a_1 = (mode >> 37) & 0xf;
        let b_1 = (mode >> 24) & 0xf;
        let c_1 = (mode >> 32) & 0x1f;
        let d_1 = (mode >> 6) & 0x7;

        let a_alpha_1 = (mode >> 21) & 0x7;
        let b_alpha_1 = (mode >> 3) & 0x7;
        let c_alpha_1 = (mode >> 18) & 0x7;
        let d_alpha_1 = mode & 0x7;

        Self {
            a_0: ASrc::from_repr(a_0 as u8).unwrap(),
            b_0: BSrc::from_repr(b_0 as u8).unwrap(),
            c_0: CSrc::from_repr(c_0 as u8).unwrap(),
            d_0: DSrc::from_repr(d_0 as u8).unwrap(),
            a_alpha_0: AAlphaSrc::from_repr(a_alpha_0 as u8).unwrap(),
            b_alpha_0: BAlphaSrc::from_repr(b_alpha_0 as u8).unwrap(),
            c_alpha_0: CAlphaSrc::from_repr(c_alpha_0 as u8).unwrap(),
            d_alpha_0: DAlphaSrc::from_repr(d_alpha_0 as u8).unwrap(),
            a_1: ASrc::from_repr(a_1 as u8).unwrap(),
            b_1: BSrc::from_repr(b_1 as u8).unwrap(),
            c_1: CSrc::from_repr(c_1 as u8).unwrap(),
            d_1: DSrc::from_repr(d_1 as u8).unwrap(),
            a_alpha_1: AAlphaSrc::from_repr(a_alpha_1 as u8).unwrap(),
            b_alpha_1: BAlphaSrc::from_repr(b_alpha_1 as u8).unwrap(),
            c_alpha_1: CAlphaSrc::from_repr(c_alpha_1 as u8).unwrap(),
            d_alpha_1: DAlphaSrc::from_repr(d_alpha_1 as u8).unwrap(),
        }
    }
}

impl ColorCombiner {
    pub fn to_command(&self) -> u64 {
        let a_0 = (self.a_0 as u64) << 52;
        let b_0 = (self.b_0 as u64) << 28;
        let c_0 = (self.c_0 as u64) << 47;
        let d_0 = (self.d_0 as u64) << 15;

        let a_alpha_0 = (self.a_alpha_0 as u64) << 44;
        let b_alpha_0 = (self.b_alpha_0 as u64) << 12;
        let c_alpha_0 = (self.c_alpha_0 as u64) << 41;
        let d_alpha_0 = (self.d_alpha_0 as u64) << 9;

        let a_1 = (self.a_1 as u64) << 37;
        let b_1 = (self.b_1 as u64) << 24;
        let c_1 = (self.c_1 as u64) << 32;
        let d_1 = (self.d_1 as u64) << 6;

        let a_alpha_1 = (self.a_alpha_1 as u64) << 21;
        let b_alpha_1 = (self.b_alpha_1 as u64) << 3;
        let c_alpha_1 = (self.c_alpha_1 as u64) << 18;
        let d_alpha_1 = self.d_alpha_1 as u64;

        a_0 | b_0
            | c_0
            | d_0
            | a_alpha_0
            | b_alpha_0
            | c_alpha_0
            | d_alpha_0
            | a_1
            | b_1
            | c_1
            | d_1
            | a_alpha_1
            | b_alpha_1
            | c_alpha_1
            | d_alpha_1
    }
}
