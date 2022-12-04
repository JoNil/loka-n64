use super::{health::Health, movable::Movable};
use crate::{
    camera::Camera,
    ecs::{query::query, world::World},
    model::ModelData,
};
use core::f32::consts::PI;
use game_derive::SparseComponent;
use n64::{
    gfx::{
        color_combiner_mode::{
            AAlphaSrc, ASrc, BAlphaSrc, BSrc, CAlphaSrc, CSrc, ColorCombinerMode, DAlphaSrc, DSrc,
        },
        CommandBuffer, Pipeline,
    },
    VideoMode,
};
use n64_math::{vec3, Mat4, Quat};

#[derive(SparseComponent)]
pub struct MeshDrawable {
    pub model: ModelData<'static>,
    pub rot: Quat,
}

static MESH_PIPELINE: Pipeline = Pipeline {
    color_combiner_mode: ColorCombinerMode::simple(ASrc::Zero, BSrc::Zero, CSrc::Zero, DSrc::Shade),
    z_compare: true,
    z_update: true,
    ..Pipeline::default()
};

pub fn draw(world: &mut World, cb: &mut CommandBuffer, video_mode: VideoMode, camera: &Camera) {
    n64::scope!("mesh_drawable::draw");

    let half_width = 0.5 * video_mode.width() as f32;
    let half_height = 0.5 * video_mode.height() as f32;

    let post_transform = Mat4::from_cols_array_2d(&[
        [half_width, 0.0, 0.0, 0.0],
        [0.0, half_height, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [half_width, half_height, 0.0, 1.0],
    ]);

    let proj = Mat4::perspective_rh_gl(PI / 2.0, 1.0, 0.1, 10.0);

    let pre_transform = Mat4::from_cols_array_2d(&[
        [2.0, 0.0, 0.0, 0.0],
        [0.0, 2.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [-1.0, -1.0, 0.0, 1.0],
    ]);

    let proj = post_transform * proj * pre_transform;

    for (_e, mesh_drawable, movable, health) in
        query::<(MeshDrawable, Movable, Option<Health>)>(&mut world.components)
    {
        let mut pipeline = MESH_PIPELINE;

        if let Some(health) = health {
            if health.damaged_this_frame {
                pipeline.color_combiner_mode = ColorCombinerMode::one(
                    ASrc::One,
                    BSrc::Zero,
                    CSrc::Shade,
                    DSrc::Primitive,
                    AAlphaSrc::Zero,
                    BAlphaSrc::Zero,
                    CAlphaSrc::Zero,
                    DAlphaSrc::TexelAlpha,
                );
                pipeline.prim_color = Some(0xa0a0a0ff);
            }
        }

        cb.set_pipeline(&pipeline);

        let transform = proj
            * Mat4::from_rotation_translation(
                mesh_drawable.rot,
                vec3(
                    movable.pos.x - camera.pos.x,
                    movable.pos.y - camera.pos.y,
                    -1.0,
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
