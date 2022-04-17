use crate::Vec2;

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct Aabb2 {
    upper_left: Vec2,
    lower_right: Vec2,
}

impl Aabb2 {
    #[inline]
    pub const fn new(upper_left: Vec2, lower_right: Vec2) -> Aabb2 {
        Aabb2 {
            upper_left,
            lower_right,
        }
    }

    #[inline]
    pub fn from_center_size(center: Vec2, size: Vec2) -> Aabb2 {
        let half_size = size / 2.0;

        Aabb2 {
            upper_left: center - half_size,
            lower_right: center + half_size,
        }
    }

    #[inline]
    pub fn top(&self) -> f32 {
        self.upper_left.y
    }

    #[inline]
    pub fn bottom(&self) -> f32 {
        self.lower_right.y
    }

    #[inline]
    pub fn left(&self) -> f32 {
        self.upper_left.x
    }

    #[inline]
    pub fn right(&self) -> f32 {
        self.lower_right.x
    }

    #[inline]
    pub fn center(&self) -> Vec2 {
        Vec2::new(
            (self.upper_left.x + self.lower_right.x) / 2.0,
            (self.upper_left.y + self.lower_right.y) / 2.0,
        )
    }

    #[inline]
    pub fn collides(&self, other: &Aabb2) -> bool {
        self.left() <= other.right()
            && self.right() >= other.left()
            && self.top() <= other.bottom()
            && self.bottom() >= other.top()
    }

    #[inline]
    pub fn outsize_distance(&self, other: &Aabb2) -> Vec2 {
        let mut res = Vec2::ZERO;

        if other.left() < self.left() {
            res.x = other.left() - self.left();
        } else if self.right() < other.right() {
            res.x = other.right() - self.right();
        }

        if other.top() < self.top() {
            res.y = other.top() - self.top();
        } else if self.bottom() < other.bottom() {
            res.y = other.bottom() - self.bottom();
        }

        res
    }
}
