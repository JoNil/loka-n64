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
    map: HashMap<TypeId, Rc<dyn Any + 'static>, BuildFnvHasher>,
}

impl TypeMap {
    pub fn new() -> Self {
        Self {
            map: HashMap::default(),
        }
    }

    pub fn insert<T: 'static>(&mut self, val: T) {
        self.map
            .insert(TypeId::of::<T>(), Rc::new(RefCell::new(val)));
    }

    pub fn contains<T: 'static>(&self) -> bool {
        self.map.get(&TypeId::of::<T>()).is_some()
    }

    pub fn get<T: 'static>(&self) -> Option<Rc<RefCell<T>>> {
        self.map
            .get(&TypeId::of::<T>())
            .cloned()
            .and_then(|rc| rc.downcast().ok())
    }

    pub fn clear(&mut self) {
        self.map.clear();
    }
}
