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

    fn get_unchecked<T: 'static>(&mut self) -> *const Storage<T> {
        let key = TypeId::of::<T>();

        if !self.map.contains_key(&key) {
            self.map.insert(key, Box::new(Storage::<T>::new()));
            self.removers.as_ref().borrow_mut().push(|map, entity| {
                map.get::<T>().remove(entity);
            });
        }

        let res: &Storage<T> = self
            .map
            .get(&key)
            .and_then(|b| b.downcast_ref::<Storage<T>>())
            .unwrap_or_else(|| panic!("Could not find component: {}", type_name::<T>()));

        res as *const Storage<T>
    }

    pub fn get<T: 'static>(&mut self) -> &mut Storage<T> {
        let t = self.get_unchecked::<T>();

        unsafe { &mut *(t as *mut Storage<T>) }
    }

    pub fn get2<'a, T1: 'static, T2: 'static>(
        &'a mut self,
    ) -> (&'a mut Storage<T1>, &'a mut Storage<T2>) {
        let t1 = self.get_unchecked::<T1>();
        let t2 = self.get_unchecked::<T2>();

        assert!(t1 as *const u8 != t2 as *const u8);

        unsafe {
            (
                &mut *(t1 as *mut Storage<T1>),
                &mut *(t2 as *mut Storage<T2>),
            )
        }
    }

    pub fn get3<'a, T1: 'static, T2: 'static, T3: 'static>(
        &'a mut self,
    ) -> (
        &'a mut Storage<T1>,
        &'a mut Storage<T2>,
        &'a mut Storage<T3>,
    ) {
        let t1 = self.get_unchecked::<T1>();
        let t2 = self.get_unchecked::<T2>();
        let t3 = self.get_unchecked::<T3>();

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

    pub fn get4<'a, T1: 'static, T2: 'static, T3: 'static, T4: 'static>(
        &'a mut self,
    ) -> (
        &'a mut Storage<T1>,
        &'a mut Storage<T2>,
        &'a mut Storage<T3>,
        &'a mut Storage<T4>,
    ) {
        let t1 = self.get_unchecked::<T1>();
        let t2 = self.get_unchecked::<T2>();
        let t3 = self.get_unchecked::<T3>();
        let t4 = self.get_unchecked::<T4>();

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

    pub fn get5<'a, T1: 'static, T2: 'static, T3: 'static, T4: 'static, T5: 'static>(
        &'a mut self,
    ) -> (
        &'a mut Storage<T1>,
        &'a mut Storage<T2>,
        &'a mut Storage<T3>,
        &'a mut Storage<T4>,
        &'a mut Storage<T5>,
    ) {
        let t1 = self.get_unchecked::<T1>();
        let t2 = self.get_unchecked::<T2>();
        let t3 = self.get_unchecked::<T3>();
        let t4 = self.get_unchecked::<T4>();
        let t5 = self.get_unchecked::<T5>();

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

    pub fn get6<
        'a,
        T1: 'static,
        T2: 'static,
        T3: 'static,
        T4: 'static,
        T5: 'static,
        T6: 'static,
    >(
        &'a mut self,
    ) -> (
        &'a mut Storage<T1>,
        &'a mut Storage<T2>,
        &'a mut Storage<T3>,
        &'a mut Storage<T4>,
        &'a mut Storage<T5>,
        &'a mut Storage<T6>,
    ) {
        let t1 = self.get_unchecked::<T1>();
        let t2 = self.get_unchecked::<T2>();
        let t3 = self.get_unchecked::<T3>();
        let t4 = self.get_unchecked::<T4>();
        let t5 = self.get_unchecked::<T5>();
        let t6 = self.get_unchecked::<T6>();

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
