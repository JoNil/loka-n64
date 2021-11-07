use crate::{camera::Camera, impl_system, world::World};
use n64_math::{Aabb2, Color, Vec2};

use super::{box_drawable::BoxDrawableComponent, movable::MovableComponent, player::SHIP_SIZE};

const BULLET_SIZE: Vec2 = Vec2::new(0.00825, 0.00825);

#[derive(Copy, Clone)]
struct Bullet {
    can_hit_player: bool,
    can_hit_enemy: bool,
}

pub fn shoot_bullet(world: &mut World, pos: Vec2, speed: Vec2) {
    let spread = (n64_math::random_f32() - 0.5) * 0.05;

    let entity = world.entity.create();
    world.movable.add(
        entity,
        MovableComponent {
            pos,
            speed: Vec2::new(speed.x() + spread, speed.y()),
        },
    );
    world.box_drawable.add(
        entity,
        BoxDrawableComponent {
            size: BULLET_SIZE,
            color: Color::from_rgb(0.2, 0.2, 0.9),
        },
    );
    world.bullet.add(
        entity,
        Bullet {
            can_hit_player: false,
            can_hit_enemy: true,
        },
    );
}

pub fn shoot_bullet_enemy(world: &mut World, pos: Vec2, speed: Vec2) {
    let entity = world.entity.create();
    world.movable.add(entity, MovableComponent { pos, speed });
    world.box_drawable.add(
        entity,
        BoxDrawableComponent {
            size: BULLET_SIZE,
            color: Color::from_rgb(0.9, 0.2, 0.2),
        },
    );
    world.bullet.add(
        entity,
        Bullet {
            can_hit_player: true,
            can_hit_enemy: false,
        },
    );
}

impl System {
    pub fn update(&mut self, world: &mut World, camera: &Camera) {
        let camera_bb: Aabb2 = Aabb2::new(camera.pos, camera.pos + Vec2::new(1.0, 1.0));

        for (bullet, entity) in self.components_and_entities() {
            if let Some(movable) = world.movable.lookup(entity) {
                let mut delete = false;
                let bullet_bb = Aabb2::from_center_size(movable.pos, BULLET_SIZE);

                if !bullet_bb.collides(&camera_bb) {
                    delete = true;
                }

                if bullet.can_hit_enemy {
                    for enemy_entity in world.enemy.entities() {
                        if let Some(sprite_drawable) = world.sprite_drawable.lookup(*enemy_entity) {
                            let enemy_bb = Aabb2::from_center_size(
                                world.movable.pos(*enemy_entity).unwrap_or_else(Vec2::zero),
                                sprite_drawable.size,
                            );

                            if bullet_bb.collides(&enemy_bb) {
                                world.health.damage(
                                    *enemy_entity,
                                    50 + (n64_math::random_f32() * 20.0) as i32,
                                );
                                delete = true;
                            }
                        }
                    }
                }

                if bullet.can_hit_player {
                    for player_entity in world.player.entities() {
                        let player_bb = Aabb2::from_center_size(
                            world.movable.pos(*player_entity).unwrap_or_else(Vec2::zero),
                            SHIP_SIZE,
                        );

                        if bullet_bb.collides(&player_bb) {
                            world.health.damage(
                                *player_entity,
                                50 + (n64_math::random_f32() * 20.0) as i32,
                            );
                            delete = true;
                        }
                    }
                }

                if delete {
                    entity.despawn();
                }
            }
        }
    }
}

impl_system!(Bullet);
