use crate::ecs::storage::Storage;
use alloc::{rc::Rc, vec::Vec};
use core::{any::type_name, cell::RefCell, mem};

use super::{
    entity::{Entity, EntitySystem},
    type_map::TypeMap,
};

pub struct World {
    entities: EntitySystem,
    components: TypeMap,
    commands: Vec<Box<dyn FnOnce(&mut World)>>,
    removers: Vec<fn(&mut TypeMap, Entity)>,
}

impl World {
    pub fn new() -> Self {
        Self {
            entities: EntitySystem::new(),
            components: TypeMap::new(),
            commands: Vec::new(),
            removers: Vec::new(),
        }
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

    pub fn spawn(&mut self) -> EntityBuilder {
        EntityBuilder {
            entity: self.entities.create(),
            commands: &mut self.commands,
        }
    }

    pub fn housekeep(&mut self) {
        let commands = mem::replace(&mut self.commands, Vec::new());
        for command in commands.into_iter() {
            command(self);
        }

        self.entities
            .gc(&mut self.components, self.removers.as_slice());
    }
}

pub struct EntityBuilder<'a> {
    entity: Entity,
    commands: &'a mut Vec<Box<dyn FnOnce(&mut World)>>,
}

impl<'a> EntityBuilder<'a> {
    pub fn add<T: 'static>(&'a mut self, component: T) -> &'a mut Self {
        let entity = self.entity;

        self.commands.push(Box::new(move |world| {
            if !world.components.contains::<Storage<T>>() {
                world.components.insert(Storage::<T>::new());
                world.removers.push(|components, entity| {
                    let entry = components.get::<Storage<T>>().unwrap_or_else(|| {
                        panic!("Could not find component: {}", type_name::<T>())
                    });
                    entry.borrow_mut().remove(entity);
                });
            }

            let entry = world
                .components
                .get::<Storage<T>>()
                .unwrap_or_else(|| panic!("Could not find component: {}", type_name::<T>()));

            entry.borrow_mut().add(entity, component);
        }));

        self
    }

    pub fn entity(&self) -> Entity {
        self.entity
    }
}
