use super::{movable::Movable, size::Size};
use crate::{camera::Camera, ecs::world::World};
use n64::{
    gfx::{
        color_combiner::{
            AAlphaSrc, ASrc, BAlphaSrc, BSrc, CAlphaSrc, CSrc, ColorCombiner, DAlphaSrc, DSrc,
        },
        CommandBuffer, Texture,
    },
    VideoMode,
};
use n64_math::Vec2;

#[derive(Copy, Clone)]
pub struct SpriteDrawable {
    pub texture: Texture<'static>,
}

pub fn draw(world: &mut World, cb: &mut CommandBuffer, video_mode: VideoMode, camera: &Camera) {
    let (sprite_drawable, movable, size) = world.components.get3::<SpriteDrawable, Movable, Size>();

    cb.set_color_combiner_mode(ColorCombiner {
        a_0: ASrc::Zero,
        b_0: BSrc::Zero,
        c_0: CSrc::Zero,
        d_0: DSrc::Texel,
        a_alpha_0: AAlphaSrc::Zero,
        b_alpha_0: BAlphaSrc::Zero,
        c_alpha_0: CAlphaSrc::Zero,
        d_alpha_0: DAlphaSrc::TexelAlpha,

        a_1: ASrc::Zero,
        b_1: BSrc::Zero,
        c_1: CSrc::Zero,
        d_1: DSrc::Texel,
        a_alpha_1: AAlphaSrc::Zero,
        b_alpha_1: BAlphaSrc::Zero,
        c_alpha_1: CAlphaSrc::Zero,
        d_alpha_1: DAlphaSrc::TexelAlpha,
    });

    for (component, entity) in sprite_drawable.components_and_entities() {
        if let (Some(movable), Some(size)) = (movable.lookup(entity), size.lookup(entity)) {
            let half_size = size.size / 2.0;

            let upper_left = movable.pos - half_size;
            let lower_right = movable.pos + half_size;

            let screen_size = Vec2::new(video_mode.width() as f32, video_mode.height() as f32);

            cb.add_textured_rect(
                (upper_left - camera.pos) * screen_size,
                (lower_right - camera.pos) * screen_size,
                component.texture,
                None,
            );
        }
    }
}
