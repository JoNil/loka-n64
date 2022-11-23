use super::{component::Component, component_map::ComponentMap, entity::Entity, storage::Storage};

#[inline(always)]
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

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match unsafe { Q::get(&mut self.data as _, &mut self.index) } {
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

/// # Safety
///
/// This is probably not safe 😅
pub unsafe trait WorldQuery {
    type Item<'w>;
    type WorldQueryIteratorData<'w>;

    fn iterator_data(component_map: &mut ComponentMap) -> Self::WorldQueryIteratorData<'_>;

    /// # Safety
    ///
    /// WorldQueryIteratorData must be valid
    unsafe fn get<'w>(
        data: *mut Self::WorldQueryIteratorData<'w>,
        index: &mut i32,
    ) -> WorldQueryResult<Self::Item<'w>>;
}

unsafe impl<T1> WorldQuery for (T1,)
where
    T1: Component + 'static,
{
    type Item<'w> = (Entity, T1::RefInner<'w>);
    type WorldQueryIteratorData<'w> = (&'w [Entity], &'w mut [T1::Inner]);

    fn iterator_data(component_map: &mut ComponentMap) -> Self::WorldQueryIteratorData<'_> {
        let storage = component_map.get::<(T1::Inner,)>();

        let (entities, components) = storage.components_and_entities_slice_mut();

        assert!(entities.len() == components.len());

        (entities, components)
    }

    unsafe fn get<'w>(
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

    unsafe fn get<'w>(
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
    T1: Component + 'static,
    T2: Component + 'static,
    T3: Component + 'static,
{
    type Item<'w> = (Entity, T1::RefInner<'w>, T2::RefInner<'w>, T3::RefInner<'w>);
    type WorldQueryIteratorData<'w> = (
        &'w [Entity],
        &'w mut [T1::Inner],
        &'w mut Storage<T2::Inner>,
        &'w mut Storage<T3::Inner>,
    );

    fn iterator_data(component_map: &mut ComponentMap) -> Self::WorldQueryIteratorData<'_> {
        let storage = component_map.get::<(T1::Inner, T2::Inner, T3::Inner)>();

        let (entities, components) = storage.0.components_and_entities_slice_mut();

        assert!(entities.len() == components.len());

        (entities, components, storage.1, storage.2)
    }

    unsafe fn get<'w>(
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

        let Some(c3) = T3::get_from_storage(data.3, e) else {
            return WorldQueryResult::Filtered;
        };

        WorldQueryResult::Some((e, c1, c2, c3))
    }
}

unsafe impl<T1, T2, T3, T4> WorldQuery for (T1, T2, T3, T4)
where
    T1: Component + 'static,
    T2: Component + 'static,
    T3: Component + 'static,
    T4: Component + 'static,
{
    type Item<'w> = (
        Entity,
        T1::RefInner<'w>,
        T2::RefInner<'w>,
        T3::RefInner<'w>,
        T4::RefInner<'w>,
    );
    type WorldQueryIteratorData<'w> = (
        &'w [Entity],
        &'w mut [T1::Inner],
        &'w mut Storage<T2::Inner>,
        &'w mut Storage<T3::Inner>,
        &'w mut Storage<T4::Inner>,
    );

    fn iterator_data(component_map: &mut ComponentMap) -> Self::WorldQueryIteratorData<'_> {
        let storage = component_map.get::<(T1::Inner, T2::Inner, T3::Inner, T4::Inner)>();

        let (entities, components) = storage.0.components_and_entities_slice_mut();

        assert!(entities.len() == components.len());

        (entities, components, storage.1, storage.2, storage.3)
    }

    unsafe fn get<'w>(
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

        let Some(c3) = T3::get_from_storage(data.3, e) else {
            return WorldQueryResult::Filtered;
        };

        let Some(c4) = T4::get_from_storage(data.4, e) else {
            return WorldQueryResult::Filtered;
        };

        WorldQueryResult::Some((e, c1, c2, c3, c4))
    }
}

