use crate::components::{movable, systems};
use crate::impl_system;
use n64::gfx::{CommandBuffer, Texture};
use n64_math::Vec2;

#[derive(Copy, Clone)]
pub struct SpriteDrawableComponent {
    pub size: Vec2,
    pub texture: Texture<'static>,
}

pub fn draw(cb: &mut CommandBuffer) {
    for (component, entity) in lock().components_and_entities() {
        if let Some(movable) = movable::lock().lookup(&entity) {
            let half_size = component.size / 2.0;

            let upper_left = movable.pos - half_size;
            let lower_right = movable.pos + half_size;

            let screen_size = Vec2::new(320.0, 240.0);

            cb.add_textured_rect(
                upper_left * screen_size,
                lower_right * screen_size,
                component.texture,
            );
        }
    }
}

impl_system!(SpriteDrawableComponent);
