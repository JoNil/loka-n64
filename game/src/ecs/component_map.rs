#![allow(dead_code)]
#![allow(clippy::type_complexity)]

use super::{component::Component, entity::Entity, storage::Storage};
use alloc::{boxed::Box, rc::Rc, vec::Vec};
use core::{
    any::{type_name, Any, TypeId},
    cell::RefCell,
};
use hashbrown::HashMap;
use n64_math::BuildFnvHasher;

pub struct ComponentMap {
    map: HashMap<TypeId, Box<dyn Any + 'static>, BuildFnvHasher>,
    removers: Rc<RefCell<Vec<fn(&mut ComponentMap, Entity)>>>,
}

impl ComponentMap {
    pub fn new() -> Self {
        Self {
            map: HashMap::default(),
            removers: Rc::new(RefCell::new(Vec::new())),
        }
    }

    pub fn removers(&self) -> Rc<RefCell<Vec<fn(&mut ComponentMap, Entity)>>> {
        self.removers.clone()
    }

    pub fn get<T: ComponentTuple>(&mut self) -> T::Item<'_> {
        T::get(self)
    }

    fn get_ptr<T: Component + 'static>(&mut self) -> *mut T::Storage {
        let key = TypeId::of::<T>();

        if !self.map.contains_key(&key) {
            self.map.insert(key, Box::<T::Storage>::default());
            self.removers.as_ref().borrow_mut().push(|map, entity| {
                map.get::<(T,)>().remove(entity);
            });
        }

        let res: &mut T::Storage = self
            .map
            .get_mut(&key)
            .and_then(|b| b.downcast_mut::<T::Storage>())
            .unwrap_or_else(|| panic!("Could not find component: {}", type_name::<T>()));

        res as *mut T::Storage
    }
}

impl Default for ComponentMap {
    fn default() -> Self {
        Self::new()
    }
}

/// # Safety
///
/// This is probably not safe 😅
pub unsafe trait ComponentTuple {
    type Item<'a>;

    fn get(component_map: &mut ComponentMap) -> Self::Item<'_>;
}

unsafe impl<T> ComponentTuple for (T,)
where
    T: Component + 'static,
{
    type Item<'a> = &'a mut T::Storage;

    fn get(component_map: &mut ComponentMap) -> Self::Item<'_> {
        let t = component_map.get_ptr::<T::Inner>();

        unsafe { &mut *(t as *mut T::Storage) }
    }
}
unsafe impl<T1, T2> ComponentTuple for (T1, T2)
where
    T1: Component + 'static,
    T2: Component + 'static,
{
    type Item<'a> = (&'a mut T1::Storage, &'a mut T2::Storage);

    fn get(component_map: &mut ComponentMap) -> Self::Item<'_> {
        let t1 = component_map.get_ptr::<T1::Inner>();
        let t2 = component_map.get_ptr::<T2::Inner>();

        assert!(t1 as *const u8 != t2 as *const u8);

        unsafe {
            (
                &mut *(t1 as *mut T1::Storage),
                &mut *(t2 as *mut T2::Storage),
            )
        }
    }
}

unsafe impl<T1, T2, T3> ComponentTuple for (T1, T2, T3)
where
    T1: Component + 'static,
    T2: Component + 'static,
    T3: Component + 'static,
{
    type Item<'a> = (
        &'a mut T1::Storage,
        &'a mut T2::Storage,
        &'a mut T3::Storage,
    );

    fn get(component_map: &mut ComponentMap) -> Self::Item<'_> {
        let t1 = component_map.get_ptr::<T1::Inner>();
        let t2 = component_map.get_ptr::<T2::Inner>();
        let t3 = component_map.get_ptr::<T3::Inner>();

        assert!(t1 as *const u8 != t2 as *const u8);
        assert!(t1 as *const u8 != t3 as *const u8);
        assert!(t2 as *const u8 != t3 as *const u8);

        unsafe {
            (
                &mut *(t1 as *mut T1::Storage),
                &mut *(t2 as *mut T2::Storage),
                &mut *(t3 as *mut T3::Storage),
            )
        }
    }
}

unsafe impl<T1, T2, T3, T4> ComponentTuple for (T1, T2, T3, T4)
where
    T1: Component + 'static,
    T2: Component + 'static,
    T3: Component + 'static,
    T4: Component + 'static,
{
    type Item<'a> = (
        &'a mut T1::Storage,
        &'a mut T2::Storage,
        &'a mut T3::Storage,
        &'a mut T4::Storage,
    );

    fn get(component_map: &mut ComponentMap) -> Self::Item<'_> {
        let t1 = component_map.get_ptr::<T1::Inner>();
        let t2 = component_map.get_ptr::<T2::Inner>();
        let t3 = component_map.get_ptr::<T3::Inner>();
        let t4 = component_map.get_ptr::<T4::Inner>();

        assert!(t1 as *const u8 != t2 as *const u8);
        assert!(t1 as *const u8 != t3 as *const u8);
        assert!(t1 as *const u8 != t4 as *const u8);
        assert!(t2 as *const u8 != t3 as *const u8);
        assert!(t2 as *const u8 != t4 as *const u8);
        assert!(t3 as *const u8 != t4 as *const u8);

        unsafe {
            (
                &mut *(t1 as *mut T1::Storage),
                &mut *(t2 as *mut T2::Storage),
                &mut *(t3 as *mut T3::Storage),
                &mut *(t4 as *mut T4::Storage),
            )
        }
    }
}

