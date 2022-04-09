use super::{movable::Movable, size::Size};
use crate::{
    camera::Camera,
    ecs::world::World,
    model::{ModelData, StaticModelData},
};
use n64::{gfx::CommandBuffer, VideoMode};
use n64_math::Vec2;

pub struct MeshDrawable {
    pub model: ModelData<'static>,
}

pub fn draw(world: &mut World, cb: &mut CommandBuffer, video_mode: VideoMode, camera: &Camera) {
    let (mesh_drawable, movable, size) = world.components.get3::<MeshDrawable, Movable, Size>();

    for (component, entity) in mesh_drawable.components_and_entities() {
        if let (Some(movable), Some(size)) = (movable.lookup(entity), size.lookup(entity)) {
            //let half_size = component.size / 2.0;

            //let upper_left = movable.pos - half_size;
            //let lower_right = movable.pos + half_size;

            //let screen_size = Vec2::new(video_mode.width() as f32, video_mode.height() as f32);

            cb.add_mesh_indexed(
                &component.model.verts,
                &component.model.uvs,
                &component.model.colors,
                &component.model.indices,
                &[
                    [1.0, 0.0, 0.0, 0.0],
                    [0.0, 1.0, 0.0, 0.0],
                    [0.0, 0.0, 0.0, 0.0],
                    [160.0, 120.0, 0.0, 1.0],
                ],
                None,
            );
        }
    }
}
