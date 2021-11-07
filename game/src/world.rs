use crate::{
    components::{box_drawable, bullet, enemy, health, missile, movable, player, sprite_drawable},
    entity::EntitySystem,
};

pub struct World {
    pub entity: EntitySystem,
    pub movable: movable::System,
    pub box_drawable: box_drawable::System,
    pub sprite_drawable: sprite_drawable::System,
    pub health: health::System,
    pub bullet: bullet::System,
    pub missile: missile::System,
    pub enemy: enemy::System,
    pub player: player::System,
}

impl World {
    pub fn new() -> Self {
        Self {
            entity: EntitySystem::new(),
            movable: movable::System::new(),
            box_drawable: box_drawable::System::new(),
            sprite_drawable: sprite_drawable::System::new(),
            health: health::System::new(),
            bullet: bullet::System::new(),
            missile: missile::System::new(),
            enemy: enemy::System::new(),
            player: player::System::new(),
        }
    }
}
