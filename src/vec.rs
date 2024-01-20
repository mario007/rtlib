
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, Neg};
use std::convert::From;
use crate::math::difference_of_products;


/// A 3-dimensional vector.
///
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    /// The x component of the vector.
    pub x: f32,
    /// The y component of the vector.
    pub y: f32,
    /// The z component of the vector.
    pub z: f32,
}

impl Vec3 {
    /// Create new 3D vector
    #[inline(always)]
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {x, y, z}
    }
    /// Calculate dot product of two 3D vectors
    #[inline(always)]
    pub fn dot(self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    /// Calculate length of 3D vector
    #[inline(always)]
    pub fn length(self) -> f32 {
        self.dot(self).sqrt()
    }

    #[inline(always)]
    pub fn length_sqr(self) -> f32 {
        self.dot(self)
    }

    /// Normalize a 3D vector
    #[inline(always)]
    pub fn normalize(self) -> Self {
        let inv_len = self.length().recip();
        Self{x: self.x * inv_len, y: self.y * inv_len, z: self.z * inv_len}
    }

    /// Calculate cross product of two 3D vectors
    #[inline(always)]
    pub fn cross(self, rhs: Self) -> Self {
        Self{x: difference_of_products(self.y, rhs.z, self.z, rhs.y),
             y: difference_of_products(self.z, rhs.x, self.x, rhs.z),
             z: difference_of_products(self.x, rhs.y, self.y, rhs.x)}
    }
}


impl Add for Vec3 {
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: Self) -> Self {
        Self{x:self.x + rhs.x, y:self.y + rhs.y, z:self.z + rhs.z}
    }
}

impl AddAssign for Vec3 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self {
        Self{x:self.x - rhs.x, y:self.y - rhs.y, z:self.z - rhs.z}
    }
}

impl SubAssign for Vec3 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    #[inline(always)]
    fn mul(self, rhs: f32) -> Self {
        Self{x: self.x * rhs, y: self.y * rhs, z: self.z * rhs}
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    #[inline(always)]
    fn mul(self, rhs: Vec3) -> Self::Output {
        Self::Output{x: self * rhs.x, y: self * rhs.y, z: self * rhs.z}
    }
}

impl Mul for Vec3 {
    type Output = f32;

    #[inline(always)]
    fn mul(self, rhs: Vec3) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

}

impl Neg for Vec3 {
    type Output = Self;

    #[inline(always)]
    fn neg(self) -> Self {
        Self{x: -self.x, y: -self.y, z: -self.z}
    }
}

impl From<f32> for Vec3 {
    
    #[inline(always)]
    fn from(value: f32) -> Self {
        Self {x: value, y: value, z: value}
    }
}
