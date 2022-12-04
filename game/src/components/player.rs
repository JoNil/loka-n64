use super::{
    enemy::Enemy,
    health::Health,
    keep_on_screen::KeepOnScreen,
    mesh_drawable::MeshDrawable,
    movable::Movable,
    shadow::Shadow,
    size::Size,
    weapon::{self, Weapon, WeaponTarget, WeaponType},
};
use crate::{
    camera::Camera,
    ecs::{
        component::Component,
        entity::{Entity, EntitySystem},
        query::query,
        storage::Storage,
        world::World,
    },
    font::{draw_text, text_width},
    models::SHIP_3,
    sound_mixer::SoundMixer,
};
use core::f32::consts::PI;
use game_derive::Component;
use n64::{gfx::CommandBuffer, Controllers, VideoMode};
use n64_math::{const_vec2, vec2, Quat, Vec2, Vec3};

const PLAYER_START_POS: Vec2 = const_vec2!([0.5, 0.8]);
const SHIP_SPEED: f32 = 0.35;

#[derive(Component)]
pub struct Player {
    pub score: i32,
}

pub fn spawn_player(entities: &mut EntitySystem, start_pos: Vec2) -> Entity {
    entities
        .spawn()
        .add(Movable {
            pos: start_pos + PLAYER_START_POS,
            speed: Vec2::new(0.0, 0.0),
        })
        .add(Size { size: SHIP_3.size })
        .add(MeshDrawable {
            model: SHIP_3.as_model_data(),
            rot: Quat::IDENTITY,
        })
        .add(Shadow)
        .add(Health {
            health: 10000,
            damaged_this_frame: true,
        })
        .add(Weapon {
            weapon_type: WeaponType::Flak,
            last_shoot_time: i64::MIN / 2,
            direction: 0.0,
        })
        .add(Player { score: 0 })
        .add(KeepOnScreen)
        .entity()
}

pub fn add_score(player: &mut <Player as Component>::Storage, score: i32) {
    for mut player in player.components_mut() {
        player.score += score;
    }
}

pub fn draw_player_weapon(world: &mut World, cb: &mut CommandBuffer, video_mode: &VideoMode) {
    for (_e, _player, weapon) in query::<(Player, Weapon)>(&mut world.components) {
        let text = (&weapon.weapon_type).into();
        let pos = vec2(
            video_mode.width() as f32 * 0.5 - text_width(text) as f32 * 0.5,
            video_mode.height() as f32 * 0.9,
        );

        draw_text(cb, text, pos, 0x202020a0);
    }
}

pub fn update(
    world: &mut World,
    controllers: &Controllers,
    sound_mixer: &mut SoundMixer,
    camera: &Camera,
) {
    let (player, movable, size, mesh_drawable, weapon, enemy) =
        world
            .components
            .get::<(Player, Movable, Size, MeshDrawable, Weapon, Enemy)>();

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
                weapon,
                movable,
                size,
                enemy,
                player,
                WeaponTarget::Enemy,
            );
        }
    }
}
