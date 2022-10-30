use crate::{current_time_us, framebuffer::Framebuffer, VideoMode};
use n64_sys::{rdp, rsp, vi};

pub struct Graphics {}

impl Graphics {
    #[inline]
    pub(crate) fn new(video_mode: VideoMode, framebuffer: &mut Framebuffer) -> Self {
        vi::init(video_mode, &mut framebuffer.vi_buffer.0);
        rdp::init();
        rsp::init();
        Self {}
    }

    #[inline]
    pub fn swap_buffers(&mut self, framebuffer: &mut Framebuffer) -> i64 {
        rdp::wait_for_done();

        framebuffer.swap();

        let swap_start = current_time_us();
        vi::wait_for_vblank();
        let swap_end = current_time_us();
        unsafe { vi::set_vi_buffer(&mut framebuffer.vi_buffer.0) };

        swap_end - swap_start
    }
}
