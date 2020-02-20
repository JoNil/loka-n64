#![cfg_attr(target_vendor = "nintendo64", no_std)]
#![cfg_attr(target_vendor = "nintendo64", feature(alloc_error_handler))]
#![cfg_attr(target_vendor = "nintendo64", feature(global_asm))]
#![cfg_attr(target_vendor = "nintendo64", feature(lang_items))]
#![cfg_attr(target_vendor = "nintendo64", feature(start))]

extern crate alloc;

mod bullet_system;
mod components;
mod enemy_system;
mod entity;
mod player;
mod textures;

use alloc::vec::Vec;
use bullet_system::BulletSystem;
use components::health;
use components::movable;
use components::sprite_drawable;
use components::box_drawable;
use enemy_system::EnemySystem;
use n64::{self, audio, current_time_us, graphics, ipl3font, Controllers, gfx::CommandBuffer};
use n64_math::Color;
use player::{Player, SHIP_SIZE};

const BLUE: Color = Color::new(0b00001_00001_11100_1);
const RED: Color = Color::new(0b10000_00011_00011_1);

fn main() {
    n64::run(|| {

        let mut controllers = Controllers::new();
        let mut player = Player::new();
        let mut bullet_system = BulletSystem::new();
        let mut enemy_system = EnemySystem::new();

        /*let mut audio_buffer = {
            let mut buffer = Vec::new();
            buffer.resize_with(audio::BUFFER_NO_SAMPLES, Default::default);
            buffer.into_boxed_slice()
        };*/

        let mut time_used;
        let mut time_frame = current_time_us();
        let mut dt;

        enemy_system.spawn_enemy();
        enemy_system.spawn_enemy();
        enemy_system.spawn_enemy();
        enemy_system.spawn_enemy();
        enemy_system.spawn_enemy();
        enemy_system.spawn_enemy();

        loop {
            {
                let now = current_time_us();
                dt = (now - time_frame) as f32 / 1e6;
                time_frame = now;
            }

            time_used = current_time_us();

            {
                // Update

                controllers.update();

                enemy_system.update(&mut bullet_system, &mut player);

                player.update(&controllers, &mut bullet_system);

                bullet_system.update(&mut enemy_system, &mut player);

                movable::simulate(dt);

                if !health::is_alive(player.entity()) {
                    break;
                }
            }

            /*{
                // Audio

                if !audio::all_buffers_are_full() {

                    for (i, chunk) in audio_buffer.chunks_mut(128).enumerate() {
                        for sample in chunk {
                            if i % 2 == 0 {
                                *sample = 5000;
                            } else {
                                *sample = -5000;
                            }
                        }
                    }

                    audio::write_audio_blocking(&audio_buffer);
                }

                audio::update();
            }*/

            {

                graphics::with_framebuffer(|fb| {
                    let mut cb = CommandBuffer::new(fb);

                    cb.clear();

                    box_drawable::draw(&mut cb);
                    sprite_drawable::draw();

                    cb.run();
                });

                ipl3font::draw_number(300, 10, BLUE, player.score());
                ipl3font::draw_number(
                    300,
                    215,
                    BLUE,
                    health::get_component(player.entity())
                        .map(|hc| hc.health)
                        .unwrap_or(0),
                );

                {
                    let used_frame_time = current_time_us() - time_used;
                    ipl3font::draw_number(200, 10, RED, used_frame_time as i32);
                    ipl3font::draw_number(100, 10, RED, (dt * 1000.0 * 1000.0) as i32);
                }

                graphics::swap_buffers();
            }
        }

        loop {
            graphics::slow_cpu_clear();
            ipl3font::draw_str(50, 10, RED, b"GAME OVER");
            graphics::swap_buffers();
        }
    });
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
fn panic(info: &core::panic::PanicInfo) -> ! {
    graphics::slow_cpu_clear();
    ipl3font::draw_str(15, 15, RED, b"PANIC!");
    if let Some(location) = info.location() {
        ipl3font::draw_str(
            15,
            30,
            RED,
            alloc::format!(
                "{}:{}",
                location.file().rsplit("\\").nth(0).unwrap_or(""),
                location.line()
            )
            .as_bytes(),
        );
    }
    graphics::swap_buffers();

    loop {}
}

#[cfg(target_vendor = "nintendo64")]
#[alloc_error_handler]
fn oom(_: core::alloc::Layout) -> ! {
    graphics::slow_cpu_clear();
    ipl3font::draw_str(50, 15, RED, b"OUT OF MEMORY!");
    graphics::swap_buffers();

    loop {}
}

#[cfg(target_vendor = "nintendo64")]
#[lang = "eh_personality"]
extern "C" fn rust_eh_personality() {}

#[cfg(target_vendor = "nintendo64")]
global_asm!(include_str!("entrypoint.s"));
