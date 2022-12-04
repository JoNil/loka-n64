use super::entity::Entity;

pub trait Storage<T> {
    fn add(&mut self, entity: Entity, component: T);
    fn lookup(&self, entity: Entity) -> Option<&T>;
    fn lookup_mut(&mut self, entity: Entity) -> Option<&mut T>;
    fn components(&self) -> &[T];
    fn components_mut(&mut self) -> &mut [T];
    fn entities(&self) -> &[Entity];
    fn components_and_entities_slice_mut(&mut self) -> (&[Entity], &mut [T]);
    fn remove(&mut self, entity: Entity);
}
