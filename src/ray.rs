use std::ops::Mul;
use crate::transformations::Transformation;
use crate::vec::{Normal, Point3, Vec3};


#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn point_at(&self, t: f32) -> Point3 {
        self.origin + t * self.direction
    }
}

impl Mul<Transformation> for Ray {
    type Output = Self;

    fn mul(self, rhs: Transformation) -> Self::Output {
        Self::new(rhs * self.origin, rhs * self.direction)
    }
}

pub fn offset_ray_origin(hit: Point3, normal: Normal) -> Point3 {

    const fn int_scale() -> f32 {256.0}
    fn origin() -> f32 { 1.0 / 32.0}
    fn float_scale() -> f32 {1.0 / 65536.0}

    fn float_as_int(n: f32) -> i32 { i32::from_le_bytes(n.to_le_bytes())}
    fn int_as_float(n: i32) -> f32 { f32::from_le_bytes(n.to_le_bytes())}

    let of_i_x = (int_scale() * normal.x) as i32;
    let of_i_y = (int_scale() * normal.y) as i32;
    let of_i_z = (int_scale() * normal.z) as i32;

    let p_i_x: f32;
    let p_i_y: f32;
    let p_i_z: f32;

    if hit.x < 0.0 {
        p_i_x = int_as_float(float_as_int(hit.x) - of_i_x);
    } else {
        p_i_x = int_as_float(float_as_int(hit.x) + of_i_x);
    }

    if hit.y < 0.0 {
        p_i_y = int_as_float(float_as_int(hit.y) - of_i_y);
    } else {
        p_i_y = int_as_float(float_as_int(hit.y) + of_i_y);
    }

    if hit.z < 0.0 {
        p_i_z = int_as_float(float_as_int(hit.z) - of_i_z);
    } else {
        p_i_z = int_as_float(float_as_int(hit.z) + of_i_z);
    }

    let rx: f32;
    let ry: f32;
    let rz: f32;

    if hit.x.abs() < origin() {
        rx = hit.x + float_scale() * normal.x;
    } else {
        rx = p_i_x;
    }

    if hit.y.abs() < origin() {
        ry = hit.y + float_scale() * normal.y;
    } else {
        ry = p_i_y;
    }

    if hit.z.abs() < origin() {
        rz = hit.z + float_scale() * normal.z;
    } else {
        rz = p_i_z;
    }

    Point3::new(rx, ry, rz)

}

pub fn spawn_new_ray(hit: Point3, normal: Normal, new_direction: Vec3) -> Ray {
    let offset = if normal * new_direction < 0.0 {
        offset_ray_origin(hit, -normal)
    } else {
        offset_ray_origin(hit, normal)
    };
    Ray::new(offset, new_direction)
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_offset() {
        let hit = Point3::new(0.2, 0.3, 1.5);
        let normal = Normal::new(1.0, 1.0, 1.0).normalize();
        println!("Offset point {:?}", offset_ray_origin(hit, normal));

        let hit = Point3::new(112.0, 366.0, 885.0);
        println!("Offset point {:?}", offset_ray_origin(hit, normal));
    }
}
