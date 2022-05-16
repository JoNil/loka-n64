use crate::{current_time_us, framebuffer::Framebuffer, VideoMode};
use n64_sys::{rdp, vi};

pub struct Graphics {}

impl Graphics {
    #[inline]
    pub(crate) fn new(video_mode: VideoMode, framebuffer: &mut Framebuffer) -> Self {
        vi::init(video_mode, &mut framebuffer.vi_buffer.0);
        rdp::init();
        Self {}
    }

    #[inline]
    pub fn swap_buffers(&mut self, framebuffer: &mut Framebuffer) -> i64 {
        rdp::wait_for_done();

        framebuffer.swap();

        let frame_end_time = current_time_us();

        vi::wait_for_vblank();
        unsafe { vi::set_vi_buffer(&mut framebuffer.vi_buffer.0) };

        frame_end_time
    }
}
