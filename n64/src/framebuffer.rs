use crate::{gfx::TextureMut, VideoMode};
use alloc::{boxed::Box, vec::Vec};
use core::mem;
use n64_math::Color;

pub struct ViFramebuffer(pub(crate) Box<[Color]>);
pub struct ViBufferToken(pub(crate) *mut Color);
pub struct GpuFramebuffer(pub(crate) Box<[Color]>);

pub struct Framebuffer {
    video_mode: VideoMode,
    pub(crate) vi_buffer: ViFramebuffer,
    pub(crate) gpu_buffer: GpuFramebuffer,
}

impl Framebuffer {
    #[inline]
    pub(crate) fn new(video_mode: VideoMode) -> Self {
        Framebuffer {
            video_mode,
            vi_buffer: ViFramebuffer({
                let mut buffer = Vec::new();
                buffer.resize_with(video_mode.size() as usize, || Color::new(0x0001));
                buffer.into_boxed_slice()
            }),
            gpu_buffer: GpuFramebuffer({
                let mut buffer = Vec::new();
                buffer.resize_with(video_mode.size() as usize, || Color::new(0x0001));
                buffer.into_boxed_slice()
            }),
        }
    }

    pub fn gpu_buffer(&mut self) -> TextureMut {
        TextureMut {
            width: self.video_mode.width(),
            height: self.video_mode.height(),
            data: &mut self.gpu_buffer.0,
        }
    }

    #[inline]
    pub(crate) fn swap(&mut self) {
        mem::swap(&mut self.vi_buffer.0, &mut self.gpu_buffer.0)
    }

    pub fn vi_buffer_token(&mut self) -> ViBufferToken {
        ViBufferToken(self.vi_buffer.0.as_mut_ptr())
    }
}
