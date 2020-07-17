use crate::{components::{Removers, box_drawable, health, movable, sprite_drawable, Remover}, entity::EntitySystem};

pub struct World {
    pub entities: EntitySystem,
    pub movables: movable::System,
    pub box_drawables: box_drawable::System,
    pub sprite_drawables: sprite_drawable::System,
    pub healths: health::System,
}

impl World {
    fn new() -> Self {
        Self {
            entities: EntitySystem::new(),
            movables: movable::System::new(),
            box_drawables: box_drawable::System::new(),
            sprite_drawables: sprite_drawable::System::new(),
            healths: health::System::new(),
        }
    }

    fn get_removers(&mut self) -> [&mut dyn Remover; 4] {
        [&mut self.movables, &mut self.box_drawables, &mut self.sprite_drawables, &mut self.healths]
    }
}