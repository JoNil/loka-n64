use super::{
    diver_ai::DiverAi,
    health::{self, Health},
    movable::Movable,
    player::{self, Player},
    remove_when_below::RemoveWhenBelow,
    size::Size,
    spawner::{Spawner, SpawnerFunc},
    sprite_drawable::SpriteDrawable,
    trap::{Trap, TrapType},
    waypoint_ai::WaypointAi,
    weapon::{self, Weapon, WeaponTarget, WeaponType},
};
use crate::{
    ecs::{entity::EntitySystem, world::World},
    sound_mixer::SoundMixer,
    sounds::EXPLOSION_0,
};
use core::f32::consts::PI;
use n64::gfx::Texture;
use n64_math::{vec2, Vec2};

pub struct Enemy {}

pub fn add_enemy_spawner(
    entities: &mut EntitySystem,
    pos: Vec2,
    texture: Texture<'static>,
    spawner_func: SpawnerFunc,
) {
    entities
        .spawn()
        .add(Movable {
            pos,
            speed: Vec2::ZERO,
        })
        .add(Size {
            size: vec2(texture.width as f32 / 320.0, texture.height as f32 / 240.0),
        })
        .add(Spawner {
            texture,
            spawner_func,
        });
}

pub fn spawn_enemy_aircraft(
    entities: &mut EntitySystem,
    movable: Movable,
    size: Size,
    texture: Texture<'static>,
) {
    let mut b = entities.spawn();
    let mut b = b
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
        .add(WaypointAi {
            waypoint: 0,
            waypoint_step: 1.0,
        })
        .add(Enemy {})
        .add(RemoveWhenBelow);

    if n64_math::random_f32() < 1.2 {
        b.add(Trap {
            trap_type: if n64_math::random_f32() < 1.2 {
                TrapType::DualMissile
            } else {
                TrapType::BulletStorm
            },
            target_type: WeaponTarget::Player,
        });
    }
}

pub fn spawn_enemy_diver(
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
        .add(DiverAi {})
        .add(Enemy {})
        .add(RemoveWhenBelow);
}

pub fn update(world: &mut World, sound_mixer: &mut SoundMixer) {
    let (enemy, movable, health, size, player, weapon) = world
        .components
        .get6::<Enemy, Movable, Health, Size, Player, Weapon>();

    for entity in enemy.entities() {
        if !health::is_alive(health, *entity) {
            sound_mixer.play_sound(EXPLOSION_0.as_sound_data());
            player::add_score(player, 1000);
            world.entities.despawn(*entity);
        }

        if false {
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
    }
}
