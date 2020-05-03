use n64_math::Color;
use crate::gfx::TextureMut;

pub const GLYPH_WIDTH: i32 = 13;
pub const GLYPH_HEIGHT: i32 = 14;
const KERNING: i32 = 1;

const GLYPHS: &[u8; 50] = br##"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!"#'*+,-./:=?@"##;
const UNKNOWN: usize = 48;
const GLYPH_SIZE: usize = 23;
const GLYPH_ADDR: usize = 0xB000_0B70;

#[inline]
pub fn draw_str(out_tex: &mut TextureMut, mut x: i32, mut y: i32, color: Color, string: &[u8]) {
    let start_x = x;

    for mut ch in string.iter().copied() {
        if ch == b' ' {
            x += GLYPH_WIDTH + KERNING;
            continue;
        }

        if ch == b'\n' {
            y += GLYPH_HEIGHT + 3;
            x = start_x;
            continue;
        }

        if ch == b'{' || ch == b'}' {
            x += GLYPH_WIDTH + KERNING;
            continue;
        }

        if ch >= b'a' && ch <= b'z' {
            ch -= b'a' - b'A';
        }

        draw_char(out_tex, x, y, color, ch);
        x += GLYPH_WIDTH + KERNING;
    }
}

#[inline]
fn digit_to_hex_char(digit: u8) -> u8 {
    match digit {
        0..=9 => b'0' + digit,
        10..=15 => b'A' + (digit - 10),
        _ => panic!(),
    }
}

#[inline]
pub fn draw_hex(out_tex: &mut TextureMut, mut x: i32, y: i32, color: Color, mut number: u32) {
    if number == 0 {
        draw_char(out_tex, x, y, color, b'0');
        return;
    }

    while number > 0 {
        draw_char(out_tex, x, y, color, digit_to_hex_char((number & 0xF) as u8));
        x -= GLYPH_WIDTH + KERNING;
        number >>= 4;
    }
}

#[inline]
fn digit_to_char(digit: u8) -> u8 {
    match digit {
        0..=9 => b'0' + digit,
        _ => panic!(),
    }
}

#[inline]
pub fn draw_number(out_tex: &mut TextureMut, mut x: i32, y: i32, color: Color, mut number: i32) {
    let mut negative = false;

    if number == 0 {
        draw_char(out_tex, x, y, color, b'0');
        return;
    }

    if number < 0 {
        number = number.abs();
        negative = true;
    }

    while number > 0 {
        draw_char(out_tex, x, y, color, digit_to_char((number % 10) as u8));
        x -= GLYPH_WIDTH + KERNING;
        number /= 10;
    }

    if negative {
        draw_char(out_tex, x, y, color, b'-');
    }
}

#[cfg(target_vendor = "nintendo64")]
pub fn draw_char(out_tex: &mut TextureMut, x: i32, y: i32, color: Color, ch: u8) {
    let index = GLYPHS.iter().position(|c| *c == ch).unwrap_or(UNKNOWN);

    let mut address = GLYPH_ADDR + index * GLYPH_SIZE;
    let mut shift = (4 - (address & 3)) * 8 - 1;
    address &= 0xFFFF_FFFC;
    let mut bits = unsafe { *(address as *const u32) };

    for yy in y..(y + GLYPH_HEIGHT) {
        if yy < 0 {
            return;
        }

        if yy >= out_tex.height {
            return;
        }

        for xx in x..(x + GLYPH_WIDTH) {
            if (bits >> shift) & 1 == 1 && xx < out_tex.width && x >= 0 {
                out_tex.data[(yy * out_tex.width + xx) as usize] = color;
            }

            if shift == 0 {
                address += 4;
                bits = unsafe { *(address as *const u32) };
                shift = 31;
            } else {
                shift -= 1;
            }
        }
    }
}

#[cfg(not(target_vendor = "nintendo64"))]
pub fn draw_char(out_tex: &mut TextureMut, x: i32, y: i32, color: Color, ch: u8) {
    
    use core::convert::TryInto;

    let ipl3 = std::include_bytes!("../../bootcode.bin");

    let index = GLYPHS.iter().position(|c| *c == ch).unwrap_or(UNKNOWN);

    let mut address = (GLYPH_ADDR - 64) + index * GLYPH_SIZE;
    let mut shift = (4 - (address & 3)) * 8 - 1;
    address &= 0x0FFF_FFFC;
    let mut bits = u32::from_be_bytes(ipl3[address..(address + 4)].try_into().unwrap());

    for yy in y..(y + GLYPH_HEIGHT) {
        if yy < 0 {
            return;
        }

        if yy >= out_tex.height {
            return;
        }

        for xx in x..(x + GLYPH_WIDTH) {
            if (bits >> shift) & 1 == 1 && xx < out_tex.width && x >= 0 {
                out_tex.data[(yy * out_tex.width + xx) as usize] = color;
            }

            if shift == 0 {
                address += 4;
                bits = u32::from_be_bytes(ipl3[address..(address + 4)].try_into().unwrap());
                shift = 31;
            } else {
                shift -= 1;
            }
        }
    }
}
