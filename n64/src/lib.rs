#![cfg_attr(target_vendor = "nintendo64", no_std)]

extern crate alloc;

pub use controllers::Controllers;

pub mod gfx;
pub mod ipl3font;

cfg_if::cfg_if! {
    if #[cfg(target_vendor = "nintendo64")] {
        pub mod audio;
        pub mod graphics;
        pub mod controllers;
    } else {
        pub mod audio_emu;
        pub mod graphics_emu;
        pub mod controllers_emu;

        pub use audio_emu as audio;
        pub use graphics_emu as graphics;
        pub use controllers_emu as controllers;
    }
}

#[inline]
pub fn init() {
    audio::init();
    graphics::init();
}

#[inline]
#[cfg(target_vendor = "nintendo64")]
pub fn current_time_us() -> i64 {
    n64_sys::sys::current_time_us()
}

#[inline]
#[cfg(not(target_vendor = "nintendo64"))]
pub fn current_time_us() -> i64 {
    use std::time::Instant;

    thread_local! {
        static BEGINNING: Instant = Instant::now();
    }

    BEGINNING.with(|beginning| (beginning.elapsed().as_secs_f64() * 1000.0 * 1000.0) as i64)
}
