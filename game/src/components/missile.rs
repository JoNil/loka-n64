use n64_math::{Quat, Vec2};

use super::{
    mesh_drawable::MeshDrawable,
    movable::{self, Movable},
};
use crate::ecs::{entity::Entity, world::World};

const MISSILE_ACCELERATION: f32 = 0.6;
const MISSILE_MAX_SPEED: f32 = 1.0;

pub struct Missile {
    pub target: Option<Entity>,
}

pub fn update(world: &mut World, dt: f32) {
    let (missile, movable, mesh_drawable) =
        world.components.get3::<Missile, Movable, MeshDrawable>();

    for (missile, entity) in missile.components_and_entities() {
        let target_pos = missile
            .target
            .and_then(|target| movable::pos(movable, target));

        if let (Some(m), Some(d)) = (movable.lookup_mut(entity), mesh_drawable.lookup_mut(entity)) {
            if let Some(target_pos) = target_pos {
                let towords_target = (target_pos - m.pos).normalize();
                let speed_dir = m.speed.normalize();
                let new_speed_dir = (0.02 * towords_target + 0.98 * speed_dir).normalize();
                let new_speed = new_speed_dir
                    * MISSILE_MAX_SPEED.min(m.speed.length() + MISSILE_ACCELERATION * dt);
                m.speed = new_speed;

                d.rot = Quat::from_rotation_z((-Vec2::Y).angle_between(new_speed_dir))
            }
        }
    }
}
