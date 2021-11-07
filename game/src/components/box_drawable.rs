use crate::components::movable;
use crate::{camera::Camera, impl_system};
use n64::{gfx::CommandBuffer, VideoMode};
use n64_math::{Color, Vec2};

#[derive(Copy, Clone)]
pub struct BoxDrawableComponent {
    pub size: Vec2,
    pub color: Color,
}

impl System {
    pub fn draw(
        &mut self,
        movalbe: &movable::System,
        cb: &mut CommandBuffer,
        video_mode: VideoMode,
        camera: &Camera,
    ) {
        for (component, entity) in self.components_and_entities() {
            if let Some(movable) = movalbe.lookup(entity) {
                let half_size = component.size / 2.0;

                let upper_left = movable.pos - half_size;
                let lower_right = movable.pos + half_size;

                let screen_size = Vec2::new(video_mode.width() as f32, video_mode.height() as f32);

                cb.add_colored_rect(
                    (upper_left - camera.pos) * screen_size,
                    (lower_right - camera.pos) * screen_size,
                    component.color,
                );
            }
        }
    }
}

impl_system!(BoxDrawableComponent);
