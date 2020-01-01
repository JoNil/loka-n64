use crate::entity::Entity;
use alloc::vec::Vec;
use spin::{Mutex, MutexGuard, Once};

pub mod char_drawable;
pub mod movable;

static SYSTEMS: Once<Mutex<Systems>> = Once::new();

pub fn systems() -> MutexGuard<'static, Systems> {
    SYSTEMS.call_once(|| Mutex::new(Systems::new())).lock()
}

pub struct Systems {
    removers: Vec<fn(&Entity)>,
}

impl Systems {
    fn new() -> Systems {
        Systems {
            removers: Vec::new(),
        }
    }

    pub fn register_remover(&mut self, remover: fn(&Entity)) {
        self.removers.push(remover);
    }

    pub fn removers(&self) -> &[fn(&Entity)] {
        &self.removers
    }
}

#[macro_export]
macro_rules! impl_system {
    ($component_ident: ident) => {
        static SYSTEM: spin::Once<spin::RwLock<System>> = spin::Once::new();

        fn create() -> spin::RwLock<System> {
            let res = spin::RwLock::new(System::new());
            systems().register_remover(|e| lock_mut().remove(e));
            res
        }

        #[allow(dead_code)]
        pub fn lock() -> spin::RwLockReadGuard<'static, System> {
            SYSTEM.call_once(create).read()
        }

        #[allow(dead_code)]
        pub fn lock_mut() -> spin::RwLockWriteGuard<'static, System> {
            SYSTEM.call_once(create).write()
        }

        #[allow(dead_code)]
        pub fn add(component: $component_ident) {
            lock_mut().add(component);
        }

        #[allow(dead_code)]
        pub fn get_component(e: &Entity) -> Option<$component_ident> {
            SYSTEM.call_once(create).read().lookup(e).map(|c| *c)
        }

        #[allow(dead_code)]
        pub struct System {
            components: alloc::vec::Vec<$component_ident>,
            map: hashbrown::HashMap<Entity, usize>,
        }

        impl System {
            #[allow(dead_code)]
            fn new() -> System {
                System {
                    components: alloc::vec::Vec::new(),
                    map: hashbrown::HashMap::new(),
                }
            }

            #[allow(dead_code)]
            pub fn add(&mut self, component: $component_ident) {
                self.components.push(component);
                self.map.insert(component.entity, self.components.len() - 1);
            }

            #[allow(dead_code)]
            pub fn remove(&mut self, e: &Entity) {
                if let Some(&index) = self.map.get(e) {
                    let last = self.components.len() - 1;
                    let last_entity = self.components[last].entity;

                    self.components[index as usize] = self.components[last];
                    self.components.remove(last);

                    self.map.insert(last_entity, index);
                    self.map.remove(e);
                }
            }

            #[allow(dead_code)]
            pub fn lookup(&self, e: &Entity) -> Option<&$component_ident> {
                if let Some(&index) = self.map.get(e) {
                    return Some(&self.components[index]);
                }

                None
            }

            #[allow(dead_code)]
            pub fn lookup_mut(&mut self, e: &Entity) -> Option<&mut $component_ident> {
                if let Some(&mut index) = self.map.get_mut(e) {
                    return Some(&mut self.components[index]);
                }

                None
            }

            #[allow(dead_code)]
            pub fn components(&self) -> &[$component_ident] {
                &self.components
            }

            #[allow(dead_code)]
            pub fn components_mut(&mut self) -> &mut [$component_ident] {
                &mut self.components
            }
        }
    };
}
