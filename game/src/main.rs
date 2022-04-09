#![cfg_attr(target_vendor = "nintendo64", feature(alloc_error_handler))]
#![cfg_attr(target_vendor = "nintendo64", feature(asm_experimental_arch))]
#![cfg_attr(target_vendor = "nintendo64", feature(lang_items))]
#![cfg_attr(target_vendor = "nintendo64", feature(panic_info_message))]
#![cfg_attr(target_vendor = "nintendo64", feature(start))]
#![cfg_attr(target_vendor = "nintendo64", no_std)]
#![allow(clippy::inconsistent_digit_grouping)]

extern crate alloc;

use crate::components::{
    box_drawable, bullet, enemy,
    health::{self, Health},
    missile, movable,
    player::{self, Player},
    sprite_drawable,
};
use camera::Camera;
use components::{mesh_drawable, player::spawn_player};
use ecs::world::World;
use map::Map;
use maps::MAP_1;
use models::SHIP_3_BODY;
use n64::{
    self, current_time_us,
    gfx::{CommandBuffer, CommandBufferCache},
    ipl3font, slow_cpu_clear, VideoMode, N64,
};
use n64_math::{vec2, vec3, Color};
use sound_mixer::SoundMixer;

mod camera;
mod components;
mod ecs;
mod font;
mod map;
mod maps;
mod model;
mod models;
mod sound;
mod sound_mixer;
mod sounds;
mod textures;
mod weapon;

const RED: Color = Color::new(0b10000_00011_00011_1);
const GREEN: Color = Color::new(0b00011_10000_00011_1);
const BLUE: Color = Color::new(0b0011_00011_10000_1);
const WHITE: Color = Color::new(0b11111_11111_11111_1);

const VIDEO_MODE: VideoMode = VideoMode::Pal {
    width: 320,
    height: 240,
};

const DEBUG_TRIANGLES: bool = false;

