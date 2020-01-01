use crate::bullet_system::BulletSystem;
use crate::components::char_drawable::{self, CharDrawableComponent};
use crate::components::movable::{self, MovableComponent};
use crate::entity::{self, OwnedEntity};
use crate::Player;
use alloc::vec::Vec;
use n64::{current_time_us, graphics, ipl3font, Rng};
use n64_math::{Color, Vec2};

pub const ENEMY_SIZE: Vec2 = Vec2::new(
    ipl3font::GLYPH_WIDTH as f32 / graphics::WIDTH as f32,
    ipl3font::GLYPH_HEIGHT as f32 / graphics::HEIGHT as f32,
);

pub struct Enemy {
    entity: OwnedEntity,
    health: i32,
    shoot_speed: i32,
    last_shoot_time: i32,
}

impl Enemy {
    #[inline]
    pub fn pos(&self) -> Vec2 {
        if let Some(movable) = movable::get_component(&self.entity) {
            movable.pos
        } else {
            Vec2::zero()
        }
    }

    pub fn damage(&mut self, damage: i32) {
        self.health = 0.max(self.health - damage);
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

    pub fn spawn_enemy(&mut self, rng: &mut Rng) {
        let entity = entity::create();
        movable::add(&entity, MovableComponent {
            pos: Vec2::new(rng.next_f32(), rng.next_f32() * 0.6),
            speed: Vec2::zero(),
        });
        char_drawable::add(&entity, CharDrawableComponent {
            color: Color::from_rgb(rng.next_f32(), rng.next_f32(), rng.next_f32()),
            chr: b'E',
        });

        self.enemies.push(Enemy {
            entity: entity,
            health: 100,
            shoot_speed: 500 + (rng.next_f32() * 200.0) as i32,
            last_shoot_time: 0,
        });
    }

    pub fn update(&mut self, bullet_system: &mut BulletSystem, player: &mut Player, rng: &mut Rng) {
        let mut delete_list = Vec::new();

        let now = current_time_us();

        for (i, enemy) in self.enemies_mut().iter_mut().enumerate() {
            if enemy.health <= 0 {
                player.add_score(1000);
                delete_list.push(i);
            }

            if let Some(movable) = movable::get_component(&enemy.entity) {
                if now - enemy.last_shoot_time > enemy.shoot_speed * 1000 {
                    bullet_system.shoot_bullet_enemy(rng, movable.pos, Vec2::new(0.0, 0.65));
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

    #[inline]
    pub fn enemies(&self) -> &[Enemy] {
        &self.enemies
    }

    #[inline]
    pub fn enemies_mut(&mut self) -> &mut [Enemy] {
        &mut self.enemies
    }
}
