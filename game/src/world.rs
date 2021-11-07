use crate::{
    component_storage::Storage,
    components::{box_drawable, bullet, enemy, health, missile, movable, player, sprite_drawable},
    entity::{Entity, EntitySystem},
    type_map::TypeMap,
};
use core::any::type_name;

pub struct World {
    pub entities: EntitySystem,
    components: TypeMap,
    pub movable: movable::Storage,
    pub box_drawable: box_drawable::Storage,
    pub sprite_drawable: sprite_drawable::Storage,
    pub health: health::Storage,
    pub bullet: bullet::Storage,
    pub missile: missile::Storage,
    pub enemy: enemy::Storage,
    pub player: player::Storage,
}

impl World {
    pub fn new() -> Self {
        Self {
            entities: EntitySystem::new(),
            components: TypeMap::new(),
            movable: movable::Storage::new(),
            box_drawable: box_drawable::Storage::new(),
            sprite_drawable: sprite_drawable::Storage::new(),
            health: health::Storage::new(),
            bullet: bullet::Storage::new(),
            missile: missile::Storage::new(),
            enemy: enemy::Storage::new(),
            player: player::Storage::new(),
        }
    }

    pub fn add<T: 'static>(&mut self, entity: Entity, component: T) {
        let entry = self
            .components
            .entry::<Storage<T>>()
            .or_insert_with(|| Storage::<T>::new());
        entry.add(entity, component);
    }

    pub fn lookup<T: 'static>(&self, entity: Entity) -> Option<&T> {
        let entry = self
            .components
            .get::<Storage<T>>()
            .unwrap_or_else(|| panic!("Could not find component: {}", type_name::<T>()));
        entry.lookup(entity)
    }

    pub fn lookup_mut<T: 'static>(&mut self, entity: Entity) -> Option<&mut T> {
        let entry = self
            .components
            .get_mut::<Storage<T>>()
            .unwrap_or_else(|| panic!("Could not find component: {}", type_name::<T>()));
        entry.lookup_mut(entity)
    }

    pub fn components<T: 'static>(&self) -> &[T] {
        let entry = self
            .components
            .get::<Storage<T>>()
            .unwrap_or_else(|| panic!("Could not find component: {}", type_name::<T>()));
        entry.components()
    }

    pub fn components_mut<T: 'static>(&mut self) -> &mut [T] {
        let entry = self
            .components
            .get_mut::<Storage<T>>()
            .unwrap_or_else(|| panic!("Could not find component: {}", type_name::<T>()));
        entry.components_mut()
    }

    pub fn entities<T: 'static>(&self) -> &[Entity] {
        let entry = self
            .components
            .get::<Storage<T>>()
            .unwrap_or_else(|| panic!("Could not find component: {}", type_name::<T>()));
        entry.entities()
    }

    pub fn components_and_entities<T: 'static>(&self) -> impl Iterator<Item = (&T, Entity)> {
        let entry = self
            .components
            .get::<Storage<T>>()
            .unwrap_or_else(|| panic!("Could not find component: {}", type_name::<T>()));
        entry.components_and_entities()
    }

    pub fn components_and_entities_mut<T: 'static>(
        &mut self,
    ) -> impl Iterator<Item = (&mut T, Entity)> {
        let entry = self
            .components
            .get_mut::<Storage<T>>()
            .unwrap_or_else(|| panic!("Could not find component: {}", type_name::<T>()));
        entry.components_and_entities_mut()
    }
}
