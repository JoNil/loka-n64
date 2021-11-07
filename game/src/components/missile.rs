use super::{
    box_drawable::BoxDrawable,
    enemy::Enemy,
    health,
    movable::{self, Movable},
    sprite_drawable::SpriteDrawable,
};
use crate::{camera::Camera, entity::Entity, world::World};
use n64_math::{Aabb2, Color, Vec2};

const MISSILE_SIZE: Vec2 = Vec2::new(4.0 * 0.00825, 4.0 * 0.00825);

#[derive(Copy, Clone)]
struct Missile {
    target: Option<Entity>,
}

pub fn shoot_missile(world: &mut World, pos: Vec2, speed: Vec2, target: Option<Entity>) {
    let spread = (n64_math::random_f32() - 0.5) * 0.05;

    let entity = world.entities.create();
    world.add(
        entity,
        Movable {
            pos,
            speed: Vec2::new(speed.x() + spread, speed.y()),
        },
    );
    world.add(
        entity,
        BoxDrawable {
            size: MISSILE_SIZE,
            color: Color::from_rgb(1.0, 0.4, 0.4),
        },
    );
    world.add(entity, Missile { target });
}

pub fn update(world: &mut World, camera: &Camera) {
    let camera_bb: Aabb2 = Aabb2::new(camera.pos, camera.pos + Vec2::new(1.0, 1.0));

    for (missile, entity) in world.components_and_entities::<Missile>() {
        let target_pos = missile
            .target
            .and_then(|target| movable::pos(world, target));

        if let Some(movable) = world.lookup_mut::<Movable>(entity) {
            if let Some(target_pos) = target_pos {
                let towords_target = (target_pos - movable.pos).normalize();
                let speed_dir = movable.speed.normalize();
                let new_speed_dir = (0.05 * towords_target + 0.95 * speed_dir).normalize();
                let new_speed = new_speed_dir * movable.speed.length();
                movable.speed = new_speed;
            }

            let mut delete = false;
            let missile_bb = Aabb2::from_center_size(movable.pos, MISSILE_SIZE);

            if !missile_bb.collides(&camera_bb) {
                delete = true;
            }

            for enemy_entity in world.entities::<Enemy>() {
                if let Some(sprite_drawable) = world.lookup::<SpriteDrawable>(*enemy_entity) {
                    let enemy_bb = Aabb2::from_center_size(
                        movable::pos(world, *enemy_entity).unwrap_or_else(Vec2::zero),
                        sprite_drawable.size,
                    );

                    if missile_bb.collides(&enemy_bb) {
                        health::damage(
                            world,
                            *enemy_entity,
                            100 + (n64_math::random_f32() * 50.0) as i32,
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
