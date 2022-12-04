#![allow(dead_code)]

use super::{component::Component, entity::Entity};
use alloc::vec::Vec;

pub struct DenseStorage<T>
where
    T: Component + Default,
{
    components: Vec<T>,
    entities: Vec<Entity>,
}

impl<T> Storage<T>
where
    T: Component,
{
    pub fn new() -> Self {
        Self {
            components: Vec::with_capacity(256),
            entities: Vec::with_capacity(256),
        }
    }

    pub fn add(&mut self, entity: Entity, component: T) {
        let index = entity.index();
        self.components
            .resize(self.components.len().max(index as usize + 1), -1);
        self.entities
            .resize(self.entities.len().max(index as usize + 1), -1);

        self.components[index] = component;
        self.entities[index] = entity;
    }

    pub fn lookup(&self, entity: Entity) -> Option<&T> {
        let index = entity.index();

        if *self.entities.get(*index as usize)? == entity {
            return self.components.get(*index as usize);
        }

        None
    }

    pub fn lookup_mut(&mut self, entity: Entity) -> Option<&mut T> {
        let index = self.map.get(entity.index() as usize)?;

        if *self.entities.get(*index as usize)? == entity {
            return self.components.get_mut(*index as usize);
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

    pub fn components_and_entities_slice_mut(&mut self) -> (&[Entity], &mut [T]) {
        (self.entities.as_slice(), self.components.as_mut_slice())
    }

    pub fn remove(&mut self, entity: Entity) {
        let entity_index = entity.index();

        let index = index as usize;
        let last = self.components.len() - 1;

        if index == last {
            self.components.remove(index);
            self.entities.remove(index);
        }
    }
}

impl<T> Default for Storage<T>
where
    T: Component,
{
    fn default() -> Self {
        Self::new()
    }
}
