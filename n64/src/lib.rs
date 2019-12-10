#![cfg_attr(target_vendor = "nintendo64", no_std)]

pub mod ipl3font;
pub mod rand;

#[cfg_attr(target_vendor = "nintendo64", path = "audio.rs")]
#[cfg_attr(not(target_vendor = "nintendo64"), path = "audio_emu.rs")]
pub mod audio;

#[cfg_attr(target_vendor = "nintendo64", path = "graphics.rs")]
#[cfg_attr(not(target_vendor = "nintendo64"), path = "graphics_emu.rs")]
pub mod graphics;

#[cfg_attr(target_vendor = "nintendo64", path = "controllers.rs")]
#[cfg_attr(not(target_vendor = "nintendo64"), path = "controllers_emu.rs")]
pub mod controllers;

pub use controllers::Controllers;
pub use rand::Rng;

pub fn init() {
    audio::init();
    graphics::init();
}

#[inline]
#[cfg(target_vendor = "nintendo64")]
pub fn current_time_us() -> i32 {
    n64_sys::sys::current_time_us()
}

#[inline]
#[cfg(not(target_vendor = "nintendo64"))]
pub fn current_time_us() -> i32 {

    use std::time::Instant;

    thread_local! {
        static BEGINNING: Instant = Instant::now();
    }

    BEGINNING.with(|beginning| {
        (beginning.elapsed().as_secs_f64() * 1000.0 * 1000.0) as i32
    })
}