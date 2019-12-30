use alloc::collections::VecDeque;
use alloc::vec::Vec;
use core::ops::Deref;
use core::ops::Drop;
use spin::{Once, Mutex, MutexGuard};
use crate::components::systems;

const INDEX_BITS: u32 = 24;
const INDEX_MASK: u32 = (1 << INDEX_BITS) - 1;

const GENERATION_BITS: u32 = 8;
const GENERATION_MASK: u32 = (1 << GENERATION_BITS) - 1;

const MINIMUM_FREE_INDICES: u32 = 1024;

static ENTITY_SYSTEM: Once<Mutex<EntitySystem>> = Once::new();

pub fn lock() -> MutexGuard<'static, EntitySystem> {
    ENTITY_SYSTEM.call_once(|| {
        Mutex::new(EntitySystem::new())
    }).lock()
}

pub fn create() -> OwnedEntity {
    lock().create()
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub struct Entity {
    id: u32,
}

impl Entity {

    fn index(&self) -> u32 {
        self.id & INDEX_MASK
    }

    fn generation(&self) -> u32 {
        (self.id >> INDEX_BITS) & GENERATION_MASK
    }
}

#[derive(Hash, Eq, PartialEq)]
pub struct OwnedEntity {
    e: Entity,
}

impl OwnedEntity {
    fn new(index: u32, generation: u32) -> OwnedEntity {

        assert!(index & !INDEX_MASK == 0);
        assert!(generation & !GENERATION_MASK == 0);

        OwnedEntity {
            e: Entity {
                id: (generation << INDEX_BITS) | index,
            }
        }
    }

    pub fn as_entity(&self) -> Entity {
        self.e
    }
}

impl Drop for OwnedEntity {
    fn drop(&mut self) {
        lock().destroy(self);
    }
}

impl Deref for OwnedEntity {
    type Target = Entity;

    fn deref(&self) -> &Self::Target {
        &self.e
    }
}

pub struct EntitySystem {
    generation: Vec<u8>,
    free_indices: VecDeque<u32>,
}

impl EntitySystem {

    pub fn new() -> EntitySystem {
        EntitySystem {
            generation: Vec::new(),
            free_indices: VecDeque::new(),
        }
    }

    pub fn create(&mut self) -> OwnedEntity {

        let index = if self.free_indices.len() as u32 > MINIMUM_FREE_INDICES {
            self.free_indices.pop_front().unwrap()
        } else {
            self.generation.push(0);
            self.generation.len() as u32 - 1
        };

        assert!(index < (1 << INDEX_BITS));

        OwnedEntity::new(index, self.generation[index as usize] as u32)
    }
   
    pub fn alive(&self, e: &Entity) -> bool {
        return self.generation[e.index() as usize] as u32 == e.generation();
    }
   
    fn destroy(&mut self, e: &Entity) {
        let index = e.index();
        self.generation[index as usize] += 1;
        self.free_indices.push_back(index);

        for remove in systems().removers() {
            remove(e);
        }
    }
}