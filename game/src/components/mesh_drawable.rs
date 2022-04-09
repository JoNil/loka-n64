use super::{movable::Movable, size::Size};
use crate::{camera::Camera, ecs::world::World, model::ModelData};
use n64::{gfx::CommandBuffer, VideoMode};
use n64_math::{vec3, Mat4, Quat};

pub struct MeshDrawable {
    pub model: ModelData<'static>,
    pub rot: Quat,
    pub scale: f32,
}

pub fn draw(world: &mut World, cb: &mut CommandBuffer, video_mode: VideoMode, camera: &Camera) {
    let (mesh_drawable, movable, size) = world.components.get3::<MeshDrawable, Movable, Size>();

    for (component, entity) in mesh_drawable.components_and_entities() {
        if let (Some(movable), Some(size)) = (movable.lookup(entity), size.lookup(entity)) {
            let transform = Mat4::from_cols_array_2d(&[
                [
                    component.scale * size.size.x * video_mode.width() as f32,
                    0.0,
                    0.0,
                    0.0,
                ],
                [
                    0.0,
                    component.scale * size.size.y * video_mode.height() as f32,
                    0.0,
                    0.0,
                ],
                [0.0, 0.0, 1.0, 0.0],
                [
                    (movable.pos.x - camera.pos.x) * video_mode.width() as f32,
                    (movable.pos.y - camera.pos.y) * video_mode.height() as f32,
                    0.0,
                    1.0,
                ],
            ]);

            cb.add_mesh_indexed(
                &component.model.verts,
                &component.model.uvs,
                &component.model.colors,
                &component.model.indices,
                &(transform * Mat4::from_quat(component.rot)).to_cols_array_2d(),
                None,
            );
        }
    }
}
