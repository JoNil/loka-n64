use super::{
    collider::CollisionMask,
    health::{self, Health},
    movable::Movable,
    player::{self, Player},
    size::Size,
    sprite_drawable::SpriteDrawable,
    weapon::{self, Weapon, WeaponType},
};
use crate::{
    ecs::{entity::EntitySystem, world::World},
    sound_mixer::SoundMixer,
    sounds::EXPLOSION_0,
};
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

pub fn spawn_enemy(entities: &mut EntitySystem, pos: Vec2, texture: Texture<'static>) {
    entities
        .spawn()
        .add(Movable {
            pos,
            speed: Vec2::ZERO,
        })
        .add(Size {
            size: vec2(texture.width as f32 / 320.0, texture.height as f32 / 240.0),
        })
        .add(SpriteDrawable { texture })
        .add(Health { health: 100 })
        .add(Weapon {
            weapon_type: WeaponType::Bullet,
            last_shoot_time: 0,
        })
        .add(Enemy {
            waypoint: 0,
            waypoint_step: 1.0,
        });
}

pub fn update(world: &mut World, sound_mixer: &mut SoundMixer, dt: f32) {
    let (enemy, movable, health, size, player, weapon) = world
        .components
        .get6::<Enemy, Movable, Health, Size, Player, Weapon>();

    for (enemy, entity) in enemy.components_and_entities_mut() {
        if !health::is_alive(health, entity) {
            sound_mixer.play_sound(EXPLOSION_0.as_sound_data());
            player::add_score(player, 1000);
            world.entities.despawn(entity);
        }

        weapon::fire(
            &mut world.entities,
            entity,
            sound_mixer,
            movable,
            size,
            weapon,
            CollisionMask::player(),
        );

        if let Some(movable) = movable.lookup_mut(entity) {
            if enemy.waypoint_step >= 1.0 {
                enemy.waypoint_step -= 1.0;
                enemy.waypoint += 1;
                if enemy.waypoint >= ENEMY_WAYPOINT.len() {
                    enemy.waypoint = 0;
                }
            }

            let a_waypoint = (enemy.waypoint + 1) % ENEMY_WAYPOINT.len();
            let speed_a = ENEMY_WAYPOINT[a_waypoint] - ENEMY_WAYPOINT[enemy.waypoint];
            let b_waypoint = (a_waypoint + 1) % ENEMY_WAYPOINT.len();
            let speed_b = ENEMY_WAYPOINT[b_waypoint] - ENEMY_WAYPOINT[a_waypoint];

            movable.speed = (1.0 - enemy.waypoint_step) * speed_a + enemy.waypoint_step * speed_b;
            enemy.waypoint_step += dt;
        }
    }
}
