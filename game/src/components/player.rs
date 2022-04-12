use std::f32::consts::PI;

use super::{
    box_drawable::BoxDrawable,
    bullet::shoot_bullet,
    enemy::Enemy,
    health::Health,
    laser::shoot_laser,
    mesh_drawable::MeshDrawable,
    missile::shoot_missile,
    movable::{self, Movable},
    size::Size,
};
use crate::{
    camera::Camera,
    ecs::{
        entity::{Entity, EntitySystem},
        storage::Storage,
        world::World,
    },
    models::SHIP_3_BODY,
    sound_mixer::SoundMixer,
    sounds::{LASER_1, SHOOT_1, SHOOT_2},
    weapon::Weapon,
};
use alloc::vec::Vec;
use n64::{current_time_us, Controllers, VideoMode};
use n64_math::{const_vec2, vec2, Color, Quat, Vec2, Vec3};

const PLAYTER_START_POS: Vec2 = const_vec2!([0.5, 0.8]);
const SHIP_SPEED: f32 = 0.35;
const SHIP_SHOOT_DELAY_MS: i32 = 150;
const SHIP_SHOOT_MISSILE_DELAY_MS: i32 = 1000;
pub const SHIP_SIZE_PX: Vec2 = const_vec2!([32.0, 32.0]);

pub struct Player {
    pub score: i32,
    pub last_shoot_time: i64,
    pub weapon: Weapon,
}

pub fn spawn_player(
    entities: &mut EntitySystem,
    start_pos: Vec2,
    video_mode: &VideoMode,
) -> Entity {
    entities
        .spawn()
        .add(Movable {
            pos: start_pos + PLAYTER_START_POS,
            speed: Vec2::new(0.0, 0.0),
        })
        .add(Size {
            size: vec2(
                SHIP_SIZE_PX.x / video_mode.width() as f32,
                SHIP_SIZE_PX.y / video_mode.height() as f32,
            ),
        })
        .add(MeshDrawable {
            model: SHIP_3_BODY.as_model_data(),
            rot: Quat::IDENTITY,
        })
        .add(BoxDrawable {
            color: Color::from_rgb(0.1, 0.1, 0.8),
        })
        .add(Health { health: 10000 })
        .add(Player {
            score: 0,
            last_shoot_time: 0,
            weapon: Weapon::Laser,
        })
        .entity()
}

pub fn add_score(player: &mut Storage<Player>, score: i32) {
    for mut player in player.components_mut() {
        player.score += score;
    }
}

pub fn update(
    world: &mut World,
    controllers: &Controllers,
    sound_mixer: &mut SoundMixer,
    camera: &Camera,
) {
    let (player, movable, enemy, size, mesh_drawable) = world
        .components
        .get5::<Player, Movable, Enemy, Size, MeshDrawable>();

    for (player, entity) in player.components_and_entities_mut() {
        let controller_x = controllers.x();
        let controller_y = controllers.y();

        let mut controller_dir = Vec2::new(0.0, 0.0);

        if let Some(mesh) = mesh_drawable.lookup_mut(entity) {
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

        if let Some(m) = movable.lookup_mut(entity) {
            m.speed = SHIP_SPEED * controller_dir - camera.speed;
        }

        if let (Some(m), Some(s)) = (movable.lookup(entity).cloned(), size.lookup(entity)) {
            let now = current_time_us();

            if controllers.z() {
                match player.weapon {
                    Weapon::Bullet => {
                        if now - player.last_shoot_time > SHIP_SHOOT_DELAY_MS as i64 * 1000 {
                            sound_mixer.play_sound(SHOOT_1.as_sound_data());
                            shoot_bullet(
                                &mut world.entities,
                                m.pos + Vec2::new(0.0, -s.size.y / 2.0),
                                Vec2::new(0.0, m.speed.y - 1.25),
                            );
                            player.last_shoot_time = now;
                        }
                    }
                    Weapon::Missile => {
                        if now - player.last_shoot_time > SHIP_SHOOT_MISSILE_DELAY_MS as i64 * 1000
                        {
                            sound_mixer.play_sound(SHOOT_2.as_sound_data());

                            let player_pos = m.pos;

                            let mut distances = enemy
                                .entities()
                                .iter()
                                .filter_map(|e| movable::pos(movable, *e).map(|pos| (pos, e)))
                                .map(|(pos, e)| ((player_pos - pos).length(), *e))
                                .collect::<Vec<_>>();

                            distances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

                            let target_1 = distances.get(0).map(|(_, e)| *e);
                            let target_2 = distances.get(1).map(|(_, e)| *e);
                            let target_3 = distances.get(2).map(|(_, e)| *e);

                            shoot_missile(
                                &mut world.entities,
                                m.pos + Vec2::new(0.0, -s.size.y / 2.0),
                                Vec2::new(0.0, m.speed.y - 0.5),
                                target_1,
                            );
                            shoot_missile(
                                &mut world.entities,
                                m.pos + Vec2::new(0.0, -s.size.y / 2.0),
                                Vec2::new(0.15, m.speed.y - 0.5),
                                target_2,
                            );
                            shoot_missile(
                                &mut world.entities,
                                m.pos + Vec2::new(0.0, -s.size.y / 2.0),
                                Vec2::new(-0.15, m.speed.y - 0.5),
                                target_3,
                            );
                            player.last_shoot_time = now;
                        }
                    }
                    Weapon::Laser => {
                        sound_mixer.play_sound(LASER_1.as_sound_data());
                        shoot_laser(
                            &mut world.entities,
                            m.pos + Vec2::new(0.0, -s.size.y / 2.0 * 0.5),
                            m.speed,
                        );
                    }
                }
            }
        }
    }
}
