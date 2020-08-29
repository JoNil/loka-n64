use crate::bullet_system::BulletSystem;
use crate::components::box_drawable::BoxDrawableComponent;
use crate::components::health::HealthComponent;
use crate::components::movable::MovableComponent;
use crate::entity::{Entity, OwnedEntity};
use crate::{sound_mixer::SoundMixer, sounds::EXPLOSION_0, world::World, Player};
use alloc::vec::Vec;
use n64::current_time_us;
use n64_math::{self, Color, Vec2};

pub const ENEMY_SIZE: Vec2 = Vec2::new(0.05, 0.05);

static ENEMY_WAYPOINT: [Vec2; 4] = [
    Vec2::new(0.4, 0.4),
    Vec2::new(0.6, 0.4),
    Vec2::new(0.6, 0.6),
    Vec2::new(0.4, 0.6),
];

fn ai(world: &mut World, enemy: &mut Enemy, dt: f32) {
    if let Some(movable) = world.movable.lookup_mut(&enemy.entity) {
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

pub struct Enemy {
    entity: OwnedEntity,
    shoot_speed: i32,
    last_shoot_time: i64,
    waypoint: usize,
    waypoint_step: f32,
}

impl Enemy {
    pub fn entity(&self) -> &Entity {
        &self.entity
    }
}

pub struct EnemySystem {
    enemies: Vec<Enemy>,
}

impl EnemySystem {
    pub fn new() -> Self {
        Self {
            enemies: Vec::with_capacity(256),
        }
    }

    pub fn spawn_enemy(&mut self, world: &mut World, pos: Vec2) {
        let entity = world.entity.create();
        world.movable.add(
            &entity,
            MovableComponent {
                pos,
                speed: Vec2::zero(),
            },
        );
        world.box_drawable.add(
            &entity,
            BoxDrawableComponent {
                size: ENEMY_SIZE,
                color: Color::from_rgb(
                    n64_math::random_f32(),
                    n64_math::random_f32(),
                    n64_math::random_f32(),
                ),
            },
        );
        world.health.add(&entity, HealthComponent { health: 100 });

        self.enemies.push(Enemy {
            entity,
            shoot_speed: 500 + (n64_math::random_f32() * 200.0) as i32,
            last_shoot_time: 0,
            waypoint: 0,
            waypoint_step: 1.0,
        });
    }

    pub fn update(
        &mut self,
        world: &mut World,
        bullet_system: &mut BulletSystem,
        player: &mut Player,
        sound_mixer: &mut SoundMixer,
        dt: f32,
    ) {
        let mut delete_list = Vec::new();

        let now = current_time_us();

        for (i, enemy) in self.enemies_mut().iter_mut().enumerate() {
            if !world.health.is_alive(&enemy.entity) {
                sound_mixer.play_sound(EXPLOSION_0.as_sound_data());
                player.add_score(1000);
                delete_list.push(i);
            }

            if now - enemy.last_shoot_time > enemy.shoot_speed as i64 * 1000 {
                if let Some(movable) = world.movable.lookup(&enemy.entity).copied() {
                    //sound_mixer.play_sound(SHOOT_0.as_sound_data());
                    bullet_system.shoot_bullet_enemy(
                        world,
                        movable.pos + Vec2::new(0.0, ENEMY_SIZE.y() / 2.0),
                        Vec2::new(0.0, 1.25),
                    );
                    enemy.last_shoot_time = now;
                }
            }

            ai(world, enemy, dt);
        }

        {
            let len = self.enemies.len();

            for (i, delete_index) in delete_list.iter().enumerate() {
                self.enemies.swap(*delete_index, len - 1 - i);
            }

            self.enemies.drain((len - delete_list.len())..);
        }
    }

    #[allow(dead_code)]
    #[inline]
    pub fn enemies(&self) -> &[Enemy] {
        &self.enemies
    }

    #[inline]
    pub fn enemies_mut(&mut self) -> &mut [Enemy] {
        &mut self.enemies
    }
}
