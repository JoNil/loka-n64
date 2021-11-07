use super::{
    bullet::shoot_bullet_enemy,
    health::{self, HealthComponent},
    movable::MovableComponent,
    player,
    sprite_drawable::SpriteDrawableComponent,
};
use crate::{impl_component, sound_mixer::SoundMixer, sounds::EXPLOSION_0, world::World};
use n64::{current_time_us, gfx::Texture};
use n64_math::Vec2;

static ENEMY_WAYPOINT: [Vec2; 4] = [
    Vec2::new(0.4, 0.4),
    Vec2::new(0.6, 0.4),
    Vec2::new(0.6, 0.6),
    Vec2::new(0.4, 0.6),
];

#[derive(Copy, Clone)]
pub struct Enemy {
    shoot_speed: i32,
    last_shoot_time: i64,
    waypoint: usize,
    waypoint_step: f32,
}

impl_component!(Enemy);

pub fn spawn_enemy(world: &mut World, pos: Vec2, texture: Texture<'static>) {
    let entity = world.entities.create();
    world.movable.add(
        entity,
        MovableComponent {
            pos,
            speed: Vec2::zero(),
        },
    );
    world.sprite_drawable.add(
        entity,
        SpriteDrawableComponent {
            size: Vec2::new(texture.width as f32 / 320.0, texture.height as f32 / 240.0),
            texture,
        },
    );
    world.health.add(entity, HealthComponent { health: 100 });
    world.enemy.add(
        entity,
        Enemy {
            shoot_speed: 500 + (n64_math::random_f32() * 200.0) as i32,
            last_shoot_time: 0,
            waypoint: 0,
            waypoint_step: 1.0,
        },
    );
}

pub fn update(world: &mut World, sound_mixer: &mut SoundMixer, dt: f32) {
    let now = current_time_us();

    for (enemy, entity) in world.enemy.components_and_entities_mut() {
        if !health::is_alive(&world.health, entity) {
            sound_mixer.play_sound(EXPLOSION_0.as_sound_data());
            player::add_score(&mut world.player, 1000);
            entity.despawn();
        }

        if now - enemy.last_shoot_time > enemy.shoot_speed as i64 * 1000 {
            if let (Some(movable), Some(sprite_drawable)) = (
                world.movable.lookup(entity).copied(),
                world.sprite_drawable.lookup(entity).copied(),
            ) {
                //sound_mixer.play_sound(SHOOT_0.as_sound_data());
                shoot_bullet_enemy(
                    &mut world.entities,
                    &mut world.movable,
                    &mut world.box_drawable,
                    &mut world.bullet,
                    movable.pos + Vec2::new(0.0, sprite_drawable.size.y() / 2.0),
                    Vec2::new(0.0, 1.25),
                );
                enemy.last_shoot_time = now;
            }
        }
    }

    ai(world, dt);
}

fn ai(world: &mut World, dt: f32) {
    for (enemy, entity) in world.enemy.components_and_entities_mut() {
        if let Some(movable) = world.movable.lookup_mut(entity) {
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
