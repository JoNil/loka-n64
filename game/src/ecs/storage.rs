#![allow(dead_code)]

use super::entity::Entity;
use alloc::vec::Vec;
use hashbrown::HashMap;

pub struct Storage<T> {
    components: Vec<T>,
    entities: Vec<Entity>,
    map: HashMap<Entity, usize, n64_math::BuildFnvHasher>,
}

impl<T> Storage<T> {
    pub fn new() -> Self {
        Self {
            components: Vec::with_capacity(256),
            entities: Vec::with_capacity(256),
            map: HashMap::with_capacity_and_hasher(256, n64_math::BuildFnvHasher),
        }
    }

    pub fn add(&mut self, entity: Entity, component: T) {
        self.components.push(component);
        self.entities.push(entity);
        self.map.insert(entity, self.components.len() - 1);
    }

    pub fn lookup(&self, entity: Entity) -> Option<&T> {
        if let Some(&index) = self.map.get(&entity) {
            return Some(&self.components[index]);
        }

        None
    }

    pub fn lookup_mut(&mut self, entity: Entity) -> Option<&mut T> {
        if let Some(&index) = self.map.get(&entity) {
            return Some(&mut self.components[index]);
        }

        None
    }

    pub fn components(&self) -> &[T] {
        &self.components
    }

    pub fn components_mut(&mut self) -> &mut [T] {
        &mut self.components
    }

    pub fn entities(&self) -> &[Entity] {
        &self.entities
    }

    pub fn components_and_entities(&self) -> impl Iterator<Item = (&T, Entity)> {
        self.components.iter().zip(self.entities.iter().copied())
    }

    pub fn components_and_entities_mut(&mut self) -> impl Iterator<Item = (&mut T, Entity)> {
        self.components
            .iter_mut()
            .zip(self.entities.iter().copied())
    }

    pub fn remove(&mut self, entity: Entity) {
        if let Some(&index) = self.map.get(&entity) {
            let last = self.components.len() - 1;
            let last_entity = self.entities[last];

            if entity == last_entity {
                self.components.remove(index);
                self.entities.remove(index);
            } else {
                self.components[index as usize] = self.components.remove(last);
                self.entities[index as usize] = self.entities.remove(last);
            }

            self.map.insert(last_entity, index);
            self.map.remove(&entity);
        }
    }
}
