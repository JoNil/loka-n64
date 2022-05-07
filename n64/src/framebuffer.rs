use crate::{gfx::TextureMut, VideoMode};
use alloc::{boxed::Box, vec::Vec};
use n64_math::Color;

pub struct Framebuffer {
    video_mode: VideoMode,
    using_framebuffer_a: bool,
    framebuffer_a: Box<[Color]>,
    framebuffer_b: Box<[Color]>,
    depth_buffer: Box<[u16]>,
}

impl Framebuffer {
    #[inline]
    pub(crate) fn new(video_mode: VideoMode) -> Self {
        Self {
            video_mode,
            using_framebuffer_a: false,
            framebuffer_a: {
                let mut buffer = Vec::new();
                buffer.resize_with(video_mode.size() as usize, || Color::new(0x0001));
                buffer.into_boxed_slice()
            },
            framebuffer_b: {
                let mut buffer = Vec::new();
                buffer.resize_with(video_mode.size() as usize, || Color::new(0x0001));
                buffer.into_boxed_slice()
            },
            depth_buffer: {
                let mut buffer = Vec::new();
                buffer.resize_with(video_mode.size() as usize, || 0);
                buffer.into_boxed_slice()
            },
        }
    }

    #[inline]
    pub(crate) fn swap_buffer(&mut self) {
        self.using_framebuffer_a = !self.using_framebuffer_a;
    }

    #[inline]
    pub fn next_buffer(&mut self) -> (TextureMut, &mut [u16]) {
        if self.using_framebuffer_a {
            (
                TextureMut::new(
                    self.video_mode.width(),
                    self.video_mode.height(),
                    &mut self.framebuffer_a[..],
                ),
                &mut *self.depth_buffer,
            )
        } else {
            (
                TextureMut::new(
                    self.video_mode.width(),
                    self.video_mode.height(),
                    &mut self.framebuffer_b[..],
                ),
                &mut *self.depth_buffer,
            )
        }
    }
}

#[inline]
pub fn slow_cpu_clear(fb: &mut [Color]) {
    #[allow(clippy::cast_ptr_alignment)]
    let mut p = fb.as_mut_ptr() as *mut u32;

    for _ in 0..(fb.len() / 2) {
        unsafe {
            p.write_unaligned(0x0001_0001);
            p = p.offset(1);
        }
    }
}
