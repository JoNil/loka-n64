use super::{entity::Entity, storage::Storage, world::World};

pub fn query<Q>(world: &mut World) -> Query<Q>
where
    Q: WorldQuery,
{
    let data = Q::iterator_data(world);

    Query { data, index: 0 }
}

pub struct Query<'w, Q>
where
    Q: WorldQuery,
{
    data: Q::WorldQueryIteratorData<'w>,
    index: i32,
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
    type Item<'w> = (Entity, &'w mut T1, &'w mut T2);
    type WorldQueryIteratorData<'w> = (&'w [Entity], &'w mut [T1], &'w mut Storage<T2>);

    fn iterator_data(world: &mut World) -> Self::WorldQueryIteratorData<'_> {
        let storage = world.components.get2::<T1, T2>();
        let (entities, components) = storage.0.components_and_entities_slice_mut();

        assert!(entities.len() == components.len());

        (entities, components, storage.1)
    }

    fn get<'w>(
        data: *mut Self::WorldQueryIteratorData<'w>,
        index: &mut i32,
    ) -> WorldQueryResult<Self::Item<'w>> {
        let data = unsafe { &mut *data };
        let i = *index as usize;

        if i >= data.0.len() {
            return WorldQueryResult::End;
        }

        let e = unsafe { *data.0.get_unchecked(i) };
        let c1 = unsafe { data.1.get_unchecked_mut(i) };

        *index += 1;

        let Some(c2) = data.2.lookup_mut(e) else {
            return WorldQueryResult::Filtered;
        };

        WorldQueryResult::Some((e, c1, c2))
    }
}

pub enum WorldQueryResult<T> {
    Some(T),
    End,
    Filtered,
}
