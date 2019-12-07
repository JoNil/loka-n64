#![cfg_attr(target_vendor = "nintendo64", no_std)]

// Pull panic into scope
// Required by panic_handler
#[cfg(target_vendor = "nintendo64")]
pub use rrt0;

use n64::{self, controllers::Controllers, graphics, ipl3font};

// Colors are 5:5:5:1 RGB with a 16-bit color depth.
const BLUE: u16 = 0b00001_00001_11100_1;
const RED: u16 = 0b10000_00011_00011_1;

fn main() {
    n64::init();

    let mut controllers = Controllers::new();

    let mut x_pos = 0;
    let mut y_pos = 0;

    const SPEED: i32 = 2;

    loop {
        controllers.update();

        if controllers.up_pressed() {
            y_pos -= SPEED;
        }

        if controllers.down_pressed() {
            y_pos += SPEED;
        }

        if controllers.right_pressed() {
            x_pos += SPEED;
        }

        if controllers.left_pressed() {
            x_pos -= SPEED;
        }

        graphics::clear_buffer();

        ipl3font::draw_str_centered_offset(x_pos, y_pos, BLUE, b"Isafo en prutt!");

        ipl3font::draw_hex(120, 20, RED, x_pos as u32);
        ipl3font::draw_number(120, 40, RED, x_pos);

        for j in 0..2 {
            for (i, value) in controllers.data[(4*j)..(4*(j+1))].iter().enumerate() {
                ipl3font::draw_hex(200 + 80*j as i32 , 20 + 40 * (i as i32), RED, (value >> 32) as u32);
                ipl3font::draw_hex(200 + 80*j as i32, 40 + 40 * (i as i32), RED, (value & 0xffff_ffff) as u32);
            }
        }

        graphics::swap_buffers();
    }
}
