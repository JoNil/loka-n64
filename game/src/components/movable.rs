use alloc::vec::Vec;
use n64_math::Vec2;
use crate::entity::Entity;
use hashbrown::HashMap;
use spin::{Once, RwLock, RwLockReadGuard, RwLockWriteGuard};
use crate::components::systems;

static MOVABLE_SYSTEM: Once<RwLock<MovableSystem>> = Once::new();

pub fn movable() -> RwLockReadGuard<'static, MovableSystem> {
    MOVABLE_SYSTEM.call_once(|| {
        let res = RwLock::new(MovableSystem::new());
        systems().register_remover(|e| {
            movable_mut().remove(e)
        });
        res
    }).read()
}

pub fn movable_mut() -> RwLockWriteGuard<'static, MovableSystem> {
    MOVABLE_SYSTEM.call_once(|| {
        let res = RwLock::new(MovableSystem::new());
        systems().register_remover(|e| {
            movable_mut().remove(e)
        });
        res
    }).write()
}

#[derive(Copy, Clone)]
pub struct MovableComponent {
    entity: Entity,
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

    pub fn add(&mut self, e: &Entity, pos: Vec2, speed: Vec2) {
        self.components.push(MovableComponent {
            entity: *e,
            pos: pos,
            speed: speed,
        });

        self.map.insert(*e, self.components.len() - 1);
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