const INDEX_BITS: u16 = 14;
const INDEX_MASK: u16 = (1 << INDEX_BITS) - 1;

const GENERATION_BITS: u16 = 2;
const GENERATION_MASK: u16 = (1 << GENERATION_BITS) - 1;

const MAX_ENTITYS: usize = 1 << INDEX_BITS;

pub struct Entity {
    id: u16,
}

impl Entity {

    fn new(index: u16, generation: u16) -> Entity {

        assert!(index & !INDEX_MASK == 0);
        assert!(generation & !GENERATION_MASK == 0);

        Entity {
            id: (generation << INDEX_BITS) | index,
        }
    }

    fn index(&self) -> u16 {
        self.id & INDEX_MASK
    }

    fn generation(&self) -> u8 {
        ((self.id >> INDEX_BITS) & GENERATION_MASK) as u8
    }
}

pub struct EntityManager {
    generation: [u8; MAX_ENTITYS],
    next_free: u16,
    next_empty: u16,
    entities_alive: u16,
    free_indices: [u16; MAX_ENTITYS],
}

impl EntityManager {

    pub fn new() -> EntityManager {
        let mut manager = EntityManager {
            generation: [0; MAX_ENTITYS],
            next_free: 0,
            next_empty: 0,
            entities_alive: 0,
            free_indices: [0; MAX_ENTITYS],
        };

        for (index, entity_index) in manager.free_indices.iter_mut().enumerate() {
            *entity_index = index as u16; 
        }

        manager
    }

    pub fn create_entity(&mut self) -> Option<Entity> {

        if self.entities_alive == MAX_ENTITYS as u16 {
            return None;
        }

        self.entities_alive += 1;

        let index = self.free_indices[self.next_free as usize];

        self.next_free = (self.next_free + 1) & INDEX_MASK;

        Some(Entity::new(index, self.generation[index as usize] as u16))
    }
   
    pub fn alive(&self, e: &Entity) -> bool {
        return self.generation[e.index() as usize] == e.generation();
    }
   
    pub fn destroy(&mut self, e: Entity) {
        let index = e.index();
        self.generation[index as usize] = (self.generation[index as usize] + 1) & (GENERATION_MASK as u8);

        self.free_indices[self.next_empty as usize] = index;
        self.next_empty = (self.next_empty + 1) & INDEX_MASK;
        self.entities_alive -= 1;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_max() {
        let mut manager = EntityManager::new();

        let a = manager.create_entity().unwrap();

        for i in 1..MAX_ENTITYS {
            let e = manager.create_entity().unwrap();
            assert_eq!(e.index(), i as u16);
        }

        let e = manager.create_entity();
        assert!(e.is_none());

        manager.destroy(a);

        let b = manager.create_entity().unwrap();

        assert!(b.index() == 0);
        assert!(b.generation() == 1);
    }
}