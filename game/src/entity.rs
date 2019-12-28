use alloc::vec::Vec;
use alloc::collections::VecDeque;

const INDEX_BITS: u32 = 24;
const INDEX_MASK: u32 = (1 << INDEX_BITS) - 1;

const GENERATION_BITS: u32 = 8;
const GENERATION_MASK: u32 = (1 << GENERATION_BITS) - 1;

const MINIMUM_FREE_INDICES: u32 = 1024;

#[derive(Hash)]
pub struct Entity {
    id: u32,
}

impl Entity {

    fn new(index: u32, generation: u32) -> Entity {

        assert!(index & !INDEX_MASK == 0);
        assert!(generation & !GENERATION_MASK == 0);

        Entity {
            id: (generation << INDEX_BITS) | index,
        }
    }

    fn index(&self) -> u32 {
        self.id & INDEX_MASK
    }

    fn generation(&self) -> u32 {
        (self.id >> INDEX_BITS) & GENERATION_MASK
    }
}

pub struct EntityManager {
    generation: Vec<u8>,
    free_indices: VecDeque<u32>,
}

impl EntityManager {

    pub fn new() -> EntityManager {
        EntityManager {
            generation: Vec::new(),
            free_indices: VecDeque::new(),
        }
    }

    pub fn create_entity(&mut self) -> Entity {

        let index = if self.free_indices.len() as u32 > MINIMUM_FREE_INDICES {
            self.free_indices.pop_front().unwrap()
        } else {
            self.generation.push(0);
            self.generation.len() as u32 - 1
        };

        assert!(index < (1 << INDEX_BITS));

        Entity::new(index, self.generation[index as usize] as u32)
    }
   
    pub fn alive(&self, e: &Entity) -> bool {
        return self.generation[e.index() as usize] as u32 == e.generation();
    }
   
    pub fn destroy(&mut self, e: Entity) {
        let index = e.index();
        self.generation[index as usize] += 1;
        self.free_indices.push_back(index);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let mut manager = EntityManager::new();

        for i in 0..1024 {
            let e = manager.create_entity();
            assert_eq!(e.index(), i);
        }
    }
}