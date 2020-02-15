use core::slice;
use n64_math::{Color, Vec2};
use n64_sys::vi;

pub use n64_sys::rdp;
pub use n64_sys::rdp_command_builder::{other_modes::*, RdpCommandBuilder};
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

pub struct CommandBuffer {
    rdp: RdpCommandBuilder,
}

impl CommandBuffer {
    pub fn new() -> Self {
        let mut rdp = RdpCommandBuilder::new();
        rdp.set_color_image(unsafe { vi::next_buffer() })
            .set_scissor(Vec2::zero(), Vec2::new(WIDTH as f32, HEIGHT as f32));

        CommandBuffer { rdp: rdp }
    }

    pub fn clear(&mut self) -> &mut Self {
        self.rdp
            .set_other_modes(
                CYCLE_TYPE_FILL
                    | CYCLE_TYPE_COPY
                    | CYCLE_TYPE_2_CYCLE
                    | RGB_DITHER_SEL_NO_DITHER
                    | ALPHA_DITHER_SEL_NO_DITHER
                    | FORCE_BLEND,
            )
            .set_fill_color(Color::new(0b00000_00000_00000_1))
            .fill_rectangle(Vec2::new(0.0, 0.0), Vec2::new(WIDTH as f32, HEIGHT as f32));

        self
    }

    pub fn run(mut self) {
        self.rdp.sync_full();
        let commands = self.rdp.build();

        unsafe { rdp::run_command_buffer(commands) };

        with_framebuffer(|fb| {
            unsafe { n64_sys::sys::data_cache_hit_invalidate(fb) };
        });
    }
}
