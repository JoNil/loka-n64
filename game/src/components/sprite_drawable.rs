use crate::components::{movable, systems};
use crate::entity::Entity;
use crate::impl_system;
use n64::{gfx::Texture, graphics};

#[derive(Copy, Clone)]
pub struct SpriteDrawableComponent {
    pub texture: Texture,
}

pub fn draw() {
    for (_component, entity) in lock().components_and_entities() {
        if let Some(movable) = movable::lock().lookup(&entity) {
            let _screen_x = movable.pos.x() * (graphics::WIDTH as f32);
            let _screen_y = movable.pos.y() * (graphics::HEIGHT as f32);

            //graphics::draw_sprite(&component.texture, Vec2::new(screen_x, screen_y));
        }
    }
}

impl_system!(SpriteDrawableComponent);
