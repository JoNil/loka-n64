use super::{
    box_drawable::{self, BoxDrawableComponent},
    health,
    movable::{self, MovableComponent},
    player::SHIP_SIZE,
};
use crate::{camera::Camera, entity::EntitySystem, impl_component, world::World};
use n64_math::{Aabb2, Color, Vec2};

const BULLET_SIZE: Vec2 = Vec2::new(0.00825, 0.00825);

#[derive(Copy, Clone)]
struct Bullet {
    can_hit_player: bool,
    can_hit_enemy: bool,
}

impl_component!(Bullet);

pub fn shoot_bullet(
    entity_system: &mut EntitySystem,
    movable: &mut movable::Storage,
    box_drawable: &mut box_drawable::Storage,
    bullet: &mut Storage,
    pos: Vec2,
    speed: Vec2,
) {
    let spread = (n64_math::random_f32() - 0.5) * 0.05;

    let entity = entity_system.create();
    movable.add(
        entity,
        MovableComponent {
            pos,
            speed: Vec2::new(speed.x() + spread, speed.y()),
        },
    );
    box_drawable.add(
        entity,
        BoxDrawableComponent {
            size: BULLET_SIZE,
            color: Color::from_rgb(0.2, 0.2, 0.9),
        },
    );
    bullet.add(
        entity,
        Bullet {
            can_hit_player: false,
            can_hit_enemy: true,
        },
    );
}

pub fn shoot_bullet_enemy(
    entity_system: &mut EntitySystem,
    movable: &mut movable::Storage,
    box_drawable: &mut box_drawable::Storage,
    bullet: &mut Storage,
    pos: Vec2,
    speed: Vec2,
) {
    let entity = entity_system.create();
    movable.add(entity, MovableComponent { pos, speed });
    box_drawable.add(
        entity,
        BoxDrawableComponent {
            size: BULLET_SIZE,
            color: Color::from_rgb(0.9, 0.2, 0.2),
        },
    );
    bullet.add(
        entity,
        Bullet {
            can_hit_player: true,
            can_hit_enemy: false,
        },
    );
}

pub fn update(world: &mut World, camera: &Camera) {
    let camera_bb: Aabb2 = Aabb2::new(camera.pos, camera.pos + Vec2::new(1.0, 1.0));

    for (bullet, entity) in world.bullet.components_and_entities() {
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
                            movable::pos(&world.movable, *enemy_entity).unwrap_or_else(Vec2::zero),
                            sprite_drawable.size,
                        );

                        if bullet_bb.collides(&enemy_bb) {
                            health::damage(
                                &mut world.health,
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
                        movable::pos(&world.movable, *player_entity).unwrap_or_else(Vec2::zero),
                        SHIP_SIZE,
                    );

                    if bullet_bb.collides(&player_bb) {
                        health::damage(
                            &mut world.health,
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
