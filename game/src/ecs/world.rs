use super::{component_map::ComponentMap, entity::EntitySystem};

pub struct World {
    pub entities: EntitySystem,
    pub components: ComponentMap,
}

impl World {
    pub fn new() -> Self {
        Self {
            entities: EntitySystem::new(),
            components: ComponentMap::new(),
        }
    }

    pub fn housekeep(&mut self) {
        self.entities.housekeep(&mut self.components);
    }
}
