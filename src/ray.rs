use std::ops::Mul;
use crate::transformations::Transformation;
use crate::vec::{Vec3, Point3};


#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self { origin, direction }
    }
}

impl Mul<Transformation> for Ray {
    type Output = Self;

    fn mul(self, rhs: Transformation) -> Self::Output {
        Self::new(rhs * self.origin, rhs * self.direction)
    }
}
