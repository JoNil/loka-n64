use crate::bullet_system::BulletSystem;
use crate::components::box_drawable::{self, BoxDrawableComponent};
use crate::components::health::{self, HealthComponent};
use crate::components::movable::{self, MovableComponent};
use crate::entity::{self, Entity, OwnedEntity};
use crate::Player;
use alloc::vec::Vec;
use n64::{current_time_us, graphics, ipl3font};
use n64_math::{self, Color, Vec2};

pub const ENEMY_SIZE: Vec2 = Vec2::new(
    ipl3font::GLYPH_WIDTH as f32 / graphics::WIDTH as f32,
    ipl3font::GLYPH_HEIGHT as f32 / graphics::HEIGHT as f32,
);

pub struct Enemy {
    entity: OwnedEntity,
    shoot_speed: i32,
    last_shoot_time: i64,
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
    pub fn new() -> EnemySystem {
        EnemySystem {
            enemies: Vec::new(),
        }
    }

    pub fn spawn_enemy(&mut self) {
        let entity = entity::create();
        movable::add(
            &entity,
            MovableComponent {
                pos: Vec2::new(n64_math::random_f32(), n64_math::random_f32() * 0.6),
                speed: Vec2::zero(),
            },
        );
        box_drawable::add(
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
        health::add(&entity, HealthComponent { health: 100 });

        self.enemies.push(Enemy {
            entity: entity,
            shoot_speed: 500 + (n64_math::random_f32() * 200.0) as i32,
            last_shoot_time: 0,
        });
    }

    pub fn update(&mut self, bullet_system: &mut BulletSystem, player: &mut Player) {
        let mut delete_list = Vec::new();

        let now = current_time_us();

        for (i, enemy) in self.enemies_mut().iter_mut().enumerate() {
            if !health::is_alive(&enemy.entity) {
                player.add_score(1000);
                delete_list.push(i);
            }

            if let Some(movable) = movable::get_component(&enemy.entity) {
                if now - enemy.last_shoot_time > enemy.shoot_speed as i64 * 1000 {
                    bullet_system.shoot_bullet_enemy(
                        movable.pos + Vec2::new(0.0, ENEMY_SIZE.y() / 2.0),
                        Vec2::new(0.0, 0.65),
                    );
                    enemy.last_shoot_time = now;
                }
            }
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
