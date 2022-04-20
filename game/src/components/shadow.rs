use super::{mesh_drawable::MeshDrawable, movable::Movable};
use crate::{camera::Camera, ecs::world::World};
use n64::{
    gfx::{
        color_combiner::{ASrc, BSrc, CSrc, ColorCombiner, DSrc},
        CommandBuffer, Pipeline,
    },
    VideoMode,
};
use n64_math::{vec3, Mat4};
use std::f32::consts::PI;

pub struct Shadow;

static SHADOW_PIPELINE: Pipeline = Pipeline {
    combiner_mode: ColorCombiner::one_cycle_symertical(
        ASrc::Zero,
        BSrc::Zero,
        CSrc::Zero,
        DSrc::Primitive,
    ),
    prim_color: Some(0x10101080),
    z_update: false,
    z_compare: true,
    ..Pipeline::default()
};

pub fn draw(world: &mut World, cb: &mut CommandBuffer, video_mode: VideoMode, camera: &Camera) {
    let (mesh_drawable, shadow, movable) = world.components.get3::<MeshDrawable, Shadow, Movable>();

    let proj = Mat4::perspective_rh_gl(PI / 2.0, 1.0, 0.01, 1000.0);

    cb.set_pipeline(&SHADOW_PIPELINE);

    for entity in shadow.entities() {
        if let (Some(mesh_drawable), Some(movable)) =
            (mesh_drawable.lookup(*entity), movable.lookup(*entity))
        {
            let post_transform = Mat4::from_cols_array_2d(&[
                [video_mode.width() as f32, 0.0, 0.0, 0.0],
                [0.0, video_mode.height() as f32, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ]);

            let transform = post_transform
                * proj
                * Mat4::from_rotation_translation(
                    mesh_drawable.rot,
                    vec3(
                        movable.pos.x - camera.pos.x - 0.05,
                        movable.pos.y - camera.pos.y + 0.1,
                        -1.2,
                    ),
                );

            cb.add_mesh_indexed(
                &mesh_drawable.model.verts,
                &mesh_drawable.model.uvs,
                &mesh_drawable.model.colors,
                &mesh_drawable.model.indices,
                &transform.to_cols_array_2d(),
            );
        }
    }
}
