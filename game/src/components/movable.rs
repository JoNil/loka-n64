use alloc::vec::Vec;
use n64_math::Vec2;
use crate::entity::Entity;
use hashbrown::HashMap;
use spin::{Once, RwLock, RwLockReadGuard, RwLockWriteGuard};
use crate::components::systems;

static MOVABLE_SYSTEM: Once<RwLock<MovableSystem>> = Once::new();

fn create() -> RwLock<MovableSystem> {
    let res = RwLock::new(MovableSystem::new());
    systems().register_remover(|e| {
        lock_mut().remove(e)
    });
    res
}

pub fn lock() -> RwLockReadGuard<'static, MovableSystem> {
    MOVABLE_SYSTEM.call_once(create).read()
}

pub fn lock_mut() -> RwLockWriteGuard<'static, MovableSystem> {
    MOVABLE_SYSTEM.call_once(create).write()
}

pub fn add(component: MovableComponent) {
    lock_mut().add(component);
}

pub fn get_component(e: &Entity) -> Option<MovableComponent> {
    MOVABLE_SYSTEM.call_once(create)
    .read()
    .lookup(e)
    .map(|c| *c)
}

#[derive(Copy, Clone)]
pub struct MovableComponent {
    pub entity: Entity,
    pub pos: Vec2,
    pub speed: Vec2,
}

pub struct MovableSystem {
    components: Vec<MovableComponent>,
    map: HashMap<Entity, usize>,
}

impl MovableSystem {
    fn new() -> MovableSystem {
        MovableSystem {
            components: Vec::new(),
            map: HashMap::new(),
        }
    }

    pub fn simulate(&mut self, dt: f32) {
        for component in &mut self.components {
            component.pos += dt * component.speed;
        }
    }

    pub fn add(&mut self, component: MovableComponent) {
        self.components.push(component);
        self.map.insert(component.entity, self.components.len() - 1);
    }

    pub fn remove(&mut self, e: &Entity) {
        if let Some(&index) = self.map.get(e) {

            let last = self.components.len() - 1;
            let last_entity = self.components[last].entity;

            self.components[index as usize] = self.components[last];

            self.map.insert(last_entity, index);
            self.map.remove(e);
        }
    }

    pub fn lookup(&self, e: &Entity) -> Option<&MovableComponent> {
        if let Some(&index) = self.map.get(e) {
            return Some(&self.components[index]);
        }

        None
    }

    pub fn lookup_mut(&mut self, e: &Entity) -> Option<&mut MovableComponent> {
        if let Some(&mut index) = self.map.get_mut(e) {
            return Some(&mut self.components[index]);
        }

        None
    }
}