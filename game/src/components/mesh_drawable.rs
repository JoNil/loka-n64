use super::{movable::Movable, size::Size};
use crate::{camera::Camera, ecs::world::World, model::ModelData};
use n64::{gfx::CommandBuffer, VideoMode};
use n64_math::{vec3, Mat4};

pub struct MeshDrawable {
    pub model: ModelData<'static>,
    pub model_matrix: Mat4,
}

pub fn draw(world: &mut World, cb: &mut CommandBuffer, video_mode: VideoMode, camera: &Camera) {
    let (mesh_drawable, movable, size) = world.components.get3::<MeshDrawable, Movable, Size>();

    for (component, entity) in mesh_drawable.components_and_entities() {
        if let (Some(movable), Some(size)) = (movable.lookup(entity), size.lookup(entity)) {
            let model = component.model_matrix;
            let scale = Mat4::from_scale(vec3(
                size.size.x * video_mode.width() as f32,
                size.size.y * video_mode.height() as f32,
                1.0,
            ));
            let pos = Mat4::from_translation(vec3(160.0, 120.0, 0.0));

            let transform = model * scale * pos;

            cb.add_mesh_indexed(
                &component.model.verts,
                &component.model.uvs,
                &component.model.colors,
                &component.model.indices,
                &transform.to_cols_array_2d(),
                None,
            );
        }
    }
}
