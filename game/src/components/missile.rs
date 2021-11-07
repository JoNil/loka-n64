use super::{
    box_drawable::{self, BoxDrawableComponent},
    health,
    movable::{self, MovableComponent},
};
use crate::{
    camera::Camera,
    entity::{Entity, EntitySystem},
    impl_component,
    world::World,
};
use n64_math::{Aabb2, Color, Vec2};

const MISSILE_SIZE: Vec2 = Vec2::new(4.0 * 0.00825, 4.0 * 0.00825);

#[derive(Copy, Clone)]
struct Missile {
    target: Option<Entity>,
}

impl_component!(Missile);

pub fn shoot_missile(
    entity_system: &mut EntitySystem,
    movable: &mut movable::Storage,
    box_drawable: &mut box_drawable::Storage,
    missile: &mut Storage,
    pos: Vec2,
    speed: Vec2,
    target: Option<Entity>,
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
            size: MISSILE_SIZE,
            color: Color::from_rgb(1.0, 0.4, 0.4),
        },
    );
    missile.add(entity, Missile { target });
}

pub fn update(world: &mut World, camera: &Camera) {
    let camera_bb: Aabb2 = Aabb2::new(camera.pos, camera.pos + Vec2::new(1.0, 1.0));

    for (missile, entity) in world.missile.components_and_entities() {
        let target_pos = missile
            .target
            .and_then(|target| movable::pos(&world.movable, target));

        if let Some(movable) = world.movable.lookup_mut(entity) {
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

            for enemy_entity in world.enemy.entities() {
                if let Some(sprite_drawable) = world.sprite_drawable.lookup(*enemy_entity) {
                    let enemy_bb = Aabb2::from_center_size(
                        movable::pos(&world.movable, *enemy_entity).unwrap_or_else(Vec2::zero),
                        sprite_drawable.size,
                    );

                    if missile_bb.collides(&enemy_bb) {
                        health::damage(
                            &mut world.health,
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
