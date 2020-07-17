use crate::components::box_drawable::BoxDrawableComponent;
use crate::components::movable::MovableComponent;
use crate::enemy_system::{EnemySystem, ENEMY_SIZE};
use crate::entity::OwnedEntity;
use crate::{camera::Camera, world::World, Player, SHIP_SIZE};
use alloc::vec::Vec;
use n64_math::{self, Aabb2, Color, Vec2};

const BULLET_SIZE: Vec2 = Vec2::new(0.00825, 0.00825);

struct Bullet {
    entity: OwnedEntity,
    can_hit_player: bool,
    can_hit_enemy: bool,
}

pub struct BulletSystem {
    bullets: Vec<Bullet>,
}

impl BulletSystem {
    pub fn new() -> Self {
        Self {
            bullets: Vec::new(),
        }
    }

    pub fn shoot_bullet(&mut self, world: &mut World, pos: Vec2, speed: Vec2) {
        let spread = (n64_math::random_f32() - 0.5) * 0.05;

        let entity = world.entity.create();
        world.movable.add(
            &entity,
            MovableComponent {
                pos,
                speed: Vec2::new(speed.x() + spread, speed.y()),
            },
        );
        world.box_drawable.add(
            &entity,
            BoxDrawableComponent {
                size: BULLET_SIZE,
                color: Color::from_rgb(0.2, 0.2, 0.9),
            },
        );

        self.bullets.push(Bullet {
            entity,
            can_hit_player: false,
            can_hit_enemy: true,
        });
    }

    pub fn shoot_bullet_enemy(&mut self, world: &mut World, pos: Vec2, speed: Vec2) {
        let entity = world.entity.create();
        world.movable.add(&entity, MovableComponent { pos, speed });
        world.box_drawable.add(
            &entity,
            BoxDrawableComponent {
                size: BULLET_SIZE,
                color: Color::from_rgb(0.9, 0.2, 0.2),
            },
        );

        self.bullets.push(Bullet {
            entity,
            can_hit_player: true,
            can_hit_enemy: false,
        });
    }

    pub fn update(
        &mut self,
        world: &mut World,
        enemy_system: &mut EnemySystem,
        player: &mut Player,
        camera: &Camera,
    ) {
        let mut delete_list = Vec::new();

        let camera_bb: Aabb2 = Aabb2::new(camera.pos, camera.pos + Vec2::new(1.0, 1.0));

        for (i, bullet) in self.bullets.iter_mut().enumerate() {
            if let Some(movable) = world.movable.lookup(&bullet.entity) {
                let mut delete = false;
                let bullet_bb = Aabb2::from_center_size(movable.pos, BULLET_SIZE);

                if !bullet_bb.collides(&camera_bb) {
                    delete = true;
                }

                if bullet.can_hit_enemy {
                    for enemy in enemy_system.enemies_mut() {
                        let enemy_bb = Aabb2::from_center_size(
                            world.movable.pos(enemy.entity()).unwrap_or_else(Vec2::zero),
                            ENEMY_SIZE,
                        );

                        if bullet_bb.collides(&enemy_bb) {
                            world.health.damage(
                                enemy.entity(),
                                50 + (n64_math::random_f32() * 20.0) as i32,
                            );
                            delete = true;
                        }
                    }
                }

                if bullet.can_hit_player {
                    let player_bb = Aabb2::from_center_size(
                        world.movable.pos(player.entity()).unwrap_or_else(Vec2::zero),
                        SHIP_SIZE,
                    );

                    if bullet_bb.collides(&player_bb) {
                        world
                            .health
                            .damage(player.entity(), 50 + (n64_math::random_f32() * 20.0) as i32);
                        delete = true;
                    }
                }

                if delete {
                    delete_list.push(i);
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
