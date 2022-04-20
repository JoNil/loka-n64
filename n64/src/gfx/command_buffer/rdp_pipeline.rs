use super::rdp_command_builder::*;
use crate::gfx::pipeline::Pipeline;

pub fn apply(rdp: &mut RdpCommandBuilder, new: &Pipeline) {
    let other_modes = OTHER_MODE_CYCLE_TYPE_1_CYCLE
        | OTHER_MODE_SAMPLE_TYPE
        | OTHER_MODE_BI_LERP_0
        | OTHER_MODE_ALPHA_DITHER_SEL_NO_DITHER
        | OTHER_MODE_B_M1A_0_0;

    rdp.sync_pipe()
        .set_other_modes(other_modes)
        .set_combine_mode(new.combiner_mode.to_command());

    if let Some(prim_color) = new.prim_color {
        rdp.set_prim_color(prim_color);
    }

    if let Some(env_color) = new.env_color {
        rdp.set_prim_color(env_color);
    }

    if let Some(blend_color) = new.blend_color {
        rdp.set_prim_color(blend_color);
    }
}
