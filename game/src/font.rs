use crate::textures::*;
use n64::gfx::{
    color_combiner::{
        AAlphaSrc, ASrc, BAlphaSrc, BSrc, CAlphaSrc, CSrc, ColorCombiner, DAlphaSrc, DSrc,
    },
    CommandBuffer, StaticTexture,
};
use n64_math::{const_vec2, Vec2};

static ATLAS: &[&StaticTexture] = &[
    &FONT_1_SPACE,
    &FONT_1_EXCLAMATION,
    &FONT_1_DBL_QUOTE,
    &FONT_1_HASHTAG,
    &FONT_1_DOLLAR,
    &FONT_1_PERCENT,
    &FONT_1_AMPERSAND,
    &FONT_1_QUOTE,
    &FONT_1_PARENTHESIS_OPEN,
    &FONT_1_PARENTHESIS_CLOSE,
    &FONT_1_STAR,
    &FONT_1_PLUS,
    &FONT_1_COMMA,
    &FONT_1_DASH,
    &FONT_1_DOT,
    &FONT_1_SLASH,
    &FONT_1_0,
    &FONT_1_1,
    &FONT_1_2,
    &FONT_1_3,
    &FONT_1_4,
    &FONT_1_5,
    &FONT_1_6,
    &FONT_1_7,
    &FONT_1_8,
    &FONT_1_9,
    &FONT_1_COLON,
    &FONT_1_SEMI_COLON,
    &FONT_1_LESS,
    &FONT_1_EQUAL,
    &FONT_1_GREATER,
    &FONT_1_QUESTION,
    &FONT_1_AT,
    &FONT_1_A,
    &FONT_1_B,
    &FONT_1_C,
    &FONT_1_D,
    &FONT_1_E,
    &FONT_1_F,
    &FONT_1_G,
    &FONT_1_H,
    &FONT_1_I,
    &FONT_1_J,
    &FONT_1_K,
    &FONT_1_L,
    &FONT_1_M,
    &FONT_1_N,
    &FONT_1_O,
    &FONT_1_P,
    &FONT_1_Q,
    &FONT_1_R,
    &FONT_1_S,
    &FONT_1_T,
    &FONT_1_U,
    &FONT_1_V,
    &FONT_1_W,
    &FONT_1_X,
    &FONT_1_Y,
    &FONT_1_Z,
    &FONT_1_BRACKET_OPEN,
    &FONT_1_BRACKET_CLOSE,
    &FONT_1_BACKSLASH,
    &FONT_1_HAT,
    &FONT_1_UNDERSCORE,
    &FONT_1_ACCENT,
    &FONT_1_A_LOWER,
    &FONT_1_B_LOWER,
    &FONT_1_C_LOWER,
    &FONT_1_D_LOWER,
    &FONT_1_E_LOWER,
    &FONT_1_F_LOWER,
    &FONT_1_G_LOWER,
    &FONT_1_H_LOWER,
    &FONT_1_I_LOWER,
    &FONT_1_J_LOWER,
    &FONT_1_K_LOWER,
    &FONT_1_L_LOWER,
    &FONT_1_M_LOWER,
    &FONT_1_N_LOWER,
    &FONT_1_O_LOWER,
    &FONT_1_P_LOWER,
    &FONT_1_Q_LOWER,
    &FONT_1_R_LOWER,
    &FONT_1_S_LOWER,
    &FONT_1_T_LOWER,
    &FONT_1_U_LOWER,
    &FONT_1_V_LOWER,
    &FONT_1_W_LOWER,
    &FONT_1_X_LOWER,
    &FONT_1_Y_LOWER,
    &FONT_1_Z_LOWER,
    &FONT_1_CURLY_OPEN,
    &FONT_1_PIPE,
    &FONT_1_CURLY_CLOSE,
    &FONT_1_TILDE,
];

const GLYPH_SIZE: Vec2 = const_vec2!([11.0, 0.0]);

#[inline]
fn digit_to_char(digit: u8) -> char {
    match digit {
        0..=9 => (b'0' + digit) as char,
        _ => panic!(),
    }
}

pub fn draw_number(cb: &mut CommandBuffer, mut number: i32, upper_right: Vec2, color: u32) {
    let mut negative = false;
    let mut next_pos = upper_right - GLYPH_SIZE;

    cb.set_color_combiner_mode(ColorCombiner {
        a_0: ASrc::Zero,
        b_0: BSrc::Zero,
        c_0: CSrc::Zero,
        d_0: DSrc::Texel,
        a_alpha_0: AAlphaSrc::Zero,
        b_alpha_0: BAlphaSrc::Zero,
        c_alpha_0: CAlphaSrc::Zero,
        d_alpha_0: DAlphaSrc::TexelAlpha,

        a_1: ASrc::Zero,
        b_1: BSrc::Zero,
        c_1: CSrc::Zero,
        d_1: DSrc::Texel,
        a_alpha_1: AAlphaSrc::Zero,
        b_alpha_1: BAlphaSrc::Zero,
        c_alpha_1: CAlphaSrc::Zero,
        d_alpha_1: DAlphaSrc::TexelAlpha,
    });

    if number == 0 {
        draw_char(cb, '0', next_pos, color);
        next_pos -= GLYPH_SIZE;
        return;
    }

    if number < 0 {
        number = number.abs();
        negative = true;
    }

    while number > 0 {
        draw_char(cb, digit_to_char((number % 10) as u8), next_pos, color);
        next_pos -= GLYPH_SIZE;
        number /= 10;
    }

    if negative {
        draw_char(cb, '-', next_pos, color);
    }
}

pub fn draw_text(cb: &mut CommandBuffer, text: &str, upper_left: Vec2, color: u32) {
    cb.set_color_combiner_mode(ColorCombiner {
        a_0: ASrc::Zero,
        b_0: BSrc::Zero,
        c_0: CSrc::Zero,
        d_0: DSrc::Texel,
        a_alpha_0: AAlphaSrc::Zero,
        b_alpha_0: BAlphaSrc::Zero,
        c_alpha_0: CAlphaSrc::Zero,
        d_alpha_0: DAlphaSrc::TexelAlpha,

        a_1: ASrc::Zero,
        b_1: BSrc::Zero,
        c_1: CSrc::Zero,
        d_1: DSrc::Texel,
        a_alpha_1: AAlphaSrc::Zero,
        b_alpha_1: BAlphaSrc::Zero,
        c_alpha_1: CAlphaSrc::Zero,
        d_alpha_1: DAlphaSrc::TexelAlpha,
    });

    let mut next_pos = upper_left;

    for ch in text.chars() {
        draw_char(cb, ch, next_pos, color);
        next_pos += GLYPH_SIZE;
    }
}

pub fn draw_char(cb: &mut CommandBuffer, ch: char, pos: Vec2, color: u32) {
    cb.add_textured_rect(
        pos,
        pos + Vec2::new(16.0, 16.0),
        match ch {
            ' '..='~' => ATLAS[(ch as usize) - (' ' as usize)],
            _ => &FONT_1_BAD,
        }
        .as_texture(),
        Some(color),
    );
}

pub fn text_width(text: &str) -> i32 {
    text.len() as i32 * GLYPH_SIZE.x as i32
}
