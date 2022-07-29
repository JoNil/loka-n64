use alloc::{boxed::Box, collections::VecDeque, vec::Vec};
use core::{mem, num::Wrapping};

use super::component_map::ComponentMap;

const INDEX_BITS: u32 = 24;
const INDEX_MASK: u32 = (1 << INDEX_BITS) - 1;

const GENERATION_BITS: u32 = 8;
const GENERATION_MASK: u32 = (1 << GENERATION_BITS) - 1;

const MINIMUM_FREE_INDICES: u32 = 1024;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Entity {
    id: u32,
}

impl Entity {
    fn new(index: u32, generation: Wrapping<u8>) -> Entity {
        assert!(index & !INDEX_MASK == 0);
        assert!((generation.0 as u32) & !GENERATION_MASK == 0);

        Entity {
            id: ((generation.0 as u32) << INDEX_BITS) | index,
        }
    }

    pub fn index(&self) -> u32 {
        self.id & INDEX_MASK
    }

    pub fn generation(&self) -> Wrapping<u8> {
        Wrapping(((self.id >> INDEX_BITS) & GENERATION_MASK) as u8)
    }
}

pub struct EntitySystem {
    generation: Vec<Wrapping<u8>>,
    free_indices: VecDeque<u32>,
    remove_list: Vec<Entity>,
    commands: Vec<Box<dyn FnOnce(&mut ComponentMap)>>,
}

impl EntitySystem {
    pub fn new() -> EntitySystem {
        EntitySystem {
            generation: Vec::with_capacity(256),
            free_indices: VecDeque::with_capacity((2 * MINIMUM_FREE_INDICES) as usize),
            remove_list: Vec::with_capacity(8),
            commands: Vec::new(),
        }
    }

    pub fn spawn(&mut self) -> EntityBuilder {
        let entity = self.create();
        n64::debugln!("Spawn entity {:?}", entity);
        EntityBuilder {
            entity,
            commands: &mut self.commands,
        }
    }

    pub fn despawn(&mut self, entity: Entity) {
        self.remove_list.push(entity);
    }

    fn create(&mut self) -> Entity {
        let index = if self.free_indices.len() as u32 > MINIMUM_FREE_INDICES {
            self.free_indices.pop_front().unwrap()
        } else {
            self.generation.push(Wrapping(0));
            self.generation.len() as u32 - 1
        };

        assert!(index < (1 << INDEX_BITS));

        Entity::new(index, self.generation[index as usize])
    }

    pub fn alive(&self, entity: Entity) -> bool {
        self.generation[entity.index() as usize] == entity.generation()
    }

    pub fn housekeep(&mut self, components: &mut ComponentMap) {
        {
            let commands = mem::take(&mut self.commands);
            for command in commands.into_iter() {
                command(components);
            }
        }

        {
            let removers = components.removers();
            let removers = removers.as_ref().borrow_mut();

            for entity in self.remove_list.iter() {
                if self.alive(*entity) {
                    n64::debugln!("despawn entity {:?}", entity);
                    let index = entity.index();
                    self.generation[index as usize] += Wrapping(1);
                    self.free_indices.push_back(index);

                    for remover in removers.iter() {
                        remover(components, *entity);
                    }
                }
            }

            self.remove_list.clear();
        }
    }
}

pub struct EntityBuilder<'a> {
    entity: Entity,
    commands: &'a mut Vec<Box<dyn FnOnce(&mut ComponentMap)>>,
}

impl<'a> EntityBuilder<'a> {
    pub fn add<T: 'static>(&'a mut self, component: T) -> &'a mut Self {
        let entity = self.entity;

        self.commands.push(Box::new(move |components| {
            components.get::<T>().add(entity, component);
        }));

        self
    }

    pub fn entity(&self) -> Entity {
        self.entity
    }
}
