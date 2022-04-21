use super::rdp_command_builder::*;
use crate::gfx::{CycleType, FillPipeline, Pipeline};
use n64_math::{vec2, Color};

#[derive(Copy, Clone, Default)]
pub struct RdpState {
    other_modes: u64,
    combine_mode: u64,
    fill_color: Color,
    prim_color: u32,
    env_color: u32,
    blend_color: u32,
    texture: usize,
}

fn apply_sync_if_first_change(rdp: &mut RdpCommandBuilder, emitted_sync: &mut bool) {
    if !*emitted_sync {
        rdp.sync_pipe();
        *emitted_sync = true;
    }
}

pub fn apply_fill_pipeline(
    rdp: &mut RdpCommandBuilder,
    state: &mut RdpState,
    pipeline: &FillPipeline,
) {
    let mut emitted_sync = false;

    {
        let mut other_modes = OTHER_MODE_CYCLE_TYPE_FILL
            | OTHER_MODE_RGB_DITHER_SEL_NO_DITHER
            | OTHER_MODE_ALPHA_DITHER_SEL_NO_DITHER;

        if pipeline.blend {
            other_modes |= OTHER_MODE_FORCE_BLEND;
        }

        if other_modes != state.other_modes {
            apply_sync_if_first_change(rdp, &mut emitted_sync);
            rdp.set_other_modes(other_modes);
            state.other_modes = other_modes;
        }
    }

    {
        let combine_mode = pipeline.combiner_mode.to_command();

        if combine_mode != state.combine_mode {
            apply_sync_if_first_change(rdp, &mut emitted_sync);
            rdp.set_combine_mode(combine_mode);
            state.combine_mode = combine_mode;
        }
    }

    {
        let fill_color = pipeline.fill_color;

        if fill_color != state.fill_color {
            apply_sync_if_first_change(rdp, &mut emitted_sync);
            rdp.set_fill_color(fill_color);
            state.fill_color = fill_color;
        }
    }
}

pub fn apply_pipeline(rdp: &mut RdpCommandBuilder, state: &mut RdpState, pipeline: &Pipeline) {
    let mut emitted_sync = false;

    {
        let mut other_modes = OTHER_MODE_CYCLE_TYPE_1_CYCLE
            | OTHER_MODE_SAMPLE_TYPE
            | OTHER_MODE_BI_LERP_0
            | OTHER_MODE_RGB_DITHER_SEL_NO_DITHER
            | OTHER_MODE_ALPHA_DITHER_SEL_NO_DITHER
            | OTHER_MODE_B_M1A_0_0;

        if pipeline.cycle_type == CycleType::Two {
            other_modes |= OTHER_MODE_CYCLE_TYPE_2_CYCLE;
        }

        if pipeline.blend_color.is_some() {
            other_modes |= OTHER_MODE_B_M1A_0_2;
        }

        if pipeline.blend {
            other_modes |= OTHER_MODE_FORCE_BLEND;
        }

        if pipeline.texture.is_some() {
            other_modes |= OTHER_MODE_IMAGE_READ_EN;
        }

        if other_modes != state.other_modes {
            apply_sync_if_first_change(rdp, &mut emitted_sync);
            rdp.set_other_modes(other_modes);
            state.other_modes = other_modes;
        }
    }

    {
        let combine_mode = pipeline.combiner_mode.to_command();

        if combine_mode != state.combine_mode {
            apply_sync_if_first_change(rdp, &mut emitted_sync);
            rdp.set_combine_mode(combine_mode);
            state.combine_mode = combine_mode;
        }
    }

    if let Some(blend_color) = pipeline.blend_color {
        if blend_color != state.blend_color {
            apply_sync_if_first_change(rdp, &mut emitted_sync);
            rdp.set_blend_color(blend_color);
            state.blend_color = blend_color;
        }
    }

    if let Some(prim_color) = pipeline.prim_color {
        if prim_color != state.prim_color {
            apply_sync_if_first_change(rdp, &mut emitted_sync);
            rdp.set_prim_color(prim_color);
            state.prim_color = prim_color;
        }
    }

    if let Some(env_color) = pipeline.env_color {
        if env_color != state.env_color {
            apply_sync_if_first_change(rdp, &mut emitted_sync);
            rdp.set_env_color(env_color);
            state.env_color = env_color;
        }
    }

    if let Some(texture) = pipeline.texture {
        if state.texture != texture.data.as_ptr() as usize {
            rdp.sync_tile()
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
                    vec2((texture.width) as f32, (texture.height) as f32),
                    vec2(0.0, 0.0),
                    0,
                );
            state.texture = texture.data.as_ptr() as usize;
        }
    }
}
