use core::f32::consts::PI;

use n64_math::{vec2, Mat2, Quat, Vec2, Vec3};

use crate::{
    ecs::{
        entity::{Entity, EntitySystem},
        storage::Storage,
        world::World,
    },
    models::{BULLET, MISSILE},
};

use super::{
    health::{self, Health},
    mesh_drawable::MeshDrawable,
    missile::Missile,
    movable::Movable,
    player::Player,
    projectile::Projectile,
    size::Size,
    weapon::WeaponTarget,
};

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum TrapType {
    DualMissile,
    BulletStorm,
}
pub struct Trap {
    pub trap_type: TrapType,
    pub target_type: WeaponTarget,
}

pub fn update(world: &mut World) {
    let (trap, health, movable, player) = world.components.get4::<Trap, Health, Movable, Player>();

    for entity in trap.entities() {
        if !health::is_alive(health, *entity) {
            if let (Some(t), Some(m)) = (trap.lookup(*entity), movable.lookup(*entity)) {
                match t.trap_type {
                    TrapType::DualMissile => {
                        dual_missile(player, movable, &mut world.entities, t.target_type, m.pos)
                    }
                    TrapType::BulletStorm => {
                        bullet_storm(&mut world.entities, t.target_type, m.pos)
                    }
                }
            }

            // TODO: Trap sound
            //sound_mixer.play_sound(EXPLOSION_0.as_sound_data());
        }
    }
}

fn shoot_bullet(entities: &mut EntitySystem, target_type: WeaponTarget, pos: Vec2, angle: f32) {
    let dir = vec2(libm::cosf(angle), libm::sinf(angle));
    let offset = dir * 0.1;
    let speed = dir * 0.30;
    entities
        .spawn()
        .add(Movable {
            pos: pos + offset,
            speed,
        })
        .add(Size { size: BULLET.size })
        .add(Health {
            health: 5,
            damaged_this_frame: true,
        })
        .add(MeshDrawable {
            model: BULLET.as_model_data(),
            rot: Quat::from_axis_angle(Vec3::Z, angle + PI / 2.0),
        })
        .add(Projectile {
            target_type,
            damage: 50 + (n64_math::random_f32() * 20.0) as i32,
            projectile_collision_grace_period_ms: 0,
        });
}

fn bullet_storm(entities: &mut EntitySystem, target_type: WeaponTarget, pos: Vec2) {
    let mut angle = 0.0;
    for i in 0..9 {
        angle += PI * 2.0 / 9.0;
        shoot_bullet(entities, target_type, pos, angle);
    }
}

pub fn shoot_missile(
    entities: &mut EntitySystem,
    pos: Vec2,
    angle: f32,
    target: Option<Entity>,
    target_type: WeaponTarget,
) {
    let direction = vec2(libm::cosf(angle), libm::sinf(angle));
    let offset = direction * 0.10;
    let speed = direction * 0.01;

    let speed_offset = vec2(0.0, 0.0);

    let spread = 0.0; //(n64_math::random_f32() - 0.5) * 0.05;

    let rot = Mat2::from_angle(angle);

    //let offset = rot.mul_vec2(offset);
    let speed_offset = rot.mul_vec2(vec2(speed_offset.x + spread, speed_offset.y));

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

fn dual_missile(
    player: &mut Storage<Player>,
    movable: &mut Storage<Movable>,
    entities: &mut EntitySystem,
    target_type: WeaponTarget,
    pos: Vec2,
) {
    if target_type == WeaponTarget::Player {
        for player_entity in player.entities() {
            if let Some(m2) = movable.lookup(*player_entity) {
                shoot_missile(
                    entities,
                    pos,
                    PI * 1.0 / 4.0, // * -3.0 / 4.0,
                    Some(*player_entity),
                    target_type,
                );
                shoot_missile(
                    entities,
                    pos,
                    PI * 3.0 / 4.0, // / 4.0,
                    Some(*player_entity),
                    target_type,
                );
            }
        }
    } else {
    }
}
