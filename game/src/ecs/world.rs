use crate::ecs::storage::Storage;
use alloc::{boxed::Box, rc::Rc, vec::Vec};
use core::{any::type_name, cell::RefCell, mem};

use super::{
    component_map::ComponentMap,
    entity::{Entity, EntitySystem},
};

pub struct World {
    pub entities: EntitySystem,
    pub components: ComponentMap,
    removers: Vec<fn(&mut ComponentMap, Entity)>,
}

impl World {
    pub fn new() -> Self {
        Self {
            entities: EntitySystem::new(),
            components: ComponentMap::new(),
            removers: Vec::new(),
        }
    }

    pub fn housekeep(&mut self) {
        self.entities.housekeep(&mut self.components, self.);
    }
}
