use super::movable::Movable;
use crate::{camera::Camera, ecs::world::World, model::ModelData};
use core::f32::consts::PI;
use n64::{
    gfx::{
        color_combiner::{
            AAlphaSrc, ASrc, BAlphaSrc, BSrc, CAlphaSrc, CSrc, ColorCombiner, DAlphaSrc, DSrc,
        },
        CommandBuffer, CycleType, Pipeline,
    },
    VideoMode,
};
use n64_math::{vec3, Mat4, Quat};

pub struct MeshDrawable {
    pub model: ModelData<'static>,
    pub rot: Quat,
}

static MESH_PIPELINE: Pipeline = Pipeline {
    cycle_type: CycleType::One,
    combiner_mode: ColorCombiner {
        a_0: ASrc::Zero,
        b_0: BSrc::Zero,
        c_0: CSrc::Zero,
        d_0: DSrc::Shade,

        a_alpha_0: AAlphaSrc::Zero,
        b_alpha_0: BAlphaSrc::Zero,
        c_alpha_0: CAlphaSrc::Zero,
        d_alpha_0: DAlphaSrc::ShadeAlpha,

        a_1: ASrc::Zero,
        b_1: BSrc::Zero,
        c_1: CSrc::Zero,
        d_1: DSrc::Shade,

        a_alpha_1: AAlphaSrc::Zero,
        b_alpha_1: BAlphaSrc::Zero,
        c_alpha_1: CAlphaSrc::Zero,
        d_alpha_1: DAlphaSrc::ShadeAlpha,
    },
    prim_color: None,
    env_color: None,
    blend_color: None,
    texture: None,
    z_update: true,
    z_compare: true,
};

pub fn draw(world: &mut World, cb: &mut CommandBuffer, video_mode: VideoMode, camera: &Camera) {
    let (mesh_drawable, movable) = world.components.get2::<MeshDrawable, Movable>();

    let proj = Mat4::perspective_rh_gl(PI / 2.0, 1.0, 0.01, 1000.0);

    for (component, entity) in mesh_drawable.components_and_entities() {
        if let Some(movable) = movable.lookup(entity) {
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
                &MESH_PIPELINE,
            );
        }
    }
}
