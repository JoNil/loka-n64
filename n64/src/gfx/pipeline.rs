use super::{color_combiner::ColorCombiner, Texture};

#[derive(Default)]
pub struct Pipeline {
    pub combiner_mode: ColorCombiner,
    pub prim_color: Option<u32>,
    pub env_color: Option<u32>,

    pub blend_color: Option<u32>,
    pub texture: Option<Texture<'static>>,
}
