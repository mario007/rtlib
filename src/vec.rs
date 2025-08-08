
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, Neg, Index};
use std::convert::From;
use crate::math::{difference_of_products, sum_of_products};
use std::f32;

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
   
    /// Calculate length of 3D vector
    #[inline(always)]
    pub fn length(self) -> f32 {
        (self*self).sqrt()
    }

    #[inline(always)]
    pub fn length_sqr(self) -> f32 {
        self*self
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

    /// Calculate the angle between two normalized 3D vectors
    pub fn angle_between(self, vec: Vec3) -> f32 {
        if self * vec < 0.0 {
            f32::consts::PI - 2.0 * safe_asin((self + vec).length() * 0.5)
        } else {
            2.0 * safe_asin((vec - self).length() * 0.5)
        }
    }

}

fn safe_asin(x: f32) -> f32 {
    x.clamp(-1.0, 1.0).asin()
}

impl Add for Vec3 {
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for Vec3 {

    #[inline(always)]
    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
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
    fn sub_assign(&mut self, rhs: Vec3) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
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
        #[cfg(target_feature = "fma")]
        {self.x.mul_add(rhs.x, sum_of_products(self.y, rhs.y, self.z, rhs.z))}

        #[cfg(not(target_feature = "fma"))]
        {self.x * rhs.x + self.y * rhs.y + self.z * rhs.z}
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

impl Index<usize> for Vec3 {
    type Output = f32;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,   
            2 => &self.z,
            _ => panic!("Invalid index for Vec3, expected 0, 1, or 2 and got {}", index),
        }
    }
}



/// A 3-dimensional point.
#[derive(Debug, Clone, Copy, PartialEq)]
/// Represents a point in two-dimensional space.
pub struct Point2 {
    /// The x coordinate of the point.
    pub x: f32,
    /// The y coordinate of the point.
    pub y: f32,
}

impl Point2 {
    /// Create a new 2D point.
    #[inline(always)]
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}



/// A 3-dimensional point.
#[derive(Debug, Clone, Copy, PartialEq)]
/// Represents a point in three-dimensional space.
pub struct Point3 {
    /// The x coordinate of the point.
    pub x: f32,
    /// The y coordinate of the point.
    pub y: f32,
    /// The z coordinate of the point.
    pub z: f32,
}

impl Point3 {
    /// Create a new 3D point.
    #[inline(always)]
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// Calculate the distance between two points.
    #[inline(always)]
    pub fn distance(self, other: Self) -> f32 {
        (self - other).length()
    }

    /// Calculate the distance squared between two points.
    #[inline(always)]
    pub fn distance_sqr(self, other: Self) -> f32 {
        (self - other).length_sqr()
    }

    #[inline(always)]
    pub fn min(self, other: Self) -> Self {
        Self{x: self.x.min(other.x), y: self.y.min(other.y), z: self.z.min(other.z)}
    }

    #[inline(always)]
    pub fn max(self, other: Self) -> Self {
        Self{x: self.x.max(other.x), y: self.y.max(other.y), z: self.z.max(other.z)}
    }
}

impl Add for Point3 {
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: Point3) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for Point3 {

    #[inline(always)]
    fn add_assign(&mut self, rhs: Point3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Point3 {
    type Output = Vec3;

    #[inline(always)]
    fn sub(self, rhs: Point3) -> Vec3 {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul<f32> for Point3 {
    type Output = Self;

    #[inline(always)]
    fn mul(self, rhs: f32) -> Self {
        Self{x: self.x * rhs, y: self.y * rhs, z: self.z * rhs}
    }
}

impl Mul<Point3> for f32 {
    type Output = Point3;

    #[inline(always)]
    fn mul(self, rhs: Point3) -> Self::Output {
        Self::Output{x: self * rhs.x, y: self * rhs.y, z: self * rhs.z}
    }
}

impl From<Point3> for Vec3 {
    #[inline(always)]
    fn from(value: Point3) -> Self {
        Vec3{x: value.x, y: value.y, z: value.z}
    }
}

impl From<Vec3> for Point3 {
    #[inline(always)]
    fn from(value: Vec3) -> Self {
        Self{x: value.x, y: value.y, z: value.z}
    }
}

impl Add<Vec3> for Point3 {
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: Vec3) -> Self {
        Self{x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z}
    }
}

impl Index<usize> for Point3 {
    type Output = f32;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Invalid index for Point3, expected 0, 1, or 2 and got {}", index),
        }
    }
}

/// A 3-dimensional normal vector.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Normal {
    /// The x component of the normal vector.
    pub x: f32,
    /// The y component of the normal vector.
    pub y: f32,
    /// The z component of the normal vector.
    pub z: f32,
}

