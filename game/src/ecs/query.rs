use super::{entity::Entity, storage::Storage, world::World};
use core::ops::{Deref, DerefMut};

pub struct Query<'w, Q>
where
    Q: WorldQuery,
{
    data: Q::WorldQueryIteratorData<'w>,
    index: i32,
}

impl<'w, Q> Query<'w, Q>
where
    Q: WorldQuery,
{
    pub fn new(world: &'w mut World) -> Self {
        let data = Q::iterator_data(world);

        Self { data, index: 0 }
    }
}

impl<'w, Q> Iterator for Query<'w, Q>
where
    Q: WorldQuery,
{
    type Item = Q::Item<'w>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match Q::get(&mut self.data as _, &mut self.index) {
                WorldQueryResult::Some(val) => return Some(val),
                WorldQueryResult::End => return None,
                WorldQueryResult::Filtered => continue,
            }
        }
    }
}

/// # Safety
///
/// This is probably not safe 😅
pub unsafe trait WorldQuery {
    type Item<'w>;
    type WorldQueryIteratorData<'w>;

    fn iterator_data(world: &mut World) -> Self::WorldQueryIteratorData<'_>;
    fn get<'w>(
        data: *mut Self::WorldQueryIteratorData<'w>,
        index: &mut i32,
    ) -> WorldQueryResult<Self::Item<'w>>;
}

unsafe impl<T1, T2> WorldQuery for (T1, T2)
where
    T1: 'static,
    T2: 'static,
{
    type Item<'w> = (Entity, Mut<'w, T1>, Mut<'w, T2>);
    type WorldQueryIteratorData<'w> = (&'w [Entity], &'w mut [T1], &'w mut Storage<T2>);

    fn iterator_data(world: &mut World) -> Self::WorldQueryIteratorData<'_> {
        let storage = world.components.get2::<T1, T2>();
        let (entities, components) = storage.0.components_and_entities_slice_mut();
        (entities, components, storage.1)
    }

    fn get<'w>(
        data: *mut Self::WorldQueryIteratorData<'w>,
        index: &mut i32,
    ) -> WorldQueryResult<Self::Item<'w>> {
        let data = unsafe { &mut *data };

        if *index >= data.0.len() as i32 {
            return WorldQueryResult::End;
        }

        let e = data.0[*index as usize];
        let c1 = &mut data.1[*index as usize];

        *index += 1;

        let Some(c2) = data.2.lookup_mut(e) else {
            return WorldQueryResult::Filtered;
        };

        WorldQueryResult::Some((e, Mut::new(c1), Mut::new(c2)))
    }
}

pub enum WorldQueryResult<T> {
    Some(T),
    End,
    Filtered,
}

pub struct Mut<'a, T> {
    item: &'a mut T,
}

impl<'a, T> Mut<'a, T> {
    fn new(item: &'a mut T) -> Self {
        Self { item }
    }
}

impl<'a, T> Deref for Mut<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.item
    }
}

impl<'a, T> DerefMut for Mut<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.item
    }
}
