use super::{entity::Entity, storage::Storage};

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
