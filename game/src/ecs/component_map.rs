#![allow(dead_code)]

use super::{entity::Entity, storage::Storage};
use alloc::rc::Rc;
use core::{
    any::{type_name, Any, TypeId},
    cell::RefCell,
    mem,
};
use hashbrown::HashMap;
use n64_math::BuildFnvHasher;

pub struct ComponentMap {
    map: HashMap<TypeId, Box<dyn Any + 'static>, BuildFnvHasher>,
    removers: Vec<fn(&mut ComponentMap, Entity)>,
}

impl ComponentMap {
    pub fn new() -> Self {
        Self {
            map: HashMap::default(),
            removers: Vec::new(),
        }
    }

    pub fn removers(&self) -> &[fn(&mut ComponentMap, Entity)] {
        self.removers.as_slice()
    }

    fn fetch<T: 'static>(&self) -> *const Storage<T> {
        let key = TypeId::of::<T>();

        if !self.map.contains_key(&key) {
            self.map.insert(key, Box::new(Storage::<T>::new()));
            self.removers.push(|map, entity| {
                map.get::<T>().remove(entity);
            });
        }

        let res: &Box<Storage<T>> = self
            .map
            .get(&key)
            .and_then(|b| b.downcast_ref())
            .unwrap_or_else(|| panic!("Could not find component: {}", type_name::<T>()));

        res.as_ref() as *const Storage<T>
    }

    pub fn get<T: 'static>(&mut self) -> &mut Storage<T> {
        let t = self.fetch::<T>();

        unsafe { mem::transmute::<*mut Storage<T>, &mut Storage<T>>(t as *mut Storage<T>) }
    }

    pub fn get2<'a, T1: 'static, T2: 'static>(
        &'a mut self,
    ) -> (&'a mut Storage<T1>, &'a mut Storage<T2>) {
        let t1 = self.fetch::<T1>();
        let t2 = self.fetch::<T2>();

        assert!(t1 as *const u8 != t2 as *const u8);

        unsafe {
            (
                mem::transmute::<*mut Storage<T1>, &mut Storage<T1>>(t1 as *mut Storage<T1>),
                mem::transmute::<*mut Storage<T2>, &mut Storage<T2>>(t2 as *mut Storage<T2>),
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
        let t1 = self.fetch::<T1>();
        let t2 = self.fetch::<T2>();
        let t3 = self.fetch::<T3>();

        assert!(t1 as *const u8 != t2 as *const u8);
        assert!(t1 as *const u8 != t3 as *const u8);
        assert!(t2 as *const u8 != t3 as *const u8);

        unsafe {
            (
                mem::transmute::<*mut Storage<T1>, &mut Storage<T1>>(t1 as *mut Storage<T1>),
                mem::transmute::<*mut Storage<T2>, &mut Storage<T2>>(t2 as *mut Storage<T2>),
                mem::transmute::<*mut Storage<T3>, &mut Storage<T3>>(t2 as *mut Storage<T3>),
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
        let t1 = self.fetch::<T1>();
        let t2 = self.fetch::<T2>();
        let t3 = self.fetch::<T3>();
        let t4 = self.fetch::<T4>();

        assert!(t1 as *const u8 != t2 as *const u8);
        assert!(t1 as *const u8 != t3 as *const u8);
        assert!(t1 as *const u8 != t4 as *const u8);
        assert!(t2 as *const u8 != t3 as *const u8);
        assert!(t2 as *const u8 != t4 as *const u8);
        assert!(t3 as *const u8 != t4 as *const u8);

        unsafe {
            (
                mem::transmute::<*mut Storage<T1>, &mut Storage<T1>>(t1 as *mut Storage<T1>),
                mem::transmute::<*mut Storage<T2>, &mut Storage<T2>>(t2 as *mut Storage<T2>),
                mem::transmute::<*mut Storage<T3>, &mut Storage<T3>>(t2 as *mut Storage<T3>),
                mem::transmute::<*mut Storage<T4>, &mut Storage<T4>>(t2 as *mut Storage<T4>),
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
        let t1 = self.fetch::<T1>();
        let t2 = self.fetch::<T2>();
        let t3 = self.fetch::<T3>();
        let t4 = self.fetch::<T4>();
        let t5 = self.fetch::<T5>();

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
                mem::transmute::<*mut Storage<T1>, &mut Storage<T1>>(t1 as *mut Storage<T1>),
                mem::transmute::<*mut Storage<T2>, &mut Storage<T2>>(t2 as *mut Storage<T2>),
                mem::transmute::<*mut Storage<T3>, &mut Storage<T3>>(t2 as *mut Storage<T3>),
                mem::transmute::<*mut Storage<T4>, &mut Storage<T4>>(t2 as *mut Storage<T4>),
                mem::transmute::<*mut Storage<T5>, &mut Storage<T5>>(t2 as *mut Storage<T5>),
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
        let t1 = self.fetch::<T1>();
        let t2 = self.fetch::<T2>();
        let t3 = self.fetch::<T3>();
        let t4 = self.fetch::<T4>();
        let t5 = self.fetch::<T5>();
        let t6 = self.fetch::<T6>();

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
                mem::transmute::<*mut Storage<T1>, &mut Storage<T1>>(t1 as *mut Storage<T1>),
                mem::transmute::<*mut Storage<T2>, &mut Storage<T2>>(t2 as *mut Storage<T2>),
                mem::transmute::<*mut Storage<T3>, &mut Storage<T3>>(t2 as *mut Storage<T3>),
                mem::transmute::<*mut Storage<T4>, &mut Storage<T4>>(t2 as *mut Storage<T4>),
                mem::transmute::<*mut Storage<T5>, &mut Storage<T5>>(t2 as *mut Storage<T5>),
                mem::transmute::<*mut Storage<T6>, &mut Storage<T6>>(t2 as *mut Storage<T6>),
            )
        }
    }
}
