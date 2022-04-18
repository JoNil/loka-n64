#![allow(dead_code)]

use strum_macros::FromRepr;

#[derive(Clone, Copy, FromRepr)]
pub enum ASrc {
    Combined = 0,
    Texel = 1,
    Primitive = 3,
    Shade = 4,
    Environment = 5,
    Noise = 7,
    One = 6,
    Zero = 8,
}

#[derive(Clone, Copy, FromRepr)]
pub enum BSrc {
    Combined = 0,
    Texel = 1,
    Primitive = 3,
    Shade = 4,
    Environment = 5,
    Zero = 8,
}

#[derive(Clone, Copy, FromRepr)]
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
    Zero = 16,
}

#[derive(Clone, Copy, FromRepr)]
pub enum DSrc {
    Combined = 0,
    Texel = 1,
    Primitive = 3,
    Shade = 4,
    Environment = 5,
    One = 6,
    Zero = 7,
}

#[derive(Clone, Copy, FromRepr)]
pub enum AAlphaSrc {
    CombinedAlpha = 0,
    TexelAlpha = 1,
    PrimitiveAlpha = 3,
    ShadeAlpha = 4,
    EnvironmentAlpha = 5,
    One = 6,
    Zero = 7,
}

#[derive(Clone, Copy, FromRepr)]
pub enum BAlphaSrc {
    CombinedAlpha = 0,
    TexelAlpha = 1,
    PrimitiveAlpha = 3,
    ShadeAlpha = 4,
    EnvironmentAlpha = 5,
    One = 6,
    Zero = 7,
}

#[derive(Clone, Copy, FromRepr)]
pub enum CAlphaSrc {
    TexelAlpha = 1,
    PrimitiveAlpha = 3,
    ShadeAlpha = 4,
    EnvironmentAlpha = 5,
    Zero = 7,
}

#[derive(Clone, Copy, FromRepr)]
pub enum DAlphaSrc {
    CombinedAlpha = 0,
    TexelAlpha = 1,
    PrimitiveAlpha = 3,
    ShadeAlpha = 4,
    EnvironmentAlpha = 5,
    One = 6,
    Zero = 7,
}

#[derive(Clone, Copy)]
pub struct ColorCombiner {
    pub a: ASrc,
    pub b: BSrc,
    pub c: CSrc,
    pub d: DSrc,
    pub aa: AAlphaSrc,
    pub ba: BAlphaSrc,
    pub ca: CAlphaSrc,
    pub da: DAlphaSrc,
}

impl Default for ColorCombiner {
    fn default() -> Self {
        Self {
            a: ASrc::Zero,
            b: BSrc::Zero,
            c: CSrc::Zero,
            d: DSrc::Combined,
            aa: AAlphaSrc::Zero,
            ba: BAlphaSrc::Zero,
            ca: CAlphaSrc::Zero,
            da: DAlphaSrc::CombinedAlpha,
        }
    }
}

impl From<u64> for ColorCombiner {
    fn from(mode: u64) -> Self {
        let sub_a_0 = (mode >> 52) & 0xf;
        let sub_b_0 = (mode >> 28) & 0xf;
        let mul_c_0 = (mode >> 47) & 0x1f;
        let add_d_0 = (mode >> 15) & 0x7;

        let sub_a_alpha_0 = (mode >> 44) & 0x7;
        let sub_b_alpha_0 = (mode >> 12) & 0x7;
        let mul_c_alpha_0 = (mode >> 41) & 0x7;
        let add_d_alpha_0 = (mode >> 9) & 0x7;

        let sub_a_1 = (mode >> 37) & 0xf;
        let sub_b_1 = (mode >> 24) & 0xf;
        let mul_c_1 = (mode >> 32) & 0x1f;
        let add_d_1 = (mode >> 6) & 0x7;

        let sub_a_alpha_1 = (mode >> 21) & 0x7;
        let sub_b_alpha_1 = (mode >> 3) & 0x7;
        let mul_c_alpha_1 = (mode >> 18) & 0x7;
        let add_d_alpha_1 = (mode >> 0) & 0x7;

        Default::default()
    }
}

impl ColorCombiner {
    pub fn to_command(&self) -> u64 {
        let sub_a_0 = (self.a as u64) << 52;
        let sub_b_0 = (self.b as u64) << 28;
        let mul_c_0 = (self.c as u64) << 47;
        let add_d_0 = (self.d as u64) << 15;

        let sub_a_alpha_0 = (self.aa as u64) << 44;
        let sub_b_alpha_0 = (self.ba as u64) << 12;
        let mul_c_alpha_0 = (self.ca as u64) << 41;
        let add_d_alpha_0 = (self.da as u64) << 9;

        let sub_a_1 = (ASrc::Zero as u64) << 37;
        let sub_b_1 = (BSrc::Zero as u64) << 24;
        let mul_c_1 = (CSrc::Zero as u64) << 32;
        let add_d_1 = (DSrc::Zero as u64) << 6;

        let sub_a_alpha_1 = (AAlphaSrc::Zero as u64) << 21;
        let sub_b_alpha_1 = (BAlphaSrc::Zero as u64) << 3;
        let mul_c_alpha_1 = (CAlphaSrc::Zero as u64) << 18;
        let add_d_alpha_1 = DAlphaSrc::Zero as u64;

        sub_a_0
            | sub_b_0
            | mul_c_0
            | add_d_0
            | sub_a_alpha_0
            | sub_b_alpha_0
            | mul_c_alpha_0
            | add_d_alpha_0
            | sub_a_1
            | sub_b_1
            | mul_c_1
            | add_d_1
            | sub_a_alpha_1
            | sub_b_alpha_1
            | mul_c_alpha_1
            | add_d_alpha_1
    }
}
