#![allow(dead_code)]

use alloc::rc::Rc;
use core::{
    any::{Any, TypeId},
    cell::RefCell,
};
use hashbrown::HashMap;
use n64_math::BuildFnvHasher;

#[derive(Debug, Default)]
pub struct TypeMap {
    map: HashMap<TypeId, Box<dyn Any + 'static>, BuildFnvHasher>,
}

impl TypeMap {
    pub fn new() -> Self {
        Self {
            map: HashMap::default(),
        }
    }

    pub fn insert<T: 'static>(&mut self, val: T) {
        self.map.insert(TypeId::of::<T>(), Box::new(val));
    }

    pub fn contains<T: 'static>(&self) -> bool {
        self.map.contains_key(&TypeId::of::<T>())
    }

    pub fn get<T: 'static>(&mut self) -> Option<&mut T> {
        self.map
            .get_mut(&TypeId::of::<T>())
            .and_then(|b| b.downcast_mut())
    }

    // This is safe as long as T1 and T2 are different types
    pub fn get2<'a, T1: 'static, T2: 'static>(&'a mut self) -> Option<(&'a mut T1, &'a mut T2)> {
        assert!(TypeId::of::<T1>() != TypeId::of::<T2>());

        let this = self as *mut Self;

        unsafe {
            let mut t1 = (*this)
                .map
                .get_mut(&TypeId::of::<T1>())
                .and_then(|b| b.downcast_mut());
            let mut t2 = (*this)
                .map
                .get_mut(&TypeId::of::<T2>())
                .and_then(|b| b.downcast_mut());

            if let (Some(t1), Some(t2)) = (t1, t2) {
                Some((t1, t2))
            } else {
                None
            }
        }
    }

    pub fn clear(&mut self) {
        self.map.clear();
    }
}
