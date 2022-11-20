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
        let data = Q::iterator_data(self.world);
        WorldQueryIterator { data, index: 0 }
    }
}

pub enum WorldQueryResult<T> {
    Some(T),
    End,
    Filtered,
}

pub trait WorldQuery {
    type Item;
    type WorldQueryIteratorData<'w>;

    fn iterator_data(world: &mut World) -> Self::WorldQueryIteratorData<'_>;
    fn get(data: &mut Self::WorldQueryIteratorData<'_>, index: i32)
        -> WorldQueryResult<Self::Item>;
}

impl<T1: ComponentRef, T2: ComponentRef> WorldQuery for (T1, T2)
where
    <T1 as ComponentRef>::Component: 'static,
    <T2 as ComponentRef>::Component: 'static,
{
    type Item = (Entity, T1, T2);
    type WorldQueryIteratorData<'w> = (
        &'w [Entity],
        &'w mut [T1::Component],
        &'w mut Storage<T2::Component>,
    );

    fn iterator_data(world: &mut World) -> Self::WorldQueryIteratorData<'_> {
        let storage = world.components.get2::<T1::Component, T2::Component>();
        let (entities, components) = storage.0.components_and_entities_slice_mut();
        (entities, components, storage.1)
    }

    fn get(
        data: &mut Self::WorldQueryIteratorData<'_>,
        index: i32,
    ) -> WorldQueryResult<Self::Item> {
        if index > data.0.len() as i32 {
            return WorldQueryResult::End;
        }

        WorldQueryResult::Filtered
    }
}

pub struct WorldQueryIterator<'w, Q>
where
    Q: WorldQuery,
{
    data: Q::WorldQueryIteratorData<'w>,
    index: i32,
}

impl<'w, Q> Iterator for WorldQueryIterator<'w, Q>
where
    Q: WorldQuery,
{
    type Item = Q::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let mut res = WorldQueryResult::Filtered;

        while matches!(res, WorldQueryResult::Filtered) {
            res = Q::get(&mut self.data, self.index);
            self.index += 1;
        }

        match res {
            WorldQueryResult::Some(item) => Some(item),
            WorldQueryResult::End => None,
            WorldQueryResult::Filtered => {
                panic!("Internal iterator error");
            }
        }
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
