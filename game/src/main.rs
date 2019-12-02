#![no_std]

mod n64;

// Pull panic into scope
// Required by panic_handler
#[cfg(all(not(test), not(windows)))]
pub use rrt0;

use n64::{ipl3font, vi};

// Colors are 5:5:5:1 RGB with a 16-bit color depth.
const WHITE: u16 = 0b00001_00001_11100_1;

fn clear_buffer() {
    let frame_buffer = vi::next_buffer() as usize;
    for i in 0..vi::WIDTH * vi::HEIGHT {
        let p = (frame_buffer + i * 4) as *mut u32;
        unsafe {
            *p = 0x1001_1001;
        }
    }
}

fn main() {
    vi::init();

    let mut x_pos = 0;
    let mut y_pos = 0;

    let mut x_speed = 2;
    let mut y_speed = 2;

    const HALF_WIDTH: i16 = vi::WIDTH as i16 / 2;
    const HALF_HEIGHT: i16 = vi::HEIGHT as i16 / 2;

    loop {
        x_pos += x_speed;
        y_pos += y_speed;

        if x_pos >= HALF_WIDTH {
            x_speed = -x_speed;
            x_pos = HALF_WIDTH;
        }

        if x_pos <= -HALF_WIDTH {
            x_speed = -x_speed;
            x_pos = -HALF_WIDTH;
        }

        if y_pos >= HALF_HEIGHT {
            y_speed = -y_speed;
            y_pos = HALF_HEIGHT;
        }

        if y_pos <= -HALF_HEIGHT {
            y_speed = -y_speed;
            y_pos = -HALF_HEIGHT;
        }

        clear_buffer();

        ipl3font::draw_str_centered_offset(x_pos, y_pos, WHITE, "Isafo en prutt!");

        vi::swap_buffer();

        vi::wait_for_ready();
    }
}
