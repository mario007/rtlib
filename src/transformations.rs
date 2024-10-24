
use crate::vec::{Normal, Point3, Vec3};
use crate::matrix::Matrix4x4;
use std::ops::Mul;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Transformation {
    mat: Matrix4x4,
    inv_mat: Matrix4x4
}

impl Transformation {
    pub fn scale(x: f32, y: f32, z: f32) -> Self {
        let mat = Matrix4x4::new([
            [x, 0.0, 0.0, 0.0],
            [0.0, y, 0.0, 0.0],
            [0.0, 0.0, z, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let inv_mat = Matrix4x4::new([
            [1.0 / x, 0.0, 0.0, 0.0],
            [0.0, 1.0 / y, 0.0, 0.0],
            [0.0, 0.0, 1.0 / z, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        Self { mat, inv_mat }
    }

    pub fn translate(delta: &Vec3) -> Self {
        let mat = Matrix4x4::new([
            [1.0, 0.0, 0.0, delta.x],
            [0.0, 1.0, 0.0, delta.y],
            [0.0, 0.0, 1.0, delta.z],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let inv_mat = Matrix4x4::new([
            [1.0, 0.0, 0.0, -delta.x],
            [0.0, 1.0, 0.0, -delta.y],
            [0.0, 0.0, 1.0, -delta.z],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        Self { mat, inv_mat }
    }

    pub fn rotate_x(theta: f32) -> Self {
        let cos_theta = theta.cos();
        let sin_theta = theta.sin();
        let mat = Matrix4x4::new([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, cos_theta, -sin_theta, 0.0],
            [0.0, sin_theta, cos_theta, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let inv_mat = mat.transpose();
        Self { mat, inv_mat }
    }

    pub fn rotate_y(theta: f32) -> Self {
        let cos_theta = theta.cos();
        let sin_theta = theta.sin();
        let mat = Matrix4x4::new([
            [cos_theta, 0.0, sin_theta, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [-sin_theta, 0.0, cos_theta, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let inv_mat = mat.transpose();
        Self { mat, inv_mat }
    }

    pub fn rotate_z(theta: f32) -> Self {
        let cos_theta = theta.cos();
        let sin_theta = theta.sin();
        let mat = Matrix4x4::new([
            [cos_theta, -sin_theta, 0.0, 0.0],
            [sin_theta, cos_theta, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let inv_mat = mat.transpose();
        Self { mat, inv_mat }
    }

    pub fn look_at(pos: Point3, look: Point3, up: Vec3) -> Self {
        let dir = (look - pos).normalize();
        if up.normalize().cross(dir).length() == 0.0 {
            panic!("up vector ({}, {}, {}) and viewing direction ({}, {}, {}) are parallel.",
                   up.x, up.y, up.z, dir.x, dir.y, dir.z);
        }
        let right = up.normalize().cross(dir).normalize();
        let new_up = dir.cross(right);
        let mat = Matrix4x4::new([
            [right.x, new_up.x, dir.x, pos.x],
            [right.y, new_up.y, dir.y, pos.y],
            [right.z, new_up.z, dir.z, pos.z],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let inv_mat = mat.inverse();
        if inv_mat.is_none() {
            panic!("Matrix {:?} is not invertible.", mat);
        }
        Self { mat, inv_mat: inv_mat.unwrap() }
    }

    pub fn orthographic(z_near: f32, z_far: f32) -> Self {
        Transformation::scale(1.0, 1.0, 1.0 / (z_far - z_near)) * 
        Transformation::translate(&Vec3::new(0.0, 0.0, -z_near))
    }
    
    pub fn perspective(fov: f32, z_near: f32, z_far: f32) -> Self {
        let mat = Matrix4x4::new([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, z_far / (z_far - z_near), -z_near * z_far / (z_far - z_near)],
            [0.0, 0.0, 1.0, 0.0],
        ]);
        let inv_tan_angle = (0.5 * fov.to_radians()).tan().recip();
        Transformation::scale(inv_tan_angle, inv_tan_angle, 1.0) * Transformation::from(mat)
    }

    pub fn inverse(&self) -> Self {
        Self { mat: self.inv_mat, inv_mat: self.mat }
    }

}

impl Mul for Transformation {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let mat = self.mat * other.mat;
        let inv_mat = other.inv_mat * self.inv_mat;
        Self { mat, inv_mat }
    }
}

impl From<Matrix4x4> for Transformation {
    fn from(mat: Matrix4x4) -> Self {
        let inv_mat = mat.inverse();
        if inv_mat.is_none() {
            panic!("Matrix {:?} is not invertible.", mat);
        }
        Self { mat, inv_mat: inv_mat.unwrap() }
    }
}

impl Mul<Point3> for Transformation {
    type Output = Point3;

    fn mul(self, point: Point3) -> Point3 {
        self.mat * point
    }
}

impl Mul<Transformation> for Point3 {
    type Output = Point3;

    fn mul(self, transformation: Transformation) -> Point3 {
        transformation * self
    }
}


impl Mul<Vec3> for Transformation {
    type Output = Vec3;

    fn mul(self, vec: Vec3) -> Vec3 {
        self.mat * vec
    }
}

impl Mul<Normal> for Transformation {
    type Output = Normal;

    fn mul(self, normal: Normal) -> Normal {
        self.inv_mat * normal
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_x() {
        let theta = 0.5;
        let transformation = Transformation::rotate_x(theta);
        // Add assertions here to test the correctness of the rotation
    }

    #[test]
    fn test_rotate_y() {
        let theta = 0.5;
        let transformation = Transformation::rotate_y(theta);
        // Add assertions here to test the correctness of the rotation
    }

    #[test]
    fn test_rotate_z() {
        let theta = 0.5;
        let transformation = Transformation::rotate_z(theta);
        // Add assertions here to test the correctness of the rotation
    }

    #[test]
    fn test_look_at() {
        let pos = Point3::new(0.0, 0.0, 0.0);
        let look = Point3::new(1.0, 0.0, 0.0);
        let up = Vec3::new(0.0, 1.0, 0.0);
        let transformation = Transformation::look_at(pos, look, up);
        // Add assertions here to test the correctness of the look_at transformation
    }

    #[test]
    fn test_orthographic() {
        let z_near = 1.0;
        let z_far = 10.0;
        let transformation = Transformation::orthographic(z_near, z_far);
        // Add assertions here to test the correctness of the orthographic transformation
    }

    #[test]
    fn test_perspective() {
        let fov = 60.0;
        let z_near = 1.0;
        let z_far = 100.0;
        let transformation = Transformation::perspective(fov, z_near, z_far);
        // Add assertions here to test the correctness of the perspective transformation
    }
}
