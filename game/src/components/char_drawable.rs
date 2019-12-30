use alloc::vec::Vec;
use n64_math::Color;
use crate::entity::Entity;
use hashbrown::HashMap;
use spin::{Once, RwLock, RwLockReadGuard, RwLockWriteGuard};
use n64::{graphics, ipl3font};
use crate::components::{systems, movable};

static CHAR_DRAWABLE_SYSTEM: Once<RwLock<CharDrawableSystem>> = Once::new();

fn create() -> RwLock<CharDrawableSystem> {
    let res = RwLock::new(CharDrawableSystem::new());
        systems().register_remover(|e| {
            lock_mut().remove(e)
        });
        res
}

pub fn lock() -> RwLockReadGuard<'static, CharDrawableSystem> {
    CHAR_DRAWABLE_SYSTEM.call_once(create).read()
}

pub fn lock_mut() -> RwLockWriteGuard<'static, CharDrawableSystem> {
    CHAR_DRAWABLE_SYSTEM.call_once(create).write()
}

pub fn add(component: CharDrawableComponent) {
    lock_mut().add(component);
}

pub fn get_component(e: &Entity) -> Option<CharDrawableComponent> {
    CHAR_DRAWABLE_SYSTEM.call_once(create)
    .read()
    .lookup(e)
    .map(|c| *c)
}

#[derive(Copy, Clone)]
pub struct CharDrawableComponent {
    pub entity: Entity,
    pub color: Color,
    pub chr: char,
}

pub struct CharDrawableSystem {
    components: Vec<CharDrawableComponent>,
    map: HashMap<Entity, usize>,
}

impl CharDrawableSystem {
    fn new() -> CharDrawableSystem {
        CharDrawableSystem {
            components: Vec::new(),
            map: HashMap::new(),
        }
    }

    pub fn draw(&self) {

        for component in &self.components {
            if let Some(movable) = movable::lock().lookup(&component.entity) {
                let screen_x = (movable.pos.x() * (graphics::WIDTH as f32)) as i32 - ipl3font::GLYPH_WIDTH / 2;
                let screen_y =
                    (movable.pos.y() * (graphics::HEIGHT as f32)) as i32 + ipl3font::GLYPH_HEIGHT / 2;
    
                ipl3font::draw_char(screen_x, screen_y, component.color, component.chr as u8);
            }
        }
    }

    pub fn add(&mut self, component: CharDrawableComponent) {
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

    pub fn lookup(&self, e: &Entity) -> Option<&CharDrawableComponent> {
        if let Some(&index) = self.map.get(e) {
            return Some(&self.components[index]);
        }

        None
    }

    pub fn lookup_mut(&mut self, e: &Entity) -> Option<&mut CharDrawableComponent> {
        if let Some(&mut index) = self.map.get_mut(e) {
            return Some(&mut self.components[index]);
        }

        None
    }
}