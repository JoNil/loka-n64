#![cfg_attr(target_vendor = "nintendo64", feature(no_std))]

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

    let mut x_speed = 2;
    let mut y_speed = 2;

    const HALF_WIDTH: i16 = graphics::WIDTH as i16 / 2;
    const HALF_HEIGHT: i16 = graphics::HEIGHT as i16 / 2;

    loop {
        controllers.update();

        x_pos += x_speed;
        y_pos += y_speed;

        if !controllers.up_pressed() {
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
        }

        graphics::clear_buffer();

        ipl3font::draw_str_centered_offset(x_pos, y_pos, BLUE, b"Isafo en prutt!");

        ipl3font::draw_hex(120, 20, RED, x_pos as u32);
        ipl3font::draw_number(120, 40, RED, x_pos as i32);

        for (i, value) in controllers.data[0..4].iter().enumerate() {

            ipl3font::draw_hex(280, 20 + 40*i, RED, (value >> 32) as u32);
            ipl3font::draw_hex(280, 40 + 40*i, RED, (value & 0xffff_ffff) as u32);
        }
        
        graphics::wait_for_vblank();

        graphics::swap_buffers();
    }
}