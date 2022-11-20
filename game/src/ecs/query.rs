use core::marker::PhantomData;

use super::world::World;

pub struct Query<'world, Q: WorldQuery> {
    world: &'world World,
    marker: PhantomData<Q>,
}

impl<'world, Q: WorldQuery> Query<'world, Q> {
    pub fn new(world: &'world mut World) -> Self {
        Self {
            world,
            marker: PhantomData,
        }
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = Q::Item<'_>> {
        unreachable!()
    }
}

pub trait WorldQuery {
    type Item<'w>;
}

impl<T: 'static> WorldQuery for &T {
    type Item<'w> = &'w T;
}

impl<T: 'static> WorldQuery for &mut T {
    type Item<'w> = &'w mut T;
}

impl<T1: WorldQuery, T2: WorldQuery> WorldQuery for (T1, T2) {
    type Item<'w> = (T1, T2);
}
