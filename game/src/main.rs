#![cfg_attr(target_vendor = "nintendo64", no_std)]

#![feature(alloc_error_handler)]
#![feature(global_asm)]
#![feature(lang_items)]
#![feature(start)]

extern crate alloc;

mod bullet_system;
mod enemy_system;
mod player;

use alloc::boxed::Box;
use alloc::vec::Vec;
use bullet_system::BulletSystem;
use enemy_system::EnemySystem;
use n64_math::Color;
use n64::{self, current_time_us, graphics, ipl3font, Controllers, Rng, audio};
use player::Player;

const BLUE: Color = Color::new(0b00001_00001_11100_1);
const RED: Color = Color::new(0b10000_00011_00011_1);

fn main() {
    // Todo maybe return n64 object that has funcs
    n64::init();

    let mut controllers = Controllers::new();
    let mut player = Player::new();
    let mut bullet_system = BulletSystem::new();
    let mut enemy_system = EnemySystem::new();
    let mut rng = Rng::new_unseeded();

    let mut time_update_and_draw;
    let mut time_frame = current_time_us();
    let mut dt;

    enemy_system.spawn_enemy(&mut rng);
    enemy_system.spawn_enemy(&mut rng);
    enemy_system.spawn_enemy(&mut rng);
    enemy_system.spawn_enemy(&mut rng);

    let mut audo_dbg = 0;

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

            enemy_system.update(&mut bullet_system, &mut rng);

            player.update(dt, &controllers, &mut bullet_system, &mut rng);

            bullet_system.update(dt, &mut enemy_system, &mut rng);
        }

        /*{
            if !audio::all_buffers_are_full() {
                // Audio

                let mut buffer = {
                    let mut buffer = Vec::new();
                    buffer.resize_with(2 * 512, Default::default);
                    buffer.into_boxed_slice()
                };

                for (i, chunk) in buffer.chunks_mut(128).enumerate() {
                    for sample in chunk {
                        if i % 2 == 0 {
                            *sample = 5000;
                        } else {
                            *sample = -5000;
                        }
                    }
                }

                audo_dbg += audio::write_audio_blocking(&buffer);
            }

            audio::update();
        }*/

        {
            // Draw

            graphics::clear_buffer();

            player.draw();
            enemy_system.draw();
            bullet_system.draw();

            ipl3font::draw_number(100, 10, RED, audo_dbg);

            {
                let used_frame_time = current_time_us() - time_update_and_draw;
                ipl3font::draw_number(50, 10, RED, used_frame_time);
            }

            graphics::swap_buffers();
        }
    }
}

#[cfg(target_vendor = "nintendo64")]
#[global_allocator]
static ALLOC: n64_alloc::N64Alloc = n64_alloc::N64Alloc::INIT;

#[cfg(target_vendor = "nintendo64")]
#[start]
fn start(_argc: isize, _argv: *const *const u8) -> isize {
    main();
    0
}

#[cfg(target_vendor = "nintendo64")]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {

    graphics::clear_buffer();
    ipl3font::draw_str(50, 10, RED, b"PANIC");
    graphics::swap_buffers();

    loop {}
}

#[cfg(target_vendor = "nintendo64")]
#[alloc_error_handler]
fn oom(_: core::alloc::Layout) -> ! {
    
    graphics::clear_buffer();
    ipl3font::draw_str(50, 10, RED, b"OUT OF MEMORY");
    graphics::swap_buffers();

    loop {}
}

#[cfg(target_vendor = "nintendo64")]
#[lang = "eh_personality"]
extern fn rust_eh_personality() {}

#[cfg(target_vendor = "nintendo64")]
global_asm!(include_str!("entrypoint.s"));