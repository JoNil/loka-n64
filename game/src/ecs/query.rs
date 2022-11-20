use super::{entity::Entity, storage::Storage, world::World};
use crate::components::{movable::Movable, waypoint_ai::WaypointAi};

pub struct Query<'w> {
    entities: &'w [Entity],
    c1: &'w mut [WaypointAi],
    c2: &'w mut Storage<Movable>,
    index: i32,
}

impl<'w> Query<'w> {
    pub fn new(world: &'w mut World) -> Self {
        let storage = world.components.get2::<WaypointAi, Movable>();

        let (entities, c1) = storage.0.components_and_entities_slice_mut();

        Self {
            entities,
            c1,
            c2: storage.1,
            index: 0,
        }
    }
}

impl<'w> Iterator for Query<'w> {
    type Item = (Entity, &'w mut WaypointAi, &'w mut Movable);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.index >= self.entities.len() as i32 {
                return None;
            }

            let e = self.entities[self.index as usize];
            let c1 = &mut self.c1[self.index as usize];

            self.index += 1;

            let Some(c2) = self.c2.lookup_mut(e) else {
                continue;
            };

            return unsafe { Some((e, &mut *(c1 as *mut _), &mut *(c2 as *mut _))) };
        }
    }
}
