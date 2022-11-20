use super::{
    component_map::ComponentMap,
    entity::EntitySystem,
    query::{Query, WorldQuery},
};

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

    pub fn as_query<'world, Q>(&'world mut self) -> Query<'world, Q>
    where
        Q: WorldQuery,
    {
        Query::new(self)
    }

    pub fn housekeep(&mut self) {
        self.entities.housekeep(&mut self.components);
    }
}
