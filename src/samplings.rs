use crate::vec::Vec3;

pub struct SampleDirection {
    pub direction: Vec3,
    pub pdfw: f32
}

pub fn sample_cos_hemisphere(u1: f32, u2: f32) -> SampleDirection {
    let term1 = 2.0 * std::f32::consts::PI * u1;
    let term2 = (1.0 - u2).sqrt();
    let x = term1.cos() * term2;
    let y = term1.sin() * term2;
    let z = u2.sqrt();

    let direction = Vec3::new(x, y, z);
    // pdfw = cos(theta) / PI
    let pdfw = z * std::f32::consts::FRAC_1_PI;

    SampleDirection { direction, pdfw }
}

pub fn sample_uniform_hemisphere(u1: f32, u2: f32) -> SampleDirection {
    let term1 = 2.0 * std::f32::consts::PI * u2;
    let term2 = (1.0 - u1 * u1).sqrt();
    let x = term1.cos() * term2;
    let y = term1.sin() * term2;
    let z = u1;

    let direction = Vec3::new(x, y, z);
    // pdfw = 1 / (2 * PI)
    let pdfw = 0.5 * std::f32::consts::FRAC_1_PI;

    SampleDirection { direction, pdfw }
}


pub fn sample_uniform_sphere(u1: f32, u2: f32) -> SampleDirection {
    let term1 = 2.0 * std::f32::consts::PI * u1;
    let term2 = 2.0 * (u2 - u2 * u2).sqrt();

    let x = term1.cos() * term2;
    let y = term1.sin() * term2;
    let z = 1.0 - 2.0 * u2;

    let direction = Vec3::new(x, y, z);
    // pdfw = 1 / (4 * PI)
    let pdfw = 0.25 * std::f32::consts::FRAC_1_PI;

    SampleDirection { direction, pdfw }
}
