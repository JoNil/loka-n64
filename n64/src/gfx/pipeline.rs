use super::{color_combiner::ColorCombiner, Texture};

pub enum CycleType {
    One,
    Two,
}

pub struct Pipeline {
    pub cycle_type: CycleType,

    pub combiner_mode: ColorCombiner,
    pub prim_color: Option<u32>,
    pub env_color: Option<u32>,

    pub blend_color: Option<u32>,
    pub texture: Option<Texture<'static>>,
}

impl Default for Pipeline {
    fn default() -> Self {
        Self {
            cycle_type: CycleType::One,
            combiner_mode: Default::default(),
            prim_color: Default::default(),
            env_color: Default::default(),
            blend_color: Default::default(),
            texture: Default::default(),
        }
    }
}
