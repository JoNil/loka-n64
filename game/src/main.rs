#![cfg_attr(target_vendor = "nintendo64", feature(alloc_error_handler))]
#![cfg_attr(target_vendor = "nintendo64", feature(asm_experimental_arch))]
#![cfg_attr(target_vendor = "nintendo64", feature(lang_items))]
#![cfg_attr(target_vendor = "nintendo64", feature(panic_info_message))]
#![cfg_attr(target_vendor = "nintendo64", feature(start))]
#![cfg_attr(target_vendor = "nintendo64", no_std)]
#![allow(clippy::inconsistent_digit_grouping)]
#![allow(clippy::too_many_arguments)]

extern crate alloc;

use crate::components::{
    box_drawable, enemy,
    health::{self, Health},
    mesh_drawable, missile, movable,
    player::{self, spawn_player, Player},
    print_position, projectile, sprite_drawable, waypoint_ai,
};
use camera::Camera;
use components::{
    keep_on_screen,
    pickup::{self, spawn_pickup},
    player::draw_player_weapon,
    remove_when_below, shadow, spawner,
    weapon::draw_missile_target,
};
use ecs::world::World;
use map::Map;
use maps::MAP_1;
use n64::{
    self, current_time_us,
    gfx::{CommandBuffer, CommandBufferCache, FillPipeline, Pipeline},
    ipl3font, slow_cpu_clear, VideoMode, N64,
};
use n64_math::{random_u32, vec2, vec3, Color};
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

const RED: Color = Color::new(0b10000_00011_00011_1);
const GREEN: Color = Color::new(0b00011_10000_00011_1);
const BLUE: Color = Color::new(0b0011_00011_10000_1);
const WHITE: Color = Color::new(0b11111_11111_11111_1);

static DEBUG_PIPELINE: FillPipeline = FillPipeline::default();

const VIDEO_MODE: VideoMode = VideoMode::Pal {
    width: 320,
    height: 240,
};

const DEBUG_TRIANGLES: bool = false;

fn main() {
    n64::init_profiler();

    let mut n64 = N64::new(VIDEO_MODE);

    let mut world = World::new();
    let map = Map::load(MAP_1);

    let start_pos = vec2(
        map.get_start_pos().x / VIDEO_MODE.width() as f32,
        map.get_start_pos().y / VIDEO_MODE.height() as f32 - 1.0,
    );

    let mut sound_mixer = SoundMixer::new();
    let mut camera = Camera::new(start_pos);
    let mut command_buffer_cache = CommandBufferCache::new(VIDEO_MODE);

    let _test_pickup = spawn_pickup(&mut world.entities, start_pos + vec2(0.5, 0.2));

    let player = spawn_player(&mut world.entities, start_pos);

    map.spawn_enemies(&mut world, &VIDEO_MODE);

    let mut frame_begin_time;
    let mut last_frame_begin_time = current_time_us();
    let mut swap_time = 0;
    let mut dt;

    let mut last_colored_rect_count = 0;
    let mut last_textured_rect_count = 0;
    let mut last_mesh_count = 0;

    loop {
        n64::frame!();
        n64::scope!("Frame");

        {
            frame_begin_time = current_time_us();
            dt = (frame_begin_time - last_frame_begin_time) as f32 / 1e6;
            last_frame_begin_time = frame_begin_time;
        }

        {
            n64::scope!("Update");

            n64.controllers.update(&n64.graphics);

            camera.update(&n64.controllers, dt, &VIDEO_MODE);

            health::clear_was_damaged(&mut world);

            enemy::update(&mut world, &mut sound_mixer);
            player::update(&mut world, &n64.controllers, &mut sound_mixer, &camera);

            waypoint_ai::update(&mut world, dt);
            missile::update(&mut world, dt);

            movable::simulate(&mut world, dt);

            projectile::update(&mut world, &mut sound_mixer, &camera, dt);
            pickup::update(&mut world, &mut sound_mixer, &camera);
            spawner::update(&mut world, &camera);
            keep_on_screen::update(&mut world, &camera);
            remove_when_below::update(&mut world, &camera);
            print_position::print(&mut world);
        }

        {
            n64::scope!("Audio");

            n64.audio.update(|buffer| {
                sound_mixer.mix(buffer);
            });
        }

        let cb = {
            n64::scope!("Build Command Buffer");

            let mut cb =
                CommandBuffer::new(n64.framebuffer.vi_buffer_token(), &mut command_buffer_cache);

            cb.clear();

            map.render(&mut cb, VIDEO_MODE, &camera);

            shadow::draw(&mut world, &mut cb, VIDEO_MODE, &camera);

            box_drawable::draw(&mut world, &mut cb, VIDEO_MODE, &camera);
            sprite_drawable::draw(&mut world, &mut cb, VIDEO_MODE, &camera);
            mesh_drawable::draw(&mut world, &mut cb, VIDEO_MODE, &camera);

            draw_missile_target(&mut world, &mut cb, VIDEO_MODE, &camera);

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
                    let p = 2.0943951;
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

                    cb.set_fill_pipeline(&DEBUG_PIPELINE.with_fill_color(RED));
                    cb.add_colored_rect(vec2(v0.x, v0.y), vec2(v0.x + 10.0, v0.y + 10.0));
                    cb.set_fill_pipeline(&DEBUG_PIPELINE.with_fill_color(GREEN));
                    cb.add_colored_rect(vec2(v1.x, v1.y), vec2(v1.x + 10.0, v1.y + 10.0));
                    cb.set_fill_pipeline(&DEBUG_PIPELINE.with_fill_color(BLUE));
                    cb.add_colored_rect(vec2(v2.x, v2.y), vec2(v2.x + 10.0, v2.y + 10.0));

                    cb.set_pipeline(&Pipeline::default());
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
                    );

                    cb.set_fill_pipeline(&DEBUG_PIPELINE.with_fill_color(WHITE));
                    cb.add_colored_rect(vec2(v0.x, v0.y), vec2(v0.x + 2.0, v0.y + 2.0));
                    cb.add_colored_rect(vec2(v1.x, v1.y), vec2(v1.x + 2.0, v1.y + 2.0));
                    cb.add_colored_rect(vec2(v2.x, v2.y), vec2(v2.x + 2.0, v2.y + 2.0));
                }
            }

            {
                n64::scope!("HUD");

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
                        n64::ALLOC_BYTES_USED.load(core::sync::atomic::Ordering::SeqCst),
                        vec2(100.0, 160.0),
                        0xff0000ff,
                    );
                    font::draw_number(
                        &mut cb,
                        n64::ALLOC_BYTES_LEFT.load(core::sync::atomic::Ordering::SeqCst),
                        vec2(100.0, 180.0),
                        0xff0000ff,
                    );
                    font::draw_number(
                        &mut cb,
                        *n64::ALLOC_PAGE_OFFSET.lock() as i32,
                        vec2(100.0, 200.0),
                        0xff0000ff,
                    );
                }

                {
                    font::draw_number(&mut cb, (dt * 1e6) as i32, vec2(100.0, 10.0), 0x00af00ff);
                    font::draw_number(
                        &mut cb,
                        (dt * 1e6) as i32 - swap_time as i32,
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
                    font::draw_number(&mut cb, last_mesh_count, vec2(300.0, 30.0), 0xaf0000ff);
                }

                draw_player_weapon(&mut world, &mut cb, &VIDEO_MODE);
            }

            cb
        };

        swap_time = {
            n64::scope!("Swap");
            n64.graphics.swap_buffers(&mut n64.framebuffer)
        };

        let (colored_rect_count, textured_rect_count, mesh_count) = {
            n64::scope!("Submit Command Buffer");
            let cb = cb;
            cb.submit(&mut n64.graphics)
        };

        last_colored_rect_count = colored_rect_count;
        last_textured_rect_count = textured_rect_count;
        last_mesh_count = mesh_count;

        {
            // Cycle the random number generator
            let now = current_time_us() as u64;
            for _ in 0..(now & 0xf) {
                let _ = random_u32();
            }
        }

        {
            n64::scope!("Housekeep");
            world.housekeep();
        }

        if !health::is_alive(world.components.get::<Health>(), player) {
            break;
        }
    }

    loop {
        {
            let mut out_tex = n64.framebuffer.gpu_buffer();
            slow_cpu_clear(out_tex.data);
            ipl3font::draw_str(&mut out_tex, 50, 10, RED, b"GAME OVER");
        }

        n64.graphics.swap_buffers(&mut n64.framebuffer);
    }
}

