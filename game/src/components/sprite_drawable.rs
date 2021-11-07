use super::movable::Movable;
use crate::{camera::Camera, world::World};
use n64::{
    gfx::{CommandBuffer, Texture},
    VideoMode,
};
use n64_math::Vec2;

#[derive(Copy, Clone)]
pub struct SpriteDrawable {
    pub size: Vec2,
    pub texture: Texture<'static>,
}

pub fn draw(world: &mut World, cb: &mut CommandBuffer, video_mode: VideoMode, camera: &Camera) {
    let sprite_drawable = world.get::<SpriteDrawable>();
    let sprite_drawable = sprite_drawable.borrow();
    let movable = world.get::<Movable>();
    let movable = movable.borrow();

    for (component, entity) in sprite_drawable.components_and_entities() {
        if let Some(movable) = movable.lookup(entity) {
            let half_size = component.size / 2.0;

            let upper_left = movable.pos - half_size;
            let lower_right = movable.pos + half_size;

            let screen_size = Vec2::new(video_mode.width() as f32, video_mode.height() as f32);

            cb.add_textured_rect(
                (upper_left - camera.pos) * screen_size,
                (lower_right - camera.pos) * screen_size,
                component.texture,
                None,
            );
        }
    }
}
