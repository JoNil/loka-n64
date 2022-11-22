use super::{component_map::ComponentMap, entity::Entity, storage::Storage};

pub fn query<Q>(component_map: &mut ComponentMap) -> Query<Q>
where
    Q: WorldQuery,
{
    let data = Q::iterator_data(component_map);

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

pub enum WorldQueryResult<T> {
    Some(T),
    End,
    Filtered,
}

pub trait Component {
    type Inner;
    type RefInner<'w>;

    fn convert(v: &mut Self::Inner) -> Self::RefInner<'_>;
    fn empty<'w>() -> Self::RefInner<'w>;

    fn get_from_storage(
        storage: &mut Storage<Self::Inner>,
        entity: Entity,
    ) -> Option<Self::RefInner<'_>> {
        storage.lookup_mut(entity).map(|v| Self::convert(v))
    }
}

impl<T> Component for Option<T>
where
    T: 'static,
{
    type Inner = T;
    type RefInner<'w> = Option<&'w mut T>;

    fn convert(v: &mut Self::Inner) -> Self::RefInner<'_> {
        Some(v)
    }

    fn empty<'w>() -> Self::RefInner<'w> {
        None
    }

    fn get_from_storage(
        storage: &mut Storage<Self::Inner>,
        entity: Entity,
    ) -> Option<Self::RefInner<'_>> {
        match storage.lookup_mut(entity) {
            Some(v) => Some(Self::convert(v)),
            None => Some(Self::empty()),
        }
    }
}

/// # Safety
///
/// This is probably not safe 😅
pub unsafe trait WorldQuery {
    type Item<'w>;
    type WorldQueryIteratorData<'w>;

    fn iterator_data(component_map: &mut ComponentMap) -> Self::WorldQueryIteratorData<'_>;
    fn get<'w>(
        data: *mut Self::WorldQueryIteratorData<'w>,
        index: &mut i32,
    ) -> WorldQueryResult<Self::Item<'w>>;
}

unsafe impl<T1> WorldQuery for (T1,)
where
    T1: 'static,
{
    type Item<'w> = (Entity, &'w mut T1);
    type WorldQueryIteratorData<'w> = (&'w [Entity], &'w mut [T1]);

    fn iterator_data(component_map: &mut ComponentMap) -> Self::WorldQueryIteratorData<'_> {
        let storage = component_map.get::<(T1,)>();
        let (entities, components) = storage.components_and_entities_slice_mut();

        assert!(entities.len() == components.len());

        (entities, components)
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

        WorldQueryResult::Some((e, c1))
    }
}

unsafe impl<T1, T2> WorldQuery for (T1, T2)
where
    T1: Component + 'static,
    T2: Component + 'static,
{
    type Item<'w> = (Entity, T1::RefInner<'w>, T2::RefInner<'w>);
    type WorldQueryIteratorData<'w> = (
        &'w [Entity],
        &'w mut [T1::Inner],
        &'w mut Storage<T2::Inner>,
    );

    fn iterator_data(component_map: &mut ComponentMap) -> Self::WorldQueryIteratorData<'_> {
        let storage = component_map.get::<(T1::Inner, T2::Inner)>();
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
        let c1 = T1::convert(unsafe { data.1.get_unchecked_mut(i) });

        *index += 1;

        let Some(c2) = T2::get_from_storage(data.2, e) else {
            return WorldQueryResult::Filtered;
        };

        WorldQueryResult::Some((e, c1, c2))
    }
}

unsafe impl<T1, T2, T3> WorldQuery for (T1, T2, T3)
where
    T1: 'static,
    T2: 'static,
    T3: 'static,
{
    type Item<'w> = (Entity, &'w mut T1, &'w mut T2, &'w mut T3);
    type WorldQueryIteratorData<'w> = (
        &'w [Entity],
        &'w mut [T1],
        &'w mut Storage<T2>,
        &'w mut Storage<T3>,
    );

    fn iterator_data(component_map: &mut ComponentMap) -> Self::WorldQueryIteratorData<'_> {
        let storage = component_map.get::<(T1, T2, T3)>();
        let (entities, components) = storage.0.components_and_entities_slice_mut();

        assert!(entities.len() == components.len());

        (entities, components, storage.1, storage.2)
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

        let Some(c3) = data.3.lookup_mut(e) else {
            return WorldQueryResult::Filtered;
        };

        WorldQueryResult::Some((e, c1, c2, c3))
    }
}

