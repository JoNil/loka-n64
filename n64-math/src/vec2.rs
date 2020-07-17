use core::{
    f32, fmt,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

/// A 2-dimensional vector.
#[derive(Clone, Copy, PartialEq, PartialOrd, Debug, Default)]
#[repr(C)]
pub struct Vec2(pub f32, pub f32);

impl Vec2 {
    /// Creates a new `Vec2`.
    #[inline]
    pub const fn new(x: f32, y: f32) -> Vec2 {
        Vec2(x, y)
    }

    /// Creates a new `Vec2` with all elements set to `0.0`.
    #[inline]
    pub fn zero() -> Vec2 {
        Vec2(0.0, 0.0)
    }

    /// Creates a new `Vec2` with all elements set to `1.0`.
    #[inline]
    pub fn one() -> Vec2 {
        Vec2(1.0, 1.0)
    }

    /// Creates a new `Vec2` with values `[x: 1.0, y: 0.0]`.
    #[inline]
    pub fn unit_x() -> Vec2 {
        Vec2(1.0, 0.0)
    }

    /// Creates a new `Vec2` with values `[x: 0.0, y: 1.0]`.
    #[inline]
    pub fn unit_y() -> Vec2 {
        Vec2(0.0, 1.0)
    }

    /// Creates a new `Vec2` with all elements set to `v`.
    #[inline]
    pub fn splat(v: f32) -> Vec2 {
        Vec2(v, v)
    }

    /// Returns element `x`.
    #[inline]
    pub fn x(self) -> f32 {
        self.0
    }

    /// Returns element `y`.
    #[inline]
    pub fn y(self) -> f32 {
        self.1
    }

    /// Sets element `x`.
    #[inline]
    pub fn set_x(&mut self, x: f32) {
        self.0 = x;
    }

    /// Sets element `y`.
    #[inline]
    pub fn set_y(&mut self, y: f32) {
        self.1 = y;
    }

    /// Computes the dot product of `self` and `other`.
    #[inline]
    pub fn dot(self, other: Vec2) -> f32 {
        (self.0 * other.0) + (self.1 * other.1)
    }

    /// Computes the length of `self`.
    #[inline]
    pub fn length(self) -> f32 {
        libm::sqrtf(self.dot(self))
    }

    /// Computes the squared length of `self`.
    ///
    /// This is generally faster than `Vec2::length()` as it avoids a square
    /// root operation.
    #[inline]
    pub fn length_squared(self) -> f32 {
        self.dot(self)
    }

    /// Computes `1.0 / Vec2::length()`.
    ///
    /// For valid results, `self` must _not_ be of length zero.
    #[inline]
    pub fn length_reciprocal(self) -> f32 {
        1.0 / self.length()
    }

    /// Returns `self` normalized to length 1.0.
    ///
    /// For valid results, `self` must _not_ be of length zero.
    #[inline]
    pub fn normalize(self) -> Vec2 {
        self * self.length_reciprocal()
    }

    /// Computes the reciprocal `1.0/n` of each element, returning the
    /// results in a new `Vec2`.
    #[inline]
    pub fn reciprocal(self) -> Self {
        Self::one() / self
    }

    /// Performs a linear interpolation between `self` and `other` based on
    /// the value `s`.
    ///
    /// When `s` is `0.0`, the result will be equal to `self`.  When `s`
    /// is `1.0`, the result will be equal to `other`.
    #[inline]
    pub fn lerp(self, other: Self, s: f32) -> Self {
        self + ((other - self) * s)
    }

    /// Returns the vertical minimum of `self` and `other`.
    ///
    /// In other words, this computes
    /// `[x: min(x1, x2), y: min(y1, y2)]`,
    /// taking the minimum of each element individually.
    #[inline]
    pub fn min(self, other: Vec2) -> Vec2 {
        Vec2(self.0.min(other.0), self.1.min(other.1))
    }

    /// Returns the vertical maximum of `self` and `other`.
    ///
    /// In other words, this computes
    /// `[x: max(x1, x2), y: max(y1, y2)]`,
    /// taking the maximum of each element individually.
    #[inline]
    pub fn max(self, other: Vec2) -> Vec2 {
        Vec2(self.0.max(other.0), self.1.max(other.1))
    }

    /// Returns the horizontal minimum of `self`'s elements.
    ///
    /// In other words, this computes `min(x, y)`.
    #[inline]
    pub fn min_element(self) -> f32 {
        self.0.min(self.1)
    }

    /// Returns the horizontal maximum of `self`'s elements.
    ///
    /// In other words, this computes `max(x, y)`.
    #[inline]
    pub fn max_element(self) -> f32 {
        self.0.max(self.1)
    }

    /// Creates a new `Vec2` from the first two values in `slice`.
    ///
    /// # Panics
    ///
    /// Panics if `slice` is less than two elements long.
    #[inline]
    pub fn from_slice_unaligned(slice: &[f32]) -> Self {
        Self(slice[0], slice[1])
    }

    /// Writes the elements of `self` to the first two elements in `slice`.
    ///
    /// # Panics
    ///
    /// Panics if `slice` is less than two elements long.
    #[inline]
    pub fn write_to_slice_unaligned(self, slice: &mut [f32]) {
        slice[0] = self.0;
        slice[1] = self.1;
    }

    /// Returns a new `Vec2` containing the absolute value of each element of the original
    /// `Vec2`.
    #[inline]
    pub fn abs(self) -> Self {
        Self(libm::fabsf(self.0), libm::fabsf(self.1))
    }
}

impl fmt::Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}]", self.0, self.1)
    }
}

