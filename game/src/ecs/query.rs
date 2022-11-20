use core::marker::PhantomData;

use super::{storage::Storage, world::World};

pub struct Query<'w, Q: WorldQuery> {
    world: &'w World,
    marker: PhantomData<Q>,
}

impl<'w, Q: WorldQuery> Query<'w, Q> {
    pub fn new(world: &'w mut World) -> Self {
        Self {
            world,
            marker: PhantomData,
        }
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = Q::Item> {
        let storage = Q::get_storage(&mut self.world);
    }
}

pub trait WorldQuery {
    type Item;
    type Storage<'w>;

    fn get_storage<'w>(world: &'w mut World) -> Self::Storage<'w>;
}

impl<T1: WorldRef, T2: WorldRef> WorldQuery for (T1, T2)
where
    for<'w> <T1 as WorldRef>::T: 'w,
    for<'w> <T2 as WorldRef>::T: 'w,
{
    type Item = (T1, T2);
    type Storage<'w> = (&'w mut Storage<T1::T>, &'w mut Storage<T2::T>);

    fn get_storage<'w>(world: &'w mut World) -> Self::Storage<'w> {
        world.components.get2::<T1::T, T2::T>()
    }
}

trait WorldRef {
    type T;
}

impl<T> WorldRef for &T {
    type T = T;
}
impl<T> WorldRef for &mut T {
    type T = T;
}
