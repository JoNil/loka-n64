use super::{
    box_drawable::BoxDrawable,
    health::Health,
    mesh_drawable::MeshDrawable,
    movable::Movable,
    size::Size,
    weapon::{self, Weapon, WeaponTarget, WeaponType},
};
use crate::{
    camera::Camera,
    ecs::{
        entity::{Entity, EntitySystem},
        storage::Storage,
        world::World,
    },
    font::{draw_text, text_width},
    models::SHIP_3_BODY,
    sound_mixer::SoundMixer,
};
use n64::{gfx::CommandBuffer, Controllers, VideoMode};
use n64_math::{const_vec2, vec2, Color, Quat, Vec2, Vec3};
use std::f32::consts::PI;

const PLAYTER_START_POS: Vec2 = const_vec2!([0.5, 0.8]);
const SHIP_SPEED: f32 = 0.35;

pub struct Player {
    pub score: i32,
}

pub fn spawn_player(entities: &mut EntitySystem, start_pos: Vec2) -> Entity {
    entities
        .spawn()
        .add(Movable {
            pos: start_pos + PLAYTER_START_POS,
            speed: Vec2::new(0.0, 0.0),
        })
        .add(Size {
            size: SHIP_3_BODY.size,
        })
        .add(MeshDrawable {
            model: SHIP_3_BODY.as_model_data(),
            rot: Quat::IDENTITY,
        })
        .add(BoxDrawable {
            color: Color::from_rgb(0.1, 0.1, 0.8),
        })
        .add(Health { health: 10000 })
        .add(Weapon {
            weapon_type: WeaponType::Laser,
            last_shoot_time: 0,
            direction: 0.0,
        })
        .add(Player { score: 0 })
        .entity()
}

pub fn add_score(player: &mut Storage<Player>, score: i32) {
    for mut player in player.components_mut() {
        player.score += score;
    }
}

pub fn draw_player_weapon(world: &mut World, cb: &mut CommandBuffer, video_mode: &VideoMode) {
    let (player, weapon) = world.components.get2::<Player, Weapon>();

    for entity in player.entities() {
        if let Some(weapon) = weapon.lookup(*entity) {
            let text = (&weapon.weapon_type).into();
            let pos = vec2(
                video_mode.width() as f32 * 0.5 - text_width(text) as f32 * 0.5,
                video_mode.height() as f32 * 0.9,
            );

            draw_text(cb, text, pos, 0x80808080);
        }
    }
}

pub fn update(
    world: &mut World,
    controllers: &Controllers,
    sound_mixer: &mut SoundMixer,
    camera: &Camera,
) {
    let (player, movable, size, mesh_drawable, weapon) = world
        .components
        .get5::<Player, Movable, Size, MeshDrawable, Weapon>();

    for entity in player.entities() {
        let controller_x = controllers.x();
        let controller_y = controllers.y();

        let mut controller_dir = Vec2::new(0.0, 0.0);

        if let Some(mesh) = mesh_drawable.lookup_mut(*entity) {
            if controller_x.abs() > 32 {
                controller_dir.x = if controller_x > 0 {
                    mesh.rot = Quat::from_axis_angle(Vec3::Y, PI / 4.0);
                    1.0
                } else {
                    mesh.rot = Quat::from_axis_angle(Vec3::Y, -PI / 4.0);
                    -1.0
                };
            } else {
                mesh.rot = Quat::IDENTITY;
            }
        }

        if controller_y.abs() > 32 {
            controller_dir.y = if controller_y > 0 { -1.0 } else { 1.0 };
        }

        if let Some(m) = movable.lookup_mut(*entity) {
            m.speed = SHIP_SPEED * controller_dir - camera.speed;
        }

        if controllers.z() {
            weapon::fire(
                &mut world.entities,
                *entity,
                sound_mixer,
                movable,
                size,
                weapon,
                WeaponTarget::Enemy,
            );
        }
    }
}
