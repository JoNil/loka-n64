#![no_std]

mod n64;

// Pull panic into scope
// Required by panic_handler
#[cfg(all(not(test), not(windows)))]
pub use rrt0;

use n64::{ipl3font, vi};

// Colors are 5:5:5:1 RGB with a 16-bit color depth.
const WHITE: u16 = 0b00001_00001_11100_1;

fn main() {
    vi::init();

    ipl3font::draw_str_centered(WHITE, "Isafo en prutt!");
    vi::swap_buffer();

    loop {}
}
