use alloc::vec::Vec;
use n64::current_time_us;
use n64_math::{const_vec2, vec2, Color, Mat2, Quat, Vec2};
use strum_macros::{EnumCount, EnumIter, IntoStaticStr};

use super::{
    box_drawable::BoxDrawable, enemy::Enemy, health::Health, mesh_drawable::MeshDrawable,
    missile::Missile, movable::Movable, player::Player, projectile::Projectile, size::Size,
};
use crate::{
    ecs::{
        entity::{Entity, EntitySystem},
        storage::Storage,
    },
    models::LASER_BODY,
    sound_mixer::SoundMixer,
    sounds::{LASER_1, SHOOT_1, SHOOT_2},
};

#[derive(EnumCount, EnumIter, IntoStaticStr)]
pub enum WeaponType {
    Bullet,
    Missile,
    Laser,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum WeaponTarget {
    Player,
    Enemy,
}

pub struct Weapon {
    pub weapon_type: WeaponType,
    pub last_shoot_time: i64,
    pub direction: f32,
}

const BULLET_DELAY_MS: i32 = 300;
const MISSILE_DELAY_MS: i32 = 2000;
const BULLET_SIZE: Vec2 = const_vec2!([0.02, 0.02]);
const MISSILE_SIZE: Vec2 = const_vec2!([4.0 * 0.00825, 4.0 * 0.00825]);

pub fn shoot_bullet(
    entities: &mut EntitySystem,
    pos: Vec2,
    offset: Vec2,
    speed: Vec2,
    speed_offset: Vec2,
    direction: f32,
    target_type: WeaponTarget,
) {
    let spread = (n64_math::random_f32() - 0.5) * 0.05;

    let rot = Mat2::from_angle(direction);

    let offset = rot.mul_vec2(offset);
    let speed_offset = rot.mul_vec2(vec2(speed_offset.x + spread, speed_offset.y));

    entities
        .spawn()
        .add(Movable {
            pos: pos + offset,
            speed: speed + speed_offset,
        })
        .add(Size { size: BULLET_SIZE })
        .add(Health { health: 5 })
        .add(BoxDrawable {
            color: Color::from_rgb(0.9, 0.2, 0.7),
        })
        .add(Projectile {
            target_type,
            damage: 50 + (n64_math::random_f32() * 20.0) as i32,
            projectile_collision_grace_period_ms: 0,
        });
}

pub fn shoot_missile(
    entities: &mut EntitySystem,
    pos: Vec2,
    offset: Vec2,
    speed: Vec2,
    speed_offset: Vec2,
    direction: f32,
    target: Option<Entity>,
    target_type: WeaponTarget,
) {
    let spread = (n64_math::random_f32() - 0.5) * 0.05;

    let rot = Mat2::from_angle(direction);

    let offset = rot.mul_vec2(offset);
    let speed_offset =
        Mat2::from_angle(direction).mul_vec2(vec2(speed_offset.x + spread, speed_offset.y));

    entities
        .spawn()
        .add(Movable {
            pos: pos + offset,
            speed: speed + speed_offset,
        })
        .add(Size { size: MISSILE_SIZE })
        .add(Health { health: 15 })
        .add(BoxDrawable {
            color: Color::from_rgb(1.0, 0.4, 0.4),
        })
        .add(Projectile {
            target_type,
            damage: 100 + (n64_math::random_f32() * 50.0) as i32,
            projectile_collision_grace_period_ms: 1000,
        })
        .add(Missile { target });
}

pub fn shoot_laser(
    entities: &mut EntitySystem,
    pos: Vec2,
    speed: Vec2,
    direction: f32,
    target_type: WeaponTarget,
) {
    let extent = Mat2::from_angle(direction).mul_vec2(vec2(0.0, -LASER_BODY.size.y / 2.0));

    entities
        .spawn()
        .add(Movable {
            pos: pos + extent,
            speed,
        })
        .add(Size {
            size: LASER_BODY.size,
        })
        .add(MeshDrawable {
            model: LASER_BODY.as_model_data(),
            rot: Quat::IDENTITY,
        })
        .add(Projectile {
            target_type,
            damage: 1,
            projectile_collision_grace_period_ms: 0,
        });
}

pub fn fire(
    entities: &mut EntitySystem,
    entity: Entity,
    sound_mixer: &mut SoundMixer,
    weapon: &mut Storage<Weapon>,
    movable: &Storage<Movable>,
    size: &Storage<Size>,
    enemy: &Storage<Enemy>,
    player: &Storage<Player>,
    target_type: WeaponTarget,
) {
    let now = current_time_us();

    if let (Some(m), Some(s), Some(w)) = (
        movable.lookup(entity),
        size.lookup(entity),
        weapon.lookup_mut(entity),
    ) {
        match w.weapon_type {
            WeaponType::Bullet => {
                if now - w.last_shoot_time > BULLET_DELAY_MS as i64 * 1000 {
                    //sound_mixer.play_sound(SHOOT_1.as_sound_data());
                    shoot_bullet(
                        entities,
                        m.pos,
                        vec2(0.0, -s.size.y / 2.0),
                        m.speed,
                        vec2(0.0, -1.25),
                        w.direction,
                        target_type,
                    );
                    w.last_shoot_time = now;
                }
            }
            WeaponType::Missile => {
                if now - w.last_shoot_time > MISSILE_DELAY_MS as i64 * 1000 {
                    sound_mixer.play_sound(SHOOT_2.as_sound_data());

                    let shooter_pos = m.pos;

                    let mut distances = enemy
                        .entities()
                        .iter()
                        .chain(player.entities())
                        .filter_map(|e| movable.lookup(*e).map(|m| (m, *e)))
                        .filter_map(|(m, e)| {
                            if e != entity {
                                Some(((shooter_pos - m.pos).length(), e))
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>();

                    distances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

                    let target_1 = distances.get(0).map(|(_, e)| *e);
                    let target_2 = distances.get(1).map(|(_, e)| *e);
                    let target_3 = distances.get(2).map(|(_, e)| *e);

                    let offset = vec2(0.0, -s.size.y / 2.0);

                    shoot_missile(
                        entities,
                        m.pos,
                        offset,
                        m.speed,
                        vec2(0.0, -0.5),
                        w.direction,
                        target_1,
                        target_type,
                    );
                    shoot_missile(
                        entities,
                        m.pos,
                        offset,
                        m.speed,
                        vec2(0.15, -0.5),
                        w.direction,
                        target_2,
                        target_type,
                    );
                    shoot_missile(
                        entities,
                        m.pos,
                        offset,
                        m.speed,
                        vec2(-0.15, -0.5),
                        w.direction,
                        target_3,
                        target_type,
                    );
                    w.last_shoot_time = now;
                }
            }
            WeaponType::Laser => {
                sound_mixer.play_sound(LASER_1.as_sound_data());
                shoot_laser(entities, m.pos, m.speed, w.direction, target_type);
            }
        }
    }
}