unsafe impl<T1, T2, T3, T4, T5> WorldQuery for (T1, T2, T3, T4, T5)
where
    T1: Component + 'static,
    T2: Component + 'static,
    T3: Component + 'static,
    T4: Component + 'static,
    T5: Component + 'static,
{
    type Item<'w> = (
        Entity,
        T1::RefInner<'w>,
        T2::RefInner<'w>,
        T3::RefInner<'w>,
        T4::RefInner<'w>,
        T5::RefInner<'w>,
    );
    type WorldQueryIteratorData<'w> = (
        &'w [Entity],
        &'w mut [T1::Inner],
        &'w mut Storage<T2::Inner>,
        &'w mut Storage<T3::Inner>,
        &'w mut Storage<T4::Inner>,
        &'w mut Storage<T5::Inner>,
    );

    fn iterator_data(component_map: &mut ComponentMap) -> Self::WorldQueryIteratorData<'_> {
        let storage =
            component_map.get::<(T1::Inner, T2::Inner, T3::Inner, T4::Inner, T5::Inner)>();

        let (entities, components) = storage.0.components_and_entities_slice_mut();

        assert!(entities.len() == components.len());

        (
            entities, components, storage.1, storage.2, storage.3, storage.4,
        )
    }

    unsafe fn get<'w>(
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

        let Some(c3) = T3::get_from_storage(data.3, e) else {
            return WorldQueryResult::Filtered;
        };

        let Some(c4) = T4::get_from_storage(data.4, e) else {
            return WorldQueryResult::Filtered;
        };

        let Some(c5) = T5::get_from_storage(data.5, e) else {
            return WorldQueryResult::Filtered;
        };

        WorldQueryResult::Some((e, c1, c2, c3, c4, c5))
    }
}

unsafe impl<T1, T2, T3, T4, T5, T6> WorldQuery for (T1, T2, T3, T4, T5, T6)
where
    T1: Component + 'static,
    T2: Component + 'static,
    T3: Component + 'static,
    T4: Component + 'static,
    T5: Component + 'static,
    T6: Component + 'static,
{
    type Item<'w> = (
        Entity,
        T1::RefInner<'w>,
        T2::RefInner<'w>,
        T3::RefInner<'w>,
        T4::RefInner<'w>,
        T5::RefInner<'w>,
        T6::RefInner<'w>,
    );
    type WorldQueryIteratorData<'w> = (
        &'w [Entity],
        &'w mut [T1::Inner],
        &'w mut Storage<T2::Inner>,
        &'w mut Storage<T3::Inner>,
        &'w mut Storage<T4::Inner>,
        &'w mut Storage<T5::Inner>,
        &'w mut Storage<T6::Inner>,
    );

    fn iterator_data(component_map: &mut ComponentMap) -> Self::WorldQueryIteratorData<'_> {
        let storage = component_map.get::<(
            T1::Inner,
            T2::Inner,
            T3::Inner,
            T4::Inner,
            T5::Inner,
            T6::Inner,
        )>();

        let (entities, components) = storage.0.components_and_entities_slice_mut();

        assert!(entities.len() == components.len());

        (
            entities, components, storage.1, storage.2, storage.3, storage.4, storage.5,
        )
    }

    unsafe fn get<'w>(
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

        let Some(c3) = T3::get_from_storage(data.3, e) else {
            return WorldQueryResult::Filtered;
        };

        let Some(c4) = T4::get_from_storage(data.4, e) else {
            return WorldQueryResult::Filtered;
        };

        let Some(c5) = T5::get_from_storage(data.5, e) else {
            return WorldQueryResult::Filtered;
        };

        let Some(c6) = T6::get_from_storage(data.6, e) else {
            return WorldQueryResult::Filtered;
        };

        WorldQueryResult::Some((e, c1, c2, c3, c4, c5, c6))
    }
}
