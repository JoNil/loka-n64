use super::{entity::Entity, storage::Storage, world::World};
use core::marker::PhantomData;

pub struct Query<'w, Q: WorldQuery> {
    world: &'w mut World,
    marker: PhantomData<Q>,
}

impl<'w, Q: WorldQuery> Query<'w, Q> {
    pub fn new(world: &'w mut World) -> Self {
        Self {
            world,
            marker: PhantomData,
        }
    }

    pub fn iter_mut(&mut self) -> WorldQueryIterator<'_, Q> {
        let storage = Q::get_storage(self.world);

        WorldQueryIterator { storage, index: 0 }
    }
}

pub trait WorldQuery {
    type Item;
    type StorageTuple<'w>;

    fn get_storage(world: &mut World) -> Self::StorageTuple<'_>;
}

impl<T1: ComponentRef, T2: ComponentRef> WorldQuery for (T1, T2)
where
    <T1 as ComponentRef>::Component: 'static,
    <T2 as ComponentRef>::Component: 'static,
{
    type Item = (T1, T2);
    type StorageTuple<'w> = (
        &'w mut Storage<T1::Component>,
        &'w mut Storage<T2::Component>,
    );

    fn get_storage(world: &mut World) -> Self::StorageTuple<'_> {
        world.components.get2::<T1::Component, T2::Component>()
    }
}

pub struct WorldQueryIterator<'w, Q>
where
    Q: WorldQuery,
{
    storage: Q::StorageTuple<'w>,
    index: i32,
}

impl<'w, Q> Iterator for WorldQueryIterator<'w, Q>
where
    Q: WorldQuery,
{
    type Item = (Entity, Q::Item);

    fn next(&mut self) -> Option<Self::Item> {
        /*if self.index > self.entities.len() as i32 {
            return None;
        }
        let entity = self.entities[self.index as usize];
        self.index += 1;
        Some((entity, (c1, c2)))*/
        unimplemented!()
    }
}

pub trait ComponentRef {
    type Component;
}

impl<T> ComponentRef for &T
where
    T: 'static,
{
    type Component = T;
}

impl<T> ComponentRef for &mut T
where
    T: 'static,
{
    type Component = T;
}
