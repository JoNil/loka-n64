#![cfg_attr(target_vendor = "nintendo64", no_std)]

extern crate alloc;

pub use audio::{Audio, BUFFER_NO_SAMPLES};
pub use controllers::Controllers;
pub use framebuffer::{slow_cpu_clear, Framebuffer};
pub use graphics::Graphics;
pub use video_mode::VideoMode;

pub mod gfx;
pub mod ipl3font;

mod framebuffer;
mod video_mode;

cfg_if::cfg_if! {
    if #[cfg(target_vendor = "nintendo64")] {
        mod audio;
        mod graphics;
        mod controllers;
    } else {
        pub mod audio_emu;
        pub mod graphics_emu;
        pub mod controllers_emu;

        use audio_emu as audio;
        use graphics_emu as graphics;
        use controllers_emu as controllers;
    }
}

pub struct N64 {
    pub audio: Audio,
    pub framebuffer: Framebuffer,
    pub graphics: Graphics,
    pub controllers: Controllers,
}

impl N64 {
    #[inline]
    pub fn new(video_mode: VideoMode) -> N64 {
        let audio = Audio::new();
        let mut framebuffer = Framebuffer::new(video_mode);
        let graphics = Graphics::new(video_mode, &mut framebuffer);
        let controllers = Controllers::new();

        N64 {
            audio,
            framebuffer,
            graphics,
            controllers,
        }
    }
}

cfg_if::cfg_if! {
    if #[cfg(target_vendor = "nintendo64")] {
        #[inline]
        pub fn current_time_us() -> i64 {
            n64_sys::sys::current_time_us()
        }
    } else {

        use lazy_static::lazy_static;
        use std::time::Instant;

        lazy_static! {
            static ref BEGINNING: Instant = Instant::now();
        }

        #[inline]
        pub fn current_time_us() -> i64 {
            (BEGINNING.elapsed().as_secs_f64() * 1000.0 * 1000.0) as i64
        }
    }
}
