use n64_sys::vi;

pub use n64_sys::vi::HEIGHT;
pub use n64_sys::vi::WIDTH;

#[inline]
pub fn wait_for_vblank() {
    vi::wait_for_vblank();
}

#[inline]
pub fn swap_buffers() {
    vi::swap_buffers()
}

#[inline]
pub fn clear_buffer() {
    let frame_buffer = vi::next_buffer() as usize;
    for i in 0..vi::WIDTH * vi::HEIGHT {
        let p = (frame_buffer + i * 4) as *mut u32;
        unsafe {
            *p = 0x1001_1001;
        }
    }
}
