use super::{movable::Movable, size::Size};
use crate::{camera::Camera, ecs::world::World};
use n64::{
    gfx::{CommandBuffer, FillPipeline},
    VideoMode,
};
use n64_math::{Color, Vec2};

static BOX_PIPELINE: FillPipeline = FillPipeline::default();

pub struct BoxDrawable {
    pub color: Color,
}

pub fn draw(world: &mut World, cb: &mut CommandBuffer, video_mode: VideoMode, camera: &Camera) {
    let (box_drawable, movable, size) = world.components.get::<(BoxDrawable, Movable, Size)>();

    for (component, entity) in box_drawable.components_and_entities() {
        if let (Some(movable), Some(size)) = (movable.lookup(entity), size.lookup(entity)) {
            let half_size = size.size / 2.0;

            let upper_left = movable.pos - half_size;
            let lower_right = movable.pos + half_size;

            let screen_size = Vec2::new(video_mode.width() as f32, video_mode.height() as f32);

            cb.set_fill_pipeline(&BOX_PIPELINE.with_fill_color(component.color));

            cb.add_colored_rect(
                (upper_left - camera.pos) * screen_size,
                (lower_right - camera.pos) * screen_size,
            );
        }
    }
}
