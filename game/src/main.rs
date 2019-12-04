#![no_std]

// Pull panic into scope
// Required by panic_handler
#[cfg(all(not(test), not(windows)))]
pub use rrt0;

use n64::{self, graphics, ipl3font, controllers::Controllers};

// Colors are 5:5:5:1 RGB with a 16-bit color depth.
const WHITE: u16 = 0b00001_00001_11100_1;

fn main() {
    n64::init();

    let mut controllers = Controllers::new();

    let mut x_pos = 0;
    let mut y_pos = 0;

    const HALF_WIDTH: i16 = graphics::WIDTH as i16 / 2;
    const HALF_HEIGHT: i16 = graphics::HEIGHT as i16 / 2;

    loop {

        controllers.update();

        let mut x_speed = 0;
        let mut y_speed = 0;

        if controllers.up_pressed() {
            y_speed = 1;
        }

        x_pos += x_speed;
        y_pos += y_speed;

        graphics::clear_buffer();

        ipl3font::draw_str_centered_offset(x_pos, y_pos, WHITE, "Isafo en prutt!");

        graphics::swap_buffers();

        graphics::wait_for_vblank();
    }
}
