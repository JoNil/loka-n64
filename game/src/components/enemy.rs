use super::{
    bullet::shoot_bullet_enemy,
    health::{self, Health},
    movable::Movable,
    player::{self, Player},
    size::Size,
    sprite_drawable::SpriteDrawable,
};
use crate::{
    ecs::{entity::EntitySystem, world::World},
    sound_mixer::SoundMixer,
    sounds::EXPLOSION_0,
};
use n64::{current_time_us, gfx::Texture};
use n64_math::{const_vec2, vec2, Vec2};

static ENEMY_WAYPOINT: [Vec2; 4] = [
    const_vec2!([0.4, 0.4]),
    const_vec2!([0.6, 0.4]),
    const_vec2!([0.6, 0.6]),
    const_vec2!([0.4, 0.6]),
];

pub struct Enemy {
    pub shoot_speed: i32,
    pub last_shoot_time: i64,
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
        .add(Enemy {
            shoot_speed: 500 + (n64_math::random_f32() * 200.0) as i32,
            last_shoot_time: 0,
            waypoint: 0,
            waypoint_step: 1.0,
        });
}

pub fn update(world: &mut World, sound_mixer: &mut SoundMixer, dt: f32) {
    {
        let (enemy, movable, health, size, player) = world
            .components
            .get5::<Enemy, Movable, Health, Size, Player>();

        let now = current_time_us();

        for (enemy, entity) in enemy.components_and_entities_mut() {
            if !health::is_alive(health, entity) {
                sound_mixer.play_sound(EXPLOSION_0.as_sound_data());
                player::add_score(player, 1000);
                world.entities.despawn(entity);
            }

            if now - enemy.last_shoot_time > enemy.shoot_speed as i64 * 1000 {
                if let (Some(movable), Some(size)) = (
                    movable.lookup(entity).copied(),
                    size.lookup(entity).copied(),
                ) {
                    //sound_mixer.play_sound(SHOOT_0.as_sound_data());
                    shoot_bullet_enemy(
                        &mut world.entities,
                        movable.pos + vec2(0.0, size.size.y / 2.0),
                        vec2(0.0, 1.25),
                    );
                    enemy.last_shoot_time = now;
                }
            }
        }
    }

    ai(world, dt);
}

fn ai(world: &mut World, dt: f32) {
    let (enemy, movable) = world.components.get2::<Enemy, Movable>();

    for (enemy, entity) in enemy.components_and_entities_mut() {
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
