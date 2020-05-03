use crate::components::{movable, systems};
use crate::impl_system;
use n64::{VideoMode, gfx::CommandBuffer};
use n64_math::{Color, Vec2};

#[derive(Copy, Clone)]
pub struct BoxDrawableComponent {
    pub size: Vec2,
    pub color: Color,
}

pub fn draw(cb: &mut CommandBuffer, video_mode: VideoMode) {
    for (component, entity) in lock().components_and_entities() {
        if let Some(movable) = movable::lock().lookup(&entity) {
            let half_size = component.size / 2.0;

            let upper_left = movable.pos - half_size;
            let lower_right = movable.pos + half_size;

            let screen_size = Vec2::new(video_mode.width() as f32, video_mode.height() as f32);

            cb.add_colored_rect(
                upper_left * screen_size,
                lower_right * screen_size,
                component.color,
            );
        }
    }
}

impl_system!(BoxDrawableComponent);
