use n64_sys::vi;
use crate::{framebuffer::Framebuffer, VideoMode, current_time_us};

pub struct Graphics {}

impl Graphics {

    #[inline]
    pub(crate) fn new(video_mode: VideoMode, framebuffer: &mut Framebuffer) -> Self {
        vi::init(video_mode, framebuffer.next_buffer().data);
        Self {}
    }

    #[inline]
    pub fn swap_buffers(&mut self, framebuffer: &mut Framebuffer) -> i64 {
        
        let fb = framebuffer.next_buffer();

        unsafe { n64_sys::sys::data_cache_hit_writeback(fb.data) };

        let frame_end_time = current_time_us();

        vi::wait_for_vblank();
        unsafe { vi::set_vi_buffer(fb.data) };

        framebuffer.swap_buffer();

        frame_end_time
    }
}