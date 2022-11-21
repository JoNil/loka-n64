use alloc::vec::Vec;
use core::f32::consts::PI;
use n64::{
    current_time_us,
    gfx::{
        color_combiner_mode::{ASrc, BSrc, CSrc, ColorCombinerMode, DSrc},
        CommandBuffer, Pipeline,
    },
    VideoMode,
};
use n64_math::{vec2, vec3, Mat2, Mat4, Quat, Vec2};
use strum_macros::{EnumCount, EnumIter, IntoStaticStr};

use super::{
    enemy::Enemy,
    health::Health,
    mesh_drawable::MeshDrawable,
    missile::Missile,
    movable::Movable,
    player::Player,
    projectile::Projectile,
    size::Size,
    trap::{Trap, TrapType},
};
use crate::{
    camera::Camera,
    ecs::{
        entity::{Entity, EntitySystem},
        storage::Storage,
        world::World,
    },
    models::{BULLET, LASER, MISSILE, TARGET_INDICATOR},
    sound_mixer::SoundMixer,
    sounds::{LASER_1, MISSILE_1, SHOOT_3},
};

#[derive(EnumCount, EnumIter, IntoStaticStr, PartialEq, Eq, PartialOrd, Ord)]
pub enum WeaponType {
    Bullet,
    Laser,
    Missile,
    TripleMissile,
    Flak,
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

const FLAK_DELAY_MS: i32 = 900;
const BULLET_DELAY_MS: i32 = 300;
const MISSILE_DELAY_MS: i32 = 2000;

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
        .add(Size { size: BULLET.size })
        .add(Health {
            health: 5,
            damaged_this_frame: true,
        })
        .add(MeshDrawable {
            model: BULLET.as_model_data(),
            rot: Quat::IDENTITY,
        })
        .add(Projectile {
            target_type,
            damage: 50 + (n64_math::random_f32() * 20.0) as i32,
            projectile_collision_grace_period_ms: 0,
        })
        .add_optional(if n64_math::random_f32() < 0.1 {
            Some(Trap {
                trap_type: TrapType::BulletStorm,
                target_type,
            })
        } else {
            None
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
        .add(Size { size: MISSILE.size })
        .add(Health {
            health: 15,
            damaged_this_frame: true,
        })
        .add(MeshDrawable {
            model: MISSILE.as_model_data(),
            rot: Quat::IDENTITY,
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
    let extent = Mat2::from_angle(direction).mul_vec2(vec2(0.0, -LASER.size.y / 2.0));

    entities
        .spawn()
        .add(Movable {
            pos: pos + extent,
            speed,
        })
        .add(Size { size: LASER.size })
        .add(MeshDrawable {
            model: LASER.as_model_data(),
            rot: Quat::IDENTITY,
        })
        .add(Projectile {
            target_type,
            damage: 2,
            projectile_collision_grace_period_ms: 0,
        });
}

pub fn shoot_flak(
    entities: &mut EntitySystem,
    pos: Vec2,
    direction: f32,
    target_type: WeaponTarget,
) {
    for _ in 0..20 {
        let spread = 0.15 * (n64_math::random_f32() - 0.5);
        let offset_spread = -0.01 - 0.3 * n64_math::random_f32();
        let offset = vec2(spread, offset_spread);
        let speed_offset = Mat2::from_angle(direction).mul_vec2(vec2(0.0, -0.75));

        entities
            .spawn()
            .add(Movable {
                pos: pos + offset,
                speed: speed_offset,
            })
            .add(Size {
                size: BULLET.size * 0.3,
            })
            .add(Health {
                health: 5,
                damaged_this_frame: true,
            })
            .add(MeshDrawable {
                model: BULLET.as_model_data(),
                rot: Quat::IDENTITY,
            })
            .add(Projectile {
                target_type,
                damage: 50 + (n64_math::random_f32() * 20.0) as i32,
                projectile_collision_grace_period_ms: 0,
            });
    }
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
                    if target_type == WeaponTarget::Enemy {
                        sound_mixer.play_sound(SHOOT_3.as_sound_data());
                    }
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
            WeaponType::Laser => {
                if target_type == WeaponTarget::Enemy {
                    sound_mixer.play_sound(LASER_1.as_sound_data());
                }
                shoot_laser(entities, m.pos, m.speed, w.direction, target_type);
            }
            WeaponType::Missile => {
                if now - w.last_shoot_time > MISSILE_DELAY_MS as i64 * 1000 {
                    if target_type == WeaponTarget::Enemy {
                        sound_mixer.play_sound(MISSILE_1.as_sound_data());
                    }

                    let shooter_pos = m.pos;

                    let mut distances = enemy
                        .entities()
                        .iter()
                        .chain(player.entities())
                        .filter_map(|e| movable.lookup(*e).map(|m| (m, *e)))
                        .filter_map(|(m, e)| {
                            if e != entity && shooter_pos.y - m.pos.y > 0.0 {
                                Some(((shooter_pos - m.pos).length(), m.pos, e))
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>();

                    distances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

                    let target_1 = distances.get(0).map(|e| e.2);

                    shoot_missile(
                        entities,
                        m.pos,
                        vec2(0.0, -s.size.y / 2.0),
                        m.speed,
                        vec2(0.0, -0.5),
                        w.direction,
                        target_1,
                        target_type,
                    );

                    w.last_shoot_time = now;
                }
            }
            WeaponType::TripleMissile => {
                if now - w.last_shoot_time > MISSILE_DELAY_MS as i64 * 1000 {
                    if target_type == WeaponTarget::Enemy {
                        sound_mixer.play_sound(MISSILE_1.as_sound_data());
                    }

                    let shooter_pos = m.pos;

                    let mut distances = enemy
                        .entities()
                        .iter()
                        .chain(player.entities())
                        .filter_map(|e| movable.lookup(*e).map(|m| (m, *e)))
                        .filter_map(|(m, e)| {
                            if e != entity && shooter_pos.y - m.pos.y > 0.0 {
                                Some(((shooter_pos - m.pos).length(), m.pos, e))
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>();

                    distances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

                    distances.truncate(3);

                    distances.sort_by(|a, b| a.1.x.partial_cmp(&b.1.x).unwrap());

                    let target_1 = distances.get(0).map(|e| e.2);
                    let target_2 = distances.get(1).map(|e| e.2);
                    let target_3 = distances.get(2).map(|e| e.2);

                    let offset = vec2(0.0, -s.size.y / 2.0);

                    shoot_missile(
                        entities,
                        m.pos,
                        offset,
                        m.speed,
                        vec2(-0.15, -0.5),
                        w.direction,
                        target_1,
                        target_type,
                    );
                    shoot_missile(
                        entities,
                        m.pos,
                        offset,
                        m.speed,
                        vec2(0.0, -0.5),
                        w.direction,
                        target_2,
                        target_type,
                    );
                    shoot_missile(
                        entities,
                        m.pos,
                        offset,
                        m.speed,
                        vec2(0.15, -0.5),
                        w.direction,
                        target_3,
                        target_type,
                    );

                    w.last_shoot_time = now;
                }
            }
            WeaponType::Flak => {
                if now - w.last_shoot_time > FLAK_DELAY_MS as i64 * 1000 {
                    shoot_flak(entities, m.pos, w.direction, target_type);
                    w.last_shoot_time = now;
                }
            }
        }
    }
}

static TARGET_PIPELINE: Pipeline = Pipeline {
    color_combiner_mode: ColorCombinerMode::simple(
        ASrc::Zero,
        BSrc::Zero,
        CSrc::Zero,
        DSrc::Environment,
    ),
    ..Pipeline::default()
};

pub fn draw_missile_target(
    world: &mut World,
    cb: &mut CommandBuffer,
    video_mode: VideoMode,
    camera: &Camera,
) {
    n64::scope!("draw_missile_target");

    let (player, enemy, weapon, movable) =
        world.components.get::<(Player, Enemy, Weapon, Movable)>();

    let target_indicator = TARGET_INDICATOR.as_model_data();

    for player_entity in player.entities() {
        if let (Some(m), Some(w)) = (
            movable.lookup(*player_entity),
            weapon.lookup_mut(*player_entity),
        ) {
            let pipeline = if current_time_us() - w.last_shoot_time > MISSILE_DELAY_MS as i64 * 1000
            {
                TARGET_PIPELINE.with_env_color(Some(0x008000ff))
            } else {
                TARGET_PIPELINE.with_env_color(Some(0x800000ff))
            };

            cb.set_pipeline(&pipeline);

            if w.weapon_type == WeaponType::Missile {
                let shooter_pos = m.pos;

                let mut distances = enemy
                    .entities()
                    .iter()
                    .chain(player.entities())
                    .filter_map(|e| movable.lookup(*e).map(|m| (m, *e)))
                    .filter_map(|(m, e)| {
                        if e != *player_entity && shooter_pos.y - m.pos.y > 0.0 {
                            Some(((shooter_pos - m.pos).length(), m.pos, e))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();

                distances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
                distances.truncate(1);

                let proj = Mat4::perspective_rh_gl(PI / 2.0, 1.0, 0.01, 1000.0);

                for (_, pos, _) in distances {
                    let post_transform = Mat4::from_cols_array_2d(&[
                        [video_mode.width() as f32, 0.0, 0.0, 0.0],
                        [0.0, video_mode.height() as f32, 0.0, 0.0],
                        [0.0, 0.0, 1.0, 0.0],
                        [0.0, 0.0, 0.0, 1.0],
                    ]);

                    let transform = post_transform
                        * proj
                        * Mat4::from_translation(vec3(
                            pos.x - camera.pos.x,
                            pos.y - camera.pos.y,
                            -1.0,
                        ));

                    cb.add_mesh_indexed(
                        &target_indicator.verts,
                        &target_indicator.uvs,
                        &target_indicator.colors,
                        &target_indicator.indices,
                        &transform.to_cols_array_2d(),
                    );
                }
            } else if w.weapon_type == WeaponType::TripleMissile {
                let shooter_pos = m.pos;

                let mut distances = enemy
                    .entities()
                    .iter()
                    .chain(player.entities())
                    .filter_map(|e| movable.lookup(*e).map(|m| (m, *e)))
                    .filter_map(|(m, e)| {
                        if e != *player_entity && shooter_pos.y - m.pos.y > 0.0 {
                            Some(((shooter_pos - m.pos).length(), m.pos, e))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();

                distances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
                distances.truncate(3);
                distances.sort_by(|a, b| a.1.x.partial_cmp(&b.1.x).unwrap());

                let proj = Mat4::perspective_rh_gl(PI / 2.0, 1.0, 0.01, 1000.0);

                for (_, pos, _) in distances {
                    let post_transform = Mat4::from_cols_array_2d(&[
                        [video_mode.width() as f32, 0.0, 0.0, 0.0],
                        [0.0, video_mode.height() as f32, 0.0, 0.0],
                        [0.0, 0.0, 1.0, 0.0],
                        [0.0, 0.0, 0.0, 1.0],
                    ]);

                    let transform = post_transform
                        * proj
                        * Mat4::from_translation(vec3(
                            pos.x - camera.pos.x,
                            pos.y - camera.pos.y,
                            -1.0,
                        ));

                    cb.add_mesh_indexed(
                        &target_indicator.verts,
                        &target_indicator.uvs,
                        &target_indicator.colors,
                        &target_indicator.indices,
                        &transform.to_cols_array_2d(),
                    );
                }
            }
        }
    }
}
