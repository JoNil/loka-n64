use n64_math::{Vec2, Color};
use rdp_command_builder::*;
use crate::graphics::{WIDTH, HEIGHT};

#[cfg(target_vendor = "nintendo64")]
use n64_sys::rdp;

mod rdp_command_builder;

pub struct CommandBuffer<'a> {
    framebuffer: &'a mut [Color],
    rdp: RdpCommandBuilder,
}

impl<'a> CommandBuffer<'a> {
    pub fn new(framebuffer: &'a mut [Color]) -> Self {
        let mut rdp = RdpCommandBuilder::new();
        rdp.set_color_image(framebuffer.as_mut_ptr() as *mut u16)
            .set_scissor(Vec2::zero(), Vec2::new(WIDTH as f32, HEIGHT as f32));

        CommandBuffer { framebuffer, rdp }
    }

    pub fn clear(&mut self) -> &mut Self {
        self.rdp
            .set_other_modes(
                OTHER_MODE_CYCLE_TYPE_FILL
                    | OTHER_MODE_CYCLE_TYPE_COPY
                    | OTHER_MODE_CYCLE_TYPE_2_CYCLE
                    | OTHER_MODE_RGB_DITHER_SEL_NO_DITHER
                    | OTHER_MODE_ALPHA_DITHER_SEL_NO_DITHER
                    | OTHER_MODE_FORCE_BLEND,
            )
            .set_fill_color(Color::new(0b00000_00000_00000_1))
            .fill_rectangle(Vec2::new(0.0, 0.0), Vec2::new(WIDTH as f32, HEIGHT as f32));

        self
    }

    pub fn add_rect(&mut self, upper_left: Vec2, lower_right: Vec2, color: Color) -> &mut Self {
        self.rdp
            .sync_pipe()
            .set_other_modes(
                OTHER_MODE_CYCLE_TYPE_FILL
                    | OTHER_MODE_CYCLE_TYPE_COPY
                    | OTHER_MODE_CYCLE_TYPE_2_CYCLE
                    | OTHER_MODE_RGB_DITHER_SEL_NO_DITHER
                    | OTHER_MODE_ALPHA_DITHER_SEL_NO_DITHER
                    | OTHER_MODE_FORCE_BLEND,
            )
            .set_fill_color(color)
            .fill_rectangle(upper_left, lower_right);

        self
    }

    pub fn run(mut self) {
        self.rdp.sync_full();
        let commands = self.rdp.build();

        #[cfg(target_vendor = "nintendo64")]
        {
            unsafe { rdp::run_command_buffer(commands) };

            unsafe { n64_sys::sys::data_cache_hit_invalidate(self.framebuffer) };
        }
    }
}