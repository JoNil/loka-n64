use n64_math::Color;
use crate::entity::Entity;
use crate::impl_system;
use n64::{graphics, ipl3font};
use crate::components::{systems, movable};

#[derive(Copy, Clone)]
pub struct CharDrawableComponent {
    pub entity: Entity,
    pub color: Color,
    pub chr: char,
}

pub fn draw() {
    for component in lock().components() {
        if let Some(movable) = movable::lock().lookup(&component.entity) {
            let screen_x = (movable.pos.x() * (graphics::WIDTH as f32)) as i32 - ipl3font::GLYPH_WIDTH / 2;
            let screen_y =
                (movable.pos.y() * (graphics::HEIGHT as f32)) as i32 + ipl3font::GLYPH_HEIGHT / 2;

            ipl3font::draw_char(screen_x, screen_y, component.color, component.chr as u8);
        }
    }
}

impl_system!(CharDrawableComponent);