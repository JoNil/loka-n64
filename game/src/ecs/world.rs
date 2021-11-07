use crate::ecs::component_storage::Storage;
use alloc::{rc::Rc, vec::Vec};
use core::{any::type_name, cell::RefCell};

use super::{
    entity::{Entity, EntitySystem},
    type_map::TypeMap,
};

pub struct World {
    pub entities: EntitySystem,
    components: TypeMap,
    removers: Vec<fn(&mut TypeMap, Entity)>,
}

impl World {
    pub fn new() -> Self {
        Self {
            entities: EntitySystem::new(),
            components: TypeMap::new(),
            removers: Vec::new(),
        }
    }

    pub fn add<T: 'static>(&mut self, entity: Entity, component: T) {
        if !self.components.contains::<Storage<T>>() {
            self.components.insert(Storage::<T>::new());
            self.removers.push(|components, entity| {
                let entry = components
                    .get::<Storage<T>>()
                    .unwrap_or_else(|| panic!("Could not find component: {}", type_name::<T>()));
                entry.borrow_mut().remove(entity);
            });
        }

        let entry = self
            .components
            .get::<Storage<T>>()
            .unwrap_or_else(|| panic!("Could not find component: {}", type_name::<T>()));

        entry.borrow_mut().add(entity, component);
    }

    pub fn get<T: 'static>(&mut self) -> Rc<RefCell<Storage<T>>> {
        if !self.components.contains::<Storage<T>>() {
            self.components.insert(Storage::<T>::new());
            self.removers.push(|components, entity| {
                let entry = components
                    .get::<Storage<T>>()
                    .unwrap_or_else(|| panic!("Could not find component: {}", type_name::<T>()));
                entry.borrow_mut().remove(entity);
            });
        }

        self.components
            .get::<Storage<T>>()
            .unwrap_or_else(|| panic!("Could not find component: {}", type_name::<T>()))
    }
    pub fn gc(&mut self) {
        self.entities
            .gc(&mut self.components, self.removers.as_slice());
    }
}
