use core::slice;
use n64_math::Color;
use n64_sys::vi;

pub use n64_sys::vi::HEIGHT;
pub use n64_sys::vi::WIDTH;

#[inline]
pub(crate) fn init() {
    vi::init();
}

#[inline]
pub fn swap_buffers() {
    vi::wait_for_vblank();
    vi::swap_buffers();
}

#[inline]
pub fn with_framebuffer<F: FnOnce(&mut [Color])>(f: F) {
    let frame_buffer = unsafe {
        slice::from_raw_parts_mut(vi::next_buffer() as *mut Color, (WIDTH*HEIGHT) as usize)
    };
    f(frame_buffer);
}

pub fn clear_buffer() {
    with_framebuffer(|fb| {
        fb.iter_mut().for_each(|v| *v = Color::new(0b00001_00001_00001_1));
    });
}