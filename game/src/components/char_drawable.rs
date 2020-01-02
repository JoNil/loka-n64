use crate::components::{movable, systems};
use crate::entity::Entity;
use crate::impl_system;
use n64::{graphics, ipl3font};
use n64_math::Color;

#[derive(Copy, Clone)]
pub struct CharDrawableComponent {
    pub color: Color,
    pub chr: u8,
}

pub fn draw() {
    for (component, entity) in lock().components_and_entities() {
        if let Some(movable) = movable::lock().lookup(&entity) {
            let screen_x =
                (movable.pos.x() * (graphics::WIDTH as f32)) as i32 - ipl3font::GLYPH_WIDTH / 2;
            let screen_y =
                (movable.pos.y() * (graphics::HEIGHT as f32)) as i32 + ipl3font::GLYPH_HEIGHT / 2;

            ipl3font::draw_char(screen_x, screen_y, component.color, component.chr);
        }
    }
}

impl_system!(CharDrawableComponent);
