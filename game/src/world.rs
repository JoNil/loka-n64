use crate::{
    component_storage::Storage,
    components::{box_drawable, bullet, enemy, health, missile, movable, player, sprite_drawable},
    entity::{Entity, EntitySystem},
    type_map::TypeMap,
};

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

    /*pub fn lookup(&self, entity: Entity) -> Option<&T> {
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
    }*/
}
