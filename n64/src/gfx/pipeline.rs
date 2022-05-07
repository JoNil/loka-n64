use super::{blender::Blender, color_combiner::ColorCombiner, Texture};
use n64_math::Color;

#[derive(Copy, Clone)]
pub struct FillPipeline {
    pub combiner_mode: ColorCombiner,
    pub fill_color: Color,
    pub blend: bool,
}

impl FillPipeline {
    pub const fn default() -> Self {
        Self {
            combiner_mode: ColorCombiner::default(),
            fill_color: Color::new(0),
            blend: false,
        }
    }

    pub fn with_combiner_mode(&self, combiner_mode: ColorCombiner) -> Self {
        let mut res = *self;
        res.combiner_mode = combiner_mode;
        res
    }

    pub fn with_fill_color(&self, fill_color: Color) -> Self {
        let mut res = *self;
        res.fill_color = fill_color;
        res
    }

    pub fn with_blend(&self, blend: bool) -> Self {
        let mut res = *self;
        res.blend = blend;
        res
    }
}

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum CycleType {
    One,
    Two,
}

#[derive(Copy, Clone)]
pub struct Pipeline {
    pub cycle_type: CycleType,
    pub combiner_mode: ColorCombiner,
    pub blender: Blender,

    pub texture: Option<Texture<'static>>,

    pub prim_color: Option<u32>,
    pub env_color: Option<u32>,
    pub blend_color: Option<u32>,

    pub blend: bool,
    pub z_update: bool,
    pub z_compare: bool,
}

impl Pipeline {
    pub const fn default() -> Self {
        Self {
            cycle_type: CycleType::One,
            combiner_mode: ColorCombiner::default(),
            blender: Blender::default(),
            prim_color: None,
            env_color: None,
            blend_color: None,
            texture: None,
            blend: false,
            z_update: false,
            z_compare: false,
        }
    }

    pub fn with_cycle_type(&self, cycle_type: CycleType) -> Self {
        let mut res = *self;
        res.cycle_type = cycle_type;
        res
    }

    pub fn with_combiner_mode(&self, combiner_mode: ColorCombiner) -> Self {
        let mut res = *self;
        res.combiner_mode = combiner_mode;
        res
    }

    pub fn with_texture(&self, texture: Option<Texture<'static>>) -> Self {
        let mut res = *self;
        res.texture = texture;
        res
    }

    pub fn with_prim_color(&self, prim_color: Option<u32>) -> Self {
        let mut res = *self;
        res.prim_color = prim_color;
        res
    }

    pub fn with_env_color(&self, env_color: Option<u32>) -> Self {
        let mut res = *self;
        res.env_color = env_color;
        res
    }

    pub fn with_blend_color(&self, blend_color: Option<u32>) -> Self {
        let mut res = *self;
        res.blend_color = blend_color;
        res
    }

    pub fn with_blend(&self, blend: bool) -> Self {
        let mut res = *self;
        res.blend = blend;
        res
    }

    pub fn with_z_update(&self, z_update: bool) -> Self {
        let mut res = *self;
        res.z_update = z_update;
        res
    }

    pub fn with_z_compare(&self, z_compare: bool) -> Self {
        let mut res = *self;
        res.z_compare = z_compare;
        res
    }
}

impl Default for Pipeline {
    fn default() -> Self {
        Self::default()
    }
}
