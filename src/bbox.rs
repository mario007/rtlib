use std::ops::Mul;
use crate::vec::{Point3, Vec3};
use crate::transformations::Transformation;

#[derive(Debug, Clone, Copy)]
pub struct AABB {
    pub min: Point3,
    pub max: Point3,
}

impl AABB {
    pub fn new(min: Point3, max: Point3) -> Self {
        Self { min, max }
    }

    pub fn intersect(&self, ray_origin: Point3, ray_inv_direction: Vec3) -> bool {
        crate::isect::isect_ray_bbox(ray_origin, ray_inv_direction, self.min, self.max)
    }

    pub fn centroid(&self) -> Point3 {
        (self.min + self.max) * 0.5
    }

    pub fn union(&self, other: &AABB) -> AABB {
        AABB::new(self.min.min(other.min), self.max.max(other.max))
    }

    pub fn area(&self) -> f32 {
        let delta = self.max - self.min;
        (delta.x * delta.y + delta.y * delta.z + delta.z * delta.x) * 2.0
    }

    pub fn max_axis(&self) -> usize {
        let extent = self.max - self.min;
        if extent.x > extent.y && extent.x > extent.z {
            0
        } else if extent.y > extent.z {
            1
        } else {
            2
        }
    }
}

impl Mul<Transformation> for AABB {
    type Output = Self;
    fn mul(self, rhs: Transformation) -> Self::Output {
        let delta = self.max - self.min;
        let p1 = rhs * self.min;
        let p2 = rhs * self.max;
        let p3 = rhs * (self.min + Vec3::new(delta.x, 0.0, 0.0));
        let p4 = rhs * (self.min + Vec3::new(0.0, delta.y, 0.0));
        let p5 = rhs * (self.min + Vec3::new(delta.x, delta.y, 0.0));
        let p6 = rhs * (self.max + Vec3::new(delta.x, 0.0, 0.0));
        let p7 = rhs * (self.max + Vec3::new(0.0, delta.y, 0.0));
        let p8 = rhs * (self.max + Vec3::new(delta.x, delta.y, 0.0));
        let min_p = p1.min(p2).min(p3).min(p4).min(p5).min(p6).min(p7).min(p8);
        let max_p = p1.max(p2).max(p3).max(p4).max(p5).max(p6).max(p7).max(p8);
        AABB::new(min_p, max_p)
    }
}
