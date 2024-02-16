use crate::vec::Vec3;

/// Calculate intersection of ray with sphere
/// 
/// * `origin`: The origin of ray.
/// * `direction`: Direction of ray (direction must be normalized).
/// * `position`: Position of sphere.
/// * `radius`: Radius of sphere.
/// * `tmax`: Maximum distance of ray origin from sphere.
pub fn isect_ray_sphere(origin: Vec3, direction: Vec3, position: Vec3, radius: f32, tmax: f32) -> Option<f32>{
    // This is implementation from ray tracing gems book that improves precision
    // direction is assumed to be normalized so a = 1
    let f = origin - position;
    let c = f * f - radius * radius;

    let b_prime = -(f * direction);
    let tmp = f + b_prime * direction;
    let discriminant = radius * radius - tmp * tmp;

    if discriminant < 0.0 {
        None
    } else {
        let q = b_prime + b_prime.signum() * discriminant.sqrt();
        let t = c / q;
        if t > 0.0 && t < tmax {
            return Some(t);
        }
        let t = q;
        if t > 0.0 && t < tmax {
            return Some(t);
        }
        None
    }
}


fn isect_ray_sphere2(origin: Vec3, direction: Vec3, position: Vec3, radius: f32, tmax: f32) -> Option<f32>{
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn isect_sphere_test() {
        let origin = Vec3::new(1.0, -2.0, -1.0);
        let direction = Vec3::new(1.0, 2.0, 4.0).normalize();
        let position = Vec3::new(3.0, 0.0, 5.0);
        let radius = 3.0;
        let tmax = 100.0;
        let t1 = isect_ray_sphere(origin , direction, position, radius, tmax);
        let t2 = isect_ray_sphere2(origin , direction, position, radius, tmax);
        println!("{:?}", t1);
        println!("{:?}", t2);
    }

    #[test]
    fn isect_sphere2_test() {
        let origin = Vec3::new(0.0, 0.5, 0.0);
        let direction = Vec3::new(0.0, 0.0, -1.0).normalize();
        let position = Vec3::new(0.0, 0.0, 1.0);
        let radius = 20.0;
        let tmax = 1000.0;
        let t1 = isect_ray_sphere(origin , direction, position, radius, tmax);
        let t2 = isect_ray_sphere2(origin , direction, position, radius, tmax);
        println!("{:?}", t1);
        println!("{:?}", t2);
    }

    #[test]
    fn isect_sphere3_test() {
        let origin = Vec3::new(0.0, 1.0, 0.0);
        let direction = Vec3::new(0.0, 0.0, 1.0).normalize();
        let position = Vec3::new(0.0, 0.0, 1.0);
        let radius = 1.0;
        let tmax = 1000.0;
        let t1 = isect_ray_sphere(origin , direction, position, radius, tmax);
        let t2 = isect_ray_sphere2(origin , direction, position, radius, tmax);
        println!("{:?}", t1);
        println!("{:?}", t2);
    }
}
