const COLLISION_MASK_PLAYER: u16 = 0b0000_0000_0000_0001;
const COLLISION_MASK_ENEMY: u16 = 0b0000_0000_0000_0010;

pub struct CollisionMask(u16);

impl CollisionMask {
    pub fn player() -> Self {
        Self(COLLISION_MASK_PLAYER)
    }

    pub fn enemy() -> Self {
        Self(COLLISION_MASK_ENEMY)
    }
}

pub struct Collider {
    pub mask: CollisionMask,
}
