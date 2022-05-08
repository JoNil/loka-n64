use super::{movable::Movable, size::Size};
use crate::{camera::Camera, ecs::world::World};
use n64::{
    gfx::{
        color_combiner_mode::{ColorCombinerMode, DSrc},
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

#[derive(Copy, Clone)]
pub struct SpriteDrawable {
    pub texture: Texture<'static>,
}

pub fn draw(world: &mut World, cb: &mut CommandBuffer, video_mode: VideoMode, camera: &Camera) {
    let (sprite_drawable, movable, size) = world.components.get3::<SpriteDrawable, Movable, Size>();

    for (component, entity) in sprite_drawable.components_and_entities() {
        if let (Some(movable), Some(size)) = (movable.lookup(entity), size.lookup(entity)) {
            let half_size = size.size / 2.0;

            let upper_left = movable.pos - half_size;
            let lower_right = movable.pos + half_size;

            let screen_size = Vec2::new(video_mode.width() as f32, video_mode.height() as f32);

            cb.set_pipeline(&SPRITE_PIPELINE.with_texture(Some(component.texture)));

            cb.add_textured_rect(
                (upper_left - camera.pos) * screen_size,
                (lower_right - camera.pos) * screen_size,
            );
        }
    }
}