unsafe impl<T1, T2, T3, T4, T5> ComponentTuple for (T1, T2, T3, T4, T5)
where
    T1: Component + 'static,
    T2: Component + 'static,
    T3: Component + 'static,
    T4: Component + 'static,
    T5: Component + 'static,
{
    type Item<'a> = (
        &'a mut T1::Storage,
        &'a mut T2::Storage,
        &'a mut T3::Storage,
        &'a mut T4::Storage,
        &'a mut T5::Storage,
    );

    fn get(component_map: &mut ComponentMap) -> Self::Item<'_> {
        let t1 = component_map.get_ptr::<T1::Inner>();
        let t2 = component_map.get_ptr::<T2::Inner>();
        let t3 = component_map.get_ptr::<T3::Inner>();
        let t4 = component_map.get_ptr::<T4::Inner>();
        let t5 = component_map.get_ptr::<T5::Inner>();

        assert!(t1 as *const u8 != t2 as *const u8);
        assert!(t1 as *const u8 != t3 as *const u8);
        assert!(t1 as *const u8 != t4 as *const u8);
        assert!(t1 as *const u8 != t5 as *const u8);
        assert!(t2 as *const u8 != t3 as *const u8);
        assert!(t2 as *const u8 != t4 as *const u8);
        assert!(t2 as *const u8 != t5 as *const u8);
        assert!(t3 as *const u8 != t4 as *const u8);
        assert!(t3 as *const u8 != t5 as *const u8);
        assert!(t4 as *const u8 != t5 as *const u8);

        unsafe {
            (
                &mut *(t1 as *mut T1::Storage),
                &mut *(t2 as *mut T2::Storage),
                &mut *(t3 as *mut T3::Storage),
                &mut *(t4 as *mut T4::Storage),
                &mut *(t5 as *mut T5::Storage),
            )
        }
    }
}

unsafe impl<T1, T2, T3, T4, T5, T6> ComponentTuple for (T1, T2, T3, T4, T5, T6)
where
    T1: Component + 'static,
    T2: Component + 'static,
    T3: Component + 'static,
    T4: Component + 'static,
    T5: Component + 'static,
    T6: Component + 'static,
{
    type Item<'a> = (
        &'a mut T1::Storage,
        &'a mut T2::Storage,
        &'a mut T3::Storage,
        &'a mut T4::Storage,
        &'a mut T5::Storage,
        &'a mut T6::Storage,
    );

    fn get(component_map: &mut ComponentMap) -> Self::Item<'_> {
        let t1 = component_map.get_ptr::<T1::Inner>();
        let t2 = component_map.get_ptr::<T2::Inner>();
        let t3 = component_map.get_ptr::<T3::Inner>();
        let t4 = component_map.get_ptr::<T4::Inner>();
        let t5 = component_map.get_ptr::<T5::Inner>();
        let t6 = component_map.get_ptr::<T6::Inner>();

        assert!(t1 as *const u8 != t2 as *const u8);
        assert!(t1 as *const u8 != t3 as *const u8);
        assert!(t1 as *const u8 != t4 as *const u8);
        assert!(t1 as *const u8 != t5 as *const u8);
        assert!(t1 as *const u8 != t6 as *const u8);
        assert!(t2 as *const u8 != t3 as *const u8);
        assert!(t2 as *const u8 != t4 as *const u8);
        assert!(t2 as *const u8 != t5 as *const u8);
        assert!(t2 as *const u8 != t6 as *const u8);
        assert!(t3 as *const u8 != t4 as *const u8);
        assert!(t3 as *const u8 != t5 as *const u8);
        assert!(t3 as *const u8 != t6 as *const u8);
        assert!(t4 as *const u8 != t5 as *const u8);
        assert!(t4 as *const u8 != t6 as *const u8);
        assert!(t5 as *const u8 != t6 as *const u8);

        unsafe {
            (
                &mut *(t1 as *mut T1::Storage),
                &mut *(t2 as *mut T2::Storage),
                &mut *(t3 as *mut T3::Storage),
                &mut *(t4 as *mut T4::Storage),
                &mut *(t5 as *mut T5::Storage),
                &mut *(t6 as *mut T6::Storage),
            )
        }
    }
}