fn main() {
    let mut n64 = N64::new(VIDEO_MODE);

    let mut world = World::new();
    let map = Map::load(MAP_1);

    let start_pos = vec2(
        map.get_start_pos().x / VIDEO_MODE.width() as f32,
        map.get_start_pos().y / VIDEO_MODE.height() as f32 - 1.0,
    );

    let mut sound_mixer = SoundMixer::new();
    let mut camera = Camera::new(start_pos);
    let mut command_buffer_cache = CommandBufferCache::new();

    let player = spawn_player(&mut world.entities, start_pos);

    map.spawn_enemies(&mut world, &VIDEO_MODE);

    let mut frame_begin_time;
    let mut last_frame_begin_time = current_time_us();
    let mut frame_used_time = 0;
    let mut dt;

    let mut last_colored_rect_count = 0;
    let mut last_textured_rect_count = 0;

    let mut frame_no = 0;

    loop {
        frame_begin_time = current_time_us();

        {
            dt = (frame_begin_time - last_frame_begin_time) as f32 / 1e6;
            last_frame_begin_time = frame_begin_time;
        }

        let mut print_tri_to_cmd = false;

        {
            // Update

            n64.controllers.update(&n64.graphics);

            camera.update(&n64.controllers, dt, &VIDEO_MODE);

            enemy::update(&mut world, &mut sound_mixer, dt);
            player::update(&mut world, &n64.controllers, &mut sound_mixer, &camera);
            bullet::update(&mut world, &camera);
            missile::update(&mut world, &camera);
            movable::simulate(&mut world, dt);

            world.housekeep();

            if !health::is_alive(world.components.get::<Health>(), player) {
                break;
            }
        }

        {
            // Audio
            n64.audio.update(|buffer| {
                sound_mixer.mix(buffer);
            });
        }

        {
            // Graphics

            let (colored_rect_count, textured_rect_count) = {
                let mut fb = n64.framebuffer.next_buffer();
                let mut cb = CommandBuffer::new(&mut fb, &mut command_buffer_cache);

                cb.clear();

                map.render(&mut cb, VIDEO_MODE, &camera);

                box_drawable::draw(&mut world, &mut cb, VIDEO_MODE, &camera);
                sprite_drawable::draw(&mut world, &mut cb, VIDEO_MODE, &camera);
                mesh_drawable::draw(&mut world, &mut cb, VIDEO_MODE, &camera);

                {
                    if DEBUG_TRIANGLES {
                        let x_limit = 320.0;
                        let y_limit = 240.0;

                        let x_off = x_limit * 0.5;
                        let y_off = y_limit * 0.5;
                        let x_scale = x_limit * 0.125;
                        let y_scale = y_limit * 0.125;

                        let speed = 0.05; // 0.05
                        let t = speed * (frame_begin_time as f32) / 1e6;
                        let p = 2.0943951023931954923084289221863;
                        let v0 = vec3(
                            x_off + x_scale * libm::cosf(t),
                            y_off + y_scale * libm::sinf(t),
                            0.0,
                        );
                        let v1 = vec3(
                            x_off + x_scale * libm::cosf(t + p),
                            y_off + y_scale * libm::sinf(t + p),
                            0.0,
                        );
                        let v2 = vec3(
                            x_off + x_scale * libm::cosf(t - p),
                            y_off + y_scale * libm::sinf(t - p),
                            0.0,
                        );

                        cb.add_colored_rect(vec2(v0.x, v0.y), vec2(v0.x + 10.0, v0.y + 10.0), RED);
                        cb.add_colored_rect(
                            vec2(v1.x, v1.y),
                            vec2(v1.x + 10.0, v1.y + 10.0),
                            GREEN,
                        );
                        cb.add_colored_rect(vec2(v2.x, v2.y), vec2(v2.x + 10.0, v2.y + 10.0), BLUE);

                        cb.add_mesh_indexed(
                            &[v0.into(), v1.into(), v2.into()],
                            &[[0.5, 1.0], [0.0, 0.0], [1.0, 0.0]],
                            &[0xff_00_00_ff, 0x00_ff_00_ff, 0x00_00_ff_ff],
                            &[[0, 1, 2]],
                            &[
                                [1.0, 0.0, 0.0, 0.0],
                                [0.0, 1.0, 0.0, 0.0],
                                [0.0, 0.0, 1.0, 0.0],
                                [0.0, 0.0, 0.0, 1.0],
                            ],
                            None,
                        );

                        cb.add_colored_rect(vec2(v0.x, v0.y), vec2(v0.x + 2.0, v0.y + 2.0), WHITE);
                        cb.add_colored_rect(vec2(v1.x, v1.y), vec2(v1.x + 2.0, v1.y + 2.0), WHITE);
                        cb.add_colored_rect(vec2(v2.x, v2.y), vec2(v2.x + 2.0, v2.y + 2.0), WHITE);
                    }
                }

                if false {
                    font::draw_text(&mut cb, " !\"#$%&", vec2(1.0, 0.0), 0xffffffff);
                    font::draw_text(&mut cb, "'()+,-./", vec2(1.0, 17.0), 0xffffffff);
                    font::draw_text(&mut cb, "0123456789", vec2(1.0, 34.0), 0xffffffff);
                    font::draw_text(&mut cb, ":;<=>?@", vec2(1.0, 51.0), 0xffffffff);
                    font::draw_text(&mut cb, "ABCDEFGHIJ", vec2(1.0, 68.0), 0xffffffff);
                    font::draw_text(&mut cb, "KLMNOPQRST", vec2(1.0, 85.0), 0xffffffff);
                    font::draw_text(&mut cb, "UVWXYZ", vec2(1.0, 102.0), 0xffffffff);
                    font::draw_text(&mut cb, "[\\]^_`", vec2(1.0, 119.0), 0xffffffff);
                    font::draw_text(&mut cb, "abcdefghij", vec2(1.0, 136.0), 0xffffffff);
                    font::draw_text(&mut cb, "klmnopqrst", vec2(1.0, 153.0), 0xffffffff);
                    font::draw_text(&mut cb, "uvwxyz", vec2(1.0, 170.0), 0xffffffff);
                    font::draw_text(&mut cb, "{|}~", vec2(1.0, 187.0), 0xffffffff);
                }

                font::draw_number(
                    &mut cb,
                    world
                        .components
                        .get::<Player>()
                        .lookup(player)
                        .map(|p| p.score)
                        .unwrap_or(0),
                    vec2(300.0, 10.0),
                    0x0000efff,
                );
                font::draw_number(
                    &mut cb,
                    world
                        .components
                        .get::<Health>()
                        .lookup(player)
                        .map(|hc| hc.health)
                        .unwrap_or(0),
                    vec2(300.0, 215.0),
                    0xaf0000ff,
                );

                #[cfg(target_vendor = "nintendo64")]
                {
                    font::draw_number(
                        &mut cb,
                        n64_alloc::BYTES_USED.load(core::sync::atomic::Ordering::SeqCst),
                        vec2(100.0, 160.0),
                        0xff0000ff,
                    );
                    font::draw_number(
                        &mut cb,
                        n64_alloc::BYTES_LEFT.load(core::sync::atomic::Ordering::SeqCst),
                        vec2(100.0, 180.0),
                        0xff0000ff,
                    );
                    font::draw_number(
                        &mut cb,
                        *n64_alloc::PAGE_OFFSET.lock() as i32,
                        vec2(100.0, 200.0),
                        0xff0000ff,
                    );
                }

                {
                    font::draw_number(
                        &mut cb,
                        (dt * 1000.0 * 1000.0) as i32,
                        vec2(100.0, 10.0),
                        0x00af00ff,
                    );
                    font::draw_number(
                        &mut cb,
                        frame_used_time as i32,
                        vec2(200.0, 10.0),
                        0x00af00ff,
                    );
                    font::draw_number(
                        &mut cb,
                        last_colored_rect_count,
                        vec2(100.0, 30.0),
                        0x00af00ff,
                    );
                    font::draw_number(
                        &mut cb,
                        last_textured_rect_count,
                        vec2(200.0, 30.0),
                        0x00af00ff,
                    );
                }

                cb.run(&mut n64.graphics)
            };

            last_colored_rect_count = colored_rect_count;
            last_textured_rect_count = textured_rect_count;

            frame_no += 1;

            let frame_end_time = n64.graphics.swap_buffers(&mut n64.framebuffer);
            frame_used_time = frame_end_time - frame_begin_time;
        }
    }

    loop {
        {
            let mut out_tex = n64.framebuffer.next_buffer();
            slow_cpu_clear(out_tex.data);
            ipl3font::draw_str(&mut out_tex, 50, 10, RED, b"GAME OVER");
        }

        n64.graphics.swap_buffers(&mut n64.framebuffer);
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
fn panic(info: &core::panic::PanicInfo) -> ! {
    n64_macros::debugln!("{}", &info);

    const GREEN: Color = Color::new(0b00011_10000_00011_1);

    let mut out_tex = n64::gfx::TextureMut::new(VIDEO_MODE.width(), VIDEO_MODE.height(), unsafe {
        core::slice::from_raw_parts_mut(n64_sys::vi::get_vi_buffer(), VIDEO_MODE.size() as usize)
    });

    slow_cpu_clear(out_tex.data);

    ipl3font::draw_str(&mut out_tex, 15, 15, RED, b"PANIC!");
    if let Some(location) = info.location() {
        ipl3font::draw_str(
            &mut out_tex,
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
    if let Some(args) = info.message() {
        ipl3font::draw_str(
            &mut out_tex,
            15,
            45,
            GREEN,
            alloc::format!("{}", args).as_bytes(),
        );
    } else {
        ipl3font::draw_str(&mut out_tex, 15, 45, GREEN, b"No Message");
    }

    unsafe {
        n64_sys::sys::data_cache_hit_writeback(out_tex.data);
        n64_sys::vi::set_vi_buffer(out_tex.data);
    }

    loop {}
}

#[cfg(target_vendor = "nintendo64")]
#[alloc_error_handler]
fn oom(_: core::alloc::Layout) -> ! {
    let mut out_tex = n64::gfx::TextureMut::new(VIDEO_MODE.width(), VIDEO_MODE.height(), unsafe {
        core::slice::from_raw_parts_mut(n64_sys::vi::get_vi_buffer(), VIDEO_MODE.size() as usize)
    });

    slow_cpu_clear(out_tex.data);
    ipl3font::draw_str(&mut out_tex, 50, 15, RED, b"OUT OF MEMORY!");

    unsafe {
        n64_sys::sys::data_cache_hit_writeback(out_tex.data);
        n64_sys::vi::set_vi_buffer(out_tex.data);
    }

    loop {}
}

#[cfg(target_vendor = "nintendo64")]
#[lang = "eh_personality"]
extern "C" fn rust_eh_personality() {}

#[cfg(target_vendor = "nintendo64")]
core::arch::global_asm!(include_str!("entrypoint.s"));
