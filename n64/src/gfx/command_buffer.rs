use super::{Texture, TextureMut};
use crate::graphics::Graphics;
use n64_math::{Color, Vec2};
use n64_sys::rdp;
use rdp_command_builder::*;

mod rdp_command_builder;

pub struct CommandBuffer<'a> {
    out_tex: &'a mut TextureMut<'a>,
    rdp: RdpCommandBuilder,
}

impl<'a> CommandBuffer<'a> {
    pub fn new(out_tex: &'a mut TextureMut<'a>) -> Self {
        let mut rdp = RdpCommandBuilder::new();
        rdp.set_color_image(
            FORMAT_RGBA,
            SIZE_OF_PIXEL_16B,
            out_tex.width as u16,
            out_tex.data.as_mut_ptr() as *mut u16,
        )
        .set_scissor(
            Vec2::zero(),
            Vec2::new((out_tex.width - 1) as f32, (out_tex.height - 1) as f32),
        )
        .set_combine_mode(&[0, 0, 0, 0, 6, 1, 0, 15, 1, 0, 0, 0, 0, 7, 7, 7]);

        CommandBuffer { out_tex, rdp }
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
            .fill_rectangle(
                Vec2::new(0.0, 0.0),
                Vec2::new(
                    (self.out_tex.width - 1) as f32,
                    (self.out_tex.height - 1) as f32,
                ),
            );

        self
    }

    pub fn add_colored_rect(
        &mut self,
        upper_left: Vec2,
        lower_right: Vec2,
        color: Color,
    ) -> &mut Self {
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
            .fill_rectangle(upper_left, lower_right - Vec2::new(1.0, 1.0));

        self
    }

    pub fn add_textured_rect(
        &mut self,
        upper_left: Vec2,
        lower_right: Vec2,
        texture: Texture<'static>,
    ) -> &mut Self {
        self.rdp
            .sync_tile()
            .set_other_modes(
                OTHER_MODE_SAMPLE_TYPE
                    | OTHER_MODE_BI_LERP_0
                    | OTHER_MODE_ALPHA_DITHER_SEL_NO_DITHER
                    | OTHER_MODE_B_M2A_0_1
                    | OTHER_MODE_FORCE_BLEND
                    | OTHER_MODE_IMAGE_READ_EN,
            )
            .set_texture_image(
                FORMAT_RGBA,
                SIZE_OF_PIXEL_16B,
                texture.width as u16,
                texture.data.as_ptr() as *const u16,
            )
            .set_tile(
                FORMAT_RGBA,
                SIZE_OF_PIXEL_16B,
                texture.width as u16,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            )
            .load_tile(
                Vec2::new((texture.width - 1) as f32, (texture.height - 1) as f32),
                Vec2::new(0.0, 0.0),
                0,
            )
            .texture_rectangle(
                upper_left,
                lower_right - Vec2::new(1.0, 1.0),
                0,
                Vec2::new(0.0, 0.0),
                Vec2::new(32.0, 32.0),
            );
        self
    }

    pub fn run(mut self, _graphics: &mut Graphics) {
        self.rdp.sync_full();
        let commands = self.rdp.build();

        unsafe { rdp::run_command_buffer(commands) };

        unsafe { n64_sys::sys::data_cache_hit_invalidate(self.out_tex.data) };
    }
}