unsafe impl<T1, T2, T3, T4> WorldQuery for (T1, T2, T3, T4)
where
    T1: 'static,
    T2: 'static,
    T3: 'static,
    T4: 'static,
{
    type Item<'w> = (Entity, &'w mut T1, &'w mut T2, &'w mut T3, &'w mut T4);
    type WorldQueryIteratorData<'w> = (
        &'w [Entity],
        &'w mut [T1],
        &'w mut Storage<T2>,
        &'w mut Storage<T3>,
        &'w mut Storage<T4>,
    );

    fn iterator_data(component_map: &mut ComponentMap) -> Self::WorldQueryIteratorData<'_> {
        let storage = component_map.get::<(T1, T2, T3, T4)>();
        let (entities, components) = storage.0.components_and_entities_slice_mut();

        assert!(entities.len() == components.len());

        (entities, components, storage.1, storage.2, storage.3)
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

        let Some(c3) = data.3.lookup_mut(e) else {
            return WorldQueryResult::Filtered;
        };

        let Some(c4) = data.4.lookup_mut(e) else {
            return WorldQueryResult::Filtered;
        };

        WorldQueryResult::Some((e, c1, c2, c3, c4))
    }
}

unsafe impl<T1, T2, T3, T4, T5> WorldQuery for (T1, T2, T3, T4, T5)
where
    T1: 'static,
    T2: 'static,
    T3: 'static,
    T4: 'static,
    T5: 'static,
{
    type Item<'w> = (
        Entity,
        &'w mut T1,
        &'w mut T2,
        &'w mut T3,
        &'w mut T4,
        &'w mut T5,
    );
    type WorldQueryIteratorData<'w> = (
        &'w [Entity],
        &'w mut [T1],
        &'w mut Storage<T2>,
        &'w mut Storage<T3>,
        &'w mut Storage<T4>,
        &'w mut Storage<T5>,
    );

    fn iterator_data(component_map: &mut ComponentMap) -> Self::WorldQueryIteratorData<'_> {
        let storage = component_map.get::<(T1, T2, T3, T4, T5)>();
        let (entities, components) = storage.0.components_and_entities_slice_mut();

        assert!(entities.len() == components.len());

        (
            entities, components, storage.1, storage.2, storage.3, storage.4,
        )
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

        let Some(c3) = data.3.lookup_mut(e) else {
            return WorldQueryResult::Filtered;
        };

        let Some(c4) = data.4.lookup_mut(e) else {
            return WorldQueryResult::Filtered;
        };

        let Some(c5) = data.5.lookup_mut(e) else {
            return WorldQueryResult::Filtered;
        };

        WorldQueryResult::Some((e, c1, c2, c3, c4, c5))
    }
}

unsafe impl<T1, T2, T3, T4, T5, T6> WorldQuery for (T1, T2, T3, T4, T5, T6)
where
    T1: 'static,
    T2: 'static,
    T3: 'static,
    T4: 'static,
    T5: 'static,
    T6: 'static,
{
    type Item<'w> = (
        Entity,
        &'w mut T1,
        &'w mut T2,
        &'w mut T3,
        &'w mut T4,
        &'w mut T5,
        &'w mut T6,
    );
    type WorldQueryIteratorData<'w> = (
        &'w [Entity],
        &'w mut [T1],
        &'w mut Storage<T2>,
        &'w mut Storage<T3>,
        &'w mut Storage<T4>,
        &'w mut Storage<T5>,
        &'w mut Storage<T6>,
    );

    fn iterator_data(component_map: &mut ComponentMap) -> Self::WorldQueryIteratorData<'_> {
        let storage = component_map.get::<(T1, T2, T3, T4, T5, T6)>();
        let (entities, components) = storage.0.components_and_entities_slice_mut();

        assert!(entities.len() == components.len());

        (
            entities, components, storage.1, storage.2, storage.3, storage.4, storage.5,
        )
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

        let Some(c3) = data.3.lookup_mut(e) else {
            return WorldQueryResult::Filtered;
        };

        let Some(c4) = data.4.lookup_mut(e) else {
            return WorldQueryResult::Filtered;
        };

        let Some(c5) = data.5.lookup_mut(e) else {
            return WorldQueryResult::Filtered;
        };

        let Some(c6) = data.6.lookup_mut(e) else {
            return WorldQueryResult::Filtered;
        };

        WorldQueryResult::Some((e, c1, c2, c3, c4, c5, c6))
    }
}
