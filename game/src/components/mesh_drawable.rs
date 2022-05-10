use super::{health::Health, movable::Movable};
use crate::{camera::Camera, ecs::world::World, model::ModelData};
use core::f32::consts::PI;
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
    n64_profiler::scope!("mesh_drawable::draw");

    let (mesh_drawable, movable, health) = world.components.get3::<MeshDrawable, Movable, Health>();

    let proj = Mat4::perspective_rh_gl(PI / 2.0, 1.0, 0.01, 1000.0);

    for (component, entity) in mesh_drawable.components_and_entities() {
        if let Some(movable) = movable.lookup(entity) {
            let mut pipeline = MESH_PIPELINE;

            if let Some(health) = health.lookup(entity) {
                if health.damaged_this_frame {
                    pipeline.color_combiner_mode = ColorCombinerMode::one(
                        ASrc::One,
                        BSrc::Zero,
                        CSrc::Texel,
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

            let post_transform = Mat4::from_cols_array_2d(&[
                [video_mode.width() as f32, 0.0, 0.0, 0.0],
                [0.0, video_mode.height() as f32, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ]);

            let transform = post_transform
                * proj
                * Mat4::from_rotation_translation(
                    component.rot,
                    vec3(
                        movable.pos.x - camera.pos.x,
                        movable.pos.y - camera.pos.y,
                        -1.0,
                    ),
                );

            cb.add_mesh_indexed(
                &component.model.verts,
                &component.model.uvs,
                &component.model.colors,
                &component.model.indices,
                &transform.to_cols_array_2d(),
            );
        }
    }
}
