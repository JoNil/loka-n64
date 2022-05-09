use super::{
    health::{self, Health},
    movable::Movable,
    player::{self, Player},
    remove_when_below::RemoveWhenBelow,
    size::Size,
    spawner::Spawner,
    sprite_drawable::SpriteDrawable,
    weapon::{self, Weapon, WeaponTarget, WeaponType},
};
use crate::{
    ecs::{entity::EntitySystem, world::World},
    sound_mixer::SoundMixer,
    sounds::EXPLOSION_0,
};
use core::f32::consts::PI;
use n64::gfx::Texture;
use n64_math::{const_vec2, vec2, Vec2};

static ENEMY_WAYPOINT: [Vec2; 4] = [
    const_vec2!([0.4, 0.4]),
    const_vec2!([0.6, 0.4]),
    const_vec2!([0.6, 0.6]),
    const_vec2!([0.4, 0.6]),
];

pub struct Enemy {
    pub waypoint: usize,
    pub waypoint_step: f32,
}

pub fn add_enemy(entities: &mut EntitySystem, pos: Vec2, texture: Texture<'static>) {
    entities
        .spawn()
        .add(Movable {
            pos,
            speed: Vec2::ZERO,
        })
        .add(Size {
            size: vec2(texture.width as f32 / 320.0, texture.height as f32 / 240.0),
        })
        .add(Spawner { texture });
}

pub fn spawn_enemy(
    entities: &mut EntitySystem,
    movable: Movable,
    size: Size,
    texture: Texture<'static>,
) {
    entities
        .spawn()
        .add(movable)
        .add(size)
        .add(SpriteDrawable { texture })
        .add(Health {
            health: 100,
            damaged_this_frame: false,
        })
        .add(Weapon {
            weapon_type: WeaponType::Bullet,
            last_shoot_time: 0,
            direction: PI,
        })
        .add(Enemy {
            waypoint: 0,
            waypoint_step: 1.0,
        })
        .add(RemoveWhenBelow);
}

pub fn update(world: &mut World, sound_mixer: &mut SoundMixer, dt: f32) {
    let (enemy, movable, health, size, player, weapon) = world
        .components
        .get6::<Enemy, Movable, Health, Size, Player, Weapon>();

    for entity in enemy.entities() {
        if !health::is_alive(health, *entity) {
            sound_mixer.play_sound(EXPLOSION_0.as_sound_data());
            player::add_score(player, 1000);
            world.entities.despawn(*entity);
        }

        weapon::fire(
            &mut world.entities,
            *entity,
            sound_mixer,
            weapon,
            movable,
            size,
            enemy,
            player,
            WeaponTarget::Player,
        );
    }

    for (e, entity) in enemy.components_and_entities_mut() {
        if let Some(movable) = movable.lookup_mut(entity) {
            if e.waypoint_step >= 1.0 {
                e.waypoint_step -= 1.0;
                e.waypoint += 1;
                if e.waypoint >= ENEMY_WAYPOINT.len() {
                    e.waypoint = 0;
                }
            }

            let a_waypoint = (e.waypoint + 1) % ENEMY_WAYPOINT.len();
            let speed_a = ENEMY_WAYPOINT[a_waypoint] - ENEMY_WAYPOINT[e.waypoint];
            let b_waypoint = (a_waypoint + 1) % ENEMY_WAYPOINT.len();
            let speed_b = ENEMY_WAYPOINT[b_waypoint] - ENEMY_WAYPOINT[a_waypoint];

            movable.speed = (1.0 - e.waypoint_step) * speed_a + e.waypoint_step * speed_b;
            e.waypoint_step += dt;
        }
    }
}