impl Normal {
    /// Create a new 3D normal vector.
    #[inline(always)]
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// Calculate the dot product of two 3D normal vectors.
    #[inline(always)]
    pub fn dot(self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    /// Calculate the length of the 3D normal vector.
    #[inline(always)]
    pub fn length(self) -> f32 {
        self.dot(self).sqrt()
    }

    /// Calculate the squared length of the 3D normal vector.
    #[inline(always)]
    pub fn length_sqr(self) -> f32 {
        self.dot(self)
    }

    /// Normalize the 3D normal vector.
    #[inline(always)]
    pub fn normalize(self) -> Self {
        let inv_len = self.length().recip();
        Self {
            x: self.x * inv_len,
            y: self.y * inv_len,
            z: self.z * inv_len,
        }
    }

}

impl Add for Normal {
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for Normal {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Normal) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Normal {
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}


impl SubAssign for Normal {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Normal) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Mul<f32> for Normal {
    type Output = Self;

    #[inline(always)]
    fn mul(self, rhs: f32) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Normal> for f32 {
    type Output = Normal;

    #[inline(always)]
    fn mul(self, rhs: Normal) -> Self::Output {
        Self::Output {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl Mul for Normal {
    type Output = f32;

    #[inline(always)]
    fn mul(self, rhs: Normal) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl Neg for Normal {
    type Output = Self;

    #[inline(always)]
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl From<f32> for Normal {
    #[inline(always)]
    fn from(value: f32) -> Self {
        Self {
            x: value,
            y: value,
            z: value,
        }
    }
}

impl From<Vec3> for Normal {
    #[inline(always)]
    fn from(value: Vec3) -> Self {
        Self{x: value.x, y: value.y, z: value.z}
    }
}

impl Mul<Vec3> for Normal {
    type Output = f32;

    #[inline(always)]
    fn mul(self, rhs: Vec3) -> Self::Output {
        #[cfg(target_feature = "fma")]
        {self.x.mul_add(rhs.x, sum_of_products(self.y, rhs.y, self.z, rhs.z))}

        #[cfg(not(target_feature = "fma"))]
        {self.x * rhs.x + self.y * rhs.y + self.z * rhs.z}
    }
}

impl Mul<Normal> for Vec3 {
    type Output = f32;

    #[inline(always)]
    fn mul(self, rhs: Normal) -> Self::Output {
        rhs * self
    }
}

impl From<Normal> for Vec3 {
    #[inline(always)]
    fn from(value: Normal) -> Self {
        Vec3{x: value.x, y: value.y, z: value.z}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_add() {
        let a = Normal { x: 1.0, y: 2.0, z: 3.0 };
        let b = Normal { x: 4.0, y: 5.0, z: 6.0 };
        let result = a + b;
        assert_eq!(result.x, 5.0);
        assert_eq!(result.y, 7.0);
        assert_eq!(result.z, 9.0);
    }

    #[test]
    fn test_normal_add_assign() {
        let mut a = Normal { x: 1.0, y: 2.0, z: 3.0 };
        let b = Normal { x: 4.0, y: 5.0, z: 6.0 };
        a += b;
        assert_eq!(a.x, 5.0);
        assert_eq!(a.y, 7.0);
        assert_eq!(a.z, 9.0);
    }

    #[test]
    fn test_normal_sub() {
        let a = Normal { x: 4.0, y: 5.0, z: 6.0 };
        let b = Normal { x: 1.0, y: 2.0, z: 3.0 };
        let result = a - b;
        assert_eq!(result.x, 3.0);
        assert_eq!(result.y, 3.0);
        assert_eq!(result.z, 3.0);
    }

    #[test]
    fn test_normal_sub_assign() {
        let mut a = Normal { x: 4.0, y: 5.0, z: 6.0 };
        let b = Normal { x: 1.0, y: 2.0, z: 3.0 };
        a -= b;
        assert_eq!(a.x, 3.0);
        assert_eq!(a.y, 3.0);
        assert_eq!(a.z, 3.0);
    }

    #[test]
    fn test_normal_mul_scalar() {
        let a = Normal { x: 1.0, y: 2.0, z: 3.0 };
        let scalar = 2.0;
        let result = a * scalar;
        assert_eq!(result.x, 2.0);
        assert_eq!(result.y, 4.0);
        assert_eq!(result.z, 6.0);
    }

    #[test]
    fn test_scalar_mul_normal() {
        let scalar = 2.0;
        let b = Normal { x: 1.0, y: 2.0, z: 3.0 };
        let result = scalar * b;
        assert_eq!(result.x, 2.0);
        assert_eq!(result.y, 4.0);
        assert_eq!(result.z, 6.0);
    }

    #[test]
    fn test_normal_dot_product() {
        let a = Normal { x: 1.0, y: 2.0, z: 3.0 };
        let b = Normal { x: 4.0, y: 5.0, z: 6.0 };
        let result = a * b;
        assert_eq!(result, 32.0);
    }

    #[test]
    fn test_normal_neg() {
        let a = Normal { x: 1.0, y: 2.0, z: 3.0 };
        let result = -a;
        assert_eq!(result.x, -1.0);
        assert_eq!(result.y, -2.0);
        assert_eq!(result.z, -3.0);
    }

    #[test]
    fn test_normal_from() {
        let value = 2.0;
        let result: Normal = value.into();
        assert_eq!(result.x, 2.0);
        assert_eq!(result.y, 2.0);
        assert_eq!(result.z, 2.0);
    }

    #[test]
    fn test_vec3_add() {
        let a = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
        let b = Vec3 { x: 4.0, y: 5.0, z: 6.0 };
        let result = a + b;
        assert_eq!(result.x, 5.0);
        assert_eq!(result.y, 7.0);
        assert_eq!(result.z, 9.0);
    }

    #[test]
    fn test_vec3_add_assign() {
        let mut a = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
        let b = Vec3 { x: 4.0, y: 5.0, z: 6.0 };
        a += b;
        assert_eq!(a.x, 5.0);
        assert_eq!(a.y, 7.0);
        assert_eq!(a.z, 9.0);
    }

    #[test]
    fn test_vec3_sub() {
        let a = Vec3 { x: 4.0, y: 5.0, z: 6.0 };
        let b = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
        let result = a - b;
        assert_eq!(result.x, 3.0);
        assert_eq!(result.y, 3.0);
        assert_eq!(result.z, 3.0);
    }

    #[test]
    fn test_vec3_sub_assign() {
        let mut a = Vec3 { x: 4.0, y: 5.0, z: 6.0 };
        let b = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
        a -= b;
        assert_eq!(a.x, 3.0);
        assert_eq!(a.y, 3.0);
        assert_eq!(a.z, 3.0);
    }

    #[test]
    fn test_vec3_mul_scalar() {
        let a = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
        let scalar = 2.0;
        let result = a * scalar;
        assert_eq!(result.x, 2.0);
        assert_eq!(result.y, 4.0);
        assert_eq!(result.z, 6.0);
    }

    #[test]
    fn test_scalar_mul_vec3() {
        let scalar = 2.0;
        let b = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
        let result = scalar * b;
        assert_eq!(result.x, 2.0);
        assert_eq!(result.y, 4.0);
        assert_eq!(result.z, 6.0);
    }

    #[test]
    fn test_vec3_dot_product() {
        let a = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
        let b = Vec3 { x: 4.0, y: 5.0, z: 6.0 };
        let result = a * b;
        assert_eq!(result, 32.0);
    }

    #[test]
    fn test_vec3_neg() {
        let a = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
        let result = -a;
        assert_eq!(result.x, -1.0);
        assert_eq!(result.y, -2.0);
        assert_eq!(result.z, -3.0);
    }

    #[test]
    fn test_vec3_from() {
        let value = 2.0;
        let result: Vec3 = value.into();
        assert_eq!(result.x, 2.0);
        assert_eq!(result.y, 2.0);
        assert_eq!(result.z, 2.0);
    }

    #[test]
    fn test_vec3_angle_between() {
        let v1 = Vec3::new(1.0, 0.0, 0.0);
        let v2 = Vec3::new(0.0, 1.0, 0.0);
        let v3 = Vec3::new(-1.0, 0.0, 0.0);
        let v4 = Vec3::new(1.0, 1.0, 0.0).normalize();

        assert!((v1.angle_between(v2) - std::f32::consts::FRAC_PI_2).abs() < 1e-6);
        assert_eq!(v1.angle_between(v3), std::f32::consts::PI);
        assert!((v1.angle_between(v4) - std::f32::consts::FRAC_PI_4).abs() < 1e-6);
    }

    #[test]
    fn test_vec3_index() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        
        assert_eq!(v[0], 1.0);
        assert_eq!(v[1], 2.0);
        assert_eq!(v[2], 3.0);
    }

    #[test]
    #[should_panic(expected = "Invalid index for Vec3, expected 0, 1, or 2 and got 3")]
    fn test_vec3_index_out_of_bounds() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        let _ = v[3]; // This should panic
    }

    #[test]
    fn test_point3_new() {
        let point = Point3::new(1.0, 2.0, 3.0);
        assert_eq!(point.x, 1.0);
        assert_eq!(point.y, 2.0);
        assert_eq!(point.z, 3.0);
    }

    #[test]
    fn test_point3_distance() {
        let point1 = Point3::new(1.0, 2.0, 3.0);
        let point2 = Point3::new(4.0, 5.0, 6.0);
        let distance = point1.distance(point2);
        assert_eq!(distance, 5.196152);
    }

    #[test]
    fn test_point3_distance_sqr() {
        let point1 = Point3::new(1.0, 2.0, 3.0);
        let point2 = Point3::new(4.0, 5.0, 6.0);
        let distance_sqr = point1.distance_sqr(point2);
        assert_eq!(distance_sqr, 27.0);
    }

    #[test]
    fn test_point3_add() {
        let point1 = Point3::new(1.0, 2.0, 3.0);
        let point2 = Point3::new(4.0, 5.0, 6.0);
        let result = point1 + point2;
        assert_eq!(result.x, 5.0);
        assert_eq!(result.y, 7.0);
        assert_eq!(result.z, 9.0);
    }

    #[test]
    fn test_point3_add_assign() {
        let mut point1 = Point3::new(1.0, 2.0, 3.0);
        let point2 = Point3::new(4.0, 5.0, 6.0);
        point1 += point2;
        assert_eq!(point1.x, 5.0);
        assert_eq!(point1.y, 7.0);
        assert_eq!(point1.z, 9.0);
    }

    #[test]
    fn test_point3_sub() {
        let point1 = Point3::new(4.0, 5.0, 6.0);
        let point2 = Point3::new(1.0, 2.0, 3.0);
        let result = point1 - point2;
        assert_eq!(result.x, 3.0);
        assert_eq!(result.y, 3.0);
        assert_eq!(result.z, 3.0);
    }
    
}
