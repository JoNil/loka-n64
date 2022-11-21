#![allow(dead_code)]
#![allow(clippy::type_complexity)]

use super::{entity::Entity, storage::Storage};
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

    pub fn get_ptr<T: 'static>(&mut self) -> *mut Storage<T> {
        let key = TypeId::of::<T>();

        if !self.map.contains_key(&key) {
            self.map.insert(key, Box::new(Storage::<T>::new()));
            self.removers.as_ref().borrow_mut().push(|map, entity| {
                map.get::<(T,)>().remove(entity);
            });
        }

        let res: &mut Storage<T> = self
            .map
            .get_mut(&key)
            .and_then(|b| b.downcast_mut::<Storage<T>>())
            .unwrap_or_else(|| panic!("Could not find component: {}", type_name::<T>()));

        res as *mut Storage<T>
    }

    pub fn get<T: ComponentTuple>(&mut self) -> T::Item<'_> {
        T::get(self)
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
    T: 'static,
{
    type Item<'a> = &'a mut Storage<T>;

    fn get(component_map: &mut ComponentMap) -> Self::Item<'_> {
        let t = component_map.get_ptr::<T>();

        unsafe { &mut *(t as *mut Storage<T>) }
    }
}
unsafe impl<T1, T2> ComponentTuple for (T1, T2)
where
    T1: 'static,
    T2: 'static,
{
    type Item<'a> = (&'a mut Storage<T1>, &'a mut Storage<T2>);

    fn get(component_map: &mut ComponentMap) -> Self::Item<'_> {
        let t1 = component_map.get_ptr::<T1>();
        let t2 = component_map.get_ptr::<T2>();

        assert!(t1 as *const u8 != t2 as *const u8);

        unsafe {
            (
                &mut *(t1 as *mut Storage<T1>),
                &mut *(t2 as *mut Storage<T2>),
            )
        }
    }
}

unsafe impl<T1, T2, T3> ComponentTuple for (T1, T2, T3)
where
    T1: 'static,
    T2: 'static,
    T3: 'static,
{
    type Item<'a> = (
        &'a mut Storage<T1>,
        &'a mut Storage<T2>,
        &'a mut Storage<T3>,
    );

    fn get(component_map: &mut ComponentMap) -> Self::Item<'_> {
        let t1 = component_map.get_ptr::<T1>();
        let t2 = component_map.get_ptr::<T2>();
        let t3 = component_map.get_ptr::<T3>();

        assert!(t1 as *const u8 != t2 as *const u8);
        assert!(t1 as *const u8 != t3 as *const u8);
        assert!(t2 as *const u8 != t3 as *const u8);

        unsafe {
            (
                &mut *(t1 as *mut Storage<T1>),
                &mut *(t2 as *mut Storage<T2>),
                &mut *(t3 as *mut Storage<T3>),
            )
        }
    }
}

unsafe impl<T1, T2, T3, T4> ComponentTuple for (T1, T2, T3, T4)
where
    T1: 'static,
    T2: 'static,
    T3: 'static,
    T4: 'static,
{
    type Item<'a> = (
        &'a mut Storage<T1>,
        &'a mut Storage<T2>,
        &'a mut Storage<T3>,
        &'a mut Storage<T4>,
    );

    fn get(component_map: &mut ComponentMap) -> Self::Item<'_> {
        let t1 = component_map.get_ptr::<T1>();
        let t2 = component_map.get_ptr::<T2>();
        let t3 = component_map.get_ptr::<T3>();
        let t4 = component_map.get_ptr::<T4>();

        assert!(t1 as *const u8 != t2 as *const u8);
        assert!(t1 as *const u8 != t3 as *const u8);
        assert!(t1 as *const u8 != t4 as *const u8);
        assert!(t2 as *const u8 != t3 as *const u8);
        assert!(t2 as *const u8 != t4 as *const u8);
        assert!(t3 as *const u8 != t4 as *const u8);

        unsafe {
            (
                &mut *(t1 as *mut Storage<T1>),
                &mut *(t2 as *mut Storage<T2>),
                &mut *(t3 as *mut Storage<T3>),
                &mut *(t4 as *mut Storage<T4>),
            )
        }
    }
}

unsafe impl<T1, T2, T3, T4, T5> ComponentTuple for (T1, T2, T3, T4, T5)
where
    T1: 'static,
    T2: 'static,
    T3: 'static,
    T4: 'static,
    T5: 'static,
{
    type Item<'a> = (
        &'a mut Storage<T1>,
        &'a mut Storage<T2>,
        &'a mut Storage<T3>,
        &'a mut Storage<T4>,
        &'a mut Storage<T5>,
    );

    fn get(component_map: &mut ComponentMap) -> Self::Item<'_> {
        let t1 = component_map.get_ptr::<T1>();
        let t2 = component_map.get_ptr::<T2>();
        let t3 = component_map.get_ptr::<T3>();
        let t4 = component_map.get_ptr::<T4>();
        let t5 = component_map.get_ptr::<T5>();

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
                &mut *(t1 as *mut Storage<T1>),
                &mut *(t2 as *mut Storage<T2>),
                &mut *(t3 as *mut Storage<T3>),
                &mut *(t4 as *mut Storage<T4>),
                &mut *(t5 as *mut Storage<T5>),
            )
        }
    }
}

unsafe impl<T1, T2, T3, T4, T5, T6> ComponentTuple for (T1, T2, T3, T4, T5, T6)
where
    T1: 'static,
    T2: 'static,
    T3: 'static,
    T4: 'static,
    T5: 'static,
    T6: 'static,
{
    type Item<'a> = (
        &'a mut Storage<T1>,
        &'a mut Storage<T2>,
        &'a mut Storage<T3>,
        &'a mut Storage<T4>,
        &'a mut Storage<T5>,
        &'a mut Storage<T6>,
    );

    fn get(component_map: &mut ComponentMap) -> Self::Item<'_> {
        let t1 = component_map.get_ptr::<T1>();
        let t2 = component_map.get_ptr::<T2>();
        let t3 = component_map.get_ptr::<T3>();
        let t4 = component_map.get_ptr::<T4>();
        let t5 = component_map.get_ptr::<T5>();
        let t6 = component_map.get_ptr::<T6>();

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
                &mut *(t1 as *mut Storage<T1>),
                &mut *(t2 as *mut Storage<T2>),
                &mut *(t3 as *mut Storage<T3>),
                &mut *(t4 as *mut Storage<T4>),
                &mut *(t5 as *mut Storage<T5>),
                &mut *(t6 as *mut Storage<T6>),
            )
        }
    }
}
