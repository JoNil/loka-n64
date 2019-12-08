#![cfg_attr(target_vendor = "nintendo64", no_std)]

mod bullet_system;
mod player;

#[cfg(target_vendor = "nintendo64")]
pub use rrt0;

use n64::{self, controllers::Controllers, graphics, ipl3font, current_time_us};
use player::Player;
use bullet_system::BulletSystem;

const BLUE: u16 = 0b00001_00001_11100_1;
const RED: u16 = 0b10000_00011_00011_1;

fn main() {

    // Todo maybe return n64 object that has funcs
    n64::init();

    let mut controllers = Controllers::new();
    let mut player = Player::new();
    let mut bullet_system = BulletSystem::new();

    let mut time_update_and_draw;
    let mut time_frame = current_time_us();
    let mut dt;

    loop {

        {
            let now = current_time_us();
            dt = (now - time_frame) as f32 / 1e6;
            time_frame = now;
        }

        time_update_and_draw = current_time_us();

        {
            // Update

            controllers.update();

            player.update(dt, &controllers, &mut bullet_system);

            bullet_system.update(dt);
        }

        {
            // Draw

            graphics::clear_buffer();

            player.draw();

            bullet_system.draw();

            {
                let used_frame_time = current_time_us() - time_update_and_draw;

                ipl3font::draw_number(50, 10, RED, used_frame_time);
            }

            graphics::swap_buffers();
        }
    }
}
