use crate::vec::Vec3;

pub fn cosine_hemisphere_direction(u1: f32, u2: f32) -> Vec3 {
    let term1 = 2.0 * std::f32::consts::PI * u1;
    let term2 = (1.0 - u2).sqrt();
    let x = term1.cos() * term2;
    let y = term1.sin() * term2;
    let z = u2.sqrt();
    Vec3::new(x, y, z)
}
