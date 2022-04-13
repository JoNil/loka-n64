use super::movable::{self, Movable};
use crate::ecs::{entity::Entity, world::World};

pub struct Missile {
    pub target: Option<Entity>,
}

pub fn update(world: &mut World) {
    let (missile, movable) = world.components.get2::<Missile, Movable>();

    for (missile, entity) in missile.components_and_entities() {
        let target_pos = missile
            .target
            .and_then(|target| movable::pos(movable, target));

        if let Some(m) = movable.lookup_mut(entity) {
            if let Some(target_pos) = target_pos {
                let towords_target = (target_pos - m.pos).normalize();
                let speed_dir = m.speed.normalize();
                let new_speed_dir = (0.05 * towords_target + 0.95 * speed_dir).normalize();
                let new_speed = new_speed_dir * m.speed.length();
                m.speed = new_speed;
            }
        }
    }
}
