use crate::components::box_drawable::{self, BoxDrawableComponent};
use crate::components::health;
use crate::components::movable::{self, MovableComponent};
use crate::enemy_system::{EnemySystem, ENEMY_SIZE};
use crate::entity::{self, OwnedEntity};
use crate::{Player, SHIP_SIZE};
use alloc::vec::Vec;
use n64_math::{self, Aabb2, Color, Vec2};

const BULLET_SIZE: Vec2 = Vec2::new(0.00625, 0.00625);

struct Bullet {
    entity: OwnedEntity,
    can_hit_player: bool,
    can_hit_enemy: bool,
}

pub struct BulletSystem {
    bullets: Vec<Bullet>,
    screen_bb: Aabb2,
}

impl BulletSystem {
    pub fn new() -> Self {
        Self {
            bullets: Vec::new(),
            screen_bb: Aabb2::new(Vec2::zero(), Vec2::new(1.0, 1.0)),
        }
    }

    pub fn shoot_bullet(&mut self, pos: Vec2, speed: Vec2) {

        let spread = (n64_math::random_f32() - 0.5) * 0.05;

        let entity = entity::create();
        movable::add(
            &entity,
            MovableComponent {
                pos,
                speed: Vec2::new(speed.x() + spread, speed.y()),
            },
        );
        box_drawable::add(
            &entity,
            BoxDrawableComponent {
                size: BULLET_SIZE,
                color: Color::from_rgb(
                    n64_math::random_f32(),
                    n64_math::random_f32(),
                    n64_math::random_f32(),
                ),
            },
        );

        self.bullets.push(Bullet {
            entity,
            can_hit_player: false,
            can_hit_enemy: true,
        });
    }

    pub fn shoot_bullet_enemy(&mut self, pos: Vec2, speed: Vec2) {
        let entity = entity::create();
        movable::add(&entity, MovableComponent { pos, speed });
        box_drawable::add(
            &entity,
            BoxDrawableComponent {
                size: BULLET_SIZE,
                color: Color::from_rgb(
                    n64_math::random_f32(),
                    n64_math::random_f32(),
                    n64_math::random_f32(),
                ),
            },
        );

        self.bullets.push(Bullet {
            entity,
            can_hit_player: true,
            can_hit_enemy: false,
        });
    }

    pub fn update(&mut self, enemy_system: &mut EnemySystem, player: &mut Player) {
        let mut delete_list = Vec::new();

        for (i, bullet) in self.bullets.iter_mut().enumerate() {
            if let Some(movable) = movable::get_component(&bullet.entity) {
                let bullet_bb = Aabb2::from_center_size(movable.pos, BULLET_SIZE);

                if !bullet_bb.collides(&self.screen_bb) {
                    delete_list.push(i);
                }

                if bullet.can_hit_enemy {
                    for enemy in enemy_system.enemies_mut() {
                        let enemy_bb = Aabb2::from_center_size(
                            movable::pos(enemy.entity()).unwrap_or(Vec2::zero()),
                            ENEMY_SIZE,
                        );

                        if bullet_bb.collides(&enemy_bb) {
                            health::damage(
                                enemy.entity(),
                                50 + (n64_math::random_f32() * 20.0) as i32,
                            );
                            delete_list.push(i);
                        }
                    }
                }

                if bullet.can_hit_player {
                    let player_bb = Aabb2::from_center_size(
                        movable::pos(player.entity()).unwrap_or(Vec2::zero()),
                        SHIP_SIZE,
                    );

                    if bullet_bb.collides(&player_bb) {
                        health::damage(
                            player.entity(),
                            50 + (n64_math::random_f32() * 20.0) as i32,
                        );
                        delete_list.push(i);
                    }
                }
            }
        }

        {
            let len = self.bullets.len();

            for (i, delete_index) in delete_list.iter().enumerate() {
                self.bullets.swap(*delete_index, len - 1 - i);
            }

            self.bullets.drain((len - delete_list.len())..);
        }
    }
}
