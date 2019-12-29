use alloc::vec::Vec;
use n64_math::Vec2;
use crate::entity::Entity;
use hashbrown::HashMap;

struct MovableInstance {
    index: u32,
}

struct MovableComponent {
    pos: Vec2,
    speed: Vec2,
}

struct MovableSystem {
    components: Vec<MovableComponent>,
    map: HashMap<Entity, u32>,
}

impl MovableSystem {
    fn new() -> MovableSystem {
        MovableSystem {
            components: Vec::new(),
            map: HashMap::new(),
        }
    }

    fn update(&mut self, dt: f32) {

        for component in &mut self.components {
            component.pos += dt * component.speed;
        }
    }
}