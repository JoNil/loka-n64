use alloc::vec::Vec;
use n64_math::{Vec2, Color};
use crate::entity::Entity;
use hashbrown::HashMap;
use spin::{Once, Mutex, MutexGuard};
use n64::{graphics, ipl3font};
use crate::components::{systems, movable};

static CHAR_DRAWABLE_SYSTEM: Once<Mutex<CharDrawableSystem>> = Once::new();

pub fn char_drawable() -> MutexGuard<'static, CharDrawableSystem> {
    CHAR_DRAWABLE_SYSTEM.call_once(|| {
        let res = Mutex::new(CharDrawableSystem::new());
        systems().register_remover(|e| {
            char_drawable().remove(e)
        });
        res
    }).lock()
}

#[derive(Copy, Clone)]
pub struct CharDrawableComponent {
    entity: Entity,
    pub color: Color,
    pub chr: u8,
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

    pub fn draw(&mut self) {

        for component in &mut self.components {
            if let Some(movable) = movable().lookup(&component.entity) {
                let screen_x = (movable.pos.x() * (graphics::WIDTH as f32)) as i32 - ipl3font::GLYPH_WIDTH / 2;
                let screen_y =
                    (movable.pos.y() * (graphics::HEIGHT as f32)) as i32 + ipl3font::GLYPH_HEIGHT / 2;
    
                ipl3font::draw_char(screen_x, screen_y, component.color, component.chr);
            }
        }
    }

    pub fn add(&mut self, e: &Entity, color: Color, chr: char) {
        self.components.push(CharDrawableComponent {
            entity: *e,
            color: color,
            chr: chr as u8,
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