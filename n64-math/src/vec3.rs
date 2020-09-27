use super::Vec2;
use core::{
    fmt,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

const ZERO: Vec3 = Vec3::new(0.0, 0.0, 0.0);
const ONE: Vec3 = Vec3::new(1.0, 1.0, 1.0);
const X_AXIS: Vec3 = Vec3::new(1.0, 0.0, 0.0);
const Y_AXIS: Vec3 = Vec3::new(0.0, 1.0, 0.0);
const Z_AXIS: Vec3 = Vec3::new(0.0, 0.0, 1.0);

/// A 3-dimensional vector without SIMD support.
#[derive(Clone, Copy, PartialEq, PartialOrd, Debug, Default)]
#[repr(C)]
pub struct Vec3(pub f32, pub f32, pub f32);

impl Vec3 {
    /// Creates a new `Vec3`.
    #[inline]
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self(x, y, z)
    }

    /// Creates a new `Vec3` with all elements set to `0.0`.
    #[inline]
    pub const fn zero() -> Self {
        ZERO
    }

    /// Creates a new `Vec3` with all elements set to `1.0`.
    #[inline]
    pub const fn one() -> Self {
        ONE
    }

    /// Creates a new `Vec3` with values `[x: 1.0, y: 0.0, z: 0.0]`.
    #[inline]
    pub const fn unit_x() -> Self {
        X_AXIS
    }

    /// Creates a new `Vec3` with values `[x: 0.0, y: 1.0, z: 0.0]`.
    #[inline]
    pub const fn unit_y() -> Self {
        Y_AXIS
    }

    /// Creates a new `Vec3` with values `[x: 0.0, y: 0.0, z: 1.0]`.
    #[inline]
    pub const fn unit_z() -> Self {
        Z_AXIS
    }

    /// Creates a new `Vec3` with all elements set to `v`.
    #[inline]
    pub fn splat(v: f32) -> Self {
        Self(v, v, v)
    }

    /// Creates a `Vec2` from the first three elements of `self`,
    /// removing `z`.
    #[inline]
    pub fn truncate(self) -> Vec2 {
        Vec2::new(self.0, self.1)
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

    /// Returns element `z`.
    #[inline]
    pub fn z(self) -> f32 {
        self.2
    }

    /// Returns a mutable reference to element `x`.
    #[inline]
    pub fn x_mut(&mut self) -> &mut f32 {
        &mut self.0
    }

    /// Returns a mutable reference to element `y`.
    #[inline]
    pub fn y_mut(&mut self) -> &mut f32 {
        &mut self.1
    }

    /// Returns a mutable reference to element `z`.
    #[inline]
    pub fn z_mut(&mut self) -> &mut f32 {
        &mut self.2
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

    /// Sets element `z`.
    #[inline]
    pub fn set_z(&mut self, z: f32) {
        self.2 = z;
    }

    /// Returns a `Vec3` with all elements set to the value of element `x`.
    #[inline]
    #[allow(dead_code)]
    pub(crate) fn dup_x(self) -> Self {
        Self(self.0, self.0, self.0)
    }

    /// Returns a `Vec3` with all elements set to the value of element `y`.
    #[inline]
    #[allow(dead_code)]
    pub(crate) fn dup_y(self) -> Self {
        Self(self.1, self.1, self.1)
    }

    /// Returns a `Vec3` with all elements set to the value of element `z`.
    #[inline]
    #[allow(dead_code)]
    pub(crate) fn dup_z(self) -> Self {
        Self(self.2, self.2, self.2)
    }

    /// Computes the dot product of `self` and `other`.
    #[inline]
    pub fn dot(self, other: Self) -> f32 {
        (self.0 * other.0) + (self.1 * other.1) + (self.2 * other.2)
    }

    /// Returns Vec3 dot in all lanes of Vec3
    #[inline]
    #[allow(dead_code)]
    pub(crate) fn dot_as_vec3(self, other: Self) -> Self {
        let dot = self.dot(other);
        Vec3::new(dot, dot, dot)
    }

    /// Computes the cross product of `self` and `other`.
    #[inline]
    pub fn cross(self, other: Self) -> Self {
        Self(
            self.1 * other.2 - other.1 * self.2,
            self.2 * other.0 - other.2 * self.0,
            self.0 * other.1 - other.0 * self.1,
        )
    }

    /// Computes the squared length of `self`.
    ///
    /// This is generally faster than `Vec3::length()` as it avoids a square
    /// root operation.
    #[inline]
    pub fn length_squared(self) -> f32 {
        self.dot(self)
    }

    /// Returns the vertical minimum of `self` and `other`.
    ///
    /// In other words, this computes
    /// `[x: min(x1, x2), y: min(y1, y2), z: min(z1, z2)]`,
    /// taking the minimum of each element individually.
    #[inline]
    pub fn min(self, other: Self) -> Self {
        Self(
            self.0.min(other.0),
            self.1.min(other.1),
            self.2.min(other.2),
        )
    }

    /// Returns the vertical maximum of `self` and `other`.
    ///
    /// In other words, this computes
    /// `[x: max(x1, x2), y: max(y1, y2), z: max(z1, z2)]`,
    /// taking the maximum of each element individually.
    #[inline]
    pub fn max(self, other: Self) -> Self {
        Self(
            self.0.max(other.0),
            self.1.max(other.1),
            self.2.max(other.2),
        )
    }

    /// Returns the horizontal minimum of `self`'s elements.
    ///
    /// In other words, this computes `min(x, y, z)`.
    #[inline]
    pub fn min_element(self) -> f32 {
        self.0.min(self.1.min(self.2))
    }

    /// Returns the horizontal maximum of `self`'s elements.
    ///
    /// In other words, this computes `max(x, y, z)`.
    #[inline]
    pub fn max_element(self) -> f32 {
        self.0.max(self.1.max(self.2))
    }

    /// Creates a new `Vec3` from the first three values in `slice`.
    ///
    /// # Panics
    ///
    /// Panics if `slice` is less than three elements long.
    #[inline]
    pub fn from_slice_unaligned(slice: &[f32]) -> Self {
        Self::new(slice[0], slice[1], slice[2])
    }

    /// Writes the elements of `self` to the first three elements in `slice`.
    ///
    /// # Panics
    ///
    /// Panics if `slice` is less than three elements long.
    #[inline]
    pub fn write_to_slice_unaligned(self, slice: &mut [f32]) {
        let a = self.as_ref();
        slice[0] = a[0];
        slice[1] = a[1];
        slice[2] = a[2];
    }

    /// Per element multiplication/addition of the three inputs: b + (self * a)
    #[inline]
    #[allow(dead_code)]
    pub(crate) fn mul_add(self, a: Self, b: Self) -> Self {
        Self(
            (self.0 * a.0) + b.0,
            (self.1 * a.1) + b.1,
            (self.2 * a.2) + b.2,
        )
    }

    /// Computes the reciprocal `1.0/n` of each element, returning the
    /// results in a new `Vec3`.
    #[inline]
    pub fn recip(self) -> Self {
        Self::new(1.0 / self.0, 1.0 / self.1, 1.0 / self.2)
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
}

impl AsRef<[f32; 3]> for Vec3 {
    #[inline]
    fn as_ref(&self) -> &[f32; 3] {
        unsafe { &*(self as *const Vec3 as *const [f32; 3]) }
    }
}

impl AsMut<[f32; 3]> for Vec3 {
    #[inline]
    fn as_mut(&mut self) -> &mut [f32; 3] {
        unsafe { &mut *(self as *mut Vec3 as *mut [f32; 3]) }
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}, {}]", self.0, self.1, self.2)
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Self;
    #[inline]
    fn div(self, other: Self) -> Self {
        Self(self.0 / other.0, self.1 / other.1, self.2 / other.2)
    }
}

impl DivAssign<Vec3> for Vec3 {
    #[inline]
    fn div_assign(&mut self, other: Self) {
        self.0 /= other.0;
        self.1 /= other.1;
        self.2 /= other.2;
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;
    #[inline]
    fn div(self, other: f32) -> Self {
        Self(self.0 / other, self.1 / other, self.2 / other)
    }
}

impl DivAssign<f32> for Vec3 {
    #[inline]
    fn div_assign(&mut self, other: f32) {
        self.0 /= other;
        self.1 /= other;
        self.2 /= other;
    }
}

impl Div<Vec3> for f32 {
    type Output = Vec3;
    #[inline]
    fn div(self, other: Vec3) -> Vec3 {
        Vec3(self / other.0, self / other.1, self / other.2)
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Self;
    #[inline]
    fn mul(self, other: Self) -> Self {
        Self(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}

impl MulAssign<Vec3> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, other: Self) {
        self.0 *= other.0;
        self.1 *= other.1;
        self.2 *= other.2;
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;
    #[inline]
    fn mul(self, other: f32) -> Self {
        Self(self.0 * other, self.1 * other, self.2 * other)
    }
}

impl MulAssign<f32> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, other: f32) {
        self.0 *= other;
        self.1 *= other;
        self.2 *= other;
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;
    #[inline]
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3(self * other.0, self * other.1, self * other.2)
    }
}

impl Add for Vec3 {
    type Output = Self;
    #[inline]
    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl AddAssign for Vec3 {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
        self.1 += other.1;
        self.2 += other.2;
    }
}

impl Sub for Vec3 {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl SubAssign for Vec3 {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        self.0 -= other.0;
        self.1 -= other.1;
        self.2 -= other.2;
    }
}

impl Neg for Vec3 {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        Self(-self.0, -self.1, -self.2)
    }
}

impl From<(f32, f32, f32)> for Vec3 {
    #[inline]
    fn from(t: (f32, f32, f32)) -> Self {
        Self::new(t.0, t.1, t.2)
    }
}

impl From<Vec3> for (f32, f32, f32) {
    #[inline]
    fn from(v: Vec3) -> Self {
        (v.0, v.1, v.2)
    }
}

impl From<[f32; 3]> for Vec3 {
    #[inline]
    fn from(a: [f32; 3]) -> Self {
        Self::new(a[0], a[1], a[2])
    }
}

impl From<Vec3> for [f32; 3] {
    #[inline]
    fn from(v: Vec3) -> Self {
        [v.0, v.1, v.2]
    }
}
