use crate::components::Remover;
use alloc::collections::VecDeque;
use alloc::vec::Vec;
use core::num::Wrapping;
use core::ops::Deref;
use core::ops::Drop;
use spin::{Mutex, MutexGuard, Once};

const INDEX_BITS: u32 = 24;
const INDEX_MASK: u32 = (1 << INDEX_BITS) - 1;

const GENERATION_BITS: u32 = 8;
const GENERATION_MASK: u32 = (1 << GENERATION_BITS) - 1;

const MINIMUM_FREE_INDICES: u32 = 1024;

static ENTITY_REMOVE_LIST: Once<Mutex<Vec<Entity>>> = Once::new();

fn entity_remove_list() -> MutexGuard<'static, Vec<Entity>> {
    ENTITY_REMOVE_LIST
        .call_once(|| Mutex::new(Vec::with_capacity(128)))
        .lock()
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Entity {
    id: u32,
}

impl Entity {
    pub fn index(&self) -> u32 {
        self.id & INDEX_MASK
    }

    pub fn generation(&self) -> Wrapping<u8> {
        Wrapping(((self.id >> INDEX_BITS) & GENERATION_MASK) as u8)
    }
}

#[derive(Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct OwnedEntity {
    e: Entity,
}

impl OwnedEntity {
    fn new(index: u32, generation: Wrapping<u8>) -> OwnedEntity {
        assert!(index & !INDEX_MASK == 0);
        assert!((generation.0 as u32) & !GENERATION_MASK == 0);

        OwnedEntity {
            e: Entity {
                id: ((generation.0 as u32) << INDEX_BITS) | index,
            },
        }
    }
}

impl Drop for OwnedEntity {
    fn drop(&mut self) {
        entity_remove_list().push(**self);
    }
}

impl Deref for OwnedEntity {
    type Target = Entity;

    fn deref(&self) -> &Self::Target {
        &self.e
    }
}

pub struct EntitySystem {
    generation: Vec<Wrapping<u8>>,
    free_indices: VecDeque<u32>,
}

impl EntitySystem {
    pub fn new() -> EntitySystem {
        EntitySystem {
            generation: Vec::with_capacity(256),
            free_indices: VecDeque::with_capacity((2 * MINIMUM_FREE_INDICES) as usize),
        }
    }

    pub fn create(&mut self) -> OwnedEntity {
        let index = if self.free_indices.len() as u32 > MINIMUM_FREE_INDICES {
            self.free_indices.pop_front().unwrap()
        } else {
            self.generation.push(Wrapping(0));
            self.generation.len() as u32 - 1
        };

        assert!(index < (1 << INDEX_BITS));

        OwnedEntity::new(index, self.generation[index as usize])
    }

    pub fn alive(&self, entity: &Entity) -> bool {
        self.generation[entity.index() as usize] == entity.generation()
    }

    pub fn gc(&mut self, removers: &mut [&mut dyn Remover]) {
        let mut remove_list = entity_remove_list();

        for entity in remove_list.iter() {
            let index = entity.index();
            self.generation[index as usize] += Wrapping(1);
            self.free_indices.push_back(index);

            for remover in removers.iter_mut() {
                remover.remove(entity);
            }
        }

        remove_list.clear();
    }
}
