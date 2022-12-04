#![allow(dead_code)]

use super::{component::Component, entity::Entity};
use alloc::vec::Vec;

pub struct DenseStorage<T>
where
    T: Component + Clone + Default,
{
    components: Vec<T>,
    entities: Vec<Entity>,
}

impl<T> DenseStorage<T>
where
    T: Component + Clone + Default,
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
            .resize_with(self.components.len().max(index as usize + 1), T::default);
        self.entities
            .resize_with(self.entities.len().max(index as usize + 1), Entity::default);

        self.components[index as usize] = component;
        self.entities[index as usize] = entity;
    }

    pub fn lookup(&self, entity: Entity) -> Option<&T> {
        let index = entity.index();

        let entity_stored = self.entities.get(index as usize)?;

        if !entity_stored.valid() {
            return None;
        }

        if *entity_stored == entity {
            return None;
        }

        self.components.get(index as usize)
    }

    pub fn lookup_mut(&mut self, entity: Entity) -> Option<&mut T> {
        let index = entity.index();

        let entity_stored = self.entities.get(index as usize)?;

        if !entity_stored.valid() {
            return None;
        }

        if *entity_stored == entity {
            return None;
        }

        self.components.get_mut(index as usize)
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
        let index = entity.index() as usize;

        let last = self.components.len() - 1;

        if index == last {
            self.components.remove(index);
            self.entities.remove(index);
        }
    }
}

impl<T> Default for DenseStorage<T>
where
    T: Component + Clone + Default,
{
    fn default() -> Self {
        Self::new()
    }
}
