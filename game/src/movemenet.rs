use alloc::vec::Vec;
use n64_math::Vec2;
use crate::entity::Entity;

struct MovementInstance {
    index: u32,
}

struct MovementComponent {
    pos: Vec2,
    speed: Vec2,
}

struct MovementSystem {
    components: Vec<MovementComponent>,
    

}

impl MovementSystem {
    fn new() -> MovementSystem {
        MovementSystem {
            components: Vec::new(),
        }
    }

    fn update(&mut self, dt: f32) {

        for component in &mut self.components {
            component.pos += dt * component.speed;
        }
    }
}