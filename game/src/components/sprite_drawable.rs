use crate::components::{movable, systems};
use crate::entity::Entity;
use crate::impl_system;
use n64::{gfx::Texture, graphics};
use n64_math::Vec2;

#[derive(Copy, Clone)]
pub struct SpriteDrawableComponent {
    pub texture: Texture,
}

pub fn draw() {
    for (component, entity) in lock().components_and_entities() {
        if let Some(movable) = movable::lock().lookup(&entity) {
            let screen_x = movable.pos.x() * (graphics::WIDTH as f32);
            let screen_y = movable.pos.y() * (graphics::HEIGHT as f32);

            //graphics::draw_sprite(&component.texture, Vec2::new(screen_x, screen_y));
        }
    }
}

impl_system!(SpriteDrawableComponent);