#[cfg(target_vendor = "nintendo64")]
#[global_allocator]
static ALLOC: n64::N64Alloc = n64::N64Alloc::INIT;

#[cfg(target_vendor = "nintendo64")]
#[start]
fn start(_argc: isize, _argv: *const *const u8) -> isize {
    main();
    0
}

#[cfg(target_vendor = "nintendo64")]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    n64::debugln!("{}", &info);

    const GREEN: Color = Color::new(0b00011_10000_00011_1);

    let mut out_tex = n64::gfx::TextureMut::new(VIDEO_MODE.width(), VIDEO_MODE.height(), unsafe {
        core::slice::from_raw_parts_mut(n64::vi::get_vi_buffer(), VIDEO_MODE.size() as usize)
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
                location.file().rsplit('\\').next().unwrap_or(""),
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
        n64::sys::data_cache_hit_writeback(out_tex.data);
        n64::vi::set_vi_buffer(out_tex.data);
    }

    #[allow(clippy::empty_loop)]
    loop {}
}

#[cfg(target_vendor = "nintendo64")]
#[alloc_error_handler]
fn oom(_: core::alloc::Layout) -> ! {
    let mut out_tex = n64::gfx::TextureMut::new(VIDEO_MODE.width(), VIDEO_MODE.height(), unsafe {
        core::slice::from_raw_parts_mut(n64::vi::get_vi_buffer(), VIDEO_MODE.size() as usize)
    });

    slow_cpu_clear(out_tex.data);
    ipl3font::draw_str(&mut out_tex, 50, 15, RED, b"OUT OF MEMORY!");

    unsafe {
        n64::sys::data_cache_hit_writeback(out_tex.data);
        n64::vi::set_vi_buffer(out_tex.data);
    }

    #[allow(clippy::empty_loop)]
    loop {}
}

#[cfg(target_vendor = "nintendo64")]
#[lang = "eh_personality"]
extern "C" fn rust_eh_personality() {}

#[cfg(target_vendor = "nintendo64")]
core::arch::global_asm!(include_str!("entrypoint.s"));
