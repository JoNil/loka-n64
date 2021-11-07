use crate::{
    components::{box_drawable, bullet, enemy, health, missile, movable, player, sprite_drawable},
    entity::EntitySystem,
};

pub struct World {
    pub entity: EntitySystem,
    pub movable: movable::Storage,
    pub box_drawable: box_drawable::Storage,
    pub sprite_drawable: sprite_drawable::Storage,
    pub health: health::Storage,
    pub bullet: bullet::Storage,
    pub missile: missile::Storage,
    pub enemy: enemy::Storage,
    pub player: player::Storage,
}

impl World {
    pub fn new() -> Self {
        Self {
            entity: EntitySystem::new(),
            movable: movable::Storage::new(),
            box_drawable: box_drawable::Storage::new(),
            sprite_drawable: sprite_drawable::Storage::new(),
            health: health::Storage::new(),
            bullet: bullet::Storage::new(),
            missile: missile::Storage::new(),
            enemy: enemy::Storage::new(),
            player: player::Storage::new(),
        }
    }
}
