#![cfg_attr(target_vendor = "nintendo64", no_std)]

// Pull panic into scope
// Required by panic_handler
#[cfg(target_vendor = "nintendo64")]
pub use rrt0;

use n64::{self, controllers::Controllers, graphics, ipl3font, current_time_us};

mod bullets;
mod enemies;
mod player;

// Colors are 5:5:5:1 RGB with a 16-bit color depth.
const BLUE: u16 = 0b00001_00001_11100_1;
const RED: u16 = 0b10000_00011_00011_1;

fn main() {

    // Todo maybe return n64 object that has funcs
    n64::init();

    let mut controllers = Controllers::new();

    let mut x_pos = 0;
    let mut y_pos = 0;

    let mut start;

    loop {

        start = current_time_us();

        controllers.update();

        x_pos += (controllers.x() >> 5) as i32;
        y_pos -= (controllers.y() >> 5) as i32;

        graphics::clear_buffer();

        if controllers.z() {
            ipl3font::draw_str_centered_offset(x_pos, y_pos, RED, b"Isafo en prutt!");
        } else {
            ipl3font::draw_str_centered_offset(x_pos, y_pos, BLUE, b"Isafo en prutt!");
        }

        {
            let diff = current_time_us() - start;

            ipl3font::draw_number(50, 10, RED, diff);
        }

        graphics::swap_buffers();
    }
}
