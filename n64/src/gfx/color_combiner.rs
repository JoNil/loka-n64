#![allow(dead_code)]

#[derive(Clone, Copy)]
enum ASrc {
    Combined = 0,
    Texel = 1,
    Primitive = 3,
    Shade = 4,
    Environment = 5,
    Noise = 7,
    One = 6,
    Zero = 8,
}

#[derive(Clone, Copy)]
enum BSrc {
    Combined = 0,
    Texel = 1,
    Primitive = 3,
    Shade = 4,
    Environment = 5,
    Zero = 8,
}

#[derive(Clone, Copy)]
enum CSrc {
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

#[derive(Clone, Copy)]
enum DSrc {
    Combined = 0,
    Texel = 1,
    Primitive = 3,
    Shade = 4,
    Environment = 5,
    One = 6,
    Zero = 7,
}

#[derive(Clone, Copy)]
enum AAlphaSrc {
    CombinedAlpha = 0,
    TexelAlpha = 1,
    PrimitiveAlpha = 3,
    ShadeAlpha = 4,
    EnvironmentAlpha = 5,
    One = 6,
    Zero = 7,
}

#[derive(Clone, Copy)]
enum BAlphaSrc {
    CombinedAlpha = 0,
    TexelAlpha = 1,
    PrimitiveAlpha = 3,
    ShadeAlpha = 4,
    EnvironmentAlpha = 5,
    One = 6,
    Zero = 7,
}

#[derive(Clone, Copy)]
enum CAlphaSrc {
    TexelAlpha = 1,
    PrimitiveAlpha = 3,
    ShadeAlpha = 4,
    EnvironmentAlpha = 5,
    Zero = 7,
}

#[derive(Clone, Copy)]
enum DAlphaSrc {
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
    a: ASrc,
    b: BSrc,
    c: CSrc,
    d: DSrc,
    aa: AAlphaSrc,
    ba: BAlphaSrc,
    ca: CAlphaSrc,
    da: DAlphaSrc,
}

impl Default for ColorCombiner {
    fn default() -> Self {
        Self {
            a: ASrc::Zero,
            b: BSrc::Zero,
            c: CSrc::Zero,
            d: DSrc::Shade,
            aa: AAlphaSrc::Zero,
            ba: BAlphaSrc::Zero,
            ca: CAlphaSrc::Zero,
            da: DAlphaSrc::ShadeAlpha,
        }
    }
}

impl ColorCombiner {
    pub fn get_command(&self) -> u64 {
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
