use core::slice;
use n64_math::{Color, Vec2};
use n64_sys::vi;

pub use n64_sys::rdp_command_builder::{RdpCommandBuilder, other_modes::*};
pub use n64_sys::rdp;
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

        /*let cb = RdpCommandBuilder::new()
            .set_color_image(fb.as_mut_ptr() as *mut u16)
            .set_scissor(Vec2::zero(), Vec2::new(WIDTH as f32, HEIGHT as f32))
            .set_other_modes(
                CYCLE_TYPE_FILL | CYCLE_TYPE_COPY | CYCLE_TYPE_2_CYCLE |
                RGB_DITHER_SEL_NO_DITHER | ALPHA_DITHER_SEL_NO_DITHER |
                FORCE_BLEND)
            .set_fill_color(Color::new(0b11000_00111_00000_1))
            .fill_rectangle(Vec2::new(10.0, 10.0), Vec2::new(WIDTH as f32 / 2.0, HEIGHT as f32 / 2.0))
            .sync_full()
            .build();

        unsafe { rdp::run_command_buffer(cb) };*/
        
        let mut p = fb.as_mut_ptr() as *mut u32;

        for _ in 0..(fb.len()/2) {   
            unsafe {
                *p =  0x0001_0001;
                p = p.offset(1);
            }
        }
    });
}