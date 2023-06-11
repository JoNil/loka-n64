use super::{
    diver_ai::DiverAi,
    health::{self, Health},
    mesh_drawable::MeshDrawable,
    movable::Movable,
    player::{self, Player},
    remove_when_below::RemoveWhenBelow,
    shadow::Shadow,
    size::Size,
    spawner::{Spawner, SpawnerData},
    sprite_drawable::SpriteDrawable,
    trap::{Trap, TrapType},
    waypoint_ai::WaypointAi,
    weapon::{self, Weapon, WeaponTarget, WeaponType},
};
use crate::{
    ecs::{entity::EntitySystem, storage::Storage, world::World},
    model::ModelData,
    sound_mixer::SoundMixer,
    sounds::EXPLOSION_0,
};
use core::f32::consts::PI;
use game_derive::SparseComponent;
use n64::gfx::Texture;
use n64_math::{Quat, Vec2};

#[derive(SparseComponent)]
pub struct Enemy {}

pub fn add_enemy_spawner(entities: &mut EntitySystem, pos: Vec2, spawner_data: SpawnerData) {
    entities
        .spawn()
        .add(Movable {
            pos,
            speed: Vec2::ZERO,
        })
        .add(spawner_data.size())
        .add(Spawner { data: spawner_data });
}

pub fn spawn_enemy_aircraft(
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
        .add(WaypointAi {
            waypoint: 0,
            waypoint_step: 1.0,
        })
        .add(Enemy {})
        .add(RemoveWhenBelow)
        .add_optional(if n64_math::random_f32() < 0.4 {
            Some(Trap {
                trap_type: if n64_math::random_f32() < 0.5 {
                    TrapType::DualMissile
                } else {
                    TrapType::BulletStorm
                },
                target_type: WeaponTarget::Player,
            })
        } else {
            None
        });
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

pub fn spawn_boss(
    entities: &mut EntitySystem,
    movable: Movable,
    size: Size,
    model: ModelData<'static>,
) {
    entities
        .spawn()
        .add(movable)
        .add(size)
        .add(MeshDrawable {
            model,
            rot: Quat::IDENTITY,
        })
        .add(Shadow)
        .add(Health {
            health: 10000,
            damaged_this_frame: false,
        })
        .add(Weapon {
            weapon_type: WeaponType::Laser,
            last_shoot_time: 0,
            direction: PI,
        })
        .add(WaypointAi {
            waypoint: 0,
            waypoint_step: 1.0,
        })
        .add(Enemy {})
        .add(RemoveWhenBelow);
}

pub fn update(world: &mut World, sound_mixer: &mut SoundMixer) {
    let (enemy, movable, health, size, player, weapon) =
        world
            .components
            .get::<(Enemy, Movable, Health, Size, Player, Weapon)>();

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
}
