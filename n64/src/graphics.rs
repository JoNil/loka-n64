use core::slice;
use n64_math::Color;
use n64_sys::vi;
use crate::{framebuffer::Framebuffer, VideoMode};

struct Graphics {

}

impl Graphics {

    #[inline]
    pub(crate) fn new(video_mode: VideoMode, framebuffer: &mut Framebuffer) -> Self {
        vi::init(vi, framebuffer.next_buffer());
        Self
    }

    #[inline]
    pub fn swap_buffers(&mut self, framebuffer: &mut Framebuffer) {
        
        let fb = framebuffer.next_buffer();

        unsafe { n64_sys::sys::data_cache_hit_writeback(fb) };

        vi::wait_for_vblank();
        vi::set_vi_buffers(fb);
    }
}