use super::{mesh_drawable::MeshDrawable, movable::Movable};
use crate::{camera::Camera, ecs::world::World};
use n64::{
    gfx::{
        color_combiner::{
            AAlphaSrc, ASrc, BAlphaSrc, BSrc, CAlphaSrc, CSrc, ColorCombiner, DAlphaSrc, DSrc,
        },
        CommandBuffer, CycleType, Pipeline,
    },
    VideoMode,
};
use n64_math::{vec3, Mat4};
use std::f32::consts::PI;

pub struct Shadow;

static SHADOW_PIPELINE: Pipeline = Pipeline {
    cycle_type: CycleType::One,
    combiner_mode: ColorCombiner {
        a_0: ASrc::Zero,
        b_0: BSrc::Zero,
        c_0: CSrc::Zero,
        d_0: DSrc::Primitive,

        a_alpha_0: AAlphaSrc::Zero,
        b_alpha_0: BAlphaSrc::Zero,
        c_alpha_0: CAlphaSrc::Zero,
        d_alpha_0: DAlphaSrc::PrimitiveAlpha,

        a_1: ASrc::Zero,
        b_1: BSrc::Zero,
        c_1: CSrc::Zero,
        d_1: DSrc::Primitive,

        a_alpha_1: AAlphaSrc::Zero,
        b_alpha_1: BAlphaSrc::Zero,
        c_alpha_1: CAlphaSrc::Zero,
        d_alpha_1: DAlphaSrc::PrimitiveAlpha,
    },
    texture: None,
    prim_color: Some(0x10101080),
    env_color: None,
    blend_color: None,
    z_update: false,
    z_compare: true,
};

pub fn draw(world: &mut World, cb: &mut CommandBuffer, video_mode: VideoMode, camera: &Camera) {
    let (mesh_drawable, shadow, movable) = world.components.get3::<MeshDrawable, Shadow, Movable>();

    let proj = Mat4::perspective_rh_gl(PI / 2.0, 1.0, 0.01, 1000.0);

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
                &SHADOW_PIPELINE,
            );
        }
    }
}
