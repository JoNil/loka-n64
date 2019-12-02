use n64_sys::vi;

pub use n64_sys::vi::WIDTH;
pub use n64_sys::vi::HEIGHT;

#[inline]
pub fn wait_for_vblank() {
    vi::wait_for_vblank();
}

#[inline]
pub fn next_buffer() -> *mut u16 {
    vi::next_buffer()
}

#[inline]
pub fn swap_buffer() {
    vi::swap_buffer()
}