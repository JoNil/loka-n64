use alloc::vec::Vec;
use crate::entity::Entity;
use spin::{Once, Mutex, MutexGuard};

mod movable;

pub use movable::movable;
pub use movable::MovableComponent;

static SYSTEMS: Once<Mutex<Systems>> = Once::new();

pub fn systems() -> MutexGuard<'static, Systems> {
    SYSTEMS.call_once(|| {
        Mutex::new(Systems::new())
    }).lock()
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

    pub fn removers(&self) -> impl Iterator<Item = &fn(&Entity)> {
        self.removers.iter()
    }
}