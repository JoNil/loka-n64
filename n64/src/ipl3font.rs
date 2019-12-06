use n64_sys::vi;

pub const GLYPH_WIDTH: usize = 13;
pub const GLYPH_HEIGHT: usize = 14;
const GLYPHS: &[u8; 50] = br##"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!"#'*+,-./:=?@"##;
const UNKNOWN: usize = 48;
const GLYPH_SIZE: usize = 23;
const GLYPH_ADDR: usize = 0xB000_0B70;
const KERNING: usize = 1;

#[inline]
pub fn draw_str_centered(color: u16, string: &[u8]) {
    let x = (vi::WIDTH - string.len() * GLYPH_WIDTH) / 2;
    let y = (vi::HEIGHT - GLYPH_HEIGHT) / 2;

    draw_str(x, y, color, string);
}

#[inline]
pub fn draw_str_centered_offset(x_offset: i16, y_offset: i16, color: u16, string: &[u8]) {
    let y = (vi::HEIGHT - GLYPH_HEIGHT) / 2;
    let x = (vi::WIDTH - string.len() * GLYPH_WIDTH) / 2;

    draw_str(
        (x as i16 + x_offset) as usize,
        (y as i16 + y_offset) as usize,
        color,
        string,
    );
}

#[inline]
pub fn draw_str(mut x: usize, y: usize, color: u16, string: &[u8]) {
    for mut ch in string.iter().copied() {
        if ch == b' ' {
            x += GLYPH_WIDTH;
            continue;
        }

        if ch >= b'a' && ch <= b'z' {
            ch -= b'a' - b'A';
        }

        draw_char(x, y, color, ch);
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
pub fn draw_hex(mut x: usize, y: usize, color: u16, mut number: u32) {

    if number == 0 {
        draw_char(x, y, color, b'0');
        return;
    }

    while number > 0 {
        draw_char(x, y, color, digit_to_hex_char((number & 0xF) as u8));
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
pub fn draw_number(mut x: usize, y: usize, color: u16, mut number: i32) {
    let mut negative = false;

    if number == 0 {
        draw_char(x, y, color, b'0');
        return;
    }

    if number < 0 {
        number = number.abs();
        negative = true;
    }

    while number > 0 {
        draw_char(x, y, color, digit_to_char((number % 10) as u8));
        x -= GLYPH_WIDTH + KERNING;
        number /= 10;
    }

    if negative {
        draw_char(x, y, color, b'-');
    }
}

#[inline]
fn draw_char(x: usize, y: usize, color: u16, ch: u8) {
    let frame_buffer = vi::next_buffer() as usize;

    let index = GLYPHS.iter().position(|c| *c == ch).unwrap_or(UNKNOWN);

    let mut address = GLYPH_ADDR + index * GLYPH_SIZE;
    let mut shift = (4 - (address & 3)) * 8 - 1;
    address &= 0xFFFF_FFFC;
    let mut bits = unsafe { *(address as *const u32) };

    for yy in y..y + GLYPH_HEIGHT {
        if yy >= vi::HEIGHT {
            return;
        }

        for xx in x..x + GLYPH_WIDTH {
            if (bits >> shift) & 1 == 1 && xx < vi::WIDTH {
                let offset = (yy * vi::WIDTH + xx) * 2;
                let p = (frame_buffer + offset) as *mut u16;

                unsafe {
                    *p = color;
                }
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
