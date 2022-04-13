use n64::current_time_us;
use n64_math::{vec2, Vec2};

use super::{
    bullet::shoot_bullet, laser::shoot_laser, missile::shoot_missile, movable::Movable, size::Size,
};
use crate::{
    ecs::{
        entity::{Entity, EntitySystem},
        storage::Storage,
    },
    sound_mixer::SoundMixer,
    sounds::{LASER_1, SHOOT_1, SHOOT_2},
};

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

const BULLET_DELAY_MS: i32 = 150;
const MISSILE_DELAY_MS: i32 = 1000;

pub fn fire(
    entities: &mut EntitySystem,
    entity: Entity,
    sound_mixer: &mut SoundMixer,
    movable: &mut Storage<Movable>,
    size: &mut Storage<Size>,
    weapon: &mut Storage<Weapon>,
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

                    let mut distances = movable
                        .components_and_entities()
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
