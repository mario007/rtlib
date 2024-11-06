use crate::vec::{Vec3, Point3};
use crate::ray::Ray;

/// Calculate intersection of ray with sphere
/// 
/// * `ray`: Direction of ray must be normalized.
/// * `position`: Position of sphere.
/// * `radius`: Radius of sphere.
/// * `tmax`: Maximum distance of ray origin from sphere.
pub fn isect_ray_sphere(ray: &Ray, position: Point3, radius: f32, tmin: f32, tmax: f32) -> Option<f32>{
    // This is implementation from ray tracing gems book that improves precision
    // direction is assumed to be normalized so a = 1
    let f = ray.origin - position;
    let c = f * f - radius * radius;

    let b_prime = -(f * ray.direction);
    let tmp = f + b_prime * ray.direction;
    let discriminant = radius * radius - tmp * tmp;

    if discriminant < 0.0 {
        None
    } else {
        let q = b_prime + b_prime.signum() * discriminant.sqrt();
        let t = c / q;
        if t > tmin && t < tmax {
            return Some(t);
        }
        let t = q;
        if t > tmin && t < tmax {
            return Some(t);
        }
        None
    }
}


fn isect_ray_sphere2(origin: Point3, direction: Vec3, position: Point3, radius: f32, tmax: f32) -> Option<f32>{
    let ox = origin.x as f64;
    let oy = origin.y as f64;
    let oz = origin.z as f64;

    let dx = direction.x as f64;
    let dy = direction.y as f64;
    let dz = direction.z as f64;

    let px = position.x as f64;
    let py = position.y as f64;
    let pz = position.z as f64;

    let radius = radius as f64;
    let tmax = tmax as f64;
 
    let a = dx * dx + dy * dy + dz * dz;
    let fx = ox - px;
    let fy = oy - py;
    let fz = oz - pz;
    let b = 2.0 * (fx * dx + fy * dy + fz * dz);
    let c = fx * fx + fy * fy + fz * fz - radius * radius;

    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        None
    } else {
        let e = discriminant.sqrt();
        let denom = 2.0 * a;
        let t = (-b - e) / denom;
        if t > 0.0 && t < tmax {
            return Some(t as f32);
        }

        let t = (-b + e) / denom;
        if t > 0.0 && t < tmax {
            return Some(t as f32);
        }
        None
    }
}

// This intersection routine includes boundary
// https://tavianator.com/2022/ray_box_boundary.html
#[inline(always)]
pub fn isect_ray_bbox(ray_origin: Point3, ray_inv_dir: Vec3, bbox_min: Point3, bbox_max: Point3) -> bool {

    #[inline(always)]
    fn min(x: f32, y: f32) -> f32 {
        if x < y {x} else {y}
    }

    #[inline(always)]
    fn max(x: f32, y: f32) -> f32 {
        if x > y {x} else {y}
    }

    let mut tmin = 0.0;
    let mut tmax = f32::INFINITY;

    let t1 = (bbox_min.x - ray_origin.x) * ray_inv_dir.x;
    let t2 = (bbox_max.x - ray_origin.x) * ray_inv_dir.x;

    tmin = min(max(t1, tmin), max(t2, tmin));
    tmax = max(min(t1, tmax), min(t2, tmax));

    let t1 = (bbox_min.y - ray_origin.y) * ray_inv_dir.y;
    let t2 = (bbox_max.y - ray_origin.y) * ray_inv_dir.y;

    tmin = min(max(t1, tmin), max(t2, tmin));
    tmax = max(min(t1, tmax), min(t2, tmax));

    let t1 = (bbox_min.z - ray_origin.z) * ray_inv_dir.z;
    let t2 = (bbox_max.z - ray_origin.z) * ray_inv_dir.z;

    tmin = min(max(t1, tmin), max(t2, tmin));
    tmax = max(min(t1, tmax), min(t2, tmax));

    tmin <= tmax
}

pub fn isect_ray_triangle(ray: &Ray, v0: Point3, v1: Point3, v2: Point3, tmin: f32) -> Option<f32> {

    let a = v0.x - v1.x;
    let b = v0.x - v2.x;
    let c = ray.direction.x;
    let d = v0.x - ray.origin.x;
    let e = v0.y - v1.y;
    let f = v0.y - v2.y;
    let g = ray.direction.y;
    let h = v0.y - ray.origin.y;
    let i = v0.z - v1.z;
    let j = v0.z - v2.z;
    let k = ray.direction.z;
    let l = v0.z - ray.origin.z;

    let m = f * k - g * j;
    let n = h * k - g * l;
    let p = f * l - h * j;
    let q = g * i - e * k;
    let s = e * j - f * i;

    let temp3 = a * m + b * q +  c * s;

    if temp3 == 0.0 { return None }

    let inv_denom = 1.0 / temp3;
    let e1 = d * m - b * n - c * p;
    let beta = e1 * inv_denom;

    if beta < 0.0 { return None }

    let r = e * l - h * i;
    let e2 = a * n + d * q + c * r;
    let gamma = e2 * inv_denom;

    if gamma < 0.0 { return None }

    if beta + gamma > 1.0 { return None}

    let e3 = a * p - b * r + d * s;
    let t = e3 * inv_denom;

    if t < tmin {
        return None
    }
    Some(t)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn isect_sphere_test() {
        let origin = Point3::new(1.0, -2.0, -1.0);
        let direction = Vec3::new(1.0, 2.0, 4.0).normalize();
        let ray = Ray::new(origin, direction);
        let position = Point3::new(3.0, 0.0, 5.0);
        let radius = 3.0;
        let tmax = 100.0;
        let tmin = 0.0;
        let t1 = isect_ray_sphere(&ray, position, radius, tmin, tmax);
        let t2 = isect_ray_sphere2(origin , direction, position, radius, tmax);
        println!("{:?}", t1);
        println!("{:?}", t2);
    }

    #[test]
    fn isect_sphere2_test() {
        let origin = Point3::new(0.0, 0.5, 0.0);
        let direction = Vec3::new(0.0, 0.0, -1.0).normalize();
        let ray = Ray::new(origin, direction);
        let position = Point3::new(0.0, 0.0, 1.0);
        let radius = 20.0;
        let tmax = 1000.0;
        let tmin = 0.0;
        let t1 = isect_ray_sphere(&ray, position, radius, tmin, tmax);
        let t2 = isect_ray_sphere2(origin , direction, position, radius, tmax);
        println!("{:?}", t1);
        println!("{:?}", t2);
    }

    #[test]
    fn isect_sphere3_test() {
        let origin = Point3::new(0.0, 1.0, 0.0);
        let direction = Vec3::new(0.0, 0.0, 1.0).normalize();
        let ray = Ray::new(origin, direction);
        let position = Point3::new(0.0, 0.0, 1.0);
        let radius = 1.0;
        let tmax = 1000.0;
        let tmin = 0.0;
        let t1 = isect_ray_sphere(&ray, position, radius, tmin, tmax);
        let t2 = isect_ray_sphere2(origin , direction, position, radius, tmax);
        println!("{:?}", t1);
        println!("{:?}", t2);
    }
}
