use super::{entity::Entity, storage::Storage};
use alloc::vec::Vec;
use hashbrown::HashMap;

pub struct SparseStorage<T> {
    components: Vec<T>,
    entities: Vec<Entity>,
    map: HashMap<Entity, usize, n64_math::BuildFnvHasher>,
}

impl<T> SparseStorage<T> {
    pub fn new() -> Self {
        Self {
            components: Vec::with_capacity(256),
            entities: Vec::with_capacity(256),
            map: HashMap::with_capacity_and_hasher(256, n64_math::BuildFnvHasher),
        }
    }
}

impl<T> Default for SparseStorage<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Storage<T> for SparseStorage<T> {
    fn add(&mut self, entity: Entity, component: T) {
        self.components.push(component);
        self.entities.push(entity);
        self.map.insert(entity, self.components.len() - 1);
    }

    fn lookup(&self, entity: Entity) -> Option<&T> {
        if let Some(&index) = self.map.get(&entity) {
            return Some(&self.components[index]);
        }

        None
    }

    fn lookup_mut(&mut self, entity: Entity) -> Option<&mut T> {
        if let Some(&index) = self.map.get(&entity) {
            return Some(&mut self.components[index]);
        }

        None
    }

    fn components(&self) -> &[T] {
        &self.components
    }

    fn components_mut(&mut self) -> &mut [T] {
        &mut self.components
    }

    fn entities(&self) -> &[Entity] {
        &self.entities
    }

    fn components_and_entities_slice_mut(&mut self) -> (&[Entity], &mut [T]) {
        (self.entities.as_slice(), self.components.as_mut_slice())
    }

    fn remove(&mut self, entity: Entity) {
        if let Some(&index) = self.map.get(&entity) {
            let last = self.components.len() - 1;
            let last_entity = self.entities[last];

            if entity == last_entity {
                self.components.remove(index);
                self.entities.remove(index);
            } else {
                self.components[index] = self.components.remove(last);
                self.entities[index] = self.entities.remove(last);
            }

            self.map.insert(last_entity, index);
            self.map.remove(&entity);
        }
    }
}
