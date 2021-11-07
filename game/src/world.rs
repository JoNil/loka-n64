use crate::{
    component_storage::Storage,
    entity::{Entity, EntitySystem},
    type_map::TypeMap,
};
use alloc::rc::Rc;
use core::{
    any::type_name,
    cell::{Ref, RefCell, RefMut},
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
        if !self.components.contains::<T>() {
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

    pub fn get<T: 'static>(&self) -> Rc<RefCell<Storage<T>>> {
        self.components
            .get::<Storage<T>>()
            .unwrap_or_else(|| panic!("Could not find component: {}", type_name::<T>()))
    }
    pub fn gc(&mut self) {
        self.entities
            .gc(&mut self.components, self.removers.as_slice());
    }
}
