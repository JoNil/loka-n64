use core::slice;
use n64_math::Color;
use n64_sys::vi;

pub use n64_sys::rdp;
pub use n64_sys::vi::HEIGHT;
pub use n64_sys::vi::WIDTH;

#[inline]
pub(crate) fn init() {
    vi::init();
}

#[inline]
pub fn swap_buffers() {
    with_framebuffer(|fb| {
        unsafe { n64_sys::sys::data_cache_hit_writeback(fb) };
    });

    vi::wait_for_vblank();
    vi::swap_buffers();
}

#[inline]
pub fn with_framebuffer<F: FnOnce(&mut [Color])>(f: F) {
    let frame_buffer = unsafe {
        slice::from_raw_parts_mut(vi::next_buffer() as *mut Color, (WIDTH * HEIGHT) as usize)
    };
    f(frame_buffer);
}

#[inline]
pub fn slow_cpu_clear() {
    
    with_framebuffer(|fb| {

        let mut p = fb.as_mut_ptr() as *mut u32;

        for _ in 0..(fb.len()/2) {   
            unsafe {
                *p = 0x0001_0001;
                p = p.offset(1);
            }
        }
    });
}