impl Div<Vec2> for Vec2 {
    type Output = Self;
    #[inline]
    fn div(self, other: Vec2) -> Self {
        Self(self.0 / other.0, self.1 / other.1)
    }
}

impl DivAssign<Vec2> for Vec2 {
    #[inline]
    fn div_assign(&mut self, other: Vec2) {
        *self = Self(self.0 / other.0, self.1 / other.1)
    }
}

impl Div<f32> for Vec2 {
    type Output = Self;
    #[inline]
    fn div(self, other: f32) -> Self {
        Self(self.0 / other, self.1 / other)
    }
}

impl DivAssign<f32> for Vec2 {
    #[inline]
    fn div_assign(&mut self, other: f32) {
        *self = Self(self.0 / other, self.1 / other)
    }
}

impl Mul<Vec2> for Vec2 {
    type Output = Self;
    #[inline]
    fn mul(self, other: Vec2) -> Self {
        Self(self.0 * other.0, self.1 * other.1)
    }
}

impl MulAssign<Vec2> for Vec2 {
    #[inline]
    fn mul_assign(&mut self, other: Vec2) {
        *self = Self(self.0 * other.0, self.1 * other.1)
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;
    #[inline]
    fn mul(self, other: f32) -> Self {
        Self(self.0 * other, self.1 * other)
    }
}

impl MulAssign<f32> for Vec2 {
    #[inline]
    fn mul_assign(&mut self, other: f32) {
        *self = Self(self.0 * other, self.1 * other)
    }
}

impl Mul<Vec2> for f32 {
    type Output = Vec2;
    #[inline]
    fn mul(self, other: Vec2) -> Vec2 {
        Vec2(self * other.0, self * other.1)
    }
}

impl Add for Vec2 {
    type Output = Self;
    #[inline]
    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

impl AddAssign for Vec2 {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        *self = Self(self.0 + other.0, self.1 + other.1)
    }
}

impl Sub for Vec2 {
    type Output = Self;
    #[inline]
    fn sub(self, other: Vec2) -> Self {
        Self(self.0 - other.0, self.1 - other.1)
    }
}

impl SubAssign for Vec2 {
    #[inline]
    fn sub_assign(&mut self, other: Vec2) {
        *self = Self(self.0 - other.0, self.1 - other.1)
    }
}

impl Neg for Vec2 {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        Self(-self.0, -self.1)
    }
}

impl AsRef<[f32; 2]> for Vec2 {
    #[inline]
    fn as_ref(&self) -> &[f32; 2] {
        unsafe { &*(self as *const Vec2 as *const [f32; 2]) }
    }
}

impl AsMut<[f32; 2]> for Vec2 {
    #[inline]
    fn as_mut(&mut self) -> &mut [f32; 2] {
        unsafe { &mut *(self as *mut Vec2 as *mut [f32; 2]) }
    }
}

impl From<(f32, f32)> for Vec2 {
    #[inline]
    fn from(t: (f32, f32)) -> Self {
        Self(t.0, t.1)
    }
}

impl From<Vec2> for (f32, f32) {
    #[inline]
    fn from(v: Vec2) -> Self {
        (v.0, v.1)
    }
}

impl From<[f32; 2]> for Vec2 {
    #[inline]
    fn from(a: [f32; 2]) -> Self {
        Self(a[0], a[1])
    }
}

impl From<Vec2> for [f32; 2] {
    #[inline]
    fn from(v: Vec2) -> Self {
        [v.0, v.1]
    }
}
