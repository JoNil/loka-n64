use super::{health::Health, movable::Movable, size::Size};
use crate::{
    camera::Camera,
    ecs::{query::query, world::World},
};
use game_derive::Component;
use n64::{
    gfx::{
        color_combiner_mode::{
            AAlphaSrc, ASrc, BAlphaSrc, BSrc, CAlphaSrc, CSrc, ColorCombinerMode, DAlphaSrc, DSrc,
        },
        CommandBuffer, Pipeline, Texture,
    },
    VideoMode,
};
use n64_math::Vec2;

static SPRITE_PIPELINE: Pipeline = Pipeline {
    color_combiner_mode: ColorCombinerMode::single(DSrc::Texel),
    blend: true,
    ..Pipeline::default()
};

#[derive(Copy, Clone, Component)]
pub struct SpriteDrawable {
    pub texture: Texture<'static>,
}

pub fn draw(world: &mut World, cb: &mut CommandBuffer, video_mode: VideoMode, camera: &Camera) {
    n64::scope!("sprite_drawable::draw");

    for (_e, sprite_drawable, movable, size, health) in
        query::<(SpriteDrawable, Movable, Size, Option<Health>)>(&mut world.components)
    {
        let half_size = size.size / 2.0;

        let upper_left = movable.pos - half_size;
        let lower_right = movable.pos + half_size;

        let screen_size = Vec2::new(video_mode.width() as f32, video_mode.height() as f32);

        let mut pipeline = SPRITE_PIPELINE.with_texture(Some(sprite_drawable.texture));

        if let Some(health) = health {
            if health.damaged_this_frame {
                pipeline.color_combiner_mode = ColorCombinerMode::one(
                    ASrc::One,
                    BSrc::Zero,
                    CSrc::Texel,
                    DSrc::Primitive,
                    AAlphaSrc::Zero,
                    BAlphaSrc::Zero,
                    CAlphaSrc::Zero,
                    DAlphaSrc::TexelAlpha,
                );
                pipeline.prim_color = Some(0xa0a0a0ff);
            }
        }

        cb.set_pipeline(&pipeline);

        cb.add_textured_rect(
            (upper_left - camera.pos) * screen_size,
            (lower_right - camera.pos) * screen_size,
        );
    }
}
