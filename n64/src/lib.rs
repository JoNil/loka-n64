#![cfg_attr(target_vendor = "nintendo64", no_std)]

extern crate alloc;

pub use audio::Audio;
pub use controllers::Controllers;
pub use framebuffer::Framebuffer;
pub use graphics::Graphics;

pub use n64_macros::*;
pub use n64_profiler::*;
pub use n64_sys::*;
pub use n64_types::*;

#[cfg(target_vendor = "nintendo64")]
pub use n64_alloc::*;

pub mod gfx;
pub mod ipl3font;
pub mod utils;

mod framebuffer;

mod audio_n64;
mod controllers_n64;
mod graphics_n64;

#[cfg(not(target_vendor = "nintendo64"))]
pub mod audio_emu;
#[cfg(not(target_vendor = "nintendo64"))]
pub mod controllers_emu;
#[cfg(not(target_vendor = "nintendo64"))]
pub mod graphics_emu;

#[cfg(target_vendor = "nintendo64")]
use audio_n64 as audio;
#[cfg(target_vendor = "nintendo64")]
use controllers_n64 as controllers;
#[cfg(target_vendor = "nintendo64")]
use graphics_n64 as graphics;

#[cfg(not(target_vendor = "nintendo64"))]
use audio_emu as audio;
#[cfg(not(target_vendor = "nintendo64"))]
use controllers_emu as controllers;
#[cfg(not(target_vendor = "nintendo64"))]
use graphics_emu as graphics;

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

        #[cfg(target_vendor = "nintendo64")]
        n64_sys::pi::init();

        #[cfg(target_vendor = "nintendo64")]
        n64_sys::ed::init();

        N64 {
            audio,
            framebuffer,
            graphics,
            controllers,
        }
    }
}

#[cfg(target_vendor = "nintendo64")]
mod inner {
    #[inline]
    pub fn current_time_us() -> i64 {
        n64_sys::sys::current_time_us()
    }
}

#[cfg(not(target_vendor = "nintendo64"))]
mod inner {

    use once_cell::sync::Lazy;
    use std::time::Instant;

    static BEGINNING: Lazy<Instant> = Lazy::new(Instant::now);

    #[inline]
    pub fn current_time_us() -> i64 {
        (BEGINNING.elapsed().as_secs_f64() * 1000.0 * 1000.0) as i64
    }
}

#[cfg(target_vendor = "nintendo64")]
pub use inner::*;

#[cfg(not(target_vendor = "nintendo64"))]
pub use inner::*;

#[inline]
pub fn slow_cpu_clear(fb: &mut [n64_math::Color]) {
    #[allow(clippy::cast_ptr_alignment)]
    let mut p = fb.as_mut_ptr() as *mut u32;

    for _ in 0..(fb.len() / 2) {
        unsafe {
            p.write_unaligned(0x0001_0001);
            p = p.offset(1);
        }
    }
}
